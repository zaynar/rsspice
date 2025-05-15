//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MAXORD: i32 = 148;

/// Convert an integer to ordinal text
///
/// Convert an integer to an equivalent written ordinal phrase.
/// For example, convert 121 to 'ONE HUNDRED TWENTY-FIRST'.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  N          I   An integer (less than 10**12 in absolute value).
///  STRING     O   An English string representing the ordinal of N.
/// ```
///
/// # Detailed Input
///
/// ```text
///  N        is an integer (less than 10**12 in absolute value).
///           Moreover, if N is less than zero, -N must be a
///           a legitimate number on the host machine.
///
///           In the context of this routine N represents the
///           ranking of some item within a group.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STRING   is the English ordinal equivalent of N. STRING will
///           contain only upper case letters.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXORD   is one more than the length of the longest ordinal
///           string that can be produced by a call to this routine:
///           One string of maximum length is:
///
///              'NEGATIVE '                                  //
///              'SEVEN HUNDRED SEVENTY-SEVEN BILLION '       //
///              'SEVEN HUNDRED SEVENTY-SEVEN MILLION '       //
///              'SEVEN HUNDRED SEVENTY-SEVEN THOUSAND '      //
///              'SEVEN HUNDRED SEVENTY-SEVENTH'
///
///           It has 147 characters.
///
///           The parameter MAXORD is used to declare a local string
///           of sufficient length to allow the construction of
///           any ordinal string.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the resulting ordinal is longer than the output string,
///      it will be truncated on the right, leaving only the most
///      significant portion of the ordinal.
/// ```
///
/// # Particulars
///
/// ```text
///  This is used primarily for generating error messages. For example,
///  if the third letter or token in a string is in error, it might
///  be desirable to supply a message like the following:
///
///     'The third token of 31-JAN-198$ is not a valid year.'
/// ```
///
/// # Examples
///
/// ```text
///  N           STRING
///  ------      -------------------------------------------
///  -6          NEGATIVE SIXTH
///   1          FIRST
///   2          SECOND
///   3          THIRD
///   4          FOURTH
///   20         TWENTIETH
///   21         TWENTY-FIRST
///   99         NINETY-NINTH
///   82131      EIGHTY-TWO THOUSAND ONE HUNDRED THIRTY-FIRST
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Whatever restrictions apply to INTTXT apply to this routine
///      as well.
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
/// -    SPICELIB Version 1.0.0, 15-AUG-1990 (WLT)
/// ```
pub fn intord(ctx: &mut SpiceContext, n: i32, string: &mut str) {
    INTORD(n, fstr::StrBytes::new(string).as_mut(), ctx.raw_context());
}

//$Procedure INTORD ( Convert an integer to ordinal text )
pub fn INTORD(N: i32, STRING: &mut [u8], ctx: &mut Context) {
    let mut I: i32 = 0;
    let mut LAST: i32 = 0;
    let mut MYSTR = [b' '; MAXORD as usize];

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // First get the English equivalent of the cardinal N.
    //
    fstr::assign(&mut MYSTR, b" ");

    INTTXT(N, &mut MYSTR, ctx);

    LAST = LASTNB(&MYSTR);
    I = LAST;

    //
    // Find the beginning of the last number of MYSTR.
    //
    while ((fstr::ne(fstr::substr(&MYSTR, I..=I), b"-")
        && fstr::ne(fstr::substr(&MYSTR, I..=I), b" "))
        && (I > 1))
    {
        I = (I - 1);
    }

    if (fstr::eq(fstr::substr(&MYSTR, I..=I), b" ") || fstr::eq(fstr::substr(&MYSTR, I..=I), b"-"))
    {
        I = (I + 1);
    }

    //
    // Now convert the last cardinal to an ordinal.
    //
    if fstr::eq(fstr::substr(&MYSTR, I..=LAST), b"ONE") {
        fstr::assign(fstr::substr_mut(&mut MYSTR, I..), b"FIRST");
    } else if fstr::eq(fstr::substr(&MYSTR, I..=LAST), b"TWO") {
        fstr::assign(fstr::substr_mut(&mut MYSTR, I..), b"SECOND");
    } else if fstr::eq(fstr::substr(&MYSTR, I..=LAST), b"THREE") {
        fstr::assign(fstr::substr_mut(&mut MYSTR, I..), b"THIRD");
    } else if fstr::eq(fstr::substr(&MYSTR, I..=LAST), b"FIVE") {
        fstr::assign(fstr::substr_mut(&mut MYSTR, I..), b"FIFTH");
    } else if fstr::eq(fstr::substr(&MYSTR, I..=LAST), b"EIGHT") {
        fstr::assign(fstr::substr_mut(&mut MYSTR, I..), b"EIGHTH");
    } else if fstr::eq(fstr::substr(&MYSTR, I..=LAST), b"NINE") {
        fstr::assign(fstr::substr_mut(&mut MYSTR, I..), b"NINTH");
    } else if fstr::eq(fstr::substr(&MYSTR, I..=LAST), b"TWELVE") {
        fstr::assign(fstr::substr_mut(&mut MYSTR, I..), b"TWELFTH");
    } else if fstr::eq(fstr::substr(&MYSTR, LAST..=LAST), b"Y") {
        fstr::assign(fstr::substr_mut(&mut MYSTR, LAST..), b"IETH");
    } else {
        SUFFIX(b"TH", 0, &mut MYSTR);
    }

    //
    // Now simply put MYSTR into STRING and return.
    //
    fstr::assign(STRING, &MYSTR);
}
