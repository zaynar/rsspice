//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Remove an item from an integer set
///
/// Remove an item from an integer set.
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
///  ITEM       I   Item to be removed.
///  A         I-O  Removal set.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ITEM     is an item which is to be removed from the specified set.
///           ITEM may or may not already be an element of the set.
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
///  A        on output, contains the difference of the input set and
///           the input item. If the item is not an element of the set,
///           the set is not changed.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input set A has invalid cardinality, an error is
///      signaled by a routine in the call tree of this routine.
///
///  2)  If the input set A has invalid size, an error is signaled by a
///      routine in the call tree of this routine.
///
///  3)  The data values in set A must be monotone strictly increasing.
///      This is not checked. If this condition is not met, the results
///      are unpredictable.
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
///           PROGRAM REMOVI_EX1
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
///           WRITE(*,'(10I6)') ( A(I), I=1, CARDI ( A ) )
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
///           WRITE(*,'(10I6)') ( A(I), I=1, CARDI ( A ) )
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
/// -    SPICELIB Version 1.1.0, 24-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example. Extended the $Exceptions section.
///
///         Removed unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (WLT) (IMU) (NJB)
/// ```
pub fn removi(ctx: &mut SpiceContext, item: i32, a: &mut [i32]) -> crate::Result<()> {
    REMOVI(item, a, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure REMOVI ( Remove an item from an integer set )
pub fn REMOVI(ITEM: i32, A: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut A = DummyArrayMut::new(A, LBCELL..);
    let mut CARD: i32 = 0;
    let mut LOC: i32 = 0;
    let mut IN: bool = false;

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
        CHKIN(b"REMOVI", ctx)?;
    }
    //
    // What is the cardinality of the set?
    //
    CARD = CARDI(A.as_slice(), ctx)?;

    //
    // Determine the location (if any) of the item within the set.
    //
    LOC = BSRCHI(ITEM, CARD, A.subarray(1));

    //
    // Is the item in the set? If so, it needs to be removed.
    //
    IN = (LOC > 0);

    if IN {
        //
        // Move succeeding elements forward to take up the slack left
        // by the departing element. And update the cardinality for
        // future reference.
        //
        for I in LOC..=(CARD - 1) {
            A[I] = A[(I + 1)];
        }

        SCARDI((CARD - 1), A.as_slice_mut(), ctx)?;
    }

    CHKOUT(b"REMOVI", ctx)?;
    Ok(())
}
