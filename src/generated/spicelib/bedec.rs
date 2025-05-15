//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Be a decimal number?
///
/// Determine whether a string represents a decimal number.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   Character string.
///
///  The function returns .TRUE. if the string represents a decimal
///  number. Otherwise, it returns .FALSE.
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
///  If the input string contains a decimal number (as defined
///  in $Particulars below), the function returns .TRUE. Otherwise,
///  the functions returns .FALSE.
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
///  A decimal number may be constructed by concatenating
///  the following components in the order shown.
///
///     1) A sign ('+' or '-'), or the null string.
///
///     2) An unsigned integer (as defined by function BEUNS),
///        or the null string.
///
///     3) A decimal point, or the null string.
///
///     4) An unsigned integer, or the null string.
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
pub fn bedec(string: &str) -> bool {
    let ret = BEDEC(string.as_bytes());
    ret
}

//$Procedure BEDEC  ( Be a decimal number? )
pub fn BEDEC(STRING: &[u8]) -> bool {
    let mut BEDEC: bool = false;
    let mut C: i32 = 0;
    let mut D: i32 = 0;
    let mut E: i32 = 0;
    let mut L: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // First determine whether or not a decimal point is present.
    //
    D = POS(STRING, b".", 1);
    C = (D - 1);
    E = (D + 1);

    if (D == 0) {
        //
        // If there is no decimal point just apply the integer test.
        //
        BEDEC = BEINT(STRING);
    } else {
        //
        // A decimal point is present, get the length of the string
        // and see where the decimal point is relative to the last
        // character.
        //
        L = intrinsics::LEN(STRING);

        if (L == 1) {
            //
            // The string is one character long and a decimal point.
            // Sorry, this is not a decimal number.
            //
            BEDEC = false;
        } else if (D == 1) {
            //
            // The decimal point occurs as the first character of the
            // string.  The string following it must begin with
            // a non-blank character and be an unsigned integer.
            //
            BEDEC =
                (fstr::ne(fstr::substr(STRING, E..=E), b" ") && BEUNS(fstr::substr(STRING, E..)));
        } else if (D == L) {
            //
            // The decimal point is the last character of the string.
            // The character that precedes it must be non-blank and
            // the substring to the left must be an integer.
            //
            BEDEC =
                (fstr::ne(fstr::substr(STRING, C..=C), b" ") && BEINT(fstr::substr(STRING, 1..=C)));
        } else if fstr::eq(fstr::substr(STRING, C..=C), b" ") {
            //
            // The decimal point occurs somewhere in the middle of the
            // string and the character preceding it is blank.
            //
            BEDEC = ((fstr::ne(fstr::substr(STRING, E..=E), b" ")
                && fstr::eq(fstr::substr(STRING, 1..=C), b" "))
                && BEUNS(fstr::substr(STRING, E..)));
        } else if fstr::eq(fstr::substr(STRING, E..=E), b" ") {
            //
            // Again the decimal point occurs somewhere in the middle of
            // the string and the character following it is blank.
            //
            BEDEC = ((fstr::eq(fstr::substr(STRING, E..=L), b" ")
                && fstr::ne(fstr::substr(STRING, C..=C), b" "))
                && BEINT(fstr::substr(STRING, 1..=C)));
        } else if (fstr::eq(fstr::substr(STRING, C..=C), b"-")
            || fstr::eq(fstr::substr(STRING, C..=C), b"+"))
        {
            //
            // The decimal point is in the middle of the string and
            // is preceded by a '+' or '-'.  There should be nothing
            // preceding the sign and what follows the decimal point
            // should be an unsigned integer. (we already know that the
            // character following the decimal point is not a blank)
            //
            if (C == 1) {
                BEDEC = BEUNS(fstr::substr(STRING, E..=L));
            } else {
                BEDEC = (BEUNS(fstr::substr(STRING, E..=L))
                    && fstr::eq(fstr::substr(STRING, 1..=(C - 1)), b" "));
            }
        } else {
            //
            // Last chance, the decimal point is in the middle of the
            // string.  The characters to the right and left of the
            // point are non-blank and we know the character to the
            // left of the point is not a sign.  The string left must
            // be an integer, the string to the right must be an
            // unsigned integer.
            //
            BEDEC = (BEINT(fstr::substr(STRING, 1..=C)) && BEUNS(fstr::substr(STRING, E..=L)));
        }
    }

    BEDEC
}
