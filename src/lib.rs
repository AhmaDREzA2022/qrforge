pub mod core;
pub mod error;
pub mod render;
pub mod validation;

pub use core::generate_code;
pub use error::QrError;
pub use render::{
    ascii::to_ascii, png::to_png, svg::to_svg, AsciiOpts, ColorOptions, PngOpts, SvgOpts,
};
pub use validation::{parse_hex_color, validate_data, EccLevel};

use image::DynamicImage;
// use qrcode::QrCode;

/// How the QR code should be printed / saved
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintMode {
    Ascii,
    Png, // terminal inline image (viuer)
    Svg, // useful for GUI / files
}

/// High-level options used by the CLI and library consumers
#[derive(Debug, Clone)]
pub struct QrOptions {
    pub data: String,
    pub output: Option<String>, // file path (png or svg)
    pub print_mode: Option<PrintMode>,
    pub ecc: EccLevel,
    pub scale: u32,
    pub quiet_zone: u32,
    pub colors: ColorOptions,
    pub invert: bool,
}

impl Default for QrOptions {
    fn default() -> Self {
        Self {
            data: String::new(),
            output: None,
            print_mode: None,
            ecc: EccLevel::M,
            scale: 10,
            quiet_zone: 4,
            colors: ColorOptions::default(),
            invert: false,
        }
    }
}

/// Main entry point used by the CLI.
/// Generates the QR code and handles printing / saving according to options.
pub fn generate(opts: QrOptions) -> Result<(), QrError> {
    let code = generate_code(&opts.data, opts.ecc)?;

    // --- Save to file ---
    if let Some(ref path) = opts.output {
        if path.ends_with(".svg") {
            let svg = to_svg(
                &code,
                SvgOpts {
                    scale: opts.scale,
                    quiet_zone: opts.quiet_zone,
                    colors: opts.colors.clone(),
                    invert: opts.invert,
                },
            )?;
            std::fs::write(path, svg).map_err(|e| QrError::ImageSave(e.to_string()))?;
        } else {
            // default to PNG
            let img = to_png(
                &code,
                PngOpts {
                    scale: opts.scale,
                    quiet_zone: opts.quiet_zone,
                    colors: opts.colors.clone(),
                    invert: opts.invert,
                },
            )?;
            img.save(path)
                .map_err(|e| QrError::ImageSave(e.to_string()))?;
        }
        println!("Saved to {path}");
    }

    // --- Print to terminal ---
    if let Some(mode) = opts.print_mode {
        match mode {
            PrintMode::Ascii => {
                let ascii = to_ascii(
                    &code,
                    AsciiOpts {
                        quiet_zone: opts.quiet_zone > 0,
                        invert: opts.invert,
                    },
                )?;
                println!("{ascii}");
            }
            PrintMode::Png => {
                let img = to_png(
                    &code,
                    PngOpts {
                        scale: opts.scale,
                        quiet_zone: opts.quiet_zone,
                        colors: opts.colors.clone(),
                        invert: opts.invert,
                    },
                )?;
                print_terminal_image(&img)?;
            }
            PrintMode::Svg => {
                let svg = to_svg(
                    &code,
                    SvgOpts {
                        scale: opts.scale,
                        quiet_zone: opts.quiet_zone,
                        colors: opts.colors.clone(),
                        invert: opts.invert,
                    },
                )?;
                println!("{svg}");
            }
        }
    }

    Ok(())
}

/// Helper: print a DynamicImage in the terminal (Kitty/Ghostty/iTerm2)
fn print_terminal_image(img: &DynamicImage) -> Result<(), QrError> {
    let conf = viuer::Config {
        use_kitty: true,
        absolute_offset: false,
        ..Default::default()
    };
    viuer::print(img, &conf).map_err(|e| QrError::RenderFailed(e.to_string()))?;
    Ok(())
}
