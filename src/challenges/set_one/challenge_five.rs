use crate::utility::encoding::xor;
use crate::utility::hex;

pub fn challenge_five() {
    let crypt = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";

    let res = xor::repeating_key(crypt, key);

    let hex = hex::to_str_hex(&res);

    assert_eq!(
        hex,
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343\
         c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2\
         b2027630c692b20283165286326302e27282f"
    )
}
