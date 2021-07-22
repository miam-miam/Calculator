use crate::types::{Fraction, MathError, Token};

#[derive(Copy, Clone)]
pub struct Com {
    // Stands for Commutative
    pub l_num: Token,
    pub r_num: Token,
}

impl Com {
    pub fn new(l_num: Token, r_num: Token) -> Result<Com, MathError> {
        // Should go double, power, fraction and integer
        match (l_num, r_num) {
            (Token::Double(_), _) => Ok(Com { l_num, r_num }),
            (_, Token::Double(_)) => Ok(Com {
                l_num: r_num,
                r_num: l_num,
            }),
            (Token::Power(..), _) => Ok(Com { l_num, r_num }),
            (_, Token::Power(..)) => Ok(Com {
                l_num: r_num,
                r_num: l_num,
            }),
            (Token::Fraction(_), _) => Ok(Com { l_num, r_num }),
            (_, Token::Fraction(_)) => Ok(Com {
                l_num: r_num,
                r_num: l_num,
            }),
            (Token::Integer(_), _) => Ok(Com { l_num, r_num }),
            (_, Token::Integer(_)) => Ok(Com {
                l_num: r_num,
                r_num: l_num,
            }),
            (_, _) => Err(MathError::Impossible),
        }
    }
}

pub fn add(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    let try_add = |com: Com| -> Result<Token, MathError> {
        match com {
            Com {
                l_num: Token::Double(la),
                r_num: ra,
            } => Ok(Token::Double(double_check!(la + double!(ra)))),
            // Com {
            //     l_num: Token::Power(la),
            //     r_num: ra,
            // } => Ok(Token::Double(double_check!(la + double!(ra)))),
            Com {
                l_num: Token::Fraction(mut la),
                r_num: Token::Fraction(ra),
            } => {
                return match la.add(&ra) {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(la)),
                }
            }
            Com {
                l_num: Token::Fraction(mut la),
                r_num: Token::Integer(ra),
            } => {
                la.int = add!(la.int, ra);
                Ok(Token::Fraction(la))
            }
            Com {
                l_num: Token::Integer(la),
                r_num: Token::Integer(ra),
            } => Ok(Token::Integer(add!(la, ra))),
            _ => Err(MathError::Impossible),
        }
    };
    let commutative = Com::new(l_number, r_number)?;
    match try_add(commutative) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            double!(commutative.l_num) + double!(commutative.r_num)
        ))),
        value => value,
    }
}

pub fn sub(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    let try_sub = |tup: (Token, Token)| -> Result<Token, MathError> {
        match tup {
            (Token::Integer(la), Token::Integer(ra)) => Ok(Token::Integer(sub!(la, ra))),
            (Token::Fraction(mut la), Token::Integer(ra)) => {
                la.int = sub!(la.int, ra);
                Ok(Token::Fraction(la))
            }
            (Token::Integer(la), Token::Fraction(mut ra)) => {
                ra.int = sub!(la, ra.int);
                ra.num = mul!(ra.num, -1);
                Ok(Token::Fraction(ra))
            }
            (Token::Fraction(mut la), Token::Fraction(ra)) => {
                return match la.sub(&ra) {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(la)),
                }
            }
            (Token::Double(la), ra) => Ok(Token::Double(double_check!(la - double!(ra)))),
            (la, Token::Double(ra)) => Ok(Token::Double(double_check!(double!(la) - ra))),
            _ => Err(MathError::Impossible),
        }
    };
    match try_sub((l_number, r_number)) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            double!(l_number) - double!(r_number)
        ))),
        value => value,
    }
}

pub fn mul(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    let try_mul = |com: Com| -> Result<Token, MathError> {
        match com {
            Com {
                l_num: Token::Double(la),
                r_num: ra,
            } => Ok(Token::Double(double_check!(la * double!(ra)))),
            Com {
                l_num: Token::Fraction(mut la),
                r_num: Token::Fraction(ra),
            } => {
                return match la.mul(&ra) {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(la)),
                }
            }
            Com {
                l_num: Token::Fraction(mut la),
                r_num: Token::Integer(ra),
            } => {
                la.int = mul!(la.int, ra);
                la.num = mul!(la.num, ra);
                return match la.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(la)),
                };
            }
            Com {
                l_num: Token::Integer(la),
                r_num: Token::Integer(ra),
            } => Ok(Token::Integer(mul!(la, ra))),
            _ => Err(MathError::Impossible),
        }
    };
    let commutative = Com::new(l_number, r_number)?;
    match try_mul(commutative) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            double!(commutative.l_num) * double!(commutative.r_num)
        ))),
        value => value,
    }
}

pub fn div(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    let try_div = |tup: (Token, Token)| -> Result<Token, MathError> {
        // Check if zero.
        if let (_, Token::Integer(0)) = tup {
            return Err(MathError::DivisionByZero);
        } else if let (_, Token::Double(a)) = tup {
            if !a.is_normal() {
                return Err(MathError::DivisionByZero);
            }
        }
        match tup {
            (Token::Integer(la), Token::Integer(ra)) => {
                let mut res = Fraction {
                    int: 0,
                    num: la,
                    den: ra,
                };
                return match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(res.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(res)),
                };
            }
            (Token::Fraction(mut la), Token::Integer(ra)) => {
                la.num = add!(la.num, mul!(la.int, la.den));
                la.den = mul!(la.den, ra);
                la.int = 0;
                return match la.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(la)),
                };
            }
            (Token::Integer(la), Token::Fraction(mut ra)) => {
                let old_num = ra.num;
                ra.num = mul!(la, ra.den);
                ra.den = add!(old_num, mul!(ra.den, ra.int));
                ra.int = 0;
                return match ra.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(ra.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(ra)),
                };
            }
            (Token::Fraction(mut la), Token::Fraction(ra)) => {
                return match la.div(&ra) {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(la)),
                }
            }
            (Token::Double(la), ra) => Ok(Token::Double(double_check!(la / double!(ra)))),
            (la, Token::Double(ra)) => Ok(Token::Double(double_check!(double!(la) / ra))),
            _ => Err(MathError::Impossible),
        }
    };
    match try_div((l_number, r_number)) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            double!(l_number) / double!(r_number)
        ))),
        value => value,
    }
}
