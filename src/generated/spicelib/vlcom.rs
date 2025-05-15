//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector linear combination, 3 dimensions
///
/// Compute a vector linear combination of two double precision,
/// 3-dimensional vectors.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A          I   Coefficient of V1.
///  V1         I   Vector in 3-space.
///  B          I   Coefficient of V2.
///  V2         I   Vector in 3-space.
///  SUM        O   Linear vector combination A*V1 + B*V2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  A        is the double precision scalar variable that multiplies
///           V1.
///
///  V1       is an arbitrary, double precision 3-dimensional vector.
///
///  B        is the double precision scalar variable that multiplies
///           V2.
///
///  V2       is an arbitrary, double precision 3-dimensional vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SUM      is the double precision 3-dimensional vector which
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
///     For each value of the index I, from 1 to 3:
///
///        SUM(I) = A * V1(I) + B * V2(I)
///
///  No error checking is performed to guard against numeric overflow.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Suppose you want to generate a sequence of points representing
///     an elliptical footprint, from the known semi-major
///     and semi-minor axes.
///
///
///     Example code begins here.
///
///
///           PROGRAM VLCOM_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION      TWOPI
///
///     C
///     C     Local parameters.
///     C
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      THETA
///           DOUBLE PRECISION      SMAJOR ( 3 )
///           DOUBLE PRECISION      SMINOR ( 3 )
///           DOUBLE PRECISION      VECTOR ( 3 )
///
///           INTEGER               I
///
///     C
///     C     Let SMAJOR and SMINOR be the two known semi-major and
///     C     semi-minor axes of our elliptical footprint.
///     C
///           DATA                  SMAJOR /
///          .                    0.070115D0, 0.D0,        0.D0 /
///
///           DATA                  SMINOR /
///          .                    0.D0,       0.035014D0,  0.D0 /
///
///
///     C
///     C     Compute the vectors of interest and display them
///     C
///           THETA = 0.D0
///           STEP  = TWOPI() / 16
///
///           DO I = 1, 16
///
///              CALL VLCOM (  COS(THETA), SMAJOR,
///          .                 SIN(THETA), SMINOR, VECTOR )
///
///              WRITE(*,'(I2,A,3F10.6)') I, ':', VECTOR
///
///              THETA = THETA + STEP
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
///      1:  0.070115  0.000000  0.000000
///      2:  0.064778  0.013399  0.000000
///      3:  0.049579  0.024759  0.000000
///      4:  0.026832  0.032349  0.000000
///      5:  0.000000  0.035014  0.000000
///      6: -0.026832  0.032349  0.000000
///      7: -0.049579  0.024759  0.000000
///      8: -0.064778  0.013399  0.000000
///      9: -0.070115  0.000000  0.000000
///     10: -0.064778 -0.013399 -0.000000
///     11: -0.049579 -0.024759 -0.000000
///     12: -0.026832 -0.032349 -0.000000
///     13: -0.000000 -0.035014 -0.000000
///     14:  0.026832 -0.032349  0.000000
///     15:  0.049579 -0.024759  0.000000
///     16:  0.064778 -0.013399  0.000000
///
///
///  2) As a second example, suppose that U and V are orthonormal
///     vectors that form a basis of a plane. Moreover suppose that we
///     wish to project a vector X onto this plane.
///
///
///     Example code begins here.
///
///
///           PROGRAM VLCOM_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION      VDOT
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      PUV    ( 3 )
///           DOUBLE PRECISION      X      ( 3 )
///           DOUBLE PRECISION      U      ( 3 )
///           DOUBLE PRECISION      V      ( 3 )
///
///     C
///     C     Let X be an arbitrary 3-vector
///     C
///           DATA                  X  /  4.D0, 35.D0, -5.D0  /
///
///     C
///     C     Let U and V be orthonormal 3-vectors spanning the
///     C     plane of interest.
///     C
///           DATA                  U  /  0.D0,  0.D0,  1.D0 /
///
///           V(1) =  SQRT(2.D0)/2.D0
///           V(2) = -SQRT(2.D0)/2.D0
///           V(3) =  0.D0
///
///     C
///     C     Compute the projection of X onto this 2-dimensional
///     C     plane in 3-space.
///     C
///           CALL VLCOM ( VDOT ( X, U ), U, VDOT ( X, V ), V, PUV )
///
///     C
///     C     Display the results.
///     C
///           WRITE(*,'(A,3F6.1)') 'Input vector             : ', X
///           WRITE(*,'(A,3F6.1)') 'Projection into 2-d plane: ', PUV
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Input vector             :    4.0  35.0  -5.0
///     Projection into 2-d plane:  -15.5  15.5  -5.0
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
///         Added complete code example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn vlcom(a: f64, v1: &[f64; 3], b: f64, v2: &[f64; 3], sum: &mut [f64; 3]) {
    VLCOM(a, v1, b, v2, sum);
}

//$Procedure VLCOM ( Vector linear combination, 3 dimensions )
pub fn VLCOM(A: f64, V1: &[f64], B: f64, V2: &[f64], SUM: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=3);
    let V2 = DummyArray::new(V2, 1..=3);
    let mut SUM = DummyArrayMut::new(SUM, 1..=3);

    SUM[1] = ((A * V1[1]) + (B * V2[1]));
    SUM[2] = ((A * V1[2]) + (B * V2[2]));
    SUM[3] = ((A * V1[3]) + (B * V2[3]));
}
