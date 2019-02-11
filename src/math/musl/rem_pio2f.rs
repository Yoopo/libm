/* origin: FreeBSD /usr/src/lib/msun/src/e_rem_pio2f.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 * Debugged and optimized by Bruce D. Evans.
 */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

use crate::math::rem_pio2_large;
use crate::math::consts::*;

use core::f64;

const TOINT: f64 = 1.5 / f64::EPSILON;

/// 53 bits of 2/pi
const INV_PIO2: f64 = 6.366_197_723_675_813_824_33_e-01; /* 0x_3FE4_5F30, 0x_6DC9_C883 */
/// first 25 bits of pi/2
const PIO2_1: f64 = 1.570_796_310_901_641_845_7; /* 0x_3FF9_21FB, 0x_5000_0000 */
/// pi/2 - pio2_1
const PIO2_1T: f64 = 1.589_325_477_352_819_669_16_e-08; /* 0x_3E51_10b4, 0x_611A_6263 */

/// Return the remainder of x rem pi/2 in *y
///
/// use double precision for everything except passing x
/// use __rem_pio2_large() for large x
#[inline]
pub fn rem_pio2f(x: f32) -> (i32, f64) {
    let x64 = x as f64;

    let mut tx: [f64; 1] = [0.];

    let hx = x.to_bits();
    let sign = (hx >> 31) != 0;
    let ix = hx & UF_ABS;
    /* 25+53 bit pi is good enough for medium size */
    if ix < 0x_4dc9_0fdb {
        /* |x| ~< 2^28*(pi/2), medium size */
        /* Use a specialized rint() to get fn.  Assume round-to-nearest. */
        let f_n = x64 * INV_PIO2 + TOINT - TOINT;
        return (f_n as i32, x64 - f_n * PIO2_1 - f_n * PIO2_1T);
    }
    if ix >= UF_INF {
        /* x is inf or NaN */
        return (0, f64::NAN);
    }
    /* scale x into [2^23, 2^24-1] */
    let e0 = ((ix >> 23) - (0x7f + 23)) as i32; /* e0 = ilogb(|x|)-23, positive */
    tx[0] = f32::from_bits(ix - (e0 << 23) as u32) as f64;
    let (n, ty) = rem_pio2_large(&tx, e0, 0);
    if sign {
        return (-n, -ty[0]);
    }
    (n, ty[0])
}
