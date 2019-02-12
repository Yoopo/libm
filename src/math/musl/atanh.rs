use super::log1p;
use crate::math::consts::*;

/* atanh(x) = log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2 ~= x + x^3/3 + o(x^5) */
pub fn atanh(mut x: f64) -> f64 {
    let mut u = x.to_bits();
    let e = ((u >> 52) as usize) & 0x7ff;
    let sign = (u >> 63) != 0;

    /* |x| */
    u &= UD_ABS;
    x = f64::from_bits(u);

    if e < 0x3ff - 1 {
        if e < 0x3ff - 32 {
            /* handle underflow */
            if e == 0 {
                force_eval!(x as f32);
            }
        } else {
            /* |x| < 0.5, up to 1.7ulp error */
            x = 0.5 * log1p(2. * x + 2. * x * x / (1. - x));
        }
    } else {
        /* avoid overflow */
        x = 0.5 * log1p(2. * (x / (1. - x)));
    }

    if sign {
        -x
    } else {
        x
    }
}
