/// MCP server — tool definitions and dispatch.
use rmcp::{ServerHandler, model::*, tool, Error as McpError};

/// MCP server that exposes GUI vision and control tools to AI assistants.
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct SlintGuiServer_ui;

#[tool(tool_box)]
impl SlintGuiServer_ui {
    /// Take a screenshot of a named window and return it as base64 PNG.
    #[tool(description = "Take a screenshot of the named window and return a base64 PNG")]
    async fn screenshot_window(
        &self,
        #[tool(param)] window_title: String,
    ) -> Result<CallToolResult, McpError> {
        todo!("dispatch to app_adp::screenshot_window")
    }

    /// Find a UI element via template matching without clicking.
    #[tool(description = "Find a UI element via image template matching — returns coordinates without clicking")]
    async fn find_element(
        &self,
        #[tool(param)] window_title: String,
        #[tool(param)] template_base64: String,
        #[tool(param)] confidence: Option<f32>,
    ) -> Result<CallToolResult, McpError> {
        todo!("dispatch to app_adp::find_element")
    }

    /// Find a UI element via template matching and click it.
    #[tool(description = "Find a UI element via template matching and click it")]
    async fn click_element(
        &self,
        #[tool(param)] window_title: String,
        #[tool(param)] template_base64: String,
        #[tool(param)] confidence: Option<f32>,
        #[tool(param)] button: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        todo!("dispatch to app_adp::click_element")
    }

    /// Type text into the currently focused element.
    #[tool(description = "Type text into the currently focused element")]
    async fn type_text(
        &self,
        #[tool(param)] text: String,
    ) -> Result<CallToolResult, McpError> {
        todo!("dispatch to app_adp::type_text")
    }

    /// Send a key combination such as ctrl+s, enter, or tab.
    #[tool(description = "Send a key combination such as ctrl+s, enter, or tab")]
    async fn send_keys(
        &self,
        #[tool(param)] keys: String,
    ) -> Result<CallToolResult, McpError> {
        todo!("dispatch to app_adp::send_keys")
    }

    /// Get information about a specific window by title.
    #[tool(description = "Get title, rect, and visibility info for a window")]
    async fn get_window_info(
        &self,
        #[tool(param)] window_title: String,
    ) -> Result<CallToolResult, McpError> {
        todo!("dispatch to app_adp::get_window_info")
    }

    /// List all visible windows — used to discover the correct window title.
    #[tool(description = "List all visible windows with their titles")]
    async fn list_windows(&self) -> Result<CallToolResult, McpError> {
        todo!("dispatch to app_adp::list_windows")
    }
}

impl ServerHandler for SlintGuiServer_ui {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: Default::default(),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "slint-gui-mcp".into(),
                version: env!("CARGO_PKG_VERSION").into(),
            },
            instructions: None,
        }
    }
}
