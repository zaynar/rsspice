//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Validate a DP window
///
/// Form a valid double precision window from the contents
/// of a window array.
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
///  SIZE       I   Size of window.
///  N          I   Original number of endpoints.
///  WINDOW    I-O  Input, output window.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SIZE     is the size of the window to be validated. This is the
///           maximum number of endpoints that the cell used to
///           implement the window is capable of holding at any one
///           time.
///
///  N        is the original number of endpoints in the input cell.
///
///  WINDOW   on input is a (possibly uninitialized) cell array of
///           maximum size SIZE containing N endpoints of (possibly
///           unordered and non-disjoint) intervals.
/// ```
///
/// # Detailed Output
///
/// ```text
///  WINDOW   on output is a validated window, in which any overlapping
///           input intervals have been merged and the resulting set of
///           intervals is arranged in increasing order.
///
///           WINDOW is ready for use with any of the window routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the original number of endpoints N is odd, the error
///      SPICE(UNMATCHENDPTS) is signaled.
///
///  2)  If the original number of endpoints of the window exceeds its
///      size, the error SPICE(WINDOWTOOSMALL) is signaled.
///
///  3)  If the left endpoint is greater than the right endpoint, the
///      error SPICE(BADENDPOINTS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine takes as input a cell array containing pairs of
///  endpoints and validates it to form a window.
///
///  On input, WINDOW is a cell of size SIZE containing N endpoints.
///  During validation, the intervals are ordered, and overlapping
///  intervals are merged. On output, the cardinality of WINDOW is
///  the number of endpoints remaining, and it is ready for use with
///  any of the window routines.
///
///  Because validation is done in place, there is no chance of
///  overflow.
///
///  Validation is primarily useful for ordering and merging
///  intervals read from input files or initialized in DATA
///  statements.
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
///  1) Define an array containing a set of unordered and possibly
///     overlapping intervals, and validate the array as a SPICE
///     window.
///
///
///     Example code begins here.
///
///
///           PROGRAM WNVALD_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           INTEGER               CARDD
///           INTEGER               SIZED
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               WINSIZ
///           PARAMETER           ( WINSIZ = 20 )
///
///           INTEGER               DATSIZ
///           PARAMETER           ( DATSIZ = 16 )
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      WINDOW  ( LBCELL : WINSIZ )
///           DOUBLE PRECISION      WINDAT  ( DATSIZ )
///
///           INTEGER               I
///
///
///           DATA                  WINDAT  /  0,  0,
///          .                                10, 12,
///          .                                 2,  7,
///          .                                13, 15,
///          .                                 1,  5,
///          .                                23, 29,  4*0 /
///
///
///     C
///     C     Insert the data into the SPICE cell array.
///     C
///           CALL MOVED ( WINDAT, WINSIZ, WINDOW(1) )
///
///     C
///     C     Validate the input WINDOW array as a SPICE window.
///     C
///           CALL WNVALD ( WINSIZ, DATSIZ, WINDOW )
///
///           WRITE (*,*) 'Current intervals: ', CARDD ( WINDOW ) / 2
///           WRITE (*,*) 'Maximum intervals: ', SIZED ( WINDOW ) / 2
///           WRITE (*,*)
///           WRITE (*,*) 'Intervals:'
///           WRITE (*,*)
///
///           DO I = 1, CARDD ( WINDOW ), 2
///              WRITE (*,*) WINDOW(I), WINDOW(I+1)
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Current intervals:            5
///      Maximum intervals:           10
///
///      Intervals:
///
///        0.0000000000000000        0.0000000000000000
///        1.0000000000000000        7.0000000000000000
///        10.000000000000000        12.000000000000000
///        13.000000000000000        15.000000000000000
///        23.000000000000000        29.000000000000000
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
/// -    SPICELIB Version 1.2.0, 16-MAR-2021 (JDR)
///
///         Changed argument name A to WINDOW for consistency with other
///         routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply to NAIF standard. Created complete
///         code example from code fragment and added example's problem
///         statement.
///
///         Improved description of argument WINDOW in $Detailed_Output.
///
///         Removed unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.1.1, 30-JUL-2002 (NJB)
///
///         Fixed bugs in example program.
///
/// -    SPICELIB Version 1.1.0, 14-AUG-1995 (HAN)
///
///         Fixed a character string that continued over two lines.
///         The "//" characters were missing. The Alpha/OpenVMS compiler
///         issued a warning regarding this incorrect statement syntax.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn wnvald(ctx: &mut SpiceContext, size: i32, n: i32, window: &mut [f64]) -> crate::Result<()> {
    WNVALD(size, n, window, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNVALD ( Validate a DP window )
pub fn WNVALD(SIZE: i32, N: i32, WINDOW: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut WINDOW = DummyArrayMut::new(WINDOW, LBCELL..);
    let mut LEFT: f64 = 0.0;
    let mut RIGHT: f64 = 0.0;
    let mut I: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Setting up error processing.
    //
    if RETURN(ctx) {
        return Ok(());
    }
    CHKIN(b"WNVALD", ctx)?;

    //
    // First, some error checks. The number of endpoints must be even,
    // and smaller than the reported size of the window.
    //
    if ODD(N) {
        SETMSG(b"WNVALD: Unmatched endpoints", ctx);
        SIGERR(b"SPICE(UNMATCHENDPTS)", ctx)?;
        CHKOUT(b"WNVALD", ctx)?;
        return Ok(());
    } else if (N > SIZE) {
        SETMSG(b"WNVALD: Inconsistent value for SIZE.", ctx);
        SIGERR(b"SPICE(WINDOWTOOSMALL)", ctx)?;
        CHKOUT(b"WNVALD", ctx)?;
        return Ok(());
    }

    //
    // Taking the easy way out, we will simply insert each new interval
    // as we happen upon it. We can do this safely in place. The output
    // window can't possibly contain more intervals than the input array.
    //
    // What can go wrong is this: a left endpoint might be greater than
    // the corresponding left endpoint. This is a boo-boo, and should be
    // reported.
    //
    SSIZED(SIZE, WINDOW.as_slice_mut(), ctx)?;
    SCARDD(0, WINDOW.as_slice_mut(), ctx)?;

    I = 1;

    while (I < N) {
        LEFT = WINDOW[I];
        RIGHT = WINDOW[(I + 1)];

        if (LEFT > RIGHT) {
            SETMSG(b"WNVALD: Left endpoint may not exceed right endpoint.", ctx);
            SIGERR(b"SPICE(BADENDPOINTS)", ctx)?;
            CHKOUT(b"WNVALD", ctx)?;
            return Ok(());
        }

        WNINSD(LEFT, RIGHT, WINDOW.as_slice_mut(), ctx)?;
        I = (I + 2);
    }

    CHKOUT(b"WNVALD", ctx)?;

    Ok(())
}
