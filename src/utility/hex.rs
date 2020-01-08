use crate::utility::errors::TransformError;

pub fn from_str_hex(s: &[u8]) -> Result<Vec<u8>, TransformError> {
    s.chunks_exact(2)
        .map(|pair| -> Result<u8, TransformError> {
            Ok(char_to_nibble(pair[0])? << 4 | char_to_nibble(pair[1])?)
        })
        .collect()
}

pub fn to_str_hex(hex: &[u8]) -> String {
    let mut ret = String::new();
    ret.reserve(hex.len() * 2);

    for ch in hex {
        ret.push(nibble_to_char(ch >> 4));
        ret.push(nibble_to_char(ch & 0xF));
    }

    ret
}

fn char_to_nibble(ch: u8) -> Result<u8, TransformError> {
    match ch {
        b'0'..=b'9' => Ok(ch - b'0'),
        b'A'..=b'F' => Ok(ch + 0xa - b'A'),
        b'a'..=b'f' => Ok(ch + 0xa - b'a'),
        _ => Err(TransformError {
            cause: "Non hex character passed to hex::char_to_nibble",
        }),
    }
}

fn nibble_to_char(nibble: u8) -> char {
    match nibble {
        0..=9 => (nibble + b'0') as char,
        0xA..=0xF => ((nibble - 0xa) + b'a') as char,
        _ => panic!("Non Hex character passed"),
    }
}

#[cfg(test)]
mod from_str_hex_tests {
    use super::*;

    #[test]
    pub fn all_zero() {
        let str_hex = [b'0'; 8];

        let hex = match from_str_hex(&str_hex) {
            Ok(hex) => hex,
            Err(e) => {
                panic!("{:?}", e);
            }
        };

        assert_eq!(hex, [0; 4]);
    }

    #[test]
    pub fn alternate_fs() {
        let str_hex = b"0F0F0F0F";

        let hex = match from_str_hex(str_hex) {
            Ok(hex) => hex,
            Err(e) => {
                panic!("{:?}", e);
            }
        };

        assert_eq!(hex, [15, 15, 15, 15]);
    }
}
