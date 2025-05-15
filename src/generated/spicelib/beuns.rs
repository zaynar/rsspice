//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Be an unsigned integer?
///
/// Determine whether a string represents an unsigned integer.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   Character string.
///
///  The function returns .TRUE. if the string represents an unsigned
///  integer. Otherwise, it returns .FALSE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is any string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  If STRING contains a single word made entirely from the
///  characters '0' through '9', then the function returns .TRUE.
///  Otherwise, it returns .FALSE.
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
///  By definition an unsigned integer is a word made exclusively
///  from the characters '0', '1', '2', '3', '4', '5', '6', '7', '8',
///  and '9'.
/// ```
///
/// # Examples
///
/// ```text
///  Four classes of numbers recognized by the various BE functions.
///
///     UNS      unsigned integer
///     INT      integer                (includes INT)
///     DEC      decimal number         (includes UNS, INT)
///     NUM      number                 (includes UNS, INT, NUM)
///
///  The following table illustrates the differences between
///  the classes. (Any number of leading and trailing blanks
///  are acceptable.)
///
///     String                  Accepted by
///     ------------------      ------------------
///     0                       UNS, INT, DEC, NUM
///     21
///     21994217453648
///
///     +0                      INT, DEC, NUM
///     -13
///     +21946
///
///     1.23                    DEC, NUM
///     12.
///     .17
///     +4.1
///     -.25
///
///     2.3e17                  NUM
///     17.D-13275849
///     -.194265E+0004
///
///  Note that the functions don't take the magnitudes of the numbers
///  into account. They may accept numbers that cannot be represented
///  in Fortran variables. (For example, '2.19E999999999999' probably
///  exceeds the maximum floating point number on any machine, but
///  is perfectly acceptable to BENUM.)
///
///  The following strings are not accepted by any of the functions.
///
///     String             Reason
///     ---------------    ----------------------------------------
///     3/4                No implied operations (rational numbers)
///     37+14              No explicit operations
///     E12                Must have mantissa
///     217,346.91         No commas
///     3.14 159 264       No embedded spaces
///     PI                 No special numbers
///     FIVE               No textual numbers
///     CXIV               No roman numerals
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
/// -    SPICELIB Version 1.1.0, 24-NOV-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 01-DEC-1995 (WLT)
/// ```
pub fn beuns(string: &str) -> bool {
    let ret = BEUNS(string.as_bytes());
    ret
}

//$Procedure BEUNS  ( Be an unsigned integer? )
pub fn BEUNS(STRING: &[u8]) -> bool {
    let mut BEUNS: bool = false;
    let mut I: i32 = 0;
    let mut L: i32 = 0;
    let mut OK: bool = false;

    //
    // SPICE functions
    //

    //
    // Local variables
    //

    //
    // Get the length of the string and the position of its
    // first non-blank character.
    //
    L = intrinsics::LEN(STRING);
    I = FRSTNB(STRING);

    //
    // If there isn't a non-blank character, this isn't an
    // unsigned integer.
    //
    if (I == 0) {
        BEUNS = false;
        return BEUNS;
    }

    //
    // As far as we know right now, everything is ok.  Examine
    // characters until we run out of string or until we
    // hit a non-digit character.
    //
    OK = true;

    while (OK && (I <= L)) {
        if (intrinsics::INDEX(b"0123456789", fstr::substr(STRING, I..=I)) > 0) {
            I = (I + 1);
        } else {
            OK = false;
        }
    }

    //
    // If the string still is ok as an unsigned integer, it must be
    // one...
    //
    if OK {
        BEUNS = true;
    } else {
        //
        // ... otherwise, it's an unsigned integer if the remainder is blank.
        //
        BEUNS = fstr::eq(fstr::substr(STRING, I..), b" ");
    }

    BEUNS
}
