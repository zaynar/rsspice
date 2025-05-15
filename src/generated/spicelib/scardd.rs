//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Set the cardinality of a double precision cell
///
/// Set the cardinality of a double precision cell.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CARD       I   Cardinality of (number of elements in) the cell.
///  CELL       O   The cell.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CARD     is the cardinality of (number of elements in) the
///           cell.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CELL     is a cell.
///
///
///             On output, the cardinality of the cell is CARD.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the cardinality value supplied is less than 0 or greater
///      than the cell size, the error SPICE(INVALIDCARDINALITY) is
///      signaled.
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
///
///  The set cardinality routines are also used by library routines
///  which manipulate cells (including set and window routines) to
///  reset the cardinalities of cells as they gain or lose elements.
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
///         Check for invalid cardinality value added. An error
///         is signaled if the value is out of range. Examples
///         updated so as not to refer to the EMPTYx routines, and
///         to show the correct calling protocol for EXCESS.
/// ```
pub fn scardd(ctx: &mut SpiceContext, card: i32, cell: &mut [f64]) -> crate::Result<()> {
    SCARDD(card, cell, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SCARDD ( Set the cardinality of a double precision cell )
pub fn SCARDD(CARD: i32, CELL: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CELL = DummyArrayMut::new(CELL, LBCELL..);

    //
    // SPICELIB functions
    //

    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SCARDD", ctx)?;
    }

    //
    // The cardinality may range from 0 to the size of the cell,
    // inclusive.  Other values will be snubbed.
    //
    if ((CARD < 0) || (CARD > (CELL[-1] as i32))) {
        SETMSG(
            b"Attempt to set cardinality of cell to invalid value.  The value was #.",
            ctx,
        );
        ERRINT(b"#", CARD, ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"SCARDD", ctx)?;
        return Ok(());
    }

    //
    // Not much to this.
    //
    CELL[0] = (CARD as f64);

    CHKOUT(b"SCARDD", ctx)?;
    Ok(())
}
