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

        // default-then-swap is cheaper than clone, since output is large
        let mut result = BigUint::default();

        // let mut v = &res * &StateVec::from_unsigned(&[1, 0u8]);
        // mem::swap(&mut result, &mut v[0]);

        mem::swap(&mut result, &mut res[0][0]);

        result
    }
}
