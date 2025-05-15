//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    ZZDIV: f64,
    LOGNUM: f64,
    LOGDEN: f64,
    EXPNT: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ZZDIV: f64 = 0.0;
        let mut LOGNUM: f64 = 0.0;
        let mut LOGDEN: f64 = 0.0;
        let mut EXPNT: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            ZZDIV,
            LOGNUM,
            LOGDEN,
            EXPNT,
            FIRST,
        }
    }
}

//$Procedure ZZDIV ( Safer division )
pub fn ZZDIV(NUMR: f64, DENOM: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // The bounds on the potential result of the calculation.
    //

    //
    // First entry flag.
    //

    //
    // Return on error.
    //
    if RETURN(ctx) {
        save.ZZDIV = 0.0;
        return Ok(save.ZZDIV);
    }

    //
    // Participate in error tracing.
    //
    CHKIN(b"ZZDIV", ctx)?;

    //
    // Calculate the bounds parameter on first entry.
    // The double precision maximum value has the form
    // "d*(10**EXPNT)." The value of interest is "EXPNT."
    //
    if save.FIRST {
        save.FIRST = false;

        //
        // A "floor" evaluation.
        //
        save.EXPNT = ((f64::log10(DPMAX()) as i32) as f64);
    }

    //
    // If the denominator is zero, return zero and signal an error.
    // This is equivalent to a signaling NaN (not-a-number) for
    // the 0/0 case.
    //
    if (DENOM == 0.0) {
        save.ZZDIV = 0.0;
        SETMSG(b"Numerical divide by zero event. Numerator value #1.", ctx);
        ERRDP(b"#1", NUMR, ctx);
        SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
        CHKOUT(b"ZZDIV", ctx)?;
        return Ok(save.ZZDIV);
    }

    //
    // If the numerator is zero, the division is zero. DENOM
    // is known non-zero.
    //
    if (NUMR == 0.0) {
        save.ZZDIV = 0.0;
        CHKOUT(b"ZZDIV", ctx)?;
        return Ok(save.ZZDIV);
    }

    //
    // Calculate base 10 logarithms of the absolute value of the
    // numerator and denominator. Recall the base 10 log of a negative
    // real is a complex number (an irritating reality). Our interest
    // is the magnitude of the result, not the sign.
    //
    // An earlier check returned if NUMR or DENOM equals zero, so the
    // LOG10 call is safe from an infinite return value. An infinite
    // return value defeats the purpose of this routine.
    //
    save.LOGNUM = f64::log10(f64::abs(NUMR));
    save.LOGDEN = f64::log10(f64::abs(DENOM));

    //
    // Local possible overflow check.
    //
    if ((save.LOGNUM - save.LOGDEN) > save.EXPNT) {
        save.ZZDIV = 0.0;
        SETMSG(
            b"Numerical overflow event. Numerator value #1, denominator value #2.",
            ctx,
        );
        ERRDP(b"#1", NUMR, ctx);
        ERRDP(b"#2", DENOM, ctx);
        SIGERR(b"SPICE(NUMERICOVERFLOW)", ctx)?;
        CHKOUT(b"ZZDIV", ctx)?;
        return Ok(save.ZZDIV);
    }

    //
    // Local possible underflow check. Accept this may occur,
    // return a zero.
    //
    if ((save.LOGNUM - save.LOGDEN) < -(save.EXPNT - 1.0)) {
        save.ZZDIV = 0.0;
        CHKOUT(b"ZZDIV", ctx)?;
        return Ok(save.ZZDIV);
    }

    //
    // This operation should be safe. Probably.
    //
    save.ZZDIV = (NUMR / DENOM);

    CHKOUT(b"ZZDIV", ctx)?;

    Ok(save.ZZDIV)
}
