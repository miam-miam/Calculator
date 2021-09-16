use crate::types::{BasicToken, Fraction, MathError, SRoot, Token};

pub fn sin(number: Token) -> Result<Token, MathError> {
    match number {
        Token::Basic(BasicToken::Integer(0)) | Token::Pi(BasicToken::Integer(_)) => {
            Ok(Token::Basic(BasicToken::Integer(0)))
        }
        Token::Pi(BasicToken::Fraction(mut frac)) => {
            let mut negative = abs!(frac.int) % 2 == 1;
            if frac.num < 0 {
                frac.num = abs!(frac.num);
                negative = !negative;
            }
            if frac.num > frac.den / 2 {
                frac.num = frac.den - frac.num;
            }
            Ok(match (frac.num, frac.den, negative) {
                (1, 12, false) => Token::combined(
                    vec![
                        BasicToken::s_frac_root(0, 1, 4, 6),
                        BasicToken::s_frac_root(0, -1, 4, 2),
                    ],
                    vec![],
                ),
                (1, 10, false) => Token::combined(
                    vec![
                        BasicToken::s_frac_root(0, 1, 4, 5),
                        BasicToken::fraction(0, -1, 4),
                    ],
                    vec![],
                ),
                (1, 6, false) => Token::Basic(BasicToken::fraction(0, 1, 2)),
                (1, 4, false) => Token::Basic(BasicToken::s_frac_root(0, 1, 2, 2)),
                (3, 10, false) => Token::combined(
                    vec![
                        BasicToken::s_frac_root(0, 1, 4, 5),
                        BasicToken::fraction(0, 1, 4),
                    ],
                    vec![],
                ),
                (1, 3, false) => Token::Basic(BasicToken::s_frac_root(0, 1, 2, 3)),
                (5, 12, false) => Token::combined(
                    vec![
                        BasicToken::s_frac_root(0, 1, 4, 6),
                        BasicToken::s_frac_root(0, 1, 4, 2),
                    ],
                    vec![],
                ),
                (1, 2, false) => Token::Basic(BasicToken::Integer(1)),
                (1, 12, true) => Token::combined(
                    vec![
                        BasicToken::s_frac_root(0, -1, 4, 6),
                        BasicToken::s_frac_root(0, 1, 4, 2),
                    ],
                    vec![],
                ),
                (1, 10, true) => Token::combined(
                    vec![
                        BasicToken::s_frac_root(0, -1, 4, 5),
                        BasicToken::fraction(0, 1, 4),
                    ],
                    vec![],
                ),
                (1, 6, true) => Token::Basic(BasicToken::fraction(0, -1, 2)),
                (1, 4, true) => Token::Basic(BasicToken::s_frac_root(0, -1, 2, 2)),
                (3, 10, true) => Token::combined(
                    vec![
                        BasicToken::s_frac_root(0, -1, 4, 5),
                        BasicToken::fraction(0, -1, 4),
                    ],
                    vec![],
                ),
                (1, 3, true) => Token::Basic(BasicToken::s_frac_root(0, -1, 2, 3)),
                (5, 12, true) => Token::combined(
                    vec![
                        BasicToken::s_frac_root(0, -1, 4, 6),
                        BasicToken::s_frac_root(0, -1, 4, 2),
                    ],
                    vec![],
                ),
                (1, 2, true) => Token::Basic(BasicToken::Integer(-1)),
                (num, den, false) => Token::Basic(BasicToken::Double(
                    ((num as f64 / den as f64) * std::f64::consts::PI).sin(),
                )),
                (num, den, true) => Token::Basic(BasicToken::Double(
                    (-(num as f64 / den as f64) * std::f64::consts::PI).sin(),
                )),
            })
        }
        val => Ok(Token::Basic(BasicToken::Double(
            trig_check!(val.double()).sin(),
        ))),
    }
}

pub fn asin(number: Token) -> Result<Token, MathError> {
    use self::Fraction as FractionStruct;
    use BasicToken::*;
    use Token::*;
    match number {
        Basic(Integer(0)) => Ok(Basic(Integer(0))),
        Combined(val) => match val.basic.vec[..] {
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 6,
                }),
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 2,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 1, 12))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 5,
                }),
                Fraction(FractionStruct {
                    int: 0,
                    num: -1,
                    den: 4,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 1, 10))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 5,
                }),
                Fraction(FractionStruct {
                    int: 0,
                    num: 1,
                    den: 4,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 3, 10))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 6,
                }),
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 2,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 5, 12))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 6,
                }),
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 2,
                })
            ) => Ok(Pi(BasicToken::fraction(0, -1, 12))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 5,
                }),
                Fraction(FractionStruct {
                    int: 0,
                    num: 1,
                    den: 4,
                })
            ) => Ok(Pi(BasicToken::fraction(0, -1, 10))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 5,
                }),
                Fraction(FractionStruct {
                    int: 0,
                    num: -1,
                    den: 4,
                })
            ) => Ok(Pi(BasicToken::fraction(0, -3, 10))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 6,
                }),
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 2,
                })
            ) => Ok(Pi(BasicToken::fraction(0, -5, 12))),
            _ => {
                let double = val.double();
                match !(-1.0..=1.0).contains(&double) {
                    true => Err(MathError::DomainError),
                    false => Ok(Basic(Double(double.asin()))),
                }
            }
        },
        Basic(Fraction(FractionStruct {
            int: 0,
            num: 1,
            den: 2,
        })) => Ok(Pi(BasicToken::fraction(0, 1, 6))),
        Basic(SFracRoot(SRoot {
            mul:
                FractionStruct {
                    int: 0,
                    num: 1,
                    den: 2,
                },
            base: 2,
        })) => Ok(Pi(BasicToken::fraction(0, 1, 4))),
        Basic(SFracRoot(SRoot {
            mul:
                FractionStruct {
                    int: 0,
                    num: 1,
                    den: 2,
                },
            base: 3,
        })) => Ok(Pi(BasicToken::fraction(0, 1, 3))),
        Basic(Integer(1)) => Ok(Pi(BasicToken::fraction(0, 1, 2))),
        Basic(Fraction(FractionStruct {
            int: 0,
            num: -1,
            den: 2,
        })) => Ok(Pi(BasicToken::fraction(0, -1, 6))),
        Basic(SFracRoot(SRoot {
            mul:
                FractionStruct {
                    int: 0,
                    num: -1,
                    den: 2,
                },
            base: 2,
        })) => Ok(Pi(BasicToken::fraction(0, -1, 4))),

        Basic(SFracRoot(SRoot {
            mul:
                FractionStruct {
                    int: 0,
                    num: -1,
                    den: 2,
                },
            base: 3,
        })) => Ok(Pi(BasicToken::fraction(0, -1, 3))),
        Basic(Integer(-1)) => Ok(Pi(BasicToken::fraction(0, -1, 2))),
        val => {
            let double = val.double();
            match !(-1.0..=1.0).contains(&double) {
                true => Err(MathError::DomainError),
                false => Ok(Basic(Double(double.asin()))),
            }
        }
    }
}

pub fn cos(number: Token) -> Result<Token, MathError> {
    match number {
        Token::Basic(BasicToken::Integer(0)) => Ok(Token::Basic(BasicToken::Integer(1))),
        Token::Pi(BasicToken::Integer(x)) => match x % 2 == 1 {
            true => Ok(Token::Basic(BasicToken::Integer(-1))),
            false => Ok(Token::Basic(BasicToken::Integer(1))),
        },
        Token::Pi(BasicToken::Fraction(mut frac)) => {
            frac.num = abs!(frac.num);
            if frac.num > frac.den / 2 {
                frac.num = frac.den - frac.num;
            }
            Ok(
                match (
                    frac.num,
                    frac.den,
                    (abs!(frac.int) % 2 == 1) ^ (frac.num > frac.den / 2),
                ) {
                    (5, 12, false) => Token::combined(
                        vec![
                            BasicToken::s_frac_root(0, 1, 4, 6),
                            BasicToken::s_frac_root(0, -1, 4, 2),
                        ],
                        vec![],
                    ),
                    (2, 5, false) => Token::combined(
                        vec![
                            BasicToken::s_frac_root(0, 1, 4, 5),
                            BasicToken::fraction(0, -1, 4),
                        ],
                        vec![],
                    ),
                    (1, 3, false) => Token::Basic(BasicToken::fraction(0, 1, 2)),
                    (1, 4, false) => Token::Basic(BasicToken::s_frac_root(0, 1, 2, 2)),
                    (1, 5, false) => Token::combined(
                        vec![
                            BasicToken::s_frac_root(0, 1, 4, 5),
                            BasicToken::fraction(0, 1, 4),
                        ],
                        vec![],
                    ),
                    (1, 6, false) => Token::Basic(BasicToken::s_frac_root(0, 1, 2, 3)),
                    (1, 12, false) => Token::combined(
                        vec![
                            BasicToken::s_frac_root(0, 1, 4, 6),
                            BasicToken::s_frac_root(0, 1, 4, 2),
                        ],
                        vec![],
                    ),
                    (5, 12, true) => Token::combined(
                        vec![
                            BasicToken::s_frac_root(0, -1, 4, 6),
                            BasicToken::s_frac_root(0, 1, 4, 2),
                        ],
                        vec![],
                    ),
                    (2, 5, true) => Token::combined(
                        vec![
                            BasicToken::s_frac_root(0, -1, 4, 5),
                            BasicToken::fraction(0, 1, 4),
                        ],
                        vec![],
                    ),
                    (1, 3, true) => Token::Basic(BasicToken::fraction(0, -1, 2)),
                    (1, 4, true) => Token::Basic(BasicToken::s_frac_root(0, -1, 2, 2)),
                    (1, 5, true) => Token::combined(
                        vec![
                            BasicToken::s_frac_root(0, -1, 4, 5),
                            BasicToken::fraction(0, -1, 4),
                        ],
                        vec![],
                    ),
                    (1, 6, true) => Token::Basic(BasicToken::s_frac_root(0, -1, 2, 3)),
                    (1, 12, true) => Token::combined(
                        vec![
                            BasicToken::s_frac_root(0, -1, 4, 6),
                            BasicToken::s_frac_root(0, -1, 4, 2),
                        ],
                        vec![],
                    ),
                    (1, 2, _) => Token::Basic(BasicToken::Integer(0)),
                    (num, den, true) => Token::Basic(BasicToken::Double(
                        -((num as f64 / den as f64) * std::f64::consts::PI).cos(),
                    )),
                    (num, den, false) => Token::Basic(BasicToken::Double(
                        ((num as f64 / den as f64) * std::f64::consts::PI).cos(),
                    )),
                },
            )
        }
        val => Ok(Token::Basic(BasicToken::Double(
            trig_check!(val.double()).cos(),
        ))),
    }
}

pub fn acos(number: Token) -> Result<Token, MathError> {
    use self::Fraction as FractionStruct;
    use BasicToken::*;
    use Token::*;
    match number {
        Combined(val) => match val.basic.vec[..] {
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 6,
                }),
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 2,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 5, 12))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 5,
                }),
                Fraction(FractionStruct {
                    int: 0,
                    num: -1,
                    den: 4,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 2, 5))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 5,
                }),
                Fraction(FractionStruct {
                    int: 0,
                    num: 1,
                    den: 4,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 1, 5))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 6,
                }),
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 2,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 1, 12))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 6,
                }),
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: 1,
                        den: 4,
                    },
                    base: 2,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 7, 12))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 5,
                }),
                Fraction(FractionStruct {
                    int: 0,
                    num: 1,
                    den: 4,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 3, 5))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 5,
                }),
                Fraction(FractionStruct {
                    int: 0,
                    num: -1,
                    den: 4,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 4, 5))),
            combined!(
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 6,
                }),
                SFracRoot(SRoot {
                    mul: FractionStruct {
                        int: 0,
                        num: -1,
                        den: 4,
                    },
                    base: 2,
                })
            ) => Ok(Pi(BasicToken::fraction(0, 11, 12))),

            _ => {
                let double = val.double();
                match !(-1.0..=1.0).contains(&double) {
                    true => Err(MathError::DomainError),
                    false => Ok(Basic(Double(double.acos()))),
                }
            }
        },
        Basic(Integer(1)) => Ok(Basic(Integer(0))),
        Basic(Integer(-1)) => Ok(Pi(Integer(1))),
        Basic(Integer(0)) => Ok(Pi(Fraction(FractionStruct {
            int: 0,
            num: 1,
            den: 2,
        }))),
        Basic(Fraction(FractionStruct {
            int: 0,
            num: 1,
            den: 2,
        })) => Ok(Pi(BasicToken::fraction(0, 1, 3))),
        Basic(SFracRoot(SRoot {
            mul:
                FractionStruct {
                    int: 0,
                    num: 1,
                    den: 2,
                },
            base: 2,
        })) => Ok(Pi(BasicToken::fraction(0, 1, 4))),
        Basic(SFracRoot(SRoot {
            mul:
                FractionStruct {
                    int: 0,
                    num: 1,
                    den: 2,
                },
            base: 3,
        })) => Ok(Pi(BasicToken::fraction(0, 1, 6))),
        Basic(Fraction(FractionStruct {
            int: 0,
            num: -1,
            den: 2,
        })) => Ok(Pi(BasicToken::fraction(0, 2, 3))),
        Basic(SFracRoot(SRoot {
            mul:
                FractionStruct {
                    int: 0,
                    num: -1,
                    den: 2,
                },
            base: 2,
        })) => Ok(Pi(BasicToken::fraction(0, 3, 4))),
        Basic(SFracRoot(SRoot {
            mul:
                FractionStruct {
                    int: 0,
                    num: -1,
                    den: 2,
                },
            base: 3,
        })) => Ok(Pi(BasicToken::fraction(0, 5, 6))),
        val => {
            let double = val.double();
            match !(-1.0..=1.0).contains(&double) {
                true => Err(MathError::DomainError),
                false => Ok(Basic(Double(double.acos()))),
            }
        }
    }
}

pub fn tan(number: Token) -> Result<Token, MathError> {
    match number {
        Token::Basic(BasicToken::Integer(0)) | Token::Pi(BasicToken::Integer(_)) => {
            Ok(Token::Basic(BasicToken::Integer(0)))
        }
        Token::Pi(BasicToken::Fraction(mut frac)) => {
            let mut negative = frac.num < 0;
            frac.num = abs!(frac.num);
            frac.int = abs!(frac.int);
            if frac.num > frac.den / 2 {
                frac.num = frac.den - frac.num;
                negative = !negative;
            }
            Ok(match (frac.num, frac.den, negative) {
                (1, 24, false) => Token::combined(
                    vec![
                        BasicToken::s_int_root(1, 6),
                        BasicToken::s_int_root(-1, 3),
                        BasicToken::s_int_root(1, 2),
                        BasicToken::Integer(-2),
                    ],
                    vec![],
                ),
                (1, 12, false) => Token::combined(
                    vec![BasicToken::Integer(2), BasicToken::s_int_root(-1, 3)],
                    vec![],
                ),
                (1, 8, false) => Token::combined(
                    vec![BasicToken::s_int_root(1, 2), BasicToken::Integer(-1)],
                    vec![],
                ),
                (1, 6, false) => Token::Basic(BasicToken::s_frac_root(0, 1, 3, 3)),
                (5, 24, false) => Token::combined(
                    vec![
                        BasicToken::s_int_root(1, 6),
                        BasicToken::s_int_root(1, 3),
                        BasicToken::s_int_root(-1, 2),
                        BasicToken::Integer(-2),
                    ],
                    vec![],
                ),
                (1, 4, false) => Token::Basic(BasicToken::Integer(1)),
                (7, 24, false) => Token::combined(
                    vec![
                        BasicToken::s_int_root(1, 6),
                        BasicToken::s_int_root(-1, 3),
                        BasicToken::s_int_root(-1, 2),
                        BasicToken::Integer(2),
                    ],
                    vec![],
                ),
                (1, 3, false) => Token::Basic(BasicToken::s_int_root(1, 3)),
                (3, 8, false) => Token::combined(
                    vec![BasicToken::Integer(1), BasicToken::s_int_root(1, 2)],
                    vec![],
                ),
                (5, 12, false) => Token::combined(
                    vec![BasicToken::Integer(2), BasicToken::s_int_root(1, 3)],
                    vec![],
                ),
                (11, 24, false) => Token::combined(
                    vec![
                        BasicToken::s_int_root(1, 6),
                        BasicToken::s_int_root(1, 3),
                        BasicToken::s_int_root(1, 2),
                        BasicToken::Integer(2),
                    ],
                    vec![],
                ),
                (1, 24, true) => Token::combined(
                    vec![
                        BasicToken::s_int_root(-1, 6),
                        BasicToken::s_int_root(1, 3),
                        BasicToken::s_int_root(-1, 2),
                        BasicToken::Integer(2),
                    ],
                    vec![],
                ),
                (1, 12, true) => Token::combined(
                    vec![BasicToken::Integer(-2), BasicToken::s_int_root(1, 3)],
                    vec![],
                ),
                (1, 8, true) => Token::combined(
                    vec![BasicToken::s_int_root(-1, 2), BasicToken::Integer(1)],
                    vec![],
                ),
                (1, 6, true) => Token::Basic(BasicToken::s_frac_root(0, -1, 3, 3)),
                (5, 24, true) => Token::combined(
                    vec![
                        BasicToken::s_int_root(-1, 6),
                        BasicToken::s_int_root(-1, 3),
                        BasicToken::s_int_root(1, 2),
                        BasicToken::Integer(2),
                    ],
                    vec![],
                ),
                (1, 4, true) => Token::Basic(BasicToken::Integer(-1)),
                (7, 24, true) => Token::combined(
                    vec![
                        BasicToken::s_int_root(-1, 6),
                        BasicToken::s_int_root(1, 3),
                        BasicToken::s_int_root(1, 2),
                        BasicToken::Integer(-2),
                    ],
                    vec![],
                ),
                (1, 3, true) => Token::Basic(BasicToken::s_int_root(-1, 3)),
                (3, 8, true) => Token::combined(
                    vec![BasicToken::Integer(-1), BasicToken::s_int_root(-1, 2)],
                    vec![],
                ),
                (5, 12, true) => Token::combined(
                    vec![BasicToken::Integer(-2), BasicToken::s_int_root(-1, 3)],
                    vec![],
                ),
                (11, 24, true) => Token::combined(
                    vec![
                        BasicToken::s_int_root(-1, 6),
                        BasicToken::s_int_root(-1, 3),
                        BasicToken::s_int_root(-1, 2),
                        BasicToken::Integer(-2),
                    ],
                    vec![],
                ),
                (1, 2, _) => return Err(MathError::TangentError),
                (num, den, true) => Token::Basic(BasicToken::Double(double_check!(-((num
                    as f64
                    / den as f64)
                    * std::f64::consts::PI)
                    .tan()))),
                (num, den, false) => Token::Basic(BasicToken::Double(double_check!(((num
                    as f64
                    / den as f64)
                    * std::f64::consts::PI)
                    .tan()))),
            })
        }
        val => Ok(Token::Basic(BasicToken::Double(
            trig_check!(val.double()).tan(),
        ))),
    }
}

pub fn atan(number: Token) -> Result<Token, MathError> {
    use self::Fraction as FractionStruct;
    use BasicToken::*;
    use Token::*;
    match number {
        Combined(val) => match val.basic.vec[..] {
            combined!(
                SIntRoot(SRoot { mul: 1, base: 6 }),
                SIntRoot(SRoot { mul: -1, base: 3 }),
                SIntRoot(SRoot { mul: 1, base: 2 }),
                Integer(-2)
            ) => Ok(Pi(BasicToken::fraction(0, 1, 24))),
            combined!(Integer(2), SIntRoot(SRoot { mul: -1, base: 3 })) => {
                Ok(Pi(BasicToken::fraction(0, 1, 12)))
            }
            combined!(SIntRoot(SRoot { mul: 1, base: 2 }), Integer(-1)) => {
                Ok(Pi(BasicToken::fraction(0, 1, 8)))
            }
            combined!(
                SIntRoot(SRoot { mul: 1, base: 6 }),
                SIntRoot(SRoot { mul: 1, base: 3 }),
                SIntRoot(SRoot { mul: -1, base: 2 }),
                Integer(-2)
            ) => Ok(Pi(BasicToken::fraction(0, 5, 24))),
            combined!(
                SIntRoot(SRoot { mul: 1, base: 6 }),
                SIntRoot(SRoot { mul: -1, base: 3 }),
                SIntRoot(SRoot { mul: -1, base: 2 }),
                Integer(2)
            ) => Ok(Pi(BasicToken::fraction(0, 7, 24))),
            combined!(Integer(1), SIntRoot(SRoot { mul: 1, base: 2 })) => {
                Ok(Pi(BasicToken::fraction(0, 3, 8)))
            }
            combined!(Integer(2), SIntRoot(SRoot { mul: 1, base: 3 })) => {
                Ok(Pi(BasicToken::fraction(0, 5, 12)))
            }
            combined!(
                SIntRoot(SRoot { mul: 1, base: 6 }),
                SIntRoot(SRoot { mul: 1, base: 3 }),
                SIntRoot(SRoot { mul: 1, base: 2 }),
                Integer(2)
            ) => Ok(Pi(BasicToken::fraction(0, 11, 24))),
            _ => {
                let double = val.double();
                match !(-1.0..=1.0).contains(&double) {
                    true => Err(MathError::DomainError),
                    false => Ok(Basic(Double(double.atan()))),
                }
            }
        },
        Basic(Integer(0)) => Ok(Basic(Integer(0))),
        Basic(Integer(1)) => Ok(Pi(BasicToken::fraction(0, 1, 4))),
        Basic(SFracRoot(SRoot {
            mul:
                FractionStruct {
                    int: 0,
                    num: 1,
                    den: 3,
                },
            base: 3,
        })) => Ok(Pi(BasicToken::fraction(0, 1, 6))),
        Basic(SIntRoot(SRoot { mul: 1, base: 3 })) => Ok(Pi(BasicToken::fraction(0, 1, 3))),
        val => Ok(Basic(Double(val.double().atan()))),
    }
}
