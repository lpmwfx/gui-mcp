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
