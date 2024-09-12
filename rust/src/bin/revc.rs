use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut writer = BufWriter::new(std::io::stdout());
    let mut answer = reader
        .bytes()
        .filter_map(|b| match b {
            Ok(b'A') => Some(b'T'),
            Ok(b'T') => Some(b'A'),
            Ok(b'C') => Some(b'G'),
            Ok(b'G') => Some(b'C'),
            _ => None,
        })
        .collect::<Vec<u8>>();
    answer.reverse();
    writer.write_all(&answer)
}
