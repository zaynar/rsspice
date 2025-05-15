//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Difference two DP windows
///
/// Place the difference of two double precision windows into
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
///  C          O   Difference of A and B.
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
///  C        is the output SPICE window, containing the difference
///           of A and B --- every point contained in A, but not
///           contained in B.
///
///           C must be distinct from both A and B.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the difference of the two windows results in an excess of
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
///  Mathematically, the difference of two windows contains every
///  point contained in the first window but not contained in the
///  second window.
///
///  Fortran offers no satisfactory floating point representation
///  of open intervals. Thus, for floating point windows we must
///  return the closure of the set theoretical difference: that is,
///  the difference plus the endpoints of the first window that are
///  contained in the second window.
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
///  Then the difference of A and B contains the intervals
///
///        [ 1, 2 ]  [ 7, 8 ]  [ 10, 11 ]  [ 23, 27 ]
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
/// -    SPICELIB Version 2.1.0, 24-AUG-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Fixed I/O type
///         of argument C in $Brief_I/O table.
///
///         Added entry #2 in $Exceptions section.
///
/// -    SPICELIB Version 2.0.0, 16-SEP-1998 (WLT)
///
///         The previous version did not work when removing
///         singletons. This has been corrected.
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
pub fn wndifd(ctx: &mut SpiceContext, a: &[f64], b: &[f64], c: &mut [f64]) -> crate::Result<()> {
    WNDIFD(a, b, c, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WNDIFD ( Difference two DP windows )
pub fn WNDIFD(A: &[f64], B: &[f64], C: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let A = DummyArray::new(A, LBCELL..);
    let B = DummyArray::new(B, LBCELL..);
    let mut C = DummyArrayMut::new(C, LBCELL..);
    let mut ACARD: i32 = 0;
    let mut BCARD: i32 = 0;
    let mut CSIZE: i32 = 0;
    let mut APB: i32 = 0;
    let mut APE: i32 = 0;
    let mut BPB: i32 = 0;
    let mut BPE: i32 = 0;
    let mut PUT: i32 = 0;
    let mut F: f64 = 0.0;
    let mut L: f64 = 0.0;
    let mut NEEDED: i32 = 0;
    let mut OVER: i32 = 0;
    let mut KEEP: bool = false;
    let mut UNRSLV: bool = false;

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

    CHKIN(b"WNDIFD", ctx)?;

    //
    // Find the cardinality of the input windows, and the allowed size
    // of the output window. Also, save the size of the second window.
    //
    ACARD = CARDD(A.as_slice(), ctx)?;
    BCARD = CARDD(B.as_slice(), ctx)?;
    CSIZE = SIZED(C.as_slice(), ctx)?;
    OVER = 0;

    //
    // Empty out the output window.
    //
    SSIZED(CSIZE, C.as_slice_mut(), ctx)?;

    //
    // Let's handle the pathological cases first.
    //
    if (BCARD == 0) {
        COPYD(A.as_slice(), C.as_slice_mut(), ctx)?;
        CHKOUT(b"WNDIFD", ctx)?;
        return Ok(());
    } else if (ACARD == 0) {
        CHKOUT(b"WNDIFD", ctx)?;
        return Ok(());
    }

    //
    // Now get pointers to the first intervals of A and B.
    //
    APB = 1;
    APE = 2;
    BPB = 1;
    BPE = 2;
    PUT = 1;
    //
    // As long as the end pointer for A is less than the cardinality
    // of A we need to examine intervals and decide how much of
    // them to keep in C.
    //
    while (APE <= ACARD) {
        //
        // We will work with the interval [F,L] which starts out
        // as the next interval of A.  We modify it below as required
        // when subtracting out intervals of B.
        //
        F = A[APB];
        L = A[APE];
        //
        // Right now we have not resolved whether to keep the interval
        // [F,L], but until we know better we assume it is a keeper.
        //
        UNRSLV = (BPE <= BCARD);
        KEEP = true;

        while UNRSLV {
            if (L < B[BPB]) {
                //
                // The interval [F,L] is before the next interval of B, we
                // have resolved what to do with this one.   It is a
                // keeper.
                //
                UNRSLV = false;
            } else if (F > B[BPE]) {
                //
                // [F,L] is after the end of the current interval in B,
                // we need to look at the next interval of B
                //
                BPB = (BPB + 2);
                BPE = (BPE + 2);
                UNRSLV = (BPE <= BCARD);
            } else {
                //
                // There is some overlap between the current interval
                // of B and the current interval of A. There are
                // several possibilities
                //
                // 1) The current interval of A is contained in the
                //    current interval of B (This includes singleton
                //    intervals in A). We just mark [F,L] so that it
                //    won't be kept.  We have fully resolved what to
                //    do with [F,L].
                //
                // 2) The interval from B overlaps at the beginning
                //    of the interval of A
                //
                //    B interval [......]
                //    A interval     [............]
                //    result of A-B     [.........]
                //
                //    In this case we need to shrink the interval [F,L]
                //    but we have not resolved how much of the result
                //    to keep.
                //
                // 3) The interval from B falls inside the current
                //    interval [F,L]
                //
                //    B interval        [......]
                //    A interval     [............]
                //    result of A-B  [..]      [..]
                //
                //    If the interval from B is not a singleton, we store
                //    the first part of [F,L] in C and then set [F,L] to
                //    be the right interval which is still not resolved.
                //
                //    If the B interval is a singleton we can ignore ignore
                //    it.  But we have not resolved what to do about
                //    [F,L], we need to look at the next interval of B.
                //

                //
                // 4) The interval from B overlaps at the ending
                //    of the interval of A
                //
                //    B interval          [......]
                //    A interval     [......]
                //    result of A-B  [....]
                //
                //    We need to shrink [F,L]. In this case we know we can
                //    keep all of what's left because all other intervals
                //    of B are to the right of [F,L]
                //
                if ((B[BPB] <= F) && (L <= B[BPE])) {
                    //
                    // Case 1 above
                    //
                    KEEP = false;
                    UNRSLV = false;
                } else if (B[BPB] <= F) {
                    //
                    // Case 2 above
                    //
                    F = B[BPE];
                    BPB = (BPB + 2);
                    BPE = (BPE + 2);
                    UNRSLV = (BPE <= BCARD);
                } else if (((F <= B[BPB]) && (L >= B[BPE])) && (B[BPB] < B[BPE])) {
                    //
                    // Case 3 above (non-singleton interval of B).
                    //

                    if (PUT < CSIZE) {
                        C[PUT] = F;
                        C[(PUT + 1)] = B[BPB];

                        SCARDD((PUT + 1), C.as_slice_mut(), ctx)?;

                        PUT = (PUT + 2);
                    } else {
                        OVER = (OVER + 2);
                    }

                    F = B[BPE];
                    //
                    // If the interval from B contained L, we will not
                    // want to be keeping the singleton [F,L].
                    //
                    if (F == L) {
                        KEEP = false;
                        UNRSLV = false;
                    }

                    BPB = (BPB + 2);
                    BPE = (BPE + 2);
                    UNRSLV = (UNRSLV && (BPE <= BCARD));
                } else if (((F <= B[BPB]) && (L >= B[BPE])) && (B[BPB] == B[BPE])) {
                    //
                    // Case 3 above (singleton interval of B).
                    //
                    BPB = (BPB + 2);
                    BPE = (BPE + 2);
                    UNRSLV = (BPE <= BCARD);
                } else {
                    //
                    // Case 4 above
                    //
                    L = B[BPB];
                    UNRSLV = false;
                }
            }
        }

        //
        // If there is anything to keep in C, put it there.
        //
        if KEEP {
            //
            // Make sure there is sufficient room to do the putting.
            //
            if (PUT < CSIZE) {
                C[PUT] = F;
                C[(PUT + 1)] = L;
                SCARDD((PUT + 1), C.as_slice_mut(), ctx)?;
                PUT = (PUT + 2);
            } else {
                OVER = (OVER + 2);
            }
        }
        //
        // Move the pointers in A to the next interval.
        //
        APB = (APB + 2);
        APE = (APE + 2);
    }

    //
    // We've examined all of the intervals of A and B, but if we
    // didn't actually store all of the difference, signal an error.
    //
    if (OVER > 0) {
        NEEDED = (OVER + CSIZE);

        SETMSG(b"The output window did not have sufficient room to contain the result of the window difference.  It has room for # endpoints, but # were needed to describe the difference. ", ctx);

        ERRINT(b"#", CSIZE, ctx);
        ERRINT(b"#", NEEDED, ctx);
        SIGERR(b"SPICE(WINDOWEXCESS)", ctx)?;
    }

    CHKOUT(b"WNDIFD", ctx)?;

    Ok(())
}
