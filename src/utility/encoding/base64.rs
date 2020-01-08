use crate::utility::errors::TransformError;
use crate::utility::hex;
use std::error::Error;

const BASE64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub fn from_string_hex(s: &[u8]) -> Result<String, Box<dyn Error>> {
    let hex_vec = hex::from_str_hex(s)?;
    Ok(encode(hex_vec))
}

pub fn encode(bytes: Vec<u8>) -> String {
    let mut out = String::new();

    let mut prev = b'0';

    for (i, ch) in bytes.iter().enumerate() {
        match i % 3 {
            0 => out.push(BASE64[(ch >> 2) as usize]),
            1 => out.push(BASE64[((prev & 0x03) << 4 | ch >> 4) as usize]),
            2 => {
                out.push(BASE64[((prev & 0x0F) << 2 | ch >> 6) as usize]);
                out.push(BASE64[(ch & 0x3F) as usize]);
            }
            _ => (),
        }
        prev = *ch;
    }

    if bytes.len() % 3 != 0 {
        padding(&mut out, bytes.len() % 3);
    }

    out
}

pub fn to_bytes(input: String) -> Result<Vec<u8>, TransformError> {
    if input.len() % 4 != 0 {
        return Err(TransformError {
            cause: "Passed incorrect length string to base64::to_bytes",
        });
    }

    let mut pad: usize = 0;
    let mut block: [u8; 4] = Default::default();
    let mut count: usize = 0;
    let mut out: Vec<u8> = Vec::new();

    for ch in input.chars() {
        block[count] = if ch == '=' {
            pad += 1;
            0 // Will be erased anyway
        } else {
            sextet_decode(ch)? as u8
        };

        count += 1;

        if count == 4 {
            out.push(block[0] << 2 | block[1] >> 4);
            out.push(block[1] << 4 | block[2] >> 2);
            out.push(block[2] << 6 | block[3]);
            count = 0;
        }
    }

    out.resize(out.len() - pad, 0);

    Ok(out)
}

fn padding(b64_string: &mut String, pad: usize) {
    match pad {
        1 => b64_string.push(
            BASE64[b64_string.as_bytes()[(b64_string.len() - 1 as usize) & 0x03 << 4] as usize],
        ),
        2 => b64_string.push(
            BASE64[b64_string.as_bytes()[(b64_string.len() - 1 as usize) & 0x0F << 2] as usize],
        ),
        _ => (),
    }

    b64_string.extend(std::iter::repeat('=').take(pad - 1));
}

fn sextet_decode(ch: char) -> Result<usize, TransformError> {
    BASE64.iter().position(|&c| c == ch).ok_or(TransformError {
        cause: "Passed non base64 character to base64::sextet_decode",
    })
}
