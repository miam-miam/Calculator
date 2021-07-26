use core::fmt;

use gcd::Gcd;

#[derive(PartialEq, Copy, Clone)]
pub enum Token {
    Integer(i128),
    Fraction(Fraction),
    SIntRoot(SRoot<i128>),
    SFracRoot(SRoot<Fraction>),
    CIntRoot(CRoot<i128>),
    CFracRoot(CRoot<Fraction>),
    Double(f64),
    None,
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponentiation,
    LBracket,
    RBracket,
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
            Token::Double(d) => write!(f, "Double: {}", d),
            Token::None => write!(f, "None"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Multiply => write!(f, "Multiply"),
            Token::Divide => write!(f, "Divide"),
            Token::Exponentiation => write!(f, "Exponentiation"),
            Token::LBracket => write!(f, "LBracket"),
            Token::RBracket => write!(f, "RBracket"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MathError {
    None,
    UnmatchedBracket,
    UnknownOperator,
    Overflow,
    DoubleOverflow,
    DivisionByZero,
    ComplexNumber,
    // For 0^0
    ExponentiationError,
    InvalidDecimalPoint,
    Error,
    // Using Fraction to store int
    InvalidFraction,
    Impossible,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::None => write!(f, "All good!"),
            MathError::UnmatchedBracket => write!(f, "Unmatched bracket"),
            MathError::UnknownOperator => write!(f, "Unknown operator"),
            MathError::Overflow => write!(f, "Overflow"),
            MathError::DoubleOverflow => write!(f, "Proper Overflow"),
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::ComplexNumber => write!(f, "Complex numbers not implemented"),
            MathError::ExponentiationError => write!(f, "Cannot compute 0^0"),
            MathError::InvalidDecimalPoint => write!(f, "Invalid decimal point"),
            MathError::InvalidFraction => write!(f, "Fraction should be integer"),
            MathError::Error => write!(f, "A general error happened"),
            MathError::Impossible => write!(f, "Not possible"),
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
