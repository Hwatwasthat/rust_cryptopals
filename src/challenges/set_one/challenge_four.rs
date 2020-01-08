use crate::utility::hex;
use crate::utility::string_process::guess::{guess, Guess};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn challenge_four(file: File) -> Result<(), Box<dyn std::error::Error>> {
    let mut results: Vec<Guess> = Vec::new();

    for line in BufReader::new(file).split(b'\n') {
        let hex_vec = hex::from_str_hex(&line?)?;
        results.push(guess(&hex_vec));
    }

    results.sort_by(|a, b| a.score.cmp(&b.score));

    let most_likely = results.last();

    if let Some(item) = most_likely {
        let s = String::from_utf8_lossy(&item.bytes);
        println!("{}", s);
    } else {
        println!("No valid strings");
    }

    Ok(())
}
