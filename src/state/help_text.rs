/// Help text constants for the `help` MCP tool.

/// Default help -- tool overview.
pub const HELP_TOOLS: &str = "\
gui-mcp -- Windows GUI vision and control for AI assistants

TOOLS (15):

  Vision
    list_windows              List all visible windows with titles
    get_window_info(title)    Get title, rect, and dimensions
    screenshot_window(title)  Capture window as base64 PNG
    screenshot_burst(title, count?)  Rapid multi-frame capture (max 10)

  Find & Click
    find_element(title, template_base64, confidence?)  Template match, returns coords
    click_element(title, template_base64, confidence?, button?)  Template match + click
    click_at(title, x, y, button?)  Click at pixel coordinates

  Text & Keys
    type_text(title, text)    Type text (Unicode, supports \\n)
    send_keys(title, keys)    Key combo (e.g. ctrl+s, enter, alt+f4)

  Clipboard
    select_all(title)         Select all text in edit control
    copy(title)               Copy selection to clipboard
    cut(title)                Cut selection to clipboard
    paste(title)              Paste clipboard into edit control

  Image
    crop_region(title, x, y, width, height)  Crop region as base64 PNG

  Help
    help(topic?)              This help. Topics: tools, keys, mimic, workflow

Use help(topic) for details on a specific topic.";

/// Key names accepted by send_keys.
pub const HELP_KEYS: &str = "\
send_keys -- key names and combos

MODIFIERS (prefix with +):
  ctrl, control, alt, shift, win, meta, super

SPECIAL KEYS:
  enter, return       Enter/Return
  escape, esc         Escape
  tab                 Tab
  backspace           Backspace
  delete, del         Delete
  home, end           Home / End
  pageup, pagedown    Page Up / Page Down
  up, down, left, right   Arrow keys
  space               Space
  f1..f12             Function keys

SINGLE CHARACTERS:
  a..z, 0..9          Sent as their VK code (case-insensitive)

COMBOS (join with +):
  ctrl+s              Save
  ctrl+shift+z        Redo
  alt+f4              Close window
  ctrl+a              Select all

EXAMPLES:
  send_keys(title, \"ctrl+c\")     Copy
  send_keys(title, \"alt+tab\")    Switch window
  send_keys(title, \"Return\")     Press Enter
  send_keys(title, \"a\")          Press A";

/// Mimic scripting guide.
pub const HELP_MIMIC: &str = "\
gui-mcp -- mimic automation (scripting without AI)

gui-mcp is both an MCP server and a Rust library. The adapter layer
can be called directly from Rust to replay GUI workflows without AI.

RUST DEPENDENCY:
  [dependencies]
  slint-gui-mcp = { path = \"../gui-mcp\" }

ADAPTER API:
  use slint_gui_mcp::adapter::app_adp;

  app_adp::list_windows()                          -> Result<String>
  app_adp::get_window_info(title)                  -> Result<String>
  app_adp::screenshot_window(title)                -> Result<(b64, w, h, title)>
  app_adp::screenshot_burst(title, count)          -> Result<Vec<(b64, w, h)>>
  app_adp::find_element(title, tpl, conf)          -> Result<Option<(x,y,w,h,conf)>>
  app_adp::click_at_adp(title, x, y, button)       -> Result<()>
  app_adp::click_element(title, tpl, conf, btn)    -> Result<(x,y,conf,after_png)>
  app_adp::focused_type_text(title, text)          -> Result<()>
  app_adp::focused_send_keys(title, keys)          -> Result<()>
  app_adp::select_all_adp(title)                   -> Result<()>
  app_adp::copy_adp(title) / cut_adp / paste_adp   -> Result<()>
  app_adp::crop_region_adp(title, x,y,w,h)        -> Result<String>

EXAMPLE:
  use slint_gui_mcp::adapter::app_adp;

  fn main() -> Result<(), Box<dyn std::error::Error>> {
      let (b64, w, h, _) = app_adp::screenshot_window(\"My App\")?;
      app_adp::click_at_adp(\"My App\", 150, 200, \"left\")?;
      app_adp::focused_type_text(\"My App\", \"Hello!\")?;
      app_adp::focused_send_keys(\"My App\", \"ctrl+s\")?;
      Ok(())
  }";

/// The mimic workflow concept.
pub const HELP_WORKFLOW: &str = "\
gui-mcp -- the mimic workflow

  1. EXPLORE WITH AI
     Use gui-mcp via Claude Code / Codex to interactively figure out
     coordinates, templates, and the right sequence of clicks and keys.

  2. CAPTURE THE STEPS
     Note the tool calls the AI made: window titles, coordinates, keys,
     and any templates (crop_region output).

  3. SCRIPT IT IN RUST
     Translate those calls into app_adp::* function calls in a Rust
     binary or script.

  4. RUN DETERMINISTICALLY
     cargo run --bin my_workflow

This gives AI-assisted discovery with script-speed replay --
no LLM latency in the automation loop.

NOTES:
  - Input tools only work with what the target app implements.
    click_at requires a clickable surface (e.g. TouchArea in Slint).
    send_keys requires keyboard focus handling (e.g. FocusScope in Slint).
  - Coordinates match the screenshot output (0,0 = top-left including title bar).
  - Templates from crop_region are base64 PNG -- store them as files for reuse.";

/// Returns the help text for a given topic, or the default overview.
pub fn help_for_topic(topic: Option<&str>) -> &'static str {
    match topic.map(|s| s.trim().to_ascii_lowercase()).as_deref() {
        None | Some("") | Some("tools") => HELP_TOOLS,
        Some("keys") | Some("key") | Some("send_keys") => HELP_KEYS,
        Some("mimic") | Some("script") | Some("scripting") | Some("api") => HELP_MIMIC,
        Some("workflow") | Some("flow") => HELP_WORKFLOW,
        _ => HELP_TOOLS,
    }
}
