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

//$Procedure   ZZEPRC76   ( Earth precession, 1976 IAU model )
pub fn ZZEPRC76(ET: f64, PRECXF: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut PRECXF = DummyArrayMut2D::new(PRECXF, 1..=6, 1..=6);
    let mut CENT: f64 = 0.0;
    let mut DTHETA: f64 = 0.0;
    let mut DZ: f64 = 0.0;
    let mut DZETA: f64 = 0.0;
    let mut EULANG = StackArray::<f64, 6>::new(1..=6);
    let mut SCALE: f64 = 0.0;
    let mut T: f64 = 0.0;
    let mut THETA: f64 = 0.0;
    let mut TS: f64 = 0.0;
    let mut Z: f64 = 0.0;
    let mut ZETA: f64 = 0.0;

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
    CENT = (JYEAR() * 100.0);
    T = (ET / CENT);
    SCALE = (RPD(ctx) / 3600.0);

    ZETA = ((T * (ZETA1 + (T * (ZETA2 + (T * ZETA3))))) * SCALE);
    Z = ((T * (Z1 + (T * (Z2 + (T * Z3))))) * SCALE);
    THETA = ((T * (THETA1 + (T * (THETA2 + (T * THETA3))))) * SCALE);

    TS = (1.0 / CENT);
    DZETA = ((TS * (ZETA1 + (T * (((2 as f64) * ZETA2) + (((3 as f64) * T) * ZETA3))))) * SCALE);
    DZ = ((TS * (Z1 + (T * (((2 as f64) * Z2) + (((3 as f64) * T) * Z3))))) * SCALE);
    DTHETA =
        ((TS * (THETA1 + (T * (((2 as f64) * THETA2) + (((3 as f64) * T) * THETA3))))) * SCALE);

    //
    // Now compute the precession matrix.
    //
    EULANG[1] = -Z;
    EULANG[2] = THETA;
    EULANG[3] = -ZETA;
    EULANG[4] = -DZ;
    EULANG[5] = DTHETA;
    EULANG[6] = -DZETA;

    EUL2XF(EULANG.as_slice(), 3, 2, 3, PRECXF.as_slice_mut(), ctx)?;

    Ok(())
}
