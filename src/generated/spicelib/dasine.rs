//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// arc sine of bracketed argument
///
/// Compute arc sine of a bracketed argument.
///
/// This routine produces a SPICE error if the |argument| exceeds
/// 1.D0 by more than TOL. If ARG exceeds 1.D0, the argument is
/// evaluated as if it equaled 1.D0, if ARG is less than -1.,
/// the argument is evaluated as if it equaled -1.D0.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ARG        I   Argument to be evaluated.
///  TOL        I   Tolerance.
///
///  The function returns the arc sine of ARG.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ARG      is the arc sine argument that is to be evaluated such
///           that if it is less than -1.D0 by more than TOL or greater
///           than 1.D0 by more than TOL, an error results.
///
///  TOL      is a tolerance such that |ARG| is considered to be equal
///           to 1.D0 if |ARG| <= 1.D0 + TOL. TOL must be non-negative.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the arc sine of ARG. If
///
///     |ARG| >= 1.D0,
///
///  it returns DASIN (1.D0) or DASIN (-1.D0) as appropriate. Values
///  range from -PI/2 to PI/2.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If TOL is less than zero, the error SPICE(VALUEOUTOFRANGE) is
///      signaled.
///
///  2)  If |ARG| > 1.D0 + TOL, the error SPICE(INPUTOUTOFBOUNDS) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine determines whether |ARG| > 1.D0 + TOL. If it is, an
///  error will be flagged. In addition, the values of ARG are
///  constrained to [-1.D0, 1.D0].
/// ```
///
/// # Examples
///
/// ```text
///  The following illustrate the operation of DASINE.
///
///        DASINE (  -1.D0,        1.D-7 )  =  -PI/2
///        DASINE (  -1.00001D0,   1.D-3 )  =  -PI/2
///        DASINE (  -1.00001D0,   1.D-7 )  =  -PI/2 (error flagged)
///        DASINE (   0.D0,        1.D-7 )  =   0.D0
///        DASINE (   1.00001D0,   1.D-3 )  =   PI/2
///        DASINE (   1.00001D0,   1.D-7 )  =   PI/2 (error flagged)
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  L.S. Elson         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 28-FEB-2006 (LSE)
/// ```
pub fn dasine(ctx: &mut SpiceContext, arg: f64, tol: f64) -> crate::Result<f64> {
    let ret = DASINE(arg, tol, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure DASINE (arc sine of bracketed argument)
pub fn DASINE(ARG: f64, TOL: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let mut DASINE: f64 = 0.0;

    //
    // Bracket ARG.
    //

    DASINE = f64::asin(intrinsics::DMAX1(&[-1.0, intrinsics::DMIN1(&[1.0, ARG])]));

    //
    // Check that tolerance is non negative.
    //

    if (TOL < 0.0) {
        CHKIN(b"DASINE", ctx)?;
        SETMSG(b"TOL was #; must be non-negative.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"DASINE", ctx)?;
        return Ok(DASINE);
    }

    //
    // Check to see if |ARG| is within TOL of 1.D0. Signal error if
    // appropriate.
    //

    if ((f64::abs(ARG) - TOL) > 1.0) {
        CHKIN(b"DASINE", ctx)?;
        SETMSG(b"The |argument| specified was greater than 1.D0 by more than #. The value of the argument is #. ", ctx);
        ERRDP(b"#", TOL, ctx);
        ERRDP(b"#", ARG, ctx);
        SIGERR(b"SPICE(INPUTOUTOFBOUNDS)", ctx)?;
        CHKOUT(b"DASINE", ctx)?;
        return Ok(DASINE);
    }

    Ok(DASINE)
}
