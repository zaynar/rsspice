//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Replace characters in a string
///
/// Replace all occurrences of a single character with a second
/// character.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  INSTR      I   Input string.
///  OLD        I   Character to be replaced.
///  NEW        I   Replacement character.
///  OUTSTR     O   Output string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INSTR    is the input character string, possibly containing
///           one or more occurrences of the character OLD.
///
///  OLD      is the character to be replaced wherever it occurs in
///           the input string.
///
///  NEW      is the character which is to replace each occurrence
///           of the character OLD in the output string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUTSTR   is the output string. This is the input string
///           with every occurrence of the character OLD replaced
///           by the character NEW.
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
///  Copy the contents of the input string to the output string
///  a character at a time, replacing each occurrence of OLD with NEW.
///  If the output string is not long enough to contain the input
///  string, it is truncated on the right.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///        INSTR  = 'Woodsy is the Anti-Pollution Owl.'
///        OLD    = 'O'
///        NEW    = 'E'
///  then
///        OUTSTR = 'Woodsy is the Anti-Pollution Ewl.'
///
///  Note the case-sensitivity of REPLCH. The lowercase o's are
///  not affected.
///
///  REPLCH may similarly be used to replace control characters
///  (such as tab stops, line feeds, and nulls) with regular ASCII
///  characters (such as blanks).
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  REPLCH is sensitive to case, as shown in the examples above.
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
pub fn replch(instr: &str, old: char, new: char, outstr: &mut str) {
    REPLCH(
        instr.as_bytes(),
        &[u8::try_from(old).unwrap()],
        &[u8::try_from(new).unwrap()],
        fstr::StrBytes::new(outstr).as_mut(),
    );
}

//$Procedure REPLCH ( Replace characters in a string )
pub fn REPLCH(INSTR: &[u8], OLD: &[u8], NEW: &[u8], OUTSTR: &mut [u8]) {
    let OLD = &OLD[..1];
    let NEW = &NEW[..1];

    //
    // Local Variables
    //

    //
    // Move the input string to the output string. If it's too long,
    // this will truncate it.
    //
    fstr::assign(OUTSTR, INSTR);

    //
    // Check each character of OUTSTR and replace as necessary.
    //
    for I in 1..=intrinsics::LEN(OUTSTR) {
        if fstr::eq(fstr::substr(OUTSTR, I..=I), OLD) {
            fstr::assign(fstr::substr_mut(OUTSTR, I..=I), NEW);
        }
    }
}
