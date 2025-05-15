//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Included in a double precision window
///
/// Determine whether an interval is included in a double precision
/// window.
///
/// # Required Reading
///
/// * [WINDOWS](crate::required_reading::windows)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LEFT,
///  RIGHT      I   Input interval.
///  WINDOW     I   Input window.
///
///  The function returns .TRUE. if POINT is an element of WINDOW.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LEFT,
///  RIGHT    are the endpoints of an interval, which may or may not be
///           contained in one of the intervals in WINDOW.
///
///  WINDOW   is a window containing zero or more intervals.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if the input interval is included in
///  the input window --- that is, if
///
///     a(i)  <  LEFT  <  RIGHT  <  b(i)
///           -        -         -
///
///  for some interval [ a(i), b(i) ] in WINDOW --- and is .FALSE.
///  otherwise.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  The cardinality of the input WINDOW must be even. Left
///      endpoints of stored intervals must be strictly greater than
///      preceding right endpoints. Right endpoints must be greater
///      than or equal to corresponding left endpoints. Invalid window
///      data are not diagnosed by this routine and may lead to
///      unpredictable results.
///
///  2)  The order of the input interval's endpoints, LEFT and RIGHT,
///      is not checked, and that this does not affect the result.
/// ```
///
/// # Examples
///
/// ```text
///  Let A contain the intervals
///
///        [ 1, 3 ]  [ 7, 11 ]  [ 23, 27 ]
///
///  Then the following expressions are true
///
///        WNINCD ( 1.D0,  3.D0, WINDOW )
///        WNINCD ( 9.D0, 10.D0, WINDOW )
///
///  and the following expressions are false.
///
///        WNINCD (  0,  2, WINDOW )
///        WNINCD ( 13, 15, WINDOW )
///        WNINCD ( 29, 30, WINDOW )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 14-MAR-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added entries
///         #1 and #2 in $Exceptions section.
///
///         Removed unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         If the value of the function RETURN is .TRUE. upon execution of
///         this module, this function is assigned a default value of
///         either 0, 0.0D0, .FALSE., or blank depending on the type of
///         the function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU) (HAN)
/// ```
pub fn wnincd(
    ctx: &mut SpiceContext,
    left: f64,
    right: f64,
    window: &[f64],
) -> crate::Result<bool> {
    let ret = WNINCD(left, right, window, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure WNINCD ( Included in a double precision window )
pub fn WNINCD(
    LEFT: f64,
    RIGHT: f64,
    WINDOW: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let WINDOW = DummyArray::new(WINDOW, LBCELL..);
    let mut WNINCD: bool = false;
    let mut CARD: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        WNINCD = false;
        return Ok(WNINCD);
    } else {
        CHKIN(b"WNINCD", ctx)?;
    }

    //
    // How many endpoints in the window?
    //
    CARD = CARDD(WINDOW.as_slice(), ctx)?;

    //
    // Check this interval agains every interval in the window.
    // Inefficient, but foolproof.
    //
    for I in intrinsics::range(1, CARD, 2) {
        if ((LEFT >= WINDOW[I]) && (RIGHT <= WINDOW[(I + 1)])) {
            WNINCD = true;
            CHKOUT(b"WNINCD", ctx)?;
            return Ok(WNINCD);
        }
    }

    WNINCD = false;

    CHKOUT(b"WNINCD", ctx)?;
    Ok(WNINCD)
}
