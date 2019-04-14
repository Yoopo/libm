use super::signgam;
use core::{f32, i32};
use libc::{c_float, c_int, c_long, c_longlong};
use libm_internals;

#[no_mangle]
pub extern "C" fn acosf(arg: c_float) -> c_float {
    libm_internals::acosf(arg)
}

#[no_mangle]
pub extern "C" fn acoshf(arg: c_float) -> c_float {
    libm_internals::acoshf(arg)
}

#[no_mangle]
pub extern "C" fn asinf(arg: c_float) -> c_float {
    libm_internals::asinf(arg)
}

#[no_mangle]
pub extern "C" fn asinhf(arg: c_float) -> c_float {
    libm_internals::asinhf(arg)
}

#[no_mangle]
pub extern "C" fn atanf(arg: c_float) -> c_float {
    libm_internals::atanf(arg)
}

#[no_mangle]
pub extern "C" fn atan2f(y: c_float, x: c_float) -> c_float {
    libm_internals::atan2f(y, x)
}

#[no_mangle]
pub extern "C" fn atanhf(arg: c_float) -> c_float {
    libm_internals::atanhf(arg)
}

#[no_mangle]
pub extern "C" fn cbrtf(arg: c_float) -> c_float {
    libm_internals::cbrtf(arg)
}

#[no_mangle]
pub extern "C" fn ceilf(arg: c_float) -> c_float {
    libm_internals::ceilf(arg)
}

#[no_mangle]
pub extern "C" fn cosf(arg: c_float) -> c_float {
    libm_internals::cosf(arg)
}

#[no_mangle]
pub extern "C" fn coshf(arg: c_float) -> c_float {
    libm_internals::coshf(arg)
}

#[no_mangle]
pub extern "C" fn expf(arg: c_float) -> c_float {
    libm_internals::expf(arg)
}

#[no_mangle]
pub extern "C" fn exp2f(arg: c_float) -> c_float {
    libm_internals::exp2f(arg)
}

#[no_mangle]
pub extern "C" fn expm1f(arg: c_float) -> c_float {
    libm_internals::expm1f(arg)
}

#[no_mangle]
pub extern "C" fn erff(arg: c_float) -> c_float {
    libm_internals::erff(arg)
}

#[no_mangle]
pub extern "C" fn erfcf(arg: c_float) -> c_float {
    libm_internals::erff(arg)
}

#[no_mangle]
pub extern "C" fn fabsf(arg: c_float) -> c_float {
    libm_internals::fabsf(arg)
}

#[no_mangle]
pub extern "C" fn fdimf(x: c_float, y: c_float) -> c_float {
    libm_internals::fdimf(x, y)
}

#[no_mangle]
pub extern "C" fn floorf(arg: c_float) -> c_float {
    libm_internals::floorf(arg)
}

#[no_mangle]
pub extern "C" fn fmaf(x: c_float, y: c_float, z: c_float) -> c_float {
    libm_internals::fmaf(x, y, z)
}

#[no_mangle]
pub extern "C" fn fmodf(numer: c_float, denom: c_float) -> c_float {
    libm_internals::fmodf(numer, denom)
}

#[no_mangle]
pub extern "C" fn fmaxf(x: c_float, y: c_float) -> c_float {
    (x as f32).max(y)
}

#[no_mangle]
pub extern "C" fn fminf(x: c_float, y: c_float) -> c_float {
    (x as f32).min(y)
}

#[no_mangle]
pub extern "C" fn hypotf(x: c_float, y: c_float) -> c_float {
    libm_internals::hypotf(x, y)
}

#[no_mangle]
pub extern "C" fn logf(arg: c_float) -> c_float {
    libm_internals::logf(arg)
}

#[no_mangle]
pub extern "C" fn log10f(arg: c_float) -> c_float {
    libm_internals::log10f(arg)
}

#[no_mangle]
pub extern "C" fn log1pf(arg: c_float) -> c_float {
    libm_internals::log1pf(arg)
}

#[no_mangle]
pub extern "C" fn log2f(arg: c_float) -> c_float {
    libm_internals::log2f(arg)
}

#[no_mangle]
pub extern "C" fn powf(base: c_float, exponent: c_float) -> c_float {
    libm_internals::powf(base, exponent)
}

#[no_mangle]
pub extern "C" fn roundf(arg: c_float) -> c_float {
    libm_internals::roundf(arg)
}

#[no_mangle]
pub extern "C" fn scalbnf(x: c_float, n: c_int) -> c_float {
    libm_internals::scalbnf(x, n)
}

#[no_mangle]
pub extern "C" fn sinf(arg: c_float) -> c_float {
    libm_internals::sinf(arg)
}

#[no_mangle]
pub extern "C" fn sinhf(arg: c_float) -> c_float {
    libm_internals::sinhf(arg)
}

#[no_mangle]
pub extern "C" fn sqrtf(arg: c_float) -> c_float {
    libm_internals::sqrtf(arg)
}

#[no_mangle]
pub extern "C" fn tanf(arg: c_float) -> c_float {
    libm_internals::tanf(arg)
}

#[no_mangle]
pub extern "C" fn tanhf(arg: c_float) -> c_float {
    libm_internals::tanhf(arg)
}

#[no_mangle]
pub extern "C" fn truncf(arg: c_float) -> c_float {
    libm_internals::truncf(arg)
}

#[no_mangle]
pub extern "C" fn lgammaf(arg: c_float) -> c_float {
    let (res, err) = libm_internals::lgammaf_r(arg);
    unsafe { signgam = err };
    res
}
#[no_mangle]
pub extern "C" fn tgammaf(arg: c_float) -> c_float {
    libm_internals::tgammaf(arg)
}

#[no_mangle]
pub extern "C" fn ilogbf(arg: c_float) -> c_int {
    libm_internals::ilogbf(arg) as c_int
}

#[no_mangle]
pub extern "C" fn llroundf(arg: c_float) -> c_longlong {
    libm_internals::roundf(arg) as c_longlong
}

#[no_mangle]
pub extern "C" fn lroundf(arg: c_float) -> c_long {
    libm_internals::roundf(arg) as c_long
}

#[no_mangle]
pub extern "C" fn llrintf(arg: c_float) -> c_longlong {
    libm_internals::rintf(arg) as c_longlong
}

#[no_mangle]
pub extern "C" fn lrintf(arg: c_float) -> c_long {
    libm_internals::rintf(arg) as c_long
}

#[no_mangle]
pub extern "C" fn rintf(arg: c_float) -> c_float {
    libm_internals::rintf(arg)
}
#[no_mangle]
pub extern "C" fn nearbyintf(arg: c_float) -> c_float {
    // FIXME: should not rise error
    libm_internals::rintf(arg)
}

#[no_mangle]
pub extern "C" fn sincosf(x: c_float, sin: *mut c_float, cos: *mut c_float) {
    let (rsin, rcos) = libm_internals::sincosf(x);
    unsafe {
        *sin = rsin;
        *cos = rcos;
    }
}
// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn scalblnf(x: c_float, n: c_long) -> c_float {
    if n > i32::max_value() as i64 {
        libm_internals::scalbnf(x, i32::max_value())
    } else if n < i32::min_value() as i64 {
        libm_internals::scalbnf(x, i32::min_value())
    } else {
        libm_internals::scalbnf(x, n as i32)
    }
}

// fixme should probably be in internals
#[no_mangle]
pub extern "C" fn logbf(x: c_float) -> c_float {
    if (x as f32).is_infinite() {
        return x * x;
    }
    if x == 0.0 {
        return -1.0 / (x * x);
    }
    return libm_internals::ilogbf(x) as f32;
}

#[no_mangle]
pub extern "C" fn j0f(x: c_float) -> c_float {
    return libm_internals::j0f(x);
}

#[no_mangle]
pub extern "C" fn y0f(x: c_float) -> c_float {
    return libm_internals::y0f(x);
}

#[no_mangle]
pub extern "C" fn j1f(x: c_float) -> c_float {
    return libm_internals::j1f(x);
}

#[no_mangle]
pub extern "C" fn y1f(x: c_float) -> c_float {
    return libm_internals::y1f(x);
}

#[no_mangle]
pub extern "C" fn jnf(n: c_int, x: c_float) -> c_float {
    return libm_internals::jnf(n as isize, x);
}

#[no_mangle]
pub extern "C" fn ynf(n: c_int, x: c_float) -> c_float {
    return libm_internals::ynf(n as isize, x);
}

#[no_mangle]
pub extern "C" fn nextafterf(x: c_float, y: c_float) -> c_float {
    return libm_internals::nextafterf(x, y);
}

#[no_mangle]
pub extern "C" fn remquof(numer: c_float, denom: c_float, quot: *mut c_int) -> c_float {
    let (res, q) = libm_internals::remquof(numer, denom);
    unsafe {
        *quot = q as i32;
    }
    res
}

#[no_mangle]
pub extern "C" fn remainderf(numer: c_float, denom: c_float) -> c_float {
    libm_internals::remquof(numer, denom).0
}

// newlib test cfg ?
#[no_mangle]
pub extern "C" fn __isfinitef(x: c_float) -> c_int {
    if (x as f32).is_finite() {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn __isnormalf(x: c_float) -> c_int {
    if (x as f32).is_normal() {
        1
    } else {
        0
    }
}

// fixme
#[no_mangle]
pub extern "C" fn __fpclassifyf(x: c_float) -> c_int {
    const FP_NORMAL: i32 = 0x4;
    const FP_INFINITE: i32 = 0x1;
    const FP_ZERO: i32 = 0x10;
    const FP_SUBNORMAL: i32 = 0x8;
    const FP_NAN: i32 = 0x2;
    return FP_NORMAL;
}
