//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Filter small intervals from a DP window
///
/// Filter (remove) small intervals from a double precision window.
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
///  SMLINT     I   Limiting measure of small intervals.
///  WINDOW    I-O  Window to be filtered.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SMLINT   is the limiting measure of the small intervals to be
///           filtered. Intervals of measure less than or equal to
///           SMLINT are removed from the window. The measure SMLINT is
///           signed, and is used as is---the absolute value of SMLINT
///           is not used for in place of negative input values.
///
///  WINDOW   on input, is a window containing zero or more
///           intervals.
/// ```
///
/// # Detailed Output
///
/// ```text
///  WINDOW   on output, is the original window, after small
///           intervals have been removed.
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
///  2)  If SMLINT is less than or equal to zero, this routine has
///      no effect on the window.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine removes from the input window every interval with
///  measure less than or equal to the limiting measure (SMLINT).
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///  1) Given a double precision window, containing the following four
///     intervals:
///
///        [ 1.0, 3.0 ], [ 7.0, 11.0 ], [ 23.0, 27.0 ], [ 29.0, 29.0 ]
///
///     remove any intervals equal to or less than 3.0.
///
///
///     Example code begins here.
///
///
///           PROGRAM WNFLTD_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         FMT
///           PARAMETER           ( FMT    = '(A,I2,A,2(F7.3,A))' )
///
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               WNSIZE
///           PARAMETER           ( WNSIZE = 10 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      LEFT
///           DOUBLE PRECISION      RIGHT
///           DOUBLE PRECISION      WINDOW ( LBCELL:WNSIZE )
///
///           INTEGER               I
///
///     C
///     C     Validate the WINDOW with size WNSIZE and zero elements.
///     C
///           CALL WNVALD ( WNSIZE, 0, WINDOW )
///
///     C
///     C     Insert the intervals
///     C
///     C        [ 1, 3 ]  [ 7, 11 ]  [ 23, 27 ]  [ 29, 29 ]
///     C
///     C     into WINDOW.
///     C
///           CALL WNINSD (  1.0D0,  3.0D0, WINDOW )
///           CALL WNINSD (  7.0D0, 11.0D0, WINDOW )
///           CALL WNINSD ( 23.0D0, 27.0D0, WINDOW )
///           CALL WNINSD ( 29.0D0, 29.0D0, WINDOW )
///
///     C
///     C     Loop over the number of intervals in WINDOW, output
///     C     the LEFT and RIGHT endpoints for each interval.
///     C
///           WRITE(*,*) 'Initial WINDOW:'
///
///           DO I= 1, WNCARD( WINDOW )
///
///              CALL WNFETD ( WINDOW, I, LEFT, RIGHT )
///
///              WRITE(*,FMT) '   Interval ', I, ': [',
///          .                LEFT, ',', RIGHT, ' ]'
///
///           END DO
///
///     C
///     C     Filter the intervals smaller than or equal to 3.0
///     C
///           CALL WNFLTD ( 3.0D0, WINDOW )
///
///     C
///     C     Output the intervals.
///     C
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Window after filtering intervals <= 3.0:'
///
///           DO I= 1, WNCARD( WINDOW )
///
///              CALL WNFETD ( WINDOW, I, LEFT, RIGHT )
///
///              WRITE(*,FMT) '   Interval ', I, ': [',
///          .                LEFT, ',', RIGHT, ' ]'
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
///      Initial WINDOW:
///        Interval  1: [  1.000,  3.000 ]
///        Interval  2: [  7.000, 11.000 ]
///        Interval  3: [ 23.000, 27.000 ]
///        Interval  4: [ 29.000, 29.000 ]
///
///      Window after filtering intervals <= 3.0:
///        Interval  1: [  7.000, 11.000 ]
///        Interval  2: [ 23.000, 27.000 ]
///
///
///  2) Using the same window from the first example:
///
///        [ 1.0, 3.0 ], [ 7.0, 11.0 ], [ 23.0, 27.0 ], [ 29.0, 29.0 ]
///
///     Then the following series of calls
///
///        CALL WNFLTD (  0.D0, WINDOW )                           (1)
///        CALL WNFLTD (  2.D0, WINDOW )                           (2)
///        CALL WNFLTD (  3.D0, WINDOW )                           (3)
///
///     produces the following series of windows
///
///        [ 1.0, 3.0 ]   [ 7.0, 11.0 ]  [ 23.0, 27.0 ]            (1)
///                       [ 7.0, 11.0 ]  [ 23.0, 27.0 ]            (2)
///                       [ 7.0, 11.0 ]  [ 23.0, 27.0 ]            (3)
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
/// -    SPICELIB Version 1.1.0, 05-JUL-2021 (JDR) (NJB)
///
///         Changed the argument name SMALL to SMLINT for consistency with
///         other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Added complete code example. Added entry #1 and #2 in
///         $Exceptions section. Extended SMLINT description in
///         $Detailed_Input.
///
///         Updated code to remove unnecessary lines of code in the
///         Standard SPICE error handling CHKIN statements.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU) (HAN)
/// ```
pub fn wnfltd(ctx: &mut SpiceContext, smlint: f64, window: &mut [f64]) -> crate::Result<()> {
    WNFLTD(smlint, window, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNFLTD ( Filter small intervals from a DP window )
pub fn WNFLTD(SMLINT: f64, WINDOW: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut WINDOW = DummyArrayMut::new(WINDOW, LBCELL..);
    let mut CARD: i32 = 0;
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
    }

    CHKIN(b"WNFLTD", ctx)?;

    //
    // Get the cardinality of the window. (The size is not important;
    // this routine can't create any new intervals.)
    //
    CARD = CARDD(WINDOW.as_slice(), ctx)?;

    //
    // Step through the window, looking for the next interval big
    // enough to get stuck in the filter. Keep this up until the last
    // interval has been checked.
    //
    I = 0;
    J = 2;

    while (J <= CARD) {
        if ((WINDOW[J] - WINDOW[(J - 1)]) > SMLINT) {
            I = (I + 2);
            WINDOW[(I - 1)] = WINDOW[(J - 1)];
            WINDOW[I] = WINDOW[J];
        }

        J = (J + 2);
    }

    SCARDD(I, WINDOW.as_slice_mut(), ctx)?;

    CHKOUT(b"WNFLTD", ctx)?;
    Ok(())
}
