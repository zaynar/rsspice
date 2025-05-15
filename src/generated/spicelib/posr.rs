//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Position of substring, reverse search
///
/// Find the first occurrence in a string of a substring, starting at
/// a specified location, searching in reverse.
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
///  STR        I   A character string
///  SUBSTR     I   Substring to locate in the character string.
///  START      I   Where to start looking for SUBSTR in STR.
///
///  The function returns the index of SUBSTR in STR preceding START
/// ```
///
/// # Detailed Input
///
/// ```text
///  STR      is any character string.
///
///  SUBSTR   is a substring to look for in STR. Spaces in
///           SUBSTR are significant.
///
///  START    is the position in STR to begin looking for SUBSTR.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the index of the beginning of the last
///  substring of STR that begins on or before index START and is
///  equal to SUBSTR. If the substring cannot be found starting at or
///  before START, the function is returns 0.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If START is less than 1, POSR returns zero.
///
///  2)  If START is greater than LEN(STRING), the search begins
///      at the last character of the string.
/// ```
///
/// # Particulars
///
/// ```text
///  POSR is case sensitive.
///
///  An entire family of related SPICELIB routines (POS, CPOS, NCPOS,
///  POSR, CPOSR, NCPOSR) is described in the Required Reading.
///
///  Those familiar with the .TRUE. BASIC language should note that
///  these functions are equivalent to the .TRUE. BASIC intrinsic
///  functions with the same name.
/// ```
///
/// # Examples
///
/// ```text
///  Let STRING = 'AN ANT AND AN ELEPHANT        '
///                123456789012345678901234567890
///
///  Normal (Sequential) Searching:
///  ------------------------------
///
///        POSR ( STRING, 'AN',  31 ) = 20
///        POSR ( STRING, 'AN',  19 ) = 12
///        POSR ( STRING, 'AN',  11 ) =  8
///        POSR ( STRING, 'AN',   7 ) =  4
///        POSR ( STRING, 'AN',   3 ) =  1
///        POSR ( STRING, 'AN',   0 ) =  0
///
///  START out of bounds:
///  --------------------
///
///        POSR ( STRING, 'AN', -5 ) =  0
///        POSR ( STRING, 'AN',  0 ) =  0
///        POSR ( STRING, 'AN', 31 ) = 20
///        POSR ( STRING, 'AN', 44 ) = 20
///
///  Significance of Spaces:
///  -----------------------
///
///        POSR ( STRING, 'AN',    31 ) =  20
///        POSR ( STRING, ' AN',   31 ) =  11
///        POSR ( STRING, ' AN ',  31 ) =  11
///        POSR ( STRING, ' AN ',  10 ) =   0
///        POSR ( STRING, ' AN  ', 31 ) =   0
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  K.S. Zukor         (JPL)
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
/// -    SPICELIB Version 1.0.4, 31-JAN-2008 (BVS)
///
///         Removed non-standard end-of-declarations marker
///         'C%&END_DECLARATIONS' from comments.
///
/// -    SPICELIB Version 1.0.3, 25-AUG-1994 (HAN) (KSZ)
///
///         $Examples section of the header used POS instead of POSR.
///         Also, some examples were incorrect. They have been corrected.
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
pub fn posr(str: &str, substr: &str, start: i32) -> i32 {
    let ret = POSR(str.as_bytes(), substr.as_bytes(), start);
    ret
}

//$Procedure POSR ( Position of substring, reverse search)
pub fn POSR(STR: &[u8], SUBSTR: &[u8], START: i32) -> i32 {
    let mut POSR: i32 = 0;
    let mut FOUND: bool = false;
    let mut LENSTR: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut FCHNCE: i32 = 0;
    let mut B: i32 = 0;

    //
    // Local variables
    //

    //
    // Let's find out how big every body is.
    //
    LENSTR = intrinsics::LEN(STR);
    OFFSET = intrinsics::MAX0(&[0, (intrinsics::LEN(SUBSTR) - 1)]);
    FCHNCE = (LENSTR - OFFSET);

    //
    // Look for the string until we run find it or run out of room to
    // look.
    //
    B = intrinsics::MIN0(&[FCHNCE, START]);
    FOUND = false;
    POSR = 0;

    while !FOUND {
        if (B <= 0) {
            return POSR;
        } else if fstr::eq(fstr::substr(STR, B..=(B + OFFSET)), SUBSTR) {
            POSR = B;
            return POSR;
        } else {
            B = (B - 1);
        }
    }

    POSR
}
