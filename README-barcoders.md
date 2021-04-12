[package]
description = "A barcode-encoding library"
keywords = ["barcode", "barcodes", "barcode-encoding"]
exclude = [
    "media/*",
    "TODO",
]

[features]
ascii = []
json = []
svg = []
dev = ["clippy"]

[dependencies]
image = {version = "0.22", optional = true}
clippy = {version = "0.0.166", optional = true}

----------


# qr-gen

[![Current Crates.io Version](https://img.shields.io/crates/v/qr-gen.svg)](https://crates.io/crates/qr-gen)

Rust small tool to generate QR-Code in CLI

## Install

```bash
    cargo install qr-gen
```

## Usage

```bash
    qr-gen <text> [-o <path>]
```
QR Code generator library
=========================


Introduction
------------

This project aims to be the best, clearest QR Code generator library. The primary goals are flexible options and absolute correctness. Secondary goals are compact implementation size and good documentation comments.

Home page with live JavaScript demo, extensive descriptions, and competitor comparisons: https://www.nayuki.io/page/qr-code-generator-library


Features
--------

Core features:

* Significantly shorter code but more documentation comments compared to competing libraries
* Supports encoding all 40 versions (sizes) and all 4 error correction levels, as per the QR Code Model 2 standard
* Output formats: Raw modules/pixels of the QR symbol, SVG XML string
* Detects finder-like penalty patterns more accurately than other implementations
* Encodes numeric and special-alphanumeric text in less space than general text
* Open source code under the permissive MIT License

Manual parameters:

* User can specify minimum and maximum version numbers allowed, then library will automatically choose smallest version in the range that fits the data
* User can specify mask pattern manually, otherwise library will automatically evaluate all 8 masks and select the optimal one
* User can specify absolute error correction level, or allow the library to boost it if it doesn't increase the version number
* User can create a list of data segments manually and add ECI segments


Examples
--------
```rust
    use barcode::QrCode;
    use barcode::QrCodeEcc;
    use barcode::QrSegment;
    
    // Simple operation
    let qr = QrCode::encode_text("Hello, world!",
        QrCodeEcc::Medium).unwrap();
    let svg = qr.to_svg_string(4);
    
    // Manual operation
    let chrs: Vec<char> = "3141592653589793238462643383".chars().collect();
    let segs = QrSegment::make_segments(&chrs);
    let qr = QrCode::encode_segments_advanced(
        &segs, QrCodeEcc::High, 5, 5, Some(Mask::new(2)), false).unwrap();
    for y in 0 .. qr.size() {
        for x in 0 .. qr.size() {
            (... paint qr.get_module(x, y) ...)
        }
    }
```

More complete set of examples: https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs.



**Barcoders** is a barcode-encoding library for the Rust programming language.

Barcoders allows you to encode valid data for a chosen barcode symbology into a ```Vec<u8>``` representation of the underlying binary structure. From here, you can take advantage of one of the optional builtin generators (for exporting to SVG, GIF, PNG, etc) or build your own.

## Installation

For encode-only functionality (e.g if you just want to translate a `String` into a `Vec<u8>` of binary digits):

```toml
[dependencies]
barcode = "1.0.2"
```

If you want to generate barcodes into a particular format, turn on the appropriate feature(s):

```toml
[dependencies]
barcode = {version = "1.0.2", features = ["image", "ascii", "svg", "json"]}
```

Each generator is an optional feature so you only need to compile what you want to use.
See below for the feature associated to the generation functionality you desire.

## Current Support

The ultimate goal of Barcoders is to provide encoding support for all major (and many not-so-major) symbologies.

### Symbologies

* EAN-13
  * UPC-A
  * JAN
  * Bookland
* EAN-8
* EAN Supplementals
  * EAN-2
  * EAN-5
* Code11
  * USD-8
* Code39
* Code93
* Code128 (A, B, C)
* Two-Of-Five
  * Interleaved (ITF)
  * Standard (STF)
* Codabar
* More coming!

### Generators

* ASCII (feature: `ascii`)
* JSON (feature: `json`)
* SVG (feature: `svg`)
* PNG (feature: `image`)
* GIF (feature: `image`)
* JPEG (feature: `image`)
* Image Buffer (feature: `image`)
* Or add your own

## Examples

### Encoding
```rust
use barcode::sym::ean13::*;

// Each encoder accepts a String to be encoded. Valid data is barcode-specific
// and thus constructors return an Result<T, barcode::error::Error>.
let barcode = EAN13::new("593456661897").unwrap();

// The `encode` method returns a Vec<u8> of the binary representation of the
// generated barcode. This is useful if you want to add your own generator.
let encoded: Vec<u8> = barcode.encode();
```

### Image (GIF, JPEG, PNG) generation
```rust
use barcode::sym::code39::*;
use barcode::generators::image::*;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

let barcode = Code39::new("1ISTHELONELIESTNUMBER").unwrap();
let png = Image::png(80); // You must specify the height in pixels.
let encoded = barcode.encode();

// Image generators return a Result<Vec<u8>, barcode::error::Error) of encoded bytes.
let bytes = png.generate(&encoded[..]).unwrap();

// Which you can then save to disk.
let file = File::create(&Path::new("my_barcode.png")).unwrap();
let mut writer = BufWriter::new(file);
writer.write(&bytes[..]).unwrap();

// Generated file ↓ ↓ ↓
```
![Code 39: 1ISTHELONELIESTNUMBER](/media/code39_1istheloneliestnumber.png?raw=true "Code 39: 1ISTHELONELIESTNUMBER")

You can also request an [image::RgbaImage](http://www.piston.rs/image/image/type.RgbaImage.html), which you can manipulate yourself:
```rust
let barcode = Code39::new("BEELZEBUB").unwrap();
let buffer = Image::image_buffer(100);
let encoded = barcode.encode();
let img = buffer.generate_buffer(&encoded[..]).unwrap();

// Manipulate and save the image here...
```

You may also specify the barcode x-dimension, rotation, background/foreground colors and opacity by specifying the struct fields:
```rust
let gif = Image::GIF{height: 80,
                     xdim: 1,
                     rotation: Rotation::Zero,
                     // Using non black/white colors is generally not recommended by most vendors, but barcode makes it possible.
                     foreground: Color::new([255, 0, 0, 255]),
                     background: Color::new([0, 255, 20, 255])};
```

### SVG generation

SVG is similar to the other image types, but I've supplied it as a separate feature as it doesn't require third-party dependencies.

```rust
use barcode::sym::code39::*;
use barcode::generators::svg::*;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

let barcode = Code39::new("56DFU4A777H").unwrap();
let svg = SVG::new(200); // You must specify the height in pixels.
let encoded = barcode.encode();
let data: String = svg.generate(&encoded).unwrap();

let file = File::create(&Path::new("my_barcode.svg")).unwrap();
let mut writer = BufWriter::new(file);
writer.write(data.as_bytes()).unwrap();
```

You may also specify the barcode x-dimension, background/foreground colors and opacity by specifying the struct fields:
```rust
let svg = SVG{height: 80,
              xdim: 1,
              // Using non black/white colors is generally not recommended by most vendors, but barcode makes it possible.
              foreground: Color::black(),
              background: Color::new([0, 255, 20, 255])};
```

### ASCII generation

The ASCII generator is useful for testing purposes.

```rust
use barcode::sym::ean13::*;
use barcode::generators::ascii::*;

let barcode = EAN13::new("750103131130").unwrap();
let encoded = barcode.encode();

let ascii = ASCII::new();
ascii.generate(&encoded[..]);

assert_eq!(ascii.unwrap(),
"
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
".trim());
```


### JSON generation

This may be useful for passing encoded data to third-party systems in a conventional format.

```rust
use barcode::sym::codabar::*;
use barcode::generators::json::*;

let codabar = Codabar::new("A98B").unwrap();
let json = JSON::new();
let generated = json.generate(&codabar.encode()[..]);

assert_eq!(generated.unwrap(),
"
{
 \"height\": 10,
 \"xdim\": 1,
 \"encoding\": [1,0,1,1,0,0,1,0,0,1,0,1,1,0,1,0,0,1,0,1,0,1,0,0,1,1,0,1,0,1,0,1,0,1,0,0,1,0,0,1,1]
}
"
```

## Tests

Note, if you want to output actual image/svg files to the filesystem for visual confirmation, set
the `WRITE_TO_FILE` variable in the appropriate test modules.

Full suite:
```
$ cargo test --features="image svg ascii json"
```

Encoding only:
```
$ cargo test
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
