use super::{FP_ILOGBNAN,FP_ILOGB0};
use core::i64;

pub fn ilogb(x:f64) -> i64
{
	let mut i = x.to_bits();
	let e : i64 = (i>>52 & 0x7ff) as i64;

	if e == 0 {
		i <<= 12;
		if i == 0 {
			force_eval!(0.0/0.0);
			return FP_ILOGB0 as i64;
		}
		/* subnormal x */
        let mut e2 : i64 = -0x3ff;
        while(i>>63 == 0) {
        e2 -= 1;
        i<<=1;
        }
		return e2;
	}
	if e == 0x7ff {
		force_eval!(0.0/0.0);
		return if i<<12 != 0 { FP_ILOGBNAN as i64 } else { i64::MAX };
	}
	return e - 0x3ff;
}