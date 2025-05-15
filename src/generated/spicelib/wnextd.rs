//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Extract the endpoints from a DP window
///
/// Extract the left or right endpoints from a double precision
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
///  SIDE       I   Extract left ('L') or right ('R') endpoints.
///  WINDOW    I-O  Window to be extracted.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SIDE     indicates whether the left or right endpoints of the
///           intervals in the window are to be extracted.
///
///              'L', 'l'       Left endpoints.
///              'R', 'r'       Right endpoints.
///
///           If SIDE is not recognized, the input window is not
///           changed.
///
///  WINDOW   on input, is a window containing zero or more intervals.
/// ```
///
/// # Detailed Output
///
/// ```text
///  WINDOW   on output, is the collection of singleton intervals
///           containing either the left or the right endpoints of the
///           intervals in the original window.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the endpoint specification, SIDE, is not recognized, the
///      error SPICE(INVALIDENDPNTSPEC) is signaled.
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
///  This routine replaces every interval in the input window with
///  the singleton interval containing one of the endpoints of the
///  interval.
/// ```
///
/// # Examples
///
/// ```text
///  Let WINDOW contain the intervals
///
///        [ 1, 3 ]  [ 7, 11 ]  [ 23, 27 ]  [ 29, 29 ]
///
///  Then the call
///
///        CALL WNEXTD (  'L', WINDOW )
///
///  produces the window
///
///        [ 1, 1 ]  [ 7, 7 ]  [ 23, 23 ]  [ 29, 29 ]
///
///  And the call
///
///        CALL WNEXTD ( 'R', WINDOW )
///
///  produces the window
///
///        [ 3, 3 ]  [ 11, 11 ]  [ 27, 27 ]  [ 29, 29 ]
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
/// -    SPICELIB Version 1.1.0, 14-MAR-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added entry #2
///         in $Exceptions section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.2.0, 24-FEB-1989  (HAN)
///
///         Added calls to CHKIN and CHKOUT. Error handling was added to
///         detect invalid endpoint specification. The previous version
///         did not signal an error if SIDE was not 'R', 'r', 'L', or 'l'.
/// ```
pub fn wnextd(ctx: &mut SpiceContext, side: char, window: &mut [f64]) -> crate::Result<()> {
    WNEXTD(&[u8::try_from(side).unwrap()], window, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNEXTD ( Extract the endpoints from a DP window )
pub fn WNEXTD(SIDE: &[u8], WINDOW: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let SIDE = &SIDE[..1];
    let mut WINDOW = DummyArrayMut::new(WINDOW, LBCELL..);
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
        return Ok(());
    } else {
        CHKIN(b"WNEXTD", ctx)?;
    }

    //
    // Get the cardinality of the window. (The size is not important;
    // this routine can't create any new intervals.)
    //
    CARD = CARDD(WINDOW.as_slice(), ctx)?;

    //
    // Step through the window, keeping one endpoint from each interval.
    // For the sake of efficiency, we have separate loops for the two
    // possible values of SIDE.
    //
    if (fstr::eq(SIDE, b"L") || fstr::eq(SIDE, b"l")) {
        for I in intrinsics::range(1, CARD, 2) {
            WINDOW[(I + 1)] = WINDOW[I];
        }
    } else if (fstr::eq(SIDE, b"R") || fstr::eq(SIDE, b"r")) {
        for I in intrinsics::range(1, CARD, 2) {
            WINDOW[I] = WINDOW[(I + 1)];
        }
    } else {
        SETMSG(b"SIDE was *.", ctx);
        ERRCH(b"*", SIDE, ctx);
        SIGERR(b"SPICE(INVALIDENDPNTSPEC)", ctx)?;
    }

    CHKOUT(b"WNEXTD", ctx)?;
    Ok(())
}
