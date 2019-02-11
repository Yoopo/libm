pub fn frexpf(x: f32) -> (f32, isize) {
    let mut y = x.to_bits();
    let ee: isize = ((y>>23) & 0xff) as isize;

    if ee == 0 {
        if x != 0. {
            let x1p64 = f32::from_bits(0x_5f80_0000);
            let (x, e) = frexpf(x*x1p64);
            return (x, e - 64);
        } else {
            return (x, 0);
        }
    } else if ee == 0xff {
        return (x, 0);
    }

    let e = ee - 0x7e;
    y &= 0x_807f_ffff;
    y |= 0x_3f00_0000;
    (f32::from_bits(y), e)
} 
