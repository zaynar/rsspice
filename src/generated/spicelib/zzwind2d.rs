//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure  ZZWIND2D ( Find winding number of polygon about point )
pub fn ZZWIND2D(
    N: i32,
    VERTCS: &[f64],
    POINT: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<i32> {
    let VERTCS = DummyArray2D::new(VERTCS, 1..=2, 1..=N);
    let POINT = DummyArray::new(POINT, 1..=2);
    let mut ZZWIND2D: i32 = 0;
    let mut ATOTAL: f64 = 0.0;
    let mut RNEXT = StackArray::<f64, 2>::new(1..=2);
    let mut RPERP = StackArray::<f64, 2>::new(1..=2);
    let mut RVEC = StackArray::<f64, 2>::new(1..=2);
    let mut SEP: f64 = 0.0;
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
    ZZWIND2D = 0;

    if RETURN(ctx) {
        return Ok(ZZWIND2D);
    }

    CHKIN(b"ZZWIND2D", ctx)?;

    //
    // Check the number of sides of the polygon.
    //
    if (N < 3) {
        SETMSG(b"Polygon must have at least 3 sides; N = #.", ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"ZZWIND2D", ctx)?;
        return Ok(ZZWIND2D);
    }

    //
    // The total "wrap angle" starts at zero.
    //
    ATOTAL = 0.0;

    VSUBG(
        VERTCS.subarray([1, 1]),
        POINT.as_slice(),
        2,
        RVEC.as_slice_mut(),
    );

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
        VSUBG(
            VERTCS.subarray([1, J]),
            POINT.as_slice(),
            2,
            RNEXT.as_slice_mut(),
        );

        SEP = VSEPG(RNEXT.as_slice(), RVEC.as_slice(), 2, ctx);

        //
        // Create a normal vector to RVEC by rotating RVEC pi/2 radians
        // counterclockwise.  We'll use this vector RPERP to determine
        // whether the next point is reached by clockwise or
        // counterclockwise rotation from RVEC.
        //
        RPERP[1] = -RVEC[2];
        RPERP[2] = RVEC[1];

        if (VDOTG(RNEXT.as_slice(), RPERP.as_slice(), 2) >= 0.0) {
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
        MOVED(RNEXT.as_slice(), 2, RVEC.as_slice_mut());
    }

    //
    // The above sum is 2 * pi * <the number of times the polygon
    // wraps around P>.  Let ZZWIND2D be the wrap count.
    //
    ZZWIND2D = intrinsics::IDNINT((ATOTAL / TWOPI(ctx)));

    CHKOUT(b"ZZWIND2D", ctx)?;
    Ok(ZZWIND2D)
}
