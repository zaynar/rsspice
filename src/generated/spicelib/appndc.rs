//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Append an item to a character cell
///
/// Append an item to a character cell.
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
///  ITEM     is a character string which is to be appended to CELL.
///
///  CELL     is a character SPICE cell to which ITEM will be
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
///  3)  If the cell is not large enough to accommodate the addition
///      of a new element, the error SPICE(CELLTOOSMALL) is signaled.
///
///  4)  If the length of the item is longer than the length of the
///      cell, ITEM is truncated on the right.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, the item 'PLUTO' is appended to
///  the character cell PLANETS.
///
///  Before appending 'PLUTO', the cell contains:
///
///  PLANETS (1) = 'MERCURY'
///  PLANETS (2) = 'VENUS'
///  PLANETS (3) = 'EARTH'
///  PLANTES (4) = 'MARS'
///  PLANETS (5) = 'JUPITER'
///  PLANETS (6) = 'SATURN'
///  PLANETS (7) = 'URANUS'
///  PLANETS (8) = 'NEPTUNE'
///
///  The call
///
///     CALL APPNDC ( 'PLUTO', PLANETS )
///
///  appends the element 'PLUTO' at the location PLANETS (9), and the
///  cardinality is updated.
///
///  If the cell is not big enough to accommodate the addition of
///  the item, an error is signaled. In this case, the cell is not
///  altered.
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
///
///         Improved the documentation of CELL in $Detailed_Input and
///         $Detailed_Output. Added entries #1 and #2 to $Exceptions.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (HAN)
/// ```
pub fn appndc(ctx: &mut SpiceContext, item: &str, cell: CharArrayMut) -> crate::Result<()> {
    APPNDC(item.as_bytes(), cell, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure APPNDC ( Append an item to a character cell )
pub fn APPNDC(ITEM: &[u8], CELL: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CELL = DummyCharArrayMut::new(CELL, None, LBCELL..);
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
        CHKIN(b"APPNDC", ctx)?;
    }

    //
    // Check to see if the cell can accommodate the addition of a
    // new item. If there is room, append the item to the cell and
    // reset the cardinality. If the cell cannot accommodate the
    // addition of a new item, signal an error.
    //

    NWCARD = (CARDC(CELL.as_arg(), ctx)? + 1);

    if (NWCARD <= SIZEC(CELL.as_arg(), ctx)?) {
        fstr::assign(CELL.get_mut(NWCARD), ITEM);
        SCARDC(NWCARD, CELL.as_arg_mut(), ctx)?;
    } else {
        SETMSG(
            b"The cell cannot accommodate the addition of the item *.",
            ctx,
        );

        ERRCH(b"*", ITEM, ctx);
        SIGERR(b"SPICE(CELLTOOSMALL)", ctx)?;
    }

    CHKOUT(b"APPNDC", ctx)?;
    Ok(())
}
