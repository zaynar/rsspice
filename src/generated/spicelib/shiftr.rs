//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Shift right
///
/// Shift the contents of a character string to the right.
/// Characters moved past the end of the input string are
/// lost. Vacant spaces are filled with a specified character.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  NSHIFT     I   Number of times to shift.
///  FILLC      I   Character to fill spaces left vacant.
///  OUT        O   Shifted string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is the input character string.
///
///  NSHIFT   is the number of times the string is to be
///           shifted. If NSHIFT is negative, OUT will be
///           identical to IN.
///
///  FILLC    is the character with which spaces left vacant by
///           the shift are to be filled.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the output string. This is the input string,
///           shifted N times, filled with FILLC.
///
///           OUT may overwrite IN.
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
///  As a string is shifted left or right, the leftmost or
///  rightmost characters of the string disappear (as if pushed
///  off the end of the string). This is .TRUE. regardless of
///  the length of the output string.
///
///  The remaining characters are shifted simultaneously, and
///  the spaces vacated by those characters are filled with a
///  replacement character.
/// ```
///
/// # Examples
///
/// ```text
///  If FILLC = ' '
///
///     'abcde'   shifted left twice becomes     'cde  '
///     'abcde'   shifted right once becomes     ' abcd'
///
///  If FILLC = '.'
///
///     '12345 '  shifted right once becomes     '.12345'
///     'Apple '  shifted left ten times becomes '......'
///
///  Given the declarations
///
///     CHARACTER*3         SHORT
///     CHARACTER*10        LONG
///
///  The calls
///
///     CALL SHIFTR ( 'abcde ', 2, '-', SHORT )
///     CALL SHIFTR ( 'abcde ', 2, '-', LONG  )
///
///  yield the strings
///
///     SHORT = '--a'
///     LONG  = '--abcd    '
///
///  while the calls
///
///     CALL SHIFTL ( 'abcde ', 2, '-', SHORT )
///     CALL SHIFTL ( 'abcde ', 2, '-', LONG  )
///
///  yield the strings
///
///     SHORT = 'cde'
///     LONG  = 'cde ..     '
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
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
/// -    SPICELIB Version 2.0.1, 22-AUG-2001 (EDW)
///
///         Corrected ENDDO to END DO.
///
/// -    SPICELIB Version 2.0.0, 01-SEP-1994 (MJS)
///
///         This version correctly handles negative shifts.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn shiftr(in_: &str, nshift: i32, fillc: char, out: &mut str) {
    SHIFTR(
        in_.as_bytes(),
        nshift,
        &[u8::try_from(fillc).unwrap()],
        fstr::StrBytes::new(out).as_mut(),
    );
}

//$Procedure SHIFTR ( Shift right )
pub fn SHIFTR(IN: &[u8], NSHIFT: i32, FILLC: &[u8], OUT: &mut [u8]) {
    let FILLC = &FILLC[..1];
    let mut INLEN: i32 = 0;
    let mut OUTLEN: i32 = 0;
    let mut N: i32 = 0;
    let mut NSAVE: i32 = 0;
    let mut NFILL: i32 = 0;
    let mut S: i32 = 0;

    //
    // Local variables
    //

    //
    // Get the length of the input, output strings.
    //
    INLEN = intrinsics::LEN(IN);
    OUTLEN = intrinsics::LEN(OUT);

    //
    // If the shift is zero or negative, the string is not changed.
    // If longer than the input string, the entire string is shifted.
    //
    S = intrinsics::MAX0(&[NSHIFT, 0]);
    N = intrinsics::MIN0(&[INLEN, S]);

    //
    // Figure out how many characters in the input string will
    // be saved (will not be shifted off the end of the string,
    // and will fit in the output string), and how many fill
    // characters will be needed (no more than NSHIFT, no fewer
    // than zero).
    //
    NSAVE = ((INLEN - N) - intrinsics::MAX0(&[0, (INLEN - OUTLEN)]));
    NFILL = intrinsics::MIN0(&[N, OUTLEN]);

    //
    // Move the saved characters to output.
    //
    for I in intrinsics::range(NSAVE, 1, -1) {
        fstr::assign(
            fstr::substr_mut(OUT, (I + S)..=(I + S)),
            fstr::substr(IN, I..=I),
        );
    }

    //
    // Add as many fill characters as appropriate.
    //
    for I in 1..=NFILL {
        fstr::assign(fstr::substr_mut(OUT, I..=I), FILLC);
    }

    //
    // Pad the output string with blanks (to cover any previous
    // ugliness there).
    //
    if (OUTLEN > INLEN) {
        fstr::assign(fstr::substr_mut(OUT, (INLEN + 1)..), b" ");
    }
}
