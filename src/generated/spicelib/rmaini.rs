//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Remainder --- integer
///
/// Compute the integer quotient and non-negative remainder
/// of NUM and DENOM.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NUM        I   Numerator used to compute quotient and remainder.
///  DENOM      I   Denominator used to compute quotient and remainder.
///  Q          O   Integer portion of the quotient NUM/DENOM.
///  REM        O   Remainder of the quotient NUM/DENOM.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NUM      is the numerator of a quotient
///
///  DENOM    is the denominator of a quotient
/// ```
///
/// # Detailed Output
///
/// ```text
///  Q        is the largest integer less than or equal to the
///           quotient NUM/DENOM
///
///  REM      is the remainder of the integer division NUM/DENOM
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If DENOM is zero, the error SPICE(DIVIDEBYZERO) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Given the integer inputs NUM and DENOM, this routine
///  finds integers Q and REM that satisfy the following conditions:
///
///      1) NUM = DENOM * Q + REM
///
///      2) REM is a non negative integer less than the absolute
///         value of DENOM.
///
///  This routine serves as a macro. In this way the code to perform
///  this task can be written and maintained in a single location.
/// ```
///
/// # Examples
///
/// ```text
///  One frequently needs to compute the  ``360 modulus'' of a
///  number. For positive numbers the FORTRAN intrinsic mod
///  function works well. However, for negative numbers the
///  intrinsic will return a negative modulus. This routine
///  can be used to compute the positive 360 pi modulus (MOD360) for
///  any integer I by the call:
///
///      CALL RMAINI ( I, 360, Q, MOD360 )
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 01-DEC-1995 (WLT)
/// ```
pub fn rmaini(
    ctx: &mut SpiceContext,
    num: i32,
    denom: i32,
    q: &mut i32,
    rem: &mut i32,
) -> crate::Result<()> {
    RMAINI(num, denom, q, rem, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RMAINI ( Remainder --- integer )
pub fn RMAINI(
    NUM: i32,
    DENOM: i32,
    Q: &mut i32,
    REM: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // Take care of the zero-denominator case first
    //
    if ((DENOM as f64) == 0.0) {
        CHKIN(b"RMAINI", ctx)?;
        SETMSG(
            b"Attempting to compute a quotient with a divide by zero.",
            ctx,
        );
        SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
        CHKOUT(b"RMAINI", ctx)?;
        return Ok(());
    }

    *Q = (NUM / DENOM);
    *REM = (NUM - (DENOM * *Q));

    if (*REM < 0) {
        *Q = (*Q - 1);
        *REM = (*REM + DENOM);
    }

    Ok(())
}
