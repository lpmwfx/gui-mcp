# gui-mcp

A Windows MCP server that gives AI assistants vision and control of desktop GUI windows.

Built in Rust as a single binary with zero runtime dependencies. Works with Claude Code, Codex, and any MCP-compatible client.

## What it does

AI assistants can't see your screen. gui-mcp fixes that by exposing 14 tools over the [Model Context Protocol](https://modelcontextprotocol.io/) that let an AI:

- **See** windows via screenshots and rapid burst capture
- **Find** UI elements via image template matching (NCC)
- **Click** elements by template or by coordinates
- **Type** text and send keyboard shortcuts
- **Read/write** clipboard content
- **Crop** screen regions to build reusable template libraries

All operations work on **background windows** -- no foreground stealing, no focus switching.

## Tools

| Tool | Description |
|------|-------------|
| `list_windows` | List all visible windows with titles |
| `get_window_info` | Get title, rect, and dimensions for a window |
| `screenshot_window` | Capture a window as base64 PNG |
| `screenshot_burst` | Rapid multi-frame capture (up to 10 frames) |
| `find_element` | Find a UI element via image template matching |
| `click_element` | Find and click a UI element via template matching |
| `click_at` | Click at pixel coordinates (no template needed) |
| `crop_region` | Crop a region from a window screenshot as base64 PNG |
| `type_text` | Type text into a window |
| `send_keys` | Send key combinations (e.g. `ctrl+s`, `enter`) |
| `select_all` | Select all text in an edit control |
| `copy` | Copy selection to clipboard |
| `cut` | Cut selection to clipboard |
| `paste` | Paste clipboard into an edit control |

## Install

### Build from source

```
git clone https://github.com/lpmwfx/gui-mcp.git
cd gui-mcp
cargo build --release
cp target/release/slint-gui-mcp.exe ~/bin/gui-mcp.exe
```

### Configure Claude Code

Add to `~/.claude/settings.json`:

```json
{
  "mcpServers": {
    "gui-mcp": {
      "command": "C:/Users/YOU/bin/gui-mcp.exe",
      "args": [],
      "type": "stdio"
    }
  }
}
```

### Configure Codex

Add to your Codex MCP config:

```json
{
  "gui-mcp": {
    "command": "C:/Users/YOU/bin/gui-mcp.exe",
    "args": []
  }
}
```

## Usage examples

### Screenshot a window

```
screenshot_window(window_title: "Notepad")
```

Returns a base64 PNG of the window contents.

### Find and click a button

```
// First, crop a button from a screenshot to use as template
crop_region(window_title: "My App", x: 100, y: 200, width: 80, height: 30)

// Then use that template to find and click the button
click_element(window_title: "My App", template_base64: "<cropped png>")
```

### Click at known coordinates

```
click_at(window_title: "My App", x: 150, y: 215)
```

### Type and send keys

```
type_text(window_title: "Notepad", text: "Hello, world!")
send_keys(window_title: "Notepad", keys: "ctrl+s")
```

### Build a template library

The crop-then-find workflow lets AI assistants build reusable UI element datasets:

1. `screenshot_window` -- capture the full window
2. `crop_region` -- extract buttons, icons, labels as templates
3. `find_element` -- locate those templates in future screenshots
4. `click_element` -- click matched elements

This enables robust GUI automation that adapts to layout changes.

## Scripting & mimic automation

gui-mcp is an MCP server, but it's also a Rust library. You can use the adapter
layer directly to script GUI interactions without AI -- turning an AI-discovered
workflow into a deterministic automation script.

### As a Rust library

Add gui-mcp as a dependency and call the adapter API directly:

```rust
use slint_gui_mcp::adapter::app_adp;

fn main() {
    // Screenshot
    let (b64, w, h, _) = app_adp::screenshot_window("My App").unwrap();

    // Click at coordinates
    app_adp::click_at_adp("My App", 150, 200, "left").unwrap();

    // Type text
    app_adp::focused_type_text("My App", "Hello!").unwrap();

    // Send key combo
    app_adp::focused_send_keys("My App", "ctrl+s").unwrap();
}
```

### The mimic workflow

1. **Explore with AI** -- use gui-mcp via Claude Code to interactively figure out
   coordinates, templates, and the right sequence of clicks/keys
2. **Capture the steps** -- note the tool calls the AI made (window titles, coords, keys)
3. **Script it in Rust** -- translate those calls into `app_adp::*` function calls
4. **Run deterministically** -- `cargo run --bin my_workflow`

This gives you AI-assisted discovery with script-speed replay -- no LLM latency in
the automation loop.

### Available adapter functions

| Function | Description |
|----------|-------------|
| `list_windows()` | List visible window titles as JSON |
| `get_window_info(title)` | Get rect, dimensions as JSON |
| `screenshot_window(title)` | Capture as base64 PNG |
| `screenshot_burst(title, count)` | Multi-frame capture |
| `find_element(title, template_b64, confidence)` | Template match |
| `click_at_adp(title, x, y, button)` | Click at coordinates |
| `click_element(title, template_b64, confidence, button)` | Template click |
| `focused_type_text(title, text)` | Type text |
| `focused_send_keys(title, keys)` | Send key combo |
| `select_all_adp(title)` | Select all |
| `copy_adp(title)` / `cut_adp(title)` / `paste_adp(title)` | Clipboard ops |
| `crop_region_adp(title, x, y, w, h)` | Crop region |

## Architecture

Six-layer hexagonal topology with strict one-way imports:

```
src/
  ui/          -- MCP server (tool definitions, rmcp)
  adapter/     -- orchestration (coordinates core + pal)
  core/        -- pure logic (NCC matching, image cropping)
  pal/         -- platform (Win32 HWND, GDI capture, PostMessage)
  shared/      -- cross-cutting (error types)
  state/       -- constants (thresholds, timings, message IDs)
```

## Requirements

- Windows 10/11
- Rust stable (edition 2021)
- A live desktop session (not headless)

## Known limitations

- `PrintWindow` can fail for GPU-accelerated, minimized, or non-standard windows
- Partial title matching can hit the wrong window if titles overlap
- Coordinates may drift on high-DPI or mixed-scaling setups
- NCC template matching is CPU-bound; large screenshots with small templates can be slow
- **Input tools only work with what the target application implements.** `click_at` requires the app to have a clickable surface (e.g. a `TouchArea` in Slint). `send_keys` requires the app to have keyboard focus handling (e.g. a `FocusScope` in Slint). If the app doesn't handle a particular input event, the tool call will succeed but the app won't react.

## License

[EUPL-1.2](LICENSE) -- European Union Public Licence v. 1.2


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=7" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
