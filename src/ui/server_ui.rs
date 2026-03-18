/// MCP server  --  tool definitions and dispatch.
use rmcp::{ServerHandler, model::*, tool, Error as McpError};
use crate::adapter::app_adp;
use crate::state::help_text;
use crate::state::sizes::{BURST_DEFAULT_COUNT, BURST_MAX_COUNT, BURST_CONTENT_PER_FRAME};

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
    #[tool(description = "Find a UI element via image template matching  --  returns coordinates without clicking")]
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

    /// Type text into a window. Focuses the window first, then types.
    #[tool(description = "Focus a window by title, then type text into it")]
    async fn type_text(
        &self,
        #[tool(param)] window_title: String,
        #[tool(param)] text: String,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::focused_type_text(&window_title, &text) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text("ok")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Send a key combination to a window. Focuses the window first, then sends keys.
    #[tool(description = "Focus a window by title, then send a key combination (e.g. ctrl+s, enter, tab)")]
    async fn send_keys(
        &self,
        #[tool(param)] window_title: String,
        #[tool(param)] keys: String,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::focused_send_keys(&window_title, &keys) {
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

    /// List all visible windows  --  used to discover the correct window title.
    #[tool(description = "List all visible windows with their titles")]
    async fn list_windows(&self) -> Result<CallToolResult, McpError> {
        match app_adp::list_windows() {
            Ok(list_json) => Ok(CallToolResult::success(vec![Content::text(list_json)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Select all text in the edit control of a named window.
    #[tool(description = "Select all text in a window's edit control (like Ctrl+A but works in background)")]
    async fn select_all(
        &self,
        #[tool(param)] window_title: String,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::select_all_adp(&window_title) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text("ok")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Copy the current selection to clipboard.
    #[tool(description = "Copy the current selection in a window to the clipboard (like Ctrl+C but works in background)")]
    async fn copy(
        &self,
        #[tool(param)] window_title: String,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::copy_adp(&window_title) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text("ok")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Cut the current selection to clipboard.
    #[tool(description = "Cut the current selection in a window to the clipboard (like Ctrl+X but works in background)")]
    async fn cut(
        &self,
        #[tool(param)] window_title: String,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::cut_adp(&window_title) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text("ok")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Paste clipboard content into a window's edit control.
    #[tool(description = "Paste clipboard content into a window's edit control (like Ctrl+V but works in background)")]
    async fn paste(
        &self,
        #[tool(param)] window_title: String,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::paste_adp(&window_title) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text("ok")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Click at absolute client coordinates in a named window.
    #[tool(description = "Click at pixel coordinates (x, y) inside a window -- no template matching needed")]
    async fn click_at(
        &self,
        #[tool(param)] window_title: String,
        #[tool(param)] x: i32,
        #[tool(param)] y: i32,
        #[tool(param)] button: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        let btn = button.as_deref().unwrap_or("left");
        match app_adp::click_at_adp(&window_title, x, y, btn) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text("ok")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Crop a rectangular region from a window screenshot and return as base64 PNG.
    #[tool(description = "Crop a region (x, y, width, height) from a window screenshot and return it as base64 PNG -- useful for creating reusable templates")]
    async fn crop_region(
        &self,
        #[tool(param)] window_title: String,
        #[tool(param)] x: u32,
        #[tool(param)] y: u32,
        #[tool(param)] width: u32,
        #[tool(param)] height: u32,
    ) -> Result<CallToolResult, McpError> {
        match app_adp::crop_region_adp(&window_title, x, y, width, height) {
            Ok(png_b64) => Ok(CallToolResult::success(vec![
                Content::image(png_b64, "image/png"),
            ])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    /// Show help and documentation. Topics: tools, keys, mimic, workflow.
    #[tool(description = "Show help for gui-mcp tools, key names, and mimic scripting. Topics: tools, keys, mimic, workflow")]
    async fn help(
        &self,
        #[tool(param)] topic: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        let text = help_text::help_for_topic(topic.as_deref());
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    /// Take a rapid burst of screenshots for near-live GUI monitoring.
    #[tool(description = "Take multiple screenshots in rapid succession for near-live GUI viewing (default 5 frames, max 10)")]
    async fn screenshot_burst(
        &self,
        #[tool(param)] window_title: String,
        #[tool(param)] count: Option<usize>,
    ) -> Result<CallToolResult, McpError> {
        let n = count.unwrap_or(BURST_DEFAULT_COUNT).min(BURST_MAX_COUNT).max(1);
        match app_adp::screenshot_burst(&window_title, n) {
            Ok(frames) => {
                let mut content: Vec<Content> = Vec::with_capacity(n * BURST_CONTENT_PER_FRAME);
                for (i, (png, w, h)) in frames.iter().enumerate() {
                    content.push(Content::image(png.clone(), "image/png"));
                    content.push(Content::text(format!("frame {} / {}  --  {}x{}", i + 1, n, w, h)));
                }
                Ok(CallToolResult::success(content))
            }
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
