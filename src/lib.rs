#![allow(dead_code)]

use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};

use math_concept::{
    zero::Zero,
    one::One,
    pow::Pow,
    sqrt::Sqrt,
};

use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self {
            re,
            im,
        }
    }
}

impl<T> Complex<T>
where
    T: Neg<Output = T>,
{
    pub fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl<T> Complex<T>
where
    T: One + Add<Output = T> + Pow<T, Output = T> + Sqrt<Output = T> + Clone,
{
    pub fn abs(&self) -> T {
        let two = T::one() + T::one();
        let pow_sum =
            self.re.clone().pow(two.clone()) + self.im.clone().pow(two);
        pow_sum.sqrt()
    }
}
/*
impl<T> Complex<T>
where
    T: Real + Copy,
{
    pub fn arg(&self) -> T {
        let re = self.re;
        let im = self.im;

        let div = im / (re + self.abs());

        (T::one() + T::one()) * div.atan()
    }
}
*/
impl<T> Add for Complex<T>
where
T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            re: self.re + other.re,
            im: self.im + other.im,
        };
    }
}

impl<T> Div for Complex<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Sub<Output = T>
        + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denominator = other.re * other.re + other.im * other.im;
        let real_numerator = self.re * other.re + self.im * other.im;
        let imag_numerator = self.im * other.re - self.re * other.im;
        Self {
            re: real_numerator / denominator,
            im: imag_numerator / denominator,
        }
    }
}

impl<T> DivAssign for Complex<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Sub<Output = T>
    + Copy,
{
    fn div_assign(&mut self, other: Self) {
        let denominator = other.re * other.re + other.im * other.im;
        let real_numerator = self.re * other.re + self.im * other.im;
        let imag_numerator = self.im * other.re - self.re * other.im;
        *self = Self {
            re: real_numerator / denominator,
            im: imag_numerator / denominator,
        };
    }
}

impl<T> Mul for Complex<T>
where
T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.im * other.re + self.re * other.im,
        }
    }
}

impl<T> MulAssign for Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        };
    }
}

impl<T> Sub for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl<T> SubAssign for Complex<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            re: self.re - other.re,
            im: self.im - other.im,
        };
    }
}

impl<T> Zero for Complex<T>
where
T: Zero,
{
    fn zero() -> Self {
        Self {
            re: T::zero(),
            im: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.re.is_zero() && self.im.is_zero()
    }
}

impl<T> fmt::Display for Complex<T>
where
T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}i", self.re, self.im)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
/*
    fn round_at<T>(value: T, nb_digits: usize) -> T
    where
        T: One + Real + From<usize>,
    {
        let mul = T::one() * nb_digits.into();
        (value * mul).round() / mul
    }
*/
    #[test]
    fn test_conjugate() {
        let a = Complex::new(1, 2);
        let res = Complex::new(1, -2);
        assert_eq!(a.conjugate(), res);
    }

    #[test]
    fn test_abs() {
        let a = Complex::new(-3, 0);
        let b = Complex::new(0, 6);
        assert_eq!(a.abs(), 3);
        assert_eq!(b.abs(), 6)
    }
/*
    #[test]
    fn test_arg() {
        let a = Complex::new(1.0, 1.0);
        let b = Complex::new(14.0, -3.0);

        assert_eq!(
            round_at(a.arg(), 4),
            round_at(std::f64::consts::FRAC_PI_4, 4)
        );
        assert_eq!(b.arg(), -(3.0/14.0).atan());
    }
*/
    #[test]
    fn test_add() {
        let a = Complex::new(3, 4);
        let b = Complex::new(6, -10);
        let res = Complex::new(9, -6);

        assert_eq!(a+b, res);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Complex::new(3, 4);
        let b = Complex::new(6, -10);
        let res = Complex::new(9, -6);

        a += b;

        assert_eq!(a, res);
    }

    #[test]
    fn test_div() {
        let a = Complex::new(20, -4);
        let b = Complex::new(3, 2);
        let res = Complex::new(4, -4);

        assert_eq!(a/b, res);
    }

    #[test]
    fn test_div_assign() {
        let mut a = Complex::new(20, -4);
        let b = Complex::new(3, 2);
        let res = Complex::new(4, -4);

        a /= b;

        assert_eq!(a, res);
    }

    #[test]
    fn test_mul() {
        let a = Complex::new(2, 3);
        let b = Complex::new(1, -5);
        let res = Complex::new(17, -7);

        assert_eq!(a*b, res);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Complex::new(2, 3);
        let b = Complex::new(1, -5);
        let res = Complex::new(17, -7);

        a *= b;

        assert_eq!(a, res);
    }

    #[test]
    fn test_sub() {
        let a = Complex::new(3, 4);
        let b = Complex::new(6, -10);
        let res = Complex::new(-3, 14);

        assert_eq!(a-b, res);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Complex::new(3, 4);
        let b = Complex::new(6, -10);
        let res = Complex::new(-3, 14);

        a -= b;

        assert_eq!(a, res);
    }

    #[test]
    fn test_zero() {
        let res = Complex::new(0, 0);
        assert_eq!(Complex::zero(), res);
    }
}
