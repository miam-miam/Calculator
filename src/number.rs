use crate::my_math;
use core::fmt;
use gcd::Gcd;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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
    pub fn normalise(&mut self) -> my_math::MathError {
        // At the end den must be positive
        if self.den == 0 {
            return my_math::MathError::DivisionByZero;
        }
        if self.den < 0 {
            self.num = mul!(self.num, -1);
            self.den = mul!(self.den, -1);
        }
        if self.den == 1 {
            self.int = add!(self.int, self.num);
            self.num = 0;
            return my_math::MathError::InvalidFraction;
        }
        if self.num >= self.den {
            self.int = add!(self.int, self.num / self.den);
            self.num -= (self.num / self.den) * self.den;
        }
        if self.num == 0 {
            self.den = 1;
            return my_math::MathError::InvalidFraction;
        }
        let gcd: i128 = ((self.num.abs() as u128).gcd(self.den.abs() as u128)) as i128;
        self.num /= gcd;
        self.den /= gcd;
        my_math::MathError::None
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct SimpleFraction {
    pub num: i128,
    pub den: i128,
}

impl fmt::Display for SimpleFraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}
