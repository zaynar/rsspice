//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Cardinality of a double precision window
///
/// Return the cardinality (number of intervals) of a double
/// precision window.
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
///
///  The function returns the cardinality of the input window.
/// ```
///
/// # Detailed Input
///
/// ```text
///  WINDOW   is a window containing zero or more intervals.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the cardinality of (number of intervals in)
///  the input window.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the number of elements in WINDOW is not even,
///      the error SPICE(INVALIDSIZE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The window cardinality (WNCARD) function simply wraps a CARD call
///  then divides the result by 2. A common error when using the SPICE
///  windows function is to use the CARDD value as the number of
///  window intervals rather than the CARDD/2 value.
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
///           PROGRAM WNCARD_EX1
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
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Updated to remove unnecessary lines of code in the
///         Standard SPICE error handling CHKIN statements.
///
///         Edited the header to comply to NAIF standard. Created complete
///         code example from code fragment and added example's problem
///         statement.
///
/// -    SPICELIB Version 1.0.1, 24-APR-2010 (EDW)
///
///         Minor edit to code comments eliminating typo.
///
/// -    SPICELIB Version 1.0.0, 10-AUG-2007 (EDW)
/// ```
pub fn wncard(ctx: &mut SpiceContext, window: &[f64]) -> crate::Result<i32> {
    let ret = WNCARD(window, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure WNCARD ( Cardinality of a double precision window )
pub fn WNCARD(WINDOW: &[f64], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let WINDOW = DummyArray::new(WINDOW, LBCELL..);
    let mut WNCARD: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        WNCARD = 0;
        return Ok(WNCARD);
    }

    CHKIN(b"WNCARD", ctx)?;

    WNCARD = CARDD(WINDOW.as_slice(), ctx)?;

    //
    // Confirm the cardinality as an even integer.
    //
    if !EVEN(WNCARD) {
        SETMSG(b"Invalid window size, a window should have an even number of elements. The size was #.", ctx);
        ERRINT(b"#", WNCARD, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"WNCARD", ctx)?;
        WNCARD = 0;
        return Ok(WNCARD);
    }

    //
    // Set return value. Cardinality in a SPICE window sense
    // means the number of intervals, half the cell
    // cardinality value.
    //
    WNCARD = (WNCARD / 2);

    CHKOUT(b"WNCARD", ctx)?;
    Ok(WNCARD)
}
