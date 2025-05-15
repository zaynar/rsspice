//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Enclose in quotes
///
/// Enclose (quote) the non-blank part of a character string
/// between delimiting symbols.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  LEFT       I   Left delimiter.
///  RIGHT      I   Right delimiter.
///  OUT        O   Output (quoted) string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is the input string to be quoted.
///
///  LEFT,
///  RIGHT    are the left and right delimiters to be used in
///           quoting the input string. These may be the same
///           character (apostrophe, vertical bar), complementary
///           characters (left and right parentheses, brackets,
///           or braces), or two totally unrelated characters.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the output string. This is the non-blank part
///           of the input string delimited by LEFT and RIGHT.
///           If the output string is not large enough to contain
///           the quoted string, it is truncated on the right.
///           (The right delimiter would be lost in this case.)
///
///           If the input string is blank, the output string is
///           a single quoted blank.
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
///  The first character of the output string is the left delimiter,
///  LEFT. This is followed immediately by the non-blank part of the
///  input string, which is in turn followed by the right delimiter,
///  RIGHT.
///
///  If the input string is blank (has no non-blank characters),
///  a single quoted blank is returned.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///        IN    = '    This string has leading and trailing blanks  '
///        LEFT  = '('
///        RIGHT = ')'
///
///  Then
///        OUT   = '(This string has leading and trailing blanks)    '
///
///  Or, let IN = '         '. Then OUT = '( )'.
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
pub fn quote(in_: &str, left: char, right: char, out: &mut str) {
    QUOTE(
        in_.as_bytes(),
        &[u8::try_from(left).unwrap()],
        &[u8::try_from(right).unwrap()],
        fstr::StrBytes::new(out).as_mut(),
    );
}

//$Procedure QUOTE ( Enclose in quotes )
pub fn QUOTE(IN: &[u8], LEFT: &[u8], RIGHT: &[u8], OUT: &mut [u8]) {
    let LEFT = &LEFT[..1];
    let RIGHT = &RIGHT[..1];

    //
    // SPICELIB functions
    //

    //
    // Check for blank string first.
    //
    if fstr::eq(IN, b" ") {
        fstr::assign(OUT, LEFT);
        SUFFIX(RIGHT, 1, OUT);
    } else {
        fstr::assign(OUT, fstr::substr(IN, FRSTNB(IN)..=LASTNB(IN)));
        PREFIX(LEFT, 0, OUT);
        SUFFIX(RIGHT, 0, OUT);
    }
}
