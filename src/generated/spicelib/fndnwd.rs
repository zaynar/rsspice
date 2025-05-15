//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Find the next word after an index
///
/// Find the beginning and end of the first word starting at
/// or after a specified character.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   A string to examine for words.
///  START      I   Position in the string to start looking for words.
///  B          O   String position of first character of the word.
///  E          O   String position of last character of the word.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is a character string that potentially consists of
///           words of text.
///
///  START    is the index of a letter within the string from which
///           to start looking for the next word.
/// ```
///
/// # Detailed Output
///
/// ```text
///  B        is the index of the first letter of the word substring
///           of STRING that begins at or after position START. If
///           there are no such substrings I is returned as 0.
///
///  E        is the index of the last letter of the word substring
///           of STRING that begins at or after position START. If
///           there are no such substrings J is returned as 0.
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
///  Given a character string and location of a character within that
///  string, this routine finds the first full word of the string
///  that starts on or after the specified location.
/// ```
///
/// # Examples
///
/// ```text
///  1         2         3         4         5
///           12345678901234567890123456789012345678901234567890
///  STRING: 'Now is the time for all good men to go home to bed'
///
///  START    I      J
///  -----   ---    ---
///  1        1      3
///  2        5      6
///  3        5      6
///  4        5      6
///  5        5      6
///  6        8      10
///  7        8      10
///  8        8      10
///  9        12     15
///
///  48       48     50
///  49       0      0
///  111      0      0
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
/// -    SPICELIB Version 2.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 15-OCT-1993 (WLT)
///
///         The routine was completely rewritten with a resulting
///         increase in execution speed of between 2000% and 6000%.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -     SPICELIB Version 2.0.0, 15-OCT-1993 (WLT)
///
///          The routine was completely rewritten with a resulting
///          increase in execution speed of between 2000% and 6000%.
///          It was tested against the old version of the routine to
///          ensure that the functionality was exactly duplicated.
/// ```
pub fn fndnwd(string: &str, start: i32, b: &mut i32, e: &mut i32) {
    FNDNWD(string.as_bytes(), start, b, e);
}

//$Procedure FNDNWD ( Find the next word after an index )
pub fn FNDNWD(STRING: &[u8], START: i32, B: &mut i32, E: &mut i32) {
    let mut BLANK: i32 = 0;
    let mut L: i32 = 0;
    let mut N: i32 = 0;
    let mut SIZE: i32 = 0;
    let mut LASTN: bool = false;
    let mut THISB: bool = false;

    //
    // Local Variables
    //

    //
    // Set up neede parameters and check obvious out-of-bound cases.
    //
    BLANK = intrinsics::ICHAR(b" ");
    SIZE = intrinsics::LEN(STRING);

    if (START > SIZE) {
        *B = 0;
        *E = 0;
        return;
    }

    N = intrinsics::MAX0(&[1, START]);
    L = (N - 1);

    if (L <= 0) {
        LASTN = false;
    } else {
        LASTN = (intrinsics::ICHAR(fstr::substr(STRING, L..=L)) != BLANK);
    }

    THISB = (intrinsics::ICHAR(fstr::substr(STRING, N..=N)) == BLANK);

    //
    // Search for the beginning of a word (the last character
    // blank and the current non-blank).
    //
    while (THISB || LASTN) {
        N = (N + 1);

        if (N > SIZE) {
            *B = 0;
            *E = 0;
            return;
        }

        LASTN = !THISB;
        THISB = (intrinsics::ICHAR(fstr::substr(STRING, N..=N)) == BLANK);
    }

    //
    // If we get this far, we found the beginning of the
    // string.  To find the end look for the next blank and
    // back up one.
    //
    *B = N;

    for I in (N + 1)..=SIZE {
        if (intrinsics::ICHAR(fstr::substr(STRING, I..=I)) == BLANK) {
            *E = (I - 1);
            return;
        }
    }

    //
    // If we get this far, the word ends at the end of the
    // string.
    //
    *E = SIZE;
}
