use num_bigint::BigUint;
use num_traits::{Zero, One};

use crate::matrix::*;

type TransformMatrix = Matrix<BigUint, 2usize>;
// type StateVec = Vector<BigUint, 2usize>;

pub fn fib(n: &BigUint) -> BigUint {
    let mut n = n.clone();
    if &n <= &BigUint::one() {
        n
    } else {
        let mut t = TransformMatrix::new(
            [
                [BigUint::one(), BigUint::one()],
                [BigUint::one(), BigUint::zero()]
            ]
        );
        let mut res = TransformMatrix::one();

        n = &n - 1u32;
        while &n != &BigUint::zero() {
            if n.bit(0) {
                res = &res * &t;
            }
            t = &t * &t;
            n = &n / 2u32;
        }

        // let v = &res * &StateVec::new([BigUint::one(), BigUint::ZERO]);
        // v.elts[0].clone()

        res.elts[0][0].clone()
    }
}
