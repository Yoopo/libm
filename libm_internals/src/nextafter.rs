use core::f64;

pub fn nextafter(x: f64, y: f64) -> f64 {
    let mut ux = x.to_bits();
    let uy = y.to_bits();

    if (x.is_nan() || y.is_nan()) {
        return x + y;
    }
    if (ux == uy) {
        return y;
    }
    let ax = ux & (!1u64 + 1) / 2;
    let ay = uy & (!1u64 + 1) / 2;
    if (ax == 0) {
        if (ay == 0) {
            return y;
        }
        ux = (uy & 1u64 << 63) | 1;
    } else if (ax > ay || ((ux ^ uy) & 1u64 << 63) != 0) {
        ux -= 1;
    } else {
        ux += 1;
    }
    let e = ux >> 52 & 0x7ff;
    /* raise overflow if ux.f is infinite and x is finite */
    if (e == 0x7ff) {
        force_eval!(x + x);
    }
    /* raise underflow if ux.f is subnormal or zero */
    let ux_f = f64::from_bits(ux);
    if (e == 0) {
        force_eval!(x * x + ux_f * ux_f);
    }
    ux_f
}
