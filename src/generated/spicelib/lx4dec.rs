//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Scan for signed integer
///
/// Scan a string from a specified starting position for the
/// end of a decimal number.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   any character string
///  FIRST      I   first character to scan from in STRING
///  LAST       O   last character that is part of a decimal number
///  NCHAR      O   number of characters in the decimal number.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is any character string.
///
///  FIRST    is the location in the string to beginning scanning
///           for a decimal number. It is assumed that the
///           decimal number begins at FIRST.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LAST     is the last character at or after FIRST such that
///           the substring STRING(FIRST:LAST) is a decimal
///           number. If there is no such substring, LAST
///           will be returned with the value FIRST-1.
///
///  NCHAR    is the number of characters in the decimal number
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
///  2)  If STRING(FIRST:FIRST) is not part of a decimal number then
///      LAST will be returned with the value FIRST-1 and NCHAR will be
///      returned with the value 0.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows you to scan forward in a string to locate
///  a decimal number that begins on the input character FIRST. Note
///  that all signed integers are included in the list of decimal
///  numbers. See LX4SGN for a description of signed integers.
///
///  We let S stand for a signed integer and U stand for
///  an unsigned integer. With this notation, the strings
///  recognized as decimal numbers are:
///
///     U
///     S
///     S.
///     S.U
///      .U
///     -.U
///     +.U
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
///  length and % stands for some non-digit character. You could
///  use this routine to locate the decimal numbers in the
///  string as shown below. We'll keep track of the beginning and
///  ending of the decimal numbers in the integer arrays B and E.
///
///  FIRST = 1
///  I     = 0
///
///  DO WHILE ( FIRST .LT. LEN(STRING) )
///
///     CALL LX4DEC ( STRING, FIRST, LAST, NCHAR )
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
/// -    SPICELIB Version 1.2.0, 04-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Fixed return value for LAST in $Exceptions section entry #1.
///
/// -    SPICELIB Version 1.1.0, 28-NOV-1995 (WLT)
///
///         Upgraded the routine to handle strings of the form
///         '+.01' and '-.01' which were regarded as non-decimal
///         strings before.
///
/// -    SPICELIB Version 1.0.0, 12-JUL-1994 (WLT)
/// ```
pub fn lx4dec(ctx: &mut SpiceContext, string: &str, first: i32, last: &mut i32, nchar: &mut i32) {
    LX4DEC(string.as_bytes(), first, last, nchar, ctx.raw_context());
}

//$Procedure LX4DEC (Scan for signed integer)
pub fn LX4DEC(STRING: &[u8], FIRST: i32, LAST: &mut i32, NCHAR: &mut i32, ctx: &mut Context) {
    let mut F: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut L: i32 = 0;
    let mut N: i32 = 0;
    let mut NEXT: i32 = 0;

    *LAST = (FIRST - 1);
    NEXT = (FIRST + 1);
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
    // LX4SGN or LX4UNS do  almost all of the work).
    //
    I = intrinsics::ICHAR(fstr::substr(STRING, FIRST..=FIRST));

    if (NEXT < L) {
        J = intrinsics::ICHAR(fstr::substr(STRING, NEXT..=NEXT));
    } else {
        J = intrinsics::ICHAR(b" ");
    }

    if (I == intrinsics::ICHAR(b".")) {
        //
        // Case 1. The string begins with a decimal point.
        // There must be an unsigned integer following.
        //
        F = (FIRST + 1);

        LX4UNS(STRING, F, LAST, NCHAR, ctx);

        if (*NCHAR == 0) {
            *LAST = (FIRST - 1);
        } else {
            *NCHAR = (*NCHAR + 1);
        }
    } else if (((I == intrinsics::ICHAR(b"-")) || (I == intrinsics::ICHAR(b"+")))
        && (J == intrinsics::ICHAR(b".")))
    {
        //
        // Case 2. The string begins with a sign followed by
        // a decimal point. There must be an unsigned integer following.
        //
        F = (NEXT + 1);

        LX4UNS(STRING, F, LAST, NCHAR, ctx);

        if (*NCHAR == 0) {
            *LAST = (FIRST - 1);
        } else {
            *NCHAR = (*NCHAR + 1);
        }
    } else if ((I == intrinsics::ICHAR(b"+")) && (J == intrinsics::ICHAR(b"."))) {
        //
        // Case 2. The string begins with a minus sign followed by
        // a decimal point. There must be an unsigned integer following.
        //
        F = (NEXT + 1);

        LX4UNS(STRING, F, LAST, NCHAR, ctx);

        if (*NCHAR == 0) {
            *LAST = (FIRST - 1);
        } else {
            *NCHAR = (*NCHAR + 1);
        }
    } else {
        //
        // Case 3.  The leading character is not a decimal point.
        // First check to see how much signed integer we have.
        //
        LX4SGN(STRING, FIRST, LAST, NCHAR, ctx);
        //
        // If we got some part of a signed integer, we next see
        // if there is a decimal point followed by an unsigned
        // integer.
        //
        if ((*NCHAR > 0) && (*LAST < L)) {
            F = (*LAST + 1);
            I = intrinsics::ICHAR(fstr::substr(STRING, F..=F));

            if (I == intrinsics::ICHAR(b".")) {
                *LAST = F;
                F = (*LAST + 1);
                //
                // After the decimal point we may have an unsigned integer.
                //
                LX4UNS(STRING, F, LAST, &mut N, ctx);
                //
                // LAST is either pointing to the decimal point or the
                // end of an unsigned integer.  In either case we need
                // to update NCHAR.
                //
                *NCHAR = ((*LAST + 1) - FIRST);
            }
        }
    }
}
