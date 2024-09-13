use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let k = args.next().unwrap().parse::<f64>().unwrap();
    let m = args.next().unwrap().parse::<f64>().unwrap();
    let n = args.next().unwrap().parse::<f64>().unwrap();
    let mut answer = k / (k + m + n);
    answer += (m / (k + m + n)) * (k / (k + m + n - 1.0));
    answer += (m / (k + m + n)) * ((m - 1.0) / (k + m + n - 1.0)) * 0.75;
    answer += (m / (k + m + n)) * (n / (k + m + n - 1.0)) * 0.5;
    answer += (n / (k + m + n)) * (k / (k + m + n - 1.0)) * 1.0;
    answer += (n / (k + m + n)) * (m / (k + m + n - 1.0)) * 0.5;
    println!("{answer}");
}
