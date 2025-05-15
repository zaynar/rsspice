//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Same Sign Double Precision Numbers
///
/// Return .TRUE. if two given double precision numbers have the same
/// sign.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   A double precision number
///  Y          I   A double precision number
///
///  The function returns .TRUE. if the input arguments have the same
///  sign.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is any double precision number.
///
///  Y        is any double precision number.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns .TRUE. if X and Y are both positive or both
///  negative. Otherwise, it returns .FALSE.
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
///  This routine returns the value:
///
///        (      (( X .GT. 0) .AND. (Y .GT. 0))
///          .OR. (( X .LT. 0) .AND. (Y .LT. 0)) )
///
///  This is a more stable value than
///
///        ( X*Y .GT. 0 )
///
///  Note: If either of the to inputs is zero. The result returned
///  will be .FALSE.
/// ```
///
/// # Examples
///
/// ```text
///  This routine can be used whenever a decision depends upon two
///  Double Precision values having the same sign.
///
///  IF ( SMSGND ( F(X1), F(X2) ) ) THEN
///        .
///        .
///     do something
///        .
///        .
///  ELSE
///        .
///        .
///     find a root of F lying between X1 and X2
///        .
///        .
///  END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn smsgnd(x: f64, y: f64) -> bool {
    let ret = SMSGND(x, y);
    ret
}

//$Procedure SMSGND  ( Same Sign Double Precision Numbers )
pub fn SMSGND(X: f64, Y: f64) -> bool {
    let mut SMSGND: bool = false;

    SMSGND = (((X > 0 as f64) && (Y > 0 as f64)) || ((X < 0 as f64) && (Y < 0 as f64)));

    SMSGND
}
