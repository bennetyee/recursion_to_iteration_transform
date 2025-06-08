use num_bigint::BigUint;
use num_traits::One;

use crate::matrix::*;

type BigUintMatrix = Matrix22<BigUint>;
// type BigUintVec = Vector2<BigUint>;

pub fn fib(n: &BigUint) -> BigUint {
    let mut n = n.clone();
    if &n <= &BigUint::one() {
        n
    } else {
        let mut t = BigUintMatrix::new([[BigUint::one(), BigUint::one()], [BigUint::one(), BigUint::ZERO]]);
        let mut res = BigUintMatrix::one();

        n = &n - 1u32;
        while &n != &BigUint::ZERO {
            if n.bit(0) {
                res = &res * &t;
            }
            t = &t * &t;
            n = &n / 2u32;
        }

        // let v = &res * &BigUintVec::new(BigUint::one(), BigUint::ZERO);
        // v.elts[0].clone()

        res.elts[0][0].clone()
    }
}
