//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Scan for a number
///
/// Scan a string from a specified starting position for the
/// end of a number.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   any character string
///  FIRST      I   first character to scan from in STRING
///  LAST       O   last character that is part of a number
///  NCHAR      O   number of characters in the number.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is any character string.
///
///  FIRST    is the location in the string to beginning scanning
///           for a  number. It is assumed that the number begins
///           at FIRST.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LAST     is the last character at or after FIRST such that
///           the substring STRING(FIRST:LAST) is a number.
///           If there is no such substring, LAST will be returned
///           with the value FIRST-1.
///
///  NCHAR    is the number of characters in the number
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
///  2)  If STRING(FIRST:FIRST) is not part of a number
///      then LAST will be returned with the value FIRST-1 and NCHAR
///      will be returned with the value 0.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows you to scan forward in a string to locate
///  a number that begins on the input character FIRST. Note
///  that all decimal numbers are included in the list of numbers.
///  The main difference between decimal numbers and numbers is that
///  numbers may have an exponential expression attached (i.e. the
///  exponent character 'e','E','d' or 'D' followed by an signed
///  integer).
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you believe that a string has the form
///
///     X%Y%Z
///
///  where X, Y, and Z are decimal numbers of some unknown
///  length and % stands for some non-numeric character. You could
///  use this routine to locate the numbers in the
///  string as shown below. We'll keep track of the beginning and
///  ending of the numbers in the integer arrays B and E.
///
///  FIRST = 1
///  I     = 0
///
///  DO WHILE ( FIRST .LT. LEN(STRING) )
///
///     CALL LX4NUM ( STRING, FIRST, LAST, NCHAR )
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
pub fn lx4num(ctx: &mut SpiceContext, string: &str, first: i32, last: &mut i32, nchar: &mut i32) {
    LX4NUM(string.as_bytes(), first, last, nchar, ctx.raw_context());
}

//$Procedure LX4NUM (Scan for a number)
pub fn LX4NUM(STRING: &[u8], FIRST: i32, LAST: &mut i32, NCHAR: &mut i32, ctx: &mut Context) {
    let mut F: i32 = 0;
    let mut I: i32 = 0;
    let mut L: i32 = 0;
    let mut N: i32 = 0;
    let mut TEMP: i32 = 0;

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
    // If this is a number, it must begin with a decimal number
    // substring.
    //
    LX4DEC(STRING, FIRST, LAST, NCHAR, ctx);

    if ((*NCHAR > 0) && (*LAST < L)) {
        F = (*LAST + 1);
        I = intrinsics::ICHAR(fstr::substr(STRING, F..=F));
        //
        // See if we have an exponent.
        //
        if ((((I == intrinsics::ICHAR(b"e")) || (I == intrinsics::ICHAR(b"E")))
            || (I == intrinsics::ICHAR(b"D")))
            || (I == intrinsics::ICHAR(b"d")))
        {
            //
            // Starting after the exponent character see
            // if we have a signed integer.
            //
            F = (F + 1);

            LX4SGN(STRING, F, &mut TEMP, &mut N, ctx);
            //
            // If there was a signed integer, N will be bigger than
            // zero and TEMP will point to the last character of
            // the number.  Otherwise we just fall through and leave
            // LAST and NCHAR alone.
            //
            if (N > 0) {
                *LAST = TEMP;
                *NCHAR = ((*LAST + 1) - FIRST);
            }
        }
    }
}
