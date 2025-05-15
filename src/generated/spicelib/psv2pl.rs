//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const UBPL: i32 = 4;
const NMLPOS: i32 = 1;
const CONPOS: i32 = 4;

/// Point and spanning vectors to plane
///
/// Make a SPICE plane from a point and two spanning vectors.
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
///  POINT,
///  SPAN1,
///  SPAN2      I   A point and two spanning vectors defining a plane.
///  PLANE      O   An array representing the plane.
/// ```
///
/// # Detailed Input
///
/// ```text
///  POINT,
///  SPAN1,
///  SPAN2    are, respectively, a point and two spanning vectors that
///           define a geometric plane in three-dimensional space. The
///           plane is the set of vectors
///
///              POINT   +   s * SPAN1   +   t * SPAN2
///
///           where `s' and `t' are real numbers. The spanning vectors
///           SPAN1 and SPAN2 must be linearly independent, but they
///           need not be orthogonal or unitized.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PLANE    is a SPICE plane that represents the geometric plane
///           defined by POINT, SPAN1, and SPAN2.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If SPAN1 and SPAN2 are linearly dependent, i.e. the vectors
///      POINT, SPAN1, and SPAN2 do not define a plane, the error
///      SPICE(DEGENERATECASE) is signaled.
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
///  1)  Project a vector V orthogonally onto a plane defined by
///      POINT, SPAN1, and SPAN2. PROJ is the projection we want; it
///      is the closest vector in the plane to V.
///
///         CALL PSV2PL ( POINT,   SPAN1,   SPAN2,   PLANE )
///         CALL VPRJP  ( V,       PLANE,   PROJ           )
///
///
///  2)  Find the plane determined by a spacecraft's position vector
///      relative to a central body and the spacecraft's velocity
///      vector. We assume that all vectors are given in the same
///      coordinate system.
///
///         C
///         C     POS is the spacecraft's position, relative to
///         C     the central body. VEL is the spacecraft's velocity
///         C     vector. POS is a point (vector, if you like) in
///         C     the orbit plane, and it is also one of the spanning
///         C     vectors of the plane.
///         C
///               CALL PSV2PL ( POS, POS, VEL, PLANE )
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
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 31-AUG-2005 (NJB)
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
pub fn psv2pl(
    ctx: &mut SpiceContext,
    point: &[f64; 3],
    span1: &[f64; 3],
    span2: &[f64; 3],
    plane: &mut [f64; 4],
) -> crate::Result<()> {
    PSV2PL(point, span1, span2, plane, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PSV2PL ( Point and spanning vectors to plane )
pub fn PSV2PL(
    POINT: &[f64],
    SPAN1: &[f64],
    SPAN2: &[f64],
    PLANE: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let POINT = DummyArray::new(POINT, 1..=3);
    let SPAN1 = DummyArray::new(SPAN1, 1..=3);
    let SPAN2 = DummyArray::new(SPAN2, 1..=3);
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
    // Find the unitized cross product of SPAN1 and SPAN2; this is our
    // unit normal vector, or possibly its inverse.
    //
    UCRSS(
        SPAN1.as_slice(),
        SPAN2.as_slice(),
        PLANE.subarray_mut(NMLPOS),
    );

    if VZERO(PLANE.subarray(NMLPOS)) {
        CHKIN(b"PSV2PL", ctx)?;
        SETMSG(b"Spanning vectors are parallel.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"PSV2PL", ctx)?;
        return Ok(());
    }

    //
    // Find the plane constant corresponding to the unit normal
    // vector we've found.
    //
    PLANE[CONPOS] = VDOT(PLANE.subarray(NMLPOS), POINT.as_slice());

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
