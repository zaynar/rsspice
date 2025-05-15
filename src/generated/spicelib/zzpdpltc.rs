//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LIMIT: f64 = 0.01;

//$Procedure ZZPDPLTC (Planetodetic coordinates, point latitude check)
pub fn ZZPDPLTC(
    RE: f64,
    F: f64,
    P: &[f64],
    LAT: f64,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let P = DummyArray::new(P, 1..=3);
    let mut ZZPDPLTC: bool = false;
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut R2: f64 = 0.0;
    let mut XXPT: f64 = 0.0;
    let mut YXPT: f64 = 0.0;

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
    // Give the function a default value.
    //
    ZZPDPLTC = false;

    if RETURN(ctx) {
        return Ok(ZZPDPLTC);
    }

    CHKIN(b"ZZPDPLTC", ctx)?;

    //
    // The equatorial radius must be greater than zero.
    //
    if (RE <= 0.0) {
        SETMSG(b"Equatorial radius was *.", ctx);
        ERRDP(b"*", RE, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZPDPLTC", ctx)?;
        return Ok(ZZPDPLTC);
    }

    //
    // If the flattening coefficient is greater than one, the polar
    // radius computed below is negative. If it's equal to one, the
    // polar radius is zero. Either case is a problem, so signal an
    // error and check out.
    //
    if (F >= 1 as f64) {
        SETMSG(b"Flattening coefficient was *.", ctx);
        ERRDP(b"*", F, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZPDPLTC", ctx)?;
        return Ok(ZZPDPLTC);
    }

    //
    // The input point is assumed to be on the cone
    // corresponding to the input latitude.
    //
    // If the reference spheroid is prolate or spherical, there's
    // nothing to do: the point is automatically on the correct side of
    // the X-Y plane.
    //
    if (F <= 0.0) {
        ZZPDPLTC = true;
    } else {
        //
        // This is the oblate case.
        //
        //
        // If the point is on the "correct" side of the X-Y plane---
        // that is, its Z component as the same sign as LAT, the
        // point is considered to have the correct latitude.
        //
        // If the point is on the X-Y plane, or if LAT is zero, the point
        // is considered to have the indicated latitude. We condense
        // these cases by requiring that
        //
        //       LAT * P(3) >= 0
        //
        //    rather than
        //
        //       LAT * P(3) > 0
        //
        //
        if ((P[3] * LAT) >= 0.0) {
            ZZPDPLTC = true;
        } else if (f64::abs(LAT) >= LIMIT) {
            //
            // Ideally, the input point is considered to have the given
            // latitude if the point is on the side of the X-Y plane
            // corresponding to the sign of the input latitude. The
            // problem with this criterion is that it can't be applied
            // correctly when LAT has very small magnitude.
            //
            // If the magnitude of LAT is above the limit, it's ok to
            // use the sign of P(3) to determine whether the point
            // has the given latitude.
            //
            // The point has the indicated latitude if both LAT and P(3)
            // have the same sign. In this case, we know they have the
            // opposite sign.
            //
            ZZPDPLTC = false;
        } else {
            //
            // At this point we know LAT is non-zero, so the cone
            // corresponding to LAT has its vertex on the opposite side of
            // the X-Y plane from any point having latitude LAT. So it's
            // possible for a point to be on the cone but not have the
            // correct latitude.
            //
            // We're in the special case where the point's Z component
            // has the opposite sign as LAT, and the magnitude of LAT
            // is below the limit. We don't automatically reject the
            // point in this case: we'll accept it if it is far enough
            // from the Z axis to be outside the portion of the latitude
            // cone on the wrong side of the X-Y plane.
            //
            A = RE;
            B = (A * (1.0 - F));

            //
            // Compute the intercepts of a normal vector of a point
            // at latitude LAT, longitude 0, with the X and Y axes.
            //
            ZZELNAXX(A, B, LAT, &mut XXPT, &mut YXPT, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZPDPLTC", ctx)?;
                return Ok(ZZPDPLTC);
            }

            //
            // We check the point's distance from the Z axis. This can be
            // done accurately even when the Z component of P consists of
            // noise.
            //
            // The point is considered to have the correct latitude when
            // it is farther from the Z axis than the intercept on the X
            // axis of a normal line passing through a point having
            // latitude LAT and longitude 0. Ideally, a point that is on
            // the latitude cone and that satisfies this criterion must be
            // on the correct side of the X-Y plane.
            //

            R2 = ((P[1] * P[1]) + (P[2] * P[2]));

            R = f64::sqrt(intrinsics::DMAX1(&[R2, 0.0]));

            ZZPDPLTC = (R >= XXPT);
        }
    }

    CHKOUT(b"ZZPDPLTC", ctx)?;
    Ok(ZZPDPLTC)
}
