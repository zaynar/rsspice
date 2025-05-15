//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DTOL: f64 = 0.000000000001;
const NPTOL: f64 = 0.000000000001;

//
// The utility function T_ISNPLN tests whether the "near point" found
// by NPEDLN satisfies the criterion that the outward surface normal
// at this point can be extended to intersect the input line
// orthogonally.
//
pub fn T_ISNPLN(
    A: f64,
    B: f64,
    C: f64,
    LINEPT: &[f64],
    LINEDR: &[f64],
    X: &[f64],
    D: f64,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let LINEPT = DummyArray::new(LINEPT, 1..=3);
    let LINEDR = DummyArray::new(LINEDR, 1..=3);
    let X = DummyArray::new(X, 1..=3);
    let mut T_ISNPLN: bool = false;
    let mut APPROX = StackArray::<f64, 3>::new(1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut LINMIN = StackArray::<f64, 3>::new(1..=3);
    let mut DIST2: f64 = 0.0;

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
    // Executable code.
    //

    //
    // The point on the line closest to the ellipsoid ( LINMIN ).
    //
    spicelib::NPLNPT(
        LINEPT.as_slice(),
        LINEDR.as_slice(),
        X.as_slice(),
        LINMIN.as_slice_mut(),
        &mut DIST2,
        ctx,
    )?;

    //
    // Check the distance.  This is pretty easy to get right.
    //
    testutil::CHCKSD(b"D", D, b"~/", DIST2, DTOL, OK, ctx)?;

    if !*OK {
        T_ISNPLN = false;
        return Ok(T_ISNPLN);
    }

    //
    // Obtain ellipsoid surface unit normal at the near point.
    //
    spicelib::SURFNM(A, B, C, X.as_slice(), NORMAL.as_slice_mut(), ctx)?;

    //
    // Approximation to LINMIN using X and D.
    //
    spicelib::VSCLIP(D, NORMAL.as_slice_mut());
    spicelib::VADD(X.as_slice(), NORMAL.as_slice(), APPROX.as_slice_mut());

    //
    // Check the relative error in the approximation to LINMIN.
    //
    testutil::CHCKAD(
        b"APPROX",
        APPROX.as_slice(),
        b"~~/",
        LINMIN.as_slice(),
        3,
        NPTOL,
        OK,
        ctx,
    )?;

    T_ISNPLN = *OK;

    Ok(T_ISNPLN)
}
