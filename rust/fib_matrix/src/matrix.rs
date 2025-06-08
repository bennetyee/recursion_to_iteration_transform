use num_traits::Zero;
use num_traits::One;
use num_traits::Unsigned;

use std::array::from_fn;
use std::default::Default;
use std::ops::Mul;
use std::ops::Add;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Matrix<T, const N: usize>  {
    elts: [[T; N]; N],
}

impl <T, const N: usize> Matrix<T, N> {
    pub fn new(elts: [[T; N]; N]) -> Self {
        Self {
            elts: elts
        }
    }

    pub fn from_unsigned<S>(elts: &[[S; N]; N]) -> Self where
        S: Unsigned + Clone,
        T: From<S>,
    {
        let telts = from_fn(
            |r| from_fn(
                |c| elts[r][c].clone().into()
            )
        );
        Self::new(telts)
    }

    pub fn get<'a>(&'a self, r: usize, c: usize) -> &'a T {
        &self.elts[r][c]
    }

    pub fn set(&mut self, r: usize, c: usize, elt: T) {
        self.elts[r][c] = elt;
    }
}

impl<T, const N: usize> Index<usize> for Matrix<T, N> where
{
    type Output = [T; N];
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.elts[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Matrix<T, N> where
{
    fn index_mut(&mut self, index: usize) -> &mut [T; N] {
        &mut self.elts[index]
    }
}

impl<T, const N: usize> Default for Matrix<T, N> where
    T: Default,
{
    fn default() -> Self {
        Self {
            elts: from_fn(
                |_| from_fn(
                    |_| T::default()
                )
            )
        }
    }
}

impl<T, const N: usize> One for Matrix<T, N> where
    Matrix<T, N>: Mul<Output = Matrix<T, N>>,
    T: Zero + One + Default + Add + Mul,
{
    fn one() -> Self {
        Self {
            elts: from_fn(
                |r| from_fn(
                    |c| if r == c {
                        T::one()
                    } else {
                        T::zero()
                    }
                )
            )
        }
    }
}

impl<T, const N: usize> Zero for Matrix<T, N> where
    Matrix<T, N>: Add<Output = Matrix<T, N>>,
    T: Zero,
{
    fn zero() -> Self {
        Self {
            elts: from_fn(|_| from_fn(|_| T::zero()))
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
    T: std::iter::Sum,
{
    type Output = Matrix<T, N>;

    fn mul(self, other: &Matrix<T, N>) -> Self::Output {
        Self::Output {
            elts: from_fn(
                |r| from_fn(
                    |c| (0..N)
                        .map(|n| &self.elts[r][n] * &other.elts[n][c])
                        .sum()
                )
            )
        }
    }
}

impl<T, const N: usize> Add<&Matrix<T, N>> for &Matrix<T, N> where
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    type Output = Matrix<T, N>;

    fn add(self, other: &Matrix<T, N>) -> Self::Output {
        Self::Output {
            elts: from_fn(
                |r| from_fn(
                    |c| {
                        &self.elts[r][c] + &other.elts[r][c]
                    }
                )
            )
        }
    }
}

impl<T, const N: usize> Mul<Matrix<T, N>> for Matrix<T, N> where
    for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>,
    T: std::iter::Sum,
{
     type Output = Self;

     fn mul(self, other: Matrix<T, N>) -> Self::Output {
         &self * &other
     }
}

impl<T, const N: usize> Add<Matrix<T, N>> for Matrix<T, N> where
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    type Output = Self;

    fn add(self, other: Matrix<T, N>) -> Self::Output {
        &self + &other
    }
}

#[derive(Clone)]
pub struct Vector<T, const N: usize> {
     elts: [T; N]
}

impl<T, const N: usize> Vector<T, N> {
    pub fn new(e: [T; N]) -> Self {
        Self {
            elts: e
        }
    }

    pub fn get<'a>(&'a self, ix: usize) -> &'a T {
        &self.elts[ix]
    }

    pub fn set(&mut self, ix: usize, elt: T) {
        self.elts[ix] = elt;
    }
}

impl<T, const N: usize> Vector<T, N> where
    for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>,
    T: Add<Output = T> + std::iter::Sum,
{
    pub fn dot(&self, other: &Vector<T, N>) -> T {
        (0..N).map(|ix| &self.elts[ix] * &other.elts[ix]).sum()
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> where
{
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        self.get(index)
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> where
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.elts[index]
    }
}

impl<T, const N: usize> Default for Vector<T, N> where
    T: Default
{
    fn default() -> Self {
        let elts: [T; N] = from_fn(|_| T::default());
        Self {
            elts
        }
    }
}

impl<T, const N: usize> Zero for Vector<T, N> where
    Vector<T, N>: Add<Output = Vector<T, N>>,
    T: Zero,
{
    fn zero() -> Self {
        Self {
            elts: from_fn(|_| T::zero())
        }
    }

    fn is_zero(&self) -> bool {
        for ix in 0..N {
            if !self.elts[ix].is_zero() {
                return false;
            }
        }
        return true;
    }
}

impl<T, const N: usize> Mul<&Vector<T, N>> for &Matrix<T, N> where
    for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>,
    T: Zero + Default,
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

impl<T, const N: usize> Add<&Vector<T, N>> for &Vector<T, N> where
    for<'a> &'a T: Add<&'a T, Output = T>,
    T: Zero,
{
    type Output = Vector<T, N>;

    fn add(self, other: &Vector<T, N>) -> Self::Output {
        let e = from_fn(|ix| &self.elts[ix] + &other.elts[ix]);
        Self::Output {
            elts: e
        }
    }
}
