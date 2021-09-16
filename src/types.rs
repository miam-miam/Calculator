use crate::number::{try_add, try_mul};
use core::fmt;
use gcd::Gcd;
use std::mem::discriminant;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Copy, Clone)]
pub enum BasicToken {
    Integer(i128),
    Fraction(Fraction),
    SIntRoot(SRoot<i128>),
    SFracRoot(SRoot<Fraction>),
    CIntRoot(CRoot<i128>),
    CFracRoot(CRoot<Fraction>),
    Double(f64),
}

impl BasicToken {
    #[inline]
    pub fn fraction(int: i128, num: i128, den: i128) -> BasicToken {
        BasicToken::Fraction(Fraction { int, num, den })
    }
    #[inline]
    pub fn s_int_root(mul: i128, base: i128) -> BasicToken {
        BasicToken::SIntRoot(SRoot { mul, base })
    }
    #[inline]
    pub fn c_int_root(mul: i128, base: i128) -> BasicToken {
        BasicToken::CIntRoot(CRoot { mul, base })
    }
    #[inline]
    pub fn s_frac_root(int: i128, num: i128, den: i128, base: i128) -> BasicToken {
        BasicToken::SFracRoot(SRoot {
            mul: Fraction { int, num, den },
            base,
        })
    }
    #[inline]
    pub fn c_frac_root(int: i128, num: i128, den: i128, base: i128) -> BasicToken {
        BasicToken::CFracRoot(CRoot {
            mul: Fraction { int, num, den },
            base,
        })
    }
    #[inline]
    pub fn s_fraction_root(mul: Fraction, base: i128) -> BasicToken {
        BasicToken::SFracRoot(SRoot { mul, base })
    }
    #[inline]
    pub fn c_fraction_root(mul: Fraction, base: i128) -> BasicToken {
        BasicToken::CFracRoot(CRoot { mul, base })
    }

    /// This function does not check if the f64 is valid as such it is recommended to check with double_check!() once the computations are finished.
    pub fn double(&self) -> f64 {
        match &*self {
            BasicToken::Integer(i) => *i as f64,
            BasicToken::Fraction(i) => i.int as f64 + i.num as f64 / i.den as f64,
            BasicToken::SIntRoot(i) => (i.mul as f64) * (i.base as f64).sqrt(),
            BasicToken::SFracRoot(i) => {
                (i.mul.int as f64 + i.mul.num as f64 / i.mul.den as f64) * (i.base as f64).cbrt()
            }
            BasicToken::CIntRoot(i) => (i.mul as f64) * (i.base as f64).sqrt(),
            BasicToken::CFracRoot(i) => {
                (i.mul.int as f64 + i.mul.num as f64 / i.mul.den as f64) * (i.base as f64).cbrt()
            }
            BasicToken::Double(i) => *i,
        }
    }
    pub fn negate(self) -> Result<BasicToken, MathError> {
        Ok(match self {
            BasicToken::Integer(i) => BasicToken::Integer(mul!(i, -1)),
            BasicToken::Fraction(i) => {
                BasicToken::fraction(mul!(i.int, -1), mul!(i.num, -1), i.den)
            }
            BasicToken::SIntRoot(i) => BasicToken::s_int_root(mul!(i.mul, -1), i.base),
            BasicToken::SFracRoot(i) => BasicToken::SFracRoot(SRoot::new(i.mul.negate()?, i.base)),
            BasicToken::CIntRoot(i) => BasicToken::c_int_root(mul!(i.mul, -1), i.base),
            BasicToken::CFracRoot(i) => BasicToken::CFracRoot(CRoot::new(i.mul.negate()?, i.base)),
            BasicToken::Double(i) => BasicToken::Double(-i),
        })
    }

    pub fn new_s_root(tok: BasicToken, base: i128) -> Result<BasicToken, MathError> {
        Ok(match (tok, base) {
            (BasicToken::Integer(0), _) | (_, 0) => BasicToken::Integer(0),
            (val, 1) => val,
            (BasicToken::Fraction(val), _) => BasicToken::s_fraction_root(val, base),
            (BasicToken::Integer(val), _) => BasicToken::s_int_root(val, base),
            _ => unreachable!(),
        })
    }

    pub fn new_c_root(tok: BasicToken, base: i128) -> Result<BasicToken, MathError> {
        Ok(match (tok, base) {
            (BasicToken::Integer(0), _) | (_, 0) => BasicToken::Integer(0),
            (val, 1) => val,
            (BasicToken::Fraction(val), -1) => BasicToken::Fraction(val.negate()?),
            (BasicToken::Integer(val), -1) => BasicToken::Integer(-val),
            (BasicToken::Fraction(val), _) => BasicToken::c_fraction_root(val, base),
            (BasicToken::Integer(val), _) => BasicToken::c_int_root(val, base),
            _ => unreachable!(),
        })
    }
}

impl fmt::Debug for BasicToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BasicToken::Integer(i) => write!(f, "{}", i),
            BasicToken::Fraction(fr) => write!(f, "{}", fr),
            BasicToken::SIntRoot(r) => write!(f, "{}", r),
            BasicToken::SFracRoot(r) => write!(f, "{}", r),
            BasicToken::CIntRoot(r) => write!(f, "{}", r),
            BasicToken::CFracRoot(r) => write!(f, "{}", r),
            BasicToken::Double(d) => write!(f, "Double: {}", d),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Token {
    Basic(BasicToken),
    Pi(BasicToken),
    Combined(Combined),
}

impl Token {
    #[inline]
    pub fn combined(basic: Vec<BasicToken>, pi: Vec<BasicToken>) -> Token {
        Token::Combined(Combined {
            basic: Set::new(basic),
            pi: Set::new(pi),
        })
    }
    pub fn double(&self) -> f64 {
        match self {
            Token::Basic(x) => x.double(),
            Token::Pi(x) => x.double() * std::f64::consts::PI,
            Token::Combined(i) => i.double(),
        }
    }
    pub fn negate(self) -> Result<Token, MathError> {
        Ok(match self {
            Token::Basic(x) => Token::Basic(x.negate()?),
            Token::Pi(x) => Token::Pi(x.negate()?),
            Token::Combined(mut x) => {
                for tok in x.basic.vec.iter_mut() {
                    *tok = tok.negate()?;
                }
                for tok in x.pi.vec.iter_mut() {
                    *tok = tok.negate()?;
                }
                Token::Combined(x)
            }
        })
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Basic(i) => write!(f, "{:?}", i),
            Token::Pi(i) => write!(f, "π*({:?})", i),
            Token::Combined(v) => {
                for (pos, tok) in v.basic.vec.iter().enumerate() {
                    if pos == 0 {
                        write!(f, "{:?}", tok)?;
                    } else {
                        write!(f, " + {:?}", tok)?;
                    }
                }
                for tok in v.pi.vec.iter() {
                    write!(f, " + π*({:?})", tok)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Eq, Copy, PartialEq, Clone)]
pub enum MathError {
    None,
    Combine,
    SyntaxError,
    Overflow,
    DoubleOverflow,
    DivisionByZero,
    ComplexNumber,
    // For 0^0
    ExponentiationError,
    TrigAccuracy,
    TangentError,
    DomainError,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::None => write!(f, "All good!"),
            MathError::Combine => write!(f, "Should combine"),
            MathError::SyntaxError => write!(f, "Incorrect syntax"),
            MathError::Overflow => write!(f, "Overflow"),
            MathError::DoubleOverflow => write!(f, "Proper overflow"),
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::ComplexNumber => write!(f, "Complex numbers not implemented"),
            MathError::ExponentiationError => write!(f, "Cannot compute 0^0"),
            MathError::TrigAccuracy => write!(f, "Cannot compute trig value accurately enough"),
            MathError::TangentError => write!(f, "Cannot compute the tangent of pi/2"),
            MathError::DomainError => {
                write!(f, "Function cannot compute values outside of it's domain.")
            }
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

    pub fn normalise(mut self) -> Result<BasicToken, MathError> {
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
            return Ok(BasicToken::Integer(self.int));
        }
        if (self.num > 0 && self.num >= self.den) || (self.num < 0 && -self.num >= self.den) {
            self.int = add!(self.int, self.num / self.den);
            self.num -= (self.num / self.den) * self.den;
        }
        if self.num == 0 {
            return Ok(BasicToken::Integer(self.int));
        }
        let gcd: i128 = ((abs!(self.num) as u128).gcd(abs!(self.den) as u128)) as i128;
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
        Ok(BasicToken::Fraction(self))
    }

    pub fn negate(mut self) -> Result<Fraction, MathError> {
        self.num = mul!(self.num, -1);
        self.int = mul!(self.int, -1);
        Ok(self)
    }
}

impl Add for Fraction {
    type Output = Result<BasicToken, MathError>;
    fn add(mut self, rhs: Fraction) -> Result<BasicToken, MathError> {
        self.int = add!(self.int, rhs.int);
        self.num = add!(mul!(self.num, rhs.den), mul!(self.den, rhs.num));
        self.den = mul!(self.den, rhs.den);
        self.normalise()
    }
}

impl Sub for Fraction {
    type Output = Result<BasicToken, MathError>;
    fn sub(mut self, rhs: Fraction) -> Result<BasicToken, MathError> {
        self.int = sub!(self.int, rhs.int);
        self.num = sub!(mul!(self.num, rhs.den), mul!(self.den, rhs.num));
        self.den = mul!(self.den, rhs.den);
        self.normalise()
    }
}

impl Mul for Fraction {
    type Output = Result<BasicToken, MathError>;
    fn mul(mut self, rhs: Fraction) -> Result<BasicToken, MathError> {
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
}

impl Div for Fraction {
    type Output = Result<BasicToken, MathError>;

    fn div(mut self, rhs: Fraction) -> Result<BasicToken, MathError> {
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

impl SRoot<Fraction> {
    pub fn normalise(self) -> Result<BasicToken, MathError> {
        match self.base {
            0 => Ok(BasicToken::Integer(0)),
            1 => self.mul.normalise(),
            _ => Ok(match self.mul.normalise()? {
                BasicToken::Integer(0) => BasicToken::Integer(0),
                BasicToken::Fraction(val) => BasicToken::SFracRoot(SRoot::new(val, self.base)),
                BasicToken::Integer(x) => BasicToken::s_int_root(x, self.base),
                _ => unreachable!(),
            }),
        }
    }
}

impl SRoot<i128> {
    pub fn normalise(self) -> BasicToken {
        match self {
            SRoot { mul: 0, base: _ } | SRoot { mul: _, base: 0 } => BasicToken::Integer(0),
            SRoot { mul: x, base: 1 } => BasicToken::Integer(x),
            _ => BasicToken::SIntRoot(self),
        }
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

impl CRoot<Fraction> {
    pub fn normalise(self) -> Result<BasicToken, MathError> {
        match self.base {
            0 => Ok(BasicToken::Integer(0)),
            1 => self.mul.normalise(),
            -1 => self.mul.negate()?.normalise(),
            _ => Ok(match self.mul.normalise()? {
                BasicToken::Integer(0) => BasicToken::Integer(0),
                BasicToken::Fraction(val) => BasicToken::CFracRoot(CRoot::new(val, self.base)),
                BasicToken::Integer(x) => BasicToken::c_int_root(x, self.base),
                _ => unreachable!(),
            }),
        }
    }
}

impl CRoot<i128> {
    pub fn normalise(self) -> Result<BasicToken, MathError> {
        Ok(match self {
            CRoot { mul: 0, base: _ } | CRoot { mul: _, base: 0 } => BasicToken::Integer(0),
            CRoot { mul: x, base: 1 } => BasicToken::Integer(x),
            CRoot { mul: x, base: -1 } => BasicToken::Integer(mul!(x, -1)),
            _ => BasicToken::CIntRoot(self),
        })
    }
}

#[derive(PartialEq, Clone)]
pub struct Combined {
    pub basic: Set<BasicToken>,
    pub pi: Set<BasicToken>,
}

impl Combined {
    pub fn normalise(self) -> Token {
        match (self.basic.vec.len(), self.pi.vec.len()) {
            (0, 0) => Token::Basic(BasicToken::Integer(0)),
            (0, 1) => Token::Pi(self.pi.vec[0]),
            (1, 0) => Token::Basic(self.basic.vec[0]),
            _ => Token::Combined(self),
        }
    }

    #[inline]
    pub fn double(&self) -> f64 {
        self.basic
            .vec
            .iter()
            .fold(0_f64, |acc, tok| acc + tok.double())
            + self
                .pi
                .vec
                .iter()
                .fold(0_f64, |acc, tok| acc + tok.double())
                * std::f64::consts::PI
    }

    pub fn add_combined(mut self, tok: Token) -> Result<Token, MathError> {
        match tok {
            Token::Basic(BasicToken::Integer(0)) | Token::Pi(BasicToken::Integer(0)) => {
                return Ok(Token::Combined(self));
            }
            Token::Basic(tok) => {
                self.basic.add(tok)?;
            }
            Token::Pi(tok) => {
                self.pi.add(tok)?;
            }
            Token::Combined(tokens) => {
                for tok in tokens.basic.vec {
                    self.basic.add(tok)?;
                }
                for tok in tokens.pi.vec {
                    self.pi.add(tok)?;
                }
            }
        }
        Ok(self.normalise())
    }

    pub fn mul_combined(mut self, tok: Token) -> Result<Token, MathError> {
        match tok {
            Token::Basic(BasicToken::Integer(0)) | Token::Pi(BasicToken::Integer(0)) => {
                Ok(Token::Basic(BasicToken::Integer(0)))
            }
            Token::Basic(tok) => {
                for comb_tok in self.basic.vec.iter_mut().chain(self.pi.vec.iter_mut()) {
                    match try_mul(*comb_tok, tok) {
                        Ok(val) => {
                            *comb_tok = val;
                        }
                        Err(MathError::Overflow) => {
                            return Ok(Token::Basic(BasicToken::Double(double_check!(
                                self.double() * tok.double()
                            ))));
                        }
                        Err(val) => {
                            return Err(val);
                        }
                    }
                }
                Ok(self.normalise())
            }
            // Transform all the basics into Pi
            Token::Pi(tok) if self.pi.vec.is_empty() => {
                for comb_tok in self.basic.vec.iter_mut() {
                    match try_mul(*comb_tok, tok) {
                        Ok(val) => {
                            *comb_tok = val;
                        }
                        Err(MathError::Overflow) => {
                            return Ok(Token::Basic(BasicToken::Double(double_check!(
                                self.double() * tok.double()
                            ))));
                        }
                        Err(val) => {
                            return Err(val);
                        }
                    }
                }
                Ok(self.normalise())
            }
            tok => Ok(Token::Basic(BasicToken::Double(double_check!(
                self.double() * tok.double()
            )))),
        }
    }

    pub fn negate(mut self) -> Result<Combined, MathError> {
        for tok in self.basic.vec.iter_mut() {
            *tok = tok.negate()?;
        }
        for tok in self.pi.vec.iter_mut() {
            *tok = tok.negate()?;
        }
        Ok(self)
    }
}

#[derive(PartialEq, Clone)]
pub struct Set<T> {
    pub vec: Vec<T>,
}

impl<T> Set<T> {
    #[inline]
    pub fn new(vec: Vec<T>) -> Set<T> {
        Set { vec }
    }
}
impl Set<BasicToken> {
    pub fn add(&mut self, tok: BasicToken) -> Result<(), MathError> {
        for (pos, vec_tok) in self.vec.iter().enumerate() {
            match try_add(*vec_tok, tok) {
                Err(MathError::Overflow) => {
                    let double = double_check!(self
                        .vec
                        .iter()
                        .fold(tok.double(), |acc, item| acc + item.double()));
                    self.vec.clear();
                    self.vec.push(BasicToken::Double(double));
                    break;
                }
                Err(MathError::Combine) => {
                    continue;
                }
                Ok(BasicToken::Integer(0)) => {
                    self.vec.swap_remove(pos);
                    break;
                }
                val => {
                    if discriminant(vec_tok) == discriminant(&val?) {
                        // If same enum variant.
                        self.vec.splice(pos..pos + 1, [val?]);
                        break;
                    } else {
                        return self.add(val?);
                    }
                }
            }
        }
        Ok(())
    }
}
