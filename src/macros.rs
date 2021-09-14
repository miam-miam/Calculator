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

macro_rules! pow {
    {$lhs:expr, $rhs:expr} => (
        if $rhs > u32::MAX as i128 {
            return Err(MathError::Overflow);
        }
        else {
            match $lhs.checked_pow($rhs as u32) {Some(i) => i, None => {return Err(MathError::Overflow);}}
        }
    );
    {$lhs:expr, $rhs:expr, $error:expr} => (
        if $rhs > u32::MAX as i128 {return Err(&error);}
        else {match $lhs.checked_pow($rhs as u32) {Some(i) => i, None => {return Err($error);}}}

    );
    }

macro_rules! abs {
    {$self:expr} => (match $self.checked_abs() {Some(i) => i, None => {return Err(MathError::Overflow);}});
    {$self:expr, $error:expr} => (match $self.checked_abs() {Some(i) => i, None => {return $error;}});
}

// Only using fully defined values so warning is not a problem.
macro_rules! double_check {
    {$op:expr} => (match $op {f64::INFINITY | f64::NEG_INFINITY => { return Err(MathError::DoubleOverflow);}, x if x.is_nan() => { return Err(MathError::DoubleOverflow);}, x => {x}});
}

macro_rules! trig_check {
    {$op: expr} => (match $op {val if val > 157079632.6 || val < -157079632.6 => {return Err(MathError::TrigAccuracy);}, val => val, })
}

macro_rules! none_to_err {
    {$op:expr} => (match $op {Some(x) => x, None => { return Err(MathError::Overflow); }});
    {$op:expr, $error:expr} => (match $op {Some(x) => x, None => { return Err($error); }});
}

macro_rules! commutative {
    {$lhs: pat, $rhs: pat} => (($lhs, $rhs) | ($rhs, $lhs));
}
