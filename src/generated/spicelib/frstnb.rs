//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ISPACE: i32 = 32;

/// First non-blank character
///
/// Return the index of the first non-blank character in
/// a character string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   Input character string.
///
///  The function returns the index of the first non-blank character in
///  STRING.
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
///  The function returns the index if the first non-blank character in
///  the input string. If there are no non-blank characters in the
///  string, FRSTNB is zero.
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
///  If the string is blank, return zero. Otherwise, step through
///  the string one character at a time until something other than
///  a blank is found. Return the index of that something within
///  the string.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples illustrate the use of FRSTNB.
///
///        FRSTNB ( 'ABCDE'         )   = 1
///        FRSTNB ( 'AN EXAMPLE'    )   = 1
///        FRSTNB ( '   AN EXAMPLE' )   = 4
///        FRSTNB ( '             ' )   = 0
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 08-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 12-MAR-1996 (KRG)
///
///         Modified the comparison to use integer values and the ICHAR()
///         function. This improves the performance of the subroutine.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn frstnb(string: &str) -> i32 {
    let ret = FRSTNB(string.as_bytes());
    ret
}

//$Procedure FRSTNB ( First non-blank character )
pub fn FRSTNB(STRING: &[u8]) -> i32 {
    let mut FRSTNB: i32 = 0;

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Just like it says in the header.
    //
    if fstr::eq(STRING, b" ") {
        FRSTNB = 0;
    } else {
        for I in 1..=intrinsics::LEN(STRING) {
            if (intrinsics::ICHAR(fstr::substr(STRING, I..=I)) != ISPACE) {
                FRSTNB = I;
                return FRSTNB;
            }
        }
    }

    FRSTNB
}
