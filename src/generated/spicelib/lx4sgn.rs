//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Scan for signed integer
///
/// Scan a string from a specified starting position for the
/// end of a signed integer.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   any character string
///  FIRST      I   first character to scan from in STRING
///  LAST       O   last character that is part of a signed integer
///  NCHAR      O   number of characters in the signed integer.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is any character string.
///
///  FIRST    is the location in the string to beginning scanning
///           for a signed integer. It is assumed that the
///           signed integer begins at FIRST.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LAST     is the last character at or after FIRST such that
///           the substring STRING(FIRST:LAST) is a signed
///           integer. If there is no such substring, LAST
///           will be returned with the value FIRST-1.
///
///  NCHAR    is the number of characters in the signed integer
///           that begins at FIRST and ends at last. If there
///           is no such string NCHAR will be given the value 0.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If FIRST is beyond either end of the string, then LAST will be
///      returned with the value FIRST-1 and NCHAR will be returned
///      with the value 0.
///
///  2)  If STRING(FIRST:FIRST) is not part of a signed integer
///      then LAST will be returned with the value FIRST-1 and NCHAR
///      will be returned with the value 0.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows you to scan forward in a string to locate
///  a signed integer that begins on the input character FIRST. Note
///  that all unsigned integers are included in the list of signed
///  integers. The signed integers may in addition have a leading
///  plus ('+') or minus ('-') sign.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you believe that a string has the form
///
///     X%Y%Z
///
///  where X, Y, and Z are signed integers of some unknown
///  length and % stands for some non-digit character. You could
///  use this routine to locate the signed integers in the
///  string as shown below. We'll keep track of the beginning and
///  ending of the signed integers in the integer arrays B and E.
///
///  FIRST = 1
///  I     = 0
///
///  DO WHILE ( FIRST .LT. LEN(STRING) )
///
///     CALL LX4SGN ( STRING, FIRST, LAST, NCHAR )
///
///     IF ( NCHAR .GT. 0 ) THEN
///
///        I     = I    + 1
///        B(I)  = FIRST
///        E(I)  = LAST
///        FIRST = LAST + 2
///
///     ELSE
///
///        FIRST = FIRST + 1
///
///     END IF
///
///  END DO
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
/// -    SPICELIB Version 1.1.0, 04-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Fixed return value for LAST in $Exceptions section entry #1.
///
/// -    SPICELIB Version 1.0.0, 12-JUL-1994 (WLT)
/// ```
pub fn lx4sgn(ctx: &mut SpiceContext, string: &str, first: i32, last: &mut i32, nchar: &mut i32) {
    LX4SGN(string.as_bytes(), first, last, nchar, ctx.raw_context());
}

//$Procedure LX4SGN (Scan for signed integer)
pub fn LX4SGN(STRING: &[u8], FIRST: i32, LAST: &mut i32, NCHAR: &mut i32, ctx: &mut Context) {
    let mut F: i32 = 0;
    let mut I: i32 = 0;
    let mut L: i32 = 0;

    *LAST = (FIRST - 1);
    L = intrinsics::LEN(STRING);
    //
    // If start is beyond the ends of the string, we  can quit now.
    //
    if ((FIRST < 1) || (FIRST > L)) {
        *NCHAR = 0;
        return;
    }
    //
    // There are two cases to take care of (and in both cases
    // LX4UNS does almost all of the work).
    //
    I = intrinsics::ICHAR(fstr::substr(STRING, FIRST..=FIRST));

    if ((I == intrinsics::ICHAR(b"+")) || (I == intrinsics::ICHAR(b"-"))) {
        //
        // Case 1. The string begins with a + or -.  There must
        // be an unsigned integer following.
        //
        F = (FIRST + 1);

        LX4UNS(STRING, F, LAST, NCHAR, ctx);

        if (*NCHAR == 0) {
            *LAST = (FIRST - 1);
        } else {
            *NCHAR = (*NCHAR + 1);
        }
    } else {
        //
        // Case 2.  The leading character is not a sign character.
        // We simply check to see how much unsigned integer we have.
        //
        LX4UNS(STRING, FIRST, LAST, NCHAR, ctx);
    }
}
