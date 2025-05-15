//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Difference of two character sets
///
/// Take the difference of two character sets to form a third set.
///
/// # Required Reading
///
/// * [SETS](crate::required_reading::sets)
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
///
///  2)  If length of the elements of the output set is less than
///      the length of the elements of the FIRST input set, the
///      error SPICE(ELEMENTSTOOSHORT) is signaled.
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
/// -    Beta Version 2.0.0, 21-DEC-1988 (NJB)
///
///         Error signaled if output set elements are not long enough.
///         Length must be at least max of lengths of input elements.
///         Also, calling protocol for EXCESS has been changed. Call to
///         SETMSG removed.
///
///         Also, in the overflow case, the number of excess elements was
///         computed incorrectly; this has been fixed. The problem was
///         that OVER was incremented in all cases of the overflow IF
///         block, rather than only in the cases where the cardinality of
///         the output cell would have been incremented if there were
///         enough room.
/// ```
pub fn diffc(
    ctx: &mut SpiceContext,
    a: CharArray,
    b: CharArray,
    c: CharArrayMut,
) -> crate::Result<()> {
    DIFFC(a, b, c, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DIFFC ( Difference of two character sets )
pub fn DIFFC(
    A: CharArray,
    B: CharArray,
    C: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let A = DummyCharArray::new(A, None, LBCELL..);
    let B = DummyCharArray::new(B, None, LBCELL..);
    let mut C = DummyCharArrayMut::new(C, None, LBCELL..);
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
    CHKIN(b"DIFFC", ctx)?;

    //
    // Make sure output set elements are long enough.
    //
    if (intrinsics::LEN(&C[LBCELL]) < intrinsics::LEN(&A[LBCELL])) {
        SETMSG(
            b"Length of output cell is #.  Length required to contain result is #.",
            ctx,
        );

        ERRINT(b"#", intrinsics::LEN(&C[LBCELL]), ctx);
        ERRINT(
            b"#",
            intrinsics::MAX0(&[intrinsics::LEN(&A[LBCELL]), intrinsics::LEN(&B[LBCELL])]),
            ctx,
        );

        SIGERR(b"SPICE(ELEMENTSTOOSHORT)", ctx)?;

        CHKOUT(b"DIFFC", ctx)?;
        return Ok(());
    }

    //
    // Find the cardinality of the input sets, and the allowed size
    // of the output set.
    //
    ACARD = CARDC(A.as_arg(), ctx)?;
    BCARD = CARDC(B.as_arg(), ctx)?;
    CSIZE = SIZEC(C.as_arg(), ctx)?;

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
                fstr::assign(C.get_mut(CCARD), A.get(APOINT));
                APOINT = (APOINT + 1);
            } else if fstr::eq(A.get(APOINT), B.get(BPOINT)) {
                APOINT = (APOINT + 1);
                BPOINT = (BPOINT + 1);
            } else if fstr::lt(&A[APOINT], &B[BPOINT]) {
                CCARD = (CCARD + 1);
                fstr::assign(C.get_mut(CCARD), A.get(APOINT));
                APOINT = (APOINT + 1);
            } else if fstr::lt(&B[BPOINT], &A[APOINT]) {
                BPOINT = (BPOINT + 1);
            }

        //
        // Otherwise, stop filling the array, but continue to count the
        // number of elements in excess of the size of the output set.
        //
        } else {
            if (BPOINT > BCARD) {
                OVER = (OVER + 1);
                APOINT = (APOINT + 1);
            } else if fstr::eq(A.get(APOINT), B.get(BPOINT)) {
                APOINT = (APOINT + 1);
                BPOINT = (BPOINT + 1);
            } else if fstr::lt(&A[APOINT], &B[BPOINT]) {
                OVER = (OVER + 1);
                APOINT = (APOINT + 1);
            } else if fstr::lt(&B[BPOINT], &A[APOINT]) {
                BPOINT = (BPOINT + 1);
            }
        }
    }

    //
    // Set the cardinality of the output set.
    //
    SCARDC(CCARD, C.as_arg_mut(), ctx)?;

    //
    // Report any excess.
    //
    if (OVER > 0) {
        EXCESS(OVER, b"set", ctx)?;
        SIGERR(b"SPICE(SETEXCESS)", ctx)?;
    }

    CHKOUT(b"DIFFC", ctx)?;

    Ok(())
}
