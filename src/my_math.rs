use std::cmp::min;

const MAX_I128_LOG_2: i128 = 127;
const MAX_I128_LOG_10: i128 = 38;

const SMALL_PRIMES: [i128; 110] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601,
];

const PRIME_WHEEL: [i128; 8] = [6, 4, 2, 4, 2, 4, 6, 2];

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

pub fn factorise(base: i128, sqrt: bool) -> (i128, i128) {
    let mut outside_root = 1_i128;
    let mut inside_root = base;
    let mut upper_bound = (match sqrt {
        true => (inside_root as f64).sqrt(),
        false => (inside_root as f64).cbrt(),
    })
    .ceil() as i128;

    // Check small primes.
    let mut i = 0;
    let mut update_upper_bound = true;
    let mut div_to_power;
    while i < 110 && SMALL_PRIMES[i] <= upper_bound {
        div_to_power = SMALL_PRIMES[i].pow(match sqrt {
            true => 2,
            false => 3,
        });
        while inside_root % div_to_power == 0 {
            inside_root /= div_to_power;
            outside_root *= SMALL_PRIMES[i];
            update_upper_bound = true;
        }
        if update_upper_bound {
            update_upper_bound = false;
            upper_bound = (match sqrt {
                true => (inside_root as f64).sqrt(),
                false => (inside_root as f64).cbrt(),
            })
            .ceil() as i128;
        }
        i += 1;
    }

    let mut div = 601;
    let mut ii = 0;
    // Min used as it would take too long to fully factor a large prime.
    // Num used is sqrt of i32 max.
    upper_bound = min(upper_bound, 46341);
    // Start using a wheel.
    while div <= upper_bound {
        if ii < 8 {
            ii = 0;
        }
        div += PRIME_WHEEL[ii];
        div_to_power = div.pow(match sqrt {
            true => 2,
            false => 3,
        });
        while inside_root % div_to_power == 0 {
            inside_root /= div_to_power;
            outside_root *= PRIME_WHEEL[i];
            update_upper_bound = true;
        }
        if update_upper_bound {
            update_upper_bound = false;
            upper_bound = min(
                46341,
                (match sqrt {
                    true => (inside_root as f64).sqrt(),
                    false => (inside_root as f64).cbrt(),
                })
                .ceil() as i128,
            );
        }
        ii += 1;
    }
    (outside_root, inside_root)
}
