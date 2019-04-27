mod math;
pub use math::*;

pub const FP_ILOGBNAN: i32 = (-1 - 0x7fffffff);
pub const FP_ILOGB0: i32 = FP_ILOGBNAN;

// fixme
pub fn rintf(arg:f32) -> f32 {
    math::rint(arg as f64) as f32
}
pub fn nextafterf(x:f32, y:f32) -> f32 {
    math::nextafter(x as f64,y as f64) as f32
}