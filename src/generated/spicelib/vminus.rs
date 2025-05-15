//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Negate vector, "-V", 3 dimensions
///
/// Negate a double precision 3-dimensional vector.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   Vector to be negated.
///  VOUT       O   Negated vector -V1.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1       is any double precision 3-dimensional vector.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VOUT     is the negation (additive inverse) of V1. It is a
///           double precision 3-dimensional vector.
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
///  For each value of the index I from 1 to 3, VMINUS negates V1
///  by the expression:
///
///     VOUT(I) = - V1(I)
///
///  No error checking is performed since overflow can occur ONLY if
///  the dynamic range of positive floating point numbers is not the
///  same size as the dynamic range of negative floating point numbers
///  AND at least one component of V1 falls outside the common range.
///  The likelihood of this occurring is so small as to be of no
///  concern.
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
///  1) Define a set of 3-dimensional vectors and negate each of them.
///
///
///     Example code begins here.
///
///
///           PROGRAM VMINUS_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 2 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      V1   ( 3, SETSIZ )
///           DOUBLE PRECISION      VOUT ( 3         )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define a set of 3-dimensional vectors.
///     C
///           DATA                  V1  /  1.D0, -2.D0, 0.D0,
///          .                             0.D0,  0.D0, 0.D0  /
///
///     C
///     C     Negate each vector
///     C
///           DO I=1, SETSIZ
///
///              CALL VMINUS ( V1(1,I), VOUT )
///
///              WRITE(*,'(A,3F6.1)') 'Input vector  : ',
///          .                        ( V1(J,I), J=1,3 )
///              WRITE(*,'(A,3F6.1)') 'Negated vector: ', VOUT
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
///     Input vector  :    1.0  -2.0   0.0
///     Negated vector:   -1.0   2.0  -0.0
///
///     Input vector  :    0.0   0.0   0.0
///     Negated vector:   -0.0  -0.0  -0.0
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
/// -    SPICELIB Version 1.0.3, 02-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example.
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn vminus(v1: &[f64; 3], vout: &mut [f64; 3]) {
    VMINUS(v1, vout);
}

//$Procedure VMINUS ( Negate vector, "-V", 3 dimensions )
pub fn VMINUS(V1: &[f64], VOUT: &mut [f64]) {
    let V1 = DummyArray::new(V1, 1..=3);
    let mut VOUT = DummyArrayMut::new(VOUT, 1..=3);

    VOUT[1] = -V1[1];
    VOUT[2] = -V1[2];
    VOUT[3] = -V1[3];
    //
}
