use crate::number;
use core::fmt;

const MAX_I128_LOG_2: i128 = 127;
const MAX_I128_LOG_10: i128 = 38;

#[derive(PartialEq)]
pub enum Token {
    Integer(i128),
    Fraction(number::Fraction),
    Power(
        number::Fraction,
        number::SimpleFraction,
        number::SimpleFraction,
    ),
    Double(f64),
    None,
    Plus,
    Minus,
    Multiply,
    Divide,
    LBracket,
    RBracket,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(i) => write!(f, "Integer: {}", i),
            Token::Fraction(fr) => write!(f, "Fraction: {}", fr),
            Token::Power(m, b, e) => write!(f, "Power: {}*({})^({})", m, b, e),
            Token::Double(d) => write!(f, "Double: {}", d),
            Token::None => write!(f, "None"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Multiply => write!(f, "Multiply"),
            Token::Divide => write!(f, "Divide"),
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
    DivisionByZero,
    ComplexNumber,
    // For 0^0
    PowerError,
    InvalidDecimalPoint,
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
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::ComplexNumber => write!(f, "Complex numbers not implemented"),
            MathError::PowerError => write!(f, "Cannot compute 0^0"),
            MathError::InvalidDecimalPoint => write!(f, "Invalid decimal point"),
            MathError::InvalidFraction => write!(f, "Fraction should be integer"),
            MathError::Impossible => write!(f, "Not possible"),
        }
    }
}

pub fn ten_to_the_power_of(exponent: i128) -> Option<i128> {
    if exponent > MAX_I128_LOG_10 {
        return None;
    }
    let mut count: i128 = 1;
    for _ in 0..exponent {
        count *= 10;
    }
    return Some(count);
}
