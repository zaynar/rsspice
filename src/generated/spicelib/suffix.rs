//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Suffix a character string
///
/// Add a suffix to a character string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SUFF       I   Suffix.
///  SPACES     I   Number of spaces separating prefix and suffix.
///  STRING    I-O  Prefix on input, string on output.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SUFF     is the suffix to be added to the string.
///           Leading blanks are significant. (A blank
///           suffix is interpreted as a null suffix.)
///
///  SPACES   is the number of spaces (blanks) in the output
///           string separating the last non-blank character
///           of the prefix from the first (blank or non-blank)
///           character of the suffix. Typically, this will be
///           zero or one. If not positive, SPACES defaults to
///           zero.
///
///  STRING   on input is the prefix to which the suffix is
///           to be added. Leading blanks are significant.
///           Trailing blanks are ignored.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STRING   on output is the suffixed string. If STRING
///           is not large enough to contain the output string,
///           the output string is truncated on the right.
///
///           STRING may NOT overwrite SUFF.
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
///  The suffix is added to the right of the last non-blank character
///  of the prefix. (Any necessary truncation is done automatically.)
/// ```
///
/// # Examples
///
/// ```text
///  The following examples illustrate the use of SUFFIX.
///
///        SUFF         STRING (input)   SPACES    STRING (output)
///        ----------   --------------   ------    ---------------
///        'abc     '   'def    '             0    'defabc '
///        'abc     '   'def    '             1    'def abc'
///        'abc     '   ' def   '             0    ' defabc'
///        'abc     '   ' def   '             1    ' def ab'
///        ' abc    '   'def    '             0    'def abc'
///        ' abc    '   'def    '             1    'def  ab'
///        ' abc    '   ' def   '            -1    ' def ab'
///        '        '   'def    '             0    'def    '
///        '        '   'def    '             1    'def    '
///        ' abc    '   '       '             0    ' abc   '
///        ' abc    '   '       '             1    '  abc  '
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  SUFF and STRING must be distinct.
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
/// -    SPICELIB Version 1.1.0, 18-MAR-2021 (JDR)
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
pub fn suffix(suff: &str, spaces: i32, string: &mut str) {
    SUFFIX(
        suff.as_bytes(),
        spaces,
        fstr::StrBytes::new(string).as_mut(),
    );
}

//$Procedure SUFFIX (Suffix a character string)
pub fn SUFFIX(SUFF: &[u8], SPACES: i32, STRING: &mut [u8]) {
    let mut L: i32 = 0;
    let mut SLEN: i32 = 0;
    let mut END: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // SLEN is the allocated length of the string. L is the location of
    // the last non-blank character of the prefix.
    //
    SLEN = intrinsics::LEN(STRING);
    L = LASTNB(STRING);

    //
    // Put the suffix at the end of the string. The spaces will fill
    // themselves in.
    //
    END = (L + intrinsics::MAX0(&[SPACES, 0]));

    if (END < SLEN) {
        fstr::assign(fstr::substr_mut(STRING, (END + 1)..), SUFF);
    }
}
