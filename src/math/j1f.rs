/* origin: FreeBSD /usr/src/lib/msun/src/e_j1f.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

use core::f32;
use super::{cosf, fabsf, logf, sinf, sqrtf};

const INVSQRTPI: f32 = 5.641_896_128_7_e-01; /* 0x_3f10_6ebb */
const TPI: f32       = 6.366_197_466_9_e-01; /* 0x_3f22_f983 */

fn common(ix: u32, x: f32, y1: bool, sign: bool) -> f32
{
    let z: f64;
    let mut s: f64;
    let c: f64;
    let mut ss: f64;
    let mut cc: f64;

    s = sinf(x) as f64;
    if y1 {
        s = -s;
    }
    c = cosf(x) as f64;
    cc = s-c;
    if ix < 0x_7f00_0000 {
        ss = -s-c;
        z = cosf(2.0*x) as f64;
        if s*c > 0.0 {
            cc = z/ss;
        } else {
            ss = z/cc;
        }
        if ix < 0x_5880_0000 {
            if y1 {
                ss = -ss;
            }
            cc = (ponef(x) as f64)*cc-(qonef(x) as f64)*ss;
        }
    }
    if sign {
        cc = -cc;
    }
    INVSQRTPI*(cc as f32)/sqrtf(x)
}

/* R0/S0 on [0,2] */
const R00: f32 = -6.250_000_000_0_e-02; /* 0x_bd80_0000 */
const R01: f32 =  1.407_056_697_6_e-03; /* 0x_3ab8_6cfd */
const R02: f32 = -1.599_556_344_4_e-05; /* 0x_b786_2e36 */
const R03: f32 =  4.967_279_920_7_e-08; /* 0x_3355_57d2 */
const S01: f32 =  1.915_376_074_6_e-02; /* 0x_3c9c_e859 */
const S02: f32 =  1.859_467_884_1_e-04; /* 0x_3942_fab6 */
const S03: f32 =  1.177_184_685_7_e-06; /* 0x_359d_ffc2 */
const S04: f32 =  5.046_362_439_0_e-09; /* 0x_31ad_6446 */
const S05: f32 =  1.235_422_701_6_e-11; /* 0x_2d59_567e */

pub fn j1f(x: f32) -> f32
{
    let mut z: f32;
    let r: f32;
    let s: f32;
    let mut ix: u32;
    let sign: bool;

    ix = x.to_bits();
    sign = (ix>>31) != 0;
    ix &= 0x_7fff_ffff;
    if ix >= 0x_7f80_0000 {
        return 1.0/(x*x);
    }
    if ix >= 0x_4000_0000 {  /* |x| >= 2 */
        return common(ix, fabsf(x), false, sign);
    }
    if ix >= 0x_3900_0000 {  /* |x| >= 2**-13 */
        z = x*x;
        r = z*(R00+z*(R01+z*(R02+z*R03)));
        s = 1.0+z*(S01+z*(S02+z*(S03+z*(S04+z*S05))));
        z = 0.5 + r/s;
    } else {
        z = 0.5;
    }
    z*x
}

const U0: [f32; 5] = [
 -1.960_570_961_2_e-01, /* 0x_be48_c331 */
  5.044_387_280_9_e-02, /* 0x_3d4e_9e3c */
 -1.912_568_928_7_e-03, /* 0x_bafa_af2a */
  2.352_525_916_6_e-05, /* 0x_37c5_581c */
 -9.190_991_789_9_e-08, /* 0x_b3c5_6003 */
];
const V0: [f32; 5] = [
  1.991_673_186_4_e-02, /* 0x_3ca3_286a */
  2.025_525_755_0_e-04, /* 0x_3954_644b */
  1.356_087_977_9_e-06, /* 0x_35b6_02d4 */
  6.227_414_584_0_e-09, /* 0x_31d5_f8eb */
  1.665_592_490_3_e-11, /* 0x_2d92_81cf */
];

pub fn y1f(x: f32) -> f32
{
    let z: f32;
    let u: f32;
    let v: f32;
    let ix: u32;

    ix = x.to_bits();
    if ix.trailing_zeros() >= 31 {
        return f32::NEG_INFINITY;
    }
    if (ix>>31) != 0{
        return f32::NAN;
    }
    if ix >= 0x_7f80_0000 {
        return 1./x;
    }
    if ix >= 0x_4000_0000 {  /* |x| >= 2. */
        return common(ix,x,true,false);
    }
    if ix < 0x_3300_0000 {  /* x < 2**-25 */
        return -TPI/x;
    }
    z = x*x;
    u = U0[0]+z*(U0[1]+z*(U0[2]+z*(U0[3]+z*U0[4])));
    v = 1.0+z*(V0[0]+z*(V0[1]+z*(V0[2]+z*(V0[3]+z*V0[4]))));
    x*(u/v) + TPI*(j1f(x)*logf(x)-1./x)
}

/* For x >= 8, the asymptotic expansions of pone is
 *      1 + 15/128 s^2 - 4725/2^15 s^4 - ...,   where s = 1/x.
 * We approximate pone by
 *      pone(x) = 1 + (R/S)
 * where  R = pr0 + pr1*s^2 + pr2*s^4 + ... + pr5*s^10
 *        S = 1 + ps0*s^2 + ... + ps4*s^10
 * and
 *      | pone(x)-1-R/S | <= 2  ** ( -60.06)
 */

const PR8: [f32; 6] = [ /* for x in [inf, 8]=1/[0,0.125] */
  0., /* 0x_0000_0000 */
  1.171_875_000_0_e-01, /* 0x_3df0_0000 */
  1.323_948_097_2_e+01, /* 0x_4153_d4ea */
  4.120_518_493_7_e+02, /* 0x_43ce_06a3 */
  3.874_745_361_3_e+03, /* 0x_4572_2bed */
  7.914_479_492_2_e+03, /* 0x_45f7_53d6 */
];
const PS8: [f32; 5] = [
  1.142_073_669_4_e+02, /* 0x_42e4_6a2c */
  3.650_930_908_2_e+03, /* 0x_4564_2ee5 */
  3.695_620_703_1_e+04, /* 0x_4710_5c35 */
  9.760_279_687_5_e+04, /* 0x_47be_a166 */
  3.080_427_148_4_e+04, /* 0x_46f0_a88b */
];

const PR5: [f32; 6] = [ /* for x in [8,4.5454]=1/[0.125,0.22001] */
  1.319_905_209_4_e-11, /* 0x_2d68_333f */
  1.171_874_925_5_e-01, /* 0x_3def_ffff */
  6.802_751_064_3, /* 0x_40d9_b023 */
  1.083_081_817_6_e+02, /* 0x_42d8_9dca */
  5.176_361_694_3_e+02, /* 0x_4401_68b7 */
  5.287_152_099_6_e+02, /* 0x_4404_2dc6 */
];
const PS5: [f32; 5] = [
  5.928_059_768_7_e+01, /* 0x_426d_1f55 */
  9.914_014_282_2_e+02, /* 0x_4477_d9b1 */
  5.353_267_089_8_e+03, /* 0x_45a7_4a23 */
  7.844_690_429_7_e+03, /* 0x_45f5_2586 */
  1.504_046_875_0_e+03, /* 0x_44bc_0180 */
];

const PR3: [f32; 6] = [
  3.025_039_108_1_e-09, /* 0x_314f_e10d */
  1.171_868_667_0_e-01, /* 0x_3def_ffab */
  3.932_977_438, /* 0x_407b_b5e7 */
  3.511_940_383_9_e+01, /* 0x_420c_7a45 */
  9.105_500_793_5_e+01, /* 0x_42b6_1c2a */
  4.855_906_677_2_e+01, /* 0x_4242_3c7c */
];
const PS3: [f32; 5] = [
  3.479_130_935_7_e+01, /* 0x_420b_2a4d */
  3.367_624_511_7_e+02, /* 0x_43a8_6198 */
  1.046_871_460_0_e+03, /* 0x_4482_dbe3 */
  8.908_113_403_3_e+02, /* 0x_445e_b3ed */
  1.037_879_333_5_e+02, /* 0x_42cf_936c */
];

const PR2: [f32; 6] = [/* for x in [2.8570,2]=1/[0.3499,0.5] */
  1.077_108_322_5_e-07, /* 0x_33e7_4ea8 */
  1.171_762_198_2_e-01, /* 0x_3def_fa16 */
  2.368_515_014_6, /* 0x_4017_95c0 */
  1.224_261_093_1_e+01, /* 0x_4143_e1bc */
  1.769_397_163_4_e+01, /* 0x_418d_8d41 */
  5.073_523_044_6, /* 0x_40a2_5a4d */
];
const PS2: [f32; 5] = [
  2.143_648_529_1_e+01, /* 0x_41ab_7dec */
  1.252_902_298_0_e+02, /* 0x_42fa_9499 */
  2.322_764_740_0_e+02, /* 0x_4368_46c7 */
  1.176_793_746_9_e+02, /* 0x_42eb_5bd7 */
  8.364_639_282_2, /* 0x_4105_d590 */
];

fn ponef(x: f32) -> f32
{
    let p: &[f32; 6];
    let q: &[f32; 5];
    let z: f32;
    let r: f32;
    let s: f32;
    let mut ix: u32;

    ix = x.to_bits();
    ix &= 0x_7fff_ffff;
    if      ix >= 0x_4100_0000 {p = &PR8; q = &PS8;}
    else if ix >= 0x_4091_73eb {p = &PR5; q = &PS5;}
    else if ix >= 0x_4036_d917 {p = &PR3; q = &PS3;}
    else /*ix >= 0x_4000_0000*/{p = &PR2; q = &PS2;}
    z = 1./(x*x);
    r = p[0]+z*(p[1]+z*(p[2]+z*(p[3]+z*(p[4]+z*p[5]))));
    s = 1.+z*(q[0]+z*(q[1]+z*(q[2]+z*(q[3]+z*q[4]))));
    1. + r/s
}

/* For x >= 8, the asymptotic expansions of qone is
 *      3/8 s - 105/1024 s^3 - ..., where s = 1/x.
 * We approximate pone by
 *      qone(x) = s*(0.375 + (R/S))
 * where  R = qr1*s^2 + qr2*s^4 + ... + qr5*s^10
 *        S = 1 + qs1*s^2 + ... + qs6*s^12
 * and
 *      | qone(x)/s -0.375-R/S | <= 2  ** ( -61.13)
 */

const QR8: [f32; 6] = [ /* for x in [inf, 8]=1/[0,0.125] */
  0., /* 0x_0000_0000 */
 -1.025_390_625_0_e-01, /* 0x_bdd2_0000 */
 -1.627_175_331_1_e+01, /* 0x_c182_2c8d */
 -7.596_017_456_1_e+02, /* 0x_c43d_e683 */
 -1.184_980_664_1_e+04, /* 0x_c639_273a */
 -4.843_851_171_9_e+04, /* 0x_c73d_3683 */
];
const QS8: [f32; 6] = [
  1.613_953_704_8_e+02, /* 0x_4321_6537 */
  7.825_386_230_5_e+03, /* 0x_45f4_8b17 */
  1.338_753_437_5_e+05, /* 0x_4802_bcd6 */
  7.196_577_500_0_e+05, /* 0x_492f_b29c */
  6.666_012_500_0_e+05, /* 0x_4922_be94 */
 -2.944_902_500_0_e+05, /* 0x_c88f_cb48 */
];

const QR5: [f32; 6] = [ /* for x in [8,4.5454]=1/[0.125,0.22001] */
 -2.089_799_340_5_e-11, /* 0x_adb7_d219 */
 -1.025_390_476_0_e-01, /* 0x_bdd1_fffe */
 -8.056_447_982_8, /* 0x_c100_e736 */
 -1.836_696_014_4_e+02, /* 0x_c337_ab6b */
 -1.373_193_725_6_e+03, /* 0x_c4ab_a633 */
 -2.612_444_335_9_e+03, /* 0x_c523_471c */
];
const QS5: [f32; 6] = [
  8.127_655_029_3_e+01, /* 0x_42a2_8d98 */
  1.991_798_706_1_e+03, /* 0x_44f8_f98f */
  1.746_848_437_5_e+04, /* 0x_4688_78f8 */
  4.985_142_578_1_e+04, /* 0x_4742_bb6d */
  2.794_807_421_9_e+04, /* 0x_46da_5826 */
 -4.719_183_593_8_e+03, /* 0x_c593_7978 */
];

const QR3: [f32; 6] = [
 -5.078_312_437_2_e-09, /* 0x_b1ae_7d4f */
 -1.025_378_331_5_e-01, /* 0x_bdd1_ff5b */
 -4.610_116_004_9, /* 0x_c093_8612 */
 -5.784_722_137_5_e+01, /* 0x_c267_638e */
 -2.282_445_373_5_e+02, /* 0x_c364_3e9a */
 -2.192_101_287_8_e+02, /* 0x_c35b_35cb */
];
const QS3: [f32; 6] = [
  4.766_515_350_3_e+01, /* 0x_423e_a91e */
  6.738_651_123_0_e+02, /* 0x_4428_775e */
  3.380_152_832_0_e+03, /* 0x_4553_4272 */
  5.547_729_003_9_e+03, /* 0x_45ad_5dd5 */
  1.903_119_140_6_e+03, /* 0x_44ed_e3d0 */
 -1.352_011_871_3_e+02, /* 0x_c307_3381 */
];

const QR2: [f32; 6] = [ /* for x in [2.8570,2]=1/[0.3499,0.5] */
 -1.783_817_253_9_e-07, /* 0x_b43f_8932 */
 -1.025_170_460_3_e-01, /* 0x_bdd1_f475 */
 -2.752_205_610_3, /* 0x_c030_2423 */
 -1.966_361_618_0_e+01, /* 0x_c19d_4f16 */
 -4.232_531_356_8_e+01, /* 0x_c229_4d1f */
 -2.137_192_153_9_e+01, /* 0x_c1aa_f9b2 */
];
const QS2: [f32; 6] = [
  2.953_336_334_2_e+01, /* 0x_41ec_4454 */
  2.529_815_521_2_e+02, /* 0x_437c_fb47 */
  7.575_028_076_2_e+02, /* 0x_443d_602e */
  7.393_931_884_8_e+02, /* 0x_4438_d92a */
  1.559_490_051_3_e+02, /* 0x_431b_f2f2 */
 -4.959_498_882_3, /* 0x_c09e_b437 */
];

fn qonef(x: f32) -> f32
{
    let p: &[f32; 6];
    let q: &[f32; 6];
    let s: f32;
    let r: f32;
    let z: f32;
    let mut ix: u32;

    ix = x.to_bits();
    ix &= 0x_7fff_ffff;
    if      ix >= 0x_4100_0000 {p = &QR8; q = &QS8;}
    else if ix >= 0x_4091_73eb {p = &QR5; q = &QS5;}
    else if ix >= 0x_4036_d917 {p = &QR3; q = &QS3;}
    else /*ix >= 0x_4000_0000*/{p = &QR2; q = &QS2;}
    z = 1.0/(x*x);
    r = p[0]+z*(p[1]+z*(p[2]+z*(p[3]+z*(p[4]+z*p[5]))));
    s = 1.0+z*(q[0]+z*(q[1]+z*(q[2]+z*(q[3]+z*(q[4]+z*q[5])))));
    (0.375 + r/s)/x
} 
