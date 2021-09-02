use crate::my_math::factorise;
use crate::types::{CRoot, Fraction, MathError, SRoot, Token};

fn try_add(tup: (&Token, &Token)) -> Result<Token, MathError> {
    match tup {
        (Token::Fraction(mut la), Token::Fraction(ra)) => match la.add(&ra) {
            Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
            Err(x) => Err(x),
            _ => Ok(Token::Fraction(la)),
        },
        (Token::Fraction(mut la), Token::Integer(ra))
        | (Token::Integer(ra), Token::Fraction(mut la)) => {
            la.int = add!(la.int, *ra);
            Ok(Token::Fraction(la))
        }
        (Token::Integer(la), Token::Integer(ra)) => Ok(Token::Integer(add!(la, *ra))),
        (Token::SIntRoot(mut la), Token::SIntRoot(ra)) if la.base == ra.base => {
            la.mul = add!(la.mul, ra.mul);
            Ok(Token::SIntRoot(la))
        }
        (Token::SFracRoot(mut la), Token::SIntRoot(ra))
        | (Token::SIntRoot(ra), Token::SFracRoot(mut la))
            if la.base == ra.base =>
        {
            la.mul.int = add!(la.mul.int, ra.mul);
            Ok(Token::SFracRoot(la))
        }
        (Token::SFracRoot(mut la), Token::SFracRoot(ra)) if la.base == ra.base => {
            match la.mul.add(&ra.mul) {
                Err(MathError::InvalidFraction) => Ok(Token::Integer(la.mul.int)),
                Err(x) => Err(x),
                _ => Ok(Token::SFracRoot(la)),
            }
        }
        (Token::CIntRoot(mut la), Token::CIntRoot(ra)) if la.base == ra.base => {
            la.mul = add!(la.mul, ra.mul);
            Ok(Token::CIntRoot(la))
        }
        (Token::CFracRoot(mut la), Token::CIntRoot(ra))
        | (Token::CIntRoot(ra), Token::CFracRoot(mut la))
            if la.base == ra.base =>
        {
            la.mul.int = add!(la.mul.int, ra.mul);
            Ok(Token::CFracRoot(la))
        }
        (Token::CFracRoot(mut la), Token::CFracRoot(ra)) if la.base == ra.base => {
            match la.mul.add(&ra.mul) {
                Err(MathError::InvalidFraction) => Ok(Token::Integer(la.mul.int)),
                Err(x) => Err(x),
                _ => Ok(Token::CFracRoot(la)),
            }
        }
        _ => Err(MathError::Overflow),
    }
}

pub fn add(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    if l_number.is_pi() && r_number.is_pi() {
        match try_add((&l_number.normal(), &r_number.normal())) {
            Err(MathError::Overflow) => Ok(Token::Double(double_check!(
                (l_number.double() + r_number.double()) * std::f64::consts::PI
            ))),
            value => value?.pi(),
        }
    } else {
        match try_add((&l_number, &r_number)) {
            Err(MathError::Overflow) => Ok(Token::Double(double_check!(
                l_number.double() + r_number.double()
            ))),
            value => value,
        }
    }
}

fn try_sub(tup: (&Token, &Token)) -> Result<Token, MathError> {
    match tup {
        (Token::Integer(la), Token::Integer(ra)) => Ok(Token::Integer(sub!(la, *ra))),
        (Token::Fraction(mut la), Token::Integer(ra)) => {
            la.int = sub!(la.int, *ra);
            Ok(Token::Fraction(la))
        }
        (Token::Integer(la), Token::Fraction(mut ra)) => {
            ra.int = sub!(la, ra.int);
            ra.num = mul!(ra.num, -1);
            Ok(Token::Fraction(ra))
        }
        (Token::Fraction(mut la), Token::Fraction(ra)) => match la.sub(&ra) {
            Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
            Err(x) => Err(x),
            _ => Ok(Token::Fraction(la)),
        },
        (Token::SIntRoot(mut la), Token::SIntRoot(ra)) if la.base == ra.base => {
            la.mul = sub!(la.mul, ra.mul);
            Ok(Token::SIntRoot(la))
        }
        (Token::SFracRoot(mut la), Token::SIntRoot(ra)) if la.base == ra.base => {
            la.mul.int = sub!(la.mul.int, ra.mul);
            Ok(Token::SFracRoot(la))
        }
        (Token::SIntRoot(la), Token::SFracRoot(mut ra)) if la.base == ra.base => {
            ra.mul.int = sub!(la.mul, ra.mul.int);
            ra.mul.num = mul!(ra.mul.num, -1);
            Ok(Token::SFracRoot(ra))
        }
        (Token::SFracRoot(mut la), Token::SFracRoot(ra)) if la.base == ra.base => {
            match la.mul.sub(&ra.mul) {
                Err(MathError::InvalidFraction) => Ok(Token::Integer(la.mul.int)),
                Err(x) => Err(x),
                _ => Ok(Token::SFracRoot(la)),
            }
        }
        (Token::CIntRoot(mut la), Token::CIntRoot(ra)) if la.base == ra.base => {
            la.mul = sub!(la.mul, ra.mul);
            Ok(Token::CIntRoot(la))
        }
        (Token::CFracRoot(mut la), Token::CIntRoot(ra)) if la.base == ra.base => {
            la.mul.int = sub!(la.mul.int, ra.mul);
            Ok(Token::CFracRoot(la))
        }
        (Token::CIntRoot(la), Token::CFracRoot(mut ra)) if la.base == ra.base => {
            ra.mul.int = sub!(la.mul, ra.mul.int);
            ra.mul.num = mul!(ra.mul.num, -1);
            Ok(Token::CFracRoot(ra))
        }
        (Token::CFracRoot(mut la), Token::CFracRoot(ra)) if la.base == ra.base => {
            match la.mul.sub(&ra.mul) {
                Err(MathError::InvalidFraction) => Ok(Token::Integer(la.mul.int)),
                Err(x) => Err(x),
                _ => Ok(Token::CFracRoot(la)),
            }
        }
        _ => Err(MathError::Overflow),
    }
}

pub fn sub(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    if l_number.is_pi() && r_number.is_pi() {
        match try_sub((&l_number.normal(), &r_number.normal())) {
            Err(MathError::Overflow) => Ok(Token::Double(double_check!(
                (l_number.double() - r_number.double()) * std::f64::consts::PI
            ))),
            value => value?.pi(),
        }
    } else {
        match try_sub((&l_number, &r_number)) {
            Err(MathError::Overflow) => Ok(Token::Double(double_check!(
                l_number.double() - r_number.double()
            ))),
            value => value,
        }
    }
}

fn try_mul(tup: (&Token, &Token)) -> Result<Token, MathError> {
    match tup {
        // a*0 = 0
        (Token::Integer(0), _) | (_, Token::Integer(0)) => Ok(Token::Integer(0)),

        (Token::Fraction(mut la), Token::Fraction(ra)) => match la.mul(&ra) {
            Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
            Err(x) => Err(x),
            _ => Ok(Token::Fraction(la)),
        },
        (Token::Fraction(mut la), Token::Integer(ra))
        | (Token::Integer(ra), Token::Fraction(mut la)) => {
            la.int = mul!(la.int, *ra);
            la.num = mul!(la.num, *ra);
            match la.normalise() {
                Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
                Err(x) => Err(x),
                _ => Ok(Token::Fraction(la)),
            }
        }
        (Token::Integer(la), Token::Integer(ra)) => Ok(Token::Integer(mul!(la, *ra))),
        (Token::SIntRoot(mut la), Token::SIntRoot(ra)) => {
            la.mul = mul!(la.mul, ra.mul);
            let res = factorise(mul!(la.base, ra.base), true);
            la.mul = mul!(la.mul, res.outside);
            match res.inside {
                1 => Ok(Token::Integer(la.mul)),
                _ => {
                    la.base = res.inside;
                    Ok(Token::SIntRoot(la))
                }
            }
        }
        (Token::SFracRoot(mut la), Token::SIntRoot(ra))
        | (Token::SIntRoot(ra), Token::SFracRoot(mut la)) => {
            la.mul.int = mul!(la.mul.int, ra.mul);
            la.mul.num = mul!(la.mul.num, ra.mul);
            let res = factorise(mul!(la.base, ra.base), true);
            la.mul.int = mul!(la.mul.int, res.outside);
            la.mul.num = mul!(la.mul.num, res.outside);
            match res.inside {
                1 => match la.mul.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(la.mul.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(la.mul)),
                },
                _ => match la.mul.normalise() {
                    Err(MathError::InvalidFraction) => {
                        Ok(Token::s_int_root(la.mul.int, res.inside))
                    }
                    Err(x) => Err(x),
                    _ => Ok(Token::s_fraction_root(la.mul, res.inside)),
                },
            }
        }
        (Token::SFracRoot(mut la), Token::SFracRoot(ra)) => match la.mul.mul(&ra.mul) {
            Err(MathError::InvalidFraction) => {
                let res = factorise(mul!(la.base, ra.base), true);
                la.mul.int = mul!(la.mul.int, res.outside);
                match res.inside {
                    1 => Ok(Token::Integer(la.mul.int)),
                    _ => Ok(Token::s_int_root(la.mul.int, res.inside)),
                }
            }
            Err(x) => Err(x),
            _ => Ok(Token::SFracRoot(la)),
        },
        (Token::CIntRoot(mut la), Token::CIntRoot(ra)) => {
            la.mul = mul!(la.mul, ra.mul);
            let res = factorise(mul!(la.base, ra.base), false);
            la.mul = mul!(la.mul, res.outside);
            match res.inside {
                1 => Ok(Token::Integer(la.mul)),
                _ => {
                    la.base = res.inside;
                    Ok(Token::CIntRoot(la))
                }
            }
        }
        (Token::CFracRoot(mut la), Token::CIntRoot(ra))
        | (Token::CIntRoot(ra), Token::CFracRoot(mut la)) => {
            la.mul.int = mul!(la.mul.int, ra.mul);
            la.mul.num = mul!(la.mul.num, ra.mul);
            let res = factorise(mul!(la.base, ra.base), false);
            la.mul.int = mul!(la.mul.int, res.outside);
            la.mul.num = mul!(la.mul.num, res.outside);
            match res.inside {
                1 => match la.mul.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(la.mul.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(la.mul)),
                },
                _ => match la.mul.normalise() {
                    Err(MathError::InvalidFraction) => {
                        Ok(Token::c_int_root(la.mul.int, res.inside))
                    }
                    Err(x) => Err(x),
                    _ => Ok(Token::c_fraction_root(la.mul, res.inside)),
                },
            }
        }
        (Token::CFracRoot(mut la), Token::CFracRoot(ra)) => {
            let res = factorise(mul!(la.base, ra.base), false);
            la.mul.int = mul!(la.mul.int, res.outside);
            match la.mul.mul(&ra.mul) {
                Err(MathError::InvalidFraction) => match res.inside {
                    1 => Ok(Token::Integer(la.mul.int)),
                    _ => Ok(Token::c_int_root(la.mul.int, res.inside)),
                },
                Err(x) => Err(x),
                _ => match res.inside {
                    1 => Ok(Token::Fraction(la.mul)),
                    _ => Ok(Token::c_fraction_root(la.mul, res.inside)),
                },
            }
        }
        _ => Err(MathError::Overflow),
    }
}

pub fn mul(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    match (l_number.is_pi(), r_number.is_pi()) {
        (true, true) => match (&l_number, &r_number) {
            (Token::Integer(0), _) | (_, Token::Integer(0)) => Ok(Token::Integer(0)),
            _ => Ok(Token::Double(double_check!(
                l_number.double() * r_number.double()
            ))),
        },
        (true, false) => match try_mul((&l_number.normal(), &r_number)) {
            Err(MathError::Overflow) => Ok(Token::Double(double_check!(
                l_number.double() * r_number.double()
            ))),
            value => value?.pi(),
        },
        (false, true) => match try_mul((&l_number, &r_number.normal())) {
            Err(MathError::Overflow) => Ok(Token::Double(double_check!(
                l_number.double() * r_number.double()
            ))),
            value => value?.pi(),
        },
        (false, false) => match try_mul((&l_number, &r_number)) {
            Err(MathError::Overflow) => Ok(Token::Double(double_check!(
                l_number.double() * r_number.double()
            ))),
            value => value,
        },
    }
}

fn try_div(tup: (&Token, &Token)) -> Result<Token, MathError> {
    {
        match tup {
            // Check if zero.
            (_, Token::Integer(0)) => Err(MathError::DivisionByZero),
            (_, Token::Double(a)) if !a.is_normal() => Err(MathError::DivisionByZero),

            // 0/a = 0
            (Token::Integer(0), _) => Ok(Token::Integer(0)),

            (Token::Integer(la), Token::Integer(ra)) => {
                let mut res = Fraction {
                    int: 0,
                    num: *la,
                    den: *ra,
                };
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(res.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(res)),
                }
            }
            (Token::Fraction(mut la), Token::Integer(ra)) => {
                la.num = add!(la.num, mul!(la.int, la.den));
                la.den = mul!(la.den, *ra);
                la.int = 0;
                match la.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(la)),
                }
            }
            (Token::Integer(la), Token::Fraction(mut ra)) => {
                let old_num = ra.num;
                ra.num = mul!(la, ra.den);
                ra.den = add!(old_num, mul!(ra.den, ra.int));
                ra.int = 0;
                match ra.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(ra.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(ra)),
                }
            }
            (Token::Fraction(mut la), Token::Fraction(ra)) => match la.div(&ra) {
                Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
                Err(x) => Err(x),
                _ => Ok(Token::Fraction(la)),
            },
            (Token::SIntRoot(la), Token::SIntRoot(ra)) => {
                let mut frac = Fraction::new(0, la.mul, mul!(ra.mul, ra.base));
                let res = factorise(mul!(la.base, ra.base), true);
                frac.num = mul!(res.outside, frac.num);
                match frac.normalise() {
                    Err(MathError::InvalidFraction) => match res.inside {
                        1 => Ok(Token::Integer(frac.int)),
                        _ => Ok(Token::SIntRoot(SRoot::new(frac.int, res.inside))),
                    },
                    Err(x) => Err(x),
                    _ => match res.inside {
                        1 => Ok(Token::Fraction(frac)),
                        _ => Ok(Token::SFracRoot(SRoot::new(frac, res.inside))),
                    },
                }
            }
            (Token::SFracRoot(mut la), Token::SIntRoot(ra)) => {
                la.mul.num = add!(la.mul.num, mul!(la.mul.int, la.mul.den));
                la.mul.den = mul!(mul!(la.mul.den, ra.mul), ra.base);
                la.mul.int = 0;
                let res = factorise(mul!(la.base, ra.base), true);
                la.mul.num = mul!(la.mul.num, res.outside);
                match la.mul.normalise() {
                    Err(MathError::InvalidFraction) => match res.inside {
                        1 => Ok(Token::Integer(la.mul.int)),
                        _ => Ok(Token::SIntRoot(SRoot::new(la.mul.int, res.inside))),
                    },
                    Err(x) => Err(x),
                    _ => match res.inside {
                        1 => Ok(Token::Fraction(la.mul)),
                        _ => Ok(Token::SFracRoot(SRoot::new(la.mul, res.inside))),
                    },
                }
            }
            (Token::SIntRoot(la), Token::SFracRoot(ra)) => {
                let mut frac = Fraction::new(
                    0,
                    mul!(la.mul, ra.mul.den),
                    mul!(add!(ra.mul.num, mul!(ra.mul.den, ra.mul.int)), ra.base),
                );
                let res = factorise(mul!(la.base, ra.base), true);
                frac.num = mul!(frac.num, res.outside);
                match frac.normalise() {
                    Err(MathError::InvalidFraction) => match res.inside {
                        1 => Ok(Token::Integer(frac.int)),
                        _ => Ok(Token::SIntRoot(SRoot::new(frac.int, res.inside))),
                    },
                    Err(x) => Err(x),
                    _ => match res.inside {
                        1 => Ok(Token::Fraction(frac)),
                        _ => Ok(Token::SFracRoot(SRoot::new(frac, res.inside))),
                    },
                }
            }
            (Token::SFracRoot(mut la), Token::SFracRoot(mut ra)) => {
                ra.mul.num = mul!(ra.mul.num, ra.base);
                ra.mul.int = mul!(ra.mul.int, ra.base);
                match ra.mul.normalise() {
                    Err(MathError::InvalidFraction) => {
                        la.mul.num = add!(la.mul.num, mul!(la.mul.int, la.mul.den));
                        la.mul.den = mul!(la.mul.den, ra.mul.int); // Already have multiplied by base so no need to do it again
                        la.mul.int = 0;
                        let res = factorise(mul!(la.base, ra.base), true);
                        la.mul.num = mul!(la.mul.num, res.outside);
                        match la.mul.normalise() {
                            Err(MathError::InvalidFraction) => match res.inside {
                                1 => Ok(Token::Integer(la.mul.int)),
                                _ => Ok(Token::s_int_root(la.mul.int, res.inside)),
                            },
                            Err(x) => Err(x),
                            _ => match res.inside {
                                1 => Ok(Token::Fraction(la.mul)),
                                _ => Ok(Token::s_fraction_root(la.mul, res.inside)),
                            },
                        }
                    }
                    Err(x) => Err(x),
                    _ => {
                        let res = factorise(mul!(la.base, ra.base), true);
                        match la.mul.div(&ra.mul) {
                            Err(MathError::InvalidFraction) => match res.inside {
                                1 => Ok(Token::Integer(mul!(la.mul.int, res.outside))),
                                _ => {
                                    Ok(Token::s_int_root(mul!(la.mul.int, res.outside), res.inside))
                                }
                            },
                            Err(x) => Err(x),
                            _ => {
                                la.mul.num = mul!(la.mul.num, res.outside);
                                la.mul.int = mul!(la.mul.int, res.outside);
                                match la.mul.normalise() {
                                    Err(MathError::InvalidFraction) => match res.inside {
                                        1 => Ok(Token::Integer(la.mul.int)),
                                        _ => Ok(Token::s_int_root(la.mul.int, res.inside)),
                                    },
                                    Err(x) => Err(x),
                                    _ => match res.inside {
                                        1 => Ok(Token::Fraction(la.mul)),
                                        _ => Ok(Token::s_fraction_root(la.mul, res.inside)),
                                    },
                                }
                            }
                        }
                    }
                }
            }
            (Token::CIntRoot(la), Token::CIntRoot(ra)) => {
                let mut frac = Fraction::new(0, la.mul, mul!(ra.mul, mul!(ra.base, ra.base)));
                let res = factorise(mul!(la.base, mul!(ra.base, ra.base)), false);
                frac.num = mul!(res.outside, frac.num);
                match frac.normalise() {
                    Err(MathError::InvalidFraction) => match res.inside {
                        1 => Ok(Token::Integer(frac.int)),
                        _ => Ok(Token::c_int_root(frac.int, res.inside)),
                    },
                    Err(x) => Err(x),
                    _ => match res.inside {
                        1 => Ok(Token::Fraction(frac)),
                        _ => Ok(Token::c_fraction_root(frac, res.inside)),
                    },
                }
            }
            (Token::CFracRoot(la), Token::CIntRoot(ra)) => {
                let mut frac = Fraction::new(
                    0,
                    add!(la.mul.num, mul!(la.mul.int, la.mul.den)),
                    mul!(mul!(la.mul.den, ra.mul), ra.base),
                );
                let res = factorise(mul!(la.base, mul!(ra.base, ra.base)), false);
                frac.num = mul!(frac.num, res.outside);
                match frac.normalise() {
                    Err(MathError::InvalidFraction) => match res.inside {
                        1 => Ok(Token::Integer(frac.int)),
                        _ => Ok(Token::c_int_root(frac.int, res.inside)),
                    },
                    Err(x) => Err(x),
                    _ => match res.inside {
                        1 => Ok(Token::Fraction(frac)),
                        _ => Ok(Token::c_fraction_root(frac, res.inside)),
                    },
                }
            }
            (Token::CIntRoot(la), Token::CFracRoot(ra)) => {
                let mut frac = Fraction::new(
                    0,
                    mul!(la.mul, ra.mul.den),
                    mul!(add!(ra.mul.num, mul!(ra.mul.den, ra.mul.int)), ra.base),
                );
                let res = factorise(mul!(la.base, mul!(ra.base, ra.base)), false);
                frac.num = mul!(frac.num, res.outside);
                match frac.normalise() {
                    Err(MathError::InvalidFraction) => match res.inside {
                        1 => Ok(Token::Integer(frac.int)),
                        _ => Ok(Token::c_int_root(frac.int, res.inside)),
                    },
                    Err(x) => Err(x),
                    _ => match res.inside {
                        1 => Ok(Token::Fraction(frac)),
                        _ => Ok(Token::c_fraction_root(frac, res.inside)),
                    },
                }
            }
            (Token::CFracRoot(mut la), Token::CFracRoot(mut ra)) => {
                ra.mul.num = mul!(ra.mul.num, ra.base);
                ra.mul.int = mul!(ra.mul.int, ra.base);
                match ra.mul.normalise() {
                    Err(MathError::InvalidFraction) => {
                        la.mul.num = add!(la.mul.num, mul!(la.mul.int, la.mul.den));
                        la.mul.den = mul!(la.mul.den, ra.mul.int); // Already have multiplied by base so no need to do it again
                        la.mul.int = 0;
                        let res = factorise(mul!(la.base, mul!(ra.base, ra.base)), true);
                        la.mul.num = mul!(la.mul.num, res.outside);
                        match la.mul.normalise() {
                            Err(MathError::InvalidFraction) => match res.inside {
                                1 => Ok(Token::Integer(la.mul.int)),
                                _ => Ok(Token::c_int_root(la.mul.int, res.inside)),
                            },
                            Err(x) => Err(x),
                            _ => match res.inside {
                                1 => Ok(Token::Fraction(la.mul)),
                                _ => Ok(Token::c_fraction_root(la.mul, res.inside)),
                            },
                        }
                    }
                    Err(x) => Err(x),
                    _ => {
                        let res = factorise(mul!(la.base, ra.base), true);
                        match la.mul.div(&ra.mul) {
                            Err(MathError::InvalidFraction) => match res.inside {
                                1 => Ok(Token::Integer(mul!(la.mul.int, res.outside))),
                                _ => Ok(Token::CIntRoot(CRoot::new(
                                    mul!(la.mul.int, res.outside),
                                    res.inside,
                                ))),
                            },
                            Err(x) => Err(x),
                            _ => {
                                la.mul.num = mul!(la.mul.num, res.outside);
                                la.mul.int = mul!(la.mul.int, res.outside);
                                match la.mul.normalise() {
                                    Err(MathError::InvalidFraction) => match res.inside {
                                        1 => Ok(Token::Integer(la.mul.int)),
                                        _ => Ok(Token::c_int_root(la.mul.int, res.inside)),
                                    },
                                    Err(x) => Err(x),
                                    _ => match res.inside {
                                        1 => Ok(Token::Fraction(la.mul)),
                                        _ => Ok(Token::c_fraction_root(la.mul, res.inside)),
                                    },
                                }
                            }
                        }
                    }
                }
            }
            _ => Err(MathError::Overflow),
        }
    }
}

pub fn div(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    match (l_number.is_pi(), r_number.is_pi()) {
        (true, true) => match try_div((&l_number.normal(), &r_number.normal())) {
            Err(MathError::Overflow) => Ok(Token::Double(double_check!(
                l_number.double() / r_number.double()
            ))),
            value => value,
        },
        (true, false) => match try_div((&l_number.normal(), &r_number)) {
            Err(MathError::Overflow) => Ok(Token::Double(double_check!(
                l_number.double() / r_number.double()
            ))),
            value => value?.pi(),
        },
        (false, true) => match (&l_number, &r_number) {
            // No need to check for x/0 as pi numbers promise to be non-zero
            (Token::Integer(0), _) => Ok(Token::Integer(0)),
            _ => Ok(Token::Double(double_check!(
                l_number.double() / r_number.double()
            ))),
        },
        (false, false) => match try_div((&l_number, &r_number)) {
            Err(MathError::Overflow) => Ok(Token::Double(double_check!(
                l_number.double() / r_number.double()
            ))),
            value => value,
        },
    }
}

fn try_exp(tup: (&Token, &Token)) -> Result<Token, MathError> {
    match tup {
        // Check if 0^0.
        (Token::Integer(0), Token::Integer(0)) => Err(MathError::ExponentiationError),
        (Token::Double(a), Token::Double(b)) if !a.is_normal() && !b.is_normal() => {
            Err(MathError::ExponentiationError)
        }
        (Token::Integer(0), Token::Double(a)) | (Token::Double(a), Token::Integer(0))
            if !a.is_normal() =>
        {
            Err(MathError::ExponentiationError)
        }

        // Check if x^0, x^1, 0^x or 1^x
        (Token::Integer(1), _) | (_, Token::Integer(0)) => Ok(Token::Integer(1)),
        (Token::Integer(0), _) => Ok(Token::Integer(0)),
        (x, Token::Integer(1)) => Ok(x.clone()),

        (Token::Integer(la), Token::Integer(mut ra)) => {
            let negative = ra < 0;
            ra = ra.abs();
            if ra > u32::MAX as i128 {
                return Err(MathError::Overflow);
            }
            let a = none_to_err!(la.checked_pow(ra as u32));
            match negative {
                false => Ok(Token::Integer(a)),
                true => Ok(Token::fraction(0, 1, a)),
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
            match res.normalise() {
                Err(MathError::InvalidFraction) => Ok(Token::Integer(res.int)),
                Err(x) => Err(x),
                _ => Ok(Token::Fraction(res)),
            }
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
            let res = factorise(*la, ra.den == 2);
            let outside_root = none_to_err!(la.checked_pow(ra.int as u32));
            let inside_root = none_to_err!(res.inside.checked_pow(ra.num as u32));
            let outside = none_to_err!(res.outside.checked_pow(ra.num as u32));
            if inside_root == 1 {
                return match negative {
                    true => Ok(Token::fraction(0, 1, mul!(outside_root, outside))),
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
                        Some(sq_inside_root) => Ok(Token::c_frac_root(
                            0,
                            1,
                            mul!(mul!(outside_root, outside), inside_root),
                            sq_inside_root,
                        )),
                    },
                }
            } else {
                match negative {
                    false => Ok(Token::s_int_root(mul!(outside_root, outside), inside_root)),
                    true => Ok(Token::s_frac_root(
                        0,
                        1,
                        mul!(mul!(outside_root, outside), inside_root),
                        inside_root,
                    )),
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
            let mut res = Fraction::new(
                0,
                mul!(outside_root_num, outside_num),
                mul!(mul!(outside_root_den, outside_den), inside_root_den),
            );
            if ra.den == 3 {
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::c_int_root(
                        res.int,
                        mul!(
                            inside_root_num,
                            none_to_err!(inside_root_den.checked_pow(2))
                        ),
                    )),
                    Err(x) => Err(x),
                    _ => Ok(Token::c_fraction_root(
                        res,
                        mul!(
                            inside_root_num,
                            none_to_err!(inside_root_den.checked_pow(2))
                        ),
                    )),
                }
            } else {
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::SIntRoot(SRoot::new(
                        res.int,
                        mul!(inside_root_num, inside_root_den),
                    ))),
                    Err(x) => Err(x),
                    _ => Ok(Token::s_fraction_root(
                        res,
                        mul!(inside_root_num, inside_root_den),
                    )),
                }
            }
        }

        (Token::SIntRoot(la), Token::Integer(mut ra)) => {
            let negative = ra < 0;
            ra = ra.abs();
            if ra > u32::MAX as i128 {
                return Err(MathError::Overflow);
            }
            let mul = mul!(
                none_to_err!(la.mul.checked_pow(ra as u32)),
                none_to_err!(la.base.checked_pow(ra as u32 / 2))
            );
            if ra % 2 == 0 {
                match negative {
                    true => Ok(Token::fraction(0, 1, mul)),
                    false => Ok(Token::Integer(mul)),
                }
            } else {
                match negative {
                    true => Ok(Token::s_frac_root(0, 1, mul, la.base)),
                    false => Ok(Token::s_int_root(mul, la.base)),
                }
            }
        }

        (Token::CIntRoot(la), Token::Integer(mut ra)) => {
            let negative = ra < 0;
            ra = ra.abs();
            if ra > u32::MAX as i128 {
                return Err(MathError::Overflow);
            }
            let mul = mul!(
                none_to_err!(la.mul.checked_pow(ra as u32)),
                none_to_err!(la.base.checked_pow(ra as u32 / 3))
            );
            if ra % 3 == 0 {
                match negative {
                    true => Ok(Token::fraction(0, 1, mul)),
                    false => Ok(Token::Integer(mul)),
                }
            } else {
                let base = none_to_err!(la.base.checked_pow((ra % 3) as u32));
                match negative {
                    true => Ok(Token::c_frac_root(0, 1, mul, base)),
                    false => Ok(Token::c_int_root(mul, base)),
                }
            }
        }

        (Token::SFracRoot(la), Token::Integer(mut ra)) => {
            let negative = ra < 0;
            ra = ra.abs();
            if ra > u32::MAX as i128 {
                return Err(MathError::Overflow);
            }
            let num = la.mul.int * la.mul.den + la.mul.num;
            let mul_num = mul!(
                none_to_err!(num.checked_pow(ra as u32)),
                none_to_err!(la.base.checked_pow(ra as u32 / 2))
            );
            let mul_den = none_to_err!(la.mul.den.checked_pow(ra as u32));

            let mut res = match negative {
                true => Fraction::new(0, mul_den, mul_num),
                false => Fraction::new(0, mul_den, mul_num),
            };

            if ra % 2 == 0 {
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(res.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(res)),
                }
            } else {
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::c_int_root(res.int, la.base)),
                    Err(x) => Err(x),
                    _ => Ok(Token::c_fraction_root(res, la.base)),
                }
            }
        }
        (Token::CFracRoot(la), Token::Integer(mut ra)) => {
            let negative = ra < 0;
            ra = ra.abs();
            if ra > u32::MAX as i128 {
                return Err(MathError::Overflow);
            }
            let num = la.mul.int * la.mul.den + la.mul.num;
            let mul_num = mul!(
                none_to_err!(num.checked_pow(ra as u32)),
                none_to_err!(la.base.checked_pow(ra as u32 / 3))
            );
            let mul_den = none_to_err!(la.mul.den.checked_pow(ra as u32));

            let mut res = match negative {
                true => Fraction::new(0, mul_den, mul_num),
                false => Fraction::new(0, mul_den, mul_num),
            };

            if ra % 3 == 0 {
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(res.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(res)),
                }
            } else {
                let base = none_to_err!(la.base.checked_pow((ra % 3) as u32));
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::c_int_root(res.int, base)),
                    Err(x) => Err(x),
                    _ => Ok(Token::c_fraction_root(res, base)),
                }
            }
        }
        _ => Err(MathError::Overflow),
    }
}

pub fn exp(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    match try_exp((&l_number, &r_number)) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            (l_number.double()).powf(r_number.double())
        ))),
        value => value,
    }
}
