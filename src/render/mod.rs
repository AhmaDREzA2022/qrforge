pub mod ascii;
pub mod png;
pub mod svg;

#[derive(Debug, Clone)]
pub struct ColorOptions {
    pub fg: [u8; 3],
    pub bg: [u8; 3],
}

impl Default for ColorOptions {
    fn default() -> Self {
        Self {
            fg: [0, 0, 0],       // black
            bg: [255, 255, 255], // white
        }
    }
}

#[derive(Debug, Clone)]
pub struct PngOpts {
    pub scale: u32,
    pub quiet_zone: u32,
    pub colors: ColorOptions,
    pub invert: bool,
}

impl Default for PngOpts {
    fn default() -> Self {
        Self {
            scale: 10,
            quiet_zone: 4,
            colors: ColorOptions::default(),
            invert: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AsciiOpts {
    pub quiet_zone: bool,
    pub invert: bool,
}

impl Default for AsciiOpts {
    fn default() -> Self {
        Self {
            quiet_zone: true,
            invert: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SvgOpts {
    pub scale: u32,
    pub quiet_zone: u32,
    pub colors: ColorOptions,
    pub invert: bool,
}

impl Default for SvgOpts {
    fn default() -> Self {
        Self {
            scale: 10,
            quiet_zone: 4,
            colors: ColorOptions::default(),
            invert: false,
        }
    }
}
