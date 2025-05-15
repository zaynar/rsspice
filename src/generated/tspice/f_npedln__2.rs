//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LVLTOL: f64 = 0.000000000001;

//
// Supporting function T_ISEDPT
//
pub fn T_ISEDPT(
    PT: &[f64],
    A: f64,
    B: f64,
    C: f64,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let PT = DummyArray::new(PT, 1..=3);
    let mut T_ISEDPT: bool = false;
    let mut ELPT = StackArray::<f64, 3>::new(1..=3);
    let mut LEVEL: f64 = 0.0;
    let mut SCALE: f64 = 0.0;
    let mut SCLA: f64 = 0.0;
    let mut SCLB: f64 = 0.0;
    let mut SCLC: f64 = 0.0;

    //
    // Test whether a point is on a specified ellipsoid.
    //

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
    // Scale ellipse and plane.
    //
    SCALE = intrinsics::DMAX1(&[f64::abs(A), f64::abs(B), f64::abs(C)]);

    SCLA = (A / SCALE);
    SCLB = (B / SCALE);
    SCLC = (C / SCALE);

    spicelib::VSCL((1.0 / SCALE), PT.as_slice(), ELPT.as_slice_mut());

    //
    // Check the `level' of the point.
    //
    LEVEL = (((f64::powi(ELPT[1], 2) / f64::powi(SCLA, 2))
        + (f64::powi(ELPT[2], 2) / f64::powi(SCLB, 2)))
        + (f64::powi(ELPT[3], 2) / f64::powi(SCLC, 2)));

    testutil::CHCKSD(b"LEVEL", LEVEL, b"~", 1.0, LVLTOL, OK, ctx)?;

    T_ISEDPT = *OK;

    Ok(T_ISEDPT)
}
