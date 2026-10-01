use std::fmt;

#[derive(Debug)]
pub enum QrError {
    EmptyData,
    DataTooLong { len: usize, max: usize },
    InvalidColor(String),
    QrGeneration(String),
    ImageSave(String),
    RenderFailed(String),
}

impl fmt::Display for QrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QrError::EmptyData => {
                write!(f, "input data cannot be empty")
            }
            QrError::DataTooLong { len, max } => {
                write!(
                    f,
                    "data too long: got {len} bytes, maximum for chosen ECC level is {max}"
                )
            }
            QrError::InvalidColor(s) => {
                write!(f, "invalid color '{s}': expected hex like #ff0000")
            }
            QrError::QrGeneration(s) => {
                write!(f, "QR generation failed: {s}")
            }
            QrError::ImageSave(s) => {
                write!(f, "failed to save image: {s}")
            }
            QrError::RenderFailed(s) => {
                write!(f, "render failed: {s}")
            }
        }
    }
}

impl std::error::Error for QrError {}
