//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector projection, 3 dimensions
///
/// Compute the projection of one 3-dimensional vector onto another
/// 3-dimensional vector.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A          I   The vector to be projected.
///  B          I   The vector onto which A is to be projected.
///  P          O   The projection of A onto B.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A        is a double precision, 3-dimensional vector. This
///           vector is to be projected onto the vector B.
///
///  B        is a double precision, 3-dimensional vector. This
///           vector is the vector which receives the projection.
/// ```
///
/// # Detailed Output
///
/// ```text
///  P        is a double precision, 3-dimensional vector containing
///           the projection of A onto B. (P is necessarily parallel
///           to B.) If B is the zero vector then P will be returned
///           as the zero vector.
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
///  Given any vectors A and B, there is a unique decomposition of
///  A as a sum V + P such that V, the dot product of V and B, is zero,
///  and the dot product of P with B is equal the product of the
///  lengths of P and B. P is called the projection of A onto B. It
///  can be expressed mathematically as
///
///     DOT(A,B)
///     -------- * B
///     DOT(B,B)
///
///  (This is not necessarily the prescription used to compute the
///  projection. It is intended only for descriptive purposes.)
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Define two sets of vectors and compute the projection of
///     each vector of the first set on the corresponding vector of
///     the second set.
///
///     Example code begins here.
///
///
///           PROGRAM VPROJ_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NDIM
///           PARAMETER           ( NDIM   = 3 )
///
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 4 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      SETA ( NDIM, SETSIZ )
///           DOUBLE PRECISION      SETB ( NDIM, SETSIZ )
///           DOUBLE PRECISION      PVEC ( NDIM )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the two vector sets.
///     C
///           DATA                  SETA / 6.D0,  6.D0,  6.D0,
///          .                             6.D0,  6.D0,  6.D0,
///          .                             6.D0,  6.D0,  0.D0,
///          .                             6.D0,  0.D0,  0.D0  /
///
///           DATA                  SETB / 2.D0,  0.D0,  0.D0,
///          .                            -3.D0,  0.D0,  0.D0,
///          .                             0.D0,  7.D0,  0.D0,
///          .                             0.D0,  0.D0,  9.D0  /
///
///     C
///     C     Calculate the projection
///     C
///           DO I=1, SETSIZ
///
///              CALL VPROJ ( SETA(1,I), SETB(1,I), PVEC )
///
///              WRITE(*,'(A,3F5.1)') 'Vector A  : ',
///          .                        ( SETA(J,I), J=1,3 )
///              WRITE(*,'(A,3F5.1)') 'Vector B  : ',
///          .                        ( SETB(J,I), J=1,3 )
///              WRITE(*,'(A,3F5.1)') 'Projection: ', PVEC
///              WRITE(*,*) ' '
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Vector A  :   6.0  6.0  6.0
///     Vector B  :   2.0  0.0  0.0
///     Projection:   6.0  0.0  0.0
///
///     Vector A  :   6.0  6.0  6.0
///     Vector B  :  -3.0  0.0  0.0
///     Projection:   6.0 -0.0 -0.0
///
///     Vector A  :   6.0  6.0  0.0
///     Vector B  :   0.0  7.0  0.0
///     Projection:   0.0  6.0  0.0
///
///     Vector A  :   6.0  0.0  0.0
///     Vector B  :   0.0  0.0  9.0
///     Projection:   0.0  0.0  0.0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  An implicit assumption exists that A and B are specified in
///      the same reference frame. If this is not the case, the
///      numerical result has no meaning.
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
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example. Added entry in $Restrictions section.
///
/// -    SPICELIB Version 1.0.2, 23-APR-2010 (NJB)
///
///         Header correction: assertions that the output
///         can overwrite the input have been removed.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 4-JAN-1989 (WLT)
///
///         Upgrade the routine to work with negative axis indexes. Also
///         take care of the funky way the indices (other than the input)
///         were obtained via the MOD function. It works but isn't as
///         clear (or fast) as just reading the axes from data.
/// ```
pub fn vproj(a: &[f64; 3], b: &[f64; 3], p: &mut [f64; 3]) {
    VPROJ(a, b, p);
}

//$Procedure VPROJ ( Vector projection, 3 dimensions )
pub fn VPROJ(A: &[f64], B: &[f64], P: &mut [f64]) {
    let A = DummyArray::new(A, 1..=3);
    let B = DummyArray::new(B, 1..=3);
    let mut P = DummyArrayMut::new(P, 1..=3);
    let mut BIGA: f64 = 0.0;
    let mut BIGB: f64 = 0.0;
    let mut R = StackArray::<f64, 3>::new(1..=3);
    let mut T = StackArray::<f64, 3>::new(1..=3);
    let mut SCALE: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    BIGA = intrinsics::DMAX1(&[f64::abs(A[1]), f64::abs(A[2]), f64::abs(A[3])]);
    BIGB = intrinsics::DMAX1(&[f64::abs(B[1]), f64::abs(B[2]), f64::abs(B[3])]);

    if (BIGA == 0 as f64) {
        P[1] = 0.0;
        P[2] = 0.0;
        P[3] = 0.0;
        return;
    }

    if (BIGB == 0 as f64) {
        P[1] = 0.0;
        P[2] = 0.0;
        P[3] = 0.0;
        return;
    }

    R[1] = (B[1] / BIGB);
    R[2] = (B[2] / BIGB);
    R[3] = (B[3] / BIGB);

    T[1] = (A[1] / BIGA);
    T[2] = (A[2] / BIGA);
    T[3] = (A[3] / BIGA);

    SCALE = ((VDOT(T.as_slice(), R.as_slice()) * BIGA) / VDOT(R.as_slice(), R.as_slice()));

    VSCL(SCALE, R.as_slice(), P.as_slice_mut());
}
