use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum ParseMode {
    Id,
    Data,
}

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename)?;
    let reader = BufReader::new(file).bytes();
    let mut max_id = String::from("");
    let mut max_percent = 0.0;
    let mut curr_id: Vec<u8> = Vec::new();
    let mut curr_gc = 0.0;
    let mut curr_count = 0.0;
    let mut mode = ParseMode::Data;

    for b in reader {
        match mode {
            ParseMode::Id => match b {
                Ok(b'\n') => {
                    curr_gc = 0.0;
                    curr_count = 0.0;
                    mode = ParseMode::Data;
                }
                Ok(b) => curr_id.push(b),
                _ => {}
            },
            ParseMode::Data => match b {
                Ok(b'>') => {
                    if curr_gc / curr_count > max_percent {
                        max_percent = curr_gc / curr_count;
                        max_id = String::from_utf8(curr_id.clone()).unwrap();
                    }
                    curr_id.clear();
                    mode = ParseMode::Id;
                }
                Ok(b'G' | b'C') => {
                    curr_count += 1.0;
                    curr_gc += 1.0;
                }
                Ok(b'A' | b'T') => {
                    curr_count += 1.0;
                }
                _ => {}
            },
        }
    }

    if curr_gc / curr_count > max_percent {
        max_percent = curr_gc / curr_count;
        max_id = String::from_utf8(curr_id.clone()).unwrap();
    }

    println!("{max_id}");
    println!("{}", max_percent * 100.0);
    Ok(())
}
