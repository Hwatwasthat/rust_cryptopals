use crate::utility::encoding::xor;
use crate::utility::errors::ComparisonError;

const MAX_KEYSIZE: usize = 40;
const ACCURACY_MOD: usize = 1000;
struct Keysize {
    size: u16,
    freq: u32,
}

/// find_keysize finds the most likely keysizes when a repeating key
/// is used for encryption. Examples include block ciphers in ECB mode and XOR with
/// a repeating key. Returns a result with either a vector ordered from most to least
/// likely, or an error if the input does not exceed 40 bytes in length.
pub fn find_keysize(input: &[u8]) -> Result<u16, Box<dyn std::error::Error>> {
    if MAX_KEYSIZE > input.len() {
        return Err(Box::new(ComparisonError {
            cause: "Input for find_keysize must exceed 40 bytes in length.",
        }));
    }

    let mut distances: Vec<Keysize> = vec![];

    for keysize in 2..=MAX_KEYSIZE {
        let mut temp_res: u32 = 0;
        let mut iters: u32 = 0;
        let mut temp = input.to_vec();

        // Pad to make temp % keysize == 0
        temp.extend(std::iter::repeat(0).take(temp.len() % keysize));

        let chunked: Vec<&[u8]> = temp.chunks_exact(keysize).collect();

        for pair in chunked.windows(2) {
            temp_res += hamming_distance(pair[0], pair[1])?;
            iters += 1;
        }

        // Normalise for keysize and iterations.
        temp_res = (temp_res * ACCURACY_MOD as u32) / keysize as u32 / iters as u32;

        if temp_res != 0 {
            distances.push(Keysize {
                size: keysize as u16,
                freq: temp_res,
            });
        }
    }
    // Just get out the small hamming distance
    distances
        .iter()
        .min_by(|a: &&Keysize, b: &&Keysize| a.freq.cmp(&b.freq))
        .map(|guess| guess.size)
        .ok_or(Box::new(ComparisonError {
            cause: "No suitable keysize",
        }))
}

pub fn hamming_distance(first: &[u8], second: &[u8]) -> Result<u32, ComparisonError> {
    if first.len() != second.len() {
        return Err(ComparisonError {
            cause: "Vectors for hamming must be of equal length",
        });
    }

    Ok(xor::equal_vecs(&first, &second)
        .iter()
        .map(|byte: &u8| {
            (0..8)
                .map(|idx| ((byte >> idx) & 0x01_u8) as u32)
                .sum::<u32>()
        })
        .sum())
}
