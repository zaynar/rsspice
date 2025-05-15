//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const UBPL: i32 = 4;
const NMLPOS: i32 = 1;
const CONPOS: i32 = 4;

/// Normal vector and point to plane
///
/// Make a SPICE plane from a normal vector and a point.
///
/// # Required Reading
///
/// * [PLANES](crate::required_reading::planes)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NORMAL,
///  POINT      I   A normal vector and a point defining a plane.
///  PLANE      O   An array representing the plane.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NORMAL,
///  POINT    are, respectively, a normal vector and point that
///           define a plane in three-dimensional space. NORMAL
///           need not be a unit vector. Let the symbol < a, b >
///           indicate the inner product of vectors a and b;
///           then the geometric plane is the set of vectors X
///           in three-dimensional space that satisfy
///
///              < X - POINT, NORMAL >  =  0.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PLANE    is a SPICE plane that represents the geometric
///           plane defined by POINT and NORMAL.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input vector NORMAL is the zero vector, the error
///      SPICE(ZEROVECTOR) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  SPICELIB geometry routines that deal with planes use the `plane'
///  data type to represent input and output planes. This data type
///  makes the subroutine interfaces simpler and more uniform.
///
///  The SPICELIB routines that produce SPICE planes from data that
///  define a plane are:
///
///     NVC2PL ( Normal vector and constant to plane )
///     NVP2PL ( Normal vector and point to plane    )
///     PSV2PL ( Point and spanning vectors to plane )
///
///  The SPICELIB routines that convert SPICE planes to data that
///  define a plane are:
///
///     PL2NVC ( Plane to normal vector and constant )
///     PL2NVP ( Plane to normal vector and point    )
///     PL2PSV ( Plane to point and spanning vectors )
///
///  Any of these last three routines may be used to convert this
///  routine's output, PLANE, to another representation of a
///  geometric plane.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Project a vector V orthogonally onto a plane defined by POINT
///      and NORMAL. PROJ is the projection we want; it is the
///      closest vector in the plane to V.
///
///         CALL NVP2PL ( NORMAL, POINT,  PLANE )
///         CALL VPRJP  ( V,      PLANE,  PROJ  )
///
///
///  2)  Given a point in a plane and a normal vector, find the
///      distance of the plane from the origin. We make a
///      `plane' from the point and normal, then convert the
///      plane to a unit normal and constant. The constant CONST
///      is (according to the specification of PL2NVC) the distance of
///      the plane from the origin.
///
///         CALL NVP2PL ( NORMAL, POINT,  PLANE )
///         CALL PL2NVC ( PLANE,  NORMAL, CONST )
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  G. Thomas and R. Finney, "Calculus and Analytic Geometry,"
///       7th Edition, Addison Wesley, 1988.
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
/// -    SPICELIB Version 1.2.0, 24-AUG-2021 (JDR)
///
///         Added IMPILCIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 30-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VMINUS call.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 01-NOV-1990 (NJB)
/// ```
pub fn nvp2pl(
    ctx: &mut SpiceContext,
    normal: &[f64; 3],
    point: &[f64; 3],
    plane: &mut [f64; 4],
) -> crate::Result<()> {
    NVP2PL(normal, point, plane, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure NVP2PL ( Normal vector and point to plane )
pub fn NVP2PL(
    NORMAL: &[f64],
    POINT: &[f64],
    PLANE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let NORMAL = DummyArray::new(NORMAL, 1..=3);
    let POINT = DummyArray::new(POINT, 1..=3);
    let mut PLANE = DummyArrayMut::new(PLANE, 1..=UBPL);
    let mut TMPVEC = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // The contents of SPICE planes are as follows:
    //
    //    Elements NMLPOS through NMLPOS + 2 contain a unit normal
    //    vector for the plane.
    //
    //    Element CONPOS contains a constant for the plane;  every point
    //    X in the plane satisfies
    //
    //       < X, PLANE(NMLPOS) >  =  PLANE(CONPOS).
    //
    //    The plane constant is the distance of the plane from the
    //    origin; the normal vector, scaled by the constant, is the
    //    closest point in the plane to the origin.
    //
    //

    //
    // Local variables
    //

    //
    // This routine checks in only if an error is discovered.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // The normal vector must be non-zero.
    //
    if VZERO(NORMAL.as_slice()) {
        CHKIN(b"NVP2PL", ctx)?;
        SETMSG(b"Plane\'s normal must be non-zero.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"NVP2PL", ctx)?;
        return Ok(());
    }

    VHAT(NORMAL.as_slice(), PLANE.subarray_mut(NMLPOS));

    PLANE[CONPOS] = VDOT(POINT.as_slice(), PLANE.subarray(NMLPOS));

    //
    // The constant should be the distance of the plane from the
    // origin.  If the constant is negative, negate both it and the
    // normal vector.
    //
    if (PLANE[CONPOS] < 0.0) {
        PLANE[CONPOS] = -PLANE[CONPOS];
        VMINUS(PLANE.subarray(NMLPOS), TMPVEC.as_slice_mut());
        VEQU(TMPVEC.as_slice(), PLANE.subarray_mut(NMLPOS));
    }

    Ok(())
}
