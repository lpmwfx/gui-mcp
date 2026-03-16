/// MCP server — tool definitions and dispatch.
use rmcp::{ServerHandler, model::*, tool, Error as McpError};
use crate::adapter::app_adp;

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
        match app_adp::screenshot_window(&window_title) {
            Ok((png, w, h, _)) => Ok(CallToolResult::success(vec![
                Content::image(png, "image/png"),
                Content::text(format!("{w}x{h}")),
            ])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Find a UI element via template matching without clicking.
    #[tool(description = "Find a UI element via image template matching — returns coordinates without clicking")]
    async fn find_element(
        &self,
        #[tool(param)] window_title: String,
        #[tool(param)] template_base64: String,
        #[tool(param)] confidence: Option<f32>,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::find_element(&window_title, &template_base64, confidence) {
            Ok(Some((x, y, w, h, conf))) => Ok(CallToolResult::success(vec![
                Content::text(format!(r#"{{"x":{x},"y":{y},"width":{w},"height":{h},"confidence":{conf}}}"#)),
            ])),
            Ok(None) => Ok(CallToolResult::success(vec![Content::text("null")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
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
        match app_adp::click_element(&window_title, &template_base64, confidence, button.as_deref()) {
            Ok((x, y, conf, after_png)) => Ok(CallToolResult::success(vec![
                Content::image(after_png, "image/png"),
                Content::text(format!(r#"{{"x":{x},"y":{y},"confidence":{conf}}}"#)),
            ])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Type text into the currently focused element.
    #[tool(description = "Type text into the currently focused element")]
    async fn type_text(
        &self,
        #[tool(param)] text: String,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::type_text(&text) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text("ok")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Send a key combination such as ctrl+s, enter, or tab.
    #[tool(description = "Send a key combination such as ctrl+s, enter, or tab")]
    async fn send_keys(
        &self,
        #[tool(param)] keys: String,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::send_keys(&keys) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text("ok")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Get information about a specific window by title.
    #[tool(description = "Get title, rect, and visibility info for a window")]
    async fn get_window_info(
        &self,
        #[tool(param)] window_title: String,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::get_window_info(&window_title) {
            Ok(info_json) => Ok(CallToolResult::success(vec![Content::text(info_json)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// List all visible windows — used to discover the correct window title.
    #[tool(description = "List all visible windows with their titles")]
    async fn list_windows(&self) -> Result<CallToolResult, McpError> {
        match app_adp::list_windows() {
            Ok(list_json) => Ok(CallToolResult::success(vec![Content::text(list_json)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }
}

#[tool(tool_box)]
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
