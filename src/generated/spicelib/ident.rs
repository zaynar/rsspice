//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Return the 3x3 identity matrix
///
/// Return the 3x3 identity matrix.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MATRIX     O   The 3x3 identity matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MATRIX   is the 3x3 Identity matrix. That MATRIX is
///           the following
///
///              .-                       -.
///              |  1.0D0   0.0D0   0.0D0  |
///              |  0.0D0   1.0D0   0.0D0  |
///              |  0.0D0   0.0D0   1.0D0  |
///              `-                       -'
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
///  This is a utility routine for obtaining the 3x3 identity matrix
///  so that you may avoid having to write the loop or assignments
///  needed to get the matrix.
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
///  1) Define a 3x3 matrix and compute its inverse using the SPICELIB
///     routine INVERT. Verify the accuracy of the computed inverse
///     using the mathematical identity
///
///             -1
///        M x M   - I = 0
///
///     where I is the 3x3 identity matrix.
///
///
///     Example code begins here.
///
///
///           PROGRAM IDENT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      IDMAT  ( 3, 3 )
///           DOUBLE PRECISION      IMAT   ( 3, 3 )
///           DOUBLE PRECISION      M      ( 3, 3 )
///           DOUBLE PRECISION      MOUT   ( 3, 3 )
///           DOUBLE PRECISION      MZERO  ( 3, 3 )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define a matrix to invert.
///     C
///           DATA                  M  /  0.D0,  0.5D0, 0.D0,
///          .                           -1.D0,  0.D0,  0.D0,
///          .                            0.D0,  0.D0,  1.D0 /
///
///           WRITE(*,*) 'Original Matrix:'
///           DO I=1, 3
///
///              WRITE(*,'(3F16.7)') ( M(I,J), J=1,3 )
///
///           END DO
///     C
///     C     Invert the matrix, then output.
///     C
///           CALL INVERT ( M, MOUT )
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Inverse Matrix:'
///           DO I=1, 3
///
///              WRITE(*,'(3F16.7)') ( MOUT(I,J), J=1,3 )
///
///           END DO
///
///     C
///     C     Check the M times MOUT produces the identity matrix.
///     C
///           CALL IDENT ( IDMAT )
///           CALL MXM   ( M, MOUT, IMAT )
///
///           CALL VSUBG ( IMAT, IDMAT, 9, MZERO )
///
///           WRITE(*,*) ' '
///           WRITE(*,*) 'Original times inverse minus identity:'
///           DO I=1, 3
///
///              WRITE(*,'(3F16.7)') ( MZERO(I,J), J=1,3 )
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
///      Original Matrix:
///            0.0000000      -1.0000000       0.0000000
///            0.5000000       0.0000000       0.0000000
///            0.0000000       0.0000000       1.0000000
///
///      Inverse Matrix:
///            0.0000000       2.0000000      -0.0000000
///           -1.0000000       0.0000000      -0.0000000
///            0.0000000      -0.0000000       1.0000000
///
///      Original times inverse minus identity:
///            0.0000000       0.0000000       0.0000000
///            0.0000000       0.0000000       0.0000000
///            0.0000000       0.0000000       0.0000000
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
/// -    SPICELIB Version 1.0.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
/// -    SPICELIB Version 1.0.0, 05-FEB-1996 (WLT)
/// ```
pub fn ident(matrix: &mut [[f64; 3]; 3]) {
    IDENT(matrix.as_flattened_mut());
}

//$Procedure IDENT ( Return the 3x3 identity matrix )
pub fn IDENT(MATRIX: &mut [f64]) {
    let mut MATRIX = DummyArrayMut2D::new(MATRIX, 1..=3, 1..=3);

    MATRIX[[1, 1]] = 1.0;
    MATRIX[[2, 1]] = 0.0;
    MATRIX[[3, 1]] = 0.0;

    MATRIX[[1, 2]] = 0.0;
    MATRIX[[2, 2]] = 1.0;
    MATRIX[[3, 2]] = 0.0;

    MATRIX[[1, 3]] = 0.0;
    MATRIX[[2, 3]] = 0.0;
    MATRIX[[3, 3]] = 1.0;
}
