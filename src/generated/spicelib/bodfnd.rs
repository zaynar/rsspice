//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const VNMLEN: i32 = 32;
const NUMLEN: i32 = 16;

/// Find values from the kernel pool
///
/// Determine whether values exist for some item for any body
/// in the kernel pool.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
/// * [PCK](crate::required_reading::pck)
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BODY       I   ID code of body.
///  ITEM       I   Item to find ('RADII', 'NUT_AMP_RA', etc.).
///
///  The function returns .TRUE. if ITEM is in the kernel pool, .FALSE.
///  otherwise.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BODY     is the ID code of the body for which the item is
///           requested.
///
///  ITEM     is the item to be returned. Together, the body and item
///           name combine to form a variable name, e.g.,
///
///                 'BODY599_RADII'
///                 'BODY4_POLE_RA'
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if ITEM is in the kernel pool, and
///  .FALSE. if it is not.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  BODVCD, which returns values from the kernel pool, causes an
///  error to be signaled whenever the specified item is not found.
///  In many cases, this is appropriate. However, sometimes the
///  program may attempt to recover, by providing default values,
///  prompting for replacements, and so on.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, default values are substituted for
///  bodies for which axes are not found.
///
///     IF ( BODFND ( TARGET, 'RADII' ) ) THEN
///        CALL BODVCD ( TARGET, 'RADII', 3, N, RADII )
///     ELSE
///        CALL VPACK ( 100.D0, 100.D0, 100.D0, RADII )
///     END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Updated
///         input argument BODY detailed description. Added SPK and PCK to
///         the list of required readings.
///
/// -    SPICELIB Version 1.2.1, 24-OCT-2005 (NJB)
///
///         Header update: calls to BODVAR in example code were replaced
///         with calls to BODVCD. The string 'AXES' and variable AXES
///         were replaced with the string 'RADII' and variable 'RADII'
///         throughout the header.
///
/// -    SPICELIB Version 1.2.0, 15-MAR-2002 (NJB)
///
///         Bug fix: routine was updated to work with string-valued
///         kernel variables.
///
/// -    SPICELIB Version 1.1.0, 17-MAY-1994 (HAN)
///
///         If the value of the function RETURN is .TRUE. upon execution of
///         this module, this function is assigned a default value of
///         either 0, 0.0D0, .FALSE., or blank depending on the type of
///         the function.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn bodfnd(ctx: &mut SpiceContext, body: i32, item: &str) -> crate::Result<bool> {
    let ret = BODFND(body, item.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure BODFND ( Find values from the kernel pool )
pub fn BODFND(BODY: i32, ITEM: &[u8], ctx: &mut Context) -> f2rust_std::Result<bool> {
    let mut BODFND: bool = false;
    let mut DTYPE = [b' '; 1 as usize];
    let mut CODE = [b' '; NUMLEN as usize];
    let mut VARNAM = [b' '; VNMLEN as usize];
    let mut N: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        BODFND = false;
        return Ok(BODFND);
    } else {
        CHKIN(b"BODFND", ctx)?;
    }

    //
    // Construct the variable name from BODY and ITEM.
    //
    fstr::assign(&mut VARNAM, b"BODY");

    INTSTR(BODY, &mut CODE, ctx);
    SUFFIX(&CODE, 0, &mut VARNAM);
    SUFFIX(b"_", 0, &mut VARNAM);
    SUFFIX(ITEM, 0, &mut VARNAM);

    //
    // Search the kernel pool for the item.
    //
    DTPOOL(&VARNAM, &mut FOUND, &mut N, &mut DTYPE, ctx)?;

    //
    // Was anything there?
    //
    BODFND = FOUND;

    CHKOUT(b"BODFND", ctx)?;
    Ok(BODFND)
}
