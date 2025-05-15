//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Matrix transpose times matrix, general dimension
///
/// Multiply the transpose of a matrix with another matrix,
/// both of arbitrary size. (The dimensions of the matrices must be
/// compatible with this multiplication.)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M1         I   Left-hand matrix whose transpose is to be
///                 multiplied.
///  M2         I   Right-hand matrix to be multiplied.
///  NC1        I   Column dimension of M1 and row dimension of MOUT.
///  NR1R2      I   Row dimension of both M1 and M2.
///  NC2        I   Column dimension of both M2 and MOUT.
///  MOUT       O   Product matrix M1**T * M2.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M1       is an double precision matrix of arbitrary dimension
///           whose transpose is the left hand multiplier of a
///           matrix multiplication.
///
///  M2       is an double precision matrix of arbitrary dimension
///           whose transpose is the left hand multiplier of a
///           matrix multiplication.
///
///  NC1      is the column dimension of M1 and row dimension of
///           MOUT.
///
///  NR1R2    is the row dimension of both M1 and M2.
///
///  NC2      is the column dimension of both M2 and MOUT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MOUT     is a double precision matrix containing the product
///
///                        T
///              MOUT =  M1  x  M2
///
///           where the superscript T denotes the transpose of M1.
///
///           MOUT must NOT overwrite either M1 or M2.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NR1R2 < 1, the elements of the matrix MOUT are set equal to
///      zero.
/// ```
///
/// # Particulars
///
/// ```text
///  The code reflects precisely the following mathematical expression
///
///  For each value of the subscript I from 1 to NC1, and J from 1
///  to NC2:
///
///  MOUT(I,J) = Summation from K=1 to NR1R2 of  ( M1(K,I) * M2(K,J) )
///
///  Note that the reversal of the K and I subscripts in the left-hand
///  matrix M1 is what makes MOUT the product of the TRANSPOSE of M1
///  and not simply of M1 itself.
///
///  Since this subroutine operates on matrices of arbitrary size, it
///  is not possible to buffer intermediate results. Thus, MOUT
///  should NOT overwrite either M1 or M2.
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
///  1) Given a 2x4 and a 2x3 matrices, multiply the transpose of the
///     first matrix by the second one.
///
///
///     Example code begins here.
///
///
///           PROGRAM MTXMG_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      M1   ( 4, 2 )
///           DOUBLE PRECISION      M2   ( 2, 3 )
///           DOUBLE PRECISION      MOUT ( 4, 3 )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define M1 and M2.
///     C
///           DATA                  M1 /  1.0D0,  1.0D0,
///          .                            2.0D0,  1.0D0,
///          .                            3.0D0,  1.0D0,
///          .                            0.0D0,  1.0D0  /
///
///           DATA                  M2 /  1.0D0,  0.0D0,
///          .                            2.0D0,  0.0D0,
///          .                            3.0D0,  0.0D0  /
///
///     C
///     C     Multiply the transpose of M1 by M2.
///     C
///           CALL MTXMG ( M1, M2, 4, 2, 3, MOUT )
///
///           WRITE(*,'(A)') 'Transpose of M1 times M2:'
///           DO I = 1, 4
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
///          1.000     2.000     3.000
///          2.000     4.000     6.000
///          3.000     6.000     9.000
///          0.000     0.000     0.000
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The user is responsible for checking the magnitudes of the
///      elements of M1 and M2 so that a floating point overflow does
///      not occur.
///
///  2)  MOUT must not overwrite M1 or M2 or else the intermediate
///      will affect the final result.
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
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Added complete code example based on the existing example.
///
///         Added entry #1 to $Exceptions section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn mtxmg(m1: &[f64], m2: &[f64], nc1: i32, nr1r2: i32, nc2: i32, mout: &mut [f64]) {
    MTXMG(m1, m2, nc1, nr1r2, nc2, mout);
}

//$Procedure MTXMG ( Matrix transpose times matrix, general dimension )
pub fn MTXMG(M1: &[f64], M2: &[f64], NC1: i32, NR1R2: i32, NC2: i32, MOUT: &mut [f64]) {
    let M1 = DummyArray2D::new(M1, 1..=NR1R2, 1..=NC1);
    let M2 = DummyArray2D::new(M2, 1..=NR1R2, 1..=NC2);
    let mut MOUT = DummyArrayMut2D::new(MOUT, 1..=NC1, 1..=NC2);

    //
    // Local variables
    //

    //
    // Perform the matrix multiplication
    //
    for I in 1..=NC1 {
        for J in 1..=NC2 {
            MOUT[[I, J]] = 0.0;
            for K in 1..=NR1R2 {
                MOUT[[I, J]] = (MOUT[[I, J]] + (M1[[K, I]] * M2[[K, J]]));
            }
        }
    }
    //
}
