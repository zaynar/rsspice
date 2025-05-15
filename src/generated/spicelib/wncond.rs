//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Contract the intervals of a DP window
///
/// Contract each of the intervals of a double precision window.
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
///  LEFT       I   Amount added to each left endpoint.
///  RIGHT      I   Amount subtracted from each right endpoint.
///  WINDOW    I-O  Window to be contracted.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LEFT     is the amount to be added to the left endpoint of each
///           interval in the input window. The amount LEFT is signed.
///
///  RIGHT    is the amount to be subtracted from the right endpoint of
///           each interval in the window. The amount RIGHT is signed.
///
///  WINDOW   on input, is a window containing zero or more
///           intervals.
/// ```
///
/// # Detailed Output
///
/// ```text
///  WINDOW   on output, is the original window with each of its
///           intervals contracted by LEFT units on the left and
///           RIGHT units on the right.
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
/// # Particulars
///
/// ```text
///  This routine contracts (shortens) each of the intervals in
///  the input window. The adjustments are not necessarily symmetric.
///  That is, LEFT units are added to the left endpoint of each
///  interval, and RIGHT units are subtracted from the right endpoint
///  of each interval, where LEFT and RIGHT may be different.
///
///  Intervals are dropped when they are contracted by amounts
///  greater than their measures.
/// ```
///
/// # Examples
///
/// ```text
///  Let WINDOW contain the intervals
///
///        [ 1, 3 ]  [ 7, 11 ]  [ 23, 27 ]  [ 29, 29 ]
///
///  Then the following series of calls
///
///        CALL WNCOND (  2,  1, WINDOW )              (1)
///        CALL WNCOND ( -2,  2, WINDOW )              (2)
///        CALL WNCOND ( -2, -1, WINDOW )              (3)
///
///  produces the following series of windows
///
///        [ 9, 10 ]  [ 25, 26 ]                       (1)
///        [ 7,  8 ]  [ 23, 24 ]                       (2)
///        [ 5,  9 ]  [ 21, 25 ]                       (3)
///
///  Note that intervals may be "contracted" by negative amounts.
///  In the example above, the second call shifts each interval to
///  the left, while the third call undoes the effect of the first
///  call (without restoring the destroyed intervals).
///
///  Note also that the third call is exactly equivalent to the
///  call
///
///        CALL WNEXPD ( 2, 1, WINDOW )
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
///         Added entry #1 in $Exceptions section. Extended LEFT and RIGHT
///         description in $Detailed_Input.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU) (HAN)
/// ```
pub fn wncond(
    ctx: &mut SpiceContext,
    left: f64,
    right: f64,
    window: &mut [f64],
) -> crate::Result<()> {
    WNCOND(left, right, window, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNCOND ( Contract the intervals of a DP window )
pub fn WNCOND(
    LEFT: f64,
    RIGHT: f64,
    WINDOW: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut WINDOW = DummyArrayMut::new(WINDOW, LBCELL..);

    //
    // Spicelib functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"WNCOND", ctx)?;
    }

    //
    // This is just negative expansion.
    //
    WNEXPD(-LEFT, -RIGHT, WINDOW.as_slice_mut(), ctx)?;

    CHKOUT(b"WNCOND", ctx)?;
    Ok(())
}
