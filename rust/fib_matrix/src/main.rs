use std::env;
use std::str::FromStr;
use num_bigint::BigUint;

use crate::fib::fib;

mod fib;

fn main() {
    let av: Vec<String> = env::args().collect();
    let nvec: Vec<BigUint> = av[1..].iter().map(|s| BigUint::from_str(s).expect("not a number")).collect();
    for n in nvec.iter() {
        println!("fib({}) = {}", n, fib(n));
    }
}
