//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ZETA1: f64 = 2306.2181;
const ZETA2: f64 = 0.30188;
const ZETA3: f64 = 0.017998;
const Z1: f64 = 2306.2181;
const Z2: f64 = 1.09468;
const Z3: f64 = 0.018203;
const THETA1: f64 = 2004.3109;
const THETA2: f64 = -0.42665;
const THETA3: f64 = -0.041833;

//$Procedure   ZZEPRCSS   ( Earth precession, 1976 IAU model )
pub fn ZZEPRCSS(ET: f64, PRECM: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut PRECM = DummyArrayMut2D::new(PRECM, 1..=3, 1..=3);
    let mut SCALE: f64 = 0.0;
    let mut T: f64 = 0.0;
    let mut ZETA: f64 = 0.0;
    let mut Z: f64 = 0.0;
    let mut THETA: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // No check-in required; this routine does not participate in
    // SPICELIB error handling.
    //
    //
    // Compute the precession angles first.  The time argument has
    // units of Julian centuries.  The polynomial expressions yield
    // angles in units of arcseconds prior to scaling.  After scaling,
    // the angles are in units of radians.
    //
    T = (ET / (JYEAR() * 100.0));
    SCALE = (RPD(ctx) / 3600.0);

    ZETA = ((T * (ZETA1 + (T * (ZETA2 + (T * ZETA3))))) * SCALE);
    Z = ((T * (Z1 + (T * (Z2 + (T * Z3))))) * SCALE);
    THETA = ((T * (THETA1 + (T * (THETA2 + (T * THETA3))))) * SCALE);

    //
    // Now compute the precession matrix.
    //
    EUL2M(-Z, THETA, -ZETA, 3, 2, 3, PRECM.as_slice_mut(), ctx)?;

    Ok(())
}
