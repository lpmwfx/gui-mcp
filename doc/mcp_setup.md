# MCP Setup

## Purpose
This document explains how to run `slint-gui-mcp` locally and wire it into Codex and Claude as an MCP server.

## Build

Debug build:

```powershell
cargo build
```

Release build:

```powershell
cargo build --release
```

Debug binary path:

```text
D:\REPO\MCP\gui-mcp\target\debug\slint-gui-mcp.exe
```

Release binary path:

```text
D:\REPO\MCP\gui-mcp\target\release\slint-gui-mcp.exe
```

## Codex Setup

Edit `C:\Users\mathi\.codex\config.toml` and add:

```toml
[mcp_servers.gui-mcp]
command = "D:\\REPO\\MCP\\gui-mcp\\target\\debug\\slint-gui-mcp.exe"
```

For release usage, point `command` at `target\\release\\slint-gui-mcp.exe` instead.

After editing the config, restart the Codex session so the new MCP server is loaded.

## Claude Setup

For Claude Desktop style MCP configuration, add an entry like this:

```json
{
  "mcpServers": {
    "gui-mcp": {
      "command": "D:/REPO/MCP/gui-mcp/target/debug/slint-gui-mcp.exe"
    }
  }
}
```

For release usage, switch the command path to:

```text
D:/REPO/MCP/gui-mcp/target/release/slint-gui-mcp.exe
```

Restart Claude after updating its MCP configuration.

## Typical Flow

1. Start the target GUI application on Windows.
2. Call `list_windows` to discover the visible title.
3. Call `screenshot_window` to inspect the current UI.
4. Crop a target region from that screenshot.
5. Call `find_element` or `click_element` with the crop as `template_base64`.
6. Use `type_text` or `send_keys` after focus or click is established.

## Release Notes

- This server is Windows-only.
- GUI capture and input require an interactive desktop session.
- If the debug binary is locked by a running MCP session, build or test against a separate target directory or use the release binary.
