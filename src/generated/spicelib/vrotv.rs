//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector rotation about an axis
///
/// Rotate a vector about a specified axis vector by a specified
/// angle and return the rotated vector.
///
/// # Required Reading
///
/// * [ROTATION](crate::required_reading::rotation)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V          I   Vector to be rotated.
///  AXIS       I   Axis of the rotation.
///  THETA      I   Angle of rotation (radians).
///  R          O   Result of rotating V about AXIS by THETA.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V        is a 3-dimensional vector to be rotated.
///
///  AXIS     is the axis about which the rotation is to be
///           performed.
///
///  THETA    is the angle through which V is to be rotated about
///           AXIS.
/// ```
///
/// # Detailed Output
///
/// ```text
///  R        is the result of rotating V about AXIS by THETA.
///           If AXIS is the zero vector, R = V.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the input axis is the zero vector, R will be returned
///      as V.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes the result of rotating (in a right handed
///  sense) the vector V about the axis represented by AXIS through
///  an angle of THETA radians.
///
///  If W is a unit vector parallel to AXIS, then R is given by:
///
///      R = V + ( 1 - cos(THETA) ) Wx(WxV) + sin(THETA) (WxV)
///
///  where "x" above denotes the vector cross product.
/// ```
///
/// # Examples
///
/// ```text
///  If AXIS = ( 0, 0, 1 ) and THETA = PI/2 then the following results
///  for R will be obtained
///
///          V                           R
///     -------------             ----------------
///     ( 1, 2, 3 )                ( -2, 1, 3 )
///     ( 1, 0, 0 )                (  0, 1, 0 )
///     ( 0, 1, 0 )                ( -1, 0, 0 )
///
///
///  If AXIS = ( 0, 1, 0 ) and THETA = PI/2 then the following results
///  for R will be obtained
///
///          V                           R
///     -------------             ----------------
///     ( 1, 2, 3 )                (  3, 2, -1 )
///     ( 1, 0, 0 )                (  0, 0, -1 )
///     ( 0, 1, 0 )                (  0, 1,  0 )
///
///
///  If AXIS = ( 1, 1, 1 ) and THETA = PI/2 then the following results
///  for R will be obtained
///
///          V                                     R
///     -----------------------------   -----------------------------
///     ( 1.0,     2.0,     3.0     )   ( 2.577.., 0.845.., 2.577.. )
///     ( 2.577.., 0.845.., 2.577.. )   ( 3.0      2.0,     1.0     )
///     ( 3.0      2.0,     1.0     )   ( 1.422.., 3.154.., 1.422.. )
///     ( 1.422.., 3.154.., 1.422.. )   ( 1.0      2.0,     3.0     )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.0.2, 05-FEB-2003 (NJB)
///
///         Header examples were corrected.  $Exceptions section
///         filled in. Miscellaneous header corrections were made.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (NJB) (HAN)
/// ```
pub fn vrotv(v: &[f64; 3], axis: &[f64; 3], theta: f64, r: &mut [f64; 3]) {
    VROTV(v, axis, theta, r);
}

//$Procedure VROTV ( Vector rotation about an axis )
pub fn VROTV(V: &[f64], AXIS: &[f64], THETA: f64, R: &mut [f64]) {
    let V = DummyArray::new(V, 1..=3);
    let AXIS = DummyArray::new(AXIS, 1..=3);
    let mut R = DummyArrayMut::new(R, 1..=3);
    let mut C: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut RPLANE = StackArray::<f64, 3>::new(1..=3);
    let mut P = StackArray::<f64, 3>::new(1..=3);
    let mut V1 = StackArray::<f64, 3>::new(1..=3);
    let mut V2 = StackArray::<f64, 3>::new(1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Just in case the user tries to rotate about the zero vector -
    // check, and if so return the input vector
    //
    if (VNORM(AXIS.as_slice()) == 0.0) {
        MOVED(V.as_slice(), 3, R.as_slice_mut());
        return;
    }

    //
    // Compute the unit vector that lies in the direction of the
    // AXIS.  Call it X.
    //
    VHAT(AXIS.as_slice(), X.as_slice_mut());

    //
    // Compute the projection of V onto AXIS.  Call it P.
    //
    VPROJ(V.as_slice(), X.as_slice(), P.as_slice_mut());

    //
    // Compute the component of V orthogonal to the AXIS.  Call it V1.
    //
    VSUB(V.as_slice(), P.as_slice(), V1.as_slice_mut());

    //
    // Rotate V1 by 90 degrees about the AXIS and call the result V2.
    //
    VCRSS(X.as_slice(), V1.as_slice(), V2.as_slice_mut());

    //
    // Compute COS(THETA)*V1 + SIN(THETA)*V2. This is V1 rotated about
    // the AXIS in the plane normal to the axis, call the result RPLANE
    //
    C = f64::cos(THETA);
    S = f64::sin(THETA);

    VLCOM(C, V1.as_slice(), S, V2.as_slice(), RPLANE.as_slice_mut());

    //
    // Add the rotated component in the normal plane to AXIS to the
    // projection of V onto AXIS (P) to obtain R.
    //
    VADD(RPLANE.as_slice(), P.as_slice(), R.as_slice_mut());

    //
}
