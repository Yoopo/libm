use core::{f64};
use crate::{sqrt, log};

#[inline]
pub fn acosh(arg : f64) -> f64 {
 match arg {
            x if x < 1.0 => f64::NAN,
            x => log(x + sqrt((x * x) - 1.0)),
        }
}