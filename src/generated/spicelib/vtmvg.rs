//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Vector transpose times matrix times vector
///
/// Multiply the transpose of a n-dimensional column vector,
/// a nxm matrix, and a m-dimensional column vector.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  V1         I   N-dimensional double precision column vector.
///  MATRIX     I   NxM double precision matrix.
///  V2         I   M-dimensional double precision column vector.
///  NROW       I   Number of rows in MATRIX (number of rows in V1.)
///  NCOL       I   Number of columns in MATRIX (number of rows in V2.)
///
///  The function returns the result of (V1**T * MATRIX * V2 ).
/// ```
///
/// # Detailed Input
///
/// ```text
///  V1       is an n-dimensional double precision vector.
///
///  MATRIX   is an n x m double precision matrix.
///
///  V2       is an m-dimensional double precision vector.
///
///  NROW     is the number of rows in MATRIX. This is also
///           equivalent to the number of rows in the vector V1.
///
///  NCOL     is the number of columns in MATRIX. This is also
///           equivalent to the number of rows in the vector V2.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the double precision value of the equation
///  (V1**T * MATRIX * V2 ).
///
///  Notice that VTMVG is actually the dot product of the vector
///  resulting from multiplying the transpose of V1 and MATRIX and the
///  vector V2.
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
///  This routine implements the following vector/matrix/vector
///  multiplication:
///
///                   T    |          |  |  |
///     VTMVG = [   V1   ] |  MATRIX  |  |V2|
///                        |          |  |  |
///
///  by calculating over all values of the indices K and L from 1 to
///  NROW and 1 to NCOL, respectively, the expression
///
///     VTMVG = Summation of ( V1(K)*MATRIX(K,L)*V2(L) ) .
///
///  V1 is a column vector which becomes a row vector when transposed.
///  V2 is a column vector.
///
///  No checking is performed to determine whether floating point
///  overflow has occurred.
/// ```
///
/// # Examples
///
/// ```text
///  If  V1 = | 1.0D0 |  MATRIX = | 2.0D0  0.0D0 |  V2 = | 1.0D0 |
///           |       |           |              |       |       |
///           | 2.0D0 |           | 1.0D0  2.0D0 |       | 2.0D0 |
///           |       |           |              |
///           | 3.0D0 |           | 1.0D0  1.0D0 |
///
///  NROW = 3
///  NCOL = 2
///
///  then the value of the function is  21.0D0.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Since no error detection or recovery is implemented, the
///      programmer is required to insure that the inputs to this
///      routine are both valid and within the proper range.
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
/// -    SPICELIB Version 1.1.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added comments
///         to the code and moved the declaration of each local variable to
///         a separate line.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn vtmvg(v1: &[f64], matrix: &[f64], v2: &[f64], nrow: i32, ncol: i32) -> f64 {
    let ret = VTMVG(v1, matrix, v2, nrow, ncol);
    ret
}

//$Procedure VTMVG  ( Vector transpose times matrix times vector )
pub fn VTMVG(V1: &[f64], MATRIX: &[f64], V2: &[f64], NROW: i32, NCOL: i32) -> f64 {
    let V1 = DummyArray::new(V1, 1..=NROW);
    let MATRIX = DummyArray2D::new(MATRIX, 1..=NROW, 1..=NCOL);
    let V2 = DummyArray::new(V2, 1..=NCOL);
    let mut VTMVG: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Perform the multiplication
    //
    VTMVG = 0.0;
    for K in 1..=NROW {
        for L in 1..=NCOL {
            VTMVG = (VTMVG + ((V1[K] * MATRIX[[K, L]]) * V2[L]));
        }
    }

    VTMVG
}
