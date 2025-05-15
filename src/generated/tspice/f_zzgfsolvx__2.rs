//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//
// Test version of GFREFN. The EPSILON value causes
// an increase in the number of refinement steps
// required to converge as compared to bisection.
//
// The default GFREFN uses an EPSILON value 0.5.
//
pub fn T_REFNX(T1: f64, T2: f64, S1: bool, S2: bool, T: &mut f64) {
    let mut X: f64 = 0.0;
    let mut EPSILON: f64 = 0.0;

    EPSILON = 0.999;

    X = ((T1 * (1.0 - EPSILON)) + (T2 * EPSILON));

    *T = spicelib::BRCKTD(X, T1, T2);
}
