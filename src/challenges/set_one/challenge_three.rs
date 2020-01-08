use crate::utility::hex;
use crate::utility::string_process::guess;

pub fn challenge_three() -> Result<(), Box<dyn std::error::Error>> {
    let test_in = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let test = hex::from_str_hex(test_in)?;
    let guess_bytes = guess::guess(&test).bytes;
    println!("{}", String::from_utf8(guess_bytes)?);
    Ok(())
}
