//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Trace of a 3x3 matrix
///
/// Return the trace of a 3x3 matrix.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MATRIX     I   3x3 matrix of double precision numbers.
///
///  The function returns the trace of MATRIX.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MATRIX   is a double precision 3x3 matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the trace of MATRIX, i.e. it is the sum of
///  the diagonal elements of MATRIX.
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
///  The code reflects precisely the following mathematical
///  expression:
///
///               3
///             .----
///              \
///     TRACE =   )  MATRIX(I,I)
///              /
///             '----
///              I=1
///
///  No error detection or correction is implemented within this
///  function.
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
///  1) Given a 3x3 double precision matrix, compute its trace.
///
///
///     Example code begins here.
///
///
///           PROGRAM TRACE_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION      TRACE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      MATRIX ( 3, 3 )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define MATRIX.
///     C
///           DATA                  MATRIX  /  3.D0,  0.D0,  4.D0,
///          .                                 5.D0, -2.D0,  0.D0,
///          .                                 7.D0,  8.D0, -1.D0  /
///
///
///           WRITE(*,'(A)') 'MATRIX:'
///           DO I=1, 3
///
///              WRITE(*,'(3F6.1)') ( MATRIX(I,J), J=1,3 )
///
///           END DO
///
///     C
///     C     Compute the trace of MATRIX and display the result.
///     C
///           WRITE(*,*)
///           WRITE(*,'(A,F4.1)') 'Trace: ', TRACE ( MATRIX )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     MATRIX:
///        3.0   5.0   7.0
///        0.0  -2.0   8.0
///        4.0   0.0  -1.0
///
///     Trace:  0.0
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No checking is performed to guard against floating point
///      overflow or underflow. This routine should probably not be
///      used if the input matrix is expected to have large double
///      precision numbers along the diagonal.
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
/// -    SPICELIB Version 1.0.2, 03-JUL-2020 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example.
///
///         Added IMPLICIT NONE statement.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn trace(matrix: &[[f64; 3]; 3]) -> f64 {
    let ret = TRACE(matrix.as_flattened());
    ret
}

//$Procedure TRACE ( Trace of a 3x3 matrix )
pub fn TRACE(MATRIX: &[f64]) -> f64 {
    let MATRIX = DummyArray2D::new(MATRIX, 1..=3, 1..=3);
    let mut TRACE: f64 = 0.0;

    TRACE = ((MATRIX[[1, 1]] + MATRIX[[2, 2]]) + MATRIX[[3, 3]]);

    TRACE
}
