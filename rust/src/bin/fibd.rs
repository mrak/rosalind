use std::collections::VecDeque;
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let n = args.next().unwrap().parse::<u64>().unwrap();
    let m = args.next().unwrap().parse::<usize>().unwrap();
    let mut generations: VecDeque<u64> = VecDeque::with_capacity(m + 1);
    for _ in 2..m {
        generations.push_back(0);
    }
    generations.push_back(1);
    let mut population = 1;
    let mut young;
    let mut dying = 0;
    let mut born;

    for _i in 1..n {
        young = *generations.back().unwrap();
        //println!("{generations:?} | month {i} : young/total {young}/{population} dying {dying}");
        born = population - young;
        population += born - dying;
        generations.push_back(born);
        dying = generations.pop_front().unwrap();
    }
    println!("{population}");
}
