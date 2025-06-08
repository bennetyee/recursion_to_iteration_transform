use std::env;
use std::str::FromStr;
use fib::FibUint;

use crate::fib::fib;
mod matrix;
mod fib;

fn main() {
    let av: Vec<String> = env::args().collect();
    let nvec: Vec<FibUint> = av[1..].iter().map(|s| FibUint::from_str(s).expect("not a number")).collect();
    for n in nvec.iter() {
        println!("fib({}) = {}", n, fib(n));
    }
}
