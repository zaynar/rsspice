//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Size of a double precision cell
///
/// Return the size (maximum cardinality) of a double precision
/// cell.
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
///  The function returns the size of the input cell.
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
///  The function returns the size of (maximum number of elements in)
///  the input cell.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input array has invalid cardinality, the error
///      SPICE(INVALIDCARDINALITY) is signaled. SIZEI returns
///      an unspecified value in this case.
///
///  2)  If the input array has invalid size, the error
///      SPICE(INVALIDSIZE) is signaled. SIZEI returns
///      an unspecified value in this case.
/// ```
///
/// # Examples
///
/// ```text
///  The size (SIZE) functions are typically used in conjunction
///  with the cardinality functions to predict (and subsequently
///  avoid) overflows when manipulating cells. In the following
///  example, SIZEI is used to determine whether the integer cell
///  ORIGINAL can be safely copied into the integer cell SAVE before
///  actually attempting the operation. (If ORIGINAL contains more
///  elements than SAVE is capable of holding, then the operation
///  will fail.)
///
///        IF ( CARDI ( ORIGINAL ) .LE. SIZEI ( SAVE ) ) THEN
///           CALL COPYI ( ORIGINAL, SAVE, ERROR )
///
///        ELSE
///         .
///         .
///        END DO
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
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         If the value of the function RETURN is .TRUE. upon execution of
///         this module, this function is assigned a default value of
///         either 0, 0.0D0, .FALSE., or blank depending on the type of the
///         function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (WLT) (IMU) (NJB)
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
pub fn sized(ctx: &mut SpiceContext, cell: &[f64]) -> crate::Result<i32> {
    let ret = SIZED(cell, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure SIZED ( Size of a double precision cell )
pub fn SIZED(CELL: &[f64], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let CELL = DummyArray::new(CELL, LBCELL..);
    let mut SIZED: i32 = 0;

    //
    // SPICELIB functions
    //

    if RETURN(ctx) {
        SIZED = 0;
        return Ok(SIZED);
    } else {
        CHKIN(b"SIZED", ctx)?;
    }

    //
    // Set return value, regardless of validity.
    //
    SIZED = (CELL[-1] as i32);

    //
    // Squeal if something is awry.
    //

    if ((CELL[-1] as i32) < 0) {
        SETMSG(b"Invalid cell size.  The size was #.", ctx);
        ERRINT(b"#", (CELL[-1] as i32), ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"SIZED", ctx)?;
        return Ok(SIZED);
    } else if ((CELL[0] as i32) < 0) {
        SETMSG(b"Invalid cell cardinality.  The cardinality was #.", ctx);
        ERRINT(b"#", (CELL[0] as i32), ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"SIZED", ctx)?;
        return Ok(SIZED);
    } else if ((CELL[0] as i32) > (CELL[-1] as i32)) {
        SETMSG(b"Invalid cell cardinality; cardinality exceeds cell size.  The cardinality was #.  The size was #.", ctx);
        ERRINT(b"#", (CELL[0] as i32), ctx);
        ERRINT(b"#", (CELL[-1] as i32), ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"SIZED", ctx)?;
        return Ok(SIZED);
    }

    CHKOUT(b"SIZED", ctx)?;
    Ok(SIZED)
}
