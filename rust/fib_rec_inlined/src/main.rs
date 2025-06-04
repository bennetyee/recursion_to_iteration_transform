use std::env;
use crate::fib::fib;

mod fib;

fn main() {
    let av: Vec<String> = env::args().collect();
    let nvec: Vec<u64> = av[1..].iter().map(|s| s.parse().expect("not a number")).collect();
    for n in nvec.iter() {
        println!("fib({}) = {}", n, fib(*n));
    }
}
