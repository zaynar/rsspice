//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Matrix equal to another, general dimension
///
/// Set one double precision matrix of arbitrary size equal to
/// another.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  M1         I   Input matrix.
///  NR         I   Row dimension of M1 (and also MOUT).
///  NC         I   Column dimension of M1 (and also MOUT).
///  MOUT       O   Output matrix equal to M1.
/// ```
///
/// # Detailed Input
///
/// ```text
///  M1       is an arbitrary-sized double precision matrix.
///           There are no restrictions on what it may contain.
///
///  NR       is the number of rows in the input matrix.
///
///  NC       is the number of columns in the input matrix.
/// ```
///
/// # Detailed Output
///
/// ```text
///  MOUT     is a NRxNC matrix set to be equal to M1.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NR < 1 or NC < 1, the elements of the matrix MOUT are not
///      assigned any values.
/// ```
///
/// # Examples
///
/// ```text
///  If  M1 = | 1.0D0   2.0D0 |
///           |               |
///           | 2.0D0   4.0D0 |
///           |               |
///           | 4.0D0   6.0D0 |
///
///  the call
///
///  CALL MEQUG ( M1, 3, 2, MOUT )
///
///  produces the matrix
///
///    MOUT = | 1.0D0   2.0D0 |
///           |               |
///           | 2.0D0   4.0D0 |
///           |               |
///           | 4.0D0   6.0D0 |
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
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO)
/// ```
pub fn mequg(m1: &[f64], nr: i32, nc: i32, mout: &mut [f64]) {
    MEQUG(m1, nr, nc, mout);
}

//$Procedure MEQUG  ( Matrix equal to another, general dimension )
pub fn MEQUG(M1: &[f64], NR: i32, NC: i32, MOUT: &mut [f64]) {
    let M1 = DummyArray2D::new(M1, 1..=NR, 1..=NC);
    let mut MOUT = DummyArrayMut2D::new(MOUT, 1..=NR, 1..=NC);

    MOVED(M1.as_slice(), (NR * NC), MOUT.as_slice_mut());
    //
}
