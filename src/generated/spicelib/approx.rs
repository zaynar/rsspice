//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Approximate equality
///
/// Return .TRUE. if two double precision numbers are equal to
/// within some tolerance.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X,
///  Y          I   Double precision numbers.
///  TOL        I   Tolerance.
///
///  The function is .TRUE. whenever |X - Y| <= TOL.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X,
///  Y        are arbitrary double precision numbers.
///
///  TOL      is a tolerance. X and Y are considered to be equal
///           if they differ by no more than this amount. If TOL
///           is negative, X and Y are never considered equal.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function is .TRUE. whenever |X - Y| <= TOL, and is .FALSE.
///  otherwise.
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
///  C     If the eccentricity is near one, this a parabola.
///  C
///        IF ( APPROX ( ECC, 1.D0, 10.D-12 ) ) THEN
///           TYPE = 'PARABOLA'
///
///        ELSE IF ( ECC .LT. 1 ) THEN
///           TYPE = 'ELLIPSE'
///
///        ELSE
///           TYPE = 'HYPERBOLA'
///        END IF
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
/// -    SPICELIB Version 1.1.0, 26-OCT-2021 (JDR)
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
pub fn approx(x: f64, y: f64, tol: f64) -> bool {
    let ret = APPROX(x, y, tol);
    ret
}

//$Procedure APPROX ( Approximate equality )
pub fn APPROX(X: f64, Y: f64, TOL: f64) -> bool {
    let mut APPROX: bool = false;

    //
    // Just shorthand, really.
    //
    APPROX = (f64::abs((X - Y)) <= TOL);

    APPROX
}
