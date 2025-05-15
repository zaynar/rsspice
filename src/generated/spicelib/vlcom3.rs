//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector linear combination, 3 dimensions
///
/// Compute the vector linear combination of three double precision
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
///  C          I   Coefficient of V3.
///  V3         I   Vector in 3-space.
///  SUM        O   Linear vector combination A*V1 + B*V2 + C*V3.
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
///
///  C        is the double precision scalar variable that multiplies
///           V3.
///
///  V3       is a double precision 3-dimensional vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SUM      is the double precision 3-dimensional vector which
///           contains the linear combination
///
///              A * V1 + B * V2 + C * V3
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
///        SUM(I) = A * V1(I) + B * V2(I) + C * V3(I)
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
///  1) Suppose you have an instrument with an elliptical field
///     of view described by its angular extent along the semi-minor
///     and semi-major axes.
///
///     The following code example demonstrates how to create
///     16 vectors aiming at visualizing the field-of-view in
///     three dimensional space.
///
///
///     Example code begins here.
///
///
///           PROGRAM VLCOM3_EX1
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
///     C     Define the two angular extends, along the semi-major
///     C     (U) and semi-minor (V) axes of the elliptical field
///     C     of view, in radians.
///     C
///           DOUBLE PRECISION      MAXANG
///           PARAMETER           ( MAXANG = 0.07D0 )
///
///           DOUBLE PRECISION      MINANG
///           PARAMETER           ( MINANG = 0.035D0 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      A
///           DOUBLE PRECISION      B
///           DOUBLE PRECISION      STEP
///           DOUBLE PRECISION      THETA
///           DOUBLE PRECISION      U      ( 3 )
///           DOUBLE PRECISION      V      ( 3 )
///           DOUBLE PRECISION      VECTOR ( 3 )
///           DOUBLE PRECISION      Z      ( 3 )
///
///           INTEGER               I
///
///     C
///     C     Let U and V be orthonormal 3-vectors spanning the
///     C     focal plane of the instrument, and Z its
///     C     boresight.
///     C
///           DATA                  U  /  1.D0,  0.D0,  0.D0 /
///           DATA                  V  /  0.D0,  1.D0,  0.D0 /
///           DATA                  Z  /  0.D0,  0.D0,  1.D0 /
///
///     C
///     C     Find the length of the ellipse's axes. Note that
///     C     we are dealing with unitary vectors.
///     C
///           A = TAN ( MAXANG )
///           B = TAN ( MINANG )
///
///     C
///     C     Compute the vectors of interest and display them
///     C
///           THETA = 0.D0
///           STEP  = TWOPI() / 16
///
///           DO I = 1, 16
///
///              CALL VLCOM3 ( 1.D0,           Z, A * COS(THETA), U,
///          .                 B * SIN(THETA), V, VECTOR            )
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
///      1:  0.070115  0.000000  1.000000
///      2:  0.064777  0.013399  1.000000
///      3:  0.049578  0.024759  1.000000
///      4:  0.026832  0.032349  1.000000
///      5:  0.000000  0.035014  1.000000
///      6: -0.026832  0.032349  1.000000
///      7: -0.049578  0.024759  1.000000
///      8: -0.064777  0.013399  1.000000
///      9: -0.070115  0.000000  1.000000
///     10: -0.064777 -0.013399  1.000000
///     11: -0.049578 -0.024759  1.000000
///     12: -0.026832 -0.032349  1.000000
///     13: -0.000000 -0.035014  1.000000
///     14:  0.026832 -0.032349  1.000000
///     15:  0.049578 -0.024759  1.000000
///     16:  0.064777 -0.013399  1.000000
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
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Added restriction #1.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 01-NOV-1990 (WLT)
/// ```
pub fn vlcom3(
    a: f64,
    v1: &[f64; 3],
    b: f64,
    v2: &[f64; 3],
    c: f64,
    v3: &[f64; 3],
    sum: &mut [f64; 3],
) {
    VLCOM3(a, v1, b, v2, c, v3, sum);
}

//$Procedure VLCOM3 ( Vector linear combination, 3 dimensions )
pub fn VLCOM3(A: f64, V1: &[f64], B: f64, V2: &[f64], C: f64, V3: &[f64], SUM: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=3);
    let V2 = DummyArray::new(V2, 1..=3);
    let V3 = DummyArray::new(V3, 1..=3);
    let mut SUM = DummyArrayMut::new(SUM, 1..=3);

    SUM[1] = (((A * V1[1]) + (B * V2[1])) + (C * V3[1]));
    SUM[2] = (((A * V1[2]) + (B * V2[2])) + (C * V3[2]));
    SUM[3] = (((A * V1[3]) + (B * V2[3])) + (C * V3[3]));
}
