extern crate aes;
extern crate block_modes;

mod challenges;
mod utility;

use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // challenges::set_one::challenge_one()?;
    // challenges::set_one::challenge_two();
    // challenges::set_one::challenge_three()?;
    // challenges::set_one::challenge_four(File::open("files/1_4.txt")?)?;
    // challenges::set_one::challenge_five();
    // challenges::set_one::challenge_six(File::open("files/1_6.txt")?)?;
    // challenges::set_one::challenge_seven(File::open("files/1_7.txt")?)?;
    // challenges::set_one::challenge_eight(File::open("files/1_8.txt")?)?;
    challenges::set_two::challenge_one();
    challenges::set_two::challenge_two(File::open("files/2_2.txt")?)?;
    Ok(())
}
