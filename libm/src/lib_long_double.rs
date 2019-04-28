// FIXME: currently use lib_f64

use super::signgam;
use core::{f64, i32};
use libc::{c_double, c_int, c_long, c_longlong};
use libm_internals;

#[no_mangle]
pub extern "C" fn acosl(arg: c_double) -> c_double {
    libm_internals::acos(arg)
}

#[no_mangle]
pub extern "C" fn acoshl(arg: c_double) -> c_double {
    libm_internals::acosh(arg)
}

#[no_mangle]
pub extern "C" fn asinl(arg: c_double) -> c_double {
    libm_internals::asin(arg)
}

#[no_mangle]
pub extern "C" fn asinhl(arg: c_double) -> c_double {
    libm_internals::asinh(arg)
}

#[no_mangle]
pub extern "C" fn atanl(arg: c_double) -> c_double {
    libm_internals::atan(arg)
}

#[no_mangle]
pub extern "C" fn atan2l(y: c_double, x: c_double) -> c_double {
    libm_internals::atan2(y, x)
}

#[no_mangle]
pub extern "C" fn atanhl(arg: c_double) -> c_double {
    libm_internals::atanh(arg)
}

#[no_mangle]
pub extern "C" fn cbrtl(arg: c_double) -> c_double {
    libm_internals::cbrt(arg)
}

#[no_mangle]
pub extern "C" fn ceill(arg: c_double) -> c_double {
    libm_internals::ceil(arg)
}

#[no_mangle]
pub extern "C" fn cosl(arg: c_double) -> c_double {
    libm_internals::cos(arg)
}

#[no_mangle]
pub extern "C" fn coshl(arg: c_double) -> c_double {
    libm_internals::cosh(arg)
}

#[no_mangle]
pub extern "C" fn expl(arg: c_double) -> c_double {
    libm_internals::exp(arg)
}

#[no_mangle]
pub extern "C" fn exp2l(arg: c_double) -> c_double {
    libm_internals::exp2(arg)
}

#[no_mangle]
pub extern "C" fn expm1l(arg: c_double) -> c_double {
    libm_internals::expm1(arg)
}

#[no_mangle]
pub extern "C" fn erfl(arg: c_double) -> c_double {
    libm_internals::erf(arg)
}

#[no_mangle]
pub extern "C" fn erfcl(arg: c_double) -> c_double {
    libm_internals::erf(arg)
}

#[no_mangle]
pub extern "C" fn fabsl(arg: c_double) -> c_double {
    libm_internals::fabs(arg)
}

#[no_mangle]
pub extern "C" fn fdiml(x: c_double, y: c_double) -> c_double {
    libm_internals::fdim(x, y)
}

#[no_mangle]
pub extern "C" fn floorl(arg: c_double) -> c_double {
    libm_internals::floor(arg)
}

#[no_mangle]
pub extern "C" fn fmal(x: c_double, y: c_double, z: c_double) -> c_double {
    libm_internals::fma(x, y, z)
}

#[no_mangle]
pub extern "C" fn fmodl(numer: c_double, denom: c_double) -> c_double {
    libm_internals::fmod(numer, denom)
}

#[no_mangle]
pub extern "C" fn fmaxl(x: c_double, y: c_double) -> c_double {
    (x as f64).max(y)
}

#[no_mangle]
pub extern "C" fn fminl(x: c_double, y: c_double) -> c_double {
    (x as f64).min(y)
}

#[no_mangle]
pub extern "C" fn hypotl(x: c_double, y: c_double) -> c_double {
    libm_internals::hypot(x, y)
}

#[no_mangle]
pub extern "C" fn logl(arg: c_double) -> c_double {
    libm_internals::log(arg)
}

#[no_mangle]
pub extern "C" fn log10l(arg: c_double) -> c_double {
    libm_internals::log10(arg)
}

#[no_mangle]
pub extern "C" fn log1pl(arg: c_double) -> c_double {
    libm_internals::log1p(arg)
}

#[no_mangle]
pub extern "C" fn log2l(arg: c_double) -> c_double {
    libm_internals::log2(arg)
}

#[no_mangle]
pub extern "C" fn powl(base: c_double, exponent: c_double) -> c_double {
    libm_internals::pow(base, exponent)
}

#[no_mangle]
pub extern "C" fn roundl(arg: c_double) -> c_double {
    libm_internals::round(arg)
}

#[no_mangle]
pub extern "C" fn scalbnl(x: c_double, n: c_int) -> c_double {
    libm_internals::scalbn(x, n)
}

#[no_mangle]
pub extern "C" fn sinl(arg: c_double) -> c_double {
    libm_internals::sin(arg)
}

#[no_mangle]
pub extern "C" fn sinhl(arg: c_double) -> c_double {
    libm_internals::sinh(arg)
}

#[no_mangle]
pub extern "C" fn sqrtl(arg: c_double) -> c_double {
    libm_internals::sqrt(arg)
}

#[no_mangle]
pub extern "C" fn tanl(arg: c_double) -> c_double {
    libm_internals::tan(arg)
}

#[no_mangle]
pub extern "C" fn tanhl(arg: c_double) -> c_double {
    libm_internals::tanh(arg)
}

#[no_mangle]
pub extern "C" fn truncl(arg: c_double) -> c_double {
    libm_internals::trunc(arg)
}

#[no_mangle]
pub extern "C" fn lgammal(arg: c_double) -> c_double {
    let (res, err) = libm_internals::lgamma_r(arg);
    unsafe { signgam = err };
    res
}
#[no_mangle]
pub extern "C" fn tgammal(arg: c_double) -> c_double {
    libm_internals::tgamma(arg)
}

#[no_mangle]
pub extern "C" fn ilogbl(arg: c_double) -> c_int {
    libm_internals::ilogb(arg) as c_int
}

#[no_mangle]
pub extern "C" fn llroundl(arg: c_double) -> c_longlong {
    libm_internals::round(arg) as c_longlong
}

#[no_mangle]
pub extern "C" fn lroundl(arg: c_double) -> c_long {
    libm_internals::round(arg) as c_long
}

#[no_mangle]
pub extern "C" fn llrintl(arg: c_double) -> c_longlong {
    libm_internals::rint(arg) as c_longlong
}

#[no_mangle]
pub extern "C" fn lrintl(arg: c_double) -> c_long {
    libm_internals::rint(arg) as c_long
}

#[no_mangle]
pub extern "C" fn rintl(arg: c_double) -> c_double {
    libm_internals::rint(arg)
}

#[no_mangle]
pub extern "C" fn nearbyintl(arg: c_double) -> c_double {
    // FIXME: should not rise error
    libm_internals::rint(arg)
}

#[no_mangle]
pub extern "C" fn sincosl(x: c_double, sin: *mut c_double, cos: *mut c_double) {
    let (rsin, rcos) = libm_internals::sincos(x);
    unsafe {
        *sin = rsin;
        *cos = rcos;
    }
}
// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn scalblnl(x: c_double, n: c_long) -> c_double {
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
pub extern "C" fn logbl(x: c_double) -> c_double {
    if (x as f64).is_infinite() {
        return x * x;
    }
    if x == 0.0 {
        return -1.0 / (x * x);
    }
    return libm_internals::ilogb(x) as f64;
}

#[no_mangle]
pub extern "C" fn j0l(x: c_double) -> c_double {
    return libm_internals::j0(x);
}

#[no_mangle]
pub extern "C" fn y0l(x: c_double) -> c_double {
    return libm_internals::y0(x);
}

#[no_mangle]
pub extern "C" fn j1l(x: c_double) -> c_double {
    return libm_internals::j1(x);
}

#[no_mangle]
pub extern "C" fn y1l(x: c_double) -> c_double {
    return libm_internals::y1(x);
}

#[no_mangle]
pub extern "C" fn jnl(n: c_int, x: c_double) -> c_double {
    return libm_internals::jn(n as isize, x);
}

#[no_mangle]
pub extern "C" fn ynl(n: c_int, x: c_double) -> c_double {
    return libm_internals::yn(n as isize, x);
}

#[no_mangle]
pub extern "C" fn nextafterl(x: c_double, y: c_double) -> c_double {
    return libm_internals::nextafter(x, y);
}

#[no_mangle]
pub extern "C" fn remquol(numer: c_double, denom: c_double, quot: *mut c_int) -> c_double {
    let (res, q) = libm_internals::remquo(numer, denom);
    unsafe {
        *quot = q as i32;
    }
    res
}

#[no_mangle]
pub extern "C" fn remainderl(numer: c_double, denom: c_double) -> c_double {
    libm_internals::remquo(numer, denom).0
}

// newlib test cfg ?
#[no_mangle]
pub extern "C" fn __isfinitel(x: c_double) -> c_int {
    if (x as f64).is_finite() {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn __isnormall(x: c_double) -> c_int {
    if (x as f64).is_normal() {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn __fpclassifydl(x: c_double) -> c_int {
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
