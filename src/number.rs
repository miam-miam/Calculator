use crate::my_math::factorise;
use crate::types::{CRoot, Fraction, MathError, SRoot, Token};

pub fn add(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    let try_add = |tup| match tup {
        (Token::Fraction(mut la), Token::Fraction(ra)) => match la.add(&ra) {
            Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
            Err(x) => Err(x),
            _ => Ok(Token::Fraction(la)),
        },
        (Token::Fraction(mut la), Token::Integer(ra))
        | (Token::Integer(ra), Token::Fraction(mut la)) => {
            la.int = add!(la.int, ra);
            Ok(Token::Fraction(la))
        }
        (Token::Integer(la), Token::Integer(ra)) => Ok(Token::Integer(add!(la, ra))),
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
        (la, ra) => Ok(Token::Double(double_check!(double!(la) + double!(ra)))),
    };
    match try_add((l_number, r_number)) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            double!(l_number) + double!(r_number)
        ))),
        value => value,
    }
}

pub fn sub(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    let try_sub = |tup| match tup {
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
        (Token::Double(la), ra) => Ok(Token::Double(double_check!(la - double!(ra)))),
        (la, Token::Double(ra)) => Ok(Token::Double(double_check!(double!(la) - ra))),
        _ => Err(MathError::Impossible),
    };
    match try_sub((l_number, r_number)) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            double!(l_number) - double!(r_number)
        ))),
        value => value,
    }
}

pub fn mul(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    let try_mul = |tup| match tup {
        (Token::Fraction(mut la), Token::Fraction(ra)) => match la.mul(&ra) {
            Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
            Err(x) => Err(x),
            _ => Ok(Token::Fraction(la)),
        },
        (Token::Fraction(mut la), Token::Integer(ra))
        | (Token::Integer(ra), Token::Fraction(mut la)) => {
            la.int = mul!(la.int, ra);
            la.num = mul!(la.num, ra);
            match la.normalise() {
                Err(MathError::InvalidFraction) => Ok(Token::Integer(la.int)),
                Err(x) => Err(x),
                _ => Ok(Token::Fraction(la)),
            }
        }
        (Token::Integer(la), Token::Integer(ra)) => Ok(Token::Integer(mul!(la, ra))),
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
                        Ok(Token::SIntRoot(SRoot::new(la.mul.int, res.inside)))
                    }
                    Err(x) => Err(x),
                    _ => Ok(Token::SFracRoot(SRoot::new(la.mul, res.inside))),
                },
            }
        }
        (Token::SFracRoot(mut la), Token::SFracRoot(ra)) => match la.mul.mul(&ra.mul) {
            Err(MathError::InvalidFraction) => {
                let res = factorise(mul!(la.base, ra.base), true);
                la.mul.int = mul!(la.mul.int, res.outside);
                match res.inside {
                    1 => Ok(Token::Integer(la.mul.int)),
                    _ => Ok(Token::SIntRoot(SRoot::new(la.mul.int, res.inside))),
                }
            }
            Err(x) => Err(x),
            _ => Ok(Token::SFracRoot(la)),
        },
        (Token::CIntRoot(mut la), Token::CIntRoot(ra)) => {
            la.mul = mul!(la.mul, ra.mul);
            let res = factorise(mul!(la.base, ra.base), true);
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
                        Ok(Token::CIntRoot(CRoot::new(la.mul.int, res.inside)))
                    }
                    Err(x) => Err(x),
                    _ => Ok(Token::CFracRoot(CRoot::new(la.mul, res.inside))),
                },
            }
        }
        (Token::CFracRoot(mut la), Token::CFracRoot(ra)) => match la.mul.mul(&ra.mul) {
            Err(MathError::InvalidFraction) => {
                let res = factorise(mul!(la.base, ra.base), true);
                la.mul.int = mul!(la.mul.int, res.outside);
                match res.inside {
                    1 => Ok(Token::Integer(la.mul.int)),
                    _ => Ok(Token::CIntRoot(CRoot::new(la.mul.int, res.inside))),
                }
            }
            Err(x) => Err(x),
            _ => Ok(Token::CFracRoot(la)),
        },
        (la, ra) => Ok(Token::Double(double_check!(double!(la) * double!(ra)))),
    };
    match try_mul((l_number, r_number)) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            double!(l_number) * double!(r_number)
        ))),
        value => value,
    }
}

pub fn div(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    let try_div = |tup| {
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
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(Token::Integer(res.int)),
                    Err(x) => Err(x),
                    _ => Ok(Token::Fraction(res)),
                }
            }
            (Token::Fraction(mut la), Token::Integer(ra)) => {
                la.num = add!(la.num, mul!(la.int, la.den));
                la.den = mul!(la.den, ra);
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
    let try_exp = |tup| {
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

        // Check if x^0, 0^x or 1^x
        if let (Token::Integer(1), _) | (_, Token::Integer(0)) = tup {
            return Ok(Token::Integer(1));
        } else if let (Token::Integer(0), _) = tup {
            return Ok(Token::Integer(0));
        } else if let (x, Token::Integer(1)) = tup {
            return Ok(x);
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
                let mut res = Fraction::new(
                    0,
                    mul!(outside_root_num, outside_num),
                    mul!(mul!(outside_root_den, outside_den), inside_root_den),
                );
                if ra.den == 3 {
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
                        true => Ok(Token::Fraction(Fraction::new(0, 1, mul))),
                        false => Ok(Token::Integer(mul)),
                    }
                } else {
                    match negative {
                        true => Ok(Token::SFracRoot(SRoot::new(
                            Fraction::new(0, 1, mul),
                            la.base,
                        ))),
                        false => Ok(Token::SIntRoot(SRoot::new(mul, la.base))),
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
                        true => Ok(Token::Fraction(Fraction::new(0, 1, mul))),
                        false => Ok(Token::Integer(mul)),
                    }
                } else {
                    let base = none_to_err!(la.base.checked_pow((ra % 3) as u32));
                    match negative {
                        true => Ok(Token::CFracRoot(CRoot::new(Fraction::new(0, 1, mul), base))),
                        false => Ok(Token::CIntRoot(CRoot::new(mul, base))),
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
                        Err(MathError::InvalidFraction) => {
                            Ok(Token::CIntRoot(CRoot::new(res.int, la.base)))
                        }
                        Err(x) => Err(x),
                        _ => Ok(Token::CFracRoot(CRoot::new(res, la.base))),
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
                        Err(MathError::InvalidFraction) => {
                            Ok(Token::CIntRoot(CRoot::new(res.int, base)))
                        }
                        Err(x) => Err(x),
                        _ => Ok(Token::CFracRoot(CRoot::new(res, base))),
                    }
                }
            }
            (la, ra) => Ok(Token::Double(double_check!(double!(la).powf(double!(ra))))),
        }
    };
    match try_exp((l_number, r_number)) {
        Err(MathError::Overflow) => Ok(Token::Double(double_check!(
            (double!(l_number)).powf(double!(r_number))
        ))),
        value => value,
    }
}
