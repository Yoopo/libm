use super::k_lgamma_r::k_lgamma_r;
use super::signgam;

pub fn lgamma(x: f64) -> f64 {
    unsafe {k_lgamma_r(x, &mut signgam)}
}
