use crate::QrError;

// Maximum bytes per ECC level for QR version 40 (highest capacity)
const MAX_BYTES_L: usize = 2953;
const MAX_BYTES_M: usize = 2331;
const MAX_BYTES_Q: usize = 1663;
const MAX_BYTES_H: usize = 1273;

#[derive(Debug, Clone, Copy)]
pub enum EccLevel {
    L, // Low        ~7% damage recovery
    M, // Medium    ~15% damage recovery
    Q, // Quartile  ~25% damage recovery
    H, // High      ~30% damage recovery
}

impl EccLevel {
    pub fn max_bytes(&self) -> usize {
        match self {
            EccLevel::L => MAX_BYTES_L,
            EccLevel::M => MAX_BYTES_M,
            EccLevel::Q => MAX_BYTES_Q,
            EccLevel::H => MAX_BYTES_H,
        }
    }
}

pub fn validate_data(data: &str, ecc: EccLevel) -> Result<(), QrError> {
    if data.is_empty() {
        return Err(QrError::EmptyData);
    }

    let len = data.len();
    let max = ecc.max_bytes();

    if len > max {
        return Err(QrError::DataTooLong { len, max });
    }

    Ok(())
}

pub fn parse_hex_color(hex: &str) -> Result<[u8; 3], QrError> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(QrError::InvalidColor(format!("#{hex}")));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| QrError::InvalidColor(format!("#{hex}")))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| QrError::InvalidColor(format!("#{hex}")))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| QrError::InvalidColor(format!("#{hex}")))?;

    Ok([r, g, b])
}
