use crate::types::{Fraction, MathError, Token};

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
    let try_add = |com: &Com| -> Result<Token, MathError> {
        match *com {
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
                la.add(&ra)?;
                Ok(Token::Fraction(la))
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
    match try_add(&commutative) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            double!(commutative.l_num) + double!(commutative.r_num)
        ))),
        value => value,
    }
}
