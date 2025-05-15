//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Word Count
///
/// Return the number of words in a string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   Input character string.
///
///  The function returns the number of words in the input string
///  STRING.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is the input string to be parsed. It contains some number
///           of words, where a word is any string of consecutive
///           non-blank characters delimited by a blank or by either
///           end of the string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the number of words in the input character
///  string STRING.
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
///  WDCNT, like NTHWD and NEXTWD, is useful primarily for parsing
///  input commands consisting of one or more words, where a word is
///  defined to be any sequence of consecutive non-blank characters
///  delimited by either a blank or by either end of the string.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples illustrate the use of WDCNT.
///
///        WDCNT ( 'Now is the time'  )   = 4
///        WDCNT ( '  for all  '      )   = 2
///        WDCNT ( 'good,men.to_come' )   = 1
///        WDCNT ( ' '                )   = 0
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 08-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 10-JAN-2005 (EDW)
///
///         Added logic to prevent the evaluation of STRING(LOC:LOC)
///         if LOC exceeds the length of string. Functionally, the
///         evaluation had no effect on WDCNT's output, but the NAG
///         F95 compiler flagged the evaluation as an array
///         overrun error. This occurred because given:
///
///             A .AND. B
///
///         NAG evaluates A then B then performs the logical
///         comparison.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn wdcnt(string: &str) -> i32 {
    let ret = WDCNT(string.as_bytes());
    ret
}

//$Procedure WDCNT ( Word Count )
pub fn WDCNT(STRING: &[u8]) -> i32 {
    let mut WDCNT: i32 = 0;
    let mut N: i32 = 0;
    let mut LOC: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut CONT: bool = false;

    //
    // Local variables
    //

    //
    // This is just NTHWD, except that it keeps looking until
    // it finds the last word.
    //

    //
    // Trivial case first.
    //
    if fstr::eq(STRING, b" ") {
        WDCNT = 0;
        return WDCNT;
    } else {
        LENGTH = intrinsics::LEN(STRING);
    }

    //
    // Skip leading blanks.
    //
    LOC = 1;

    while fstr::eq(fstr::substr(STRING, LOC..=LOC), b" ") {
        LOC = (LOC + 1);
    }

    //
    // Keep stepping through STRING, counting words as we go.
    // (The current word is ended whenever a blank is encountered.)
    // Quit when the end of the string is reached.
    //
    // N is the number of words found so far.
    // LOC is the current location in STRING.
    //
    N = 1;

    while (LOC < LENGTH) {
        LOC = (LOC + 1);

        //
        // Blank signals end of the current word.
        //
        if fstr::eq(fstr::substr(STRING, LOC..=LOC), b" ") {
            //
            // Skip ahead to the next word. Ensure no
            // evaluation of STRING(LOC:LOC) when
            // LOC = LENGTH+1.
            //
            CONT = (LOC <= LENGTH);
            if CONT {
                CONT = (CONT && fstr::eq(fstr::substr(STRING, LOC..=LOC), b" "));
            }

            while CONT {
                LOC = (LOC + 1);

                CONT = (LOC <= LENGTH);
                if CONT {
                    CONT = (CONT && fstr::eq(fstr::substr(STRING, LOC..=LOC), b" "));
                }
            }

            //
            // If not at the end of the string, we have another word.
            //
            if (LOC <= LENGTH) {
                N = (N + 1);
            }
        }
    }

    //
    // Return the number of words found.
    //
    WDCNT = N;

    WDCNT
}
