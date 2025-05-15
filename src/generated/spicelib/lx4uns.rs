//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXC: i32 = 255;
const MINC: i32 = -128;

struct SaveVars {
    L: i32,
    DIGIT: ActualArray<bool>,
    DOINIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut L: i32 = 0;
        let mut DIGIT = ActualArray::<bool>::new(MINC..=MAXC);
        let mut DOINIT: bool = false;

        DOINIT = true;

        Self { L, DIGIT, DOINIT }
    }
}

/// Scan for unsigned integer
///
/// Scan a string from a specified starting position for the
/// end of an unsigned integer.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   any character string
///  FIRST      I   first character to scan from in STRING
///  LAST       O   last character that is part of an unsigned integer
///  NCHAR      O   number of characters in the unsigned integer.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is any character string.
///
///  FIRST    is the location in the string to beginning scanning
///           for an unsigned integer. It is assumed that the
///           unsigned integer begins at FIRST.
/// ```
///
/// # Detailed Output
///
/// ```text
///  LAST     is the last character at or after FIRST such that
///           the substring STRING(FIRST:LAST) is an unsigned
///           integer. If there is no such substring, LAST
///           will be returned with the value FIRST-1.
///
///  NCHAR    is the number of characters in the unsigned integer
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
///  2)  If STRING(FIRST:FIRST) is not part of an unsigned integer then
///      LAST will be returned with the value FIRST-1 and NCHAR will be
///      returned with the value 0.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows you to scan forward in a string to locate
///  an unsigned integer that begins on the input character FIRST.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you believe that a string has the form
///
///     X%Y%Z
///
///  where X, Y, and Z are unsigned integers of some unknown
///  length and % stands for some non-digit character. You could
///  use this routine to locate the unsigned integers in the
///  string as shown below. We'll keep track of the beginning and
///  ending of the unsigned integers in the integer arrays B and E.
///
///  FIRST = 1
///  I     = 0
///
///  DO WHILE ( FIRST .LT. LEN(STRING) )
///
///     CALL LX4UNS ( STRING, FIRST, LAST, NCHAR )
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
/// # Restrictions
///
/// ```text
///  1)  Assumes ICHAR returns values in the range [-128, 255].
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
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
/// -    SPICELIB Version 1.1.0, 03-DEC-2001 (NJB)
///
///         Updated to work if non-printing characters are present in
///         the input string. Updated $Restrictions section.
///
/// -    SPICELIB Version 1.0.0, 12-JUL-1994 (WLT)
/// ```
pub fn lx4uns(ctx: &mut SpiceContext, string: &str, first: i32, last: &mut i32, nchar: &mut i32) {
    LX4UNS(string.as_bytes(), first, last, nchar, ctx.raw_context());
}

//$Procedure LX4UNS (Scan for unsigned integer)
pub fn LX4UNS(STRING: &[u8], FIRST: i32, LAST: &mut i32, NCHAR: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // First we perform some initializations that are needed on
    // each pass through this routine.
    //
    if save.DOINIT {
        save.DOINIT = false;

        for I in MINC..=MAXC {
            save.DIGIT[I] = false;
        }

        save.DIGIT[intrinsics::ICHAR(b"0")] = true;
        save.DIGIT[intrinsics::ICHAR(b"1")] = true;
        save.DIGIT[intrinsics::ICHAR(b"2")] = true;
        save.DIGIT[intrinsics::ICHAR(b"3")] = true;
        save.DIGIT[intrinsics::ICHAR(b"4")] = true;
        save.DIGIT[intrinsics::ICHAR(b"5")] = true;
        save.DIGIT[intrinsics::ICHAR(b"6")] = true;
        save.DIGIT[intrinsics::ICHAR(b"7")] = true;
        save.DIGIT[intrinsics::ICHAR(b"8")] = true;
        save.DIGIT[intrinsics::ICHAR(b"9")] = true;
    }

    *LAST = (FIRST - 1);
    save.L = intrinsics::LEN(STRING);

    //
    // If start is beyond the ends of the string, we  can quit now.
    //
    if ((FIRST < 1) || (FIRST > save.L)) {
        *NCHAR = 0;
        return;
    }

    //
    // Now for the real work of the routine. Examine characters one
    // at a time...
    //
    for I in FIRST..=save.L {
        //
        // If this character is a digit, move the LAST pointer one
        // further down on the string.  Otherwise set NCHAR and return.
        //
        if save.DIGIT[intrinsics::ICHAR(fstr::substr(STRING, I..=I))] {
            *LAST = (*LAST + 1);
        } else {
            *NCHAR = ((*LAST + 1) - FIRST);
            return;
        }
    }

    *NCHAR = ((*LAST + 1) - FIRST);
}
