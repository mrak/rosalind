use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename)?;
    let ids = BufReader::new(file)
        .lines()
        .map_while(|l| match l {
            Ok(s) => Some(s.chars().take_while(|c| *c != '_').collect::<String>()),
            _ => None,
        })
        .collect::<Vec<String>>();
    println!("{ids:?}");
    Ok(())
}
