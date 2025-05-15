//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector scaling, general dimension
///
/// Multiply a scalar and a double precision vector of arbitrary
/// dimension.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  S          I   Scalar to multiply a vector.
///  V1         I   Vector to be multiplied.
///  NDIM       I   Dimension of V1 (and also VOUT).
///  VOUT       O   Product vector, S * V1.
/// ```
///
/// # Detailed Input
///
/// ```text
///  S        is a double precision scalar.
///
///  V1       is a double precision n-dimensional vector.
///
///  NDIM     is the dimension of V1 (and VOUT).
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is a double precision n-dimensional vector containing
///           the product of the scalar with the vector V1.
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
///  For each value of the index I from 1 to NDIM, this subroutine
///  performs the following multiplication
///
///     VOUT(I) = S * V1(I)
///
///  No error checking is performed to guard against numeric overflow
///  or underflow.
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
///  1) Define a sets of scalar double precision values and use them
///     to scale a given n-dimensional vector.
///
///
///     Example code begins here.
///
///
///           PROGRAM VSCLG_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NDIM
///           PARAMETER           ( NDIM   = 4 )
///
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 3 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      S    ( SETSIZ )
///           DOUBLE PRECISION      V1   ( NDIM   )
///           DOUBLE PRECISION      VOUT ( NDIM   )
///
///           INTEGER               I
///
///     C
///     C     Define the set of scalars and the input vector.
///     C
///           DATA                  S    / 3.D0, 0.D0, -1.D0 /
///
///           DATA                  V1   / 1.D0, 2.D0, -3.D0, 4.D0  /
///
///
///           WRITE(*,'(A,4F6.1)') 'Input vector : ', V1
///           WRITE(*,*)
///
///     C
///     C     Calculate product of each scalar and V1.
///     C
///           DO I=1, SETSIZ
///
///              CALL VSCLG ( S(I), V1, NDIM, VOUT )
///
///              WRITE(*,'(A,F6.1)')  'Scale factor : ', S(I)
///              WRITE(*,'(A,4F6.1)') 'Output vector: ', VOUT
///              WRITE(*,*)
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
///     Input vector :    1.0   2.0  -3.0   4.0
///
///     Scale factor :    3.0
///     Output vector:    3.0   6.0  -9.0  12.0
///
///     Scale factor :    0.0
///     Output vector:    0.0   0.0  -0.0   0.0
///
///     Scale factor :   -1.0
///     Output vector:   -1.0  -2.0   3.0  -4.0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No error checking is performed to guard against numeric
///      overflow. The programmer is thus required to insure that the
///      values in V1 and S are reasonable and will not cause overflow.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
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
///         code example based on existing example.
///
/// -    SPICELIB Version 1.0.2, 22-APR-2010 (NJB)
///
///         Header correction: assertions that the output
///         can overwrite the input have been removed.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn vsclg(s: f64, v1: &[f64], ndim: i32, vout: &mut [f64]) {
    VSCLG(s, v1, ndim, vout);
}

//$Procedure VSCLG ( Vector scaling, general dimension )
pub fn VSCLG(S: f64, V1: &[f64], NDIM: i32, VOUT: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=NDIM);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=NDIM);

    //
    // Local variables
    //

    for I in 1..=NDIM {
        VOUT[I] = (S * V1[I]);
    }
}
