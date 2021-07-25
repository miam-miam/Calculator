#![allow(illegal_floating_point_literal_pattern)]
#[macro_use]
pub mod macros;
pub mod expression;
pub mod my_math;
pub mod number;
pub mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenise() {
        let mut expr = expression::Expression::new("456123+5646546 - 46546 /1.0*9-0.01+(-9)");
        assert_eq!(expr.tokenise(), types::MathError::None);
        assert_eq!(
            expr.infix_token,
            vec![
                types::Token::Integer(456123),
                types::Token::Plus,
                types::Token::Integer(5646546),
                types::Token::Minus,
                types::Token::Integer(46546),
                types::Token::Divide,
                types::Token::Integer(1),
                types::Token::Multiply,
                types::Token::Integer(9),
                types::Token::Minus,
                types::Token::Fraction(types::Fraction {
                    int: 0,
                    num: 1,
                    den: 100
                }),
                types::Token::Plus,
                types::Token::LBracket,
                types::Token::Minus,
                types::Token::Integer(9),
                types::Token::RBracket,
            ]
        );
        let mut expr1 = expression::Expression::new("56+9");
        assert_eq!(expr1.tokenise(), types::MathError::None);
        assert_eq!(
            expr1.infix_token,
            vec![
                types::Token::Integer(56),
                types::Token::Plus,
                types::Token::Integer(9)
            ]
        );
    }

    #[test]
    fn normalise() {
        let mut fr = types::Fraction {
            int: 1,
            num: 5,
            den: 2,
        };
        assert_eq!(fr.normalise(), Ok(()));
        assert_eq!(
            fr,
            types::Fraction {
                int: 3,
                num: 1,
                den: 2,
            }
        );
    }

    #[test]
    fn add() {
        let mut expr = expression::Expression::new("5+1-5.1+2-5.1+9.5-5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001+9-5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001+5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001-7.1+5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
        assert_eq!(expr.tokenise(), types::MathError::None);
        assert_eq!(
            Ok(types::Token::Integer(6)),
            number::add(expr.infix_token[0], expr.infix_token[2])
        );
        assert_eq!(
            Ok(types::Token::Fraction(types::Fraction::new(7, 1, 10))),
            number::add(expr.infix_token[4], expr.infix_token[6])
        );
        assert_eq!(
            Ok(types::Token::Fraction(types::Fraction::new(14, 3, 5))),
            number::add(expr.infix_token[8], expr.infix_token[10])
        );
        assert_eq!(
            Ok(types::Token::Double(14_f64)),
            number::add(expr.infix_token[12], expr.infix_token[14])
        );
        assert_eq!(
            Ok(types::Token::Double(10_f64)),
            number::add(expr.infix_token[16], expr.infix_token[18])
        );
        assert_eq!(
            Ok(types::Token::Double(12.1_f64)),
            number::add(expr.infix_token[20], expr.infix_token[22])
        );
    }

    #[test]
    fn sub() {
        let mut expr = expression::Expression::new("5-1+5.1-2+5.1-9.5+5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001-9+5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001-5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001+7.1-5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
        assert_eq!(expr.tokenise(), types::MathError::None);
        assert_eq!(
            Ok(types::Token::Integer(4)),
            number::sub(expr.infix_token[0], expr.infix_token[2])
        );
        assert_eq!(
            Ok(types::Token::Fraction(types::Fraction::new(3, 1, 10))),
            number::sub(expr.infix_token[4], expr.infix_token[6])
        );
        assert_eq!(
            Ok(types::Token::Fraction(types::Fraction::new(-4, -2, 5))),
            number::sub(expr.infix_token[8], expr.infix_token[10])
        );
        assert_eq!(
            Ok(types::Token::Double(-4_f64)),
            number::sub(expr.infix_token[12], expr.infix_token[14])
        );
        assert_eq!(
            Ok(types::Token::Double(0_f64)),
            number::sub(expr.infix_token[16], expr.infix_token[18])
        );
        assert_eq!(
            Ok(types::Token::Double(2.0999999999999996_f64)),
            number::sub(expr.infix_token[20], expr.infix_token[22])
        );
    }

    #[test]
    fn mul() {
        let mut expr = expression::Expression::new("5*1+5.1*2+5.1*9.5+5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001*9+5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001*5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001+7.1*5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
        assert_eq!(expr.tokenise(), types::MathError::None);
        assert_eq!(
            Ok(types::Token::Integer(5)),
            number::mul(expr.infix_token[0], expr.infix_token[2])
        );
        assert_eq!(
            Ok(types::Token::Fraction(types::Fraction::new(10, 1, 5))),
            number::mul(expr.infix_token[4], expr.infix_token[6])
        );
        assert_eq!(
            Ok(types::Token::Fraction(types::Fraction::new(48, 9, 20))),
            number::mul(expr.infix_token[8], expr.infix_token[10])
        );
        assert_eq!(
            Ok(types::Token::Double(45_f64)),
            number::mul(expr.infix_token[12], expr.infix_token[14])
        );
        assert_eq!(
            Ok(types::Token::Double(25_f64)),
            number::mul(expr.infix_token[16], expr.infix_token[18])
        );
        assert_eq!(
            Ok(types::Token::Double(35.5_f64)),
            number::mul(expr.infix_token[20], expr.infix_token[22])
        );
    }

    #[test]
    fn div() {
        let mut expr = expression::Expression::new("5/1+5.1/2+5.1/9.5+5/9+5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001/5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001+7.1/0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
        assert_eq!(expr.tokenise(), types::MathError::None);
        assert_eq!(
            Ok(types::Token::Integer(5)),
            number::div(expr.infix_token[0], expr.infix_token[2])
        );
        assert_eq!(
            Ok(types::Token::Fraction(types::Fraction::new(2, 11, 20))),
            number::div(expr.infix_token[4], expr.infix_token[6])
        );
        assert_eq!(
            Ok(types::Token::Fraction(types::Fraction::new(0, 51, 95))),
            number::div(expr.infix_token[8], expr.infix_token[10])
        );
        assert_eq!(
            Ok(types::Token::Fraction(types::Fraction::new(0, 5, 9))),
            number::div(expr.infix_token[12], expr.infix_token[14])
        );
        assert_eq!(
            Ok(types::Token::Double(1_f64)),
            number::div(expr.infix_token[16], expr.infix_token[18])
        );
        assert_eq!(
            Err(types::MathError::DivisionByZero),
            number::div(expr.infix_token[20], expr.infix_token[22])
        );
    }

    #[test]
    fn evaluate() {
        let mut expr = expression::Expression::new("9.856/8.7+9*(5.1+0.1)");
        assert_eq!(expr.tokenise(), types::MathError::None);
        assert_eq!(
            expr.calculate(),
            Ok(types::Token::Fraction(types::Fraction::new(47, 2029, 2175)))
        );
        let mut expr = expression::Expression::new("(8/((5-5)/(9.1*4)))");
        assert_eq!(expr.tokenise(), types::MathError::None);
        assert_eq!(expr.calculate(), Err(types::MathError::DivisionByZero));
        let mut expr = expression::Expression::new("5/1+5.1/2+5.1/9.5+5/9+5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001/5.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
        assert_eq!(expr.tokenise(), types::MathError::None);
        assert_eq!(
            expr.calculate(),
            Ok(types::Token::Double(9.642397660818713_f64))
        );
        let mut expr = expression::Expression::new("5/1+5.1/2+5.1/9.5+5/9+8.4/7");
        assert_eq!(expr.tokenise(), types::MathError::None);
        assert_eq!(
            expr.calculate(),
            Ok(types::Token::Fraction(types::Fraction::new(9, 2881, 3420)))
        );
    }
}
