//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Compare two DP windows
///
/// Compare two double precision windows.
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
///  A          I   First window.
///  OP         I   Comparison operator.
///  B          I   Second window.
///
///  The function returns the result of comparison: A (OP) B.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A,
///  B        are SPICE windows, each of which contains zero or more
///           intervals.
///
///  OP       is a comparison operator, indicating the way in which the
///           input sets are to be compared. OP may be any of the
///           following:
///
///              Operator             Meaning
///              --------  -------------------------------------------
///                '='     A = B is .TRUE. if A and B are equal
///                        (contain the same intervals).
///
///                '<>'    A <> B is .TRUE. if A and B are not equal.
///
///                '<='    A <= B is .TRUE. if A is a subset of B.
///
///                '<'     A < B is .TRUE. if A is a proper subset
///                        of B.
///
///                '>='    A >= B is .TRUE. if B is a subset of A.
///
///                '>'     A > B is .TRUE. if B is a proper subset
///                        of A.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the result of the comparison.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the relational operator is not recognized, the error
///      SPICE(INVALIDOPERATION) is signaled.
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
///  This function returns .TRUE. whenever the specified relationship
///  between the input windows, A and B, is satisfied. For example,
///  the expression
///
///     WNRELD ( NEEDED, '<=', AVAIL )
///
///  is .TRUE. whenever the window NEEDED is a subset of the window
///  AVAIL. One window is a subset of another window if each of
///  the intervals in the first window is included in one of the
///  intervals in the second window. In addition, the first window
///  is a proper subset of the second if the second window contains
///  at least one point not contained in the first window. (Thus,
///  '<' implies '<=', and '>' implies '>='.)
///
///  The following pairs of expressions are equivalent.
///
///     WNRELD ( A, '>', B )
///     WNRELD ( B, '<', A )
///
///     WNRELD ( A, '>=', B )
///     WNRELD ( B, '<=', A )
/// ```
///
/// # Examples
///
/// ```text
///  Let A contain the intervals
///
///        [ 1, 3 ]  [ 7, 11 ]  [ 23, 27 ]
///
///  Let B and C contain the intervals
///
///        [ 1, 2 ]  [ 9, 9 ]  [ 24, 27 ]
///
///  Let D contain the intervals
///
///        [ 5, 10 ]  [ 15, 25 ]
///
///  Finally, let E and F be empty windows (containing no intervals).
///
///  Because B and C contain the same intervals,
///
///        WNRELD ( B, '=',  C )
///        WNRELD ( B, '<=', C )
///        WNRELD ( B, '>=', C )
///
///  are all true, while
///
///        WNRELD ( B, '<>', C )
///
///  is .FALSE. Because neither B nor C contains any points not also
///  contained by the other, neither is a proper subset of the other.
///  Thus,
///
///        WNRELD ( B, '<', C )
///        WNRELD ( B, '>', C )
///
///  are both false.
///
///  Every point contained in B and C is also contained in A. Thus,
///
///        WNRELD ( B, '<=', A )
///        WNRELD ( A, '>=', C )
///
///  are both true. In addition, A contains points not contained in
///  B and C. (That is, the differences A-B and A-C are not empty.)
///  Thus, B and C are proper subsets of A as well, and
///
///        WNRELD ( B, '<', A )
///        WNRELD ( A, '>', B )
///
///  are both true.
///
///  Although A and D have points in common, neither contains the
///  other. Thus
///
///        WNRELD ( A, '=',  D )
///        WNRELD ( A, '<=', D )
///        WNRELD ( A, '>=', D )
///
///  are all false.
///
///  In addition, any window is equal to itself, a subset of itself,
///  and a superset of itself. Thus,
///
///        WNRELD ( A, '=',  A )
///        WNRELD ( A, '<=', A )
///        WNRELD ( A, '>=', A )
///
///  are always true. However, no window is a proper subset or a
///  proper superset of itself. Thus,
///
///        WNRELD ( A, '<', A )
///        WNRELD ( A, '>', A )
///
///  are always false.
///
///  Finally, an empty window is a a proper subset of any window
///  except another empty window. Thus,
///
///        WNRELD ( E, '<', A )
///
///  is .TRUE., but
///
///        WNRELD ( E, '<', F )
///
///  is .FALSE.
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
///         Edited the header to comply with NAIF standard. Added entry #2
///         in $Exceptions section.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         Set the default function value to either 0, 0.0D0, .FALSE.,
///         or blank depending on the type of the function.
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
/// -    Beta Version 2.0.0, 02-FEB-1989 (HAN)
///
///         If the relational operator is not recognized, an error is
///         signaled. The previous version returned .FALSE. as the
///         function value, and no error was signaled.
///
///         Also, the $Required_Reading section has been changed to
///         include WINDOWS as the required reading for the module.
/// ```
pub fn wnreld(ctx: &mut SpiceContext, a: &[f64], op: &str, b: &[f64]) -> crate::Result<bool> {
    let ret = WNRELD(a, op.as_bytes(), b, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure WNRELD ( Compare two DP windows )
pub fn WNRELD(A: &[f64], OP: &[u8], B: &[f64], ctx: &mut Context) -> f2rust_std::Result<bool> {
    let A = DummyArray::new(A, LBCELL..);
    let B = DummyArray::new(B, LBCELL..);
    let mut WNRELD: bool = false;
    let mut ACARD: i32 = 0;
    let mut BCARD: i32 = 0;
    let mut EQUAL: bool = false;
    let mut SUBSET: bool = false;

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
        WNRELD = false;
        return Ok(WNRELD);
    } else {
        CHKIN(b"WNRELD", ctx)?;
        WNRELD = false;
    }

    //
    // Find the cardinality of the input windows.
    //
    ACARD = CARDD(A.as_slice(), ctx)?;
    BCARD = CARDD(B.as_slice(), ctx)?;

    //
    // A and B are equal if they contain exactly the same intervals.
    // We need to know this for nearly every relationship, so find out
    // before going any further.
    //
    if (ACARD != BCARD) {
        EQUAL = false;
    } else {
        EQUAL = true;

        for I in 1..=ACARD {
            EQUAL = (EQUAL && (A[I] == B[I]));
        }
    }

    //
    // Simple equality and inequality are trivial at this point.
    //
    if fstr::eq(OP, b"=") {
        WNRELD = EQUAL;
    } else if fstr::eq(OP, b"<>") {
        WNRELD = !EQUAL;

    //
    // Subsets are a little trickier. A is a subset of B if every
    // interval in A is included in B. In addition, A is a proper
    // subset if A and B are not equal.
    //
    } else if (fstr::eq(OP, b"<=") || fstr::eq(OP, b"<")) {
        SUBSET = true;

        for I in intrinsics::range(1, ACARD, 2) {
            SUBSET = (SUBSET && WNINCD(A[I], A[(I + 1)], B.as_slice(), ctx)?);
        }

        if fstr::eq(OP, b"<=") {
            WNRELD = SUBSET;
        } else {
            WNRELD = (SUBSET && !EQUAL);
        }

    //
    // A and B change places here...
    //
    } else if (fstr::eq(OP, b">=") || fstr::eq(OP, b">")) {
        SUBSET = true;

        for I in intrinsics::range(1, BCARD, 2) {
            SUBSET = (SUBSET && WNINCD(B[I], B[(I + 1)], A.as_slice(), ctx)?);
        }

        if fstr::eq(OP, b">=") {
            WNRELD = SUBSET;
        } else {
            WNRELD = (SUBSET && !EQUAL);
        }

    //
    // An unrecognized operator always fails.
    //
    } else {
        SETMSG(b"Relational operator, *, is not recognized.", ctx);
        ERRCH(b"*", OP, ctx);
        SIGERR(b"SPICE(INVALIDOPERATION)", ctx)?;
        CHKOUT(b"WNRELD", ctx)?;
        return Ok(WNRELD);
    }

    CHKOUT(b"WNRELD", ctx)?;
    Ok(WNRELD)
}
