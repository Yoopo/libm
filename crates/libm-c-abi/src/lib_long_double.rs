// FIXME: currently use lib_f64

use super::signgam;
use core::{f64, i32};
use libc::{c_double, c_int, c_long, c_longlong};

#[no_mangle]
pub extern "C" fn acosl(arg: c_double) -> c_double {
    libm::acos(arg)
}

#[no_mangle]
pub extern "C" fn acoshl(arg: c_double) -> c_double {
    libm::acosh(arg)
}

#[no_mangle]
pub extern "C" fn asinl(arg: c_double) -> c_double {
    libm::asin(arg)
}

#[no_mangle]
pub extern "C" fn asinhl(arg: c_double) -> c_double {
    libm::asinh(arg)
}

#[no_mangle]
pub extern "C" fn atanl(arg: c_double) -> c_double {
    libm::atan(arg)
}

#[no_mangle]
pub extern "C" fn atan2l(y: c_double, x: c_double) -> c_double {
    libm::atan2(y, x)
}

#[no_mangle]
pub extern "C" fn atanhl(arg: c_double) -> c_double {
    libm::atanh(arg)
}

#[no_mangle]
pub extern "C" fn cbrtl(arg: c_double) -> c_double {
    libm::cbrt(arg)
}

#[no_mangle]
pub extern "C" fn ceill(arg: c_double) -> c_double {
    libm::ceil(arg)
}

#[no_mangle]
pub extern "C" fn cosl(arg: c_double) -> c_double {
    libm::cos(arg)
}

#[no_mangle]
pub extern "C" fn coshl(arg: c_double) -> c_double {
    libm::cosh(arg)
}

#[no_mangle]
pub extern "C" fn expl(arg: c_double) -> c_double {
    libm::exp(arg)
}

#[no_mangle]
pub extern "C" fn exp2l(arg: c_double) -> c_double {
    libm::exp2(arg)
}

#[no_mangle]
pub extern "C" fn expm1l(arg: c_double) -> c_double {
    libm::expm1(arg)
}

#[no_mangle]
pub extern "C" fn erfl(arg: c_double) -> c_double {
    libm::erf(arg)
}

#[no_mangle]
pub extern "C" fn erfcl(arg: c_double) -> c_double {
    libm::erf(arg)
}

#[no_mangle]
pub extern "C" fn fabsl(arg: c_double) -> c_double {
    libm::fabs(arg)
}

#[no_mangle]
pub extern "C" fn fdiml(x: c_double, y: c_double) -> c_double {
    libm::fdim(x, y)
}

#[no_mangle]
pub extern "C" fn floorl(arg: c_double) -> c_double {
    libm::floor(arg)
}

#[no_mangle]
pub extern "C" fn fmal(x: c_double, y: c_double, z: c_double) -> c_double {
    libm::fma(x, y, z)
}

#[no_mangle]
pub extern "C" fn fmodl(numer: c_double, denom: c_double) -> c_double {
    libm::fmod(numer, denom)
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
    libm::hypot(x, y)
}

#[no_mangle]
pub extern "C" fn logl(arg: c_double) -> c_double {
    libm::log(arg)
}

#[no_mangle]
pub extern "C" fn log10l(arg: c_double) -> c_double {
    libm::log10(arg)
}

#[no_mangle]
pub extern "C" fn log1pl(arg: c_double) -> c_double {
    libm::log1p(arg)
}

#[no_mangle]
pub extern "C" fn log2l(arg: c_double) -> c_double {
    libm::log2(arg)
}

#[no_mangle]
pub extern "C" fn powl(base: c_double, exponent: c_double) -> c_double {
    libm::pow(base, exponent)
}

#[no_mangle]
pub extern "C" fn roundl(arg: c_double) -> c_double {
    libm::round(arg)
}

#[no_mangle]
pub extern "C" fn scalbnl(x: c_double, n: c_int) -> c_double {
    libm::scalbn(x, n)
}

#[no_mangle]
pub extern "C" fn sinl(arg: c_double) -> c_double {
    libm::sin(arg)
}

#[no_mangle]
pub extern "C" fn sinhl(arg: c_double) -> c_double {
    libm::sinh(arg)
}

#[no_mangle]
pub extern "C" fn sqrtl(arg: c_double) -> c_double {
    libm::sqrt(arg)
}

#[no_mangle]
pub extern "C" fn tanl(arg: c_double) -> c_double {
    libm::tan(arg)
}

#[no_mangle]
pub extern "C" fn tanhl(arg: c_double) -> c_double {
    libm::tanh(arg)
}

#[no_mangle]
pub extern "C" fn truncl(arg: c_double) -> c_double {
    libm::trunc(arg)
}

#[no_mangle]
pub extern "C" fn lgammal(arg: c_double) -> c_double {
    let (res, err) = libm::lgamma_r(arg);
    unsafe { signgam = err };
    res
}
#[no_mangle]
pub extern "C" fn tgammal(arg: c_double) -> c_double {
    libm::tgamma(arg)
}

#[no_mangle]
pub extern "C" fn ilogbl(arg: c_double) -> c_int {
    libm::ilogb(arg) as c_int
}

#[no_mangle]
pub extern "C" fn llroundl(arg: c_double) -> c_longlong {
    libm::round(arg) as c_longlong
}

#[no_mangle]
pub extern "C" fn lroundl(arg: c_double) -> c_long {
    libm::round(arg) as c_long
}

#[no_mangle]
pub extern "C" fn llrintl(arg: c_double) -> c_longlong {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn lrintl(arg: c_double) -> c_long {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn rintl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn nearbyintl(arg: c_double) -> c_double {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sincosl(x: c_double, sin: *mut c_double, cos: *mut c_double) {
    let (rsin, rcos) = libm::sincos(x);
    unsafe {
        *sin = rsin;
        *cos = rcos;
    }
}
// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn scalblnl(x: c_double, n: c_long) -> c_double {
    if n > i32::max_value() as i64 {
        libm::scalbn(x, i32::max_value())
    } else if n < i32::min_value() as i64 {
        libm::scalbn(x, i32::min_value())
    } else {
        libm::scalbn(x, n as i32)
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
    return libm::ilogb(x) as f64;
}

#[no_mangle]
pub extern "C" fn j0l(x: c_double) -> c_double {
    return libm::j0(x);
}

#[no_mangle]
pub extern "C" fn y0l(x: c_double) -> c_double {
    return libm::y0(x);
}

#[no_mangle]
pub extern "C" fn j1l(x: c_double) -> c_double {
    return libm::j1(x);
}

#[no_mangle]
pub extern "C" fn y1l(x: c_double) -> c_double {
    return libm::y1(x);
}

#[no_mangle]
pub extern "C" fn jnl(n: c_int, x: c_double) -> c_double {
    return libm::jn(n, x);
}

#[no_mangle]
pub extern "C" fn ynl(n: c_int, x: c_double) -> c_double {
    return libm::yn(n, x);
}

#[no_mangle]
pub extern "C" fn nextafterl(x: c_double, y: c_double) -> c_double {
    return libm::nextafter(x, y);
}

#[no_mangle]
pub extern "C" fn remquol(numer: c_double, denom: c_double, quot: *mut c_int) -> c_double {
    let (res, q) = libm::remquo(numer, denom);
    unsafe {
        *quot = q as i32;
    }
    res
}

#[no_mangle]
pub extern "C" fn remainderl(numer: c_double, denom: c_double) -> c_double {
    libm::remquo(numer, denom).0
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
