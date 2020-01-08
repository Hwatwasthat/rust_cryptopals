use crate::utility::encoding::base64;
use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Ecb};
use std::fs::File;
use std::io::prelude::*;

pub fn challenge_seven(mut file: File) -> Result<(), Box<dyn std::error::Error>> {
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    // Don't leave the newlines in, regrets will be had
    let ciphertext = base64::to_bytes(text.replace("\n", ""))?;

    let key = b"YELLOW SUBMARINE";

    let cipher = Ecb::<Aes128, Pkcs7>::new_var(key, Default::default())?;

    let decrypted = cipher.decrypt_vec(&ciphertext)?;

    println!("{}", String::from_utf8_lossy(&decrypted));
    Ok(())
}
