macro_rules! mul {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_mul($rhs) {Some(i) => i, None => return my_math::MathError::Overflow});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_mul($rhs) {Some(i) => i, None => return $error});
    }
macro_rules! add {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_add($rhs) {Some(i) => {i}, None =>  return my_math::MathError::Overflow});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_add($rhs) {Some(i) => i, None => return $error});
    }
macro_rules! sub {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_sub($rhs) {Some(i) => i, None => return my_math::MathError::Overflow});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_sub($rhs) {Some(i) => i, None => return $error});
    }
macro_rules! div {
    {$lhs:expr, $rhs:expr} => (match $lhs.checked_div($rhs) {Some(i) => i, None => return my_math::MathError::Overflow});
    {$lhs:expr, $rhs:expr, $error:expr} => (match $lhs.checked_div($rhs) {Some(i) => i, None => return $error});
    }
macro_rules! precedence {
    {$token:expr} => (match $token { my_math::Token::Plus | my_math::Token::Minus => {2}, my_math::Token::Multiply | my_math::Token::Divide => {3}, _ => {0}});
    }
