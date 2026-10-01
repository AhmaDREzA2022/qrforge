use super::PngOpts;
use crate::error::QrError;
use image::{DynamicImage, ImageBuffer, Rgb};
use qrcode::QrCode;

pub fn to_png(code: &QrCode, opts: PngOpts) -> Result<DynamicImage, QrError> {
    let matrix = code.to_colors();
    let width = code.width();

    let quiet = opts.quiet_zone;
    let scale = opts.scale;

    let (fg, bg) = if opts.invert {
        (opts.colors.bg, opts.colors.fg)
    } else {
        (opts.colors.fg, opts.colors.bg)
    };

    let img_size = (width as u32 + quiet * 2) * scale;

    let mut img = ImageBuffer::from_fn(img_size, img_size, |_, _| Rgb(bg));

    for (i, colors) in matrix.iter().enumerate() {
        let row = (i / width) as u32;
        let col = (i % width) as u32;

        if *colors == qrcode::Color::Dark {
            let x = (col + quiet) * scale;
            let y = (row + quiet) * scale;

            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel(x + dx, y + dy, Rgb(fg))
                }
            }
        }
    }

    Ok(DynamicImage::ImageRgb8(img))
}
