//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Be a number?
///
/// Determine whether a string represents a number.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   Character string.
///
///  The function returns .TRUE. if the string is a number.
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
///  If the input string contains a number (as defined in
///  $Particulars below) the function returns .TRUE. Otherwise,
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
///  A number may be either of the following:
///
///     1) A decimal number (as defined by function BEDEC).
///
///     2) A decimal number followed by an exponent character
///        ('E', 'e', 'D', or 'd') and an integer (as defined
///        by function BEINT).
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
pub fn benum(string: &str) -> bool {
    let ret = BENUM(string.as_bytes());
    ret
}

//$Procedure BENUM  ( Be a number? )
pub fn BENUM(STRING: &[u8]) -> bool {
    let mut BENUM: bool = false;
    let mut D: i32 = 0;
    let mut E: i32 = 0;
    let mut F: i32 = 0;
    let mut L: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Determine whether or not there is an exponent character in the
    // string.
    //
    L = intrinsics::LEN(STRING);
    E = CPOS(STRING, b"EeDd", 1);
    D = (E - 1);
    F = (E + 1);

    if (E == 0) {
        //
        // There is no exponent character, this is a number if it
        // is a decimal number.
        //
        BENUM = BEDEC(STRING);
    } else if ((E == 1) || (E == L)) {
        BENUM = false;
    } else if (fstr::eq(fstr::substr(STRING, D..=D), b" ")
        || fstr::eq(fstr::substr(STRING, F..=F), b" "))
    {
        BENUM = false;
    } else {
        BENUM = (BEDEC(fstr::substr(STRING, 1..=D)) && BEINT(fstr::substr(STRING, F..=L)));
    }

    BENUM
}
