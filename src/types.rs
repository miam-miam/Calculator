use core::fmt;

use gcd::Gcd;

// Not adding Copy as will be using Vec
#[derive(PartialEq, Clone)]
pub enum Token {
    Integer(i128),
    Fraction(Fraction),
    SIntRoot(SRoot<i128>),
    SFracRoot(SRoot<Fraction>),
    CIntRoot(CRoot<i128>),
    CFracRoot(CRoot<Fraction>),
    PiInteger(Pi<i128>),
    PiFraction(Pi<Fraction>),
    PiSIntRoot(Pi<SRoot<i128>>),
    PiSFracRoot(Pi<SRoot<Fraction>>),
    PiCIntRoot(Pi<CRoot<i128>>),
    PiCFracRoot(Pi<CRoot<Fraction>>),
    Double(f64),
}

impl Token {
    #[inline]
    pub fn fraction(int: i128, num: i128, den: i128) -> Token {
        Token::Fraction(Fraction { int, num, den })
    }
    #[inline]
    pub fn s_int_root(mul: i128, base: i128) -> Token {
        Token::SIntRoot(SRoot { mul, base })
    }
    #[inline]
    pub fn c_int_root(mul: i128, base: i128) -> Token {
        Token::CIntRoot(CRoot { mul, base })
    }
    #[inline]
    pub fn s_frac_root(int: i128, num: i128, den: i128, base: i128) -> Token {
        Token::SFracRoot(SRoot {
            mul: Fraction { int, num, den },
            base,
        })
    }
    #[inline]
    pub fn c_frac_root(int: i128, num: i128, den: i128, base: i128) -> Token {
        Token::CFracRoot(CRoot {
            mul: Fraction { int, num, den },
            base,
        })
    }
    #[inline]
    pub fn s_fraction_root(mul: Fraction, base: i128) -> Token {
        Token::SFracRoot(SRoot { mul, base })
    }
    #[inline]
    pub fn c_fraction_root(mul: Fraction, base: i128) -> Token {
        Token::CFracRoot(CRoot { mul, base })
    }
    #[inline]
    pub fn pi_integer(int: i128) -> Token {
        Token::PiInteger(Pi { mul: int })
    }
    #[inline]
    pub fn pi_fraction(int: i128, num: i128, den: i128) -> Token {
        Token::PiFraction(Pi {
            mul: Fraction { int, num, den },
        })
    }
    #[inline]
    pub fn pi_s_int_root(mul: i128, base: i128) -> Token {
        Token::PiSIntRoot(Pi {
            mul: SRoot { mul, base },
        })
    }
    #[inline]
    pub fn pi_c_int_root(mul: i128, base: i128) -> Token {
        Token::PiCIntRoot(Pi {
            mul: CRoot { mul, base },
        })
    }
    #[inline]
    pub fn pi_s_frac_root(int: i128, num: i128, den: i128, base: i128) -> Token {
        Token::PiSFracRoot(Pi {
            mul: SRoot {
                mul: Fraction { int, num, den },
                base,
            },
        })
    }
    #[inline]
    pub fn pi_c_frac_root(int: i128, num: i128, den: i128, base: i128) -> Token {
        Token::PiCFracRoot(Pi {
            mul: CRoot {
                mul: Fraction { int, num, den },
                base,
            },
        })
    }
    #[inline]
    pub fn pi_s_fraction_root(mul: Fraction, base: i128) -> Token {
        Token::PiSFracRoot(Pi {
            mul: SRoot { mul, base },
        })
    }
    #[inline]
    pub fn pi_c_fraction_root(mul: Fraction, base: i128) -> Token {
        Token::PiCFracRoot(Pi {
            mul: CRoot { mul, base },
        })
    }

    pub fn pi(&self) -> Result<Token, MathError> {
        match *self {
            Token::Integer(0) => Ok(Token::Integer(0)),
            Token::Integer(i) => Ok(Token::pi_integer(i)),
            Token::Fraction(i) => Ok(Token::PiFraction(Pi::new(i))),
            Token::SIntRoot(i) => Ok(Token::PiSIntRoot(Pi::new(i))),
            Token::SFracRoot(i) => Ok(Token::PiSFracRoot(Pi::new(i))),
            Token::CIntRoot(i) => Ok(Token::PiCIntRoot(Pi::new(i))),
            Token::CFracRoot(i) => Ok(Token::PiCFracRoot(Pi::new(i))),
            Token::Double(i) => Ok(Token::Double(double_check!(i * std::f64::consts::PI))),
            _ => unreachable!(),
        }
    }

    pub fn normal(&self) -> Token {
        match *self {
            Token::PiInteger(i) => Token::Integer(i.mul),
            Token::PiFraction(i) => Token::Fraction(i.mul),
            Token::PiSIntRoot(i) => Token::SIntRoot(i.mul),
            Token::PiSFracRoot(i) => Token::SFracRoot(i.mul),
            Token::PiCIntRoot(i) => Token::CIntRoot(i.mul),
            Token::PiCFracRoot(i) => Token::CFracRoot(i.mul),
            _ => unreachable!(),
        }
    }

    /// This function does not check if the f64 is valid as such it is recommended to check with double_check!() once the computations are finished.
    pub fn double(&self) -> f64 {
        match *self {
            Token::Integer(i) => i as f64,
            Token::Fraction(i) => i.int as f64 + i.num as f64 / i.den as f64,
            Token::SIntRoot(i) => (i.mul as f64) * (i.base as f64).sqrt(),
            Token::SFracRoot(i) => {
                (i.mul.int as f64 + i.mul.num as f64 / i.mul.den as f64) * (i.base as f64).cbrt()
            }
            Token::CIntRoot(i) => (i.mul as f64) * (i.base as f64).sqrt(),
            Token::CFracRoot(i) => {
                (i.mul.int as f64 + i.mul.num as f64 / i.mul.den as f64) * (i.base as f64).cbrt()
            }
            Token::PiInteger(i) => i.mul as f64 * std::f64::consts::PI,
            Token::PiFraction(i) => {
                (i.mul.int as f64 + i.mul.num as f64 / i.mul.den as f64) * std::f64::consts::PI
            }
            Token::PiSIntRoot(i) => {
                (i.mul.mul as f64) * (i.mul.base as f64).sqrt() * std::f64::consts::PI
            }
            Token::PiSFracRoot(i) => {
                (i.mul.mul.int as f64 + i.mul.mul.num as f64 / i.mul.mul.den as f64)
                    * (i.mul.base as f64).cbrt()
                    * std::f64::consts::PI
            }
            Token::PiCIntRoot(i) => {
                (i.mul.mul as f64) * (i.mul.base as f64).sqrt() * std::f64::consts::PI
            }
            Token::PiCFracRoot(i) => {
                (i.mul.mul.int as f64 + i.mul.mul.num as f64 / i.mul.mul.den as f64)
                    * (i.mul.base as f64).cbrt()
                    * std::f64::consts::PI
            }
            Token::Double(i) => i,
        }
    }

    pub fn is_pi(&self) -> bool {
        match self {
            Token::PiInteger(_)
            | Token::PiFraction(_)
            | Token::PiSIntRoot(_)
            | Token::PiSFracRoot(_)
            | Token::PiCIntRoot(_)
            | Token::PiCFracRoot(_) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(i) => write!(f, "Integer: {}", i),
            Token::Fraction(fr) => write!(f, "Fraction: {}", fr),
            Token::SIntRoot(r) => write!(f, "Sqrt: {}", r),
            Token::SFracRoot(r) => write!(f, "Sqrt: {}", r),
            Token::CIntRoot(r) => write!(f, "Cbrt: {}", r),
            Token::CFracRoot(r) => write!(f, "Cbrt: {}", r),
            Token::PiInteger(i) => write!(f, "PiInteger: {}", i),
            Token::PiFraction(fr) => write!(f, "PiFraction: {}", fr),
            Token::PiSIntRoot(r) => write!(f, "PiSqrt: {}", r),
            Token::PiSFracRoot(r) => write!(f, "PiSqrt: {}", r),
            Token::PiCIntRoot(r) => write!(f, "PiCbrt: {}", r),
            Token::PiCFracRoot(r) => write!(f, "PiCbrt: {}", r),
            Token::Double(d) => write!(f, "Double: {}", d),
        }
    }
}

#[derive(Debug, Eq, Copy, PartialEq, Clone)]
pub enum MathError {
    None,
    SyntaxError,
    Overflow,
    DoubleOverflow,
    DivisionByZero,
    ComplexNumber,
    // For 0^0
    ExponentiationError,
    InvalidDecimalPoint,
    // Using Fraction to store int
    InvalidFraction,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::None => write!(f, "All good!"),
            MathError::SyntaxError => write!(f, "Incorrect syntax"),
            MathError::Overflow => write!(f, "Overflow"),
            MathError::DoubleOverflow => write!(f, "Proper overflow"),
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::ComplexNumber => write!(f, "Complex numbers not implemented"),
            MathError::ExponentiationError => write!(f, "Cannot compute 0^0"),
            MathError::InvalidDecimalPoint => write!(f, "Invalid decimal point"),
            MathError::InvalidFraction => write!(f, "Fraction should be integer"),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Fraction {
    pub int: i128,
    pub num: i128,
    pub den: i128,
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.num < 0 {
            write!(f, "{}{}/{}", self.int, self.num, self.den)
        } else {
            write!(f, "{}+{}/{}", self.int, self.num, self.den)
        }
    }
}

impl Fraction {
    pub fn new(int: i128, num: i128, den: i128) -> Fraction {
        Fraction { int, num, den }
    }

    pub fn normalise(&mut self) -> Result<(), MathError> {
        // At the end den must be positive and int must have the same sign as num
        if self.den == 0 {
            return Err(MathError::DivisionByZero);
        }
        if self.den < 0 {
            self.num = mul!(self.num, -1);
            self.den = mul!(self.den, -1);
        }
        if self.den == 1 {
            self.int = add!(self.int, self.num);
            self.num = 0;
            return Err(MathError::InvalidFraction);
        }
        if (self.num > 0 && self.num >= self.den) || (self.num < 0 && -self.num >= self.den) {
            self.int = add!(self.int, self.num / self.den);
            self.num -= (self.num / self.den) * self.den;
        }
        if self.num == 0 {
            self.den = 1;
            return Err(MathError::InvalidFraction);
        }
        let gcd: i128 = ((self.num.abs() as u128).gcd(self.den.abs() as u128)) as i128;
        self.num /= gcd;
        self.den /= gcd;

        // If different sign
        if self.num < 0 && self.int > 0 {
            self.num += self.den;
            self.int -= 1;
        } else if self.num > 0 && self.int < 0 {
            self.num -= self.den;
            self.int += 1;
        }
        Ok(())
    }

    pub fn add(&mut self, rhs: &Fraction) -> Result<(), MathError> {
        self.int = add!(self.int, rhs.int);
        self.num = add!(mul!(self.num, rhs.den), mul!(self.den, rhs.num));
        self.den = mul!(self.den, rhs.den);
        self.normalise()
    }

    pub fn sub(&mut self, rhs: &Fraction) -> Result<(), MathError> {
        self.int = sub!(self.int, rhs.int);
        self.num = sub!(mul!(self.num, rhs.den), mul!(self.den, rhs.num));
        self.den = mul!(self.den, rhs.den);
        self.normalise()
    }

    pub fn mul(&mut self, rhs: &Fraction) -> Result<(), MathError> {
        self.num = add!(
            mul!(self.num, rhs.num),
            add!(
                mul!(mul!(rhs.int, rhs.den), self.num),
                mul!(mul!(self.den, rhs.num), self.int)
            )
        );
        self.int = mul!(self.int, rhs.int);
        self.den = mul!(self.den, rhs.den);
        self.normalise()
    }

    pub fn div(&mut self, rhs: &Fraction) -> Result<(), MathError> {
        self.num = mul!(rhs.den, add!(self.num, mul!(self.int, self.den)));
        self.den = mul!(self.den, add!(rhs.num, mul!(rhs.int, rhs.den)));
        self.int = 0;
        self.normalise()
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct SimpleFraction {
    pub num: i32,
    pub den: i32,
}

impl SimpleFraction {
    pub fn new(num: i32, den: i32) -> SimpleFraction {
        SimpleFraction { num, den }
    }
}

impl fmt::Display for SimpleFraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct SRoot<T> {
    pub mul: T,
    pub base: i128,
}

impl<T> SRoot<T> {
    pub fn new(mul: T, base: i128) -> SRoot<T> {
        SRoot { mul, base }
    }
}

impl<T: fmt::Display> fmt::Display for SRoot<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})*√{}", self.mul, self.base)
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct CRoot<T> {
    pub mul: T,
    pub base: i128,
}

impl<T> CRoot<T> {
    pub fn new(mul: T, base: i128) -> CRoot<T> {
        CRoot { mul, base }
    }
}

impl<T: fmt::Display> fmt::Display for CRoot<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})*∛{}", self.mul, self.base)
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Pi<T> {
    pub mul: T,
}

impl<T> Pi<T> {
    pub fn new(mul: T) -> Pi<T> {
        Pi { mul }
    }
}

impl<T: fmt::Display> fmt::Display for Pi<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})*π", self.mul)
    }
}
