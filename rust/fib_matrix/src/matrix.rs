use num_traits::Zero;
use num_traits::One;

use std::array::from_fn;
use std::default::Default;
use std::ops::Mul;
use std::ops::Add;

#[derive(Clone)]
pub struct Matrix<T, const N: usize>  {
    pub elts: [[T; N]; N],
}

impl <T, const N: usize> Matrix<T, N> {
    pub fn new(elts: [[T; N]; N]) -> Self {
        Self {
            elts: elts
        }
    }
}

impl<T, const N: usize> One for Matrix<T, N> where
    Matrix<T, N>: Mul<Output = Matrix<T, N>>,
    T: Zero + One + Default + Add + Mul,
{
    fn one() -> Self {
        let e: [[T; N]; N] = from_fn(
            |r| from_fn(
                |c| if r == c {
                    T::one()
                } else {
                    T::zero()
                }
            ));
        Self {
            elts: e
        }
    }
}

impl<T, const N: usize> Zero for Matrix<T, N> where
    Matrix<T, N>: Add<Output = Matrix<T, N>> + Mul<Output = Matrix<T, N>>,
    for<'a> &'a T: Add,
    T: Zero + One + Default + Add + Mul,
{
    fn zero() -> Self {
        let e: [[T; N]; N] = from_fn(|_| from_fn(|_| T::zero()));
        Self {
            elts: e
        }
    }

    fn is_zero(&self) -> bool {
        for r in 0..N {
            for c in 0..N {
                if !self.elts[r][c].is_zero() {
                    return false;
                }
            }
        }
        true
    }
}


impl<T, const N: usize> Mul<&Matrix<T, N>> for &Matrix<T, N> where
    for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>,
    T: Zero + Default + Add<Output = T> + std::iter::Sum,
{
    type Output = Matrix<T, N>;

    fn mul(self, other: &Matrix<T, N>) -> Self::Output {
        let e: [[T; N]; N] = from_fn(
            |r| from_fn(
                |c|
                (0..N).map(|n| &self.elts[r][n] * &other.elts[n][c]).sum()));
        Self::Output {
            elts: e
        }
    }
}

impl<T, const N: usize> Add<&Matrix<T, N>> for &Matrix<T, N> where
    for<'a> &'a T: Add<&'a T, Output = T>,
    T: Zero + Default + Add<T, Output = T>,
{
    type Output = Matrix<T, N>;

    fn add(self, other: &Matrix<T, N>) -> Self::Output {
        let mut e: [[T; N]; N] = from_fn(|_| from_fn(|_| T::default()));
        for r in 0.. N {
            for c in 0..N {
                e[r][c] = &self.elts[r][c] + &other.elts[r][c];
            }
        }
        Self::Output {
            elts: e
        }
    }
}

impl<T, const N: usize> Mul<Matrix<T, N>> for Matrix<T, N> where
    for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>,
    T: Add<Output = T> + Zero + Default + std::iter::Sum,
{
     type Output = Self;

     fn mul(self, other: Matrix<T, N>) -> Self::Output {
         &self * &other
     }
}

impl<T, const N: usize> Add<Matrix<T, N>> for Matrix<T, N> where T: Zero + Default + Add<T, Output = T>, for<'a> &'a T: Add<&'a T, Output = T> {
    type Output = Self;

    fn add(self, other: Matrix<T, N>) -> Self::Output {
        &self + &other
    }
}

pub type Matrix22<T> = Matrix<T, 2usize>;

#[derive(Clone)]
pub struct Vector<T, const N: usize> {
     pub elts: [T; N]
}

impl<T, const N: usize> Vector<T, N> {
    pub fn new(e: [T; N]) -> Self {
        Self {
            elts: e
        }
    }
}

impl<T, const N: usize> Mul<&Vector<T, N>> for &Matrix<T, N> where
    for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>,
    T: Add<Output = T> + Zero + Default,
{
    type Output = Vector<T, N>;

    fn mul(self, other: &Vector<T, N>) -> Self::Output
    {
        let mut e: [T; N] = from_fn(|_| T::default());
        for r in 0..N {
            let mut sum = T::zero();
            for c in 0..N {
                let prod = &self.elts[r][c] * &other.elts[r];
                sum = &sum + &prod;
            }
            e[r] = sum;
        }
        Self::Output {
            elts: e
        }
    }
}
