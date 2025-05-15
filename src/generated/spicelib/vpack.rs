//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Pack three scalar components into a vector
///
/// Pack three scalar components into a vector.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X,
///  Y,
///  Z          I   Scalar components of a vector.
///  V          O   Equivalent vector.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X,
///  Y,
///  Z        are the scalar components of a 3-dimensional vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  V        is the equivalent vector, such that
///
///              V(1) = X
///              V(2) = Y
///              V(3) = Z
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
///  Basically, this is just shorthand notation for the common
///  sequence
///
///     V(1) = X
///     V(2) = Y
///     V(3) = Z
///
///  The routine is useful largely for two reasons. First, it
///  reduces the chance that the programmer will make a "cut and
///  paste" mistake, like
///
///     V(1) = X
///     V(1) = Y
///     V(1) = Z
///
///  Second, it makes conversions between equivalent units simpler,
///  and clearer. For instance, the sequence
///
///     V(1) = X * RPD
///     V(2) = Y * RPD
///     V(3) = Z * RPD
///
///  can be replaced by the (nearly) equivalent sequence
///
///     CALL VPACK  ( X,   Y, Z, V )
///     CALL VSCLIP ( RPD, V       )
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input
///  (if any), the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Compute an upward normal of an equilateral triangle lying
///     in the X-Y plane and centered at the origin.
///
///
///     Example code begins here.
///
///
///           PROGRAM VPACK_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      NORMAL ( 3 )
///           DOUBLE PRECISION      S
///           DOUBLE PRECISION      V1     ( 3 )
///           DOUBLE PRECISION      V2     ( 3 )
///           DOUBLE PRECISION      V3     ( 3 )
///
///
///           S = SQRT(3.D0)/2
///
///     C
///     C     Define the three corners of the triangle.
///     C
///           CALL VPACK (    S,  -0.5D0,  0.D0, V1 )
///           CALL VPACK ( 0.D0,    1.D0,  0.D0, V2 )
///           CALL VPACK (   -S,  -0.5D0,  0.D0, V3 )
///
///     C
///     C     Compute an upward normal of the triangle.
///     C
///           CALL PLTNRM ( V1, V2, V3, NORMAL )
///
///           WRITE (*, '(A,3F17.13)' ) 'NORMAL = ', NORMAL
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     NORMAL =   0.0000000000000  0.0000000000000  2.5980762113533
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 16-JUL-2020 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn vpack(x: f64, y: f64, z: f64, v: &mut [f64; 3]) {
    VPACK(x, y, z, v);
}

//$Procedure VPACK ( Pack three scalar components into a vector )
pub fn VPACK(X: f64, Y: f64, Z: f64, V: &mut [f64]) {
    let mut V = DummyArrayMut::new(V, 1..=3);

    //
    // Just shorthand, like it says above.
    //
    V[1] = X;
    V[2] = Y;
    V[3] = Z;
}
