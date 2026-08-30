use anyhow::{bail, ensure};

pub fn decode_sha256_hex_string(input: &str) -> anyhow::Result<[u8; 32]> {
    const DIGEST_SIZE: usize = 32;
    const ENCODED_SIZE: usize = DIGEST_SIZE * 2;

    let input = input.as_bytes();

    ensure!(
        input.len() == ENCODED_SIZE,
        "SHA256 digest must be {ENCODED_SIZE} characters long"
    );

    let mut decoded = [0u8; DIGEST_SIZE];

    for (index, output) in decoded.iter_mut().enumerate() {
        let offset = index * 2;
        let high = decode_hex_nibble(input[offset])?;
        let low = decode_hex_nibble(input[offset + 1])?;

        *output = (high << 4) | low;
    }

    Ok(decoded)
}

fn decode_hex_nibble(byte: u8) -> anyhow::Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => bail!("invalid hex character"),
    }
}

pub fn byte_slice_to_hex_string(input: &[u8]) -> String {
    use std::fmt::Write;

    // Each byte is represented as 2 hex characters: 0A, 2C, FF, etc.
    // So we need 2 * the number of input bytes as the capacity of the output String.
    let mut output = String::with_capacity(input.len() * 2);

    for byte in input {
        // Writing a byte to a String should never fail
        write!(&mut output, "{byte:02x}").unwrap()
    }

    output
}

#[cfg(test)]
mod decode_sha256_hex_tests {
    use super::decode_sha256_hex_string;

    #[test]
    fn rejects_invalid_length_inputs() {
        assert!(decode_sha256_hex_string("").is_err());

        let input = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeef";
        assert_eq!(63, input.len());
        assert!(decode_sha256_hex_string(input).is_err());
    }

    #[test]
    fn rejects_invalid_hex_characters() {
        let input = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeefg";
        assert_eq!(64, input.len());
        assert!(decode_sha256_hex_string(input).is_err());
    }

    #[test]
    fn canonicalizes_input() {
        let input1 = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
        let input2 = "00112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF";

        let b1 = decode_sha256_hex_string(input1).unwrap();
        let b2 = decode_sha256_hex_string(input2).unwrap();

        assert_eq!(b1, b2);
    }
}

#[cfg(test)]
mod byte_slice_to_hex_tests {
    use super::byte_slice_to_hex_string;

    #[test]
    fn performs_proper_conversion() {
        let bytes = [0xDE, 0xAD, 0xBE, 0xEF];
        let s = byte_slice_to_hex_string(&bytes);

        assert_eq!("deadbeef", s);
    }

    #[test]
    fn pads_with_zeroes() {
        let bytes = [0x1, 0x01, 0x11];
        let s = byte_slice_to_hex_string(&bytes);

        assert_eq!("010111", s);
    }
}
