//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Determinant of a double precision 3x3 matrix
///
/// Compute the determinant of a double precision 3x3 matrix.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M1         I   Matrix whose determinant is to be found.
///
///  The function returns the value of the determinant found by direct
///  application of the definition of the determinant.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M1       is any double precision, 3x3 matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value of the determinant found by direct
///  application of the definition of the determinant.
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
///  DET calculates the determinant of M1 in a single arithmetic
///  expression which is, effectively, the expansion of M1 about its
///  first row. Since the calculation of the determinant involves
///  the multiplication of numbers whose magnitudes are unrestricted,
///  there is the possibility of floating point overflow or underflow.
///  NO error checking or recovery is implemented in this routine.
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
///  1) Given a 3x3 double precision matrix, compute its determinant.
///
///     Example code begins here.
///
///
///           PROGRAM DET_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION      DET
///
///     C
///     C     Local variables
///     C
///           DOUBLE PRECISION      M1     ( 3, 3 )
///           DOUBLE PRECISION      M2     ( 3, 3 )
///
///     C
///     C     Set M1 and M2.
///     C
///           DATA                  M1 /  1.D0,  2.D0,  3.D0,
///          .                            4.D0,  5.D0,  6.D0,
///          .                            7.D0,  8.D0,  9.D0  /
///
///           DATA                  M2 /  1.D0,  2.D0,  3.D0,
///          .                            0.D0,  5.D0,  6.D0,
///          .                            0.D0,  0.D0,  9.D0  /
///
///     C
///     C     Display the determinant of M1 and M2.
///     C
///           WRITE(*,'(A,F6.2)') 'Determinant of M1:', DET(M1)
///           WRITE(*,'(A,F6.2)') 'Determinant of M2:', DET(M2)
///
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Determinant of M1:  0.00
///     Determinant of M2: 45.00
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No checking is implemented to determine whether M1 will cause
///      overflow or underflow in the process of calculating the
///      determinant. In most cases, this will not pose a problem.
///      The user is required to determine if M1 is suitable matrix
///      for DET to operate on.
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
/// -    SPICELIB Version 1.0.2, 02-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing fragment.
///
///         Added missing IMPLICIT NONE statement.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn det(m1: &[[f64; 3]; 3]) -> f64 {
    let ret = DET(m1.as_flattened());
    ret
}

//$Procedure DET  ( Determinant of a double precision 3x3 matrix )
pub fn DET(M1: &[f64]) -> f64 {
    let M1 = DummyArray2D::new(M1, 1..=3, 1..=3);
    let mut DET: f64 = 0.0;

    DET = (((M1[[1, 1]] * ((M1[[2, 2]] * M1[[3, 3]]) - (M1[[2, 3]] * M1[[3, 2]])))
        - (M1[[1, 2]] * ((M1[[2, 1]] * M1[[3, 3]]) - (M1[[2, 3]] * M1[[3, 1]]))))
        + (M1[[1, 3]] * ((M1[[2, 1]] * M1[[3, 2]]) - (M1[[2, 2]] * M1[[3, 1]]))));

    DET
}
