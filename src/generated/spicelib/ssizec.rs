//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Set the size of a character cell
///
/// Set the size (maximum cardinality) of a character cell.
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
///  SIZE       I   Size (maximum cardinality) of the cell.
///  CELL       O   The cell.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SIZE     is the size (maximum number of elements) of the cell.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CELL     is a cell.
///
///
///             On output, the size of the cell is SIZE. The
///             cardinality of the cell is 0. The rest of the
///             control area is zeroed out.
/// ```
///
/// # Particulars
///
/// ```text
///  The set cardinality (SCARDC, SCARDD, and SCARDI) and set size
///  (SSIZEC, SSIZED, and SSIZEI) routines are typically used to
///  initialize cells for subsequent use. Since all cell routines
///  expect to find the size and cardinality of a cell in place,
///  no cell can be used until both have been set.
/// ```
///
/// # Examples
///
/// ```text
///  In the example below, the size and cardinality of the character
///  cell FRED are set in the main module of the program FLNSTN.
///  Both are subsequently retrieved, and the cardinality changed,
///  in one of its subroutines, WILMA.
///
///        PROGRAM FLNSTN
///
///        CHARACTER*30     FRED ( LBCELL:100 )
///         .
///         .
///        CALL SSIZEC ( 100, FRED )
///         .
///         .
///        CALL WILMA ( FRED )
///         .
///         .
///        STOP
///        END
///
///
///        SUBROUTINE WILMA ( FRED )
///
///        CHARACTER*(*)      FRED  ( LBCELL:* )
///        INTEGER            SIZE
///        INTEGER            CARD
///
///        INTEGER            CARDC
///        INTEGER            SIZEC
///         .
///         .
///        SIZE = SIZEC ( FRED )
///        CARD = CARDC ( FRED )
///         .
///         .
///        CALL SCARDC ( MIN ( SIZE, CARD ), FRED )
///        CALL EXCESS ( CARD-SIZE, 'cell' )
///         .
///         .
///        RETURN
///        END
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (WLT) (IMU) (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 2.0.0, 13-MAR-1989 (NJB)
///
///         Check for invalid size value added. An error
///         is signaled if the value is out of range. The cardinality
///         is now automatically reset to 0. The rest of the control
///         area is now zeroed out.
///
///         The examples have been updated to illustrate set initialization
///         without the use of the EMPTYx routines, which have been
///         removed from the library. Errors in the examples have been
///         removed, also.
/// ```
pub fn ssizec(ctx: &mut SpiceContext, size: i32, cell: CharArrayMut) -> crate::Result<()> {
    SSIZEC(size, cell, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SSIZEC ( Set the size of a character cell )
pub fn SSIZEC(SIZE: i32, CELL: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CELL = DummyCharArrayMut::new(CELL, None, LBCELL..);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SSIZEC", ctx)?;
    }

    //
    // The size must be non-negative.  Other values will be snubbed.
    //
    if (SIZE < 0) {
        SETMSG(
            b"Attempt to set size of cell to invalid value.  The value was #.",
            ctx,
        );
        ERRINT(b"#", SIZE, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"SSIZEC", ctx)?;
        return Ok(());
    }

    //
    // Not much to this.
    //
    ENCHAR(SIZE, &mut CELL[-1], ctx)?;
    ENCHAR(0, &mut CELL[0], ctx)?;

    for I in LBCELL..=-2 {
        ENCHAR(0, &mut CELL[I], ctx)?;
    }

    CHKOUT(b"SSIZEC", ctx)?;
    Ok(())
}
