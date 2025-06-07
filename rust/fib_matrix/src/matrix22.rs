use num_traits::Zero;
use num_traits::One;

use std::ops::Mul;
use std::ops::Add;

#[derive(Clone)]
pub struct Matrix22<T>  {
    pub elts: [[T; 2]; 2]
}

impl<T> Matrix22<T> {
    pub fn new(e00: T, e01: T, e10: T, e11: T) -> Self {
        Self {
            elts: [[e00, e01],
                   [e10, e11]]
        }
    }
}

// impl<T> One for BigUintMatrix where T: Zero + One + for<'a> Mul<&'a T, Output = T> {
impl<T> One for Matrix22<T> where T: Zero + One, T: Add + Mul, Matrix22<T>: Mul<Output = Matrix22<T>> {
    fn one() -> Self {
        Self {
            elts: [[T::one(), T::zero()],
                   [T::zero(), T::one()]]
        }
    }
}

impl<T> Mul<&Matrix22<T>> for &Matrix22<T> where for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>, T: Add<Output = T> {
    type Output = Matrix22<T>;

    fn mul(self, other: &Matrix22<T>) -> Self::Output {
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

impl<T> Add<&Matrix22<T>> for &Matrix22<T> where for<'a> &'a T: Add<&'a T, Output = T> {
    type Output = Matrix22<T>;

    fn add(self, other: &Matrix22<T>) -> Self::Output {
        Self::Output {
            elts: [
                [
                    &self.elts[0][0] + &other.elts[0][0],
                    &self.elts[0][1] + &other.elts[0][1],
                ], [
                    &self.elts[1][0] + &other.elts[1][0],
                    &self.elts[1][1] + &other.elts[1][1],
                ]
            ]
        }
    }
}

impl<T> Mul<Matrix22<T>> for Matrix22<T> where for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>, T: Add<Output =T> {
     type Output = Matrix22<T>;

     fn mul(self, other: Matrix22<T>) -> Self::Output {
         &self * &other
     }
}

impl<T> Add<Matrix22<T>> for Matrix22<T> where for<'a> &'a T: Add<&'a T, Output = T> {
    type Output = Matrix22<T>;

    fn add(self, other: Matrix22<T>) -> Self::Output {
        &self + &other
    }
}

#[derive(Clone)]
pub struct Vector2<T> {
     pub elts: [T; 2]
}

impl<T> Vector2<T> {
    pub fn new(e0: T, e1: T) -> Self {
        Self {
            elts: [e0, e1]
        }
    }
}

impl<T> Mul<&Vector2<T>> for &Matrix22<T> where for<'a> &'a T: Mul<&'a T, Output = T>, T: Add<Output = T> {
    type Output = Vector2<T>;

    fn mul(self, other: &Vector2<T>) -> Self::Output
    {
        Self::Output {
            elts: [
                &self.elts[0][0] * &other.elts[0] + 
                    &self.elts[0][1] * &other.elts[1],
                &self.elts[1][0] * &other.elts[0] +
                    &self.elts[1][1] * &other.elts[1],
            ]
        }
    }
}
