use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename)?;
    let mut binding = BufReader::new(file).bytes();
    let reader = binding.by_ref();
    let mut result = String::from("");

    loop {
        let str =
            String::from_utf8(reader.take(3).filter_map(|b| b.ok()).collect::<Vec<u8>>()).unwrap();
        match &str[..] {
            "UAA" | "UAG" | "UGA" => break,
            "UUU" | "UUC" => result.push('F'),
            "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => result.push('L'),
            "UCU" | "UCC" | "UCA" | "UCG" | "AGU" | "AGC" => result.push('S'),
            "UAU" | "UAC" => result.push('Y'),
            "UGU" | "UGC" => result.push('C'),
            "UGG" => result.push('W'),
            "CCU" | "CCC" | "CCA" | "CCG" => result.push('P'),
            "CAU" | "CAC" => result.push('H'),
            "CAA" | "CAG" => result.push('Q'),
            "CGU" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => result.push('R'),
            "AUU" | "AUC" | "AUA" => result.push('I'),
            "AUG" => result.push('M'),
            "ACU" | "ACC" | "ACA" | "ACG" => result.push('T'),
            "AAU" | "AAC" => result.push('N'),
            "AAA" | "AAG" => result.push('K'),
            "GUU" | "GUC" | "GUA" | "GUG" => result.push('V'),
            "GCU" | "GCC" | "GCA" | "GCG" => result.push('A'),
            "GAU" | "GAC" => result.push('D'),
            "GAA" | "GAG" => result.push('E'),
            "GGU" | "GGC" | "GGA" | "GGG" => result.push('G'),
            _ => break,
        }
    }
    println!("{result}");
    Ok(())
}
