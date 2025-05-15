//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

/// Fetch from a character set
///
/// Return the location within the set array of the NTH element within
/// the order imposed by the ASCII collating sequence.
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
///  The function returns the location within the set array of the
///  NTH element within the order imposed by the ASCII collating
///  sequence. Thus, a set may be traversed in order:
///
///     SET( FETCHC ( 1 ) )
///     SET( FETCHC ( 2 ) )
///      .
///      .
///     SET( FETCHC ( CARDC ( SET ) ) )
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the element does not exist, the error SPICE(INVALIDINDEX)
///      is signaled, and the value of FETCHC is zero.
/// ```
///
/// # Particulars
///
/// ```text
///  Within a set, the elements may be stored in arbitrary
///  order. The elements of a set may be retrieved by stepping
///  through the set array:
///
///     SET( 1 )
///     SET( 2 )
///      .
///      .
///     SET( CARDC ( SET ) )
///
///  Likewise, the elements may be retrieved in the order imposed by
///  the ASCII collating sequence, by using FETCHC:
///
///     SET( FETCHC ( 1, SET ) )
///     SET( FETCHC ( 2, SET ) )
///      .
///      .
///     SET( FETCHC ( CARDC ( SET ), SET ) )
///
///  In general, FETCHC ( I, SET ) is not equal to I.
/// ```
///
/// # Examples
///
/// ```text
///  Let SET contain the following elements.
///
///     'Feynman'
///     'Einstein'
///     'Bohr'
///     'Newton'
///
///  Then the code fragment
///
///     DO I = 1, CARDC ( SET )
///        WRITE (*,*) SET(FETCHC(I,SET))
///     END DO
///
///  always produces the following output.
///
///     Bohr
///     Einstein
///     Feynman
///     Newton
///
///  The code fragment
///
///     DO I = 1, CARDC ( SET )
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
/// -    SPICELIB Version 1.2.0, 27-AUG-2021 (JDR)
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
pub fn fetchc(ctx: &mut SpiceContext, nth: i32, set: CharArray) -> crate::Result<i32> {
    let ret = FETCHC(nth, set, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure FETCHC ( Fetch from a character set )
pub fn FETCHC(NTH: i32, SET: CharArray, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let SET = DummyCharArray::new(SET, None, LBCELL..);
    let mut FETCHC: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Set up the error processing.
    //
    if RETURN(ctx) {
        FETCHC = 0;
        return Ok(FETCHC);
    } else {
        CHKIN(b"FETCHC", ctx)?;
    }

    //
    // Check to see if the N'TH element exists.
    //
    if ((NTH < 1) || (NTH > CARDC(SET.as_arg(), ctx)?)) {
        FETCHC = 0;
        SETMSG(b"NTH element does not exist. NTH was *.", ctx);
        ERRINT(b"*", NTH, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;

    //
    // The great secret is that, for now, sets really are maintained
    // in order, for reasons of efficiency.
    //
    } else {
        FETCHC = NTH;
    }

    CHKOUT(b"FETCHC", ctx)?;
    Ok(FETCHC)
}
