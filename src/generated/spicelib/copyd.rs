//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Copy a double precision cell
///
/// Copy the contents of a double precision cell to another cell.
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
///  CELL       I   Cell to be copied.
///  COPY       O   New cell.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CELL     is a cell.
/// ```
///
/// # Detailed Output
///
/// ```text
///  COPY     is a cell which contains the same elements as the
///           input cell, in the same order. If the size (maximum
///           cardinality) of the output cell is smaller than
///           the cardinality of the input cell, then only as many
///           items as will fit in the output cell are copied,
///           and an error is signaled.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the output cell in not large enough to hold the elements
///      of the input cell, the error SPICE(CELLTOOSMALL) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  The copy routines (COPYC, COPYD, and COPYI) are used primarily
///  to manipulate working cells, since many routines that use cells
///  (binary set routines, for instance) do not allow cells to be
///  combined or manipulated in place.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, COPYC is used to copy the result
///  of the union of two sets (ordered cells) from a temporary
///  working set back into the one of the original set.
///
///        CALL UNIONC ( BODIES, PLANETS, TEMP )
///        CALL COPYC  ( TEMP,   BODIES        )
///
///  If the size of the temporary cell is greater than the size
///  of the original set, the function FAILED should be checked to be
///  sure that no overflow occurred. If BODIES is at least as
///  large as TEMP, no such check is necessary.
/// ```
///
/// # Author and Institution
///
/// ```text
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 09-JAN-1989 (NJB)
///
///         Calling protocol for EXCESS has been changed. Call to SETMSG
///         has been removed.
/// ```
pub fn copyd(ctx: &mut SpiceContext, cell: &[f64], copy: &mut [f64]) -> crate::Result<()> {
    COPYD(cell, copy, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure COPYD ( Copy a double precision cell )
pub fn COPYD(CELL: &[f64], COPY: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let CELL = DummyArray::new(CELL, LBCELL..);
    let mut COPY = DummyArrayMut::new(COPY, LBCELL..);
    let mut SIZE: i32 = 0;
    let mut CARD: i32 = 0;
    let mut MOVED: i32 = 0;

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
    CHKIN(b"COPYD", ctx)?;

    //
    // We need the cardinality of the input cell, and the size of
    // the output cell.
    //
    CARD = CARDD(CELL.as_slice(), ctx)?;
    SIZE = SIZED(COPY.as_slice(), ctx)?;

    //
    // Start moving the elements, one by one. Stop if the output
    // cell fills up.
    //
    MOVED = intrinsics::MIN0(&[CARD, SIZE]);

    for I in 1..=MOVED {
        COPY[I] = CELL[I];
    }

    //
    // Set the cardinality of the output cell. Report any excess.
    //
    SCARDD(MOVED, COPY.as_slice_mut(), ctx)?;

    if (CARD > SIZE) {
        EXCESS((CARD - SIZE), b"cell", ctx)?;
        SIGERR(b"SPICE(CELLTOOSMALL)", ctx)?;
        CHKOUT(b"COPYD", ctx)?;
        return Ok(());
    }

    CHKOUT(b"COPYD", ctx)?;
    Ok(())
}
