//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Remainder --- double precision
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
///  Given the double precision inputs NUM and DENOM, this routine
///  finds double precision numbers Q and REM that satisfy the
///  following conditions:
///
///      1) NUM = DENOM * Q + REM
///
///      2) DENOM has integer value.
///
///      3) REM belongs to the half open interval [0, ABS(DENOM) )
///
///  This routine serves as a macro. In this way the code to perform
///  this task can be written and maintained in a single location.
/// ```
///
/// # Examples
///
/// ```text
///  One frequently needs to compute the  ``Two pi modulus'' of a
///  number. For positive numbers the FORTRAN intrinsic mod
///  function works well. However, for negative numbers the
///  intrinsic will return a negative modulus. This routine
///  can be used to compute the positive two pi modulus (MOD2PI) for
///  any number X by the call:
///
///      CALL RMAIND ( X, TWOPI(), I, MOD2PI )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Arithmetic overflows are not trapped or detected by this
///      routine.
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
pub fn rmaind(
    ctx: &mut SpiceContext,
    num: f64,
    denom: f64,
    q: &mut f64,
    rem: &mut f64,
) -> crate::Result<()> {
    RMAIND(num, denom, q, rem, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure RMAIND ( Remainder --- double precision )
pub fn RMAIND(
    NUM: f64,
    DENOM: f64,
    Q: &mut f64,
    REM: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut MYNUM: f64 = 0.0;
    let mut MYDNOM: f64 = 0.0;

    //
    // Take care of the zero-denominator case first
    //
    if (DENOM == 0.0) {
        CHKIN(b"RMAIND", ctx)?;
        SETMSG(
            b"Attempting to compute a quotient with a divide by zero.",
            ctx,
        );
        SIGERR(b"SPICE(DIVIDEBYZERO)", ctx)?;
        CHKOUT(b"RMAIND", ctx)?;
        return Ok(());
    }

    MYDNOM = DENOM;
    MYNUM = NUM;

    *Q = f64::trunc((MYNUM / MYDNOM));
    *REM = (MYNUM - (*Q * MYDNOM));

    if (*REM < 0.0) {
        *Q = (*Q - 1.0);
        *REM = (*REM + MYDNOM);
    }

    Ok(())
}
