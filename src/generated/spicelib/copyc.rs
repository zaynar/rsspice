//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Copy a character cell
///
/// Copy the contents of a character cell to another cell.
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
///
///  2)  If length of the elements of the output cell is less than the
///      length of the elements of the input cell, the error
///      SPICE(ELEMENTSTOOSHORT) is signaled.
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
///        CALL COPYC  ( TEMP,   BODIES       )
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
/// -    Beta Version 2.0.0, 09-JAN-1989 (NJB)
///
///         Error signaled if output set elements are not long enough.
///         Length must be at least max of lengths of input elements.
///         Also, calling protocol for EXCESS has been changed. And,
///         elements LBCELL through -2 of control area are now copied to
///         the output cell.
/// ```
pub fn copyc(ctx: &mut SpiceContext, cell: CharArray, copy: CharArrayMut) -> crate::Result<()> {
    COPYC(cell, copy, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure COPYC ( Copy a character cell )
pub fn COPYC(CELL: CharArray, COPY: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<()> {
    let CELL = DummyCharArray::new(CELL, None, LBCELL..);
    let mut COPY = DummyCharArrayMut::new(COPY, None, LBCELL..);
    let mut SIZE: i32 = 0;
    let mut CARD: i32 = 0;
    let mut MOVED: i32 = 0;
    let mut TRUNC: bool = false;
    let mut REQLEN: i32 = 0;

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
    CHKIN(b"COPYC", ctx)?;

    //
    // We need the cardinality of the input cell, and the size of
    // the output cell.
    //
    CARD = CARDC(CELL.as_arg(), ctx)?;
    SIZE = SIZEC(COPY.as_arg(), ctx)?;

    //
    // Start moving the elements, one by one. Stop if the output
    // cell fills up.  Copy the control area too, except for the
    // the size and cardinality values.  Truncation indicator
    // starts at .FALSE.
    //
    TRUNC = false;
    REQLEN = 0;

    MOVED = intrinsics::MIN0(&[CARD, SIZE]);

    for I in 1..=MOVED {
        fstr::assign(COPY.get_mut(I), CELL.get(I));
        //
        // Test for truncation:
        //
        if fstr::ne(COPY.get(I), CELL.get(I)) {
            TRUNC = true;
            REQLEN = intrinsics::MAX0(&[REQLEN, LASTPC(&CELL[I])]);
        }
    }

    for I in LBCELL..=-2 {
        fstr::assign(COPY.get_mut(I), CELL.get(I));
        //
        // Test for truncation:
        //
        if fstr::ne(COPY.get(I), CELL.get(I)) {
            TRUNC = true;
            REQLEN = intrinsics::MAX0(&[REQLEN, LASTPC(&CELL[I])]);
        }
    }

    //
    // Set the cardinality of the output cell.
    //
    SCARDC(MOVED, COPY.as_arg_mut(), ctx)?;

    //
    // We've got an error if the output cell was too small.
    //
    if (SIZE < CARD) {
        EXCESS((CARD - SIZE), b"cell", ctx)?;
        SIGERR(b"SPICE(CELLTOOSMALL)", ctx)?;
        CHKOUT(b"COPYC", ctx)?;
        return Ok(());
    }

    //
    // We also have an error if the output set elements are not long
    // enough.
    //
    if TRUNC {
        SETMSG(
            b"Length of output cell is #.  Length required to contain result is #.",
            ctx,
        );

        ERRINT(b"#", intrinsics::LEN(&COPY[LBCELL]), ctx);
        ERRINT(b"#", REQLEN, ctx);
        SIGERR(b"SPICE(ELEMENTSTOOSHORT)", ctx)?;

        CHKOUT(b"COPYC", ctx)?;
        return Ok(());
    }
    CHKOUT(b"COPYC", ctx)?;
    Ok(())
}
