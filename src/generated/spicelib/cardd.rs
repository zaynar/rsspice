//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Cardinality of a double precision cell
///
/// Return the cardinality (number of elements) of a double
/// precision cell.
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
///      SPICE(INVALIDCARDINALITY) is signaled. CARDD returns
///      an unspecified value in this case.
///
///  2)  If the input array has invalid size, the error
///      SPICE(INVALIDSIZE) is signaled. CARDD returns
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
///  cells. In the following example, SIZED is used to determine
///  whether the d.p. cell ORIGINAL can be safely copied into
///  the d.p. cell SAVE before actually attempting the operation.
///  If ORIGINAL contains more elements than SAVE can hold, then
///  the operation would fail.
///
///        IF ( CARDD ( ORIGINAL ) .LE. SIZED ( SAVE ) ) THEN
///           CALL COPYD ( ORIGINAL, SAVE )
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
pub fn cardd(ctx: &mut SpiceContext, cell: &[f64]) -> crate::Result<i32> {
    let ret = CARDD(cell, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure CARDD ( Cardinality of a double precision cell )
pub fn CARDD(CELL: &[f64], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let CELL = DummyArray::new(CELL, LBCELL..);
    let mut CARDD: i32 = 0;

    //
    // SPICELIB functions
    //

    if RETURN(ctx) {
        CARDD = 0;
        return Ok(CARDD);
    } else {
        CHKIN(b"CARDD", ctx)?;
    }

    //
    // Set return value, regardless of validity.
    //
    CARDD = (CELL[0] as i32);

    //
    // Squeal if something is awry.
    //

    if ((CELL[-1] as i32) < 0) {
        SETMSG(b"Invalid cell size.  The size was #.", ctx);
        ERRINT(b"#", (CELL[-1] as i32), ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"CARDD", ctx)?;
        return Ok(CARDD);
    } else if ((CELL[0] as i32) < 0) {
        SETMSG(b"Invalid cell cardinality.  The cardinality was #.", ctx);
        ERRINT(b"#", (CELL[0] as i32), ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"CARDD", ctx)?;
        return Ok(CARDD);
    } else if ((CELL[0] as i32) > (CELL[-1] as i32)) {
        SETMSG(b"Invalid cell cardinality; cardinality exceeds cell size.  The cardinality was #.  The size was #.", ctx);
        ERRINT(b"#", (CELL[0] as i32), ctx);
        ERRINT(b"#", (CELL[-1] as i32), ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"CARDD", ctx)?;
        return Ok(CARDD);
    }

    CHKOUT(b"CARDD", ctx)?;
    Ok(CARDD)
}
