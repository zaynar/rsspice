//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//
// The utility function T_ISNPPT tests whether the "near point"
// found by NEARPT satisfies the criterion that the appropriately
// signed surface normal at the near point has small angular
// separation from the vector from the near point to view point.
//
pub fn T_ISNPPT(
    V: &[f64],
    A: f64,
    B: f64,
    C: f64,
    X: &[f64],
    ALT: f64,
    ATOL: f64,
    STOL: f64,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let V = DummyArray::new(V, 1..=3);
    let X = DummyArray::new(X, 1..=3);
    let mut T_ISNPPT: bool = false;
    let mut ALTSGN: f64 = 0.0;
    let mut LEVEL: f64 = 0.0;
    let mut MAXAX: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PVEC = StackArray::<f64, 3>::new(1..=3);
    let mut SEP: f64 = 0.0;
    let mut XALT: f64 = 0.0;

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
    // Find the level surface parameter of the view point.
    //
    LEVEL =
        ((f64::powf((V[1] / A), 2.0) + f64::powf((V[2] / B), 2.0)) + f64::powf((V[3] / C), 2.0));

    //
    // Set the sign of the altitude of the near point.
    //
    if (LEVEL >= 1.0) {
        ALTSGN = 1.0;
    } else {
        ALTSGN = -1.0;
    }

    //
    // Find the vector from the near point to the view point.
    //
    spicelib::VSUB(V.as_slice(), X.as_slice(), PVEC.as_slice_mut());

    //
    // Checking the altitude can be problematic, since small altitudes
    // can be accurate but have a large relative error.  Instead,
    // check the relative error in the sum of the altitude and
    // the maximum axis.
    //
    MAXAX = intrinsics::DMAX1(&[A, B, C]);

    XALT = (MAXAX + (ALTSGN * spicelib::VNORM(PVEC.as_slice())));

    testutil::CHCKSD(b"MAXAX+ALT", (MAXAX + ALT), b"~/", XALT, ATOL, OK, ctx)?;

    if !*OK {
        T_ISNPPT = false;
        return Ok(T_ISNPPT);
    }

    //
    // Obtain outward ellipsoid surface unit normal at the near point.
    //
    spicelib::SURFNM(A, B, C, X.as_slice(), NORMAL.as_slice_mut(), ctx)?;

    //
    // Adjust sign of normal vector according to whether view point is
    // inside or outside the ellipsoid.
    //
    spicelib::VSCLIP(ALTSGN, NORMAL.as_slice_mut());

    //
    // Find angular separation of sign-adjusted normal and PVEC.
    //
    SEP = spicelib::VSEP(NORMAL.as_slice(), PVEC.as_slice(), ctx);

    //
    // Check the angular separation.
    //
    testutil::CHCKSD(b"SEP", SEP, b"~", 0.0, STOL, OK, ctx)?;

    T_ISNPPT = *OK;

    Ok(T_ISNPPT)
}
