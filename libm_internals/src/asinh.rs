use crate::{log, sqrt};
use core::f64;

#[inline]
pub fn asinh(arg: f64) -> f64 {
    if arg == f64::NEG_INFINITY {
        f64::NEG_INFINITY
    } else {
        log(arg + sqrt((arg * arg) + 1.0))
    }
}
