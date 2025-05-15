//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector dot product, 3 dimensions
///
/// Compute the dot product of two double precision, 3-dimensional
/// vectors.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   First vector in the dot product.
///  V2         I   Second vector in the dot product.
///
///  The function returns the value of the dot product of V1 and V2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1,
///  V2       are two arbitrary double precision 3-dimensional
///           vectors.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value of the dot product (inner product)
///  of V1 and V2:
///
///     < V1, V2 >
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
///  VDOT calculates the dot product of V1 and V2 by a simple
///  application of the definition:
///
///                3
///             .-----
///              \
///     VDOT  =   )  V1(I) * V2(I)
///              /
///             '-----
///               I=1
///
///  No error checking is performed to prevent or recover from numeric
///  overflow.
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
///  1) Suppose that you have a set of double precision 3-dimensional
///     vectors. Check if they are orthogonal to the Z-axis.
///
///
///     Example code begins here.
///
///
///           PROGRAM VDOT_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION      VDOT
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               SETSIZ
///           PARAMETER           ( SETSIZ = 4 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      V1   ( 3, SETSIZ )
///           DOUBLE PRECISION      Z    ( 3         )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the vector set.
///     C
///           DATA                  V1  / 1.D0,  0.D0,  0.D0,
///          .                            0.D0, -6.D0,  0.D0,
///          .                           10.D0,  0.D0, -1.D0,
///          .                            0.D0,  0.D0,  1.D0  /
///
///           DATA                  Z   / 0.D0,  0.D0,  1.D0  /
///
///     C
///     C     Check the orthogonality with respect to Z of each
///     C     vector in V1.
///     C
///           DO I = 1, SETSIZ
///
///              WRITE(*,*)
///              WRITE(*,'(A,3F6.1)') 'Input vector (V1): ',
///          .                         ( V1(J,I), J=1,3 )
///
///              IF ( VDOT( V1(1,I), Z ) .EQ. 0.D0 ) THEN
///
///                 WRITE(*,'(A)') 'V1 and Z are orthogonal.'
///
///              ELSE
///
///                 WRITE(*,'(A)') 'V1 and Z are NOT orthogonal.'
///
///              END IF
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
///     Input vector (V1):    1.0   0.0   0.0
///     V1 and Z are orthogonal.
///
///     Input vector (V1):    0.0  -6.0   0.0
///     V1 and Z are orthogonal.
///
///     Input vector (V1):   10.0   0.0  -1.0
///     V1 and Z are NOT orthogonal.
///
///     Input vector (V1):    0.0   0.0   1.0
///     V1 and Z are NOT orthogonal.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The user is responsible for determining that the vectors V1
///      and V2 are not so large as to cause numeric overflow. In
///      most cases this will not present a problem.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 28-MAY-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example. Improved $Particulars section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn vdot(v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
    let ret = VDOT(v1, v2);
    ret
}

//$Procedure VDOT ( Vector dot product, 3 dimensions )
pub fn VDOT(V1: &[f64], V2: &[f64]) -> f64 {
    let V1 = DummyArray::new(V1, 1..=3);
    let V2 = DummyArray::new(V2, 1..=3);
    let mut VDOT: f64 = 0.0;

    VDOT = (((V1[1] * V2[1]) + (V1[2] * V2[2])) + (V1[3] * V2[3]));
    //
    VDOT
}
