/* @(#)z_asinef.c 1.0 98/08/13 */
/******************************************************************
 * The following routines are coded directly from the algorithms
 * and coefficients given in "Software Manual for the Elementary
 * Functions" by William J. Cody, Jr. and William Waite, Prentice
 * Hall, 1980.
 ******************************************************************/
/******************************************************************
 * Arcsine
 *
 * Input:
 *   x - floating point value
 *   acosine - indicates acos calculation
 *
 * Output:
 *   Arcsine of x.
 *
 * Description:
 *   This routine calculates arcsine / arccosine.
 *
 *****************************************************************/

use super::{fabsf, sqrtf};

const P: [f32; 2] = [ 0.933_935_835, -0.504_400_557 ];
const Q: [f32; 2] = [ 0.560_363_004e+1, -0.554_846_723e+1 ];
const A: [f32; 2] = [ 0., 0.785_398_163 ];
const B: [f32; 2] = [ 1.570_796_326, 0.785_398_163 ];
const Z_ROOTEPS_F: f32 = 1.726_334_918_258_910_7_e-4;

pub fn asinef(x: f32, acosine: usize) -> f32
{
    let flag: usize;
    let i: usize;
    let mut branch: bool = false;
    let g: f32;
    let mut res: f32 = 0.;
    let mut y: f32;

    /* Check for special values. */
    //i = numtestf (x);
    if x.is_nan() || x.is_infinite() {
        force_eval!(x);
        return x;
    }

    y = fabsf(x);
    flag = acosine;

    if y > 0.5 {
        i = 1 - flag;

        /* Check for range error. */
        if y > 1. {
            return f32::NAN;
        }

        g = (1. - y) / 2.;
        y = -2. * sqrtf(g);
        branch = true;
    } else {
        i = flag;
        if y < Z_ROOTEPS_F {
            res = y;
            g = 0.; // pleasing the uninitialized variable
        } else {
            g = y * y;
        }
    }

    if y >= Z_ROOTEPS_F || branch {
        /* Calculate the Taylor series. */
        let p = (P[1] * g + P[0]) * g;
        let q = (g + Q[1]) * g + Q[0];
        let r = p / q;

        res = y + y * r;
    }

    /* Calculate asine or acose. */
    if flag == 0 {
        res = (A[i] + res) + A[i];
        if x < 0. {
            res = -res;
        }
    } else {
        if x < 0. {
            res = (B[i] + res) + B[i];
        } else {
            res = (A[i] - res) + A[i];
        }
    }

    res
}
