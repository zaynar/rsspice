//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Non blank length of a string
///
/// Return the non-blank length of a character string. (That is,
/// the index of the last non-blank character when the string is
/// left-justified.)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   Input character string.
///
///  The function returns the non-blank length of STRING.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is the input character string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the non-blank length of STRING. This is the
///  same as the index of the last non-blank character in the left
///  justified string. If STRING is blank, NBLEN is zero.
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
///  Locate the first and last non-blank characters in the string.
///  Subtract to get the non-blank length.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples illustrate the use of NBLEN.
///
///        NBLEN ( 'ABCDE' )             = 5
///        NBLEN ( 'AN EXAMPLE' )        = 10
///        NBLEN ( '   AN EXAMPLE  ' )   = 10
///        NBLEN ( '               ' )   = 0
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
/// -    SPICELIB Version 1.1.0, 08-APR-2021 (JDR)
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
pub fn nblen(string: &str) -> i32 {
    let ret = NBLEN(string.as_bytes());
    ret
}

//$Procedure NBLEN ( Non blank length of a string )
pub fn NBLEN(STRING: &[u8]) -> i32 {
    let mut NBLEN: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Blank string is easy.
    //
    if fstr::eq(STRING, b" ") {
        NBLEN = 0;
    } else {
        NBLEN = ((LASTNB(STRING) - FRSTNB(STRING)) + 1);
    }

    NBLEN
}
