# qrforge

A fast QR code generator for the terminal, built with Rust.

## Features

- Print QR codes as ASCII art in any terminal
- Print QR codes as inline images (Kitty/Ghostty protocol)
- Save QR codes as PNG files
- Flags can be combined

## Installation

Requires [Rust](https://rustup.rs/) installed.

```bash
git clone https://github.com/AhmaDREzA2022/qrforge.git
cd qrforge
cargo build --release
```

The binary will be at `target/release/qrforge`.

## Usage

```bash
# Print as ASCII
qrforge "https://example.com" --print ascii

# Print as inline image (Ghostty, Kitty, iTerm2)
qrforge "https://example.com" --print png

# Save as PNG file
qrforge "https://example.com" --output qr.png

# Combine flags
qrforge "https://example.com" --print ascii --output qr.png
```

## Use as a library

`qrforge` is also a Rust library. Add it to your `Cargo.toml`:

```toml
[dependencies]
qrforge = { git = "https://github.com/AhmaDREzA2022/qrforge" }
```

```rust
use qrforge::{generate, PrintMode, QrOptions};

generate(QrOptions {
    data: "https://example.com".to_string(),
    output: Some("qr.png".to_string()),
    print_mode: Some(PrintMode::Ascii),
})?;
```

## Stack

- [`clap`](https://github.com/clap-rs/clap) — CLI argument parsing
- [`qrcode`](https://github.com/kennytm/qrcode-rust) — QR code generation
- [`image`](https://github.com/image-rs/image) — PNG rendering
- [`viuer`](https://github.com/atanunq/viuer) — inline terminal image printing
