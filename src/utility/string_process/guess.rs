use std::collections::HashMap;

use crate::utility::encoding::xor;

#[derive(Default)]
pub struct Guess {
    pub idx: u8,
    pub score: u64,
    pub bytes: Vec<u8>,
}

pub fn guess(input: &[u8]) -> Guess {
    let mut max = Guess::default();

    for i in 0..128_u8 {
        let xor = xor::single_byte(&input, i);
        // Sometimes we'll get a string that doesn't convert into readable ascii
        // so lossy is ok, as we don't really want that string anyway
        let upper_str = String::from_utf8_lossy(&xor).to_ascii_uppercase();
        let score = freq(upper_str);
        if score > max.score {
            max.idx = i;
            max.score = score;
            max.bytes = xor;
        }
    }

    max
}

pub fn freq(input: String) -> u64 {
    let mut alpha_freq: HashMap<char, u64> = HashMap::new();
    alpha_freq.insert('A', 816);
    alpha_freq.insert('B', 149);
    alpha_freq.insert('C', 278);
    alpha_freq.insert('D', 425);
    alpha_freq.insert('E', 1270);
    alpha_freq.insert('F', 222);
    alpha_freq.insert('G', 201);
    alpha_freq.insert('H', 609);
    alpha_freq.insert('I', 696);
    alpha_freq.insert('J', 15);
    alpha_freq.insert('K', 77);
    alpha_freq.insert('L', 402);
    alpha_freq.insert('M', 240);
    alpha_freq.insert('N', 674);
    alpha_freq.insert('O', 750);
    alpha_freq.insert('P', 192);
    alpha_freq.insert('Q', 9);
    alpha_freq.insert('R', 598);
    alpha_freq.insert('S', 632);
    alpha_freq.insert('T', 902);
    alpha_freq.insert('U', 275);
    alpha_freq.insert('V', 236);
    alpha_freq.insert('W', 20);
    alpha_freq.insert('X', 15);
    alpha_freq.insert('Y', 197);
    alpha_freq.insert('Z', 7);
    alpha_freq.insert(' ', 1270);

    input
        .chars()
        .map(|ch| *(alpha_freq.entry(ch).or_default()))
        .sum()
}
