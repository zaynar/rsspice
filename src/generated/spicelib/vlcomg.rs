//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector linear combination, general dimension
///
/// Compute a vector linear combination of two double precision
/// vectors of arbitrary dimension.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  N          I   Dimension of vector space.
///  A          I   Coefficient of V1.
///  V1         I   Vector in N-space.
///  B          I   Coefficient of V2.
///  V2         I   Vector in N-space.
///  SUM        O   Linear vector combination A*V1 + B*V2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  N        is the dimension of V1, V2 and SUM.
///
///  A        is the double precision scalar variable that multiplies
///           V1.
///
///  V1       is an arbitrary, double precision n-dimensional vector.
///
///  B        is the double precision scalar variable that multiplies
///           V2.
///
///  V2       is an arbitrary, double precision n-dimensional vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SUM      is the double precision n-dimensional vector which
///           contains the linear combination
///
///              A * V1 + B * V2
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
///  The code reflects precisely the following mathematical expression
///
///     For each value of the index I, from 1 to N:
///
///        SUM(I) = A * V1(I) + B * V2(I)
///
///  No error checking is performed to guard against numeric overflow.
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
///  1) Perform the projection of a 4-dimensional vector into a
///     2-dimensional plane in 4-space.
///
///
///     Example code begins here.
///
///
///           PROGRAM VLCOMG_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION      VDOTG
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NDIM
///           PARAMETER           ( NDIM = 4 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      PUV    ( NDIM )
///           DOUBLE PRECISION      X      ( NDIM )
///           DOUBLE PRECISION      U      ( NDIM )
///           DOUBLE PRECISION      V      ( NDIM )
///
///     C
///     C     Let X be an arbitrary NDIM-vector
///     C
///           DATA                  X  /  4.D0, 35.D0, -5.D0, 7.D0  /
///
///     C
///     C     Let U and V be orthonormal NDIM-vectors spanning the
///     C     plane of interest.
///     C
///           DATA                  U  /  0.D0,  0.D0,  1.D0, 0.D0 /
///
///           V(1) =  SQRT(3.D0)/3.D0
///           V(2) = -SQRT(3.D0)/3.D0
///           V(3) =  0.D0
///           V(4) =  SQRT(3.D0)/3.D0
///
///     C
///     C     Compute the projection of X onto this 2-dimensional
///     C     plane in NDIM-space.
///     C
///           CALL VLCOMG ( NDIM, VDOTG ( X, U, NDIM), U,
///          .                    VDOTG ( X, V, NDIM), V, PUV )
///
///     C
///     C     Display the results.
///     C
///           WRITE(*,'(A,4F6.1)') 'Input vector             : ', X
///           WRITE(*,'(A,4F6.1)') 'Projection into 2-d plane: ', PUV
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Input vector             :    4.0  35.0  -5.0   7.0
///     Projection into 2-d plane:   -8.0   8.0  -5.0  -8.0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No error checking is performed to guard against numeric
///      overflow or underflow. The user is responsible for insuring
///      that the input values are reasonable.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Added complete code example based on existing example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn vlcomg(n: i32, a: f64, v1: &[f64], b: f64, v2: &[f64], sum: &mut [f64]) {
    VLCOMG(n, a, v1, b, v2, sum);
}

//$Procedure VLCOMG ( Vector linear combination, general dimension )
pub fn VLCOMG(N: i32, A: f64, V1: &[f64], B: f64, V2: &[f64], SUM: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=N);
    let V2 = DummyArray::new(V2, 1..=N);
    let mut SUM = DummyArrayMut::new(SUM, 1..=N);

    //
    // Local variables
    //

    for I in 1..=N {
        SUM[I] = ((A * V1[I]) + (B * V2[I]));
    }
}
