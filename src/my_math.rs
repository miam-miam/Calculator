const MAX_I128_LOG_2: i128 = 127;
const MAX_I128_LOG_10: i128 = 38;

pub fn ten_to_the_power_of(exponent: i128) -> Option<i128> {
    if exponent > MAX_I128_LOG_10 {
        return None;
    }
    let mut count: i128 = 1;
    for _ in 0..exponent {
        count *= 10;
    }
    Some(count)
}
