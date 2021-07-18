use crate::my_math::MathError;
use core::fmt;
use gcd::Gcd;

#[macro_use]
mod macro_num {
    macro_rules! mul {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_mul($rhs) {Some(i) => i, None => return MathError::Overflow});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_mul($rhs) {Some(i) => i, None => return $error});
    }
    macro_rules! add {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_add($rhs) {Some(i) => {i}, None =>  return MathError::Overflow});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_add($rhs) {Some(i) => i, None => return $error});
    }
    macro_rules! sub {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_sub($rhs) {Some(i) => i, None => return MathError::Overflow});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_sub($rhs) {Some(i) => i, None => return $error});
    }
    macro_rules! div {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_div($rhs) {Some(i) => i, None => return MathError::Overflow});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_div($rhs) {Some(i) => i, None => return $error});
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Fraction {
    pub int: i128,
    pub num: i128,
    pub den: i128,
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}/{}", self.int, self.num, self.den)
    }
}

impl Fraction {
    pub fn normalise(&mut self) -> MathError {
        // At the end den must be positive
        if self.den == 0 {
            return MathError::DivisionByZero;
        }
        if self.den < 0 {
            self.num = mul!(self.num, -1);
            self.den = mul!(self.den, -1);
        }
        if self.den == 1 {
            self.int = add!(self.int, self.num);
            self.num = 0;
            return MathError::InvalidFraction;
        }
        if self.num >= self.den {
            self.int = add!(self.int, self.num / self.den);
            self.num -= (self.num / self.den) * self.den;
        }
        if self.num == 0 {
            self.den = 1;
            return MathError::InvalidFraction;
        }
        let gcd: i128 = ((self.num.abs() as u128).gcd(self.den.abs() as u128)) as i128;
        self.num /= gcd;
        self.den /= gcd;
        MathError::None
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct SimpleFraction {
    pub num: i128,
    pub den: i128,
}

impl fmt::Display for SimpleFraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}
