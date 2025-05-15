//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Intersect two DP windows
///
/// Place the intersection of two double precision windows into
/// a third window.
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
///  C          O   Intersection of A and B.
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
///  C        is the output SPICE window, containing the
///           intersection of A and B --- every point contained in both
///           A and B.
///
///           C must be distinct from both A and B.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the intersection of the two windows results in an excess of
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
///  The intersection of two windows contains every point contained
///  both in the first window and in the second window.
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
///        [ 2, 4 ]  [ 8, 10 ]  [ 16, 18 ]
///
///  Then the intersection of A and B contains the intervals
///
///        [ 2, 3 ]  [ 8, 10 ]
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
/// -    SPICELIB Version 1.1.0, 24-AUG-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Fixed I/O type
///         of argument C in $Brief_I/O table.
///
///         Added entry #2 in $Exceptions section.
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
pub fn wnintd(ctx: &mut SpiceContext, a: &[f64], b: &[f64], c: &mut [f64]) -> crate::Result<()> {
    WNINTD(a, b, c, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNINTD ( Intersect two DP windows )
pub fn WNINTD(A: &[f64], B: &[f64], C: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let A = DummyArray::new(A, LBCELL..);
    let B = DummyArray::new(B, LBCELL..);
    let mut C = DummyArrayMut::new(C, LBCELL..);
    let mut ACARD: i32 = 0;
    let mut BCARD: i32 = 0;
    let mut CSIZE: i32 = 0;
    let mut AP: i32 = 0;
    let mut BP: i32 = 0;
    let mut CP: i32 = 0;
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
        CHKIN(b"WNINTD", ctx)?;
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
    OVER = 0;

    //
    // When the end of either input window is reached, we're done.
    //
    while ((AP < ACARD) && (BP < BCARD)) {
        //
        // Let's see what we can do with the earlier of the next
        // intervals from A and B.
        //
        if (A[(AP + 1)] < B[(BP + 1)]) {
            fstr::assign(&mut USE, b"A");
        } else if (B[(BP + 1)] <= A[(AP + 1)]) {
            fstr::assign(&mut USE, b"B");
        }

        //
        // If there is still space in the output window, fill it
        // as necessary. Otherwise, stop filling the array, but continue
        // to count the number of elements in excess of the size of the
        // output window.
        //
        // The general idea is this: if the next interval of A overlaps
        // the next interval of B, save the area of overlap. (Similarly
        // for B.)
        //
        if fstr::eq(&USE, b"A") {
            if (A[(AP + 1)] >= B[BP]) {
                if (CP < CSIZE) {
                    CP = (CP + 2);
                    C[(CP - 1)] = intrinsics::DMAX1(&[B[BP], A[AP]]);
                    C[CP] = A[(AP + 1)];
                } else {
                    OVER = (OVER + 2);
                }
            }

            AP = (AP + 2);

        //
        // This is the same as the last clause, with B replacing A.
        //
        } else if fstr::eq(&USE, b"B") {
            if (B[(BP + 1)] >= A[AP]) {
                if (CP < CSIZE) {
                    CP = (CP + 2);
                    C[(CP - 1)] = intrinsics::DMAX1(&[A[AP], B[BP]]);
                    C[CP] = B[(BP + 1)];
                } else {
                    OVER = (OVER + 2);
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
    // If there are any excess elements, signal an error and check out
    // as usual.
    //
    if (OVER > 0) {
        EXCESS(OVER, b"window", ctx)?;
        SIGERR(b"SPICE(WINDOWEXCESS)", ctx)?;
    }

    CHKOUT(b"WNINTD", ctx)?;

    Ok(())
}
