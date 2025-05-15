//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Be an Integer?
///
/// Determine whether a string represents an integer.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   Character string.
///
///  The function returns .TRUE. if the string represents an integer.
///  Otherwise, it returns .FALSE.
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
///  If the input string contains an integer (as defined in
///  $Particulars below), the function returns .TRUE. Otherwise,
///  the function returns .FALSE.
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
///  An integer may be either of the following:
///
///     1) An unsigned integer (as defined by function BEUNS).
///
///     2) A sign ('+' or '-') followed by an unsigned
///        integer.
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
pub fn beint(string: &str) -> bool {
    let ret = BEINT(string.as_bytes());
    ret
}

//$Procedure BEINT  ( Be an Integer? )
pub fn BEINT(STRING: &[u8]) -> bool {
    let mut BEINT: bool = false;
    let mut I: i32 = 0;
    let mut L: i32 = 0;
    let mut LETTER = [b' '; 1 as usize];

    //
    // Find the first non-blank character and the length of the
    // string.
    //
    L = intrinsics::LEN(STRING);
    I = FRSTNB(STRING);

    //
    // If there isn't a non-blank character, this isn't an
    // integer.
    //
    if (I == 0) {
        BEINT = false;
        return BEINT;
    }

    //
    // Copy the first non-blank letter in the string.
    //
    fstr::assign(&mut LETTER, fstr::substr(STRING, I..=I));

    if (I < L) {
        //
        // The first character is not the last, so we might start with
        // a plus or minus.  If so the rest must be an unsigned integer.
        //
        if (fstr::eq(&LETTER, b"+") || fstr::eq(&LETTER, b"-")) {
            I = (I + 1);

            if fstr::ne(fstr::substr(STRING, I..=I), b" ") {
                BEINT = BEUNS(fstr::substr(STRING, I..));
            } else {
                BEINT = false;
            }
        } else {
            //
            // If the first character isn't plus (+) or minus (-)
            // the string must be an unsigned integer if its going
            // to be an integer.
            //
            BEINT = BEUNS(fstr::substr(STRING, I..));
        }
    } else {
        //
        // If the first (non-blank) character is the last one, then
        // it must be an unsigned integer, for the string to
        // represent an integer.
        //
        BEINT = BEUNS(&LETTER);
    }

    BEINT
}
