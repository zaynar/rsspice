//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Left justify a character string
///
/// Left-justify a character string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  INPUT      I   Input character string.
///  OUTPUT     O   Output character string, left justified.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INPUT    is the input character string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUTPUT   is the output character string, left justified.
///
///           OUTPUT may overwrite INPUT.
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
///  Leading blanks are removed from the input character string.
///  If the output string is not large enough to hold the left
///  justified string, it is truncated on the right.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples illustrate the use of LJUST.
///
///         'ABCDE'             becomes   'ABCDE'
///         'AN EXAMPLE'                  'AN EXAMPLE'
///         '   AN EXAMPLE  '             'AN EXAMPLE'
///         '               '             ' '
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 27-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 29-JUL-2013 (BVS)
///
///         Added the quick return branch for input strings that are
///         already left-justified. Removed the initial check for blank
///         input and changed logic to return an empty string after
///         scanning the input. Re-ordered header sections.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn ljust(input: &str, output: &mut str) {
    LJUST(input.as_bytes(), fstr::StrBytes::new(output).as_mut());
}

//$Procedure LJUST ( Left justify a character string )
pub fn LJUST(INPUT: &[u8], OUTPUT: &mut [u8]) {
    let mut LI: i32 = 0;
    let mut LO: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut POS: i32 = 0;

    //
    // Local variables
    //

    //
    // Is the first character of the input string non-blank? If yes, the
    // input string is already left-justified. There is nothing to do
    // but to assign the input string to the output string.
    //
    if fstr::ne(fstr::substr(INPUT, 1..=1), b" ") {
        fstr::assign(OUTPUT, INPUT);
    } else {
        //
        // Get the first non-blank character. Start OUTPUT at that point.
        //
        LI = intrinsics::LEN(INPUT);
        LO = intrinsics::LEN(OUTPUT);

        J = 1;

        //
        // Set I equal to position of first non-blank character of INPUT.
        //
        I = 0;
        POS = 1;

        while ((I == 0) && (POS <= LI)) {
            if fstr::ne(fstr::substr(INPUT, POS..=POS), b" ") {
                I = POS;
            } else {
                POS = (POS + 1);
            }
        }

        //
        // Did we find a non-blank character? If not, the input string is
        // blank. Set output to blank.
        //
        if (I == 0) {
            fstr::assign(OUTPUT, b" ");
        } else {
            //
            // I is now the index of the first non-blank character of
            // INPUT.
            //
            while ((I <= LI) && (J <= LO)) {
                fstr::assign(fstr::substr_mut(OUTPUT, J..=J), fstr::substr(INPUT, I..=I));
                J = (J + 1);
                I = (I + 1);
            }

            if (J <= LO) {
                fstr::assign(fstr::substr_mut(OUTPUT, J..), b" ");
            }
        }
    }
}
