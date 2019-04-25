#![no_std]
use core::{f64, i32};
use libc::{c_double, c_float, c_int, c_long, c_longlong};
use libm_internals;

#[no_mangle]
pub extern "C" fn acos(arg: c_double) -> c_double {
    libm_internals::acos(arg)
}

#[no_mangle]
pub extern "C" fn acosf(arg: c_float) -> c_float {
    libm_internals::acosf(arg)
}

#[no_mangle]
pub extern "C" fn acosh(arg: c_double) -> c_double {
    libm_internals::acosh(arg)
}

#[no_mangle]
pub extern "C" fn asin(arg: c_double) -> c_double {
    libm_internals::asin(arg)
}

#[no_mangle]
pub extern "C" fn asinf(arg: c_float) -> c_float {
    libm_internals::asinf(arg)
}

#[no_mangle]
pub extern "C" fn asinh(arg: c_double) -> c_double {
    libm_internals::asinh(arg)
}

#[no_mangle]
pub extern "C" fn atan(arg: c_double) -> c_double {
    libm_internals::atan(arg)
}

#[no_mangle]
pub extern "C" fn atan2(y: c_double, x: c_double) -> c_double {
    libm_internals::atan2(y, x)
}

#[no_mangle]
pub extern "C" fn atan2f(y: c_float, x: c_float) -> c_float {
    libm_internals::atan2f(y, x)
}

#[no_mangle]
pub extern "C" fn atanf(arg: c_float) -> c_float {
    libm_internals::atanf(arg)
}

#[no_mangle]
pub extern "C" fn atanh(arg: c_double) -> c_double {
    libm_internals::atanh(arg)
}

#[no_mangle]
pub extern "C" fn cbrt(arg: c_double) -> c_double {
    libm_internals::cbrt(arg)
}

#[no_mangle]
pub extern "C" fn cbrtf(arg: c_float) -> c_float {
    libm_internals::cbrtf(arg)
}

#[no_mangle]
pub extern "C" fn ceil(arg: c_double) -> c_double {
    libm_internals::ceil(arg)
}

#[no_mangle]
pub extern "C" fn ceilf(arg: c_float) -> c_float {
    libm_internals::ceilf(arg)
}

#[no_mangle]
pub extern "C" fn cos(arg: c_double) -> c_double {
    libm_internals::cos(arg)
}

#[no_mangle]
pub extern "C" fn cosf(arg: c_float) -> c_float {
    libm_internals::cosf(arg)
}

#[no_mangle]
pub extern "C" fn cosh(arg: c_double) -> c_double {
    libm_internals::cosh(arg)
}

#[no_mangle]
pub extern "C" fn coshf(arg: c_float) -> c_float {
    libm_internals::coshf(arg)
}

#[no_mangle]
pub extern "C" fn exp(arg: c_double) -> c_double {
    libm_internals::exp(arg)
}

#[no_mangle]
pub extern "C" fn exp2(arg: c_double) -> c_double {
    libm_internals::exp2(arg)
}

#[no_mangle]
pub extern "C" fn exp2f(arg: c_float) -> c_float {
    libm_internals::exp2f(arg)
}

#[no_mangle]
pub extern "C" fn expf(arg: c_float) -> c_float {
    libm_internals::expf(arg)
}

#[no_mangle]
pub extern "C" fn expm1(arg: c_double) -> c_double {
    libm_internals::expm1(arg)
}

#[no_mangle]
pub extern "C" fn expm1f(arg: c_float) -> c_float {
    libm_internals::expm1f(arg)
}
#[no_mangle]
pub extern "C" fn erf(arg: c_double) -> c_double {
    libm_internals::erf(arg)
}

#[no_mangle]
pub extern "C" fn erfc(arg: c_double) -> c_double {
    libm_internals::erf(arg)
}

#[no_mangle]
pub extern "C" fn fabs(arg: c_double) -> c_double {
    libm_internals::fabs(arg)
}

#[no_mangle]
pub extern "C" fn fabsf(arg: c_float) -> c_float {
    libm_internals::fabsf(arg)
}

#[no_mangle]
pub extern "C" fn fdim(x: c_double, y: c_double) -> c_double {
    libm_internals::fdim(x, y)
}

#[no_mangle]
pub extern "C" fn fdimf(x: c_float, y: c_float) -> c_float {
    libm_internals::fdimf(x, y)
}

#[no_mangle]
pub extern "C" fn floor(arg: c_double) -> c_double {
    libm_internals::floor(arg)
}

#[no_mangle]
pub extern "C" fn floorf(arg: c_float) -> c_float {
    libm_internals::floorf(arg)
}

#[no_mangle]
pub extern "C" fn fma(x: c_double, y: c_double, z: c_double) -> c_double {
    libm_internals::fma(x, y, z)
}

#[no_mangle]
pub extern "C" fn fmaf(x: c_float, y: c_float, z: c_float) -> c_float {
    libm_internals::fmaf(x, y, z)
}

#[no_mangle]
pub extern "C" fn fmod(numer: c_double, denom: c_double) -> c_double {
    libm_internals::fmod(numer, denom)
}

#[no_mangle]
pub extern "C" fn fmodf(numer: c_float, denom: c_float) -> c_float {
    libm_internals::fmodf(numer, denom)
}

#[no_mangle]
pub extern "C" fn fmax(x: c_double, y: c_double) -> c_double {
    (x as f64).max(y)
}
#[no_mangle]
pub extern "C" fn fmaxf(x: c_float, y: c_float) -> c_float {
    (x as f32).max(y)
}
#[no_mangle]
pub extern "C" fn fmin(x: c_double, y: c_double) -> c_double {
    (x as f64).min(y)
}
#[no_mangle]
pub extern "C" fn fminf(x: c_float, y: c_float) -> c_float {
    (x as f32).min(y)
}

#[no_mangle]
pub extern "C" fn hypot(x: c_double, y: c_double) -> c_double {
    libm_internals::hypot(x, y)
}

#[no_mangle]
pub extern "C" fn hypotf(x: c_float, y: c_float) -> c_float {
    libm_internals::hypotf(x, y)
}

#[no_mangle]
pub extern "C" fn log(arg: c_double) -> c_double {
    libm_internals::log(arg)
}

#[no_mangle]
pub extern "C" fn log10(arg: c_double) -> c_double {
    libm_internals::log10(arg)
}

#[no_mangle]
pub extern "C" fn log10f(arg: c_float) -> c_float {
    libm_internals::log10f(arg)
}

#[no_mangle]
pub extern "C" fn log1p(arg: c_double) -> c_double {
    libm_internals::log1p(arg)
}

#[no_mangle]
pub extern "C" fn log1pf(arg: c_float) -> c_float {
    libm_internals::log1pf(arg)
}

#[no_mangle]
pub extern "C" fn log2(arg: c_double) -> c_double {
    libm_internals::log2(arg)
}

#[no_mangle]
pub extern "C" fn log2f(arg: c_float) -> c_float {
    libm_internals::log2f(arg)
}

#[no_mangle]
pub extern "C" fn logf(arg: c_float) -> c_float {
    libm_internals::logf(arg)
}

#[no_mangle]
pub extern "C" fn pow(base: c_double, exponent: c_double) -> c_double {
    libm_internals::pow(base, exponent)
}

#[no_mangle]
pub extern "C" fn powf(base: c_float, exponent: c_float) -> c_float {
    libm_internals::powf(base, exponent)
}

#[no_mangle]
pub extern "C" fn round(arg: c_double) -> c_double {
    libm_internals::round(arg)
}

#[no_mangle]
pub extern "C" fn roundf(arg: c_float) -> c_float {
    libm_internals::roundf(arg)
}

#[no_mangle]
pub extern "C" fn scalbn(x: c_double, n: c_int) -> c_double {
    libm_internals::scalbn(x, n)
}

#[no_mangle]
pub extern "C" fn scalbnf(x: c_float, n: c_int) -> c_float {
    libm_internals::scalbnf(x, n)
}

#[no_mangle]
pub extern "C" fn sin(arg: c_double) -> c_double {
    libm_internals::sin(arg)
}

#[no_mangle]
pub extern "C" fn sinf(arg: c_float) -> c_float {
    libm_internals::sinf(arg)
}

#[no_mangle]
pub extern "C" fn sinh(arg: c_double) -> c_double {
    libm_internals::sinh(arg)
}

#[no_mangle]
pub extern "C" fn sinhf(arg: c_float) -> c_float {
    libm_internals::sinhf(arg)
}

#[no_mangle]
pub extern "C" fn sqrt(arg: c_double) -> c_double {
    libm_internals::sqrt(arg)
}

#[no_mangle]
pub extern "C" fn sqrtf(arg: c_float) -> c_float {
    libm_internals::sqrtf(arg)
}

#[no_mangle]
pub extern "C" fn tan(arg: c_double) -> c_double {
    libm_internals::tan(arg)
}

#[no_mangle]
pub extern "C" fn tanf(arg: c_float) -> c_float {
    libm_internals::tanf(arg)
}

#[no_mangle]
pub extern "C" fn tanh(arg: c_double) -> c_double {
    libm_internals::tanh(arg)
}

#[no_mangle]
pub extern "C" fn tanhf(arg: c_float) -> c_float {
    libm_internals::tanhf(arg)
}

#[no_mangle]
pub extern "C" fn trunc(arg: c_double) -> c_double {
    libm_internals::trunc(arg)
}

#[no_mangle]
pub extern "C" fn truncf(arg: c_float) -> c_float {
    libm_internals::truncf(arg)
}

#[no_mangle]
pub static mut signgam: isize = 0;

#[no_mangle]
pub extern "C" fn lgamma(arg: c_double) -> c_double {
    let (res, err) = libm_internals::lgamma_r(arg);
    unsafe { signgam = err };
    res
}
#[no_mangle]
pub extern "C" fn tgamma(arg: c_double) -> c_double {
    libm_internals::tgamma(arg)
}

#[no_mangle]
pub extern "C" fn ilogb(arg: c_double) -> c_int {
    libm_internals::ilogb(arg) as c_int
}

#[no_mangle]
pub extern "C" fn llround(arg: c_double) -> c_longlong {
    libm_internals::round(arg) as c_longlong
}

#[no_mangle]
pub extern "C" fn lround(arg: c_double) -> c_long {
    libm_internals::round(arg) as c_long
}

#[no_mangle]
pub extern "C" fn llrint(arg: c_double) -> c_longlong {
    libm_internals::rint(arg) as c_longlong
}

#[no_mangle]
pub extern "C" fn lrint(arg: c_double) -> c_long {
    libm_internals::rint(arg) as c_long
}

#[no_mangle]
pub extern "C" fn rint(arg: c_double) -> c_double {
    libm_internals::rint(arg)
}
#[no_mangle]
pub extern "C" fn nearbyint(arg: c_double) -> c_double {
    // FIXME: should not rise error
    libm_internals::rint(arg)
}

#[no_mangle]
pub extern "C" fn sincos(x: c_double, sin: *mut c_double, cos: *mut c_double) {
    let (rsin, rcos) = libm_internals::sincos(x);
    unsafe {
        *sin = rsin;
        *cos = rcos;
    }
}
// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn scalbln(x: c_double, n: c_long) -> c_double {
    if n > i32::max_value() as i64 {
        libm_internals::scalbn(x, i32::max_value())
    } else if n < i32::min_value() as i64 {
        libm_internals::scalbn(x, i32::min_value())
    } else {
        libm_internals::scalbn(x, n as i32)
    }
}

// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn logb(x: c_double) -> c_double {
    if (x as f64).is_infinite() {
        return x * x;
    }
    if x == 0.0 {
        return -1.0 / (x * x);
    }
    return libm_internals::ilogb(x) as f64;
}

#[no_mangle]
pub extern "C" fn j0(x: c_double) -> c_double {
    return libm_internals::j0(x);
}

#[no_mangle]
pub extern "C" fn y0(x: c_double) -> c_double {
    return libm_internals::y0(x);
}

#[no_mangle]
pub extern "C" fn j1(x: c_double) -> c_double {
    return libm_internals::j1(x);
}

#[no_mangle]
pub extern "C" fn y1(x: c_double) -> c_double {
    return libm_internals::y1(x);
}

#[no_mangle]
pub extern "C" fn jn(n: c_int, x: c_double) -> c_double {
    return libm_internals::jn(n as isize, x);
}

#[no_mangle]
pub extern "C" fn yn(n: c_int, x: c_double) -> c_double {
    return libm_internals::yn(n as isize, x);
}

#[no_mangle]
pub extern "C" fn nextafter(x: c_double, y: c_double) -> c_double {
    return libm_internals::nextafter(x, y);
}

#[no_mangle]
pub extern "C" fn remquo(numer: c_double, denom: c_double, quot: *mut c_int) -> c_double {
    let (res, q) = libm_internals::remquo(numer, denom);
    unsafe {
        *quot = q as i32;
    }
    res
}

#[no_mangle]
pub extern "C" fn remainder(numer: c_double, denom: c_double) -> c_double {
    libm_internals::remquo(numer, denom).0
}

// newlib test cfg ?
#[no_mangle]
pub extern "C" fn __isfinite(x: c_double) -> c_int {
    if (x as f64).is_finite() {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn __isnormal(x: c_double) -> c_int {
    if (x as f64).is_normal() {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn __fpclassifyd(x: c_double) -> c_int {
    const FP_NORMAL: i32 = 0x4;
    const FP_INFINITE: i32 = 0x1;
    const FP_ZERO: i32 = 0x10;
    const FP_SUBNORMAL: i32 = 0x8;
    const FP_NAN: i32 = 0x2;

    let u = (x as f64).to_bits();
    let e = u >> 52 & 0x7ff;
    if e == 0 {
        return if u << 1 != 0 { FP_SUBNORMAL } else { FP_ZERO };
    }
    if e == 0x7ff {
        return if u << 12 != 0 { FP_NAN } else { FP_INFINITE };
    }
    return FP_NORMAL;
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let pi = 3.14159265359;
        let res = cos(pi);
        assert_eq!(res, -1.0);
    }
}
