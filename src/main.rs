use clap::{Parser, ValueEnum};
use qrforge::{generate, parse_hex_color, ColorOptions, EccLevel, PrintMode, QrOptions};

#[derive(Parser)]
#[command(name = "qrforge", about = "Generate QR codes from the terminal")]
struct Cli {
    /// The text or URL to encode
    data: String,

    /// Save the QR code to a file (.png or .svg)
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Print the QR code in the terminal
    #[arg(short, long, value_name = "MODE")]
    print: Option<PrintArg>,

    /// Error correction level
    #[arg(short, long, value_name = "LEVEL", default_value = "m")]
    ecc: EccArg,

    /// Scale factor for PNG/SVG output (pixels per module)
    #[arg(short, long, default_value = "10")]
    scale: u32,

    /// Quiet zone size in modules
    #[arg(short, long, default_value = "4")]
    quiet_zone: u32,

    /// Foreground color as hex (e.g. #000000)
    #[arg(long, default_value = "#000000")]
    fg: String,

    /// Background color as hex (e.g. #ffffff)
    #[arg(long, default_value = "#ffffff")]
    bg: String,

    /// Invert foreground and background colors
    #[arg(long, default_value = "false")]
    invert: bool,
}

#[derive(ValueEnum, Clone)]
enum PrintArg {
    Ascii,
    Png,
    Svg,
}

#[derive(ValueEnum, Clone)]
enum EccArg {
    L,
    M,
    Q,
    H,
}

fn main() {
    let cli = Cli::parse();

    let fg = match parse_hex_color(&cli.fg) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let bg = match parse_hex_color(&cli.bg) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let ecc = match cli.ecc {
        EccArg::L => EccLevel::L,
        EccArg::M => EccLevel::M,
        EccArg::Q => EccLevel::Q,
        EccArg::H => EccLevel::H,
    };

    let print_mode = cli.print.map(|p| match p {
        PrintArg::Ascii => PrintMode::Ascii,
        PrintArg::Png => PrintMode::Png,
        PrintArg::Svg => PrintMode::Svg,
    });

    let opts = QrOptions {
        data: cli.data,
        output: cli.output,
        print_mode,
        ecc,
        scale: cli.scale,
        quiet_zone: cli.quiet_zone,
        colors: ColorOptions { fg, bg },
        invert: cli.invert,
    };

    if let Err(e) = generate(opts) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
