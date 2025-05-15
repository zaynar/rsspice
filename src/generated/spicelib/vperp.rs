//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Perpendicular component of a 3-vector
///
/// Find the component of a vector that is perpendicular to a second
/// vector. All vectors are 3-dimensional.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A          I   The vector whose orthogonal component is sought.
///  B          I   The vector used as the orthogonal reference.
///  P          O   The component of A orthogonal to B.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A        is a double precision, 3-dimensional vector. It the
///           vector whose component orthogonal to B is sought. (There
///           is a unique decomposition of A into a sum V + P, where V
///           is parallel to B and P is orthogonal to B. We want the
///           component P.)
///
///  B        is a double precision, 3-dimensional vector. This
///           vector is the vector used as a reference for the
///           decomposition of A.
/// ```
///
/// # Detailed Output
///
/// ```text
///  P        is a double precision, 3-dimensional vector containing
///           the component of A that is orthogonal to B.
///           P may overwrite either A or B.
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
///  Given and non-zero vector B and a vector A, there is a unique
///  decomposition of A as a sum V + P such that P is orthogonal
///  to B and V is parallel to B. This routine finds the vector P.
///
///  If B is a zero vector, P will be identical to A.
/// ```
///
/// # Examples
///
/// ```text
///  The following table gives sample inputs and results from calling
///  VPERP.
///
///     A                  B                 P
///     ------------------------------------------
///     (6, 6, 6)      ( 2, 0, 0)        (0, 6, 6)
///     (6, 6, 6)      (-3, 0, 0)        (0, 6, 6)
///     (6, 6, 0)      ( 0, 7, 0)        (6, 0, 0)
///     (6, 0, 0)      ( 0, 0, 9)        (6, 0, 0)
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
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 16-SEP-2020 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.1, 11-MAY-2010 (EDW)
///
///         Minor edit to code comments eliminating typo.
///
///         Reordered header sections to proper NAIF convention.
///         Removed Revision section, it listed a duplication of a
///         $Version section entry.
///
/// -    SPICELIB Version 1.1.0, 09-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in VSCL call.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn vperp(a: &[f64; 3], b: &[f64; 3], p: &mut [f64; 3]) {
    VPERP(a, b, p);
}

//$Procedure VPERP ( Perpendicular component of a 3-vector )
pub fn VPERP(A: &[f64], B: &[f64], P: &mut [f64]) {
    let A = DummyArray::new(A, 1..=3);
    let B = DummyArray::new(B, 1..=3);
    let mut P = DummyArrayMut::new(P, 1..=3);
    let mut BIGA: f64 = 0.0;
    let mut BIGB: f64 = 0.0;
    let mut R = StackArray::<f64, 3>::new(1..=3);
    let mut T = StackArray::<f64, 3>::new(1..=3);
    let mut V = StackArray::<f64, 3>::new(1..=3);

    //
    // Local variables
    //

    //
    // Error free routine:  no check-in.
    //
    BIGA = intrinsics::DMAX1(&[f64::abs(A[1]), f64::abs(A[2]), f64::abs(A[3])]);
    BIGB = intrinsics::DMAX1(&[f64::abs(B[1]), f64::abs(B[2]), f64::abs(B[3])]);

    //
    // If A is the zero vector, just set P to zero and return.
    //
    if (BIGA == 0.0) {
        P[1] = 0.0;
        P[2] = 0.0;
        P[3] = 0.0;
        return;
    }

    //
    // If B is the zero vector, then set P equal to A.
    //
    if (BIGB == 0.0) {
        P[1] = A[1];
        P[2] = A[2];
        P[3] = A[3];
        return;
    }

    T[1] = (A[1] / BIGA);
    T[2] = (A[2] / BIGA);
    T[3] = (A[3] / BIGA);

    R[1] = (B[1] / BIGB);
    R[2] = (B[2] / BIGB);
    R[3] = (B[3] / BIGB);

    VPROJ(T.as_slice(), R.as_slice(), V.as_slice_mut());
    VSUB(T.as_slice(), V.as_slice(), P.as_slice_mut());
    VSCLIP(BIGA, P.as_slice_mut());
}
