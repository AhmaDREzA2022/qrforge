pub mod error;
pub use error::QrError;

use std::char;

use image::Luma;
use qrcode::QrCode;

pub enum PrintMode {
    Ascii,
    Png,
}

pub struct QrOptions {
    pub data: String,
    pub output: Option<String>,
    pub print_mode: Option<PrintMode>,
}

pub fn generate(opts: QrOptions) -> Result<(), Box<dyn std::error::Error>> {
    let code = QrCode::new(opts.data.as_bytes())?;

    if let Some(ref path) = opts.output {
        let image = code.render::<Luma<u8>>().build();
        image.save(path)?;
        println!("Saved to {path}");
    }

    if let Some(ref mode) = opts.print_mode {
        match mode {
            PrintMode::Ascii => {
                let ascii = code
                    .render::<char>()
                    .quiet_zone(true)
                    .module_dimensions(2, 1)
                    .build();
                println!("{ascii}");
            }
            PrintMode::Png => {
                let image = code.render::<Luma<u8>>().build();
                let conf = viuer::Config {
                    use_kitty: true,
                    absolute_offset: false,
                    ..Default::default()
                };
                viuer::print(&image.into(), &conf)?;
            }
        }
    }

    Ok(())
}
