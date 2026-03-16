/// Local smoke tests -- verify PAL, adapter, and MCP registration.
use slint_gui_mcp::adapter::app_adp;
use slint_gui_mcp::pal::window_pal;
use base64::{engine::general_purpose::STANDARD, Engine as _};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Test MCP tool registration via full protocol handshake.
#[tokio::test]
async fn test_mcp_tools_registered() -> TestResult {
    use slint_gui_mcp::ui::server_ui::SlintGuiServer_ui;

    let (client_tx, server_rx) = tokio::io::duplex(4096);
    let (server_tx, client_rx) = tokio::io::duplex(4096);

    let server_future = async move { rmcp::serve_server(SlintGuiServer_ui, (server_rx, server_tx)).await };
    let server_handle = tokio::task::spawn(server_future);

    let client = rmcp::serve_client((), (client_rx, client_tx))
        .await
        .map_err(|e| format!("client start: {e}"))?;

    let tools_list = client
        .list_tools(Default::default())
        .await
        .map_err(|e| format!("tools/list: {e}"))?;

    assert_eq!(tools_list.tools.len(), slint_gui_mcp::state::sizes::EXPECTED_TOOL_COUNT, "Expected 14 MCP tools");

    let _ = client.cancel().await;
    server_handle.abort();
    Ok(())
}

/// Lists all visible windows and prints them. Always passes on a live desktop.
#[test]
fn test_list_windows() -> TestResult {
    let titles = window_pal::list_window_titles_pal()?;
    println!("\n=== Visible windows ({}) ===", titles.len());
    for t in &titles {
        println!("  {t}");
    }
    assert!(
        !titles.is_empty(),
        "Expected at least one visible window on the desktop"
    );
    Ok(())
}

/// Screenshots the first available window, checks dimensions, saves PNG to disk.
#[test]
fn test_screenshot_first_window() -> TestResult {
    let titles = window_pal::list_window_titles_pal()?;
    for title in titles {
        if let Ok((b64, w, h, _)) = app_adp::screenshot_window(&title) {
            println!("\n=== Screenshot of: {title} ===");
            println!("  {w}x{h}, {} base64 chars", b64.len());

            assert!(w > 0 && h > 0, "Expected positive dimensions");
            assert!(!b64.is_empty(), "Expected non-empty base64 PNG");

            let png_bytes = STANDARD.decode(&b64)?;
            println!("  PNG size: {} bytes", png_bytes.len());
            return Ok(());
        }
    }

    Err("No visible window could be captured successfully".into())
}

/// Gets window info for the first available window.
#[test]
fn test_get_window_info() -> TestResult {
    let titles = window_pal::list_window_titles_pal()?;
    let title = titles.first().ok_or("No windows found")?;

    let info_json = app_adp::get_window_info(title)?;
    println!("\n=== Window info for: {title} ===\n  {info_json}");

    assert!(info_json.contains("rect"), "Expected rect in JSON");
    assert!(info_json.contains("width"), "Expected width in JSON");
    Ok(())
}

/// Crops a region from a screenshot, crops a sub-template, finds it back via NCC.
#[test]
fn test_crop_then_find() -> TestResult {
    use slint_gui_mcp::core::crop_core;
    use slint_gui_mcp::core::vision_core;
    use slint_gui_mcp::state::sizes::CROP_FIND_MAX_DIM;
    let titles = window_pal::list_window_titles_pal()?;
    for title in &titles {
        let hwnd = window_pal::find_window_by_partial_title(title);
        if hwnd.is_err() { continue; }
        let capture = window_pal::capture_window(hwnd.unwrap());
        if capture.is_err() { continue; }
        let full = capture.unwrap();
        let (fw, fh) = full.dimensions();
        if fw < 200 || fh < 200 { continue; }
        let haystack_w = fw.min(CROP_FIND_MAX_DIM);
        let haystack_h = fh.min(CROP_FIND_MAX_DIM);
        let haystack = crop_core::crop_region_core(&full, 0, 0, haystack_w, haystack_h);
        if haystack.is_none() { continue; }
        let haystack = haystack.unwrap();
        let template = crop_core::crop_region_core(&haystack, 10, 10, 60, 40);
        if template.is_none() { continue; }
        let template = template.unwrap();
        let match_found = vision_core::find_template(&haystack, &template, None);
        if let Some(m) = match_found {
            println!("\n=== crop_then_find: {title} (haystack {haystack_w}x{haystack_h}) ===");
            println!("  found at ({},{}) conf={:.3}", m.x, m.y, m.confidence);
            assert!(m.confidence > 0.9, "Self-crop should match with high confidence");
            return Ok(());
        }
    }
    Err("No window produced a successful crop->find cycle".into())
}
