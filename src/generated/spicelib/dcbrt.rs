//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const THIRD: f64 = (1.0 / 3.0);

/// Double precision cube root
///
/// Return the cube root of a double precision number.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I     Number whose cube root is desired.
///
///  The function returns the cube root of the input value.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is a double precision value.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the cube root of the input value.
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
///  DCBRT calculates the cube root of the input value by using the
///  exponentiation operator to raise the input value to the 1/3
///  power. This operation, however, is performed on the absolute
///  value of the input variable, and then the sign of the input
///  is transferred to the output value.
///
///  All values of the input variable X should be acceptable to the
///  DCBRT.
/// ```
///
/// # Examples
///
/// ```text
///  The following table gives sample values of the variable X and
///  DCBRT(X)
///
///   X                        DCBRT(X)
///  --------------------------------------------------------------
///   0.0D0                    0.0D0
///   8.0D0                    2.0D0
///  -1.0D3                   -1.0D1
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
/// -    SPICELIB Version 1.1.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
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
pub fn dcbrt(x: f64) -> f64 {
    let ret = DCBRT(x);
    ret
}

//$Procedure DCBRT ( Double precision cube root )
pub fn DCBRT(X: f64) -> f64 {
    let mut DCBRT: f64 = 0.0;

    //
    DCBRT = f64::copysign(f64::powf(f64::abs(X), THIRD), X);
    //
    DCBRT
}
