use num_bigint::BigUint;
use num_traits::One;

use std::ops::Mul;

// This could be generic for T and then instantiated for BigUint, with
// a trait bound that T implements an algebraic ring.
#[derive(Clone)]
struct BigUintMatrix {
    pub elts: [[BigUint; 2]; 2]
}

impl BigUintMatrix {
    fn new(e00: BigUint, e01: BigUint, e10: BigUint, e11: BigUint) -> Self {
        Self {
            elts: [[e00, e01],
                   [e10, e11]]
        }
    }
}

impl One for BigUintMatrix {
    fn one() -> Self {
        Self {
            elts: [[BigUint::one(), BigUint::ZERO],
                   [BigUint::ZERO, BigUint::one()]]
        }
    }
}

impl Mul<BigUintMatrix> for BigUintMatrix {
    type Output = BigUintMatrix;

    fn mul(self, other: BigUintMatrix) -> Self::Output {
        Self::Output {
            elts: [
                [
                    &self.elts[0][0] * &other.elts[0][0] +
                        &self.elts[0][1] * &other.elts[1][0],
                    &self.elts[0][0] * &other.elts[0][1] +
                        &self.elts[0][1] * &other.elts[1][1]
                ], [
                    &self.elts[1][0] * &other.elts[0][0] +
                        &self.elts[1][1] * &other.elts[1][0],
                    &self.elts[1][0] * &other.elts[0][1] +
                        &self.elts[1][1] * &other.elts[1][1]
                ]
            ]
                    
        }
    }
}

impl Mul<&BigUintMatrix> for &BigUintMatrix {
    type Output = BigUintMatrix;

    fn mul(self, other: &BigUintMatrix) -> Self::Output {
        Self::Output {
            elts: [
                [
                    &self.elts[0][0] * &other.elts[0][0] +
                        &self.elts[0][1] * &other.elts[1][0],
                    &self.elts[0][0] * &other.elts[0][1] +
                        &self.elts[0][1] * &other.elts[1][1]
                ], [
                    &self.elts[1][0] * &other.elts[0][0] +
                        &self.elts[1][1] * &other.elts[1][0],
                    &self.elts[1][0] * &other.elts[0][1] +
                        &self.elts[1][1] * &other.elts[1][1]
                ]
            ]
                    
        }
    }
}

// struct BigUintVec {
//     pub elts: [BigUint; 2]
// }

// impl BigUintVec {
//     fn new(e0: BigUint, e1: BigUint) -> Self {
//         Self {
//             elts: [e0, e1]
//         }
//     }
// }

// impl Mul<&BigUintVec> for &BigUintMatrix {
//     type Output = BigUintVec;

//     fn mul(self, other: &BigUintVec) -> Self::Output {
//         Self::Output {
//             elts: [
//                 &self.elts[0][0] * &other.elts[0] + 
//                     &self.elts[0][1] * &other.elts[1],
//                 &self.elts[1][0] * &other.elts[0] +
//                     &self.elts[1][1] * &other.elts[1],
//             ]
//         }
//     }
// }

pub fn fib(n: &BigUint) -> BigUint {
    let mut n = n.clone();
    if &n <= &BigUint::one() {
        n
    } else {
        let mut t = BigUintMatrix::new(BigUint::one(), BigUint::one(), BigUint::one(), BigUint::ZERO);
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
