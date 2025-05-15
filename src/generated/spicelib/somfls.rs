//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Some entries false?
///
/// Determine if some of the entries in an array of logicals are
/// .FALSE.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LOGCLS     I   An array of logicals.
///  N          I   Number of elements in the array LOGCLS.
///
///  The function returns .TRUE. if some of the values in the array
///  LOGCLS are .FALSE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LOGCLS   is an array of logicals.
///
///  N        is the number of elements in the array LOGCLS
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns true if the value of some entry of LOGCLS
///  is .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If N is less than 1, the function returns a value of .FALSE.
/// ```
///
/// # Particulars
///
/// ```text
///  This function examines each element of LOGCLS until
///  a .FALSE. value is found or until all values have been
///  examined.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you need to confirm that a character set
///  WORDS does not contain at least one of the words in the phrase
///
///    'EVERY GOOD BOY DOES FINE'
///
///  You might execute the following block of code.
///
///        FOUND(1) = ELEMC  ( 'EVERY', WORDS )
///        FOUND(2) = ELEMC  ( 'GOOD',  WORDS )
///        FOUND(3) = ELEMC  ( 'BOY',   WORDS )
///        FOUND(4) = ELEMC  ( 'DOES',  WORDS )
///        FOUND(5) = ELEMC  ( 'FINE',  WORDS )
///
///        OK       = SOMFLS ( FOUND,   5     )
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
/// -    SPICELIB Version 1.0.0, 12-JUL-1991 (WLT)
/// ```
pub fn somfls(logcls: &[bool], n: i32) -> bool {
    let ret = SOMFLS(logcls, n);
    ret
}

//$Procedure SOMFLS ( Some entries false? )
pub fn SOMFLS(LOGCLS: &[bool], N: i32) -> bool {
    let LOGCLS = DummyArray::new(LOGCLS, 1..);
    let mut SOMFLS: bool = false;

    //
    // Just do it.
    //

    for I in 1..=N {
        if !LOGCLS[I] {
            SOMFLS = true;
            return SOMFLS;
        }
    }

    SOMFLS = false;
    SOMFLS
}
