//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const UBPL: i32 = 4;

//$Procedure  ZZWIND ( Find winding number of polygon about point )
pub fn ZZWIND(
    PLANE: &[f64],
    N: i32,
    VERTCS: &[f64],
    POINT: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let PLANE = DummyArray::new(PLANE, 1..=UBPL);
    let VERTCS = DummyArray2D::new(VERTCS, 1..=3, 1..);
    let POINT = DummyArray::new(POINT, 1..=3);
    let mut ZZWIND: i32 = 0;
    let mut ATOTAL: f64 = 0.0;
    let mut CONS: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut RNEXT = StackArray::<f64, 3>::new(1..=3);
    let mut RPERP = StackArray::<f64, 3>::new(1..=3);
    let mut RVEC = StackArray::<f64, 3>::new(1..=3);
    let mut SEP: f64 = 0.0;
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut J: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Initialize the function return value.
    //
    ZZWIND = 0;

    if RETURN(ctx) {
        return Ok(ZZWIND);
    }

    CHKIN(b"ZZWIND", ctx)?;

    //
    // Check the number of sides of the polygon.
    //
    if (N < 3) {
        SETMSG(b"Polygon must have at least 3 sides; N = #.", ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"ZZWIND", ctx)?;
        return Ok(ZZWIND);
    }

    //
    // Unpack the plane's normal and constant.
    //
    PL2NVC(PLANE.as_slice(), NORMAL.as_slice_mut(), &mut CONS);

    //
    // Check the normal vector.
    //
    if VZERO(NORMAL.as_slice()) {
        SETMSG(b"Plane\'s normal vector is zero.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZWIND", ctx)?;
        return Ok(ZZWIND);
    }

    //
    // We want the normal vector to point on the same side of the
    // plane as the boundary vectors.  Negate the normal
    // if necessary to make this true.  We don't touch CONS because
    // it's not used later, but in principle it should be negated.
    //
    if (VDOT(NORMAL.as_slice(), VERTCS.subarray([1, 1])) < 0.0) {
        VMINUS(NORMAL.as_slice(), VTEMP.as_slice_mut());
        VEQU(VTEMP.as_slice(), NORMAL.as_slice_mut());
    }

    //
    // Find the angular argument of each point; find the difference
    // of this angle from the preceding angle; add the difference to
    // the total.
    //
    VSUB(
        VERTCS.subarray([1, 1]),
        POINT.as_slice(),
        VTEMP.as_slice_mut(),
    );

    //
    // Get the component RVEC of the difference vector orthogonal to
    // the plane's normal vector.
    //
    VPERP(VTEMP.as_slice(), NORMAL.as_slice(), RVEC.as_slice_mut());

    //
    // The total "wrap angle" starts at zero.
    //
    ATOTAL = 0.0;

    for I in 2..=(N + 1) {
        if (I <= N) {
            J = I;
        } else {
            J = 1;
        }

        //
        // Find the angular separation of RVEC and the next vector
        // RNEXT.
        //
        VSUB(
            VERTCS.subarray([1, J]),
            POINT.as_slice(),
            VTEMP.as_slice_mut(),
        );
        VPERP(VTEMP.as_slice(), NORMAL.as_slice(), RNEXT.as_slice_mut());

        SEP = VSEP(RNEXT.as_slice(), RVEC.as_slice(), ctx);

        //
        // Create a normal vector to RVEC by rotating RVEC pi/2 radians
        // counterclockwise.  We'll use this vector RPERP to determine
        // whether the next point is reached by clockwise or
        // counterclockwise rotation from RVEC.
        //
        UCRSS(NORMAL.as_slice(), RVEC.as_slice(), RPERP.as_slice_mut());

        if (VDOT(RNEXT.as_slice(), RPERP.as_slice()) >= 0.0) {
            //
            // RNEXT is reached by counterclockwise rotation from
            // RVEC.  Note that in the case of zero rotation, the
            // sign doesn't matter because the contribution is zero.
            //
            ATOTAL = (ATOTAL + SEP);
        } else {
            ATOTAL = (ATOTAL - SEP);
        }

        //
        // Update RVEC.
        //
        VEQU(RNEXT.as_slice(), RVEC.as_slice_mut());
    }

    //
    // The above sum is 2 * pi * <the number of times the polygon
    // wraps around P>.  Let ZZWIND be the wrap count.
    //
    ZZWIND = intrinsics::IDNINT((ATOTAL / TWOPI(ctx)));

    CHKOUT(b"ZZWIND", ctx)?;
    Ok(ZZWIND)
}
