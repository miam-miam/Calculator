use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
pub use pest::Parser;

use crate::my_math::ten_to_the_power_of;
use crate::number::{add, div, exp, mul, sub};
use crate::types::{Fraction, MathError, Token};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Expression;

lazy_static! {
    pub static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right),
        ])
    };
}

pub fn eval(expression: Pairs<Rule>) -> Result<Token, MathError> {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| {
            match pair.as_rule() {
                Rule::expr => eval(pair.into_inner()),
                Rule::int => {
                    // TODO do exp
                    let int_str = pair.as_str();
                    let mut pairs = pair.into_inner();
                    let (integer, _exponent) = (pairs.next().unwrap().as_str(), pairs.next());
                    match integer.parse::<i128>() {
                        Ok(result) => Ok(Token::Integer(result)),
                        Err(_) => match match_string_to_float(int_str) {
                            Some(result) => Ok(Token::Double(result)),
                            None => Err(MathError::DoubleOverflow),
                        },
                    }
                }
                Rule::dec => {
                    let pair_str = pair.as_str();
                    let mut pairs = pair.into_inner();
                    let integer: &str;
                    let decimal: &str;
                    let pair = pairs.next().unwrap();
                    match pair.as_rule() {
                        Rule::basic_int => {
                            integer = pair.as_str();
                            decimal = pairs.next().unwrap().as_str();
                        }
                        Rule::basic_dec => {
                            // Integer may be empty as a decimal can be written like this: .5
                            decimal = pair.as_str();
                            integer = "0"
                        }
                        _ => unreachable!(),
                    }
                    let exponent = pairs.next();

                    match exponent {
                        Some(_) => {
                            // TODO make it return fraction/integer if it can.
                            match match_string_to_float(pair_str) {
                                Some(x) => Ok(Token::Double(x)),
                                None => Err(MathError::DoubleOverflow),
                            }
                        }
                        None => match integer.parse::<i128>() {
                            Ok(integer) => match decimal.parse::<i128>() {
                                Ok(decimal_int) => match ten_to_the_power_of(decimal.len() as i128)
                                {
                                    None => match match_string_to_float(pair_str) {
                                        Some(x) => Ok(Token::Double(x)),
                                        None => Err(MathError::DoubleOverflow),
                                    },
                                    Some(result) => {
                                        let mut fraction = Fraction {
                                            int: integer,
                                            num: decimal_int,
                                            den: result,
                                        };
                                        match fraction.normalise() {
                                            Err(MathError::InvalidFraction) => {
                                                Ok(Token::Integer(fraction.int))
                                            }
                                            Err(MathError::Overflow) => {
                                                match match_string_to_float(pair_str) {
                                                    Some(x) => Ok(Token::Double(x)),
                                                    None => Err(MathError::DoubleOverflow),
                                                }
                                            }
                                            _ => Ok(Token::Fraction(fraction)),
                                        }
                                    }
                                },
                                Err(_) => match match_string_to_float(pair_str) {
                                    Some(x) => Ok(Token::Double(x)),
                                    None => Err(MathError::DoubleOverflow),
                                },
                            },
                            Err(_) => match match_string_to_float(pair_str) {
                                Some(x) => Ok(Token::Double(x)),
                                None => Err(MathError::DoubleOverflow),
                            },
                        },
                    }
                }
                _ => unreachable!(),
            }
        },
        |lhs: Result<Token, MathError>, op: Pair<Rule>, rhs: Result<Token, MathError>| match op
            .as_rule()
        {
            Rule::add => add(lhs?, rhs?),
            Rule::subtract => sub(lhs?, rhs?),
            Rule::multiply => mul(lhs?, rhs?),
            Rule::divide => div(lhs?, rhs?),
            Rule::power => exp(lhs?, rhs?),
            _ => unreachable!(),
        },
    )
}

fn match_string_to_float(string: &str) -> Option<f64> {
    match string.parse::<f64>() {
        Ok(i) => {
            if i == f64::INFINITY || i == f64::NEG_INFINITY {
                return None;
            }
            Some(i)
        }
        Err(_) => None,
    }
}
