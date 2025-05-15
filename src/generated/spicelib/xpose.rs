//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Transpose a matrix, 3x3
///
/// Transpose a 3x3 matrix.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M1         I   3x3 matrix to be transposed.
///  MOUT       O   Transpose of M1.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M1       is any double precision 3x3 matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MOUT     is a double precision, 3x3 matrix which contains the
///           transpose of M1.
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
///  XPOSE first copies the diagonal elements of M1 to MOUT. Then
///  the off-diagonal elements are transposed using a temporary
///  variable in the following order:
///
///     (1,2) <---> (2,1)
///     (1,3) <---> (3,1)
///     (2,3) <---> (3,2)
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
///  1) Given a 3x3 double precision matrix, find its transpose.
///
///
///     Example code begins here.
///
///
///           PROGRAM XPOSE_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      M1     (3,3)
///           DOUBLE PRECISION      MOUT   (3,3)
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define the input matrix.
///     C
///           DATA                  M1     /  1.0D0,  0.0D0,  0.0D0,
///          .                                2.0D0,  4.0D0,  6.0D0,
///          .                                3.0D0,  5.0D0,  0.0D0 /
///
///     C
///     C     Compute the transpose of M1.
///     C
///           CALL XPOSE ( M1, MOUT )
///
///     C
///     C     Display the results.
///     C
///           WRITE(*,*) 'Input matrix (M1):'
///           WRITE(*,*)
///           DO I= 1, 3
///              WRITE(*,'(3F6.1)') ( M1(I,J), J= 1, 3 )
///           END DO
///           WRITE(*,*)
///           WRITE(*,*) 'Transpose of M1:'
///           WRITE(*,*)
///           DO I= 1, 3
///              WRITE(*,'(3F6.1)') ( MOUT(I,J), J= 1, 3 )
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Input matrix (M1):
///
///        1.0   2.0   3.0
///        0.0   4.0   5.0
///        0.0   6.0   0.0
///
///      Transpose of M1:
///
///        1.0   0.0   0.0
///        2.0   4.0   6.0
///        3.0   5.0   0.0
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
///         code example.
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
pub fn xpose(m1: &[[f64; 3]; 3], mout: &mut [[f64; 3]; 3]) {
    XPOSE(m1.as_flattened(), mout.as_flattened_mut());
}

//$Procedure XPOSE ( Transpose a matrix, 3x3 )
pub fn XPOSE(M1: &[f64], MOUT: &mut [f64]) {
    let M1 = DummyArray2D::new(M1, 1..=3, 1..=3);
    let mut MOUT = DummyArrayMut2D::new(MOUT, 1..=3, 1..=3);
    let mut TEMP: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Move the three diagonal elements from M1 to MOUT
    //
    MOUT[[1, 1]] = M1[[1, 1]];
    MOUT[[2, 2]] = M1[[2, 2]];
    MOUT[[3, 3]] = M1[[3, 3]];

    //
    // Switch the three pairs of off-diagonal elements
    //
    TEMP = M1[[1, 2]];
    MOUT[[1, 2]] = M1[[2, 1]];
    MOUT[[2, 1]] = TEMP;

    TEMP = M1[[1, 3]];
    MOUT[[1, 3]] = M1[[3, 1]];
    MOUT[[3, 1]] = TEMP;

    TEMP = M1[[2, 3]];
    MOUT[[2, 3]] = M1[[3, 2]];
    MOUT[[3, 2]] = TEMP;
}
