use crate::my_math;
use crate::number;

use itertools::Itertools;

pub struct Expression<'a> {
    string: &'a str,
    pub infix_token: Vec<my_math::Token>,
    postfix_token: Vec<my_math::Token>,
    result: my_math::Token,
}

impl<'a> Expression<'a> {
    pub fn new(string: &str) -> Expression {
        Expression {
            string,
            infix_token: vec![],
            postfix_token: vec![],
            result: my_math::Token::None,
        }
    }
    pub fn tokenise(&mut self) -> my_math::MathError {
        let string_char_len = self.string.chars().count();
        let mut decimal_point_index = None;
        let mut checking_number = false;
        let mut new_string: String = String::new();
        for (idx, (elem, next)) in self.string.chars().chain([' ']).tuple_windows().enumerate() {
            if !checking_number && '0' <= elem && elem <= '9' {
                checking_number = true;
                decimal_point_index = None;
                new_string = String::new();
            } // Not using else if as checking number can change inside previous if statement.
            if checking_number {
                if elem == '.' {
                    new_string.push('.');
                    match decimal_point_index {
                        None => {
                            decimal_point_index = Some(new_string.len() - 1);
                        }
                        Some(_) => {
                            return my_math::MathError::InvalidDecimalPoint;
                        }
                    }
                } else if '0' <= elem && elem <= '9' {
                    new_string.push(elem);
                }
                if !(('0' <= next && next <= '9') || next == ' ' || next == '.')
                    || idx == string_char_len - 1
                {
                    checking_number = false;
                    match decimal_point_index {
                        None => match new_string.parse::<i128>() {
                            Ok(result) => {
                                self.infix_token.push(my_math::Token::Integer(result));
                            }
                            Err(_) => match match_string_to_float(&new_string) {
                                Some(result) => {
                                    self.infix_token.push(my_math::Token::Double(result));
                                }
                                None => {
                                    return my_math::MathError::Overflow;
                                }
                            },
                        },
                        Some(index) => {
                            let integer: i128;
                            let decimal: i128;
                            match (&new_string[0..index]).parse::<i128>() {
                                Ok(result) => {
                                    integer = result;
                                }
                                Err(_) => match match_string_to_float(&new_string) {
                                    Some(x) => {
                                        self.infix_token.push(my_math::Token::Double(x));
                                        continue;
                                    }
                                    None => {
                                        return my_math::MathError::Overflow;
                                    }
                                },
                            }

                            match (&new_string[index + 1..]).parse::<i128>() {
                                Ok(result) => {
                                    decimal = result;
                                }
                                Err(_) => match match_string_to_float(&new_string) {
                                    Some(x) => {
                                        self.infix_token.push(my_math::Token::Double(x));
                                        continue;
                                    }
                                    None => {
                                        return my_math::MathError::Overflow;
                                    }
                                },
                            }
                            if decimal == 0 {
                                self.infix_token.push(my_math::Token::Integer(integer));
                            } else {
                                match my_math::ten_to_the_power_of(
                                    (new_string.len() - index - 1) as i128,
                                ) {
                                    None => match match_string_to_float(&new_string) {
                                        Some(x) => {
                                            self.infix_token.push(my_math::Token::Double(x));
                                            continue;
                                        }
                                        None => {
                                            return my_math::MathError::Overflow;
                                        }
                                    },
                                    Some(result) => {
                                        let mut fraction = number::Fraction {
                                            int: integer,
                                            num: decimal,
                                            den: result,
                                        };
                                        self.infix_token.push(match fraction.normalise() {
                                            my_math::MathError::InvalidFraction => {
                                                my_math::Token::Integer(fraction.int)
                                            }
                                            my_math::MathError::Overflow => {
                                                match match_string_to_float(&new_string) {
                                                    Some(x) => my_math::Token::Double(x),
                                                    None => {
                                                        return my_math::MathError::Overflow;
                                                    }
                                                }
                                            }
                                            _ => my_math::Token::Fraction(fraction),
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
            } else if elem != ' ' {
                self.infix_token.push(match elem {
                    '+' => my_math::Token::Plus,
                    '-' => my_math::Token::Minus,
                    '*' => my_math::Token::Multiply,
                    '/' => my_math::Token::Divide,
                    '(' => my_math::Token::LBracket,
                    ')' => my_math::Token::RBracket,
                    _ => {
                        println!("{}", elem);
                        return my_math::MathError::UnknownOperator;
                    }
                });
            }
        }
        my_math::MathError::None
    }
}

fn match_string_to_float(string: &str) -> Option<f64> {
    match string.parse::<f64>() {
        Ok(i) => {
            if i == f64::INFINITY || i == f64::NEG_INFINITY || i == 0.0_f64 {
                return None;
            }
            Some(i)
        }
        Err(_) => None,
    }
}
