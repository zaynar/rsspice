//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Expand the intervals of a DP window
///
/// Expand each of the intervals of a double precision window.
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
///  LEFT       I   Amount subtracted from each left endpoint.
///  RIGHT      I   Amount added to each right endpoint.
///  WINDOW    I-O  Window to be expanded.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LEFT     is the amount to be subtracted from the left endpoint of
///           each interval in the input window. The amount LEFT is
///           signed.
///
///  RIGHT    is the amount to be added to the right endpoint of each
///           interval in the window. The amount RIGHT is signed.
///
///  WINDOW   on input, is a window containing zero or more
///           intervals.
/// ```
///
/// # Detailed Output
///
/// ```text
///  WINDOW   on output, is the original window with each of its
///           intervals expanded by LEFT units on the left and
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
///  This routine expands (lengthens) each of the intervals in
///  the input window. The adjustments are not necessarily symmetric.
///  That is, LEFT units are subtracted from the left endpoint of
///  each interval, and RIGHT units are added to the right endpoint
///  of each interval, where LEFT and RIGHT may be different.
///
///  Intervals are merged when expansion causes them to overlap.
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
///        CALL WNEXPD (  2,  1, WINDOW )              (1)
///        CALL WNEXPD ( -2,  2, WINDOW )              (2)
///        CALL WNEXPD ( -2, -1, WINDOW )              (3)
///
///  produces the following series of windows
///
///        [ -1, 4 ]  [ 5, 12 ]  [ 21, 30 ]            (1)
///        [  1, 6 ]  [ 7, 14 ]  [ 23, 32 ]            (2)
///        [  3, 5 ]  [ 9, 13 ]  [ 25, 31 ]            (3)
///
///  Note that intervals may be "expanded" by negative amounts.
///  In the example above, the second call shifts each interval to
///  the right, while the third call undoes the effect of the first
///  call (without restoring the merged intervals).
///
///  Note also that the third call is exactly equivalent to the
///  call
///
///        CALL WNCOND ( 2, 1, WINDOW )
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
pub fn wnexpd(
    ctx: &mut SpiceContext,
    left: f64,
    right: f64,
    window: &mut [f64],
) -> crate::Result<()> {
    WNEXPD(left, right, window, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNEXPD ( Expand the intervals of a DP window )
pub fn WNEXPD(
    LEFT: f64,
    RIGHT: f64,
    WINDOW: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut WINDOW = DummyArrayMut::new(WINDOW, LBCELL..);
    let mut CARD: i32 = 0;
    let mut GONE: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;

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
        return Ok(());
    } else {
        CHKIN(b"WNEXPD", ctx)?;
    }

    //
    // Get the cardinality of the window. (The size is not important;
    // this routine can't create any new intervals.)
    //
    CARD = CARDD(WINDOW.as_slice(), ctx)?;

    //
    // Expand the intervals individually. We'll take care of
    // overlaps later on. Negative expansion may cause some
    // intervals to disappear.
    //
    GONE = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = CARD;
        let m3__: i32 = 2;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            WINDOW[(I - GONE)] = (WINDOW[I] - LEFT);
            WINDOW[((I - GONE) + 1)] = (WINDOW[(I + 1)] + RIGHT);

            if (WINDOW[(I - GONE)] > WINDOW[((I - GONE) + 1)]) {
                GONE = (GONE + 2);
            }
            I += m3__;
        }
    }

    //
    // Proceed only if at least one interval remains. (If there were
    // no intervals to begin with, we skip the previous loop and come
    // here without delay. Do not pass GO, do not collect $200.)
    //
    CARD = (CARD - GONE);

    if (CARD == 0) {
        SCARDD(0, WINDOW.as_slice_mut(), ctx)?;
        CHKOUT(b"WNEXPD", ctx)?;
        return Ok(());
    }

    //
    // None of the intervals can have extended to completely contain
    // any of the other intervals. (They were all expanded by the
    // same amount. Convince yourself that this is true.) So the first
    // endpoint is still the first endpoint (so to speak).
    //
    // Step through the window, looking for the next right endpoint
    // less than the following left endpoint. This marks the end of
    // the new first interval, and the beginning of the new second
    // interval. Keep this up until the last right endpoint has been
    // reached. This remains the last right endpoint.
    //
    I = 2;
    J = 2;

    while (J < CARD) {
        if (WINDOW[J] < WINDOW[(J + 1)]) {
            WINDOW[I] = WINDOW[J];
            WINDOW[(I + 1)] = WINDOW[(J + 1)];
            I = (I + 2);
        }

        J = (J + 2);
    }

    WINDOW[I] = WINDOW[J];
    SCARDD(I, WINDOW.as_slice_mut(), ctx)?;

    CHKOUT(b"WNEXPD", ctx)?;
    Ok(())
}
