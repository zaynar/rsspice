//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector distance
///
/// Return the distance between two three-dimensional vectors.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///
///  V1,
///  V2         I   Two 3-vectors.
///
///  The function returns the distance between V1 and V2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1,
///  V2       are two vectors in three-dimensional space, the
///           distance between which is desired.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the distance between V1 and V2. This is
///  defined as
///
///     ||  V1 - V2  ||,
///
///  where || x || indicates the Euclidean norm of the vector x.
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
///  This function is simply shorthand for the code
///
///     CALL VSUB ( V1, V2, DIFF )
///
///     DIST = VNORM ( DIFF )
///
///  Using this function saves you the annoyance of declaring local
///  storage for the difference vector DIFF.
///
///
///  The Euclidean norm of a three-dimensional vector (x, y, z) is
///  defined as
///
///                                  1/2
///          2        2        2
///     (   x    +   y    +   z    ).
///
///
///  This number is the distance of the point (x, y, z) from the
///  origin. If A and B are two vectors whose components are
///
///     ( A(1), A(2), A(3) )    and    ( B(1), B(2), B(3) ),
///
///  then the distance between A and B is the norm of the difference
///  A - B, which has components
///
///
///     (  A(1) - B(1),  A(2) - B(2),  A(3) - B(3)  ).
///
///
///  A related routine is VDISTG, which computes the distance between
///  two vectors of general dimension.
/// ```
///
/// # Examples
///
/// ```text
///  1)  If V1 is
///
///         ( 2.0D0,  3.0D0,  0.D0 )
///
///      and V2 is
///
///         ( 5.0D0,  7.0D0,  12.D0 ),
///
///      VDIST (V1, V2) will be 13.D0.
///
///
///  2)  If VGR2 and NEP are states of the Voyager 2 spacecraft and
///      Neptune with respect to some common center at a given time
///      ET, then
///
///         VDIST ( VGR2, NEP )
///
///      yields the distance between the spacecraft and Neptune at time
///      ET.
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
/// -    SPICELIB Version 1.1.0, 25-MAY-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 08-JUL-1990 (NJB)
/// ```
pub fn vdist(v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
    let ret = VDIST(v1, v2);
    ret
}

//$Procedure VDIST ( Vector distance )
pub fn VDIST(V1: &[f64], V2: &[f64]) -> f64 {
    let V1 = DummyArray::new(V1, 1..=3);
    let V2 = DummyArray::new(V2, 1..=3);
    let mut VDIST: f64 = 0.0;
    let mut DIFF = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // No surprises.
    //
    VSUB(V1.as_slice(), V2.as_slice(), DIFF.as_slice_mut());

    VDIST = VNORM(DIFF.as_slice());

    VDIST
}
