//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Matrix times matrix transpose, 3x3
///
/// Multiply a 3x3 matrix and the transpose of another 3x3 matrix.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M1         I   3x3 double precision matrix.
///  M2         I   3x3 double precision matrix.
///  MOUT       O   The product M1 times transpose of M2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M1       is an arbitrary 3x3 double precision matrix.
///
///  M2       is an arbitrary 3x3 double precision matrix.
///           Typically, M2 will be a rotation matrix since
///           then its transpose is its inverse (but this is
///           NOT a requirement).
/// ```
///
/// # Detailed Output
///
/// ```text
///  MOUT     is a 3x3 double precision matrix. MOUT is the product
///
///                            T
///              MOUT = M1 x M2
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
///                       3
///                    .-----
///                     \
///        MOUT(I,J) =   )  M1(I,K) * M2(J,K)
///                     /
///                    '-----
///                      K=1
///
///  Note that the reversal of the K and J subscripts in the right-
///  hand matrix M2 is what makes MOUT the product of the TRANSPOSE of
///  M2 and not simply of M2 itself.
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
///  1) Given two 3x3 double precision matrices, multiply the first
///     matrix by the transpose of the second one.
///
///
///     Example code begins here.
///
///
///           PROGRAM MXMT_EX1
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
///     C     Define M1.
///     C
///           DATA                  M1   /  0.0D0, -1.0D0,  0.0D0,
///          .                              1.0D0,  0.0D0,  0.0D0,
///          .                              0.0D0,  0.0D0,  1.0D0  /
///
///     C
///     C     Make M2 equal to M1.
///     C
///           CALL MEQU ( M1, M2 )
///
///     C
///     C     Multiply M1 by the transpose of M2.
///     C
///           CALL MXMT ( M1, M2, MOUT )
///
///           WRITE(*,'(A)') 'M1:'
///           DO I=1, 3
///
///              WRITE(*,'(3F16.7)') ( M1(I,J), J=1,3 )
///
///           END DO
///
///           WRITE(*,*)
///           WRITE(*,'(A)') 'M2:'
///           DO I=1, 3
///
///              WRITE(*,'(3F16.7)') ( M2(I,J), J=1,3 )
///
///           END DO
///
///           WRITE(*,*)
///           WRITE(*,'(A)') 'M1 times transpose of M2:'
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
///     M1:
///            0.0000000       1.0000000       0.0000000
///           -1.0000000       0.0000000       0.0000000
///            0.0000000       0.0000000       1.0000000
///
///     M2:
///            0.0000000       1.0000000       0.0000000
///           -1.0000000       0.0000000       0.0000000
///            0.0000000       0.0000000       1.0000000
///
///     M1 times transpose of M2:
///            1.0000000       0.0000000       0.0000000
///            0.0000000       1.0000000       0.0000000
///            0.0000000       0.0000000       1.0000000
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
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code examples based on existing code fragments.
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
pub fn mxmt(m1: &[[f64; 3]; 3], m2: &[[f64; 3]; 3], mout: &mut [[f64; 3]; 3]) {
    MXMT(
        m1.as_flattened(),
        m2.as_flattened(),
        mout.as_flattened_mut(),
    );
}

//$Procedure MXMT ( Matrix times matrix transpose, 3x3 )
pub fn MXMT(M1: &[f64], M2: &[f64], MOUT: &mut [f64]) {
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
            PRODM[[I, J]] = (((M1[[I, 1]] * M2[[J, 1]]) + (M1[[I, 2]] * M2[[J, 2]]))
                + (M1[[I, 3]] * M2[[J, 3]]));
        }
    }
    //
    // Move the result into MOUT
    //
    MOVED(PRODM.as_slice(), 9, MOUT.as_slice_mut());
}
