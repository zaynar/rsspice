//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Insert an item into an integer set
///
/// Insert an item into an integer set.
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
///  ITEM       I   Item to be inserted.
///  A         I-O  Insertion set.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ITEM     is an item which is to be inserted into the specified
///           set. ITEM may or may not already be an element of the
///           set.
///
///  A        is a SPICE set.
///
///           On input, A may or may not contain the input item as an
///           element.
/// ```
///
/// # Detailed Output
///
/// ```text
///  A        on output, contains the union of the input set and the
///           singleton set containing the input item, unless there was
///           not sufficient room in the set for the item to be
///           included, in which case the set is not changed and an
///           error is returned.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the insertion of the element into the set causes an excess
///      of elements, the error SPICE(SETEXCESS) is signaled.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Create an integer set for ten elements, insert items
///     to it and then remove the even values.
///
///
///     Example code begins here.
///
///
///           PROGRAM INSRTI_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           INTEGER                 CARDI
///
///     C
///     C     Local constants.
///     C
///           INTEGER                 LBCELL
///           PARAMETER             ( LBCELL = -5 )
///
///           INTEGER                 SETDIM
///           PARAMETER             ( SETDIM   = 10  )
///
///     C
///     C     Local variables.
///     C
///           INTEGER                 A      ( LBCELL:SETDIM )
///           INTEGER                 EVEN   ( SETDIM        )
///           INTEGER                 I
///           INTEGER                 ITEMS  ( SETDIM        )
///
///     C
///     C     Create a list of items and even numbers.
///     C
///           DATA                    EVEN  /  0,  2,  4,  6,  8,
///          .                                10, 12, 14, 16, 18  /
///
///           DATA                    ITEMS /  0,  1,  1,  2,  3,
///          .                                 5,  8, 10, 13, 21  /
///
///     C
///     C     Initialize the empty set.
///     C
///           CALL VALIDI ( SETDIM, 0, A )
///
///     C
///     C     Insert the list of integers into the set. If the item is
///     C     an element of the set, the set is not changed.
///     C
///           DO I = 1, SETDIM
///
///              CALL INSRTI ( ITEMS(I), A )
///
///           END DO
///
///     C
///     C     Output the original contents of set A.
///     C
///           WRITE(*,*) 'Items in original set A:'
///           WRITE(*,'(10I6)') ( A(I), I = 1, CARDI ( A ) )
///           WRITE(*,*) ' '
///
///     C
///     C     Remove the even values. If the item is not an element of
///     C     the set, the set is not changed.
///     C
///           DO I = 1, SETDIM
///
///              CALL REMOVI ( EVEN(I), A )
///
///           END DO
///
///     C
///     C     Output the contents of A.
///     C
///           WRITE(*,*) 'Odd numbers in set A:'
///           WRITE(*,'(10I6)') ( A(I), I = 1, CARDI ( A ) )
///           WRITE(*,*) ' '
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Items in original set A:
///          0     1     2     3     5     8    10    13    21
///
///      Odd numbers in set A:
///          1     3     5    13    21
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
/// -    SPICELIB Version 2.1.0, 24-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
/// -    SPICELIB Version 2.0.0, 01-NOV-2005 (NJB)
///
///         Code was modified slightly to keep logical structure parallel
///         to that of INSRTC.
///
///         Long error message was updated to include size of
///         set into which insertion was attempted.
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
/// ```
pub fn insrti(ctx: &mut SpiceContext, item: i32, a: &mut [i32]) -> crate::Result<()> {
    INSRTI(item, a, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure INSRTI ( Insert an item into an integer set )
pub fn INSRTI(ITEM: i32, A: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut A = DummyArrayMut::new(A, LBCELL..);
    let mut SIZE: i32 = 0;
    let mut CARD: i32 = 0;
    let mut LAST: i32 = 0;
    let mut IN: bool = false;

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

    CHKIN(b"INSRTI", ctx)?;

    //
    // What are the size and cardinality of the set?
    //
    SIZE = SIZEI(A.as_slice(), ctx)?;
    CARD = CARDI(A.as_slice(), ctx)?;

    //
    // Find the last element of the set which would come before the
    // input item. This will be the item itself, if it is already an
    // element of the set.
    //
    LAST = LSTLEI(ITEM, CARD, A.subarray(1));

    //
    // Is the item already in the set? If not, it needs to be inserted.
    //
    if (LAST > 0) {
        IN = (A[LAST] == ITEM);
    } else {
        IN = false;
    }

    if !IN {
        //
        // If there is room in the set for the new element, then move
        // the succeeding elements back to make room. And update the
        // cardinality for future reference.
        //
        if (CARD < SIZE) {
            for I in intrinsics::range(CARD, (LAST + 1), -1) {
                A[(I + 1)] = A[I];
            }

            A[(LAST + 1)] = ITEM;

            SCARDI((CARD + 1), A.as_slice_mut(), ctx)?;
        } else {
            SETMSG(b"An element could not be inserted into the set due to lack of space; set size is #.", ctx);
            ERRINT(b"#", SIZE, ctx);
            SIGERR(b"SPICE(SETEXCESS)", ctx)?;
        }
    }

    CHKOUT(b"INSRTI", ctx)?;

    Ok(())
}
