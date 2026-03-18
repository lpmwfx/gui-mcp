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

/// SendInput absolute coordinate range (0..65535).
pub const SENDINPUT_COORD_RANGE: i64 = 65536;

/// Buffer size for GetClassNameW result.
pub const CLASS_NAME_BUF: usize = 256;

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

/// Total number of MCP tools registered by the server.
pub const EXPECTED_TOOL_COUNT: usize = 15;

/// Maximum haystack dimension for NCC search in crop-then-find tests (debug-safe).
pub const CROP_FIND_MAX_DIM: u32 = 200;

/// Test pixel pattern coefficients for deterministic image generation.
pub const TEST_PIXEL_R_X: u32 = 17;
/// Test pixel pattern coefficient for R channel Y component.
pub const TEST_PIXEL_R_Y: u32 = 3;
/// Test pixel pattern modulus for R channel.
pub const TEST_PIXEL_R_MOD: u32 = 251;
/// Test pixel pattern coefficient for G channel X component.
pub const TEST_PIXEL_G_X: u32 = 7;
/// Test pixel pattern coefficient for G channel Y component.
pub const TEST_PIXEL_G_Y: u32 = 19;
/// Test pixel pattern offset for G channel.
pub const TEST_PIXEL_G_OFF: u32 = 11;
/// Test pixel pattern modulus for G channel.
pub const TEST_PIXEL_G_MOD: u32 = 241;
/// Test pixel pattern coefficient for B channel X component.
pub const TEST_PIXEL_B_X: u32 = 13;
/// Test pixel pattern coefficient for B channel Y component.
pub const TEST_PIXEL_B_Y: u32 = 5;
/// Test pixel pattern offset for B channel.
pub const TEST_PIXEL_B_OFF: u32 = 23;
/// Test pixel pattern modulus for B channel.
pub const TEST_PIXEL_B_MOD: u32 = 239;
