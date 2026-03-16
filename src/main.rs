// Re-export library modules so `crate::adapter` etc. resolves inside `mod ui`.
pub use slint_gui_mcp::{adapter, core, pal, shared, state};
mod ui;

use ui::server_ui::SlintGuiServer_ui;

/// Entry point — starts the MCP server on stdio transport.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = rmcp::transport::io::stdio();
    rmcp::serve_server(SlintGuiServer_ui, transport).await?;
    Ok(())
}
