macro_rules! mul {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_mul($rhs) {Some(i) => i, None => {return Err(MathError::Overflow);}});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_mul($rhs) {Some(i) => i, None => {return $error;}});
    }
macro_rules! add {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_add($rhs) {Some(i) => {i}, None =>  {return Err(MathError::Overflow);}});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_add($rhs) {Some(i) => i, None => {return $error;}});
    }
macro_rules! sub {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_sub($rhs) {Some(i) => i, None => {return Err(MathError::Overflow);}});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_sub($rhs) {Some(i) => i, None => {return $error;}});
    }
macro_rules! div {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_div($rhs) {Some(i) => i, None => {return Err(MathError::Overflow);}});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_div($rhs) {Some(i) => i, None => {return $error;}});
    }
macro_rules! precedence {
    {$token:expr} => (match $token { Token::Plus | Token::Minus => {2}, Token::Multiply | Token::Divide => {3}, _ => {0}});
    }

// Only using fully defined values so warning is not a problem.
macro_rules! double_check {
    {$op:expr} => (match $op {f64::INFINITY | f64::NEG_INFINITY => { return Err(MathError::DoubleOverflow);}, x if x.is_nan() => { return Err(MathError::Error); }, x => {x}});
}

macro_rules! double {
    {$token:expr} => (match $token {
            Token::Integer(i) => i as f64,
            Token::Fraction(i) => i.int as f64 + i.num as f64 / i.den as f64,
            Token::Power(i, b, e) => {
                double_check!((i.int as f64 + i.num as f64 / i.den as f64)
                    * (b.den as f64 / b.num as f64).powf(e.den as f64 / e.num as f64))
            }
            Token::Double(i) => i,
            _ => return Err(MathError::Impossible),
        })
}
// Ensure secure double add
