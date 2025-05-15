//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Double precision arc hyperbolic cosine
///
/// Return the inverse hyperbolic cosine of a double precision
/// argument.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///   X         I   Number whose inverse hyperbolic cosine is desired.
///
///  The function returns the inverse hyperbolic cosine of a double
///  precision number.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is any double precision number greater than or equal to
///           1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the inverse hyperbolic cosine of the double
///  precision number X.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If X is less than 1.0D0, the error SPICE(INVALIDARGUMENT) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This function simply implements the definition of the inverse
///  hyperbolic cosine as follows:
///
///     DACOSH = DLOG (X + DSQRT (X*X-1.D0))
///
///  If the input value is not valid, an error is signaled.
/// ```
///
/// # Examples
///
/// ```text
///  The following table gives a few values for X and the resulting
///  value of DACOSH.
///
///  X                       DACOSH(X)
///  ----------------------------------------------
///  1.000000000000000      0.0000000000000000E+00
///  10.00000000000000       2.993222846126381
///  100.0000000000000       5.298292365610485
///  1000.000000000000       7.600902209541989
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The value of the input variable X must be greater than or
///      equal to 1.0d0.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  W.H. Beyer, "CRC Standard Mathematical Tables," CRC Press,
///       1987.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.M. Owen          (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         Set the default function value to either 0, 0.0D0, .FALSE.,
///         or blank depending on the type of the function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn dacosh(ctx: &mut SpiceContext, x: f64) -> crate::Result<f64> {
    let ret = DACOSH(x, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure DACOSH ( Double precision arc hyperbolic cosine )
pub fn DACOSH(X: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let mut DACOSH: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Set up the error processing.
    //
    if RETURN(ctx) {
        DACOSH = 0.0;
        return Ok(DACOSH);
    } else {
        CHKIN(b"DACOSH", ctx)?;
        DACOSH = 0.0;
    }

    //
    // Check that X >= 1.
    //

    if (X < 1.0) {
        SETMSG(b"DACOSH: Invalid argument, X is less than one.", ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;

        CHKOUT(b"DACOSH", ctx)?;
        return Ok(DACOSH);
    }
    //
    // Abiding by the order implied by the parentheses in the expression
    // (1.0D0/X)/X prevents floating point overflow that might occur for
    // large values of X if the equivalent expression, 1.0D0/(X*X), were
    // used.
    //
    DACOSH = f64::ln((X + (X * f64::sqrt((1.0 - ((1.0 / X) / X))))));

    CHKOUT(b"DACOSH", ctx)?;
    Ok(DACOSH)
}
