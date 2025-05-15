//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// First printable character
///
/// Return the index of the first printable character in a character
/// string. ASCII characters 33-126 are printable. (Blanks are not
/// considered printable.)
///
/// # Brief I/O
///
/// ```text
///   VARIABLE  I/O  DESCRIPTION
///   --------  ---  --------------------------------------------------
///   STRING     I   Input character string.
///
///  The function returns the index of the first non-blank printable
///  character in STRING.
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
///  The function returns the index of the first printable character in
///  the input string.
///
///  ASCII characters 33-126 are considered to be printable characters.
///  Blanks are not considered printable characters. If the input
///  string contains no printable characters, FRSTPC is zero.
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
///  This works exactly like FRSTNB, except that it skips non-printable
///  characters (ASCII control characters) as well as blanks.
/// ```
///
/// # Examples
///
/// ```text
///  The program
///
///     INTEGER         FRSTNB
///     INTEGER         FRSTPC
///     INTEGER         LASTNB
///     INTEGER         LASTPC
///
///     CHARACTER*10    S
///
///     S( 1: 1) = ' '
///     S( 2: 2) = CHAR (  2 )
///     S( 3: 3) = CHAR (  3 )
///     S( 4: 4) = 'A'
///     S( 5: 5) = 'B'
///     S( 6: 6) = 'C'
///     S( 7: 7) = CHAR (  7 )
///     S( 8: 8) = CHAR (  8 )
///     S( 9: 9) = CHAR (  9 )
///     S(10:10) = ' '
///
///     WRITE (*,*) 'Non-blank from ', FRSTNB(S), ' to ', LASTNB(S)
///     WRITE (*,*) 'Printable from ', FRSTPC(S), ' to ', LASTPC(S)
///
///     END
///
///  produces the following output:
///
///     Non-blank from 2 to 9.
///     Printable from 4 to 6.
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
/// -    SPICELIB Version 1.1.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn frstpc(string: &str) -> i32 {
    let ret = FRSTPC(string.as_bytes());
    ret
}

//$Procedure FRSTPC ( First printable character )
pub fn FRSTPC(STRING: &[u8]) -> i32 {
    let mut FRSTPC: i32 = 0;

    //
    // Local variables
    //

    //
    // Look for the first character in the range [33,126], and return
    // its index.
    //
    for I in 1..=intrinsics::LEN(STRING) {
        if ((intrinsics::ICHAR(fstr::substr(STRING, I..=I)) >= 33)
            && (intrinsics::ICHAR(fstr::substr(STRING, I..=I)) <= 126))
        {
            FRSTPC = I;
            return FRSTPC;
        }
    }

    //
    // Still here? No printable characters. Return zero.
    //
    FRSTPC = 0;

    FRSTPC
}
