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

// Only using fully defined values so warning is not a problem.
macro_rules! double_check {
    {$op:expr} => (match $op {f64::INFINITY | f64::NEG_INFINITY => { return Err(MathError::DoubleOverflow);}, x if x.is_nan() => unreachable!(), x => {x}});
}

macro_rules! none_to_err {
    {$op:expr} => (match $op {Some(x) => x, None => { return Err(MathError::Overflow); }});
    {$op:expr, $error:expr} => (match $op {Some(x) => x, None => { return Err($error); }});
}
