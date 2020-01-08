use crate::utility::encoding::xor;
use crate::utility::hex;

pub fn challenge_two() {
    let first = b"1c0111001f010100061a024b53535009181c";
    let second = b"686974207468652062756c6c277320657965";
    let first_hex = hex::from_str_hex(first).unwrap();
    let second_hex = hex::from_str_hex(second).unwrap();
    let xored = xor::equal_vecs(&first_hex, &second_hex);
    println!("{}", String::from_utf8(xored).unwrap());
}
