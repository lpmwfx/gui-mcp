/// Application adapter — coordinates PAL and Core for each MCP tool request.
mod click_adp;
mod find_adp;
mod focus_adp;
mod helpers_adp;
mod info_adp;
mod screenshot_adp;

pub use click_adp::click_element;
pub use find_adp::find_element;
pub use focus_adp::{focused_type_text, focused_send_keys};
pub use info_adp::{get_window_info, list_windows};
pub use screenshot_adp::screenshot_window;
pub(crate) use helpers_adp::{
    after_screenshot_adp, decode_template_adp, encode_png, find_and_capture_adp,
};
