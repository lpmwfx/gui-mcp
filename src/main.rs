mod adapter;
mod core;
mod pal;
mod shared;
mod state;
mod ui;

use ui::server_ui::SlintGuiServer;

/// Entry point — starts the MCP server on stdio transport.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = rmcp::transport::io::stdio();
    rmcp::serve_server(SlintGuiServer, transport).await?;
    Ok(())
}
