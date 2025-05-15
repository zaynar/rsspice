//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Same Sign Integer Numbers
///
/// Return .TRUE. if two given integer numbers have the same sign.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   An integer.
///  Y          I   An integer.
///
///  The function returns .TRUE. if the input arguments have the same
///  sign.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is any integer.
///
///  Y        is any integer.
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
///  integer values having the same sign.
///
///  IF ( SMSGNI ( F(X1), F(X2) ) ) THEN
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
pub fn smsgni(x: i32, y: i32) -> bool {
    let ret = SMSGNI(x, y);
    ret
}

//$Procedure SMSGNI  ( Same Sign Integer Numbers )
pub fn SMSGNI(X: i32, Y: i32) -> bool {
    let mut SMSGNI: bool = false;

    SMSGNI = (((X > 0) && (Y > 0)) || ((X < 0) && (Y < 0)));

    SMSGNI
}
