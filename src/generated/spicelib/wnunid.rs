//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Union two DP windows
///
/// Place the union of two double precision windows into a third
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
///  A,
///  B          I   Input windows.
///  C          O   Union of A and B.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A,
///  B        are SPICE windows, each of which contains zero or more
///           intervals.
/// ```
///
/// # Detailed Output
///
/// ```text
///  C        is the output SPICE window, containing the union of A
///           and B --- every point contained in A, or in B, or in
///           both.
///
///           C must be distinct from both A and B.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the union of the two windows results in an excess of
///      elements, the error SPICE(WINDOWEXCESS) is signaled.
///
///  2)  The cardinality of the input windows must be even. Left
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
///  The union of two windows contains every point contained in the
///  first window, or the second window, or both.
/// ```
///
/// # Examples
///
/// ```text
///  Let A contain the intervals
///
///        [ 1, 3 ]  [ 7, 11 ]  [ 23, 27 ]
///
///  and B contain the intervals
///
///        [ 2, 6 ]  [ 8, 10 ]  [ 16, 18 ]
///
///  Then the union of A and B contains the intervals
///
///        [ 1, 6 ]  [ 7, 11 ]  [ 16, 18 ]  [ 23, 27 ]
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
/// -    SPICELIB Version 1.2.0, 24-AUG-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Fixed I/O type
///         of argument C in $Brief_I/O table.
///
///         Added entry #2 in $Exceptions section. Removed unnecessary
///         entries in $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 08-FEB-1999 (WLT)
///
///         The variable END was not initialized in the previous
///         edition. It is now initialized to be the minimum of
///         A(1) and B(1).
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
/// -    Beta Version 1.1.0, 27-FEB-1989  (HAN)
///
///         Due to the calling sequence and functionality changes
///         in the routine EXCESS, the method of signaling an
///         excess of elements needed to be changed.
/// ```
pub fn wnunid(ctx: &mut SpiceContext, a: &[f64], b: &[f64], c: &mut [f64]) -> crate::Result<()> {
    WNUNID(a, b, c, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNUNID ( Union two DP windows )
pub fn WNUNID(A: &[f64], B: &[f64], C: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let A = DummyArray::new(A, LBCELL..);
    let B = DummyArray::new(B, LBCELL..);
    let mut C = DummyArrayMut::new(C, LBCELL..);
    let mut ACARD: i32 = 0;
    let mut BCARD: i32 = 0;
    let mut CSIZE: i32 = 0;
    let mut AP: i32 = 0;
    let mut BP: i32 = 0;
    let mut CP: i32 = 0;
    let mut END: f64 = 0.0;
    let mut USE = [b' '; 1];
    let mut OVER: i32 = 0;

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
        CHKIN(b"WNUNID", ctx)?;
    }

    //
    // Find the cardinality of the input windows, and the allowed size
    // of the output window.
    //
    ACARD = CARDD(A.as_slice(), ctx)?;
    BCARD = CARDD(B.as_slice(), ctx)?;
    CSIZE = SIZED(C.as_slice(), ctx)?;

    //
    // Begin with the input pointers at the first elements of the
    // input windows. The initial cardinality of the output window
    // is zero. And there is no overflow so far.
    //
    // (Note that AP and BP point to the LEFT endpoints of intervals
    // in A and B, while CP points to the RIGHT endpoint of the latest
    // interval in C.)
    //
    AP = 1;
    BP = 1;
    CP = 0;
    END = intrinsics::DMIN1(&[A[1], B[1]]);
    OVER = 0;

    //
    // When the ends of both input windows are reached, we're done.
    //
    while ((AP < ACARD) || (BP < BCARD)) {
        //
        // If the end of one window has been reached, copy (or merge)
        // the next interval from the other window.
        //
        if (AP > ACARD) {
            fstr::assign(&mut USE, b"B");
        } else if (BP > BCARD) {
            fstr::assign(&mut USE, b"A");

        //
        // Otherwise, let's see what we can do with the earlier of
        // the next intervals from A and B.
        //
        } else if (A[AP] < B[BP]) {
            fstr::assign(&mut USE, b"A");
        } else if (B[BP] <= A[AP]) {
            fstr::assign(&mut USE, b"B");
        }

        //
        // If there is still space in the output window, fill it
        // as necessary. Otherwise, stop filling the array, but continue
        // to count the number of elements in excess of the size of the
        // output window.
        //
        // The general idea is this: if the next interval overlaps the
        // latest output interval, merge the two (extending the output
        // interval to the right). Otherwise, insert the next interval
        // intact.
        //
        if fstr::eq(&USE, b"A") {
            if (CP < CSIZE) {
                if ((A[AP] <= END) && (CP > 0)) {
                    C[CP] = intrinsics::DMAX1(&[C[CP], A[(AP + 1)]]);
                } else {
                    CP = (CP + 2);
                    C[(CP - 1)] = A[AP];
                    C[CP] = A[(AP + 1)];
                }

                END = C[CP];
            } else {
                if (A[AP] <= END) {
                    END = intrinsics::DMAX1(&[END, A[(AP + 1)]]);
                } else {
                    OVER = (OVER + 2);
                    END = A[(AP + 1)];
                }
            }

            AP = (AP + 2);

        //
        // This is the same as the last clause, with B replacing A.
        //
        } else if fstr::eq(&USE, b"B") {
            if (CP < CSIZE) {
                if ((B[BP] <= END) && (CP > 0)) {
                    C[CP] = intrinsics::DMAX1(&[C[CP], B[(BP + 1)]]);
                } else {
                    CP = (CP + 2);
                    C[(CP - 1)] = B[BP];
                    C[CP] = B[(BP + 1)];
                }

                END = C[CP];
            } else {
                if (B[BP] <= END) {
                    END = intrinsics::DMAX1(&[END, B[(BP + 1)]]);
                } else {
                    OVER = (OVER + 2);
                    END = B[(BP + 1)];
                }
            }

            BP = (BP + 2);
        }
    }

    //
    // Set the cardinality of the output window.
    //
    SCARDD(CP, C.as_slice_mut(), ctx)?;

    //
    // If there is an excess of elements, signal an error and check out
    // as usual.
    //
    if (OVER > 0) {
        EXCESS(OVER, b"window", ctx)?;
        SIGERR(b"SPICE(WINDOWEXCESS)", ctx)?;
    }

    CHKOUT(b"WNUNID", ctx)?;

    Ok(())
}
