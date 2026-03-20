# src/state/sizes.rs

## `pub const CAPTURE_BPP: i32 = 32;`

*Line 2 · const*

Bits per pixel used when capturing window bitmaps (BGRA = 32 bpp).

---

## `pub const CAPTURE_BYTES_PER_PIXEL: usize = 4;`

*Line 5 · const*

Bytes per pixel for BGRA capture buffers.

---

## `pub const RGB_CHANNELS: u32 = 3;`

*Line 8 · const*

Number of colour channels in an RGB image (R, G, B).

---

## `pub const CENTER_DIVISOR: u32 = 2;`

*Line 11 · const*

Divisor used to compute the centre offset of a template region.

---

## `pub const WINDOW_TITLE_BUF: usize = 512;`

*Line 14 · const*

Maximum number of UTF-16 code units for a window title buffer.

---

## `pub const TITLE_BUF_PADDING: usize = 2;`

*Line 17 · const*

Extra UTF-16 code units added to GetWindowTextLengthW result (null terminator + guard byte).

---

## `pub const BGRA_R_IDX: usize = 2;`

*Line 20 · const*

Byte index of the Red channel within a BGRA pixel (index 2).

---

## `pub const BGRA_G_IDX: usize = 1;`

*Line 23 · const*

Byte index of the Green channel within a BGRA pixel (index 1).

---

## `pub const BGRA_B_IDX: usize = 0;`

*Line 26 · const*

Byte index of the Blue channel within a BGRA pixel (index 0).

---

## `pub const FOCUS_DELAY_MS: u64 = 100;`

*Line 29 · const*

Milliseconds to wait after SetForegroundWindow before sending input.

---

## `pub const PW_FULL_CONTENT: u32 = 2;`

*Line 32 · const*

PrintWindow flag: PW_RENDERFULLCONTENT  --  captures background/hidden windows.

---

## `pub const KEY_EVENT_DELAY_MS: u64 = 10;`

*Line 35 · const*

Milliseconds to sleep between key-down and key-up messages.

---

## `pub const WM_CHAR_ID: u32 = 0x0102;`

*Line 38 · const*

WM_CHAR message ID.

---

## `pub const WM_KEYDOWN_ID: u32 = 0x0100;`

*Line 41 · const*

WM_KEYDOWN message ID.

---

## `pub const WM_KEYUP_ID: u32 = 0x0101;`

*Line 44 · const*

WM_KEYUP message ID.

---

## `pub const WM_LBUTTONDOWN_ID: u32 = 0x0201;`

*Line 47 · const*

WM_LBUTTONDOWN message ID.

---

## `pub const WM_LBUTTONUP_ID: u32 = 0x0202;`

*Line 50 · const*

WM_LBUTTONUP message ID.

---

## `pub const WM_RBUTTONDOWN_ID: u32 = 0x0204;`

*Line 53 · const*

WM_RBUTTONDOWN message ID.

---

## `pub const WM_RBUTTONUP_ID: u32 = 0x0205;`

*Line 56 · const*

WM_RBUTTONUP message ID.

---

## `pub const MOUSE_LPARAM_Y_SHIFT: i32 = 16;`

*Line 59 · const*

Shift for y-coordinate in LPARAM for mouse messages.

---

## `pub const MOUSE_COORD_MASK: i32 = 0xFFFF;`

*Line 62 · const*

Mask for coordinate packing in mouse LPARAM.

---

## `pub const CLASS_NAME_BUF: usize = 256;`

*Line 65 · const*

Buffer size for GetClassNameW result.

---

## `pub const LPARAM_SCANCODE_SHIFT: u32 = 16;`

*Line 68 · const*

Bit position of scan code in WM_KEYDOWN/UP lParam.

---

## `pub const LPARAM_PREV_STATE_BIT: u32 = 30;`

*Line 71 · const*

Bit position of previous key state in WM_KEYDOWN/UP lParam.

---

## `pub const LPARAM_TRANSITION_BIT: u32 = 31;`

*Line 74 · const*

Bit position of transition state in WM_KEYUP lParam.

---

## `pub const EM_SETSEL_ID: u32 = 0x00B1;`

*Line 77 · const*

EM_SETSEL message ID  --  selects text range in an edit control.

---

## `pub const WM_COPY_ID: u32 = 0x0301;`

*Line 80 · const*

WM_COPY message ID  --  copies selection to clipboard.

---

## `pub const WM_CUT_ID: u32 = 0x0300;`

*Line 83 · const*

WM_CUT message ID  --  cuts selection to clipboard.

---

## `pub const WM_PASTE_ID: u32 = 0x0302;`

*Line 86 · const*

WM_PASTE message ID  --  pastes clipboard into control.

---

## `pub const SCREENSHOT_INTERVAL_MS: u64 = 200;`

*Line 89 · const*

Milliseconds to sleep between sequential screenshots for near-live capture.

---

## `pub const BURST_DEFAULT_COUNT: usize = 5;`

*Line 92 · const*

Default number of frames in a screenshot burst.

---

## `pub const BURST_MAX_COUNT: usize = 10;`

*Line 95 · const*

Maximum number of frames in a screenshot burst.

---

## `pub const BURST_CONTENT_PER_FRAME: usize = 2;`

*Line 98 · const*

Content entries per frame in burst result (image + text label).

---

## `pub const EXPECTED_TOOL_COUNT: usize = 14;`

*Line 101 · const*

Total number of MCP tools registered by the server.

---

## `pub const CROP_FIND_MAX_DIM: u32 = 200;`

*Line 104 · const*

Maximum haystack dimension for NCC search in crop-then-find tests (debug-safe).

---

## `pub const TEST_PIXEL_R_X: u32 = 17;`

*Line 107 · const*

Test pixel pattern coefficients for deterministic image generation.

---

## `pub const TEST_PIXEL_R_Y: u32 = 3;`

*Line 109 · const*

Test pixel pattern coefficient for R channel Y component.

---

## `pub const TEST_PIXEL_R_MOD: u32 = 251;`

*Line 111 · const*

Test pixel pattern modulus for R channel.

---

## `pub const TEST_PIXEL_G_X: u32 = 7;`

*Line 113 · const*

Test pixel pattern coefficient for G channel X component.

---

## `pub const TEST_PIXEL_G_Y: u32 = 19;`

*Line 115 · const*

Test pixel pattern coefficient for G channel Y component.

---

## `pub const TEST_PIXEL_G_OFF: u32 = 11;`

*Line 117 · const*

Test pixel pattern offset for G channel.

---

## `pub const TEST_PIXEL_G_MOD: u32 = 241;`

*Line 119 · const*

Test pixel pattern modulus for G channel.

---

## `pub const TEST_PIXEL_B_X: u32 = 13;`

*Line 121 · const*

Test pixel pattern coefficient for B channel X component.

---

## `pub const TEST_PIXEL_B_Y: u32 = 5;`

*Line 123 · const*

Test pixel pattern coefficient for B channel Y component.

---

## `pub const TEST_PIXEL_B_OFF: u32 = 23;`

*Line 125 · const*

Test pixel pattern offset for B channel.

---

## `pub const TEST_PIXEL_B_MOD: u32 = 239;`

*Line 127 · const*

Test pixel pattern modulus for B channel.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=7" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
