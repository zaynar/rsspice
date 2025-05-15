//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Character position, reverse
///
/// Find the first occurrence in a string of a character belonging
/// to a collection of characters, starting at a specified location,
/// searching in reverse.
///
/// # Required Reading
///
/// * [SCANNING](crate::required_reading::scanning)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STR        I   Any character string.
///  CHARS      I   A collection of characters.
///  START      I   Position to begin looking for one of CHARS
///
///  The function returns the index of the last character of STR
///  at or before index START that is in the collection CHARS.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STR      is any character string.
///
///  CHARS    is a character string containing a collection of
///           characters. Spaces in CHARS are significant.
///
///  START    is the position in STR to begin looking for one of the
///           characters in CHARS.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the index of the last character of STR (at
///  or before index START) that is one of the characters in the
///  string CHARS. If none of the characters is found, the function
///  returns zero.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If START is less than 1, CPOSR returns zero.
///
///  2)  If START is greater than LEN(STRING), the search begins
///      at the last character of the string.
/// ```
///
/// # Particulars
///
/// ```text
///  CPOSR is case sensitive.
///
///  An entire family of related SPICELIB routines (POS, CPOS, NCPOS,
///  POSR, CPOSR, NCPOSR) is described in the Required Reading.
///
///  Those familiar with the True BASIC language should note that
///  these functions are equivalent to the True BASIC intrinsic
///  functions with the same name.
/// ```
///
/// # Examples
///
/// ```text
///  Let STRING = 'BOB, JOHN, TED, AND MARTIN    '
///                123456789012345678901234567890
///
///  Normal (sequential) searching:
///  ------------------------------
///
///        CPOSR( STRING, ' ,',    30 ) = 30
///
///        CPOSR( STRING, ' ,',    29 ) = 29
///
///        CPOSR( STRING, ' ,',    28 ) = 28
///
///        CPOSR( STRING, ' ,',    27 ) = 27
///
///        CPOSR( STRING, ' ,',    26 ) = 20
///
///        CPOSR( STRING, ' ,',    19 ) = 16
///
///        CPOSR( STRING, ' ,',    15 ) = 15
///
///        CPOSR( STRING, ' ,',    14 ) = 11
///
///        CPOSR( STRING, ' ,',    10 ) = 10
///
///        CPOSR( STRING, ' ,',     9 ) =  5
///
///        CPOSR( STRING, ' ,',     4 ) =  4
///
///        CPOSR( STRING, ' ,',     3 ) =  0
///
///  START out of bounds:
///  --------------------
///
///        CPOSR( STRING, ' ,',   231 ) = 30
///
///        CPOSR( STRING, ' ,',    31 ) = 30
///
///        CPOSR( STRING, ' ,',     0 ) =  0
///
///        CPOSR( STRING, ' ,',   -10 ) =  0
///
///
///  Order within CHARS
///  ------------------
///
///        CPOSR( STRING, 'JOHN',  23 ) =  18
///
///        CPOSR( STRING, 'OHNJ',  23 ) =  18
///
///        CPOSR( STRING, 'HNJO',  23 ) =  18
///
///        CPOSR( STRING, 'NJOH',  23 ) =  18
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
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
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.3, 31-JAN-2008 (BVS)
///
///         Removed non-standard end-of-declarations marker
///         'C%&END_DECLARATIONS' from comments.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 26-MAR-1991 (HAN)
///
///         The Required Reading file POSITION was renamed to SCANNING.
///         This header was updated to reflect the change.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn cposr(str: &str, chars: &str, start: i32) -> i32 {
    let ret = CPOSR(str.as_bytes(), chars.as_bytes(), start);
    ret
}

//$Procedure CPOSR ( Character position, reverse )
pub fn CPOSR(STR: &[u8], CHARS: &[u8], START: i32) -> i32 {
    let mut CPOSR: i32 = 0;
    let mut FOUND: bool = false;
    let mut LENSTR: i32 = 0;
    let mut B: i32 = 0;

    //
    // Local variables
    //

    LENSTR = intrinsics::LEN(STR);

    B = intrinsics::MIN0(&[LENSTR, START]);
    FOUND = false;
    CPOSR = 0;

    while !FOUND {
        if (B <= 0) {
            return CPOSR;
        } else if (intrinsics::INDEX(CHARS, fstr::substr(STR, B..=B)) != 0) {
            CPOSR = B;
            return CPOSR;
        } else {
            B = (B - 1);
        }
    }

    CPOSR
}
