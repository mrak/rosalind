use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file).lines();

    let s_str = reader.next().unwrap().unwrap();
    let s = s_str.as_bytes();
    let t_str = reader.next().unwrap().unwrap();
    let t = t_str.as_bytes();
    let mut answer: Vec<String> = Vec::new();

    'outer: for i in 0..s.len() - t.len() {
        if s[i] == t[0] {
            for j in 0..t.len() {
                if t[j] != s[i + j] {
                    continue 'outer;
                }
            }
            answer.push(format!("{}", i + 1));
        }
    }
    println!("{}", answer.join(" "));
    Ok(())
}
