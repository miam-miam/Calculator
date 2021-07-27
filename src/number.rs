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
            (Token::Integer(la), Token::Integer(mut ra)) => {
                let negative = ra < 0;
                ra = ra.abs();
                if ra > u32::MAX as i128 {
                    return Err(MathError::Overflow);
                }
                let a = none_to_err!(la.checked_pow(ra as u32));
                match negative {
                    false => Ok(Token::Integer(a)),
                    true => Ok(Token::Fraction(Fraction::new(0, 1, a))),
                }
            }
            (Token::Fraction(la), Token::Integer(mut ra)) => {
                let negative = ra < 0;
                ra = ra.abs();
                if ra > u32::MAX as i128 {
                    return Err(MathError::Overflow);
                }
                let num = none_to_err!((la.int * la.den + la.num).checked_pow(ra as u32));
                let den = none_to_err!(la.den.checked_pow(ra as u32));
                let mut res = match negative {
                    false => Fraction::new(0, num, den),
                    true => Fraction::new(0, den, num),
                };
                return match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(res.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(res)),
                };
            }
            (Token::Integer(la), Token::Fraction(mut ra)) => {
                if ra.den != 2 && ra.den != 3 {
                    return Err(MathError::Overflow);
                }
                let negative = ra.num < 0 || ra.int < 0;
                ra.num = ra.num.abs();
                ra.int = ra.int.abs();
                if ra.int > u32::MAX as i128 && ra.num > u32::MAX as i128 {
                    return Err(MathError::Overflow);
                }
                let res = factorise(la, ra.den == 2);
                let outside_root = none_to_err!(la.checked_pow(ra.int as u32));
                let inside_root = none_to_err!(res.inside.checked_pow(ra.num as u32));
                let outside = none_to_err!(res.outside.checked_pow(ra.num as u32));
                if inside_root == 1 {
                    return match negative {
                        true => Ok(Token::Fraction(Fraction::new(
                            0,
                            1,
                            mul!(outside_root, outside),
                        ))),
                        false => Ok(Token::Integer(mul!(outside_root, outside))),
                    };
                }
                if ra.den == 3 {
                    match negative {
                        false => Ok(Token::CIntRoot(CRoot::new(
                            mul!(outside_root, outside),
                            inside_root,
                        ))),
                        true => match inside_root.checked_pow(2) {
                            None => Err(MathError::Overflow),
                            Some(sq_inside_root) => Ok(Token::CFracRoot(CRoot::new(
                                Fraction::new(0, 1, mul!(mul!(outside_root, outside), inside_root)),
                                sq_inside_root,
                            ))),
                        },
                    }
                } else {
                    match negative {
                        false => Ok(Token::SIntRoot(SRoot::new(
                            mul!(outside_root, outside),
                            inside_root,
                        ))),
                        true => Ok(Token::SFracRoot(SRoot::new(
                            Fraction::new(0, 1, mul!(mul!(outside_root, outside), inside_root)),
                            inside_root,
                        ))),
                    }
                }
            }
            (Token::Fraction(la), Token::Fraction(mut ra)) => {
                if ra.den != 2 && ra.den != 3 {
                    return Err(MathError::Overflow);
                }
                let negative = ra.num < 0 || ra.int < 0;
                ra.num = ra.num.abs();
                ra.int = ra.int.abs();
                if ra.int > u32::MAX as i128 && ra.num > u32::MAX as i128 {
                    return Err(MathError::Overflow);
                }

                let num = add!(mul!(la.int, la.den), la.num);

                let mut res_num = factorise(num, ra.den == 2);
                let mut res_den = factorise(la.den, ra.den == 2);
                let mut outside_root_num = none_to_err!(num.checked_pow(ra.int as u32));
                let mut inside_root_num = none_to_err!(res_num.inside.checked_pow(ra.num as u32));
                let mut outside_num = none_to_err!(res_num.outside.checked_pow(ra.num as u32));
                let mut outside_root_den = none_to_err!(la.den.checked_pow(ra.int as u32));
                let mut inside_root_den = none_to_err!(res_den.inside.checked_pow(ra.num as u32));
                let mut outside_den = none_to_err!(res_den.outside.checked_pow(ra.num as u32));

                // Swap den to num and num to den.
                if negative {
                    std::mem::swap(&mut res_num, &mut res_den);
                    std::mem::swap(&mut outside_root_num, &mut outside_root_den);
                    std::mem::swap(&mut inside_root_num, &mut inside_root_den);
                    std::mem::swap(&mut outside_num, &mut outside_den);
                }

                if inside_root_num == 1 && inside_root_den == 1 {
                    let mut res = Fraction::new(
                        0,
                        mul!(outside_root_num, outside_num),
                        mul!(outside_root_den, outside_den),
                    );
                    return match res.normalise() {
                        Err(MathError::InvalidFraction) => Ok(Token::Integer(res.int)),
                        Err(x) => Err(x),
                        _ => Ok(Token::Fraction(res)),
                    };
                }
                return if ra.den == 3 {
                    let mut res = Fraction::new(
                        0,
                        mul!(outside_root_num, outside_num),
                        mul!(mul!(outside_root_den, outside_den), inside_root_den),
                    );
                    match res.normalise() {
                        Err(MathError::InvalidFraction) => Ok(Token::CIntRoot(CRoot::new(
                            res.int,
                            mul!(
                                inside_root_num,
                                none_to_err!(inside_root_den.checked_pow(2))
                            ),
                        ))),
                        Err(x) => Err(x),
                        _ => Ok(Token::CFracRoot(CRoot::new(
                            res,
                            mul!(
                                inside_root_num,
                                none_to_err!(inside_root_den.checked_pow(2))
                            ),
                        ))),
                    }
                } else {
                    let mut res = Fraction::new(
                        0,
                        mul!(outside_root_num, outside_num),
                        mul!(mul!(outside_root_den, outside_den), inside_root_den),
                    );
                    match res.normalise() {
                        Err(MathError::InvalidFraction) => Ok(Token::SIntRoot(SRoot::new(
                            res.int,
                            mul!(inside_root_num, inside_root_den),
                        ))),
                        Err(x) => Err(x),
                        _ => Ok(Token::SFracRoot(SRoot::new(
                            res,
                            mul!(inside_root_num, inside_root_den),
                        ))),
                    }
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
