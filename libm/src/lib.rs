#![no_std]
use libc::{c_double, c_float, c_int};
use libm_internals;

#[no_mangle]
pub extern "C" fn acos(arg : c_double) -> c_double {
   libm_internals::acos(arg) 
}

#[no_mangle]
pub extern "C" fn acosf(arg : c_float) -> c_float {
   libm_internals::acosf(arg) 
}

#[no_mangle]
pub extern "C" fn acosh(arg : c_double) -> c_double {
   libm_internals::acosh(arg) 
}

#[no_mangle]
pub extern "C" fn asin(arg : c_double) -> c_double {
   libm_internals::asin(arg) 
}

#[no_mangle]
pub extern "C" fn asinf(arg : c_float) -> c_float {
   libm_internals::asinf(arg) 
}

#[no_mangle]
pub extern "C" fn asinh(arg : c_double) -> c_double {
   libm_internals::asinh(arg) 
}

#[no_mangle]
pub extern "C" fn atan(arg : c_double) -> c_double {
   libm_internals::atan(arg) 
}

#[no_mangle]
pub extern "C" fn atan2(y : c_double, x : c_double) -> c_double {
   libm_internals::atan2(y,x) 
}

#[no_mangle]
pub extern "C" fn atan2f(y : c_float, x : c_float) -> c_float {
   libm_internals::atan2f(y,x) 
}

#[no_mangle]
pub extern "C" fn atanf(arg : c_float) -> c_float {
   libm_internals::atanf(arg) 
}

#[no_mangle]
pub extern "C" fn atanh(arg : c_double) -> c_double {
   libm_internals::atanh(arg) 
}

#[no_mangle]
pub extern "C" fn cbrt(arg : c_double) -> c_double {
   libm_internals::cbrt(arg) 
}

#[no_mangle]
pub extern "C" fn cbrtf(arg : c_float) -> c_float {
   libm_internals::cbrtf(arg) 
}

#[no_mangle]
pub extern "C" fn ceil(arg : c_double) -> c_double {
   libm_internals::ceil(arg) 
}

#[no_mangle]
pub extern "C" fn ceilf(arg : c_float) -> c_float {
   libm_internals::ceilf(arg) 
}

#[no_mangle]
pub extern "C" fn cos(arg : c_double) -> c_double {
   libm_internals::cos(arg) 
}

#[no_mangle]
pub extern "C" fn cosf(arg : c_float) -> c_float {
   libm_internals::cosf(arg) 
}

#[no_mangle]
pub extern "C" fn cosh(arg : c_double) -> c_double {
   libm_internals::cosh(arg) 
}

#[no_mangle]
pub extern "C" fn coshf(arg : c_float) -> c_float {
   libm_internals::coshf(arg) 
}

#[no_mangle]
pub extern "C" fn exp(arg : c_double) -> c_double {
   libm_internals::exp(arg) 
}

#[no_mangle]
pub extern "C" fn exp2(arg : c_double) -> c_double {
   libm_internals::exp2(arg) 
}

#[no_mangle]
pub extern "C" fn exp2f(arg : c_float) -> c_float {
   libm_internals::exp2f(arg) 
}

#[no_mangle]
pub extern "C" fn expf(arg : c_float) -> c_float {
   libm_internals::expf(arg) 
}

#[no_mangle]
pub extern "C" fn expm1(arg : c_double) -> c_double {
   libm_internals::expm1(arg) 
}

#[no_mangle]
pub extern "C" fn expm1f(arg : c_float) -> c_float {
   libm_internals::expm1f(arg) 
}
#[no_mangle]
pub extern "C" fn erf(arg : c_double) -> c_double {
   libm_internals::erf(arg) 
}

#[no_mangle]
pub extern "C" fn fabs(arg : c_double) -> c_double {
   libm_internals::fabs(arg) 
}

#[no_mangle]
pub extern "C" fn fabsf(arg : c_float) -> c_float {
   libm_internals::fabsf(arg) 
}

#[no_mangle]
pub extern "C" fn fdim(x : c_double,y : c_double) -> c_double {
   libm_internals::fdim(x,y) 
}

#[no_mangle]
pub extern "C" fn fdimf(x : c_float, y : c_float) -> c_float {
   libm_internals::fdimf(x,y) 
}

#[no_mangle]
pub extern "C" fn floor(arg : c_double) -> c_double {
   libm_internals::floor(arg) 
}

#[no_mangle]
pub extern "C" fn floorf(arg : c_float) -> c_float {
   libm_internals::floorf(arg) 
}

#[no_mangle]
pub extern "C" fn fma(x : c_double,y : c_double,z : c_double) -> c_double {
   libm_internals::fma(x,y,z) 
}

#[no_mangle]
pub extern "C" fn fmaf(x : c_float,y : c_float,z : c_float) -> c_float {
   libm_internals::fmaf(x,y,z) 
}

#[no_mangle]
pub extern "C" fn fmod(numer:c_double, denom : c_double) -> c_double {
   libm_internals::fmod(numer, denom) 
}

#[no_mangle]
pub extern "C" fn fmodf(numer:c_float, denom : c_float) -> c_float {
   libm_internals::fmodf(numer, denom) 
}

#[no_mangle]
pub extern "C" fn hypot(x : c_double,y : c_double) -> c_double {
   libm_internals::hypot(x,y) 
}

#[no_mangle]
pub extern "C" fn hypotf(x : c_float,y : c_float) -> c_float {
   libm_internals::hypotf(x,y) 
}

#[no_mangle]
pub extern "C" fn log(arg : c_double) -> c_double {
   libm_internals::log(arg) 
}

#[no_mangle]
pub extern "C" fn log10(arg : c_double) -> c_double {
   libm_internals::log10(arg) 
}

#[no_mangle]
pub extern "C" fn log10f(arg : c_float) -> c_float {
   libm_internals::log10f(arg) 
}

#[no_mangle]
pub extern "C" fn log1p(arg : c_double) -> c_double {
   libm_internals::log1p(arg) 
}

#[no_mangle]
pub extern "C" fn log1pf(arg : c_float) -> c_float {
   libm_internals::log1pf(arg) 
}

#[no_mangle]
pub extern "C" fn log2(arg : c_double) -> c_double {
   libm_internals::log2(arg) 
}

#[no_mangle]
pub extern "C" fn log2f(arg : c_float) -> c_float {
   libm_internals::log2f(arg) 
}

#[no_mangle]
pub extern "C" fn logf(arg : c_float) -> c_float {
   libm_internals::logf(arg) 
}

#[no_mangle]
pub extern "C" fn pow(base : c_double,exponent : c_double) -> c_double {
   libm_internals::pow(base,exponent) 
}

#[no_mangle]
pub extern "C" fn powf(base : c_float, exponent : c_float) -> c_float {
   libm_internals::powf(base, exponent) 
}

#[no_mangle]
pub extern "C" fn round(arg : c_double) -> c_double {
   libm_internals::round(arg) 
}

#[no_mangle]
pub extern "C" fn roundf(arg : c_float) -> c_float {
   libm_internals::roundf(arg) 
}

#[no_mangle]
pub extern "C" fn scalbn(x : c_double, n : c_int) -> c_double {
   libm_internals::scalbn(x,n) 
}

#[no_mangle]
pub extern "C" fn scalbnf(x : c_float, n : c_int) -> c_float {
   libm_internals::scalbnf(x,n) 
}

#[no_mangle]
pub extern "C" fn sin(arg : c_double) -> c_double {
   libm_internals::sin(arg) 
}

#[no_mangle]
pub extern "C" fn sinf(arg : c_float) -> c_float {
   libm_internals::sinf(arg) 
}

#[no_mangle]
pub extern "C" fn sinh(arg : c_double) -> c_double {
   libm_internals::sinh(arg) 
}

#[no_mangle]
pub extern "C" fn sinhf(arg : c_float) -> c_float {
   libm_internals::sinhf(arg) 
}

#[no_mangle]
pub extern "C" fn sqrt(arg : c_double) -> c_double {
   libm_internals::sqrt(arg) 
}

#[no_mangle]
pub extern "C" fn sqrtf(arg : c_float) -> c_float {
   libm_internals::sqrtf(arg) 
}

#[no_mangle]
pub extern "C" fn tan(arg : c_double) -> c_double {
   libm_internals::tan(arg) 
}

#[no_mangle]
pub extern "C" fn tanf(arg : c_float) -> c_float {
   libm_internals::tanf(arg) 
}

#[no_mangle]
pub extern "C" fn tanh(arg : c_double) -> c_double {
   libm_internals::tanh(arg) 
}

#[no_mangle]
pub extern "C" fn tanhf(arg : c_float) -> c_float {
   libm_internals::tanhf(arg) 
}

#[no_mangle]
pub extern "C" fn trunc(arg : c_double) -> c_double {
   libm_internals::trunc(arg) 
}

#[no_mangle]
pub extern "C" fn truncf(arg : c_float) -> c_float {
   libm_internals::truncf(arg) 
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
