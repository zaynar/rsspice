//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Cardinality of a character cell
///
/// Return the cardinality (number of elements) of a character cell.
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
///  CELL       I   Input cell.
///
///  The function returns the cardinality of the input cell.
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
///  The function returns the cardinality of (number of elements in)
///  the input cell.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input array has invalid cardinality, the error
///      SPICE(INVALIDCARDINALITY) is signaled. CARDC returns
///      an unspecified value in this case.
///
///  2)  If the input array has invalid size, the error
///      SPICE(INVALIDSIZE) is signaled. CARDC returns
///      an unspecified value in this case.
/// ```
///
/// # Examples
///
/// ```text
///  The cardinality (CARD) functions are typically used to process
///  each of the elements of a cell. In the following example, CARDC
///  is used to step through the individual elements of the character
///  cell NAMES.
///
///        DO I = 1, CARDC ( NAMES )
///         .
///         .
///        END DO
///
///  In conjunction with the size (SIZE) functions, they may be used
///  to predict (and subsequently avoid) overflows when manipulating
///  cells. In the following example, SIZEC is used to determine
///  whether the character cell ORIGINAL can be safely copied into
///  the character cell SAVE before actually attempting the operation.
///  If ORIGINAL contains more elements than SAVE can hold, then
///  the operation would fail.
///
///        IF ( CARDC ( ORIGINAL ) .LE. SIZEC ( SAVE ) ) THEN
///           CALL COPYC ( ORIGINAL, SAVE )
///
///        ELSE
///         .
///         .
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  C.A. Curzon        (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.1, 29-JUL-2002 (NJB)
///
///         Errors in code fragments in the $Examples section of
///         the header were corrected.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///        If the value of the function RETURN is .TRUE. upon execution of
///        this module, this function is assigned a default value of
///        either 0, 0.0D0, .FALSE., or blank depending on the type of the
///        function.
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
/// -    Beta Version 2.0.0, 13-MAR-1989 (NJB)
///
///         Check for valid input cell added. The input cell must
///         have valid size and cardinality values.
/// ```
pub fn cardc(ctx: &mut SpiceContext, cell: CharArray) -> crate::Result<i32> {
    let ret = CARDC(cell, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure CARDC ( Cardinality of a character cell )
pub fn CARDC(CELL: CharArray, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let CELL = DummyCharArray::new(CELL, None, LBCELL..);
    let mut CARDC: i32 = 0;
    let mut CARD: i32 = 0;
    let mut SIZE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        CARDC = 0;
        return Ok(CARDC);
    } else {
        CHKIN(b"CARDC", ctx)?;
    }
    //
    // Set return value, regardless of validity.
    //
    DECHAR(&CELL[0], &mut CARD, ctx)?;
    CARDC = CARD;

    //
    // Squeal if something is awry.
    //

    DECHAR(&CELL[-1], &mut SIZE, ctx)?;

    if (SIZE < 0) {
        SETMSG(b"Invalid cell size.  The size was #.", ctx);
        ERRINT(b"#", SIZE, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"CARDC", ctx)?;
        return Ok(CARDC);
    } else if (CARD < 0) {
        SETMSG(b"Invalid cell cardinality.  The cardinality was #.", ctx);
        ERRINT(b"#", CARD, ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"CARDC", ctx)?;
        return Ok(CARDC);
    } else if (CARD > SIZE) {
        SETMSG(b"Invalid cell cardinality; cardinality exceeds cell size.  The cardinality was #.  The size was #.", ctx);
        ERRINT(b"#", CARD, ctx);
        ERRINT(b"#", SIZE, ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"CARDC", ctx)?;
        return Ok(CARDC);
    }

    CHKOUT(b"CARDC", ctx)?;
    Ok(CARDC)
}
