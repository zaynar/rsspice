//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXSIZ: i32 = 25;

//$Procedure      T_DCHEB ( Numeric derivative from Chebyshev fit )
pub fn T_DCHEB(
    FUNC: fn(f64, &mut Context) -> f2rust_std::Result<f64>,
    X: f64,
    DELTA: f64,
    NTERMS: i32,
    NRTAIN: i32,
    F: &mut f64,
    DF: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut COEFFS = StackArray::<f64, 25>::new(1..=MAXSIZ);
    let mut LEFT: f64 = 0.0;
    let mut RIGHT: f64 = 0.0;
    let mut WORK = StackArray::<f64, 25>::new(1..=MAXSIZ);
    let mut X2S = StackArray::<f64, 2>::new(1..=2);
    let mut DEGP: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_DCHEB", ctx)?;

    //
    // Check NTERMS.
    //
    if ((NTERMS < 1) || (NTERMS > MAXSIZ)) {
        spicelib::SETMSG(b"NTERMS must be in the range 1:# but was #.", ctx);
        spicelib::ERRINT(b"#", MAXSIZ, ctx);
        spicelib::ERRINT(b"#", NTERMS, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_DCHEB", ctx)?;
        return Ok(());
    }

    //
    // Check NRTAIN.
    //
    if ((NRTAIN < 1) || (NRTAIN > NTERMS)) {
        spicelib::SETMSG(b"NRTAIN must be in the range 1:# but was #.", ctx);
        spicelib::ERRINT(b"#", NTERMS, ctx);
        spicelib::ERRINT(b"#", NRTAIN, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_DCHEB", ctx)?;
        return Ok(());
    }

    //
    // Check DELTA.
    //
    if (DELTA <= 0.0) {
        spicelib::SETMSG(b"DELTA must be positive but was #.", ctx);
        spicelib::ERRDP(b"#", DELTA, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_DCHEB", ctx)?;
        return Ok(());
    }

    //
    // Set domain bounds for Chebyshev fit:
    //
    LEFT = (X - DELTA);
    RIGHT = (X + DELTA);

    //
    // Set transformation parameters:
    //
    X2S[1] = X;
    X2S[2] = DELTA;

    //
    // Fit N-term Cheby expansion to the input function on [LEFT,
    // RIGHT].
    //
    support::CHBFIT(
        FUNC,
        LEFT,
        RIGHT,
        NTERMS,
        WORK.as_slice_mut(),
        COEFFS.as_slice_mut(),
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_DCHEB", ctx)?;
        return Ok(());
    }

    //
    // Find the first derivative of the expansion at X.
    //
    DEGP = (NRTAIN - 1);
    spicelib::CHBINT(COEFFS.as_slice(), DEGP, X2S.as_slice(), X, F, DF);

    spicelib::CHKOUT(b"T_DCHEB", ctx)?;
    Ok(())
}
