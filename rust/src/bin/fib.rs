use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let n = args.next().unwrap().parse::<u64>().unwrap();
    let k = args.next().unwrap().parse::<u64>().unwrap();
    let mut reproductive_pairs = 0;
    let mut total_pairs = 1;
    let mut new_pairs;

    for _ in 1..n {
        new_pairs = reproductive_pairs * k;
        reproductive_pairs = total_pairs;
        total_pairs = reproductive_pairs + new_pairs;
    }
    println!("{total_pairs}");
}
