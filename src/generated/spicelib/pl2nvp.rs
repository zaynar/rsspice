//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const UBPL: i32 = 4;

/// Plane to normal vector and point
///
/// Return a unit normal vector and point that define a specified
/// plane.
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
///  PLANE      I   A SPICE plane.
///  NORMAL,
///  POINT      O   A unit normal vector and point that define PLANE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  PLANE    is a SPICE plane.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NORMAL,
///  POINT    are, respectively, a unit normal vector and point
///           that define the geometric plane represented by
///           PLANE. Let the symbol < A, B > indicate the inner
///           product of vectors A and B; then the geometric
///           plane is the set of vectors X in three-dimensional
///           space that satisfy
///
///              < X - POINT, NORMAL >  =  0.
///
///           POINT is always the closest point in the input
///           plane to the origin. POINT is always a
///           non-negative scalar multiple of NORMAL.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  The input plane MUST have been created by one of the SPICELIB
///      routines
///
///         NVC2PL ( Normal vector and constant to plane )
///         NVP2PL ( Normal vector and point to plane    )
///         PSV2PL ( Point and spanning vectors to plane )
///
///      Otherwise, the results of this routine are unpredictable.
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
/// ```
///
/// # Examples
///
/// ```text
///  1)  Given a plane normal and constant, find a point in
///      the plane. POINT is the point we seek.
///
///         CALL NVC2PL ( NORMAL, CONST,  PLANE )
///         CALL PL2NVP ( PLANE,  NORMAL, POINT )
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
/// -    SPICELIB Version 1.1.0, 24-AUG-2021 (NJB) (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 01-NOV-1990 (NJB)
/// ```
pub fn pl2nvp(plane: &[f64; 4], normal: &mut [f64; 3], point: &mut [f64; 3]) {
    PL2NVP(plane, normal, point);
}

//$Procedure PL2NVP ( Plane to normal vector and point )
pub fn PL2NVP(PLANE: &[f64], NORMAL: &mut [f64], POINT: &mut [f64]) {
    let PLANE = DummyArray::new(PLANE, 1..=UBPL);
    let mut NORMAL = DummyArrayMut::new(NORMAL, 1..=3);
    let mut POINT = DummyArrayMut::new(POINT, 1..=3);
    let mut CONST: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Note for programmers: validity of the input plane is not
    // checked in the interest of efficiency. The input plane will be
    // valid if it was created by one of the SPICE plane construction
    // routines.
    //
    // Extract the unit normal and constant from the plane. Scaling the
    // unit normal by the constant gives us the closest point in the
    // plane to the origin.

    PL2NVC(PLANE.as_slice(), NORMAL.as_slice_mut(), &mut CONST);
    VSCL(CONST, NORMAL.as_slice(), POINT.as_slice_mut());
}
