# qrforge

A fast, flexible QR code generator for the terminal, built with Rust.

## Features

- Print QR codes as ASCII art in any terminal
- Print QR codes as inline images (Kitty/Ghostty protocol)
- Save QR codes as PNG or SVG files
- Custom foreground/background colors (hex)
- Invert colors for dark mode
- Configurable error correction level (L / M / Q / H)
- Configurable scale and quiet zone
- Usable as a Rust library

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
qrforge [OPTIONS] <DATA>
```

### Options

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--output <FILE>` | `-o` | — | Save as PNG or SVG (detected by extension) |
| `--print <MODE>` | `-p` | — | Print in terminal: `ascii`, `png`, `svg` |
| `--ecc <LEVEL>` | `-e` | `m` | Error correction: `l`, `m`, `q`, `h` |
| `--scale <N>` | `-s` | `10` | Pixels per module for PNG/SVG |
| `--quiet-zone <N>` | `-q` | `4` | Border size in modules |
| `--fg <HEX>` | — | `#000000` | Foreground color |
| `--bg <HEX>` | — | `#ffffff` | Background color |
| `--invert` | — | `false` | Invert foreground and background |

### Examples

```bash
# Print as ASCII
qrforge "https://example.com" --print ascii

# Print as inline image (Ghostty, Kitty, iTerm2)
qrforge "https://example.com" --print png

# Save as PNG
qrforge "https://example.com" --output qr.png

# Save as SVG
qrforge "https://example.com" --output qr.svg

# Custom colors
qrforge "https://example.com" --print png --fg "#ffffff" --bg "#1a1a1a"

# Inverted colors
qrforge "https://example.com" --print ascii --invert

# High error correction
qrforge "https://example.com" --print ascii --ecc h

# Combine flags — print and save at once
qrforge "https://example.com" --print ascii --output qr.png --ecc h --scale 15
```

## Error Correction Levels

| Level | Flag | Recovery |
|-------|------|----------|
| Low | `l` | ~7% |
| Medium | `m` | ~15% |
| Quartile | `q` | ~25% |
| High | `h` | ~30% |

Higher error correction = larger QR code but more resistant to damage.

## Use as a Library

`qrforge` is also a Rust library. Add it to your `Cargo.toml`:

```toml
[dependencies]
qrforge = { git = "https://github.com/AhmaDREzA2022/qrforge" }
```

```rust
use qrforge::{
    generate_code, to_png, to_ascii, to_svg,
    EccLevel, PngOpts, AsciiOpts, SvgOpts, ColorOptions,
};

// Generate the code
let code = generate_code("https://example.com", EccLevel::M)?;

// Render as PNG
let img = to_png(&code, PngOpts::default())?;
img.save("qr.png")?;

// Render as ASCII
let ascii = to_ascii(&code, AsciiOpts::default())?;
println!("{ascii}");

// Render as SVG
let svg = to_svg(&code, SvgOpts::default())?;
std::fs::write("qr.svg", svg)?;
```

## Stack

- [`clap`](https://github.com/clap-rs/clap) — CLI argument parsing
- [`qrcode`](https://github.com/kennytm/qrcode-rust) — QR code generation
- [`image`](https://github.com/image-rs/image) — PNG rendering
- [`viuer`](https://github.com/atanunq/viuer) — inline terminal image printing
