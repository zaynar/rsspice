//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Size of a character cell
///
/// Return the size (maximum cardinality) of a character cell.
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
///      SPICE(INVALIDCARDINALITY) is signaled. SIZEC returns
///      an unspecified value in this case.
///
///  2)  If the input array has invalid size, the error
///      SPICE(INVALIDSIZE) is signaled. SIZEC returns
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
pub fn sizec(ctx: &mut SpiceContext, cell: CharArray) -> crate::Result<i32> {
    let ret = SIZEC(cell, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure SIZEC ( Size of a character cell )
pub fn SIZEC(CELL: CharArray, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let CELL = DummyCharArray::new(CELL, None, LBCELL..);
    let mut SIZEC: i32 = 0;
    let mut SIZE: i32 = 0;
    let mut CARD: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        SIZEC = 0;
        return Ok(SIZEC);
    } else {
        CHKIN(b"SIZEC", ctx)?;
    }
    //
    // Set return value, regardless of validity.
    //
    DECHAR(&CELL[-1], &mut SIZE, ctx)?;
    SIZEC = SIZE;

    //
    // Squeal if something is awry.
    //

    DECHAR(&CELL[0], &mut CARD, ctx)?;

    if (SIZE < 0) {
        SETMSG(b"Invalid cell size.  The size was #.", ctx);
        ERRINT(b"#", SIZE, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"SIZEC", ctx)?;
        return Ok(SIZEC);
    } else if (CARD < 0) {
        SETMSG(b"Invalid cell cardinality.  The cardinality was #.", ctx);
        ERRINT(b"#", CARD, ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"SIZEC", ctx)?;
        return Ok(SIZEC);
    } else if (CARD > SIZE) {
        SETMSG(b"Invalid cell cardinality; cardinality exceeds cell size.  The cardinality was #.  The size was #.", ctx);
        ERRINT(b"#", CARD, ctx);
        ERRINT(b"#", SIZE, ctx);
        SIGERR(b"SPICE(INVALIDCARDINALITY)", ctx)?;
        CHKOUT(b"SIZEC", ctx)?;
        return Ok(SIZEC);
    }

    CHKOUT(b"SIZEC", ctx)?;
    Ok(SIZEC)
}
