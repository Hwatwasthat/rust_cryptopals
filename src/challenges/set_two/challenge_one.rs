use crate::utility::encoding::padding;

pub fn challenge_one() {
    let buffer = "YELLOW SUBMARINE";
    let output = padding::pkcs7(&mut buffer.as_bytes().to_vec(), 20);
    println!("{:?}", output);
}
