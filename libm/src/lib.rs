#![no_std]

#[no_mangle]
pub static mut signgam: isize = 0;

mod lib_f64;
mod lib_f32;
pub use lib_f64::*;
pub use lib_f32::*;

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
