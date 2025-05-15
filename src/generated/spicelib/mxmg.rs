//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Matrix times matrix, general dimension
///
/// Multiply two double precision matrices of arbitrary size.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M1         I   NR1   x NC1R2 double precision matrix.
///  M2         I   NC1R2 x NC2   double precision matrix.
///  NR1        I   Row dimension of M1 (and also MOUT).
///  NC1R2      I   Column dimension of M1 and row dimension of M2.
///  NC2        I   Column dimension of M2 (and also MOUT).
///  MOUT       O   NR1 x NC2 double precision matrix.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M1       is any double precision matrix of arbitrary size.
///
///  M2       is any double precision matrix of arbitrary size.
///           The number of rows in M2 must match the number of
///           columns in M1.
///
///  NR1      is the number of rows in both M1 and MOUT.
///
///  NC1R2    is the number of columns in M1 and (by necessity)
///           the number of rows of M2.
///
///  NC2      is the number of columns in both M2 and MOUT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MOUT     is a a double precision matrix of dimension
///           NR1 x NC2. MOUT is the product matrix given
///           by MOUT = (M1) x (M2). MOUT must not overwrite
///           M1 or M2.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NC1R2 < 1, the elements of the matrix MOUT are set equal to
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
///     MOUT(I,J) = Summation from K=1 to NC1R2 of ( M1(I,K) * M2(K,J)
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
///  1) Given a 3x2 and a 2x3 matrices, multiply the first matrix by
///     the second one.
///
///
///     Example code begins here.
///
///
///           PROGRAM MXMG_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      M1   ( 3, 2 )
///           DOUBLE PRECISION      M2   ( 2, 3 )
///           DOUBLE PRECISION      MOUT ( 3, 3 )
///
///           INTEGER               I
///           INTEGER               J
///
///     C
///     C     Define M1 and M2.
///     C
///           DATA                  M1 /  1.0D0,  2.0D0, 3.0D0,
///          .                            4.0D0,  5.0D0, 6.0D0  /
///
///           DATA                  M2 /  1.0D0,  2.0D0,
///          .                            3.0D0,  4.0D0,
///          .                            5.0D0,  6.0D0  /
///
///     C
///     C     Multiply M1 by M2.
///     C
///           CALL MXMG ( M1, M2, 3, 2, 3, MOUT )
///
///           WRITE(*,'(A)') 'M1 times M2:'
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
///     M1 times M2:
///          9.000    19.000    29.000
///         12.000    26.000    40.000
///         15.000    33.000    51.000
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No error checking is performed to prevent numeric overflow or
///      underflow.
///
///  2)  No error checking performed to determine if the input and
///      output matrices have, in fact, been correctly dimensioned.
///
///  3)  MOUT should not overwrite M1 or M2.
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
///         Changed input argument names ROW1, COL1 and COL2 to NR1, NC1R2
///         and NC2 for consistency with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example based on the existing example.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn mxmg(m1: &[f64], m2: &[f64], nr1: i32, nc1r2: i32, nc2: i32, mout: &mut [f64]) {
    MXMG(m1, m2, nr1, nc1r2, nc2, mout);
}

//$Procedure MXMG ( Matrix times matrix, general dimension )
pub fn MXMG(M1: &[f64], M2: &[f64], NR1: i32, NC1R2: i32, NC2: i32, MOUT: &mut [f64]) {
    let M1 = DummyArray2D::new(M1, 1..=NR1, 1..=NC1R2);
    let M2 = DummyArray2D::new(M2, 1..=NC1R2, 1..=NC2);
    let mut MOUT = DummyArrayMut2D::new(MOUT, 1..=NR1, 1..=NC2);
    let mut SUM: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Perform the matrix multiplication
    //
    for I in 1..=NR1 {
        for J in 1..=NC2 {
            SUM = 0.0;
            for K in 1..=NC1R2 {
                SUM = (SUM + (M1[[I, K]] * M2[[K, J]]));
            }
            MOUT[[I, J]] = SUM;
        }
    }
    //
}
