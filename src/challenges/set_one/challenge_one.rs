use crate::utility::encoding::base64;

pub fn challenge_one() -> Result<(), Box<dyn std::error::Error>> {
    let test = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64test = base64::from_string_hex(test)?;
    let bytes = base64::to_bytes(base64test)?;
    let ret_string = String::from_utf8(bytes)?;
    println!("{}", ret_string);
    Ok(())
}
