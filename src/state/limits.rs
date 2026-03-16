/// Default confidence threshold for template matching (NCC score 0.0–1.0).
pub const DEFAULT_CONFIDENCE: f32 = 0.8;

/// Minimum NCC standard deviation — below this a region is considered flat/uniform.
pub const NCC_STD_FLOOR: f32 = 1e-6;
