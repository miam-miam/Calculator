use crate::my_math::factorise;
use crate::types::{BasicToken, CRoot, Combined, Fraction, MathError, SRoot, Token};

pub fn try_add(lhs: BasicToken, rhs: BasicToken) -> Result<BasicToken, MathError> {
    match (lhs, rhs) {
        (BasicToken::Fraction(la), BasicToken::Fraction(ra)) => la + ra,
        (BasicToken::Fraction(mut la), BasicToken::Integer(ra))
        | (BasicToken::Integer(ra), BasicToken::Fraction(mut la)) => {
            la.int = add!(la.int, ra);
            Ok(BasicToken::Fraction(la))
        }
        (BasicToken::Integer(la), BasicToken::Integer(ra)) => Ok(BasicToken::Integer(add!(la, ra))),
        (BasicToken::SIntRoot(mut la), BasicToken::SIntRoot(ra)) if la.base == ra.base => {
            la.mul = add!(la.mul, ra.mul);
            Ok(la.normalise())
        }
        (BasicToken::SFracRoot(mut la), BasicToken::SIntRoot(ra))
        | (BasicToken::SIntRoot(ra), BasicToken::SFracRoot(mut la))
            if la.base == ra.base =>
        {
            la.mul.int = add!(la.mul.int, ra.mul);
            la.normalise()
        }
        (BasicToken::SFracRoot(la), BasicToken::SFracRoot(ra)) if la.base == ra.base => {
            BasicToken::new_s_root((la.mul + ra.mul)?, la.base)
        }
        (BasicToken::CIntRoot(mut la), BasicToken::CIntRoot(ra)) if la.base == ra.base => {
            la.mul = add!(la.mul, ra.mul);
            la.normalise()
        }
        (BasicToken::CFracRoot(mut la), BasicToken::CIntRoot(ra))
        | (BasicToken::CIntRoot(ra), BasicToken::CFracRoot(mut la))
            if la.base == ra.base =>
        {
            la.mul.int = add!(la.mul.int, ra.mul);
            la.normalise()
        }
        (BasicToken::CFracRoot(la), BasicToken::CFracRoot(ra)) if la.base == ra.base => {
            BasicToken::new_c_root((la.mul + ra.mul)?, la.base)
        }
        (_, BasicToken::Double(_)) | (BasicToken::Double(_), _) => Err(MathError::Overflow),
        _ => Err(MathError::Combine),
    }
}

pub fn add(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    match (l_number, r_number) {
        (Token::Pi(l), Token::Pi(r)) => match try_add(l, r) {
            Err(MathError::Overflow) => Ok(Token::Basic(BasicToken::Double(double_check!(
                (l.double() + r.double()) * std::f64::consts::PI
            )))),
            Err(MathError::Combine) => Ok(Token::combined(vec![], vec![l, r])),
            Ok(BasicToken::Integer(0)) => Ok(Token::Basic(BasicToken::Integer(0))),
            value => Ok(Token::Pi(value?)),
        },
        (Token::Basic(l), Token::Basic(r)) => match try_add(l, r) {
            Err(MathError::Overflow) => Ok(Token::Basic(BasicToken::Double(double_check!(
                l.double() + r.double()
            )))),
            Err(MathError::Combine) => Ok(Token::combined(vec![l, r], vec![])),
            value => Ok(Token::Basic(value?)),
        },
        (Token::Combined(l), r) | (r, Token::Combined(l)) => Ok(l.add_combined(r)?),
        (Token::Basic(basic), Token::Pi(pi)) | (Token::Pi(pi), Token::Basic(basic)) => {
            Ok(Token::combined(vec![basic], vec![pi]))
        }
    }
}

fn try_sub(lhs: BasicToken, rhs: BasicToken) -> Result<BasicToken, MathError> {
    match (lhs, rhs) {
        (BasicToken::Integer(la), BasicToken::Integer(ra)) => Ok(BasicToken::Integer(sub!(la, ra))),
        (BasicToken::Fraction(mut la), BasicToken::Integer(ra)) => {
            la.int = sub!(la.int, ra);
            Ok(BasicToken::Fraction(la))
        }
        (BasicToken::Integer(la), BasicToken::Fraction(mut ra)) => {
            ra.int = sub!(la, ra.int);
            ra.num = mul!(ra.num, -1);
            Ok(BasicToken::Fraction(ra))
        }
        (BasicToken::Fraction(la), BasicToken::Fraction(ra)) => la - ra,
        (BasicToken::SIntRoot(mut la), BasicToken::SIntRoot(ra)) if la.base == ra.base => {
            la.mul = sub!(la.mul, ra.mul);
            Ok(la.normalise())
        }
        (BasicToken::SFracRoot(mut la), BasicToken::SIntRoot(ra)) if la.base == ra.base => {
            la.mul.int = sub!(la.mul.int, ra.mul);
            la.normalise()
        }
        (BasicToken::SIntRoot(la), BasicToken::SFracRoot(mut ra)) if la.base == ra.base => {
            ra.mul.int = sub!(la.mul, ra.mul.int);
            ra.mul.num = mul!(ra.mul.num, -1);
            ra.normalise()
        }
        (BasicToken::SFracRoot(la), BasicToken::SFracRoot(ra)) if la.base == ra.base => {
            BasicToken::new_s_root((la.mul - ra.mul)?, la.base)
        }
        (BasicToken::CIntRoot(mut la), BasicToken::CIntRoot(ra)) if la.base == ra.base => {
            la.mul = sub!(la.mul, ra.mul);
            la.normalise()
        }
        (BasicToken::CFracRoot(mut la), BasicToken::CIntRoot(ra)) if la.base == ra.base => {
            la.mul.int = sub!(la.mul.int, ra.mul);
            la.normalise()
        }
        (BasicToken::CIntRoot(la), BasicToken::CFracRoot(mut ra)) if la.base == ra.base => {
            ra.mul.int = sub!(la.mul, ra.mul.int);
            ra.mul.num = mul!(ra.mul.num, -1);
            ra.normalise()
        }
        (BasicToken::CFracRoot(la), BasicToken::CFracRoot(ra)) if la.base == ra.base => {
            BasicToken::new_c_root((la.mul - ra.mul)?, la.base)
        }
        (_, BasicToken::Double(_)) | (BasicToken::Double(_), _) => Err(MathError::Overflow),
        _ => Err(MathError::Combine),
    }
}

pub fn sub(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    match (l_number, r_number) {
        (Token::Pi(l), Token::Pi(r)) => match try_sub(l, r) {
            Err(MathError::Overflow) => Ok(Token::Basic(BasicToken::Double(double_check!(
                (l.double() - r.double()) * std::f64::consts::PI
            )))),
            Err(MathError::Combine) => Ok(Token::combined(vec![], vec![l, r.negate()?])),
            Ok(BasicToken::Integer(0)) => Ok(Token::Basic(BasicToken::Integer(0))),
            value => Ok(Token::Pi(value?)),
        },
        (Token::Basic(l), Token::Basic(r)) => match try_sub(l, r) {
            Err(MathError::Overflow) => Ok(Token::Basic(BasicToken::Double(double_check!(
                l.double() - r.double()
            )))),
            Err(MathError::Combine) => Ok(Token::combined(vec![l, r.negate()?], vec![])),
            value => Ok(Token::Basic(value?)),
        },
        (Token::Combined(l), r) => Ok(l.add_combined(r.negate()?)?),
        (l, Token::Combined(r)) => Ok(r.negate()?.add_combined(l)?),
        (Token::Basic(basic), Token::Pi(pi)) => {
            Ok(Token::combined(vec![basic], vec![pi.negate()?]))
        }
        (Token::Pi(pi), Token::Basic(basic)) => {
            Ok(Token::combined(vec![basic.negate()?], vec![pi]))
        }
    }
}

pub fn try_mul(lhs: BasicToken, rhs: BasicToken) -> Result<BasicToken, MathError> {
    match (lhs, rhs) {
        // a*0 = 0
        (BasicToken::Integer(0), _) | (_, BasicToken::Integer(0)) => Ok(BasicToken::Integer(0)),

        (BasicToken::Fraction(la), BasicToken::Fraction(ra)) => la * ra,
        (BasicToken::Fraction(mut la), BasicToken::Integer(ra))
        | (BasicToken::Integer(ra), BasicToken::Fraction(mut la)) => {
            la.int = mul!(la.int, ra);
            la.num = mul!(la.num, ra);
            la.normalise()
        }
        (BasicToken::Integer(la), BasicToken::Integer(ra)) => Ok(BasicToken::Integer(mul!(la, ra))),
        (BasicToken::SIntRoot(mut la), BasicToken::SIntRoot(ra)) => {
            la.mul = mul!(la.mul, ra.mul);
            let res = factorise(mul!(la.base, ra.base), true);
            la.mul = mul!(la.mul, res.outside);
            la.base = res.inside;
            la.base = res.inside;
            Ok(la.normalise())
        }
        (BasicToken::SFracRoot(mut la), BasicToken::SIntRoot(ra))
        | (BasicToken::SIntRoot(ra), BasicToken::SFracRoot(mut la)) => {
            la.mul.int = mul!(la.mul.int, ra.mul);
            la.mul.num = mul!(la.mul.num, ra.mul);
            let res = factorise(mul!(la.base, ra.base), true);
            la.mul.int = mul!(la.mul.int, res.outside);
            la.mul.num = mul!(la.mul.num, res.outside);
            la.base = res.inside;
            la.normalise()
        }
        (BasicToken::SFracRoot(mut la), BasicToken::SFracRoot(ra)) => {
            let res = factorise(mul!(la.base, ra.base), true);
            la.mul.int = mul!(la.mul.int, res.outside);
            la.mul.num = mul!(la.mul.num, res.outside);
            BasicToken::new_s_root((la.mul * ra.mul)?, res.inside)
        }
        (BasicToken::CIntRoot(mut la), BasicToken::CIntRoot(ra)) => {
            la.mul = mul!(la.mul, ra.mul);
            let res = factorise(mul!(la.base, ra.base), false);
            la.mul = mul!(la.mul, res.outside);
            la.base = res.inside;
            la.normalise()
        }
        (BasicToken::CFracRoot(mut la), BasicToken::CIntRoot(ra))
        | (BasicToken::CIntRoot(ra), BasicToken::CFracRoot(mut la)) => {
            la.mul.int = mul!(la.mul.int, ra.mul);
            la.mul.num = mul!(la.mul.num, ra.mul);
            let res = factorise(mul!(la.base, ra.base), false);
            la.mul.int = mul!(la.mul.int, res.outside);
            la.mul.num = mul!(la.mul.num, res.outside);
            la.base = res.inside;
            la.normalise()
        }
        (BasicToken::CFracRoot(mut la), BasicToken::CFracRoot(ra)) => {
            let res = factorise(mul!(la.base, ra.base), false);
            la.mul.int = mul!(la.mul.int, res.outside);
            la.mul.num = mul!(la.mul.num, res.outside);
            BasicToken::new_c_root((la.mul * ra.mul)?, res.inside)
        }
        _ => Err(MathError::Overflow),
    }
}

pub fn mul(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    match (l_number, r_number) {
        (Token::Pi(l), Token::Basic(r)) | (Token::Basic(l), Token::Pi(r)) => match try_mul(l, r) {
            Err(MathError::Overflow) => Ok(Token::Basic(BasicToken::Double(double_check!(
                (l.double() * r.double()) * std::f64::consts::PI
            )))),
            Ok(BasicToken::Integer(0)) => Ok(Token::Basic(BasicToken::Integer(0))),
            value => Ok(Token::Pi(value?)),
        },
        (Token::Basic(l), Token::Basic(r)) => match try_mul(l, r) {
            Err(MathError::Overflow) => Ok(Token::Basic(BasicToken::Double(double_check!(
                l.double() * r.double()
            )))),
            value => Ok(Token::Basic(value?)),
        },
        (Token::Pi(l), Token::Pi(r)) => Ok(Token::Basic(BasicToken::Double(double_check!(
            l.double() * r.double() * std::f64::consts::PI * std::f64::consts::PI
        )))),
        (Token::Combined(l), r) | (r, Token::Combined(l)) => Ok(l.mul_combined(r)?),
    }
}

fn try_div(lhs: BasicToken, rhs: BasicToken) -> Result<BasicToken, MathError> {
    {
        match (lhs, rhs) {
            // Check if zero.
            (_, BasicToken::Integer(0)) => Err(MathError::DivisionByZero),
            (_, BasicToken::Double(a)) if !a.is_normal() => Err(MathError::DivisionByZero),

            // 0/a = 0
            (BasicToken::Integer(0), _) => Ok(BasicToken::Integer(0)),

            (BasicToken::Integer(la), BasicToken::Integer(ra)) => {
                Fraction::new(0, la, ra).normalise()
            }
            (BasicToken::Fraction(mut la), BasicToken::Integer(ra)) => {
                la.num = add!(la.num, mul!(la.int, la.den));
                la.den = mul!(la.den, ra);
                la.int = 0;
                la.normalise()
            }
            (BasicToken::Integer(la), BasicToken::Fraction(mut ra)) => {
                let old_num = ra.num;
                ra.num = mul!(la, ra.den);
                ra.den = add!(old_num, mul!(ra.den, ra.int));
                ra.int = 0;
                ra.normalise()
            }
            (BasicToken::Fraction(la), BasicToken::Fraction(ra)) => la / ra,
            (BasicToken::SIntRoot(la), BasicToken::SIntRoot(ra)) => {
                let mut frac = Fraction::new(0, la.mul, mul!(ra.mul, ra.base));
                let res = factorise(mul!(la.base, ra.base), true);
                frac.num = mul!(res.outside, frac.num);
                BasicToken::new_s_root(frac.normalise()?, res.inside)
            }
            (BasicToken::SFracRoot(mut la), BasicToken::SIntRoot(ra)) => {
                la.mul.num = add!(la.mul.num, mul!(la.mul.int, la.mul.den));
                la.mul.den = mul!(mul!(la.mul.den, ra.mul), ra.base);
                la.mul.int = 0;
                let res = factorise(mul!(la.base, ra.base), true);
                la.mul.num = mul!(la.mul.num, res.outside);
                BasicToken::new_s_root(la.mul.normalise()?, res.inside)
            }
            (BasicToken::SIntRoot(la), BasicToken::SFracRoot(ra)) => {
                let mut frac = Fraction::new(
                    0,
                    mul!(la.mul, ra.mul.den),
                    mul!(add!(ra.mul.num, mul!(ra.mul.den, ra.mul.int)), ra.base),
                );
                let res = factorise(mul!(la.base, ra.base), true);
                frac.num = mul!(frac.num, res.outside);
                BasicToken::new_s_root(frac.normalise()?, res.inside)
            }
            (BasicToken::SFracRoot(mut la), BasicToken::SFracRoot(mut ra)) => {
                ra.mul.num = mul!(ra.mul.num, ra.base);
                ra.mul.int = mul!(ra.mul.int, ra.base);
                match ra.mul.normalise()? {
                    BasicToken::Integer(int) => {
                        la.mul.num = add!(la.mul.num, mul!(la.mul.int, la.mul.den));
                        la.mul.den = mul!(la.mul.den, int); // Already have multiplied by base so no need to do it again
                        la.mul.int = 0;
                        let res = factorise(mul!(la.base, ra.base), true);
                        la.mul.num = mul!(la.mul.num, res.outside);
                        BasicToken::new_s_root(la.mul.normalise()?, res.inside)
                    }
                    BasicToken::Fraction(frac) => {
                        let res = factorise(mul!(la.base, ra.base), true);
                        la.mul.int = mul!(la.mul.int, res.outside);
                        la.mul.num = mul!(la.mul.int, res.outside);
                        BasicToken::new_s_root((la.mul / frac)?, res.inside)
                    }
                    _ => unreachable!(),
                }
            }
            (BasicToken::CIntRoot(la), BasicToken::CIntRoot(ra)) => {
                let mut frac = Fraction::new(0, la.mul, mul!(ra.mul, ra.base));
                let res = factorise(mul!(la.base, ra.base), false);
                frac.num = mul!(res.outside, frac.num);
                BasicToken::new_c_root(frac.normalise()?, res.inside)
            }
            (BasicToken::CFracRoot(mut la), BasicToken::CIntRoot(ra)) => {
                la.mul.num = add!(la.mul.num, mul!(la.mul.int, la.mul.den));
                la.mul.den = mul!(mul!(la.mul.den, ra.mul), ra.base);
                la.mul.int = 0;
                let res = factorise(mul!(la.base, ra.base), false);
                la.mul.num = mul!(la.mul.num, res.outside);
                BasicToken::new_c_root(la.mul.normalise()?, res.inside)
            }
            (BasicToken::CIntRoot(la), BasicToken::CFracRoot(ra)) => {
                let mut frac = Fraction::new(
                    0,
                    mul!(la.mul, ra.mul.den),
                    mul!(add!(ra.mul.num, mul!(ra.mul.den, ra.mul.int)), ra.base),
                );
                let res = factorise(mul!(la.base, ra.base), false);
                frac.num = mul!(frac.num, res.outside);
                BasicToken::new_c_root(frac.normalise()?, res.inside)
            }
            (BasicToken::CFracRoot(mut la), BasicToken::CFracRoot(mut ra)) => {
                ra.mul.num = mul!(ra.mul.num, ra.base);
                ra.mul.int = mul!(ra.mul.int, ra.base);
                match ra.mul.normalise()? {
                    BasicToken::Integer(int) => {
                        la.mul.num = add!(la.mul.num, mul!(la.mul.int, la.mul.den));
                        la.mul.den = mul!(la.mul.den, int); // Already have multiplied by base so no need to do it again
                        la.mul.int = 0;
                        let res = factorise(mul!(la.base, ra.base), false);
                        la.mul.num = mul!(la.mul.num, res.outside);
                        BasicToken::new_c_root(la.mul.normalise()?, res.inside)
                    }
                    BasicToken::Fraction(frac) => {
                        let res = factorise(mul!(la.base, ra.base), false);
                        la.mul.int = mul!(la.mul.int, res.outside);
                        la.mul.num = mul!(la.mul.int, res.outside);
                        BasicToken::new_c_root((la.mul / frac)?, res.inside)
                    }
                    _ => unreachable!(),
                }
            }
            _ => Err(MathError::Overflow),
        }
    }
}

pub fn div(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    match (l_number, r_number) {
        (Token::Basic(BasicToken::Integer(0)), _) => Ok(Token::Basic(BasicToken::Integer(0))),
        (_, Token::Basic(BasicToken::Integer(0))) => Err(MathError::DivisionByZero),
        (Token::Pi(l_number), Token::Pi(r_number))
        | (Token::Basic(l_number), Token::Basic(r_number)) => match try_div(l_number, r_number) {
            Err(MathError::Overflow) => Ok(Token::Basic(BasicToken::Double(double_check!(
                l_number.double() / r_number.double()
            )))),
            value => Ok(Token::Basic(value?)),
        },
        (Token::Pi(l_number), Token::Basic(r_number)) => match try_div(l_number, r_number) {
            Err(MathError::Overflow) => Ok(Token::Basic(BasicToken::Double(double_check!(
                (l_number.double() * std::f64::consts::PI) / r_number.double()
            )))),
            value => Ok(Token::Pi(value?)),
        },
        (Token::Basic(l_number), Token::Pi(r_number)) => Ok(Token::Basic(BasicToken::Double(
            double_check!(l_number.double() / (r_number.double() * std::f64::consts::PI)),
        ))),
        (Token::Basic(l_number), Token::Combined(r_number)) => Ok(Token::Basic(
            BasicToken::Double(double_check!(l_number.double() / r_number.double())),
        )),
        (Token::Pi(l_number), Token::Combined(r_number)) => Ok(Token::Basic(BasicToken::Double(
            double_check!((l_number.double() * std::f64::consts::PI) / r_number.double()),
        ))),
        (Token::Combined(l_number), Token::Combined(r_number)) => Ok(Token::Basic(
            BasicToken::Double(double_check!(l_number.double() / r_number.double())),
        )),
        (Token::Combined(l_number), Token::Basic(r_number)) => {
            let mut basic = vec![];
            let mut pi = vec![];
            for basic_tok in &l_number.basic {
                basic.push(match try_div(*basic_tok, r_number) {
                    Err(MathError::Overflow) => {
                        return Ok(Token::Basic(BasicToken::Double(double_check!(
                            l_number.double() / r_number.double()
                        ))))
                    }
                    val => val?,
                })
            }
            for pi_tok in &l_number.pi {
                pi.push(match try_div(*pi_tok, r_number) {
                    Err(MathError::Overflow) => {
                        return Ok(Token::Basic(BasicToken::Double(double_check!(
                            l_number.double() / r_number.double()
                        ))))
                    }
                    val => val?,
                })
            }
            Ok(Combined { basic, pi }.normalise())
        }
        (Token::Combined(l_number), Token::Pi(r_number)) if l_number.basic.is_empty() => {
            let mut basic = vec![];
            for pi_tok in &l_number.pi {
                basic.push(match try_div(*pi_tok, r_number) {
                    Err(MathError::Overflow) => {
                        return Ok(Token::Basic(BasicToken::Double(double_check!(
                            l_number.double() / r_number.double()
                        ))))
                    }
                    val => val?,
                })
            }
            Ok(Combined { basic, pi: vec![] }.normalise())
        }
        (Token::Combined(l_number), Token::Pi(r_number)) => Ok(Token::Basic(BasicToken::Double(
            double_check!(l_number.double() / (r_number.double() * std::f64::consts::PI)),
        ))),
    }
}

/*
fn try_exp(tup: (&BasicToken, &BasicToken)) -> Result<BasicToken, MathError> {
    match tup {
        // Check if 0^0.
        (BasicToken::Integer(0), BasicToken::Integer(0)) => Err(MathError::ExponentiationError),
        (BasicToken::Double(a), BasicToken::Double(b)) if !a.is_normal() && !b.is_normal() => {
            Err(MathError::ExponentiationError)
        }
        (BasicToken::Integer(0), BasicToken::Double(a))
        | (BasicToken::Double(a), BasicToken::Integer(0))
            if !a.is_normal() =>
        {
            Err(MathError::ExponentiationError)
        }

        // Check if x^0, x^1, 0^x or 1^x
        (BasicToken::Integer(1), _) | (_, BasicToken::Integer(0)) => Ok(BasicToken::Integer(1)),
        (BasicToken::Integer(0), _) => Ok(BasicToken::Integer(0)),
        (x, BasicToken::Integer(1)) => Ok(x.clone()),

        (BasicToken::Integer(la), BasicToken::Integer(mut ra)) => {
            let negative = ra < 0;
            ra = ra.abs();
            if ra > u32::MAX as i128 {
                return Err(MathError::Overflow);
            }
            let a = none_to_err!(la.checked_pow(ra as u32));
            match negative {
                false => Ok(BasicToken::Integer(a)),
                true => Ok(BasicToken::fraction(0, 1, a)),
            }
        }
        (BasicToken::Fraction(la), BasicToken::Integer(mut ra)) => {
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
                Err(MathError::InvalidFraction) => Ok(BasicToken::Integer(res.int)),
                Err(x) => Err(x),
                _ => Ok(BasicToken::Fraction(res)),
            }
        }
        (BasicToken::Integer(la), BasicToken::Fraction(mut ra)) => {
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
                    true => Ok(BasicToken::fraction(0, 1, mul!(outside_root, outside))),
                    false => Ok(BasicToken::Integer(mul!(outside_root, outside))),
                };
            }
            if ra.den == 3 {
                match negative {
                    false => Ok(BasicToken::CIntRoot(CRoot::new(
                        mul!(outside_root, outside),
                        inside_root,
                    ))),
                    true => match inside_root.checked_pow(2) {
                        None => Err(MathError::Overflow),
                        Some(sq_inside_root) => Ok(BasicToken::c_frac_root(
                            0,
                            1,
                            mul!(mul!(outside_root, outside), inside_root),
                            sq_inside_root,
                        )),
                    },
                }
            } else {
                match negative {
                    false => Ok(BasicToken::s_int_root(
                        mul!(outside_root, outside),
                        inside_root,
                    )),
                    true => Ok(BasicToken::s_frac_root(
                        0,
                        1,
                        mul!(mul!(outside_root, outside), inside_root),
                        inside_root,
                    )),
                }
            }
        }
        (BasicToken::Fraction(la), BasicToken::Fraction(mut ra)) => {
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
                    Err(MathError::InvalidFraction) => Ok(BasicToken::Integer(res.int)),
                    Err(x) => Err(x),
                    _ => Ok(BasicToken::Fraction(res)),
                };
            }
            let mut res = Fraction::new(
                0,
                mul!(outside_root_num, outside_num),
                mul!(mul!(outside_root_den, outside_den), inside_root_den),
            );
            if ra.den == 3 {
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(BasicToken::c_int_root(
                        res.int,
                        mul!(
                            inside_root_num,
                            none_to_err!(inside_root_den.checked_pow(2))
                        ),
                    )),
                    Err(x) => Err(x),
                    _ => Ok(BasicToken::c_fraction_root(
                        res,
                        mul!(
                            inside_root_num,
                            none_to_err!(inside_root_den.checked_pow(2))
                        ),
                    )),
                }
            } else {
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(BasicToken::SIntRoot(SRoot::new(
                        res.int,
                        mul!(inside_root_num, inside_root_den),
                    ))),
                    Err(x) => Err(x),
                    _ => Ok(BasicToken::s_fraction_root(
                        res,
                        mul!(inside_root_num, inside_root_den),
                    )),
                }
            }
        }

        (BasicToken::SIntRoot(la), BasicToken::Integer(mut ra)) => {
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
                    true => Ok(BasicToken::fraction(0, 1, mul)),
                    false => Ok(BasicToken::Integer(mul)),
                }
            } else {
                match negative {
                    true => Ok(BasicToken::s_frac_root(0, 1, mul, la.base)),
                    false => Ok(BasicToken::s_int_root(mul, la.base)),
                }
            }
        }

        (BasicToken::CIntRoot(la), BasicToken::Integer(mut ra)) => {
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
                    true => Ok(BasicToken::fraction(0, 1, mul)),
                    false => Ok(BasicToken::Integer(mul)),
                }
            } else {
                let base = none_to_err!(la.base.checked_pow((ra % 3) as u32));
                match negative {
                    true => Ok(BasicToken::c_frac_root(0, 1, mul, base)),
                    false => Ok(BasicToken::c_int_root(mul, base)),
                }
            }
        }

        (BasicToken::SFracRoot(la), BasicToken::Integer(mut ra)) => {
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
                    Err(MathError::InvalidFraction) => Ok(BasicToken::Integer(res.int)),
                    Err(x) => Err(x),
                    _ => Ok(BasicToken::Fraction(res)),
                }
            } else {
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(BasicToken::c_int_root(res.int, la.base)),
                    Err(x) => Err(x),
                    _ => Ok(BasicToken::c_fraction_root(res, la.base)),
                }
            }
        }
        (BasicToken::CFracRoot(la), BasicToken::Integer(mut ra)) => {
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
                    Err(MathError::InvalidFraction) => Ok(BasicToken::Integer(res.int)),
                    Err(x) => Err(x),
                    _ => Ok(BasicToken::Fraction(res)),
                }
            } else {
                let base = none_to_err!(la.base.checked_pow((ra % 3) as u32));
                match res.normalise() {
                    Err(MathError::InvalidFraction) => Ok(BasicToken::c_int_root(res.int, base)),
                    Err(x) => Err(x),
                    _ => Ok(BasicToken::c_fraction_root(res, base)),
                }
            }
        }
        _ => Err(MathError::Overflow),
    }
}

pub fn exp(l_number: Token, r_number: Token) -> Result<Token, MathError> {
    match try_exp((&l_number, &r_number)) {
        Err(MathError::Overflow) => Ok(BasicToken::Double(double_check!(
            (l_number.double()).powf(r_number.double())
        ))),
        value => value,
    }
}
 */
