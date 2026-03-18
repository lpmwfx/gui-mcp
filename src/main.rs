use slint_gui_mcp::ui::server_ui::SlintGuiServer_ui;

/// Sets per-monitor DPI awareness so all Win32 coordinate APIs
/// (GetWindowRect, SetCursorPos, PrintWindow) use physical pixels consistently.
#[cfg(windows)]
fn enable_dpi_awareness() {
    use windows::Win32::UI::HiDpi::{
        SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
    };
    // SAFETY: must be called before any other Win32 API; safe at process start.
    unsafe { let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2); }
}

/// Entry point  --  starts the MCP server on stdio transport.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    enable_dpi_awareness();
    let transport = rmcp::transport::io::stdio();
    let server = rmcp::serve_server(SlintGuiServer_ui, transport).await?;
    server.waiting().await?;
    Ok(())
}
