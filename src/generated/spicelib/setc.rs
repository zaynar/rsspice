//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Compare character sets
///
/// Compare two character sets, as indicated by a relational operator.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
/// * [SETS](crate::required_reading::sets)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A          I   First set.
///  OP         I   Comparison operator.
///  B          I   Second set.
///
///  The function returns the result of the comparison: A (OP) B.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A        is a set.
///
///
///  OP       is a comparison operator, indicating the way in
///           which the input sets are to be compared. OP may
///           be any of the following:
///
///               Operator             Meaning
///               --------  -------------------------------------
///                 '='     A = B is .TRUE. if A and B are equal
///                         (contain the same elements).
///
///                 '<>'    A <> B is .TRUE. if A and B are not
///                         equal.
///
///                 '<='    A <= B is .TRUE. if A is a subset of B.
///
///                 '<'     A < B is .TRUE. if A is a proper subset
///                         of B.
///
///                 '>='    A >= B is .TRUE. if B is a subset of A.
///
///                 '>'     A > B is .TRUE. if B is a proper subset
///                         of A.
///
///                 '&'     A & B is .TRUE. if A and B have one or
///                         more elements in common. (The
///                         intersection of the two sets in
///                         non-empty.)
///
///                 '~'     A ~ B is .TRUE. if A and B are disjoint
///                         sets.
///
///  B        is a set.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the result of the comparison: A (OP) B.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the set relational operator is not recognized, the error
///      SPICE(INVALIDOPERATION) is signaled.
/// ```
///
/// # Examples
///
/// ```text
///  1) In the following example, SETx is used to repeat an operation
///     for as long as the integer set FINISHED remains a proper
///     subset of the integer set PLANNED.
///
///        DO WHILE ( SETx ( FINISHED, '<', PLANNED ) )
///           .
///           .
///        END DO
///
///
///  2) In the following example, let the integer sets A, B, and C
///     contain the elements listed below. Let E be an empty integer
///     set.
///
///          A        B        C
///        ---      ---      ---
///          1        1        1
///          2        3        3
///          3
///          4
///
///  Then all of the following expressions are true.
///
///        SETI ( B, '=',  C )      "B is equal to C"
///        SETI ( A, '<>', C )      "A is not equal to C"
///        SETI ( A, '>',  B )      "A is a proper superset of B"
///        SETI ( B, '<=', C )      "B is a subset of C"
///        SETI ( C, '<=', B )      "C is a subset of B"
///        SETI ( A, '<=', A )      "A is a subset of A"
///        SETI ( E, '<=', B )      "E is a subset of B"
///        SETI ( E, '<',  B )      "E is a proper subset of B"
///        SETI ( E, '<=', E )      "E is a subset of E"
///        SETI ( A, '&',  B )      "A has elements in common with B."
///        SETI ( B, '&',  C )      "B has elements in common with C."
///
///  And all of the following are false.
///
///        SETI ( B, '<>',  C )      "B is not equal to C"
///        SETI ( A, '=',   C )      "A is equal to C"
///        SETI ( A, '<',   B )      "A is a proper subset of B"
///        SETI ( B, '<',   C )      "B is a proper subset of C"
///        SETI ( B, '>=',  A )      "B is a superset of A"
///        SETI ( A, '>',   A )      "A is a proper superset of A"
///        SETI ( E, '>=',  A )      "E is a superset of A"
///        SETI ( E, '<',   E )      "E is a proper subset of E"
///        SETI ( A, '~',   B )      "A and B are disjoint sets."
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         Set the default function value to either 0, 0.0D0, .FALSE.,
///         or blank depending on the type of the function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 2.0.0, 11-JAN-1989 (WLT) (HAN)
///
///        The old version was not compatible with the error handling
///        mechanism. Taking the difference of sets A and B caused an
///        overflow of the set DIFF, whose dimension was one. The method of
///        determining the function value has been redesigned, and the
///        difference of the sets is no longer computed.
///
///        The new routine recognizes two new operators, '~' and '&'.
///        If the operator is not recognized, an error is now signaled.
/// ```
pub fn setc(ctx: &mut SpiceContext, a: CharArray, op: &str, b: CharArray) -> crate::Result<bool> {
    let ret = SETC(a, op.as_bytes(), b, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure SETC ( Compare character sets )
pub fn SETC(A: CharArray, OP: &[u8], B: CharArray, ctx: &mut Context) -> f2rust_std::Result<bool> {
    let A = DummyCharArray::new(A, None, LBCELL..);
    let B = DummyCharArray::new(B, None, LBCELL..);
    let mut SETC: bool = false;
    let mut CARDA: i32 = 0;
    let mut CARDB: i32 = 0;
    let mut CONDLT: i32 = 0;
    let mut CONDEQ: i32 = 0;
    let mut CONDGT: i32 = 0;
    let mut CONDOA: i32 = 0;
    let mut CONDOB: i32 = 0;
    let mut CONDAB: i32 = 0;
    let mut INDEXA: i32 = 0;
    let mut INDEXB: i32 = 0;
    let mut COND: i32 = 0;

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
        SETC = false;
        return Ok(SETC);
    } else {
        CHKIN(b"SETC", ctx)?;
        SETC = false;
    }

    //
    // Obtain the cardinality of the sets.
    //
    CARDA = CARDC(A.as_arg(), ctx)?;
    CARDB = CARDC(B.as_arg(), ctx)?;

    //
    // The easiest way to compare two sets is to list them side by side
    // as shown below:
    //
    // Set A  Set B
    // -----  -----
    //   1      1
    //          2
    //   3      3
    //   4      4
    //   5
    //          6
    //   7      7
    //
    // When listed this way, one can easily determine intersections,
    // differences, and unions.  Moreover, to determine if one set
    // is a subset of another, if they are equal, etc, one can just
    // inspect the two lists.
    //
    // We can mimic this in an algorithm.  The main trick is to figure
    // out how to list the sets in this way.  Once we know how to
    // list them, we can simply adapt the listing algorithm to get
    // a comparison algorithm.
    //
    // By the time we get this far, we know that our sets have distinct
    // elements and they are ordered. To write out the list above,
    // we start at the beginning of both sets (they're ordered,
    // remember?).  Look at the next element of A and the next element
    // of B ( to start out ``next'' means ``first'' ).  If the item
    // from A is smaller it should be written and space should be left
    // in the B column. If they are the same write them both.  Otherwise,
    // the item from B is smaller, so  leave space in the A column and
    // write the item from B.  Continue until you run out of items in
    // one of the sets.  Then just write down all those remaining in the
    // other set in the appropriate column.  This is what the loop
    // below does.
    //
    //
    //       NEXTA = 1
    //       NEXTB = 1
    //
    //       DO WHILE (       ( NEXTA .LT. CARD(A) )
    //      .           .AND. ( NEXTB .LT. CARD(B) ) )
    //
    //          IF ( A(NEXTA) .LT. B(NEXTB) ) THEN
    //
    //             WRITE (UNIT,*) A(NEXTA),   SPACES
    //             NEXTA = NEXTA + 1
    //
    //          ELSE IF ( A(NEXTA) .EQ. B(NEXTB) ) THEN
    //
    //             WRITE (UNIT,*) A(NEXTA),   B(NEXTB)
    //             NEXTA = NEXTA + 1
    //             NEXTB = NEXTB + 1
    //
    //          ELSE
    //
    //             WRITE (UNIT,*) SPACES, B(NEXTB)
    //             NEXTB = NEXTB + 1
    //
    //          END IF
    //       END DO
    //
    //       DO NEXTA = 1, CARD(A)
    //          WRITE (UNIT,*) A(NEXTA),SPACES
    //       END DO
    //
    //       DO NEXTB = 1, CARD(B)
    //          WRITE (UNIT,*) B(NEXTB),SPACES
    //       END DO
    //
    //
    // This also gives us a way to compare the elements of the two
    // sets one item at a time.  Instead of writing the items, we
    // can make a decision as to whether or not the sets have the
    // relationship we are interested in.
    //
    // At the beginning of the loop we assume that the two sets are
    // related in the way we want.  Once the comparison has been made
    // we can decide if they are still related in that way.  If not,
    // we can RETURN .FALSE.  Using psuedo-code the loop is modified
    // as shown below.
    //
    //       NEXTA = 1
    //       NEXTB = 1
    //
    //       DO WHILE (       ( NEXTA .LT. CARD(A) )
    //      .           .AND. ( NEXTB .LT. CARD(B) ) )
    //
    //          IF ( A(NEXTA) .LT. B(NEXTB) ) THEN
    //
    //             RELATED = RELATIONSHIP_OF_INTEREST(A<B)
    //             NEXTA   = NEXTA + 1
    //
    //          ELSE IF ( A(NEXTA) .EQ. B(NEXTB) ) THEN
    //
    //             RELATED = RELATIONSHIP_OF_INTEREST(A=B)
    //             NEXTA   = NEXTA + 1
    //             NEXTB   = NEXTB + 1
    //
    //          ELSE
    //
    //             RELATED = RELATIONSHIP_OF_INTEREST(A>B)
    //             NEXTB   = NEXTB + 1
    //
    //          END IF
    //
    //          IF ( SURE_NOW(RELATED) ) THEN
    //             RETURN with the correct value.
    //          ELSE
    //             Keep going.
    //          END IF
    //
    //       END DO
    //

    //
    // Using the cardinality of the two sets, some function
    // values can be determined right away. If the cardinality
    // is not enough, we need to set up some conditions for the
    // loop which compares the individual elements of the sets.
    //

    //
    // A cannot be a proper subset of B if the cardinality of A is
    // greater than or equal to the cardinality of B.
    //
    if fstr::eq(OP, b"<") {
        if (CARDA >= CARDB) {
            SETC = false;
            CHKOUT(b"SETC", ctx)?;
            return Ok(SETC);
        } else {
            CONDLT = 0;
            CONDEQ = 1;
            CONDGT = 1;
            CONDOA = 0;
            CONDOB = 1;
            CONDAB = 1;
        }

    //
    // A cannot be a subset of B if A contains more elements than B.
    //
    } else if fstr::eq(OP, b"<=") {
        if (CARDA > CARDB) {
            SETC = false;
            CHKOUT(b"SETC", ctx)?;
            return Ok(SETC);
        } else {
            CONDLT = 0;
            CONDEQ = 1;
            CONDGT = 1;
            CONDOA = 0;
            CONDOB = 1;
            CONDAB = 1;
        }

    //
    // If the cardinality of the two sets is not equal, there's no way
    // that the two sets could be equal.
    //
    } else if fstr::eq(OP, b"=") {
        if (CARDA != CARDB) {
            SETC = false;
            CHKOUT(b"SETC", ctx)?;
            return Ok(SETC);
        } else {
            CONDLT = 0;
            CONDEQ = 1;
            CONDGT = 0;
            CONDOA = 0;
            CONDOB = 0;
            CONDAB = 1;
        }

    //
    // If the cardinality of the two sets is not equal, the sets
    // are not equal.
    //
    } else if fstr::eq(OP, b"<>") {
        if (CARDA != CARDB) {
            SETC = true;
            CHKOUT(b"SETC", ctx)?;
            return Ok(SETC);
        } else {
            CONDLT = 2;
            CONDEQ = 1;
            CONDGT = 2;
            CONDOA = 0;
            CONDOB = 0;
            CONDAB = 0;
        }

    //
    // B cannot be a proper subset of A if the cardinality of A is less
    // than or equal to the cardinality of B.
    //
    } else if fstr::eq(OP, b">") {
        if (CARDA <= CARDB) {
            SETC = false;
            CHKOUT(b"SETC", ctx)?;
            return Ok(SETC);
        } else {
            CONDLT = 1;
            CONDEQ = 1;
            CONDGT = 0;
            CONDOA = 1;
            CONDOB = 0;
            CONDAB = 1;
        }
    //
    // B cannot be a subset of A if B contains more elements than A.
    //
    } else if fstr::eq(OP, b">=") {
        if (CARDA < CARDB) {
            SETC = false;
            CHKOUT(b"SETC", ctx)?;
            return Ok(SETC);
        } else {
            CONDLT = 1;
            CONDEQ = 1;
            CONDGT = 0;
            CONDOA = 1;
            CONDOB = 0;
            CONDAB = 1;
        }
    //
    // If the cardinality of one of the sets is zero, they can't
    // possibly have any elements in common.
    //
    } else if fstr::eq(OP, b"&") {
        if ((CARDA == 0) || (CARDB == 0)) {
            SETC = false;
            CHKOUT(b"SETC", ctx)?;
            return Ok(SETC);
        } else {
            CONDLT = 1;
            CONDEQ = 2;
            CONDGT = 1;
            CONDOA = 0;
            CONDOB = 0;
        }

    //
    // If either A or B is the null set, the two sets are disjoint.
    //
    } else if fstr::eq(OP, b"~") {
        if ((CARDA == 0) || (CARDB == 0)) {
            SETC = true;
            CHKOUT(b"SETC", ctx)?;
            return Ok(SETC);
        } else {
            CONDLT = 1;
            CONDEQ = 0;
            CONDGT = 1;
            CONDOA = 1;
            CONDOB = 1;
        }

    //
    // If the relational operator is not recognized, signal an
    // error.
    //
    } else {
        SETMSG(b"Relational operator, *, is not recognized.", ctx);
        ERRCH(b"*", OP, ctx);
        SIGERR(b"SPICE(INVALIDOPERATION)", ctx)?;
        CHKOUT(b"SETC", ctx)?;
        return Ok(SETC);
    }

    //
    // Initialize counters used for checking the elements of the sets.
    //
    INDEXA = 1;
    INDEXB = 1;
    COND = 0;

    //
    // If we've come this far we need to check the elements of the
    // sets to determine the function value.
    //
    while ((INDEXA <= CARDA) && (INDEXB <= CARDB)) {
        if fstr::lt(A.get(INDEXA), B.get(INDEXB)) {
            COND = CONDLT;
            INDEXA = (INDEXA + 1);
        } else if fstr::eq(A.get(INDEXA), B.get(INDEXB)) {
            COND = CONDEQ;
            INDEXA = (INDEXA + 1);
            INDEXB = (INDEXB + 1);
        } else {
            COND = CONDGT;
            INDEXB = (INDEXB + 1);
        }

        //
        // At this point, there are several cases which allow us to
        // determine the function value without continuing to compare
        // the elements of the sets:
        //
        // 1. If the operator is '~' and a common element was found,
        //    the sets are not disjoint ( COND = 0 ).
        //
        // 2. If the operator is '&' and a common element was found,
        //    the sets have at least one common element ( COND = 2 ).
        //
        // 3. If the sets are being compared for containment, and the
        //    first element of the "contained" set is less than the first
        //    element of the "containing" set, the "contained" set
        //    cannot be a subset of the "containing" set ( COND = 0 ).
        //
        // 4. If the operator is '=' and the elements being compared are
        //    not equal, the sets are not equal ( COND = 0 ).
        //
        // 5. If the operator is '<>' and the elements being compared are
        //    not equal, the sets are not equal ( COND = 2 ).
        //
        //
        if (COND == 0) {
            SETC = false;
            CHKOUT(b"SETC", ctx)?;
            return Ok(SETC);
        } else if (COND == 2) {
            SETC = true;
            CHKOUT(b"SETC", ctx)?;
            return Ok(SETC);
        }
    }

    //
    // We've exited the loop, so now we need to make a decision based on
    // what's left over.
    //

    //
    // We've gone through all of set B and there are elements left in
    // A.
    //
    if (INDEXA <= CARDA) {
        COND = CONDOA;
    //
    // We've gone through all of set A and there are elements left in
    // B.
    //
    } else if (INDEXB <= CARDB) {
        COND = CONDOB;
    //
    // We've gone through both the sets.
    //
    } else {
        COND = CONDAB;
    }

    //
    // Determine the value of SETC from the results.
    //
    SETC = (COND == 1);

    CHKOUT(b"SETC", ctx)?;
    Ok(SETC)
}
