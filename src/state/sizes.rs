/// Bits per pixel used when capturing window bitmaps (BGRA = 32 bpp).
pub const CAPTURE_BPP: i32 = 32;

/// Bytes per pixel for BGRA capture buffers.
pub const CAPTURE_BYTES_PER_PIXEL: usize = 4;

/// Number of colour channels in an RGB image (R, G, B).
pub const RGB_CHANNELS: u32 = 3;

/// Divisor used to compute the centre offset of a template region.
pub const CENTER_DIVISOR: u32 = 2;

/// Maximum number of UTF-16 code units for a window title buffer.
pub const WINDOW_TITLE_BUF: usize = 512;

/// Extra UTF-16 code units added to GetWindowTextLengthW result (null terminator + guard byte).
pub const TITLE_BUF_PADDING: usize = 2;

/// Byte index of the Red channel within a BGRA pixel (index 2).
pub const BGRA_R_IDX: usize = 2;

/// Byte index of the Green channel within a BGRA pixel (index 1).
pub const BGRA_G_IDX: usize = 1;

/// Byte index of the Blue channel within a BGRA pixel (index 0).
pub const BGRA_B_IDX: usize = 0;

/// Milliseconds to wait after SetForegroundWindow before sending input.
pub const FOCUS_DELAY_MS: u64 = 100;

/// PrintWindow flag: PW_RENDERFULLCONTENT  --  captures background/hidden windows.
pub const PW_FULL_CONTENT: u32 = 2;

/// Milliseconds to sleep between key-down and key-up messages.
pub const KEY_EVENT_DELAY_MS: u64 = 10;

/// WM_CHAR message ID.
pub const WM_CHAR_ID: u32 = 0x0102;

/// WM_KEYDOWN message ID.
pub const WM_KEYDOWN_ID: u32 = 0x0100;

/// WM_KEYUP message ID.
pub const WM_KEYUP_ID: u32 = 0x0101;

/// WM_LBUTTONDOWN message ID.
pub const WM_LBUTTONDOWN_ID: u32 = 0x0201;

/// WM_LBUTTONUP message ID.
pub const WM_LBUTTONUP_ID: u32 = 0x0202;

/// WM_RBUTTONDOWN message ID.
pub const WM_RBUTTONDOWN_ID: u32 = 0x0204;

/// WM_RBUTTONUP message ID.
pub const WM_RBUTTONUP_ID: u32 = 0x0205;

/// Shift for y-coordinate in LPARAM for mouse messages.
pub const MOUSE_LPARAM_Y_SHIFT: i32 = 16;

/// Mask for coordinate packing in mouse LPARAM.
pub const MOUSE_COORD_MASK: i32 = 0xFFFF;

/// Buffer size for GetClassNameW result.
pub const CLASS_NAME_BUF: usize = 256;

/// Bit position of scan code in WM_KEYDOWN/UP lParam.
pub const LPARAM_SCANCODE_SHIFT: u32 = 16;

/// Bit position of previous key state in WM_KEYDOWN/UP lParam.
pub const LPARAM_PREV_STATE_BIT: u32 = 30;

/// Bit position of transition state in WM_KEYUP lParam.
pub const LPARAM_TRANSITION_BIT: u32 = 31;

/// EM_SETSEL message ID  --  selects text range in an edit control.
pub const EM_SETSEL_ID: u32 = 0x00B1;

/// WM_COPY message ID  --  copies selection to clipboard.
pub const WM_COPY_ID: u32 = 0x0301;

/// WM_CUT message ID  --  cuts selection to clipboard.
pub const WM_CUT_ID: u32 = 0x0300;

/// WM_PASTE message ID  --  pastes clipboard into control.
pub const WM_PASTE_ID: u32 = 0x0302;

/// Milliseconds to sleep between sequential screenshots for near-live capture.
pub const SCREENSHOT_INTERVAL_MS: u64 = 200;

/// Default number of frames in a screenshot burst.
pub const BURST_DEFAULT_COUNT: usize = 5;

/// Maximum number of frames in a screenshot burst.
pub const BURST_MAX_COUNT: usize = 10;

/// Content entries per frame in burst result (image + text label).
pub const BURST_CONTENT_PER_FRAME: usize = 2;
