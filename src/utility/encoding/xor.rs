pub fn equal_vecs(first_vec: &[u8], sec_vec: &[u8]) -> Vec<u8> {
    first_vec
        .iter()
        .zip(sec_vec)
        .map(|(first_byte, second_byte)| first_byte ^ second_byte)
        .collect()
}

pub fn single_byte(v: &[u8], byte: u8) -> Vec<u8> {
    v.iter().map(|input| input ^ byte).collect()
}

pub fn repeating_key(plain_vec: &[u8], key: &[u8]) -> Vec<u8> {
    plain_vec
        .iter()
        .zip(key.iter().cycle())
        .map(|(plain_byte, key_byte)| plain_byte ^ key_byte)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utility::hex;

    #[test]
    pub fn equal_vecs_test() {
        let first = b"1c0111001f010100061a024b53535009181c";
        let second = b"686974207468652062756c6c277320657965";
        let first_hex = hex::from_str_hex(first).unwrap();
        let second_hex = hex::from_str_hex(second).unwrap();
        let xored = equal_vecs(&first_hex, &second_hex);
        let res = hex::to_str_hex(&xored);

        for (res_ch, cmp_ch) in res
            .chars()
            .zip("746865206b696420646f6e277420706c6179".chars())
        {
            assert_eq!(res_ch, cmp_ch);
        }
    }
}
