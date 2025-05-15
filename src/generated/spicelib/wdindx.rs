//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Index of a Word Within a String
///
/// Find the index of a word within a string. If the word does not
/// exist as a word within the string, the value zero is returned.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   String of characters, potentially containing words
///  WORD       I   A string of consecutive printable letters.
///
///  The function returns the location of WORD within STRING.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is a string of characters, potentially containing the
///           word.
///
///           Leading and trailing blanks are not significant in
///           STRING.
///
///  WORD     is a string of consecutive printable characters.
///
///           Leading and trailing blanks are not significant in
///           WORD.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the location of WORD within STRING, providing
///  the index of the first letter of WORD within the input STRING. If
///  WORD does not exist or WORD is blank, the function returns zero.
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
///  A word within a string is a substring beginning and ending with a
///  non-blank characters that is delimited by blanks on each end. (A
///  blank is assumed to precede and follow the first and last
///  characters of a string.)
///
///  Given a word, this routine returns the index of the first letter
///  of the first word of STRING that matches the word.
/// ```
///
/// # Examples
///
/// ```text
///  STRING:
///                   1         2         3         4
///  WORD    1234567890123456789012345678901234567890123456    WDINDX
///  ------  ----------------------------------------------    ------
///  'POT'   'PUT THE POTATOES IN THE POT'                     25
///  'TOES'                                                    0
///  'PUT'                                                     1
///  'THE'                                                     5
///  'IN THE'                                                  18
///  'THE PO'                                                  0
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
/// -    SPICELIB Version 1.1.0, 03-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
pub fn wdindx(string: &str, word: &str) -> i32 {
    let ret = WDINDX(string.as_bytes(), word.as_bytes());
    ret
}

//$Procedure WDINDX ( Index of a Word Within a String )
pub fn WDINDX(STRING: &[u8], WORD: &[u8]) -> i32 {
    let mut WDINDX: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut BEGWD: i32 = 0;
    let mut ENDWD: i32 = 0;
    let mut BEGSTR: i32 = 0;
    let mut ENDSTR: i32 = 0;
    let mut STRLEN: i32 = 0;
    let mut WDLEN: i32 = 0;
    let mut BGTOND: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Find the ends of the word and input string.
    //
    ENDSTR = LASTNB(STRING);
    BEGSTR = FRSTNB(STRING);

    ENDWD = LASTNB(WORD);
    BEGWD = FRSTNB(WORD);

    //
    // Get the offset from the beginning of the word to the end of the
    // word, the word length and the string length.
    //
    BGTOND = (ENDWD - BEGWD);
    WDLEN = (1 + BGTOND);
    STRLEN = ((1 + ENDSTR) - BEGSTR);

    //
    // We deal with all of the pathologies first...
    //
    if ((ENDWD < 1) || (STRLEN < WDLEN)) {
        //
        // ... If we got a blank word or a string that is too short, then
        // the index of the word is zero.
        //
        WDINDX = 0;
        return WDINDX;
    } else if (STRLEN == WDLEN) {
        //
        // ... the word and string have the same non-blank length.
        // Either they match up or they don't.  Find out and return.
        //
        if fstr::eq(
            fstr::substr(STRING, BEGSTR..=ENDSTR),
            fstr::substr(WORD, BEGWD..=ENDWD),
        ) {
            WDINDX = BEGSTR;
        } else {
            WDINDX = 0;
        }

        return WDINDX;
    }

    //
    // Ok.  Now we've got a realistic case to deal with.  The string
    // length is longer than the word length.  Check to see if we have a
    // match at the beginning of the string.
    //
    I = BEGSTR;
    J = (I + BGTOND);

    if (fstr::eq(
        fstr::substr(STRING, I..=J),
        fstr::substr(WORD, BEGWD..=ENDWD),
    ) && fstr::eq(fstr::substr(STRING, (J + 1)..=(J + 1)), b" "))
    {
        WDINDX = I;
        return WDINDX;
    }

    //
    // No luck yet?  Search the string until we find a word match or
    // we run out of string to check.
    //
    I = (BEGSTR + 1);
    J = (I + BGTOND);

    while ((J < ENDSTR)
        && !((fstr::eq(
            fstr::substr(STRING, I..=J),
            fstr::substr(WORD, BEGWD..=ENDWD),
        ) && fstr::eq(fstr::substr(STRING, (I - 1)..=(I - 1)), b" "))
            && fstr::eq(fstr::substr(STRING, (J + 1)..=(J + 1)), b" ")))
    {
        I = (I + 1);
        J = (J + 1);
    }

    //
    // If J equals ENDSTR then no match was found in the interior of the
    // string.  We make a last check at the end.
    //
    if (J == ENDSTR) {
        if (fstr::eq(fstr::substr(STRING, (I - 1)..=(I - 1)), b" ")
            && fstr::eq(
                fstr::substr(STRING, I..=J),
                fstr::substr(WORD, BEGWD..=ENDWD),
            ))
        {
            WDINDX = I;
        } else {
            WDINDX = 0;
        }
    } else {
        //
        // The only way to get here is if we exited the above loop before
        // running out of room --- that is we had a word match.  Set
        // the index to the value of "I" that got us out of the loop.
        //
        WDINDX = I;
    }

    WDINDX
}
