//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector distance, general dimension
///
/// Return the distance between two vectors of arbitrary dimension.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1,
///  V2         I   Two vectors of arbitrary dimension.
///  NDIM       I   The common dimension of V1 and V2
///
///  The function returns the distance between V1 and V2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1,
///  V2       are two vectors of arbitrary dimension, the
///           distance between which is desired.
///
///  NDIM     is the common dimension of V1 and V2. NDIM must be
///           non-negative and must not exceed the minimum of the
///           declared sizes of the actual arguments corresponding
///           to V1 and V2.
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
///
///  If NDIM is less than 1, the function value is set to 0.D0.
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
///  The Euclidean norm of an n-dimensional vector
///
///     (x ,  x , ... , x )
///       1    2         n
///
///  is defined as
///
///                                             1/2
///           2        2                  2
///     (   x    +   x    +  . . .  +   x     ).
///          1        2                  n
///
///  This number is the distance of the point (x, y, z) from the
///  origin. If n = 3, and A and B are two vectors whose components
///  are
///
///     ( A(1), A(2), A(3) )    and    ( B(1), B(2), B(3) ),
///
///  then the distance between A and B is the norm of the difference
///  A - B, which has components
///
///     (  A(1) - B(1),  A(2) - B(2),  A(3) - B(3)  ).
///
///  A related routine is VDIST, which computes the distance between
///  two 3-vectors.
/// ```
///
/// # Examples
///
/// ```text
///  1)  If V1 is
///
///         ( 2.0D0,  3.0D0 )
///
///      and V2 is
///
///         ( 5.0D0,  7.0D0 ),
///
///      and NDIM is 2, then
///
///         VDISTG (V1, V2, NDIM )
///
///      will be 5.D0.
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
/// -    SPICELIB Version 1.0.0, 17-JUL-1990 (NJB)
/// ```
pub fn vdistg(v1: &[f64], v2: &[f64], ndim: i32) -> f64 {
    let ret = VDISTG(v1, v2, ndim);
    ret
}

//$Procedure VDISTG ( Vector distance, general dimension )
pub fn VDISTG(V1: &[f64], V2: &[f64], NDIM: i32) -> f64 {
    let V1 = DummyArray::new(V1, 1..);
    let V2 = DummyArray::new(V2, 1..);
    let mut VDISTG: f64 = 0.0;
    let mut SCALE: f64 = 0.0;

    //
    // Local variables
    //

    //
    // We find the norm of a scaled version of the difference vector,
    // and then rescale this norm.  This method helps prevent overflow
    // due to squaring the components of the difference vector.
    //
    // The code here is almost identical to that of VNORMG.  We'd love
    // to just call VNORMG, but that would require storage for the
    // difference vector.  So we do the job ourselves.
    //

    //
    // Find the scale factor.
    //
    SCALE = 0.0;

    for I in 1..=NDIM {
        SCALE = intrinsics::DMAX1(&[SCALE, f64::abs((V1[I] - V2[I]))]);
    }

    if (SCALE == 0.0) {
        VDISTG = 0.0;
        return VDISTG;
    } else {
        VDISTG = 0.0;

        for I in 1..=NDIM {
            VDISTG = (VDISTG + f64::powi(((V1[I] - V2[I]) / SCALE), 2));
        }

        VDISTG = (SCALE * f64::sqrt(VDISTG));
    }

    VDISTG
}
