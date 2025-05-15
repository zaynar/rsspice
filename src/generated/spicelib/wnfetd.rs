//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Fetch an interval from a DP window
///
/// Fetch a particular interval from a double precision window.
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
///  WINDOW     I   Input window.
///  N          I   Index of interval to be fetched.
///  LEFT,
///  RIGHT      O   Left, right endpoints of the Nth interval.
/// ```
///
/// # Detailed Input
///
/// ```text
///  WINDOW   is a window containing zero or more intervals.
///
///  N        is the index of a particular interval within the window.
///           Indices range from 1 to NINT, where NINT is the number of
///           intervals in the window (CARDD(WINDOW)/2).
/// ```
///
/// # Detailed Output
///
/// ```text
///  LEFT,
///  RIGHT    are the left and right endpoints of the N'th interval in
///           the input window. If the interval is not found, LEFT and
///           RIGHT are not defined.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If N is less than one, the error SPICE(NOINTERVAL) is
///      signaled.
///
///  2)  If the interval does not exist, i.e. N > CARDD(WINDOW)/2, the
///      error SPICE(NOINTERVAL) is signaled.
///
///  3)  The cardinality of the input WINDOW must be even. Left
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
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///  1) The following code example demonstrates how to insert an
///     interval into an existing double precision SPICE window, and
///     how to loop over all its intervals to extract their left and
///     right points.
///
///
///     Example code begins here.
///
///
///           PROGRAM WNFETD_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               WNSIZE
///           PARAMETER           ( WNSIZE = 10 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      WINDOW      ( LBCELL:WNSIZE )
///           DOUBLE PRECISION      LEFT
///           DOUBLE PRECISION      RIGHT
///
///           INTEGER               I
///
///     C
///     C     Validate the window with size WNSIZE and zero elements.
///     C
///           CALL WNVALD( WNSIZE, 0, WINDOW )
///
///     C
///     C     Insert the intervals
///     C
///     C        [ 1, 3 ]  [ 7, 11 ]  [ 23, 27 ]
///     C
///     C     into WINDOW.
///     C
///           CALL WNINSD(  1.D0,  3.D0, WINDOW )
///           CALL WNINSD(  7.D0, 11.D0, WINDOW )
///           CALL WNINSD( 23.D0, 27.D0, WINDOW )
///
///     C
///     C     Loop over the number of intervals in WINDOW, output
///     C     the LEFT and RIGHT endpoints for each interval.
///     C
///           DO I=1, WNCARD(WINDOW)
///
///              CALL WNFETD( WINDOW, I, LEFT, RIGHT )
///
///              WRITE(*,'(A,I2,2(A,F8.3),A)') 'Interval', I,
///          .                  ' [', LEFT,',',  RIGHT, ']'
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Interval 1 [   1.000,   3.000]
///     Interval 2 [   7.000,  11.000]
///     Interval 3 [  23.000,  27.000]
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 15-MAR-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example, problem statement and solution. Added entry #3 in
///         $Exceptions section.
///
///         Improved description of argument N in $Detailed_Input.
///
/// -    SPICELIB Version 1.0.2, 30-JUL-2007 (EDW)
///
///         Removed erroneous description in the $Examples section
///         indicating "Undefined" as a return state after an error
///         event caused by an invalid value of N.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn wnfetd(
    ctx: &mut SpiceContext,
    window: &[f64],
    n: i32,
    left: &mut f64,
    right: &mut f64,
) -> crate::Result<()> {
    WNFETD(window, n, left, right, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNFETD ( Fetch an interval from a DP window )
pub fn WNFETD(
    WINDOW: &[f64],
    N: i32,
    LEFT: &mut f64,
    RIGHT: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let WINDOW = DummyArray::new(WINDOW, LBCELL..);
    let mut CARD: i32 = 0;
    let mut END: i32 = 0;

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
    CHKIN(b"WNFETD", ctx)?;

    //
    //
    // How many endpoints in the window? Enough? Normally, endpoints
    // of the Nth interval are stored in elements 2N and 2N-1.
    //
    CARD = CARDD(WINDOW.as_slice(), ctx)?;
    END = (2 * N);

    if ((N < 1) || (CARD < END)) {
        SETMSG(b"WNFETD: No such interval.", ctx);
        SIGERR(b"SPICE(NOINTERVAL)", ctx)?;
    } else {
        *LEFT = WINDOW[(END - 1)];
        *RIGHT = WINDOW[END];
    }

    CHKOUT(b"WNFETD", ctx)?;

    Ok(())
}
