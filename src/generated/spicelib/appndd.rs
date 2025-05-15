//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Append an item to a double precision cell
///
/// Append an item to a double precision cell.
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
///  ITEM     is a double precision value which is to be appended to
///           CELL.
///
///  CELL     is a double precision SPICE cell to which ITEM will be
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
///     it and then append several more double precision numbers.
///     Validate the cell into a set and print the result.
///
///
///     Example code begins here.
///
///
///           PROGRAM APPNDD_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           INTEGER                 CARDD
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
///           DOUBLE PRECISION        A      ( LBCELL:SIZE )
///           DOUBLE PRECISION        ITEMS  ( LISTSZ      )
///
///           INTEGER                 I
///
///     C
///     C     Set the list of double precision numbers to be appended
///     C     to the cell.
///     C
///           DATA                    ITEMS /  3.D0,  1.D0,  1.D0,
///          .                                 2.D0,  5.D0,  8.D0,
///          .                                21.D0, 13.D0, 34.D0  /
///
///     C
///     C     Initialize the cell.
///     C
///           CALL SSIZED ( SIZE, A )
///
///     C
///     C     Add a single item to the new cell.
///     C
///           CALL APPNDD ( 0.D0, A )
///
///     C
///     C     Now insert a list of items.
///     C
///           DO I= 1, LISTSZ
///
///              CALL APPNDD ( ITEMS(I), A )
///
///           END DO
///
///     C
///     C     Output the original contents of cell A.
///     C
///           WRITE(*,*) 'Items in original cell A:'
///           WRITE(*,'(15F6.1)') ( A(I), I = 1, CARDD ( A ) )
///
///     C
///     C     Validate the set: remove duplicates and sort the
///     C     elements.
///     C
///           CALL VALIDD ( SIZE, CARDD( A ), A )
///
///     C
///     C     Output the contents of the set A.
///     C
///           WRITE(*,*) 'Items in cell A after VALIDD (now a set):'
///           WRITE(*,'(15F6.1)') ( A(I), I = 1, CARDD ( A ) )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Items in original cell A:
///        0.0   3.0   1.0   1.0   2.0   5.0   8.0  21.0  13.0  34.0
///      Items in cell A after VALIDD (now a set):
///        0.0   1.0   2.0   3.0   5.0   8.0  13.0  21.0  34.0
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
///         Edited the header to comply with NAIF standard.
///         Added complete code example.
///
///         Improved the documentation of CELL in $Detailed_Input and
///         $Detailed_Output. Added entries #1 and #2 to $Exceptions.
///
/// -    SPICELIB Version 1.0.2, 09-NOV-2006 (WLT)
///
///         Corrected typo in $Examples section describing the cell as
///         character instead of d.p.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (HAN)
/// ```
pub fn appndd(ctx: &mut SpiceContext, item: f64, cell: &mut [f64]) -> crate::Result<()> {
    APPNDD(item, cell, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure APPNDD ( Append an item to a double precision cell )
pub fn APPNDD(ITEM: f64, CELL: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
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
        CHKIN(b"APPNDD", ctx)?;
    }

    //
    // Check to see if the cell can accommodate the addition of a
    // new item. If there is room, append the item to the cell and
    // reset the cardinality. If the cell cannot accommodate the
    // addition of a new item, signal an error.
    //

    NWCARD = (CARDD(CELL.as_slice(), ctx)? + 1);

    if (NWCARD <= SIZED(CELL.as_slice(), ctx)?) {
        CELL[NWCARD] = ITEM;
        SCARDD(NWCARD, CELL.as_slice_mut(), ctx)?;
    } else {
        SETMSG(
            b"The cell cannot accommodate the addition of the element *. ",
            ctx,
        );

        ERRDP(b"*", ITEM, ctx);
        SIGERR(b"SPICE(CELLTOOSMALL)", ctx)?;
    }

    CHKOUT(b"APPNDD", ctx)?;
    Ok(())
}
