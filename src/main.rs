use slint_gui_mcp::ui::server_ui::SlintGuiServer_ui;

/// Entry point — starts the MCP server on stdio transport.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = rmcp::transport::io::stdio();
    let server = rmcp::serve_server(SlintGuiServer_ui, transport).await?;
    server.waiting().await?;
    Ok(())
}
