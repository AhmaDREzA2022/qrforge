use crate::error::QrError;
use crate::validation::{validate_data, EccLevel};
use qrcode::{EcLevel, QrCode};

impl From<EccLevel> for EcLevel {
    fn from(ecc: EccLevel) -> Self {
        match ecc {
            EccLevel::L => EcLevel::L,
            EccLevel::M => EcLevel::M,
            EccLevel::Q => EcLevel::Q,
            EccLevel::H => EcLevel::H,
        }
    }
}

pub fn generate_code(data: &str, ecc: EccLevel) -> Result<QrCode, QrError> {
    validate_data(data, ecc)?;

    QrCode::with_error_correction_level(data, ecc.into())
        .map_err(|e| QrError::QrGeneration(e.to_string()))
}
