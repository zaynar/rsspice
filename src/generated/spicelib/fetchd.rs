//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Fetch from a DP set
///
/// Return the location within the set array of the NTH element
/// within the order imposed by the values of the elements.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NTH        I   Index of a particular element.
///  SET        I   Input set.
///
///  The function returns the location of the NTH element in the set.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NTH      is an index to an element of a set. If the set is to
///           be conceived as sorted in increasing order, then the
///           NTH element of a set is well defined.
///
///  SET      is a set.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the location within the set array of the NTH
///  element within the order imposed by the values of the elements,
///
///     ... -1.D0 < 0.D0 < 1.D0 < 2.D0 < 3.D0 ...
///
///  Thus, a set may be traversed in order:
///
///     SET( FETCHD ( 1 ) )
///     SET( FETCHD ( 2 ) )
///      .
///      .
///     SET( FETCHD ( CARDD ( SET ) ) )
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the element does not exist, the error SPICE(INVALIDINDEX)
///      is signaled, and the value of FETCHD is zero.
/// ```
///
/// # Particulars
///
/// ```text
///  Within a set, the elements may be stored in arbitrary order.
///  The elements of a set may be retrieved by stepping through the
///  set array:
///
///     SET( 1 )
///     SET( 2 )
///      .
///      .
///     SET( CARDD ( SET ) )
///
///  Likewise, the elements may be retrieved in the order imposed by
///  their values:
///
///     SET( FETCHD ( 1, SET ) )
///     SET( FETCHD ( 2, SET ) )
///      .
///      .
///     SET( FETCHD ( CARDD ( SET ), SET ) )
///
///  In general, FETCHD ( I, SET ) is not equal to I.
/// ```
///
/// # Examples
///
/// ```text
///  Let SET contain the following elements.
///
///      8.D0
///     32.D0
///      2.D0
///     16.D0
///      4.D0
///
///  Then the code fragment
///
///     DO I = 1, CARDD ( SET )
///        WRITE (*,*) SET(FETCHD(I,SET))
///     END DO
///
///  always produces the following output.
///
///      2.D0
///      4.D0
///      8.D0
///     16.D0
///     32.D0
///
///   The code fragment
///
///     DO I = 1, CARDD ( SET )
///        WRITE (*,*) SET(I)
///     END DO
///
///   produces the same elements in unspecified order.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
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
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn fetchd(ctx: &mut SpiceContext, nth: i32, set: &[f64]) -> crate::Result<i32> {
    let ret = FETCHD(nth, set, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure FETCHD ( Fetch from a DP set )
pub fn FETCHD(NTH: i32, SET: &[f64], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let SET = DummyArray::new(SET, LBCELL..);
    let mut FETCHD: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Set up the error processing.
    //
    if RETURN(ctx) {
        FETCHD = 0;
        return Ok(FETCHD);
    } else {
        CHKIN(b"FETCHD", ctx)?;
    }

    //
    // Check to see if the N'TH element exists.
    //
    if ((NTH < 1) || (NTH > CARDD(SET.as_slice(), ctx)?)) {
        FETCHD = 0;
        SETMSG(b"NTH element does not exist. NTH was *.", ctx);
        ERRINT(b"*", NTH, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
    //
    // The great secret is that, for now, sets really are maintained
    // in order, for reasons of efficiency.
    //
    } else {
        FETCHD = NTH;
    }

    CHKOUT(b"FETCHD", ctx)?;
    Ok(FETCHD)
}
