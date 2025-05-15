//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Nearest point on line to point
///
/// Find the nearest point on a line to a specified point, and find
/// the distance between the two points.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LINPT,
///  LINDIR     I   Point on a line and the line's direction vector.
///  POINT      I   A second point.
///  PNEAR      O   Nearest point on the line to POINT.
///  DIST       O   Distance between POINT and PNEAR.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LINPT,
///  LINDIR   are, respectively, a point and a direction vector
///           that define a line in 3-dimensional space. The
///           line is the set of points
///
///              LINPT   +   t * LINDIR
///
///           where `t' is any real number.
///
///  POINT    is a point in 3-dimensional space.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PNEAR    is the nearest point on the input line to the input
///           point.
///
///  DIST     is the distance between the input line and input
///           point.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the line direction vector LINDIR is the zero vector, the
///      error SPICE(ZEROVECTOR) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  For every line L and point P, there is a unique closest point
///  on L to P. Call this closest point C. It is always true that
///  P - C  is perpendicular to L, and the length of P - C is called
///  the `distance' between P and L.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Suppose a line passes through the point ( 1, 2, 3 ) and
///      has direction vector ( 0, 1, 1 ).  We wish to find the
///      closest point on the line to the point ( -6, 9, 10 ).  We
///      can use the code fragment
///
///         LINPT(1)   =  1.D0
///         LINPT(2)   =  2.D0
///         LINPT(3)   =  3.D0
///
///         LINDIR(1)  =  0.D0
///         LINDIR(2)  =  1.D0
///         LINDIR(3)  =  1.D0
///
///         POINT(1)   = -6.D0
///         POINT(2)   =  9.D0
///         POINT(3)   = 10.D0
///
///         CALL NPLNPT ( LINPT, LINDIR, POINT, PNEAR, DIST )
///
///      After the call, PNEAR will take the value
///
///         ( 1.D0, 9.D0, 10.D0 );
///
///      DIST will be 7.0.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 27-MAY-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 17-SEP-2014 (NJB)
///
///         Now uses discovery check-in.
///
/// -    SPICELIB Version 1.1.0, 09-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VADD call.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 02-NOV-1990 (NJB)
/// ```
pub fn nplnpt(
    ctx: &mut SpiceContext,
    linpt: &[f64; 3],
    lindir: &[f64; 3],
    point: &[f64; 3],
    pnear: &mut [f64; 3],
    dist: &mut f64,
) -> crate::Result<()> {
    NPLNPT(linpt, lindir, point, pnear, dist, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure NPLNPT ( Nearest point on line to point )
pub fn NPLNPT(
    LINPT: &[f64],
    LINDIR: &[f64],
    POINT: &[f64],
    PNEAR: &mut [f64],
    DIST: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let LINPT = DummyArray::new(LINPT, 1..=3);
    let LINDIR = DummyArray::new(LINDIR, 1..=3);
    let POINT = DummyArray::new(POINT, 1..=3);
    let mut PNEAR = DummyArrayMut::new(PNEAR, 1..=3);
    let mut PROJ = StackArray::<f64, 3>::new(1..=3);
    let mut TRANS = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // We need a real direction vector to work with.
    //
    if VZERO(LINDIR.as_slice()) {
        CHKIN(b"NPLNPT", ctx)?;
        SETMSG(b"Direction vector must be non-zero.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"NPLNPT", ctx)?;
        return Ok(());
    }

    //
    // We translate line and input point so as to put the line through
    // the origin.  Then the nearest point on the translated line to the
    // translated point TRANS is the projection of TRANS onto the line.
    //
    VSUB(POINT.as_slice(), LINPT.as_slice(), TRANS.as_slice_mut());
    VPROJ(TRANS.as_slice(), LINDIR.as_slice(), PROJ.as_slice_mut());
    VADD(PROJ.as_slice(), LINPT.as_slice(), PNEAR.as_slice_mut());

    *DIST = VDIST(PNEAR.as_slice(), POINT.as_slice());

    Ok(())
}
