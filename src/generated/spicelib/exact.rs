//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Round to exact value
///
/// Round an input double precision number to a specified exact value
/// if the number and the value are equal to within some tolerance.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NUMBER     I   Double precision number.
///  VALUE      I   Target value.
///  TOL        I   Tolerance.
///
///  The function returns VALUE whenever |NUMBER - VALUE| < TOL.
///                                                       -
/// ```
///
/// # Detailed Input
///
/// ```text
///  NUMBER   is an arbitrary double precision number.
///
///  VALUE    is a target value.
///
///  TOL      is a tolerance. NUMBER and VALUE are considered to
///           be equal if they differ by no more than this amount.
///           If TOL is negative, they are never considered equal.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns VALUE whenever |NUMBER - VALUE| < TOL, and
///  otherwise returns NUMBER.                            -
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
///  C
///  C     If the eccentricity is near one, make this a parabola.
///  C
///        ECC = EXACT ( ECC, 1.D0, 10.D-12 )
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn exact(number: f64, value: f64, tol: f64) -> f64 {
    let ret = EXACT(number, value, tol);
    ret
}

//$Procedure EXACT ( Round to exact value )
pub fn EXACT(NUMBER: f64, VALUE: f64, TOL: f64) -> f64 {
    let mut EXACT: f64 = 0.0;

    //
    // Just shorthand, really.
    //
    if (f64::abs((NUMBER - VALUE)) <= TOL) {
        EXACT = VALUE;
    } else {
        EXACT = NUMBER;
    }

    EXACT
}
