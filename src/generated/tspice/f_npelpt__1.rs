//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const UBEL: i32 = 9;
const LVLTOL: f64 = 0.00000000001;
const DSTTOL: f64 = 0.00000000001;

//
//
// Supporting function T_ISNPEL
//
pub fn T_ISNPEL(
    VIEWPT: &[f64],
    ELLIPS: &[f64],
    PNEAR: &[f64],
    D: f64,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let VIEWPT = DummyArray::new(VIEWPT, 1..=3);
    let ELLIPS = DummyArray::new(ELLIPS, 1..=UBEL);
    let PNEAR = DummyArray::new(PNEAR, 1..=3);
    let mut T_ISNPEL: bool = false;
    let mut AXMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut AXPT = StackArray::<f64, 3>::new(1..=3);
    let mut AXVIEW = StackArray::<f64, 3>::new(1..=3);
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut LEVEL: f64 = 0.0;
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut SCALE: f64 = 0.0;
    let mut SCLA: f64 = 0.0;
    let mut SCLB: f64 = 0.0;
    let mut SCLPT = StackArray::<f64, 3>::new(1..=3);
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut TANGNT = StackArray::<f64, 3>::new(1..=3);
    let mut VPAX = StackArray::<f64, 3>::new(1..=3);
    let mut VPSCL = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);

    //
    // The utility function T_ISNPEL tests whether the "near point"
    // found by NPELPT satisfies the criterion that the line segment
    // connecting this point to the viewing point is orthogonal to the
    // ellipse's tangent vector at the near point.  The function also
    // test the viewpoint-ellipse distance and verifies that the
    // near point lies on the ellipse.

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
    // Set an initial return value.
    //
    T_ISNPEL = false;

    //
    // We're going to express the vector from the ellipse's center
    // to the near point in a frame whose axes are parallel to the
    // ellipse's semi-axes.
    //
    // Unpack ellipse.
    //
    spicelib::EL2CGV(
        ELLIPS.as_slice(),
        CENTER.as_slice_mut(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
    );

    //
    // Scale view point, ellipse and near point.
    //
    SCALE = intrinsics::DMAX1(&[
        spicelib::VNORM(CENTER.as_slice()),
        spicelib::VNORM(SMAJOR.as_slice()),
        spicelib::VNORM(SMINOR.as_slice()),
    ]);

    spicelib::VSCL((1.0 / SCALE), VIEWPT.as_slice(), VPSCL.as_slice_mut());

    spicelib::VSCLIP((1.0 / SCALE), CENTER.as_slice_mut());
    spicelib::VSCLIP((1.0 / SCALE), SMAJOR.as_slice_mut());
    spicelib::VSCLIP((1.0 / SCALE), SMINOR.as_slice_mut());

    spicelib::VSCL((1.0 / SCALE), PNEAR.as_slice(), SCLPT.as_slice_mut());

    SCLA = spicelib::VNORM(SMAJOR.as_slice());
    SCLB = spicelib::VNORM(SMINOR.as_slice());

    //
    // Find the vector from the near point to the center.
    //
    spicelib::VSUB(SCLPT.as_slice(), CENTER.as_slice(), OFFSET.as_slice_mut());

    //
    // Create the matrix that maps from the base frame to
    // a frame where the semi-axes are the x and y axes.
    //
    spicelib::TWOVEC(
        SMAJOR.as_slice(),
        1,
        SMINOR.as_slice(),
        2,
        AXMAT.as_slice_mut(),
        ctx,
    )?;

    //
    // Map the offset vector into the semi-axis frame.
    //
    spicelib::MXV(AXMAT.as_slice(), OFFSET.as_slice(), AXPT.as_slice_mut());

    //
    // Check the `level' of the near point.
    //
    LEVEL = ((f64::powi(AXPT[1], 2) / f64::powi(SCLA, 2))
        + (f64::powi(AXPT[2], 2) / f64::powi(SCLB, 2)));

    testutil::CHCKSD(b"LEVEL", LEVEL, b"~", 1.0, LVLTOL, OK, ctx)?;

    if !*OK {
        return Ok(T_ISNPEL);
    }

    //
    // The z-component of AXPT should be zero.
    //
    testutil::CHCKSD(b"AXPT(3)", AXPT[3], b"~", 0.0, LVLTOL, OK, ctx)?;

    if !*OK {
        return Ok(T_ISNPEL);
    }

    //
    // If we're still here, the near point is considered to be on
    // the ellipse.
    //
    // Next step:  find the tangent direction at the near point.
    //
    TANGNT[1] = -(AXPT[2] / f64::powi(SCLB, 2));
    TANGNT[2] = (AXPT[1] / f64::powi(SCLA, 2));
    TANGNT[3] = 0.0;

    //
    // Map the scaled view point into the semi-axis frame.  To do
    // this we must shift the origin to the ellipse's center, then
    // rotate the vector.
    //
    spicelib::VSUB(VPSCL.as_slice(), CENTER.as_slice(), VTEMP.as_slice_mut());
    spicelib::MXV(AXMAT.as_slice(), VTEMP.as_slice(), VPAX.as_slice_mut());

    //
    // Find the vector from the near point to the view point.
    //
    spicelib::VSUB(VPAX.as_slice(), AXPT.as_slice(), AXVIEW.as_slice_mut());

    //
    // AXVIEW should be orthogonal to the tangent vector.
    //
    testutil::CHCKSD(
        b"<AXVIEW,TANGNT>",
        spicelib::VDOT(AXVIEW.as_slice(), TANGNT.as_slice()),
        b"~",
        0.0,
        DSTTOL,
        OK,
        ctx,
    )?;

    if !*OK {
        return Ok(T_ISNPEL);
    }

    //
    // Check the distance to the near point.
    //
    testutil::CHCKSD(
        b"Distance",
        D,
        b"~/",
        (spicelib::VNORM(AXVIEW.as_slice()) * SCALE),
        DSTTOL,
        OK,
        ctx,
    )?;

    if !*OK {
        return Ok(T_ISNPEL);
    }

    //
    // The tests passed if we made it this far.
    //
    T_ISNPEL = true;

    Ok(T_ISNPEL)
}
