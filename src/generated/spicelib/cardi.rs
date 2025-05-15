//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Cardinality of an integer cell
///
/// Return the cardinality (number of elements) of an integer cell.
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
///      SPICE(INVALIDCARDINALITY) is signaled. CARDI returns
///      an unspecified value in this case.
///
///  2)  If the input array has invalid size, the error
///      SPICE(INVALIDSIZE) is signaled. CARDI returns
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
///  cells. In the following example, SIZEI is used to determine
///  whether the integer cell ORIGINAL can be safely copied into
///  the integer cell SAVE before actually attempting the operation.
///  If ORIGINAL contains more elements than SAVE can hold, then
///  the operation would fail.
///
///        IF ( CARDI ( ORIGINAL ) .LE. SIZEI ( SAVE ) ) THEN
///           CALL COPYI ( ORIGINAL, SAVE )
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
pub fn cardi(ctx: &mut SpiceContext, cell: &[i32]) -> crate::Result<i32> {
    let ret = CARDI(cell, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure CARDI ( Cardinality of an integer cell )
pub fn CARDI(CELL: &[i32], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let CELL = DummyArray::new(CELL, LBCELL..);
    let mut CARDI: i32 = 0;

    //
    // SPICELIB functions
    //

    if RETURN(ctx) {
        CARDI = 0;
        return Ok(CARDI);
    } else {
        CHKIN(b"CARDI", ctx)?;
    }

    //
    // Set return value, regardless of validity.
    //
    CARDI = CELL[0];

    //
    // Squeal if something is awry.
    //

    if (CELL[-1] < 0) {
        SETMSG(b"Invalid cell size.  The size was #.", ctx);
        ERRINT(b"#", CELL[-1], ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"CARDI", ctx)?;
        return Ok(CARDI);
    } else if (CELL[0] < 0) {
        SETMSG(b"Invalid cell cardinality.  The cardinality was #.", ctx);
        ERRINT(b"#", CELL[0], ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"CARDI", ctx)?;
        return Ok(CARDI);
    } else if (CELL[0] > CELL[-1]) {
        SETMSG(b"Invalid cell cardinality; cardinality exceeds cell size.  The cardinality was #.  The size was #.", ctx);
        ERRINT(b"#", CELL[0], ctx);
        ERRINT(b"#", CELL[-1], ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"CARDI", ctx)?;
        return Ok(CARDI);
    }

    CHKOUT(b"CARDI", ctx)?;
    Ok(CARDI)
}
