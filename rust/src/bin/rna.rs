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
    writer.write_all(
        &reader
            .bytes()
            .filter_map(|b| match b {
                Ok(b'T') => Some(b'U'),
                Ok(b) => Some(b),
                Err(_) => None,
            })
            .collect::<Vec<u8>>(),
    )
}
