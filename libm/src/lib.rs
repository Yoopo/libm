mod lib_f32;
mod lib_f64;
mod lib_fenv;
mod lib_long_double;
pub use lib_f32::*;
pub use lib_f64::*;
pub use lib_long_double::*;
pub use lib_fenv::*;

#[no_mangle]
pub static mut signgam: isize = 0;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let pi = 3.14159265359;
        let res = cos(pi);
        assert_eq!(res, -1.0);
    }
    #[test]
    fn it_worksf() {
        let pi = 3.14159265359;
        let res = cosf(pi);
        assert_eq!(res, -1.0);
    }
}
