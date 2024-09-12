use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file).lines();

    let first_str = reader.next().unwrap().unwrap();
    let first = first_str.as_bytes();
    let second_str = reader.next().unwrap().unwrap();
    let second = second_str.as_bytes();

    let mut hamming_distance = 0;

    for (i, b) in first.iter().enumerate() {
        if second[i] != *b {
            hamming_distance += 1;
        }
    }
    println!("{hamming_distance}");
    Ok(())
}
