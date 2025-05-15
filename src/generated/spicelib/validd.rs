//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Validate a double precision set
///
/// Create a valid set from a double precision set array.
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
///  SIZE       I   Size (maximum cardinality) of the set.
///  N          I   Initial no. of (possibly non-distinct) elements.
///  A         I-O  Set to be validated.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SIZE     is the maximum cardinality (number of elements)
///           of the set.
///
///  N        is the number of (possibly non-distinct) elements
///           initially contained in the array used to maintain
///           the set. N cannot be greater than the size of the
///           set.
///
///
///  A        is a set.
///
///
///           On input, A contains N elements beginning at A(1).
///           To create a valid set, the elements are ordered,
///           and duplicate elements are removed. The contents
///           of A(LBCELL) through A(0) are lost during validation.
/// ```
///
/// # Detailed Output
///
/// ```text
///  A        on output, is the set containing the ordered,
///           distinct values in the input array, ready for
///           use with other set routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the size of the set is too small to hold the set BEFORE
///      validation, the error SPICE(INVALIDSIZE) is signaled. The set
///      A is not modified.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is typically used to turn an array which has been
///  initialized through DATA or I/O statements into a set, which
///  can then be used with the other set routines.
///
///  Because a set is ordered and contains distinct values, to
///  create a set from an array, it is necessary to sort the array
///  into the set and remove duplicates. Once the array has been
///  sorted, duplicate elements (adjacent after sorting) are removed.
///  The size and cardinality of the set are initialized, and the
///  set is ready to go.
///
///  Because validation is done in place, there is no chance of
///  overflow.
/// ```
///
/// # Examples
///
/// ```text
///  Empty sets may be initialized with the cell routines SSIZEx.
///  Sets may also be initialized from nonempty set arrays.
///  This process, called validation, is done by the set routines
///  VALIDC and VALIDI. In the following example,
///
///        INTEGER      BODIES  ( LBCELL:100 )
///
///        DATA       ( BODIES(I), I=1,8)       /  3, 301,
///       .                                        3, 399,
///       .                                        5, 501,
///       .                                        6, 601,  /
///
///        CALL VALIDI ( 100, 8, BODIES )
///
///  the integer set BODIES is validated. The size of BODIES set to
///  100. The eight elements of the array (stored in elements 1-8)
///  are sorted, and duplicate elements (in this case, the number 3,
///  which appears twice) are removed, and the cardinality of the set
///  is set to the number of distinct elements, now seven. The set is
///  now ready for use with the rest of the set routines.
///
///  The previous contents of elements LBCELL through 0 are lost
///  during the process of validation.
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
/// -    SPICELIB Version 1.1.0, 05-AUG-2021 (JDR)
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
/// -    Beta Version 2.0.0, 13-MAR-1989 (NJB)
///
///         Now participates in error handling. References to RETURN,
///         CHKIN, and CHKOUT added. Check for adequate set size added.
///
///         The examples have been updated to illustrate set initialization
///         without the use of the EMPTYx routines, which have been
///         removed from the library. Errors in the examples have been
///         removed, also.
/// ```
pub fn validd(ctx: &mut SpiceContext, size: i32, n: i32, a: &mut [f64]) -> crate::Result<()> {
    VALIDD(size, n, a, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure VALIDD ( Validate a double precision set )
pub fn VALIDD(SIZE: i32, N: i32, A: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut A = DummyArrayMut::new(A, LBCELL..);
    let mut CARD: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard error handling:
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"VALIDD", ctx)?;
    }

    //
    // Is the set size big enough?
    //
    if (N > SIZE) {
        SETMSG(
            b"Size of un-validated set is too small.  Size is #, size required is #. ",
            ctx,
        );

        ERRINT(b"#", SIZE, ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"VALIDD", ctx)?;
        return Ok(());
    }

    //
    // Just like it says above. Order the array, and remove duplicates.
    //
    CARD = N;
    RMDUPD(&mut CARD, A.subarray_mut(1));

    //
    // Set the size and cardinality of the input set.
    //
    SSIZED(SIZE, A.as_slice_mut(), ctx)?;
    SCARDD(CARD, A.as_slice_mut(), ctx)?;

    CHKOUT(b"VALIDD", ctx)?;
    Ok(())
}
