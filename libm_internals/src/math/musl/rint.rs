use core::f64;

// #if FLT_EVAL_METHOD==0 || FLT_EVAL_METHOD==1
// #define EPS DBL_EPSILON
const EPS: f64 = 2.2204460492503131e-16;
// #elif FLT_EVAL_METHOD==2
// #define EPS LDBL_EPSILON
// #endif

const TOINT: f64 = 1.0 / EPS;

#[inline]
pub fn rint(x: f64) -> f64 {
    let u = x.to_bits();
    let e = u >> 52 & 0x7ff;
    let s = u >> 63;

    if e >= 0x3ff + 52 {
        return x;
    }

    let y = if s != 0 {
        x - TOINT
     + TOINT
    
    } else {
        x + TOINT
     - TOINT
    
    };
    if y == 0.0 {
        return if s != 0 { -0.0 } else { 0.0 };
    }

    y
}
