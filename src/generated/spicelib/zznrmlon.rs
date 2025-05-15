//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    PI2: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PI2: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self { PI2, FIRST }
    }
}

//$Procedure ZZNRMLON ( Normalize longitude bounds )
pub fn ZZNRMLON(
    INMIN: f64,
    INMAX: f64,
    TOL: f64,
    OUTMIN: &mut f64,
    OUTMAX: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DELTA: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Use discovery check-in. Don't check RETURN.
    //
    if save.FIRST {
        save.PI2 = TWOPI(ctx);
        save.FIRST = false;
    }

    //
    // TOL cannot be negative.
    //
    if (TOL < 0.0) {
        CHKIN(b"ZZNRMLON", ctx)?;
        SETMSG(b"Tolerance must be non-negative but was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZNRMLON", ctx)?;
        return Ok(());
    }

    //
    // Reject inputs that lie outside of [-2*pi, 2*pi], accounting
    // for the tolerance.
    //
    if ((INMIN < (-save.PI2 - TOL)) || (INMIN > (save.PI2 + TOL))) {
        CHKIN(b"ZZNRMLON", ctx)?;
        SETMSG(b"Longitude lower bound INMIN = # (radians),  = # (deg). The minimum allowed value is  -2*pi - TOL = # (radians), = # (deg).", ctx);
        ERRDP(b"#", INMIN, ctx);
        ERRDP(b"#", (INMIN * DPR(ctx)), ctx);
        ERRDP(b"#", (-save.PI2 - TOL), ctx);
        ERRDP(b"#", ((-save.PI2 - TOL) * DPR(ctx)), ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZNRMLON", ctx)?;
        return Ok(());
    }

    //
    // The input bounds may not be equal.
    //
    if (INMIN == INMAX) {
        CHKIN(b"ZZNRMLON", ctx)?;
        SETMSG(
            b"Longitude lower bound INMIN = # (radians),  = # (deg), is equal to upper bound.",
            ctx,
        );
        ERRDP(b"#", INMIN, ctx);
        ERRDP(b"#", (INMIN * DPR(ctx)), ctx);
        SIGERR(b"SPICE(ZEROBOUNDSEXTENT)", ctx)?;
        CHKOUT(b"ZZNRMLON", ctx)?;
        return Ok(());
    }

    //
    // The input longitude is within range or is out of range by at most
    // |TOL| radians. Bracket it.

    *OUTMIN = intrinsics::DMAX1(&[-save.PI2, intrinsics::DMIN1(&[INMIN, save.PI2])]);

    //
    // Same deal for the upper bound.
    //
    if ((INMAX < (-save.PI2 - TOL)) || (INMAX > (save.PI2 + TOL))) {
        CHKIN(b"ZZNRMLON", ctx)?;
        SETMSG(b"Longitude upper bound INMAX = # (radians),  = # (deg). The minimum allowed value is  -2*pi - TOL = # (radians), = # (deg).", ctx);
        ERRDP(b"#", INMAX, ctx);
        ERRDP(b"#", (INMAX * DPR(ctx)), ctx);
        ERRDP(b"#", (-save.PI2 - TOL), ctx);
        ERRDP(b"#", ((-save.PI2 - TOL) * DPR(ctx)), ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZNRMLON", ctx)?;
        return Ok(());
    }

    *OUTMAX = intrinsics::DMAX1(&[-save.PI2, intrinsics::DMIN1(&[INMAX, save.PI2])]);

    //
    // If the bounds are out of order, put them in order.
    // It is assumed that no interval has length zero.
    //
    // If the upper bound is greater than the lower bound by
    // less than TOL, the bounds are considered to be "out of
    // order."
    //
    if (*OUTMAX <= TOUCHD((*OUTMIN + TOL))) {
        //
        // Shift one of the bounds by 2*pi, while keeping
        // the bounds in the range [-2pi, 2pi].
        //
        if (*OUTMAX <= 0.0) {
            //
            // OUTMAX is non-positive. Shift it to the right.
            //
            *OUTMAX = intrinsics::DMIN1(&[TOUCHD((*OUTMAX + save.PI2)), save.PI2]);

            if (*OUTMAX < *OUTMIN) {
                //
                // If the bounds are still out of order, shift the lower
                // bound left.
                //
                *OUTMIN = intrinsics::DMAX1(&[TOUCHD((*OUTMIN - save.PI2)), -save.PI2]);
            }
        } else {
            //
            // OUTMAX is > 0. Shift the lower bound left.
            //
            *OUTMIN = intrinsics::DMAX1(&[TOUCHD((*OUTMIN - save.PI2)), -save.PI2]);
        }
    }

    //
    // If the bounds are too far apart, move them together. Note
    // that OUTMIN and OUTMAX are already set at this point.
    //
    DELTA = TOUCHD((*OUTMAX - *OUTMIN));

    if (DELTA > TOUCHD((save.PI2 + TOL))) {
        //
        // Shift the upper bound lower by 2*pi.
        //
        *OUTMAX = TOUCHD((*OUTMAX - save.PI2));
    }

    //
    // The output bounds must not be equal. We could end up with
    // equal output bounds if the input maximum is less than
    // the input minimum and the bounds differ by an integer
    // multiple of 2*pi.
    //
    if (*OUTMIN == *OUTMAX) {
        CHKIN(b"ZZNRMLON", ctx)?;
        SETMSG(b"After adjustment, input longitude lower bound INMIN = # (radians),  = # (deg), is equal to adjusted longitude upper bound. Input upper bound = # (radians),  = # (deg). When the input upper bound is less than the input lower bound, the difference must not be an integer multiple of 2*pi.", ctx);
        ERRDP(b"#", INMIN, ctx);
        ERRDP(b"#", (INMIN * DPR(ctx)), ctx);
        ERRDP(b"#", INMAX, ctx);
        ERRDP(b"#", (INMAX * DPR(ctx)), ctx);
        SIGERR(b"SPICE(ZEROBOUNDSEXTENT)", ctx)?;
        CHKOUT(b"ZZNRMLON", ctx)?;
        return Ok(());
    }

    Ok(())
}
