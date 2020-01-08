use crate::utility::string_process::guess;
use crate::utility::{
    comparison,
    encoding::{base64, xor},
    transformation,
};
use std::fs::File;
use std::io::prelude::*;

pub fn challenge_six(mut file: File) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_string = String::new();
    file.read_to_string(&mut file_string)?;

    let file_string = file_string.replace("\n", "");

    let decoded = base64::to_bytes(file_string)?;

    let keysize = comparison::find_keysize(&decoded)?;

    let possible = transformation::transpose(&decoded, keysize as usize);

    let byte_guess: Vec<u8> = possible.iter().map(|pos| guess::guess(pos).idx).collect();

    let result = xor::repeating_key(&decoded, &byte_guess);

    println!("{}", String::from_utf8(result)?);

    Ok(())
}
