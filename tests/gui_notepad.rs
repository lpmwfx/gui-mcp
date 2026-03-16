/// GUI interaction tests using Notepad -- requires a live Windows desktop.
#![cfg(windows)]

use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::{imageops, GenericImageView, RgbImage};
use rmcp::model::{CallToolRequestParam, CallToolResult};
use serde_json::{json, Value};
use slint_gui_mcp::adapter::app_adp;
use slint_gui_mcp::pal::window_pal;
use std::borrow::Cow;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::OnceLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static GUI_TEST_LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();

fn gui_test_lock() -> &'static tokio::sync::Mutex<()> {
    GUI_TEST_LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

type TestResult = Result<(), Box<dyn std::error::Error>>;
type TestResultWith<T> = Result<T, Box<dyn std::error::Error>>;

struct TestNotepad {
    child: Child,
    title_fragment: String,
    file_path: PathBuf,
}

impl Drop for TestNotepad {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let path_str = self.file_path.to_string_lossy().to_string();
        let _ = Command::new("cmd").args(["/C", "del", &path_str]).output();
    }
}

fn launch_test_notepad() -> TestResultWith<TestNotepad> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let title_fragment = format!("gui-mcp-tool-test-{stamp}.txt");
    let file_path = std::env::temp_dir().join(&title_fragment);
    let text = format!("{stamp}-GUI-MCP-UNIQUE-LINE-ABCDEFGHIJKLMNOPQRSTUVWXYZ\n");
    let path_str = file_path.to_string_lossy().to_string();
    Command::new("cmd")
        .args(["/C", &format!("echo {text}> \"{path_str}\"")])
        .output()?;

    let child = Command::new("notepad.exe").arg(&file_path).spawn()?;
    wait_for_window(&title_fragment)?;
    std::thread::sleep(Duration::from_millis(700));

    Ok(TestNotepad { child, title_fragment, file_path })
}

fn wait_for_window(title_fragment: &str) -> TestResult {
    for _ in 0..50 {
        let titles = window_pal::list_window_titles_pal()?;
        if titles.iter().any(|t| t.contains(title_fragment)) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    Err(format!("Timed out waiting for window: {title_fragment}").into())
}

fn capture_window_image(window_title: &str) -> TestResultWith<RgbImage> {
    let (b64, _, _, _) = app_adp::screenshot_window(window_title)?;
    let png_bytes = STANDARD.decode(&b64)?;
    Ok(image::load_from_memory(&png_bytes)?.into_rgb8())
}

fn encode_png_base64(img: &RgbImage) -> TestResultWith<String> {
    let mut png = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)?;
    Ok(STANDARD.encode(png))
}

fn pick_text_crop(img: &RgbImage) -> TestResultWith<(RgbImage, u32, u32)> {
    let crop_w = 24u32;
    let crop_h = 8u32;
    let max_x = img.width().min(260).saturating_sub(crop_w + 1);
    let max_y = img.height().min(220).saturating_sub(crop_h + 1);
    let mut best: Option<(RgbImage, u32, u32, u32)> = None;

    for y in (90..=max_y).step_by(4) {
        for x in (16..=max_x).step_by(4) {
            let crop = imageops::crop_imm(img, x, y, crop_w, crop_h).to_image();
            let score = variance_score(&crop);
            if score == 0 {
                continue;
            }
            match &best {
                Some((_, _, _, best_score)) if *best_score >= score => {}
                _ => best = Some((crop, x, y, score)),
            }
        }
    }

    best.map(|(crop, x, y, _)| (crop, x, y))
        .ok_or_else(|| "Failed to find a non-uniform crop in the Notepad text area".into())
}

fn variance_score(img: &RgbImage) -> u32 {
    let mut total = 0u32;
    for pixel in img.pixels() {
        total += pixel.0.iter().map(|&ch| 255u32.saturating_sub(ch as u32)).sum::<u32>();
    }
    total
}

fn parse_json_text(call_result: &CallToolResult) -> TestResultWith<Value> {
    let text = call_result
        .content
        .iter()
        .find_map(|c| c.raw.as_text().map(|t| t.text.as_str()))
        .ok_or("Expected text content in MCP tool result")?;
    Ok(serde_json::from_str(text)?)
}

fn first_image_data(call_result: &CallToolResult) -> TestResultWith<String> {
    call_result
        .content
        .iter()
        .find_map(|c| c.raw.as_image().map(|img| img.data.clone()))
        .ok_or_else(|| "Expected image content in MCP tool result".into())
}

async fn mcp_client() -> TestResultWith<(
    rmcp::service::RunningService<rmcp::RoleClient, ()>,
    tokio::task::JoinHandle<Result<rmcp::service::RunningService<rmcp::RoleServer, slint_gui_mcp::ui::server_ui::SlintGuiServer_ui>, rmcp::Error>>,
)> {
    use slint_gui_mcp::ui::server_ui::SlintGuiServer_ui;
    let (client_tx, server_rx) = tokio::io::duplex(4096);
    let (server_tx, client_rx) = tokio::io::duplex(4096);
    let server_future = async move { rmcp::serve_server(SlintGuiServer_ui, (server_rx, server_tx)).await };
    let server_handle = tokio::task::spawn(server_future);
    let client = rmcp::serve_client((), (client_rx, client_tx))
        .await
        .map_err(|e| format!("client start: {e}"))?;
    Ok((client, server_handle))
}

#[tokio::test(flavor = "current_thread")]
async fn test_mcp_find_element_on_notepad() -> TestResult {
    let _guard = gui_test_lock().lock().await;
    let notepad = launch_test_notepad()?;
    let screenshot = capture_window_image(&notepad.title_fragment)?;
    let (template, crop_x, crop_y) = pick_text_crop(&screenshot)?;
    let template_base64 = encode_png_base64(&template)?;

    let (client, _server_handle) = mcp_client().await?;
    let result_find = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("find_element"),
            arguments: json!({
                "window_title": notepad.title_fragment,
                "template_base64": template_base64,
                "confidence": 0.99
            })
            .as_object()
            .cloned(),
        })
        .await
        .map_err(|e| format!("tools/call find_element: {e}"))?;

    assert_eq!(result_find.is_error, Some(false), "Expected successful MCP result");
    let payload = parse_json_text(&result_find)?;
    let expected_x = crop_x + template.width() / 2;
    let expected_y = crop_y + template.height() / 2;
    let actual_x = payload["x"].as_u64().ok_or("Missing x")? as i64;
    let actual_y = payload["y"].as_u64().ok_or("Missing y")? as i64;
    assert!((actual_x - expected_x as i64).abs() <= 1, "Unexpected x: {actual_x}");
    assert!((actual_y - expected_y as i64).abs() <= 1, "Unexpected y: {actual_y}");
    assert_eq!(payload["width"].as_u64(), Some(template.width() as u64));
    assert_eq!(payload["height"].as_u64(), Some(template.height() as u64));
    assert!(payload["confidence"].as_f64().unwrap_or_default() >= 0.99);

    let _ = client.cancel().await;
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn test_mcp_click_element_on_notepad() -> TestResult {
    let _guard = gui_test_lock().lock().await;
    let notepad = launch_test_notepad()?;
    let screenshot = capture_window_image(&notepad.title_fragment)?;
    let (template, crop_x, crop_y) = pick_text_crop(&screenshot)?;
    let template_base64 = encode_png_base64(&template)?;

    let (client, _server_handle) = mcp_client().await?;
    let result_click = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("click_element"),
            arguments: json!({
                "window_title": notepad.title_fragment,
                "template_base64": template_base64,
                "confidence": 0.99,
                "button": "left"
            })
            .as_object()
            .cloned(),
        })
        .await
        .map_err(|e| format!("tools/call click_element: {e}"))?;

    assert_eq!(result_click.is_error, Some(false), "Expected successful MCP result");
    let payload = parse_json_text(&result_click)?;
    let expected_x = crop_x + template.width() / 2;
    let expected_y = crop_y + template.height() / 2;
    let actual_x = payload["x"].as_i64().ok_or("Missing x")?;
    let actual_y = payload["y"].as_i64().ok_or("Missing y")?;
    assert!((actual_x - expected_x as i64).abs() <= 1, "Unexpected x: {actual_x}");
    assert!((actual_y - expected_y as i64).abs() <= 1, "Unexpected y: {actual_y}");
    assert!(payload["confidence"].as_f64().unwrap_or_default() >= 0.99);

    let after_image = first_image_data(&result_click)?;
    let after_png = STANDARD.decode(after_image)?;
    assert!(!after_png.is_empty(), "Expected a non-empty after-screenshot");
    let after_dims = image::load_from_memory(&after_png)?.dimensions();
    assert_eq!(after_dims, screenshot.dimensions());

    let _ = client.cancel().await;
    Ok(())
}
