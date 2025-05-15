//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Matrix times matrix transpose, general dimension
///
/// Multiply a matrix and the transpose of a matrix, both of
/// arbitrary size.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M1         I   Left-hand matrix to be multiplied.
///  M2         I   Right-hand matrix whose transpose is to be
///                 multiplied.
///  NR1        I   Row dimension of M1 and row dimension of MOUT.
///  NC1C2      I   Column dimension of M1 and column dimension of M2.
///  NR2        I   Row dimension of M2 and column dimension of MOUT.
///  MOUT       O   Product matrix M1 * M2**T.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M1       is any double precision matrix of arbitrary size.
///
///  M2       is any double precision matrix of arbitrary size.
///
///           The number of columns in M2 must match the number of
///           columns in M1.
///
///  NR1      is the number of rows in both M1 and MOUT.
///
///  NC1C2    is the number of columns in M1 and (by necessity)
///           the number of columns of M2.
///
///  NR2      is the number of rows in both M2 and the number of
///           columns in MOUT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MOUT     is a double precision matrix of dimension NR1 x NR2.
///
///           MOUT is the product matrix given by
///
///                            T
///              MOUT = M1 x M2
///
///           where the superscript "T" denotes the transpose
///           matrix.
///
///           MOUT must not overwrite M1 or M2.
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
///  For each value of the subscript I from 1 to NR1, and J from 1
///  to NR2:
///
///  MOUT(I,J) = Summation from K=1 to NC1C2 of  ( M1(I,K) * M2(J,K) )
///
///  Notice that the order of the subscripts of M2 are reversed from
///  what they would be if this routine merely multiplied M1 and M2.
///  It is this transposition of subscripts that makes this routine
///  multiply M1 and the TRANPOSE of M2.
///
///  Since this subroutine operates on matrices of arbitrary size, it
///  is not feasible to buffer intermediate results. Thus, MOUT
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
///  1) Given a 2x3 and a 3x4 matrices, multiply the first matrix by
///     the transpose of the second one.
///
///
///     Example code begins here.
///
///
///           PROGRAM MXMTG_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      M1   ( 2, 3 )
///           DOUBLE PRECISION      M2   ( 4, 3 )
///           DOUBLE PRECISION      MOUT ( 2, 4 )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define M1 and M2.
///     C
///           DATA                  M1 /  1.0D0, 3.0D0,
///          .                            2.0D0, 2.0D0,
///          .                            3.0D0, 1.0D0  /
///
///           DATA                  M2 /  1.0D0, 2.0D0, 1.0D0, 2.0D0,
///          .                            2.0D0, 1.0D0, 2.0D0, 1.0D0,
///          .                            0.0D0, 2.0D0, 0.0D0, 2.0D0 /
///
///     C
///     C     Multiply M1 by the transpose of M2.
///     C
///           CALL MXMTG ( M1, M2, 2, 3, 4, MOUT )
///
///           WRITE(*,'(A)') 'M1 times transpose of M2:'
///           DO I = 1, 2
///
///              WRITE(*,'(4F10.3)') ( MOUT(I,J), J=1,4)
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
///     M1 times transpose of M2:
///          5.000    10.000     5.000    10.000
///          7.000    10.000     7.000    10.000
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No error checking is performed to prevent numeric overflow or
///      underflow.
///
///      The user is responsible for checking the magnitudes of the
///      elements of M1 and M2 so that a floating point overflow does
///      not occur.
///
///  2)  No error checking is performed to determine if the input and
///      output matrices have, in fact, been correctly dimensioned.
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
///         Edited the header to comply with NAIF standard.
///         Added complete code example based on the existing example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///          Comment section for permuted index source lines was added
///          following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn mxmtg(m1: &[f64], m2: &[f64], nr1: i32, nc1c2: i32, nr2: i32, mout: &mut [f64]) {
    MXMTG(m1, m2, nr1, nc1c2, nr2, mout);
}

//$Procedure MXMTG  ( Matrix times matrix transpose, general dimension )
pub fn MXMTG(M1: &[f64], M2: &[f64], NR1: i32, NC1C2: i32, NR2: i32, MOUT: &mut [f64]) {
    let M1 = DummyArray2D::new(M1, 1..=NR1, 1..=NC1C2);
    let M2 = DummyArray2D::new(M2, 1..=NR2, 1..=NC1C2);
    let mut MOUT = DummyArrayMut2D::new(MOUT, 1..=NR1, 1..=NR2);
    let mut SUM: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Perform the matrix multiplication
    //
    for I in 1..=NR1 {
        for J in 1..=NR2 {
            SUM = 0.0;

            for K in 1..=NC1C2 {
                SUM = (SUM + (M1[[I, K]] * M2[[J, K]]));
            }

            MOUT[[I, J]] = SUM;
        }
    }
}
