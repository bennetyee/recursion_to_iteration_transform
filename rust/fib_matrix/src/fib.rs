// use num::BigUint;  // num pulls in the num_bigint crate
use num_bigint::BigUint;
use num_traits::{One, Pow};
use std::mem;

use crate::matrix::*;

pub type FibUint = BigUint;

type TransformMatrix = Matrix<BigUint, 2usize>;
// type StateVec = Vector<BigUint, 2usize>;

pub fn fib(n: &BigUint) -> BigUint {
    if n <= &BigUint::one() {
        n.clone()
    } else {
        let t = TransformMatrix::from_unsigned_array(
            &[
                [1, 1],
                [1, 0u8],
            ]
        );

        let mut res = t.pow(n - 1u8);

        // default-then-swap is cheaper than clone, since output is large
        let mut result = BigUint::default();

        // let mut v = &res * &StateVec::from_unsigned_array(&[1, 0u8]);
        // mem::swap(&mut result, &mut v[0]);

        mem::swap(&mut result, &mut res[0][0]);

        result
    }
}
