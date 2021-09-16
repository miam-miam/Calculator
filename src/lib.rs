#![allow(illegal_floating_point_literal_pattern)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pest_derive;
#[macro_use]
pub mod macros;
pub mod expression;
pub mod my_math;
pub mod number;
pub mod trig_number;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::expression::{eval, Expression, Parser, Rule};
    use crate::types::{BasicToken, Token};
    use BasicToken::*;
    use Rule::calculation;
    use Token::*;

    #[test]
    fn number_parse() {
        assert_eq!(
            eval(
                Expression::parse(calculation, "170141183460469231731687303715884105727").unwrap()
            ),
            Ok(Basic(Integer(i128::MAX)))
        );
        assert_eq!(
            eval(
                Expression::parse(calculation, "170141183460469231731687303715884105728").unwrap()
            ),
            Ok(Basic(BasicToken::Double(i128::MAX as f64 + 1_f64)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "+5").unwrap()),
            Ok(Basic(Integer(5)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "-5").unwrap()),
            Ok(Basic(Integer(-5)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "5.5").unwrap()),
            Ok(Basic(BasicToken::fraction(5, 1, 2)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "-5.5").unwrap()),
            Ok(Basic(BasicToken::fraction(-5, -1, 2)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, ".5").unwrap()),
            Ok(Basic(BasicToken::fraction(0, 1, 2)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "-5.0").unwrap()),
            Ok(Basic(Integer(-5)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "1.0").unwrap()),
            Ok(Basic(Integer(1)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "0pi").unwrap()),
            Ok(Basic(Integer(0)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "-0.5").unwrap()),
            Ok(Basic(BasicToken::fraction(0, -1, 2)))
        );
        assert!(Expression::parse(calculation, "-.5").is_err());
        assert!(Expression::parse(calculation, "+.5").is_err());
    }

    #[test]
    fn e_parse() {
        assert_eq!(
            eval(
                Expression::parse(calculation, "170141183460469231731687303715884105727e0")
                    .unwrap()
            ),
            Ok(Basic(Integer(i128::MAX)))
        );
        assert_eq!(
            eval(
                Expression::parse(calculation, "170141183460469231731687303715884105728e2")
                    .unwrap()
            ),
            Ok(Basic(BasicToken::Double(
                (i128::MAX as f64 + 1_f64) * 100_f64
            )))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "+5e-1").unwrap()),
            Ok(Basic(BasicToken::fraction(0, 1, 2)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "-5e-1").unwrap()),
            Ok(Basic(BasicToken::fraction(0, -1, 2)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "5.5e-1").unwrap()),
            Ok(Basic(BasicToken::fraction(0, 11, 20)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "-5.5e+1").unwrap()),
            Ok(Basic(Integer(-55)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "-595.524e-1").unwrap()),
            Ok(Basic(BasicToken::fraction(-59, -1381, 2500)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, ".5e0").unwrap()),
            Ok(Basic(BasicToken::fraction(0, 1, 2)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "-5.0e1").unwrap()),
            Ok(Basic(Integer(-50)))
        );
        assert_eq!(
            eval(Expression::parse(calculation, "1.0e2").unwrap()),
            Ok(Basic(Integer(100)))
        );
        assert!(Expression::parse(calculation, "-.5e5").is_err());
        assert!(Expression::parse(calculation, "+.5e5").is_err());
    }

    #[test]
    fn zero_tests() {
        assert_eq!(
            eval(Expression::parse(calculation, "-5+5pi+5-5pi").unwrap()),
            Ok(Basic(Integer(0)))
        );
    }
}
