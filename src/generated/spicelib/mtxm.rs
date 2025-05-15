//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Matrix transpose times matrix, 3x3
///
/// Multiply the transpose of a 3x3 matrix and a 3x3 matrix.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M1         I   3x3 double precision matrix.
///  M2         I   3x3 double precision matrix.
///  MOUT       O   3x3 double precision matrix which is the product
///                 (M1**T) * M2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M1       is any 3x3 double precision matrix. Typically,
///           M1 will be a rotation matrix since then its
///           transpose is its inverse (but this is NOT a
///           requirement).
///
///  M2       is any 3x3 double precision matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MOUT     is s 3x3 double precision matrix. MOUT is the
///           product MOUT = (M1**T) x M2.
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
///     For each value of the subscripts I and J from 1 to 3:
///
///     MOUT(I,J) = Summation from K=1 to 3 of  ( M1(K,I) * M2(K,J) )
///
///  Note that the reversal of the K and I subscripts in the left-hand
///  matrix M1 is what makes MOUT the product of the TRANSPOSE of M1
///  and not simply of M1 itself.
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
///  1) Given two 3x3 matrices, multiply the transpose of the first
///     matrix by the second one.
///
///
///     Example code begins here.
///
///
///           PROGRAM MTXM_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      M1   ( 3, 3 )
///           DOUBLE PRECISION      M2   ( 3, 3 )
///           DOUBLE PRECISION      MOUT ( 3, 3 )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define M1 and M2.
///     C
///           DATA                  M1 /  1.0D0,  4.0D0,  7.0D0,
///          .                            2.0D0,  5.0D0,  8.0D0,
///          .                            3.0D0,  6.0D0,  9.0D0  /
///
///           DATA                  M2 /  1.0D0, -1.0D0,  0.0D0,
///          .                            1.0D0,  1.0D0,  0.0D0,
///          .                            0.0D0,  0.0D0,  1.0D0  /
///
///     C
///     C     Multiply the transpose of M1 by M2.
///     C
///           CALL MTXM ( M1, M2, MOUT )
///
///           WRITE(*,'(A)') 'Transpose of M1 times M2:'
///           DO I = 1, 3
///
///              WRITE(*,'(3F10.3)') ( MOUT(I,J), J=1,3)
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
///     Transpose of M1 times M2:
///         -3.000     5.000     7.000
///         -3.000     7.000     8.000
///         -3.000     9.000     9.000
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The user is responsible for checking the magnitudes of the
///      elements of M1 and M2 so that a floating point overflow does
///      not occur. (In the typical use where M1 and M2 are rotation
///      matrices, this not a risk at all.)
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
/// -    SPICELIB Version 1.1.0, 04-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example based on the existing example.
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
pub fn mtxm(m1: &[[f64; 3]; 3], m2: &[[f64; 3]; 3], mout: &mut [[f64; 3]; 3]) {
    MTXM(
        m1.as_flattened(),
        m2.as_flattened(),
        mout.as_flattened_mut(),
    );
}

//$Procedure MTXM  ( Matrix transpose times matrix, 3x3 )
pub fn MTXM(M1: &[f64], M2: &[f64], MOUT: &mut [f64]) {
    let M1 = DummyArray2D::new(M1, 1..=3, 1..=3);
    let M2 = DummyArray2D::new(M2, 1..=3, 1..=3);
    let mut MOUT = DummyArrayMut2D::new(MOUT, 1..=3, 1..=3);
    let mut PRODM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);

    //
    // Local variables
    //

    //
    // Perform the matrix multiplication
    //
    for I in 1..=3 {
        for J in 1..=3 {
            PRODM[[I, J]] = (((M1[[1, I]] * M2[[1, J]]) + (M1[[2, I]] * M2[[2, J]]))
                + (M1[[3, I]] * M2[[3, J]]));
        }
    }

    //
    // Move the result into MOUT
    //
    MOVED(PRODM.as_slice(), 9, MOUT.as_slice_mut());
}
