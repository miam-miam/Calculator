use crate::my_math::factorise;
use crate::types::{CRoot, Fraction, MathError, SRoot, Token};

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
            (Token::SFracRoot(..), _) => Ok(Com { l_num, r_num }),
            (_, Token::SFracRoot(..)) => Ok(Com {
                l_num: r_num,
                r_num: l_num,
            }),
            (Token::SIntRoot(..), _) => Ok(Com { l_num, r_num }),
            (_, Token::SIntRoot(..)) => Ok(Com {
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

pub fn exp(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    let try_exp = |tup: (Token, Token)| -> Result<Token, MathError> {
        // Check if 0^0.
        if let (Token::Integer(0), Token::Integer(0)) = tup {
            return Err(MathError::ExponentiationError);
        } else if let (Token::Double(a), Token::Double(b)) = tup {
            if !a.is_normal() && !b.is_normal() {
                return Err(MathError::ExponentiationError);
            }
        } else if let (Token::Integer(0), Token::Double(a))
        | (Token::Double(a), Token::Integer(0)) = tup
        {
            if !a.is_normal() {
                return Err(MathError::ExponentiationError);
            }
        }

        match tup {
            (Token::Integer(la), Token::Integer(ra)) => {
                if ra < 0 {
                    if -ra > u32::MAX as i128 {
                        return Err(MathError::Overflow);
                    }
                    return match la.checked_pow(-ra as u32) {
                        None => Err(MathError::Overflow),
                        Some(den) => Ok(Token::Fraction(Fraction::new(0, 1, den))),
                    };
                }
                if ra > u32::MAX as i128 {
                    return Err(MathError::Overflow);
                }
                match la.checked_pow(ra as u32) {
                    None => Err(MathError::Overflow),
                    Some(a) => Ok(Token::Integer(a)),
                }
            }
            (Token::Fraction(la), Token::Integer(ra)) => {
                if ra < 0 {
                    if -ra > u32::MAX as i128 {
                        return Err(MathError::Overflow);
                    }
                    return match (la.int * la.den + la.num).checked_pow(-ra as u32) {
                        None => Err(MathError::Overflow),
                        Some(den) => match la.den.checked_pow(-ra as u32) {
                            None => Err(MathError::Overflow),
                            Some(num) => {
                                let mut res = Fraction::new(0, num, den);
                                return match res.normalise() {
                                    Err(MathError::InvalidFraction) => Ok(Token::Integer(res.int)),
                                    Err(x) => Err(x),
                                    _ => Ok(Token::Fraction(res)),
                                };
                            }
                        },
                    };
                }
                if ra > u32::MAX as i128 {
                    return Err(MathError::Overflow);
                }
                match (la.int * la.den + la.num).checked_pow(ra as u32) {
                    None => Err(MathError::Overflow),
                    Some(num) => match la.den.checked_pow(ra as u32) {
                        None => Err(MathError::Overflow),
                        Some(den) => {
                            let mut res = Fraction::new(0, num, den);
                            return match res.normalise() {
                                Err(MathError::InvalidFraction) => Ok(Token::Integer(res.int)),
                                Err(x) => Err(x),
                                _ => Ok(Token::Fraction(res)),
                            };
                        }
                    },
                }
            }
            (Token::Integer(la), Token::Fraction(ra)) => {
                println!("{}, {}", la, ra);
                return if ra.den == 2 || ra.den == 3 {
                    if ra.num < 0 && ra.int < 0 {
                        if -ra.int > u32::MAX as i128 && -ra.num > u32::MAX as i128 {
                            return Err(MathError::Overflow);
                        }
                        match la.checked_pow(-ra.int as u32) {
                            None => Err(MathError::Overflow),
                            Some(outside_root) => match la.checked_pow(-ra.num as u32) {
                                None => Err(MathError::Overflow),
                                Some(inside_root) => {
                                    let mut res = Fraction::new(0, inside_root, 0); //TODO
                                    if ra.den == 3 {
                                        Ok(Token::CFracRoot(CRoot::new(
                                            Fraction::new(0, 1, outside_root),
                                            inside_root,
                                        )))
                                    } else {
                                        Ok(Token::SFracRoot(SRoot::new(
                                            Fraction::new(0, 1, outside_root),
                                            inside_root,
                                        )))
                                    }
                                }
                            },
                        }
                    } else {
                        let res = factorise(la, ra.den == 2);
                        if ra.int > u32::MAX as i128 && ra.num > u32::MAX as i128 {
                            return Err(MathError::Overflow);
                        }
                        match la.checked_pow(ra.int as u32) {
                            None => Err(MathError::Overflow),
                            Some(outside_root) => match res.inside.checked_pow(ra.num as u32) {
                                None => Err(MathError::Overflow),
                                Some(inside_root) => match res.outside.checked_pow(ra.num as u32) {
                                    None => Err(MathError::Overflow),
                                    Some(outside) => {
                                        if inside_root == 1 {
                                            return Ok(Token::Integer(mul!(outside_root, outside)));
                                        }
                                        if ra.den == 3 {
                                            Ok(Token::CIntRoot(CRoot::new(
                                                mul!(outside_root, outside),
                                                inside_root,
                                            )))
                                        } else {
                                            Ok(Token::SIntRoot(SRoot::new(
                                                mul!(outside_root, outside),
                                                inside_root,
                                            )))
                                        }
                                    }
                                },
                            },
                        }
                    }
                } else {
                    Err(MathError::Overflow)
                };
            }
            (Token::Double(la), ra) => Ok(Token::Double(double_check!(la.powf(double!(ra))))),
            (la, Token::Double(ra)) => Ok(Token::Double(double_check!((double!(la)).powf(ra)))),
            _ => Err(MathError::Impossible),
        }
    };
    match try_exp((l_number, r_number)) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            (double!(l_number)).powf(double!(r_number))
        ))),
        value => value,
    }
}
