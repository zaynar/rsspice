//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    ZZMULT: f64,
    LOGA: f64,
    LOGB: f64,
    EXPNT: f64,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ZZMULT: f64 = 0.0;
        let mut LOGA: f64 = 0.0;
        let mut LOGB: f64 = 0.0;
        let mut EXPNT: f64 = 0.0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            ZZMULT,
            LOGA,
            LOGB,
            EXPNT,
            FIRST,
        }
    }
}

//$Procedure ZZMULT ( Safer multiplication )
pub fn ZZMULT(A: f64, B: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
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
        save.ZZMULT = 0.0;
        return Ok(save.ZZMULT);
    }

    //
    // Participate in error tracing.
    //
    CHKIN(b"ZZMULT", ctx)?;

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
    // If either A or B equals zero the multiplication is zero.
    //
    if ((A == 0.0) || (B == 0.0)) {
        save.ZZMULT = 0.0;
        CHKOUT(b"ZZMULT", ctx)?;
        return Ok(save.ZZMULT);
    }

    //
    // Calculate base 10 logarithms of the absolute value of the
    // numerator and denominator. Recall the base 10 log of a negative
    // real is a complex number (an irritating reality). Our interest
    // is the magnitude of the result, not the sign.
    //
    // An earlier check returned if A or B equal zero.
    //
    save.LOGA = f64::log10(f64::abs(A));
    save.LOGB = f64::log10(f64::abs(B));

    //
    // Local possible overflow check.
    //
    if ((save.LOGA + save.LOGB) > save.EXPNT) {
        save.ZZMULT = 0.0;
        SETMSG(
            b"Numerical overflow event. Multiplier value, #1, multiplicand value, #2.",
            ctx,
        );
        ERRDP(b"#1", A, ctx);
        ERRDP(b"#2", B, ctx);
        SIGERR(b"SPICE(NUMERICOVERFLOW)", ctx)?;
        CHKOUT(b"ZZMULT", ctx)?;
        return Ok(save.ZZMULT);
    }

    //
    // Local possible underflow check. Accept this may occur,
    // return a zero.
    //
    if ((save.LOGA + save.LOGB) < -(save.EXPNT - 1.0)) {
        save.ZZMULT = 0.0;
        CHKOUT(b"ZZMULT", ctx)?;
        return Ok(save.ZZMULT);
    }

    //
    // This operation should be safe. Probably.
    //
    save.ZZMULT = (A * B);

    CHKOUT(b"ZZMULT", ctx)?;

    Ok(save.ZZMULT)
}
