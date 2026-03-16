/// Shared error type for all MCP tool operations.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum AppError_x {
    /// Win32 window not found by title.
    WindowNotFound(String),
    /// Win32 capture operation failed.
    CaptureFailed(String),
    /// Template image decode or size error.
    ImageError(String),
    /// Template not found above confidence threshold.
    TemplateNotFound { confidence: f32, threshold: f32 },
    /// Input device error from enigo.
    InputError(String),
}

impl std::fmt::Display for AppError_x {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WindowNotFound(t) => write!(f, "Window not found: {t}"),
            Self::CaptureFailed(e) => write!(f, "Capture failed: {e}"),
            Self::ImageError(e) => write!(f, "Image error: {e}"),
            Self::TemplateNotFound { confidence, threshold } => {
                write!(f, "Template not found (best={confidence:.3}, need={threshold:.3})")
            }
            Self::InputError(e) => write!(f, "Input error: {e}"),
        }
    }
}
