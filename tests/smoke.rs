/// Local smoke tests — verify the PAL and adapter layers work against real windows.
///
/// Run with:  cargo test -- --nocapture
/// The screenshot test saves `smoke_screenshot.png` in the project root.
use base64::{Engine as _, engine::general_purpose::STANDARD};
use slint_gui_mcp::pal::window_pal;
use slint_gui_mcp::adapter::app_adp;

/// Test MCP tool registration via full protocol handshake.
#[tokio::test]
async fn test_mcp_tools_registered() -> TestResult {
    use slint_gui_mcp::ui::server_ui::SlintGuiServer_ui;

    let (client_tx, server_rx) = tokio::io::duplex(4096);
    let (server_tx, client_rx) = tokio::io::duplex(4096);

    let _server_handle = tokio::spawn(async move {
        rmcp::serve_server(SlintGuiServer_ui, (server_rx, server_tx)).await
    });

    let client = rmcp::serve_client((), (client_rx, client_tx)).await
        .map_err(|e| format!("client start: {e}"))?;

    let tools_result = client.list_tools(Default::default()).await
        .map_err(|e| format!("tools/list: {e}"))?;

    assert_eq!(tools_result.tools.len(), 7, "Expected 7 MCP tools");

    let _ = client.cancel().await;
    Ok(())
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Lists all visible windows and prints them.  Always passes on a live desktop.
#[test]
fn test_list_windows() -> TestResult {
    let titles = window_pal::list_window_titles_pal()?;
    println!("\n=== Visible windows ({}) ===", titles.len());
    for t in &titles {
        println!("  {t}");
    }
    assert!(!titles.is_empty(), "Expected at least one visible window on the desktop");
    Ok(())
}

/// Screenshots the first available window, checks dimensions, saves PNG to disk.
#[test]
fn test_screenshot_first_window() -> TestResult {
    let titles = window_pal::list_window_titles_pal()?;
    let title = titles.first().ok_or("No windows found")?;
    println!("\n=== Screenshot of: {title} ===");

    let (b64, w, h, _) = app_adp::screenshot_window(title)?;
    println!("  {w}x{h}, {} base64 chars", b64.len());

    assert!(w > 0 && h > 0, "Expected positive dimensions");
    assert!(!b64.is_empty(), "Expected non-empty base64 PNG");

    let png_bytes = STANDARD.decode(&b64)?;
    std::fs::write("smoke_screenshot.png", &png_bytes)?;
    println!("  Saved smoke_screenshot.png ({} bytes)", png_bytes.len());

    Ok(())
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
