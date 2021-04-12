# ux-barcode

![Test status badge](https://img.shields.io/github/workflow/status/angular-rust/ux-barcode/tests?label=tests&logo=github&style=flat-square)
![Loc](https://img.shields.io/tokei/lines/github/angular-rust/ux-barcode?style=flat-square)
[![codecov][codecov-badge]][codecov-url]

[codecov-badge]: https://img.shields.io/codecov/c/github/angular-rust/ux-barcode/main?logo=codecov&style=flat-square&token=KNUti201wT
[codecov-url]: https://codecov.io/gh/angular-rust/ux-barcode

```
[codecov-badge]: https://codecov.io/gh/angular-rust/ux-barcode/branch/develop/graph/badge.svg?token=KNUti201wT
```

[package]
keywords = ["qrcode", "generate", "png", "svg", "vec"]
categories = ["no-std", "encoding", "multimedia::images"]
description = "Generate QR Code matrices and images in RAW, PNG and SVG formats."
include = ["src/**/*", "Cargo.toml", "README.md", "LICENSE"]

[badges.travis-ci]
repository = "magiclen/qrcode-generator"
branch = "master"

[dependencies]
image-dep = { package = "image", version = "0.23.9", optional = true }

[features]
default = ["image"]
std = []

image = ["std", "image-dep"]

QR Code Generator
====================

[![Build Status](https://travis-ci.org/magiclen/qrcode-generator.svg?branch=master)](https://travis-ci.org/magiclen/qrcode-generator)

This crate provides functions to generate QR Code matrices and images in RAW, PNG and SVG formats.

## Examples

#### Encode any data to a QR Code matrix which is `Vec<Vec<bool>>`.

```rust
use barcode::QrCodeEcc;

let result: Vec<Vec<bool>> = barcode::to_matrix("Hello world!", QrCodeEcc::Low).unwrap();

println!("{:?}", result);
```

#### Encode any data to a PNG image stored in a Vec instance.

```rust
use barcode::QrCodeEcc;

let result: Vec<u8> = barcode::to_png_to_vec("Hello world!", QrCodeEcc::Low, 1024).unwrap();

println!("{:?}", result);
```

#### Encode any data to a PNG image stored in a file.

```rust
use barcode::QrCodeEcc;

barcode::to_png_to_file("Hello world!", QrCodeEcc::Low, 1024, "tests/data/file_output.png").unwrap();
```

#### Encode any data to a SVG image stored in a String instance.

```rust
use barcode::QrCodeEcc;

let result: String = barcode::to_svg_to_string("Hello world!", QrCodeEcc::Low, 1024, None::<&str>).unwrap();

println!("{:?}", result);
```

#### Encode any data to a SVG image stored in a file.

```rust
use barcode::QrCodeEcc;

barcode::to_svg_to_file("Hello world!", QrCodeEcc::Low, 1024, None::<&str>, "tests/data/file_output.png").unwrap();
```

## Low-level Usage

### Raw Image Data

The `to_image` and `to_image_buffer` functions can be used, if you want to modify your image.

### Segments

Every `to_*` function has a corresponding `_from_segments` function. You can concatenate segments by using different encoding methods, such as **numeric**, **alphanumeric** or **binary** to reduce the size (level) of your QR code matrix/image.

```rust
use barcode::{QrCodeEcc, QrSegment};

let first = "1234567";

let second = "ABCDEFG";

let first_chars: Vec<char> = first.chars().collect();
let second_chars: Vec<char> = second.chars().collect();

let segments = [QrSegment::make_numeric(&first_chars), QrSegment::make_alphanumeric(&second_chars)];

let result: Vec<Vec<bool>> = barcode::to_matrix_from_segments(&segments, QrCodeEcc::Low).unwrap();

println!("{:?}", result);
```

More segments optimization apporaches: [magiclen/qrcode-segments-optimizer](https://github.com/magiclen/qrcode-segments-optimizer)

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.qrcode-generator]
version = "*"
default-features = false
```