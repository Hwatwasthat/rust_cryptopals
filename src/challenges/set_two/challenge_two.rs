use crate::utility::encoding::xor;
use aes::Aes128;
use aes::block_cipher_trait::{ BlockCipher, generic_array::GenericArray };
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn challenge_two(file: File) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = vec![];
    BufReader::new(file).read_to_end(&mut input)?;
    let chunks = input.chunks_exact(16);
    let key = GenericArray::from_slice(b"YELLOW SUBMARINE");
    let cipher = Aes128::new(&key);

    let mut iv = vec![0u8; 16];

    let mut res: Vec<u8> = vec![];
    for chunk in chunks {
        let tmp = xor::equal_vecs(chunk, &iv);
        
        let mut ciphered = *GenericArray::from_slice(&tmp);
        cipher.encrypt_block(&mut ciphered);
        res.extend(ciphered.as_slice());
        iv = ciphered.as_slice().to_owned().clone();
    }

    let string_res = String::from_utf8_lossy(&res);
    println!("{}", string_res);
    Ok(())
}
