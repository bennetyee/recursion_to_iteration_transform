// use num::BigUint;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem;

use crate::matrix::*;

pub type FibUint = BigUint;

type TransformMatrix = Matrix<BigUint, 2usize>;
// type StateVec = Vector<BigUint, 2usize>;

pub fn fib(n: &BigUint) -> BigUint {
    let mut n = n.clone();
    if &n <= &BigUint::one() {
        n
    } else {
        let mut t = TransformMatrix::from_unsigned(
            &[
                [1, 1],
                [1, 0u8],
            ]
        );
        let mut res = TransformMatrix::one();

        n = &n - 1u32;
        while !n.is_zero() {
            if n.bit(0) {
                res = &res * &t;
            }
            t = &t * &t;
            n = &n / 2u8;
        }

        // let v = &res * &StateVec::new([BigUint::one(), BigUint::ZERO]);
        // v[0].clone()

        // res[0][0].clone()

        // default/swap is cheaper than clone
        let mut result = BigUint::default();
        mem::swap(&mut result, &mut res[0][0]);
        result
    }
}
