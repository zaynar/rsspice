//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Fetch from an integer set
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
///     ... -1 < 0 < 1 < 2 < 3 ...
///
///  Thus, a set may be traversed in order:
///
///     SET( FETCHI ( 1 ) )
///     SET( FETCHI ( 2 ) )
///      .
///      .
///     SET( FETCHI ( CARDI ( SET ) ) )
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the element does not exist, the error SPICE(INVALIDINDEX)
///      is signaled, and the value of FETCHI is zero.
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
///     SET( CARDI ( SET ) )
///
///  Likewise, the elements may be retrieved in the order imposed by
///  their values:
///
///     SET( FETCHI ( 1, SET ) )
///     SET( FETCHI ( 2, SET ) )
///      .
///      .
///     SET( FETCHI ( CARDI ( SET ), SET ) )
///
///  In general, FETCHI ( I, SET ) is not equal to I.
/// ```
///
/// # Examples
///
/// ```text
///  Let SET contain the following elements.
///
///      8
///     32
///      2
///     16
///      4
///
///  Then the code fragment
///
///     DO I = 1, CARDI ( SET )
///        WRITE (*,*) SET(FETCHI(I,SET))
///     END DO
///
///  always produces the following output.
///
///      2
///      4
///      8
///     16
///     32
///
///  The code fragment
///
///     DO I = 1, CARDI ( SET )
///        WRITE (*,*) SET(I)
///     END DO
///
///  produces the same elements in unspecified order.
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
pub fn fetchi(ctx: &mut SpiceContext, nth: i32, set: &[i32]) -> crate::Result<i32> {
    let ret = FETCHI(nth, set, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure FETCHI ( Fetch from an integer set )
pub fn FETCHI(NTH: i32, SET: &[i32], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let SET = DummyArray::new(SET, LBCELL..);
    let mut FETCHI: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Set up the error processing.
    //
    if RETURN(ctx) {
        FETCHI = 0;
        return Ok(FETCHI);
    } else {
        CHKIN(b"FETCHI", ctx)?;
    }

    //
    // Check to see if the N'TH element exists.
    //
    if ((NTH < 1) || (NTH > CARDI(SET.as_slice(), ctx)?)) {
        FETCHI = 0;
        SETMSG(b"NTH element does not exist. NTH was *.", ctx);
        ERRINT(b"*", NTH, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;

    //
    // The great secret is that, for now, sets really are maintained
    // in order, for reasons of efficiency.
    //
    } else {
        FETCHI = NTH;
    }

    CHKOUT(b"FETCHI", ctx)?;
    Ok(FETCHI)
}
