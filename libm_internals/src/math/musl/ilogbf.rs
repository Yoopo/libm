use core::f32;

#[allow(clippy::eq_op)]
const FP_ILOGBNAN: isize = -1 - ((!0) >> 1);
const FP_ILOGB0: isize = FP_ILOGBNAN;

/// Get exponent (f32)
///
/// All nonzero, normal numbers can be described as `m*2^p`.
/// Examines the argument `x`, and returns *p*.
pub fn ilogbf(x: f32) -> isize {
    let mut i = x.to_bits();
    let e = ((i >> 23) & 0xff) as isize;

    if e == 0 {
        i <<= 9;
        if i == 0 {
            force_eval!(f32::NAN);
            return FP_ILOGB0;
        }
        /* subnormal x */
        let mut e = -0x7f;
        while (i >> 31) == 0 {
            e -= 1;
            i <<= 1;
        }
        return e;
    }
    if e == 0xff {
        force_eval!(f32::NAN);
        if (i << 9) != 0 {
            return FP_ILOGBNAN;
        } else {
            return isize::max_value();
        }
    }
    e - 0x7f
}
