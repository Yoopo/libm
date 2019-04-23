       
use core::{f64};
use crate::{log1p};

#[inline]
pub fn atanh(arg : f64) -> f64 {
 0.5 * log1p((2.0 * arg) / (1.0 - arg))
}      
