use super::AsciiOpts;
use crate::error::QrError;
use qrcode::render::unicode;
use qrcode::QrCode;

pub fn to_ascii(code: &QrCode, opts: AsciiOpts) -> Result<String, QrError> {
    let fg = if opts.invert {
        unicode::Dense1x2::Light
    } else {
        unicode::Dense1x2::Dark
    };

    let bg = if opts.invert {
        unicode::Dense1x2::Dark
    } else {
        unicode::Dense1x2::Light
    };

    let result = code
        .render::<unicode::Dense1x2>()
        .dark_color(fg)
        .light_color(bg)
        .quiet_zone(opts.quiet_zone)
        .build();

    Ok(result)
}
