/// Negative-path MCP tests -- missing windows, invalid templates, low confidence.
use rmcp::model::CallToolRequestParam;
use serde_json::json;
use std::borrow::Cow;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Helper: spin up an in-process MCP client connected to the server.
async fn connected_client() -> Result<rmcp::service::RunningService<rmcp::RoleClient, ()>, Box<dyn std::error::Error>> {
    use slint_gui_mcp::ui::server_ui::SlintGuiServer_ui;
    let (client_tx, server_rx) = tokio::io::duplex(4096);
    let (server_tx, client_rx) = tokio::io::duplex(4096);
    let _server = tokio::spawn(async move {
        rmcp::serve_server(SlintGuiServer_ui, (server_rx, server_tx)).await
    });
    let client = rmcp::serve_client((), (client_rx, client_tx))
        .await
        .map_err(|e| format!("client start: {e}"))?;
    Ok(client)
}

/// screenshot_window with a non-existent window returns an error result.
#[tokio::test]
async fn test_screenshot_missing_window() -> TestResult {
    let client = connected_client().await?;
    let result_mcp = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("screenshot_window"),
            arguments: json!({"window_title": "nonexistent-window-xyz-12345"})
                .as_object().cloned(),
        })
        .await
        .map_err(|e| format!("call_tool: {e}"))?;
    assert_eq!(result_mcp.is_error, Some(true), "Expected error for missing window");
    let _ = client.cancel().await;
    Ok(())
}

/// find_element with invalid base64 template returns an error result.
#[tokio::test]
async fn test_find_element_invalid_template() -> TestResult {
    let client = connected_client().await?;
    let result_mcp = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("find_element"),
            arguments: json!({
                "window_title": "nonexistent-window-xyz-12345",
                "template_base64": "not-valid-base64!!!",
            })
            .as_object().cloned(),
        })
        .await
        .map_err(|e| format!("call_tool: {e}"))?;
    assert_eq!(result_mcp.is_error, Some(true), "Expected error for invalid template");
    let _ = client.cancel().await;
    Ok(())
}

/// click_element with a non-existent window returns an error result.
#[tokio::test]
async fn test_click_element_missing_window() -> TestResult {
    let client = connected_client().await?;
    let result_mcp = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("click_element"),
            arguments: json!({
                "window_title": "nonexistent-window-xyz-12345",
                "template_base64": "AAAA",
            })
            .as_object().cloned(),
        })
        .await
        .map_err(|e| format!("call_tool: {e}"))?;
    assert_eq!(result_mcp.is_error, Some(true), "Expected error for missing window");
    let _ = client.cancel().await;
    Ok(())
}

/// type_text with a non-existent window returns an error result.
#[tokio::test]
async fn test_type_text_missing_window() -> TestResult {
    let client = connected_client().await?;
    let result_mcp = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("type_text"),
            arguments: json!({
                "window_title": "nonexistent-window-xyz-12345",
                "text": "hello",
            })
            .as_object().cloned(),
        })
        .await
        .map_err(|e| format!("call_tool: {e}"))?;
    assert_eq!(result_mcp.is_error, Some(true), "Expected error for missing window");
    let _ = client.cancel().await;
    Ok(())
}

/// send_keys with a non-existent window returns an error result.
#[tokio::test]
async fn test_send_keys_missing_window() -> TestResult {
    let client = connected_client().await?;
    let result_mcp = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("send_keys"),
            arguments: json!({
                "window_title": "nonexistent-window-xyz-12345",
                "keys": "enter",
            })
            .as_object().cloned(),
        })
        .await
        .map_err(|e| format!("call_tool: {e}"))?;
    assert_eq!(result_mcp.is_error, Some(true), "Expected error for missing window");
    let _ = client.cancel().await;
    Ok(())
}

/// select_all with a non-existent window returns an error result.
#[tokio::test]
async fn test_select_all_missing_window() -> TestResult {
    let client = connected_client().await?;
    let result_mcp = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("select_all"),
            arguments: json!({"window_title": "nonexistent-window-xyz-12345"})
                .as_object().cloned(),
        })
        .await
        .map_err(|e| format!("call_tool: {e}"))?;
    assert_eq!(result_mcp.is_error, Some(true), "Expected error for missing window");
    let _ = client.cancel().await;
    Ok(())
}

/// screenshot_burst with a non-existent window returns an error result.
#[tokio::test]
async fn test_screenshot_burst_missing_window() -> TestResult {
    let client = connected_client().await?;
    let result_mcp = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("screenshot_burst"),
            arguments: json!({
                "window_title": "nonexistent-window-xyz-12345",
                "count": 3,
            })
            .as_object().cloned(),
        })
        .await
        .map_err(|e| format!("call_tool: {e}"))?;
    assert_eq!(result_mcp.is_error, Some(true), "Expected error for missing window");
    let _ = client.cancel().await;
    Ok(())
}

/// click_at with a non-existent window returns an error result.
#[tokio::test]
async fn test_click_at_missing_window() -> TestResult {
    let client = connected_client().await?;
    let result_mcp = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("click_at"),
            arguments: json!({
                "window_title": "nonexistent-window-xyz-12345",
                "x": 10,
                "y": 20,
            })
            .as_object().cloned(),
        })
        .await
        .map_err(|e| format!("call_tool: {e}"))?;
    assert_eq!(result_mcp.is_error, Some(true), "Expected error for missing window");
    let _ = client.cancel().await;
    Ok(())
}

/// crop_region with a non-existent window returns an error result.
#[tokio::test]
async fn test_crop_region_missing_window() -> TestResult {
    let client = connected_client().await?;
    let result_mcp = client
        .call_tool(CallToolRequestParam {
            name: Cow::Borrowed("crop_region"),
            arguments: json!({
                "window_title": "nonexistent-window-xyz-12345",
                "x": 0,
                "y": 0,
                "width": 50,
                "height": 50,
            })
            .as_object().cloned(),
        })
        .await
        .map_err(|e| format!("call_tool: {e}"))?;
    assert_eq!(result_mcp.is_error, Some(true), "Expected error for missing window");
    let _ = client.cancel().await;
    Ok(())
}
