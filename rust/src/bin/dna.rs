use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut count_a = 0;
    let mut count_c = 0;
    let mut count_g = 0;
    let mut count_t = 0;

    for b in reader.bytes() {
        match b {
            Ok(b'A') => count_a += 1,
            Ok(b'C') => count_c += 1,
            Ok(b'G') => count_g += 1,
            Ok(b'T') => count_t += 1,
            _ => {}
        }
    }

    println!("{count_a} {count_c} {count_g} {count_t}");
    Ok(())
}
