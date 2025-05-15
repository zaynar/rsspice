//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Element of a DP window
///
/// Determine whether a point is an element of a double precision
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
///  POINT      I   Input point.
///  WINDOW     I   Input window.
///
///  The function returns .TRUE. if POINT is an element of WINDOW.
/// ```
///
/// # Detailed Input
///
/// ```text
///  POINT    is a point, which may or may not be contained in one of
///           the intervals in WINDOW.
///
///  WINDOW   is a SPICE window containing zero or more intervals.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if the input point is an element of
///  the input window --- that is, if
///
///     a(i)  <  POINT  <  b(i)
///           -         -
///
///  for some interval [ a(i), b(i) ] in WINDOW --- and returns .FALSE.
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
///        WNELMD ( 1, WINDOW )
///        WNELMD ( 9, WINDOW )
///
///  and the following expressions are false.
///
///        WNELMD (  0, WINDOW )
///        WNELMD ( 13, WINDOW )
///        WNELMD ( 29, WINDOW )
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 25-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added entry #1
///         in $Exceptions section.
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
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU) (HAN)
/// ```
pub fn wnelmd(ctx: &mut SpiceContext, point: f64, window: &[f64]) -> crate::Result<bool> {
    let ret = WNELMD(point, window, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure WNELMD ( Element of a DP window )
pub fn WNELMD(POINT: f64, WINDOW: &[f64], ctx: &mut Context) -> f2rust_std::Result<bool> {
    let WINDOW = DummyArray::new(WINDOW, LBCELL..);
    let mut WNELMD: bool = false;
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
        WNELMD = false;
        return Ok(WNELMD);
    } else {
        CHKIN(b"WNELMD", ctx)?;
    }

    //
    // How many endpoints in the window?
    //
    CARD = CARDD(WINDOW.as_slice(), ctx)?;

    //
    // Check the point against every interval in the window. Quit if
    // we find an interval that contains it. Inefficient, but it works.
    //
    for I in intrinsics::range(1, CARD, 2) {
        if ((POINT >= WINDOW[I]) && (POINT <= WINDOW[(I + 1)])) {
            WNELMD = true;
            CHKOUT(b"WNELMD", ctx)?;
            return Ok(WNELMD);
        }
    }

    WNELMD = false;

    CHKOUT(b"WNELMD", ctx)?;
    Ok(WNELMD)
}
