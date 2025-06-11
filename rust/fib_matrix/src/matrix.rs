use num_traits::{One, Pow, Unsigned, Zero};

use std::array::from_fn;
use std::cmp::PartialEq;
use std::default::Default;
use std::ops::{Add, Div, Index, IndexMut, Mul, Rem};

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<T, const M: usize, const N: usize>  {
    // M rows and N columns
    elts: [[T; N]; M],
}

impl <T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(elts: [[T; N]; M]) -> Self {
        Self {
            elts: elts
        }
    }

    pub fn from_unsigned_array<S>(elts: &[[S; N]; M]) -> Self where
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

impl<T, const M: usize, const N: usize>
    Index<usize> for Matrix<T, M, N>
where
{
    type Output = [T; N];
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.elts[index]
    }
}

impl<T, const M: usize, const N: usize>
    IndexMut<usize> for Matrix<T, M, N>
where
{
    fn index_mut(&mut self, index: usize) -> &mut [T; N] {
        &mut self.elts[index]
    }
}

impl<T, const M: usize, const N: usize>
    Default for Matrix<T, M, N>
where
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

// Multiplicative identity defined only for square matrices
impl<T, const N: usize>
    One for Matrix<T, N, N>
where
    Matrix<T, N, N>: Mul<Output = Matrix<T, N, N>>,
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

impl<T, const M: usize, const N: usize>
    Zero for Matrix<T, M, N>
where
    Matrix<T, M, N>: Add<Output = Matrix<T, M, N>>,
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


impl<T, const L: usize, const M: usize, const N: usize>
    Mul<&Matrix<T, M, N>> for &Matrix<T, L, M>
where
    for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>,
    T: std::iter::Sum,
{
    type Output = Matrix<T, L, N>;

    fn mul(self, other: &Matrix<T, M, N>) -> Self::Output {
        Self::Output {
            elts: from_fn(
                |r| from_fn(
                    |c| (0..M)
                        .map(|n| &self.elts[r][n] * &other.elts[n][c])
                        .sum()
                )
            )
        }
    }
}

impl<T, const M: usize, const N: usize>
    Add<&Matrix<T, M, N>> for &Matrix<T, M, N>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    type Output = Matrix<T, M, N>;

    fn add(self, other: &Matrix<T, M, N>) -> Self::Output {
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

impl<T, const N: usize>
    Pow<T> for Matrix<T, N, N>
where
    Matrix<T, N, N>: One + Clone + Mul,
    for<'a> &'a T: Add<&'a T, Output = T> + Div<u8, Output = T> + Mul<&'a T, Output = T> + Rem<u8, Output = T>,
    for<'a> &'a Matrix<T, N, N>: Mul<&'a Matrix<T, N, N>, Output = Matrix<T, N, N>>,
    T: Clone + PartialEq + Zero + std::iter::Sum,
{
    type Output = Matrix<T, N, N>;

    fn pow(self, exp: T) -> Matrix<T, N, N>
    {
        let mut b = self.clone();
        let mut n = exp.clone();
        let mut res = Matrix::<T, N, N>::one();
        while !n.is_zero() {
            if (&n % 2u8) != T::zero() {
                res = &res * &b;
            }
            b = &b * &b;
            n = &n / 2u8;
        }
        res
    }
}

impl<T, const N: usize>
    Pow<usize> for Matrix<T, N, N>
where
    Matrix<T, N, N>: One + Clone + Mul + for<'a> Pow<&'a T, Output = Matrix<T, N, N>>,
    for<'a> &'a T: Add<&'a T, Output = T> + Div<u8, Output = T> + Mul<&'a T, Output = T> + Rem<u8, Output = T>,
    for<'a> &'a Matrix<T, N, N>: Mul<&'a Matrix<T, N, N>, Output = Matrix<T, N, N>>,
    T: Clone + PartialEq + Zero + std::iter::Sum + From<usize>,
{
    type Output = Self;

    fn pow(self, exp: usize) -> Matrix<T, N, N> {
        let texp: T = exp.into();
        self.pow(&texp)
    }
}

impl<T, const L: usize, const M: usize, const N: usize>
    Mul<Matrix<T, M, N>> for Matrix<T, L, M>
where
    for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>,
    T: std::iter::Sum,
{
     type Output = Matrix<T, L, N>;

     fn mul(self, other: Matrix<T, M, N>) -> Self::Output {
         &self * &other
     }
}

impl<T, const M: usize, const N: usize>
    Add<Matrix<T, M, N>> for Matrix<T, M, N>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    type Output = Self;

    fn add(self, other: Matrix<T, M, N>) -> Self::Output {
        &self + &other
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Vector<T, const N: usize> {
     elts: [T; N]
}

impl<T, const N: usize> Vector<T, N> {
    pub fn new(e: [T; N]) -> Self {
        Self {
            elts: e
        }
    }

    pub fn from_unsigned_array<S>(elts: &[S; N]) -> Self where
        S: Unsigned + Clone,
        T: From<S>,
    {
        Self::new(from_fn(|ix| elts[ix].clone().into()))
    }

    pub fn get<'a>(&'a self, ix: usize) -> &'a T {
        &self.elts[ix]
    }

    pub fn set(&mut self, ix: usize, elt: T) {
        self.elts[ix] = elt;
    }
}

impl<T, const N: usize>
    Vector<T, N>
where
    for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>,
    T: Add<Output = T> + std::iter::Sum,
{
    pub fn dot(&self, other: &Vector<T, N>) -> T {
        (0..N).map(|ix| &self.elts[ix] * &other.elts[ix]).sum()
    }
}

impl<T, const N: usize>
    Index<usize> for Vector<T, N>
where
{
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        self.get(index)
    }
}

impl<T, const N: usize>
    IndexMut<usize> for Vector<T, N>
where
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.elts[index]
    }
}

impl<T, const N: usize>
    Default for Vector<T, N>
where
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

impl<T, const M: usize, const N: usize>
    Mul<&Vector<T, N>> for &Matrix<T, M, N>
where
    for<'a> &'a T: Add<&'a T, Output = T> + Mul<&'a T, Output = T>,
    T: std::iter::Sum,
{
    type Output = Vector<T, M>;

    fn mul(self, other: &Vector<T, N>) -> Self::Output
    {
        Self::Output {
            elts: from_fn(
                |ix| (0..N)
                    .map(|c| &self.elts[ix][c] * &other.elts[c])
                    .sum()
            )
        }
    }
}

impl<T, const N: usize>
    Add<&Vector<T, N>> for &Vector<T, N>
where
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

#[cfg(test)]
mod test {
    use super::*;
    use num::BigUint;
    use std::str::FromStr;

    #[test]
    fn create_and_access() {
        let m = Matrix::<BigUint, 2usize, 3usize>::from_unsigned_array(
            &[
                [ 1, 2, 3 ],
                [ 4, 5, 6u8 ]
            ]
        );
        assert_eq!(m[0][0], 1u8.into());
        assert_eq!(m[1][2], 6u8.into());
    }

    #[test]
    fn modify_and_compare() {
        let mut m = Matrix::<BigUint, 2usize, 3usize>::from_unsigned_array(
            &[
                [ 1, 2, 3 ],
                [ 4, 5, 6u8 ]
            ]
        );
        m[0][0] = 7u8.into();
        assert_eq!(m, Matrix::<BigUint, 2usize, 3usize>::from_unsigned_array(
            &[
                [ 7, 2, 3 ],
                [ 4, 5, 6u8 ]
            ]
        ));
    }

    fn fib(n: usize) -> BigUint {
        let t = Matrix::<BigUint, 2usize, 2usize>::from_unsigned_array(
            &[
                [1, 1],
                [1, 0u8],
            ]
        );
        let tn = t.pow(n.into());
        let state = Vector::<BigUint, 2usize>::from_unsigned_array(
            &[1, 0u8]);
        let res = &tn * &state;
        res[1].clone()
    }

    fn fib_slow(mut n: usize) -> BigUint {
        let mut a: BigUint = 0u32.into();
        let mut b: BigUint = 1u32.into();

        while n > 0 {
            (b, a) = (&a + &b, b);
            n = n - 1;
        }
        a
    }

    #[test]
    fn big_fib() {
        assert_eq!(fib(5000), fib_slow(5000));
        assert_eq!(fib(5000),
                   BigUint::from_str("3878968454388325633701916308325905312082127714646245106160597214895550139044037097010822916462210669479293452858882973813483102008954982940361430156911478938364216563944106910214505634133706558656238254656700712525929903854933813928836378347518908762970712033337052923107693008518093849801803847813996748881765554653788291644268912980384613778969021502293082475666346224923071883324803280375039130352903304505842701147635242270210934637699104006714174883298422891491273104054328753298044273676822977244987749874555691907703880637046832794811358973739993110106219308149018570815397854379195305617510761053075688783766033667355445258844886241619210553457493675897849027988234351023599844663934853256411952221859563060475364645470760330902420806382584929156452876291575759142343809142302917491088984155209854432486594079793571316841692868039545309545388698114665082066862897420639323438488465240988742395873801976993820317174208932265468879364002630797780058759129671389634214252579116872755600360311370547754724604639987588046985178408674382863125").expect("test coding error?!?"));
    }

    #[test]
    fn rectangular_mul() {
        let m = Matrix::<BigUint, 2usize, 3usize>::from_unsigned_array(
            &[
                [1, 2, 3],
                [4, 5, 6u8],
            ]);
        let n = Matrix::<BigUint, 3usize, 2usize>::from_unsigned_array(
            &[
                [4, 3],
                [2, 1],
                [1, 2u8],
            ]);
        let prod = m * n;
        let expected = Matrix::<BigUint, 2usize, 2usize>::from_unsigned_array(
            &[
                [4 + 2*2 + 3, 3 + 2 + 2 * 3],
                [4 * 4 + 2 * 5 + 6, 3 * 4 + 5 + 2 * 6u16],
            ]);
        assert_eq!(&prod, &expected);
    }

    #[test]
    fn vector_mul_and_dot() {
        let n = Matrix::<BigUint, 3usize, 2usize>::from_unsigned_array(
            &[
                [4, 3],
                [2, 1],
                [1, 2u8],
            ]);
        let v = Vector::<BigUint, 2usize>::from_unsigned_array(
            &[2, 3u8]);
        let w = &n * &v;
        let expected = Vector::<BigUint, 3usize>::from_unsigned_array(
            &[2*4 + 3*3, 2*2 + 3*1, 2*1 + 3*2u16]);
        assert_eq!(&w, &expected);

        let dot = v.dot(&v);
        assert_eq!(dot, (4 + 9u8).into());
    }
}
