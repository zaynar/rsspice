//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Right justify a character string
///
/// Right justify a character string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  INPUT      I   Input character string.
///  OUTPUT     O   Output character string, right justified.
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
///  OUTPUT   is the output character string, right justified.
///           If INPUT is too large to fit into OUTPUT, it is
///           truncated on the left.
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
///  Any trailing blanks in the input string are removed, and
///  the remaining string is copied to the output string.
/// ```
///
/// # Examples
///
/// ```text
///  The following examples should illustrate the use of RJUST.
///
///     'ABCDE          '   becomes  '          ABCDE'
///     'AN EXAMPLE     '            '     AN EXAMPLE'
///     '   AN EXAMPLE  '            '     AN EXAMPLE'
///     '               '            '               '
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
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -     Beta Version 1.1.0, 11-DEC-1989 (IMU)
///
///          Did not work on Sun when INPUT and OUTPUT were
///          the same string, and where the initial and final
///          locations of the non-blank part of the string
///          overlapped.
///
///          The solution is to move the characters one by one,
///          starting from the right side of the input string.
///          That way, nothing gets clobbered.
/// ```
pub fn rjust(input: &str, output: &mut str) {
    RJUST(input.as_bytes(), fstr::StrBytes::new(output).as_mut());
}

//$Procedure RJUST ( Right justify a character string )
pub fn RJUST(INPUT: &[u8], OUTPUT: &mut [u8]) {
    let mut FIRST: i32 = 0;
    let mut LAST: i32 = 0;
    let mut LOC: i32 = 0;
    let mut START: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Blank string? It's all the same.
    //
    if fstr::eq(INPUT, b" ") {
        fstr::assign(OUTPUT, INPUT);

    //
    // Get the first non-blank character. Start OUTPUT at that point.
    //
    } else {
        FIRST = FRSTNB(INPUT);
        LAST = LASTNB(INPUT);
        START = (intrinsics::LEN(OUTPUT) - (LAST - FIRST));

        //
        // If the input string is too long (START < 1), move FIRST
        // up a little to truncate on the left.
        //
        if (START < 1) {
            FIRST = (FIRST + (1 - START));
            START = 1;
        }

        //
        // Move the characters in reverse order, to keep from stomping
        // anything if the operation is being done in place.
        //
        LOC = intrinsics::LEN(OUTPUT);

        for I in intrinsics::range(LAST, FIRST, -1) {
            fstr::assign(
                fstr::substr_mut(OUTPUT, LOC..=LOC),
                fstr::substr(INPUT, I..=I),
            );
            LOC = (LOC - 1);
        }

        //
        // Clear the first part of OUTPUT, if necessary.
        //
        if (START > 1) {
            fstr::assign(fstr::substr_mut(OUTPUT, 1..=(START - 1)), b" ");
        }
    }
}
