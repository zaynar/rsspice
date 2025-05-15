//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// STRIP Ascii characters from a string
///
/// Remove from a character string all characters which fall
/// between specified starting and ending characters, inclusive.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  INSTR      I   Input string.
///  ASCIIB     I   First ASCII character in range to be stripped.
///  ASCIIE     I   Last ASCII character in range to be stripped.
///  OUTSTR     O   Output (stripped) string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INSTR    is a character string from which all characters
///           between ASCIIB and ASCIIE, inclusive, are to be
///           removed.
///
///  ASCIIB   is the first ASCII character in the range of
///           characters to be removed from the input string.
///           ASCIIB is itself removed from the string, if
///           it occurs.
///
///  ASCIIE   is the last ASCII character in the range of
///           characters to be removed from the input string.
///           ASCIIE is itself removed from the string, if
///           it occurs.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUTSTR   is the input string after all the character
///           between ASCIIB and ASCIIE, inclusive, have
///           been removed.
///
///           If OUTSTR is not large enough to hold the output
///           string, it is truncated on the right.
///
///           OUTSTR may overwrite INSTR.
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
///  ASTRIP checks each character
///  in INSTR to determine if it falls between the characters ASCIIB
///  and ASCIIE. If so this character is removed from the string
///  (and the string is shortened). Remaining characters are copied
///  to the output string.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples illustrate the use of ASTRIP.
///
///        ASCIIB = 'b'
///        ASCIIE = 'k'
///        INSTR  = 'Now is the time for all good men to come quick.'
///        OUTSTR = 'Now s t tm or all oo mn to om qu.'
///
///        ASCIIB = 'a'
///        ASCIIE = 'z'
///        INSTR  = 'SELECT column TIME FROM table TEST'
///        OUTSTR = 'SELECT TIME FROM TEST'
///
///        ASCIIB = 'a'
///        ASCIIE = 'z'
///        INSTR  = 'this is going to be an empty string'
///        OUTSTR = ' '
///
///        ASCIIB = '!'
///        ASCIIE = '!'
///        INSTR  = 'Only 32 more shopping days until Christmas!'
///        OUTSTR = 'Only 32 more shopping days until Christmas'
///
///  ASTRIP may also be used to strip ASCII control characters
///  (line feeds, tab stops, and so on), as shown in the example
///  below.
///
///        ASCIIB = CHAR ( 0  )
///        ASCIIE = CHAR ( 31 )
///        CALL ASTRIP ( STRING, ASCIIB, ASCIIE, STRING )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If ASCIIB and ASCIIE are not properly ordered (that is,
///      if ICHAR(ASCIIB) is not less than or equal to ICHAR(ASCIIE))
///      then ASTRIP will not function as described. (In fact, it will
///      copy the input string to the output string without change.)
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn astrip(instr: &str, asciib: char, asciie: char, outstr: &mut str) {
    ASTRIP(
        instr.as_bytes(),
        &[u8::try_from(asciib).unwrap()],
        &[u8::try_from(asciie).unwrap()],
        fstr::StrBytes::new(outstr).as_mut(),
    );
}

//$Procedure ASTRIP ( STRIP Ascii characters from a string )
pub fn ASTRIP(INSTR: &[u8], ASCIIB: &[u8], ASCIIE: &[u8], OUTSTR: &mut [u8]) {
    let ASCIIB = &ASCIIB[..1];
    let ASCIIE = &ASCIIE[..1];
    let mut OUTLEN: i32 = 0;
    let mut LAST: i32 = 0;
    let mut LWRBND: i32 = 0;
    let mut UPRBND: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Find the length of the output string. We don't want to
    // exceed it.
    //
    OUTLEN = intrinsics::LEN(OUTSTR);

    //
    // Find the last non-blank character of the input string.
    //
    LAST = LASTNB(INSTR);

    //
    // Get the numeric representation of ASCIIB and ASCIIE.
    //
    LWRBND = intrinsics::ICHAR(ASCIIB);
    UPRBND = intrinsics::ICHAR(ASCIIE);

    //
    // Step through INSTR (I) a character at a time, transferring
    // characters to OUTSTR (J) whenever they fall outside the range
    // [ASCIIB, ASCIIE].
    //
    // If the end of OUTSTR is reached, stop transferring characters
    // and return.
    //
    J = 0;

    for I in 1..=LAST {
        K = intrinsics::ICHAR(fstr::substr(INSTR, I..=I));

        if ((K < LWRBND) || (K > UPRBND)) {
            //
            // The character is kept.  Note that if the user inputs
            // ASCIIB and ASCIIE in the wrong order this test will
            // always succeed so that the output string will be
            // the same as the input string.
            //
            J = (J + 1);
            fstr::assign(fstr::substr_mut(OUTSTR, J..=J), fstr::substr(INSTR, I..=I));

            if (J == OUTLEN) {
                return;
            }
        }
    }

    //
    // Pad the output string with blanks.
    //
    if (J < OUTLEN) {
        fstr::assign(fstr::substr_mut(OUTSTR, (J + 1)..), b" ");
    }
}
