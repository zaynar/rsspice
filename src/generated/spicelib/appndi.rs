//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Append an item to an integer cell
///
/// Append an item to an integer cell.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ITEM       I   The item to append.
///  CELL      I-O  The cell to which ITEM will be appended.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ITEM     is an integer value which is to be appended to CELL.
///
///  CELL     is an integer SPICE cell to which ITEM will be
///           appended.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CELL     is the input cell with ITEM appended. ITEM is the last
///           member of CELL.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input cell has invalid cardinality, an error is
///      signaled by a routine in the call tree of this routine.
///
///  2)  If the input cell has invalid size, an error is signaled by a
///      routine in the call tree of this routine.
///
///  3)  If the cell is not big enough to accommodate the addition
///      of a new element, the error SPICE(CELLTOOSMALL) is signaled.
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
///  1) Create a cell for fifteen elements, add a first element to
///     it and then append several more integer numbers. Validate
///     the cell into a set and print the result.
///
///
///     Example code begins here.
///
///
///           PROGRAM APPNDI_EX1
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
///           INTEGER                 LISTSZ
///           PARAMETER             ( LISTSZ = 9  )
///
///           INTEGER                 SIZE
///           PARAMETER             ( SIZE   = 15 )
///
///     C
///     C     Local variables.
///     C
///           INTEGER                 A      ( LBCELL:SIZE )
///
///           INTEGER                 I
///           INTEGER                 ITEMS  ( LISTSZ      )
///
///     C
///     C     Set the list of integers to be appended to the cell.
///     C
///           DATA                    ITEMS /  3,  1,  1 , 2, 5,
///          .                                 8, 21, 13, 34    /
///
///     C
///     C     Initialize the cell.
///     C
///           CALL SSIZEI ( SIZE, A )
///
///     C
///     C     Add a single item to the new cell.
///     C
///           CALL APPNDI ( 0, A )
///
///     C
///     C     Now insert a list of items.
///     C
///           DO I= 1, LISTSZ
///
///              CALL APPNDI ( ITEMS(I), A )
///
///           END DO
///
///     C
///     C     Output the original contents of cell A.
///     C
///           WRITE(*,*) 'Items in original cell A:'
///           WRITE(*,'(15I6)') ( A(I), I = 1, CARDI ( A ) )
///
///     C
///     C     Validate the set: remove duplicates and sort the
///     C     elements.
///     C
///           CALL VALIDI ( SIZE, CARDI( A ), A )
///
///     C
///     C     Output the contents of the set A.
///     C
///           WRITE(*,*) 'Items in cell A after VALIDI (now a set):'
///           WRITE(*,'(15I6)') ( A(I), I = 1, CARDI ( A ) )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Items in original cell A:
///          0     3     1     1     2     5     8    21    13    34
///      Items in cell A after VALIDI (now a set):
///          0     1     2     3     5     8    13    21    34
///
///
///     Note that if the cell is not big enough to accommodate the
///     addition of an item, an error is signaled. In this case, the
///     cell is not altered.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 27-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Improved the documentation of CELL in $Detailed_Input and
///         $Detailed_Output. Added entries #1 and #2 to $Exceptions.
///
/// -    SPICELIB Version 1.0.2, 31-JUL-2002 (NJB)
///
///         Corrected miscellaneous typos in header and in the long
///         error message text.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (HAN)
/// ```
pub fn appndi(ctx: &mut SpiceContext, item: i32, cell: &mut [i32]) -> crate::Result<()> {
    APPNDI(item, cell, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure APPNDI ( Append an item to an integer cell )
pub fn APPNDI(ITEM: i32, CELL: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CELL = DummyArrayMut::new(CELL, LBCELL..);
    let mut NWCARD: i32 = 0;

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
        CHKIN(b"APPNDI", ctx)?;
    }

    //
    // Check to see if the cell can accommodate the addition of a
    // new item. If there is room, append the item to the cell and
    // reset the cardinality. If the cell cannot accommodate the
    // addition of a new item, signal an error.
    //

    NWCARD = (CARDI(CELL.as_slice(), ctx)? + 1);

    if (NWCARD <= SIZEI(CELL.as_slice(), ctx)?) {
        CELL[NWCARD] = ITEM;
        SCARDI(NWCARD, CELL.as_slice_mut(), ctx)?;
    } else {
        SETMSG(
            b"The cell cannot accommodate the addition of the element *. ",
            ctx,
        );

        ERRINT(b"*", ITEM, ctx);
        SIGERR(b"SPICE(CELLTOOSMALL)", ctx)?;
    }

    CHKOUT(b"APPNDI", ctx)?;
    Ok(())
}
