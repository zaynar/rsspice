//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Matrix equal to another, 3x3
///
/// Set one double precision 3x3 matrix equal to another.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M1         I   Input matrix.
///  MOUT       O   Output matrix equal to M1.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M1       is an arbitrary input 3x3 matrix. There are no
///           restrictions on what it may contain.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MOUT     is a 3x3 matrix set to be equal to M1.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
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
///  1) This trivial example demonstrates how to use MEQU to assign
///     one matrix to another.
///
///
///     Example code begins here.
///
///
///           PROGRAM MEQU_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      M1   ( 3, 3 )
///           DOUBLE PRECISION      MOUT ( 3, 3 )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define M1.
///     C
///           DATA                  M1   /  0.0D0,  1.0D0,  0.0D0,
///          .                             -1.0D0,  0.0D0,  0.0D0,
///          .                              0.0D0,  0.0D0,  1.0D0  /
///
///     C
///     C     Assign M1 to MOUT and print MOUT.
///     C
///           CALL MEQU ( M1, MOUT )
///
///           WRITE(*,'(A)') 'MOUT:'
///           DO I=1, 3
///
///              WRITE(*,'(3F16.7)') ( MOUT(I,J), J=1,3 )
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
///     MOUT:
///            0.0000000      -1.0000000       0.0000000
///            1.0000000       0.0000000       0.0000000
///            0.0000000       0.0000000       1.0000000
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
/// -    SPICELIB Version 1.1.0, 04-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn mequ(m1: &[[f64; 3]; 3], mout: &mut [[f64; 3]; 3]) {
    MEQU(m1.as_flattened(), mout.as_flattened_mut());
}

//$Procedure MEQU ( Matrix equal to another, 3x3 )
pub fn MEQU(M1: &[f64], MOUT: &mut [f64]) {
    let M1 = DummyArray2D::new(M1, 1..=3, 1..=3);
    let mut MOUT = DummyArrayMut2D::new(MOUT, 1..=3, 1..=3);

    MOVED(M1.as_slice(), 9, MOUT.as_slice_mut());
}
