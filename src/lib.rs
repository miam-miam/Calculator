pub mod expression;
pub mod my_math;
pub mod number;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenise() {
        let mut expr = expression::Expression::new("456123+5646546 - 46546 /1.0*9-0.01+(-9)");
        assert_eq!(expr.tokenise(), my_math::MathError::None);
        assert_eq!(
            expr.infix_token,
            vec![
                my_math::Token::Integer(456123),
                my_math::Token::Plus,
                my_math::Token::Integer(5646546),
                my_math::Token::Minus,
                my_math::Token::Integer(46546),
                my_math::Token::Divide,
                my_math::Token::Integer(1),
                my_math::Token::Multiply,
                my_math::Token::Integer(9),
                my_math::Token::Minus,
                my_math::Token::Fraction(number::Fraction {
                    int: 0,
                    num: 1,
                    den: 100
                }),
                my_math::Token::Plus,
                my_math::Token::LBracket,
                my_math::Token::Minus,
                my_math::Token::Integer(9),
                my_math::Token::RBracket,
            ]
        )
    }
    #[test]
    fn normalise() {
        let mut fr = number::Fraction {
            int: 1,
            num: 5,
            den: 2,
        };
        fr.normalise();
        assert_eq!(
            number::Fraction {
                int: 3,
                num: 1,
                den: 2,
            },
            fr
        );
    }
}
