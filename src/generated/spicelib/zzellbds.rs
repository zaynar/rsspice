//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZELLBDS ( Create bounding ellipsoids )
pub fn ZZELLBDS(
    A: f64,
    B: f64,
    HMAX: f64,
    HMIN: f64,
    AMAX: &mut f64,
    BMAX: &mut f64,
    AMIN: &mut f64,
    BMIN: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    //
    // Use discovery check-in.
    //
    if (B <= 0.0) {
        CHKIN(b"ZZELLBDS", ctx)?;
        SETMSG(b"This routine requires B > 0, but B = #.", ctx);
        ERRDP(b"#", B, ctx);
        SIGERR(b"SPICE(NONPOSITIVERADIUS)", ctx)?;
        CHKOUT(b"ZZELLBDS", ctx)?;
        return Ok(());
    }

    if (B > A) {
        CHKIN(b"ZZELLBDS", ctx)?;
        SETMSG(b"This routine requires A >= B, but A = #; B = #.", ctx);
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        SIGERR(b"SPICE(RADIIOUTOFORDER)", ctx)?;
        CHKOUT(b"ZZELLBDS", ctx)?;
        return Ok(());
    }

    if ((B + HMIN) <= 0.0) {
        CHKIN(b"ZZELLBDS", ctx)?;
        SETMSG(
            b"This routine requires B + HMIN > 0, but B = #; HMIN = #, B+HMIN = #.",
            ctx,
        );
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", HMIN, ctx);
        ERRDP(b"#", (B + HMIN), ctx);
        SIGERR(b"SPICE(LOWERBOUNDTOOLOW)", ctx)?;
        CHKOUT(b"ZZELLBDS", ctx)?;
        return Ok(());
    }

    if (HMIN < 0.0) {
        if ((B + ((A / B) * HMIN)) <= 0.0) {
            CHKIN(b"ZZELLBDS", ctx)?;
            SETMSG(b"For oblate spheroids and HMIN < 0, This routine requires B + (A/B)HMIN > 0, but A = #, B = #; HMIN = #, B+(A/B)HMIN = #.", ctx);
            ERRDP(b"#", A, ctx);
            ERRDP(b"#", B, ctx);
            ERRDP(b"#", HMIN, ctx);
            ERRDP(b"#", (B + ((A / B) * HMIN)), ctx);
            SIGERR(b"SPICE(LOWERBOUNDTOOLOW)", ctx)?;
            CHKOUT(b"ZZELLBDS", ctx)?;
            return Ok(());
        }
    }

    if (HMIN > HMAX) {
        CHKIN(b"ZZELLBDS", ctx)?;
        SETMSG(
            b"This routine requires HMAX >= HMIN, but HMIN = #; HMAX = #.",
            ctx,
        );
        ERRDP(b"#", HMIN, ctx);
        ERRDP(b"#", HMAX, ctx);
        SIGERR(b"SPICE(BOUNDSOUTOFORDER)", ctx)?;
        CHKOUT(b"ZZELLBDS", ctx)?;
        return Ok(());
    }

    //
    // In the following comments, N, E, E', and LAMBDA are
    // defined as in the Particulars section above.
    //
    //
    // Generate radii of the outer bounding ellipsoid.
    //
    if (HMAX >= 0.0) {
        //
        // Pick radii of E' so that E' matches
        //
        //    E + HMAX * N / ||N||
        //
        // that is, E' has height HMAX above E, at x=A.
        //
        // For smaller x, the height of E' above E will
        // will be greater than or equal to HMAX.
        //
        // Set LAMBDA = A * HMAX.
        //
        // Then the radii of E' are
        //
        //                      |
        //    x + LAMBDA*||N||  |
        //                      |x=A,y=0
        //
        // and
        //                      |
        //    y + LAMBDA*||N||  |
        //                      |x=0,y=B
        //
        // so the radii of E', AMAX and BMAX, are:
        //
        //    AMAX =  A + LAMBDA*A/A**2  =  A + HMAX
        //    BMAX =  B + LAMBDA*B/B**2  =  B + HMAX*(A/B)
        //
        //
        *AMAX = (A + HMAX);
        *BMAX = (B + (HMAX * (A / B)));
    } else {
        //
        // HMAX < 0.
        //
        // In this case the outer bounding ellipse should match E+HMAX
        // at x = 0. The ellipse will be closer to E for x > 0.
        //
        // Set LAMBDA = B * HMAX. Then
        //
        //    AMAX =  A + LAMBDA*A/A**2  =  A + HMAX * (B/A)
        //    BMAX =  B + LAMBDA*B/B**2  =  B + HMAX

        *AMAX = (A + (HMAX * (B / A)));
        *BMAX = (B + HMAX);
    }

    //
    // Find radii of the inner bounding ellipsoid.
    //
    if (HMIN <= 0 as f64) {
        //
        // This case is similar to that of the outer bounding
        // ellipsoid for HMAX >= 0. We can create an ellipse
        // that has height HMIN at x = A and that is further
        // from E for x < A.
        //
        // Set LAMBDA = A * HMAX. Then
        //
        //    AMAX =  A + LAMBDA*A/A**2  =  A + HMAX
        //    BMAX =  B + LAMBDA*B/B**2  =  B + HMAX*(A/B)

        *AMIN = (A + HMIN);
        *BMIN = (B + (HMIN * (A / B)));
    } else {
        //
        // HMIN > 0.
        //
        // In this case the inner bounding ellipse should match E+HMIN
        // at x = 0. The ellipse will be closer to E for x > 0.
        //
        // Set LAMBDA = B * HMAX. Then
        //
        //    AMIN =  A + LAMBDA*A/A**2  =  A + HMIN * (B/A)
        //    BMIN =  B + LAMBDA*B/B**2  =  B + HMIN
        //
        *AMIN = (A + (HMIN * (B / A)));
        *BMIN = (B + HMIN);
    }

    Ok(())
}
