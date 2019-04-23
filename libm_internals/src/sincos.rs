use super::{get_high_word, k_cos, k_sin, rem_pio2};

// return (sin(x), cos(x))
pub fn sincos(x: f64) -> (f64, f64) {
    let ix = get_high_word(x) & 0x7fffffff;

    /* |x| ~< pi/4 */
    if (ix <= 0x3fe921fb) {
        /* if |x| < 2**-27 * sqrt(2) */
        if (ix < 0x3e46a09e) {
            /* raise inexact if x!=0 and underflow if subnormal */
            let x1p120 = f64::from_bits(0x7b800000); // 0x1p120f === 2 ^ 120

            if ix < 0x00100000 {
                force_eval!(x / x1p120);
            } else {
                force_eval!(x + x1p120);
            }
            return (x, 1.0);
        }
        return (k_sin(x, 0.0, 0), k_cos(x, 0.0));
    }

    /* sincos(Inf or NaN) is NaN */
    if (ix >= 0x7ff00000) {
        return (x - x, x - x);
    }

    /* argument reduction needed */
    let (n, y0, y1) = rem_pio2(x);
    let s = k_sin(y0, y1, 1);
    let c = k_cos(y0, y1);

    return match n & 3 {
        0 => (s, c),
        1 => (c, -s),
        2 => (-s, -c),
        3 | _ => (-c, s),
    };
}
