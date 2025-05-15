//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Difference of two integer sets
///
/// Take the difference of two integer sets to form a third set.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A          I   First input set.
///  B          I   Second input set.
///  C          O   Difference of A and B.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A        is a set.
///
///
///  B        is a set, distinct from A.
/// ```
///
/// # Detailed Output
///
/// ```text
///  C        is a set, distinct from sets A and B, which
///           contains the difference of A and B (that is,
///           all of the elements which are in A, but NOT
///           in B).
///
///           If the size (maximum cardinality) of C is smaller
///           than the cardinality of the difference of A and B,
///           then only as many items as will fit in C are
///           included, and an error is returned.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the difference of the two sets causes an excess of
///      elements, the error SPICE(SETEXCESS) is signaled.
/// ```
///
/// # Examples
///
/// ```text
///  The DIFFERENCE of two sets contains every element which is
///  in the first set, but NOT in the second.
///
///        {a,b}      difference  {c,d}     =  {a,b}
///        {a,b,c}                {b,c,d}      {a}
///        {a,b,c,d}              {}           {a,b,c,d}
///        {}                     {a,b,c,d}    {}
///        {}                     {}           {}
///
///  The following call
///
///        CALL DIFFC  ( PLANETS, ASTEROIDS, RESULT )
///
///  places the difference of the character sets PLANETS and
///  ASTEROIDS into the character set RESULT.
///
///  The output set must be distinct from both of the input sets.
///  For example, the following calls are invalid.
///
///        CALL DIFFI  ( CURRENT,     NEW, CURRENT )
///        CALL DIFFI  (     NEW, CURRENT, CURRENT )
///
///  In each of the examples above, whether or not the subroutine
///  signals an error, the results will almost certainly be wrong.
///  Nearly the same effect can be achieved, however, by placing the
///  result into a temporary set, which is immediately copied back
///  into one of the input sets, as shown below.
///
///        CALL DIFFI  ( CURRENT, NEW,  TEMP )
///        CALL COPYI  ( TEMP,    NEW        )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  C.A. Curzon        (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (WLT) (IMU) (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 06-JAN-1989 (NJB)
///
///         Calling protocol of EXCESS changed. Call to SETMSG removed.
///
///         Also, in the overflow case, the number of excess elements was
///         computed incorrectly; this has been fixed. The problem was
///         that OVER was incremented in all cases of the overflow IF
///         block, rather than only in the cases where the cardinality of
///         the output cell would have been incremented if there were
///         enough room.
/// ```
pub fn diffi(ctx: &mut SpiceContext, a: &[i32], b: &[i32], c: &mut [i32]) -> crate::Result<()> {
    DIFFI(a, b, c, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DIFFI ( Difference of two integer sets )
pub fn DIFFI(A: &[i32], B: &[i32], C: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let A = DummyArray::new(A, LBCELL..);
    let B = DummyArray::new(B, LBCELL..);
    let mut C = DummyArrayMut::new(C, LBCELL..);
    let mut APOINT: i32 = 0;
    let mut BPOINT: i32 = 0;
    let mut CSIZE: i32 = 0;
    let mut ACARD: i32 = 0;
    let mut BCARD: i32 = 0;
    let mut CCARD: i32 = 0;
    let mut OVER: i32 = 0;

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
    CHKIN(b"DIFFI", ctx)?;

    //
    // Find the cardinality of the input sets, and the allowed size
    // of the output set.
    //
    ACARD = CARDI(A.as_slice(), ctx)?;
    BCARD = CARDI(B.as_slice(), ctx)?;
    CSIZE = SIZEI(C.as_slice(), ctx)?;

    //
    // Begin with the input pointers at the first elements of the
    // input sets. The cardinality of the output set is zero.
    // And there is no overflow so far.
    //
    APOINT = 1;
    BPOINT = 1;

    CCARD = 0;
    OVER = 0;

    //
    // When the end of the first input set is reached, we're done.
    //
    while (APOINT <= ACARD) {
        //
        // If there is still space in the output set, fill it
        // as necessary.
        //
        if (CCARD < CSIZE) {
            if (BPOINT > BCARD) {
                CCARD = (CCARD + 1);
                C[CCARD] = A[APOINT];
                APOINT = (APOINT + 1);
            } else if (A[APOINT] == B[BPOINT]) {
                APOINT = (APOINT + 1);
                BPOINT = (BPOINT + 1);
            } else if (A[APOINT] < B[BPOINT]) {
                CCARD = (CCARD + 1);
                C[CCARD] = A[APOINT];
                APOINT = (APOINT + 1);
            } else if (B[BPOINT] < A[APOINT]) {
                BPOINT = (BPOINT + 1);
            }

        //
        // Otherwise, stop following the array, but continue to count the
        // number of elements in excess of the size of the output set.
        //
        } else {
            if (BPOINT > BCARD) {
                OVER = (OVER + 1);
                APOINT = (APOINT + 1);
            } else if (A[APOINT] == B[BPOINT]) {
                APOINT = (APOINT + 1);
                BPOINT = (BPOINT + 1);
            } else if (A[APOINT] < B[BPOINT]) {
                OVER = (OVER + 1);
                APOINT = (APOINT + 1);
            } else if (B[BPOINT] < A[APOINT]) {
                BPOINT = (BPOINT + 1);
            }
        }
    }

    //
    // Set the cardinality of the output set.
    //
    SCARDI(CCARD, C.as_slice_mut(), ctx)?;

    //
    // Report any excess.
    //

    if (OVER > 0) {
        EXCESS(OVER, b"set", ctx)?;
        SIGERR(b"SPICE(SETEXCESS)", ctx)?;
    }

    CHKOUT(b"DIFFI", ctx)?;

    Ok(())
}
