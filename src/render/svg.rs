use qrcode::QrCode;
use crate::error::QrError;
use super::SvgOpts;

pub fn to_svg(code: &QrCode, opts: SvgOpts) -> Result<String, QrError> {
    let matrix = code.to_colors();
    let width = code.width();

    let quiet = opts.quiet_zone;
    let scale = opts.scale;

    let (fg, bg) = if opts.invert {
        (opts.colors.bg, opts.colors.fg)
    } else {
        (opts.colors.fg, opts.colors.bg)
    };

    let fg_hex = rgb_to_hex(fg);
    let bg_hex = rgb_to_hex(bg);

    let img_size = (width as u32 + quiet * 2) * scale;

    let mut rects = String::new();

    for (i, color) in matrix.iter().enumerate() {
        if *color == qrcode::Color::Dark {
            let row = (i / width) as u32;
            let col = (i % width) as u32;

            let x = (col + quiet) * scale;
            let y = (row + quiet) * scale;

            rects.push_str(&format!(
                r#"<rect x="{x}" y="{y}" width="{scale}" height="{scale}" fill="#{fg_hex}"/>"#
            ));
        }
    }

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {img_size} {img_size}" width="{img_size}" height="{img_size}">
          <rect width="100%" height="100%" fill="#{bg_hex}"/>
          {rects}
        </svg>"#
    );

    Ok(svg)
}

fn rgb_to_hex(rgb: [u8; 3]) -> String {
    format!("{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
}
