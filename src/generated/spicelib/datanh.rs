//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Double precision arc hyperbolic tangent
///
/// Return the inverse hyperbolic tangent of a double precision
/// argument.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   Number whose inverse hyperbolic tangent is
///                 desired.
///
///  The function returns the inverse hyperbolic tangent of a double
///  precision number.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is any double precision.
///
///           X must be within the range -1 < X < +1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the inverse hyperbolic tangent of the double
///  precision number X.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If X is not between -1.0 and 1.0, the error
///      SPICE(INVALIDARGUMENT) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This function simply implements the definition of the inverse
///  hyperbolic tangent as follows:
///
///     DATANH = 0.5D0 * DLOG ( (1+X) / (1-X) )
///
///  If the input value is not valid, an error is signaled.
/// ```
///
/// # Examples
///
/// ```text
///  The following table gives a few values for X and the resulting
///  value of DATANH.
///
///      X                       DATANH(X)
///     ----------------------------------------------
///     -0.2000000000000000     -0.2027325540540822
///     -0.1000000000000000     -0.1003353477310756
///      0.0000000000000000E+00  0.0000000000000000E+00
///      0.1000000000000000      0.1003353477310756
///      0.2000000000000000      0.2027325540540822
///      0.4000000000000000      0.4236489301936018
///     0.8000000000000000       1.098612288668110
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The value of the input variable X must be between -1.0 and
///      1.0, otherwise an error is signaled.
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
pub fn datanh(ctx: &mut SpiceContext, x: f64) -> crate::Result<f64> {
    let ret = DATANH(x, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure DATANH  ( Double precision arc hyperbolic tangent )
pub fn DATANH(X: f64, ctx: &mut Context) -> f2rust_std::Result<f64> {
    let mut DATANH: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Set up the error processing.
    //
    if RETURN(ctx) {
        DATANH = 0.0;
        return Ok(DATANH);
    } else {
        CHKIN(b"DATANH", ctx)?;
        DATANH = 0.0;
    }

    //
    // Check that -1 < X < +1.
    //
    if (f64::abs(X) >= 1.0) {
        SETMSG(b"DATANH: Argument out of range.", ctx);
        SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;

        CHKOUT(b"DATANH", ctx)?;
        return Ok(DATANH);
    }

    DATANH = (0.5 * f64::ln(((1.0 + X) / (1.0 - X))));

    CHKOUT(b"DATANH", ctx)?;

    Ok(DATANH)
}
