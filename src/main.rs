use clap::{Parser, ValueEnum};
use qrforge::{generate, PrintMode, QrOptions};

#[derive(Parser)]
#[command(name = "qrforge", about = "Generate QR codes from the terminal")]
struct Cli {
    /// The text or URL to encode
    data: String,

    /// Save the QR code as a PNG file
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Print the QR code in the terminal
    #[arg(short, long, value_name = "MODE")]
    print: Option<PrintArg>,
}

#[derive(ValueEnum, Clone)]
enum PrintArg {
    Ascii,
    Png,
}

fn main() {
    let cli = Cli::parse();

    let print_mode = cli.print.map(|p| match p {
        PrintArg::Ascii => PrintMode::Ascii,
        PrintArg::Png => PrintMode::Png,
    });

    let opts = QrOptions {
        data: cli.data,
        output: cli.output,
        print_mode,
    };

    if let Err(e) = generate(opts) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
