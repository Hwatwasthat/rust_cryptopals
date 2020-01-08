use crate::utility::comparison;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::u32;

const AES_BLOCKSIZE: usize = 16; // Bytes in an AES block for encryption
pub fn challenge_eight(file: File) -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(file).split(b'\n');

    let mut best_ham = u32::MAX;
    let mut best_line: usize = 0;

    for (line_no, line) in reader.enumerate() {
        let bytes = line?; // To avoid temporary value being dropped
        let chunked: Vec<&[u8]> = bytes.chunks_exact(AES_BLOCKSIZE).collect();
        let mut ham = 0;
        for pair in chunked.windows(2) {
            ham += comparison::hamming_distance(pair[0], pair[1])?;
        }

        // We want the line with the lowest hamming distance. Highly improbable
        // that an actual line has a hamming distance of 0.
        if ham < best_ham && ham != 0 {
            best_ham = ham;
            best_line = line_no;
        }
    }

    println!("Best line: {}\nBest Ham: {}", best_line, best_ham);
    Ok(())
}
