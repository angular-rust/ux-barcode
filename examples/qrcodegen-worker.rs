use barcode::symbols::qr::{Mask, QrCode, QrCodeEcc, QrSegment, Version};

fn main() {
    loop {
        // Read data length or exit
        let length: i16 = read_int();
        if length == -1 {
            break;
        }

        // Read data bytes
        let mut data = Vec::<u8>::with_capacity(length as usize);
        for _ in 0..length {
            let b: i16 = read_int();
            assert_eq!(i16::from(b as u8), b, "Byte value out of range");
            data.push(b as u8);
        }
        let isascii: bool = data.iter().all(|b| *b < 128);

        // Read encoding parameters
        let errcorlvl = read_int();
        let minversion = read_int();
        let maxversion = read_int();
        let mask = read_int();
        let boostecl = read_int();
        assert!(0 <= errcorlvl && errcorlvl <= 3);
        assert!(
            i16::from(Version::MIN.value()) <= minversion
                && minversion <= maxversion
                && maxversion <= i16::from(Version::MAX.value())
        );
        assert!(-1 <= mask && mask <= 7);
        assert!(boostecl >> 1 == 0);

        // Make segments for encoding
        let segs: Vec<QrSegment> = if isascii {
            let chrs: Vec<char> = std::str::from_utf8(&data).unwrap().chars().collect();
            QrSegment::make_segments(&chrs)
        } else {
            vec![QrSegment::make_bytes(&data)]
        };

        // Try to make QR Code symbol
        let msk = if mask == -1 {
            None
        } else {
            Some(Mask::new(mask as u8))
        };
        match QrCode::encode_segments_advanced(
            &segs,
            ECC_LEVELS[errcorlvl as usize],
            Version::new(minversion as u8),
            Version::new(maxversion as u8),
            msk,
            boostecl != 0,
        ) {
            Ok(qr) => {
                // Print grid of modules
                println!("{}", qr.version().value());
                for y in 0..qr.size() {
                    for x in 0..qr.size() {
                        println!("{}", qr.get_module(x, y) as i8);
                    }
                }
            }
            Err(_) => println!("-1"),
        }
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }
}

fn read_int() -> i16 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut chrs: Vec<char> = line.chars().collect();
    assert_eq!(chrs.pop().unwrap(), '\n');
    let line: String = chrs.iter().cloned().collect();
    line.parse::<i16>().expect("Invalid number")
}

static ECC_LEVELS: [QrCodeEcc; 4] = [
    QrCodeEcc::Low,
    QrCodeEcc::Medium,
    QrCodeEcc::Quartile,
    QrCodeEcc::High,
];
