//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Complement a DP window
///
/// Determine the complement of a double precision window with
/// respect to the interval \[LEFT,RIGHT].
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
///  RIGHT      I   Left, right endpoints of complement interval.
///  WINDOW     I   Input window.
///  RESULT     O   Complement of WINDOW with respect to [LEFT,RIGHT].
/// ```
///
/// # Detailed Input
///
/// ```text
///  LEFT,
///  RIGHT    are the left and right endpoints of the complement
///           interval.
///
///  WINDOW   is the window to be complemented.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RESULT   is the output window, containing the complement
///           of WINDOW with respect to the interval from LEFT
///           to RIGHT. If the output window is not large enough
///           to contain the result, as many intervals as will
///           fit are returned.
///
///           RESULT must be distinct from WINDOW.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If LEFT is greater than RIGHT, the error SPICE(BADENDPOINTS)
///      is signaled.
///
///  2)  The cardinality of the input WINDOW must be even. Left
///      endpoints of stored intervals must be strictly greater than
///      preceding right endpoints. Right endpoints must be greater
///      than or equal to corresponding left endpoints. Invalid window
///      data are not diagnosed by this routine and may lead to
///      unpredictable results.
/// ```
///
/// # Particulars
///
/// ```text
///  Mathematically, the complement of a window contains those
///  points that are not contained in the window. That is, the
///  complement of the set of closed intervals
///
///       [ a(1), b(1) ], [ a(2), b(2) ], ..., [ a(n), b(n) ]
///
///  is the set of open intervals
///
///       ( -inf, a(1) ), ( b(1), a(2) ), ..., ( b(n), +inf )
///
///  Because Fortran offers no satisfactory representation of
///  infinity, we must take the complement with respect to a
///  finite interval.
///
///  In addition, Fortran offers no satisfactory floating point
///  representation of open intervals. Therefore, the complement
///  of a floating point window is closure of the set theoretical
///  complement. In short, the floating point complement of the
///  window
///
///       [ a(1), b(1) ], [ a(2), b(2) ], ..., [ a(n), b(n) ]
///
///  with respect to the interval from LEFT to RIGHT is the
///  intersection of the windows
///
///       ( -inf, a(1) ], [ b(1), a(2) ], ..., [ b(n), +inf )
///
///  and
///
///       [ LEFT, RIGHT ]
///
///  Note that floating point intervals of measure zero (singleton
///  intervals) in the original window are replaced by gaps of
///  measure zero, which are filled. Thus, complementing a floating
///  point window twice does not necessarily yield the original
///  window.
/// ```
///
/// # Examples
///
/// ```text
///  Let WINDOW contain the intervals
///
///        [ 1, 3 ]  [ 7, 11 ]  [ 23, 27 ]
///
///  Then the floating point complement of WINDOW with respect
///  to [2,20] contains the intervals
///
///        [ 3, 7 ]  [ 11, 20 ]
///
///  and the complement with respect to [ 0, 100 ] contains
///
///        [ 0, 1 ]  [ 3, 7 ]  [ 11, 23 ]  [ 27, 100 ]
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
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Added entry #2 in $Exceptions section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU) (HAN) (NJB)
/// ```
pub fn wncomd(
    ctx: &mut SpiceContext,
    left: f64,
    right: f64,
    window: &[f64],
    result: &mut [f64],
) -> crate::Result<()> {
    WNCOMD(left, right, window, result, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNCOMD ( Complement a DP window )
pub fn WNCOMD(
    LEFT: f64,
    RIGHT: f64,
    WINDOW: &[f64],
    RESULT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let WINDOW = DummyArray::new(WINDOW, LBCELL..);
    let mut RESULT = DummyArrayMut::new(RESULT, LBCELL..);
    let mut CARD: i32 = 0;
    let mut I: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Set up the error processing.
    //
    if RETURN(ctx) {
        return Ok(());
    }
    CHKIN(b"WNCOMD", ctx)?;

    //
    // Get the cardinality of the input window.
    //
    CARD = CARDD(WINDOW.as_slice(), ctx)?;

    //
    // Empty out the result window before proceeding.
    //
    SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    //
    // Check to see if the input interval is valid. If it is not, signal
    // an error and return.
    //
    if (LEFT > RIGHT) {
        SETMSG(b"WNCOMD: Left endpoint may not exceed right endpoint.", ctx);
        SIGERR(b"SPICE(BADENDPOINTS)", ctx)?;
        CHKOUT(b"WNCOMD", ctx)?;
        return Ok(());
    }

    //
    // There are two trivial cases: the window is empty, or it does not
    // intersect the input interval. In either case, the complement is
    // the entire interval.
    //
    if (((CARD == 0) || (WINDOW[1] >= RIGHT)) || (WINDOW[CARD] <= LEFT)) {
        WNINSD(LEFT, RIGHT, RESULT.as_slice_mut(), ctx)?;

        CHKOUT(b"WNCOMD", ctx)?;
        return Ok(());
    }

    //
    // Let WINDOW represent the set of intervals
    //
    //        [a1,b1], [a2,b2], ..., [aN,bN]
    //
    // Then the closure of the complement of WINDOW in the reals is
    //
    //        (-infinity,a1], [b1,a2], [b2,a3], ..., [bN, infinity)
    //
    // Thus the sequence of endpoints of WINDOW is also the sequence
    // of finite endpoints of its complement. Moreover, these endpoints
    // are simply "shifted" from their original positions in WINDOW.
    // This makes finding the complement of WINDOW with respect to
    // a given interval almost trivial.
    //

    //
    // Find the first right not less than the beginning of the input
    // interval.
    //
    I = 2;

    while ((I <= CARD) && (WINDOW[I] < LEFT)) {
        I = (I + 2);
    }

    //
    // If the beginning of the input interval doesn't split an interval
    // in the input window, the complement begins with LEFT.
    //
    if ((I <= CARD) && (WINDOW[(I - 1)] > LEFT)) {
        WNINSD(LEFT, WINDOW[(I - 1)], RESULT.as_slice_mut(), ctx)?;
    }

    //
    // Start schlepping endpoints [b(i),a(i+1)] from the input window
    // to the output window. Stop when we find one of our new right
    // endpoints exceeds the end of the input interval.
    //
    while ((!FAILED(ctx) && (I < CARD)) && (WINDOW[(I + 1)] < RIGHT)) {
        WNINSD(WINDOW[I], WINDOW[(I + 1)], RESULT.as_slice_mut(), ctx)?;
        I = (I + 2);
    }

    //
    // If the end of the input interval doesn't split an interval
    // in the input window, the complement ends with RIGHT.
    //
    if ((I <= CARD) && (WINDOW[I] < RIGHT)) {
        WNINSD(WINDOW[I], RIGHT, RESULT.as_slice_mut(), ctx)?;
    }

    CHKOUT(b"WNCOMD", ctx)?;

    Ok(())
}
