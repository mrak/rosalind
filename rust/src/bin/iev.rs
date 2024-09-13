use std::env;

#[allow(non_snake_case)]
fn main() {
    let mut args = env::args().skip(1);
    let AAAA = args.next().unwrap().parse::<f64>().unwrap();
    let AAAa = args.next().unwrap().parse::<f64>().unwrap();
    let AAaa = args.next().unwrap().parse::<f64>().unwrap();
    let AaAa = args.next().unwrap().parse::<f64>().unwrap();
    let Aaaa = args.next().unwrap().parse::<f64>().unwrap();
    let aaaa = args.next().unwrap().parse::<f64>().unwrap();
    let answer = AAAA * 2.0 + AAAa * 2.0 + AAaa * 2.0 + AaAa * 1.5 + Aaaa * 1.0 + aaaa * 0.0;
    println!("{answer}");
}
