//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const LNGEST: i32 = 9;

struct SaveVars {
    NUMBER: ActualCharArray,
    TENS: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NUMBER = ActualCharArray::new(LNGEST, 1..=19);
        let mut TENS = ActualCharArray::new(LNGEST, 1..=9);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"TEN"),
                Val::C(b"TWENTY"),
                Val::C(b"THIRTY"),
                Val::C(b"FORTY"),
                Val::C(b"FIFTY"),
                Val::C(b"SIXTY"),
                Val::C(b"SEVENTY"),
                Val::C(b"EIGHTY"),
                Val::C(b"NINETY"),
            ]
            .into_iter();
            TENS.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"ONE"),
                Val::C(b"TWO"),
                Val::C(b"THREE"),
                Val::C(b"FOUR"),
                Val::C(b"FIVE"),
                Val::C(b"SIX"),
                Val::C(b"SEVEN"),
                Val::C(b"EIGHT"),
                Val::C(b"NINE"),
                Val::C(b"TEN"),
                Val::C(b"ELEVEN"),
                Val::C(b"TWELVE"),
                Val::C(b"THIRTEEN"),
                Val::C(b"FOURTEEN"),
                Val::C(b"FIFTEEN"),
                Val::C(b"SIXTEEN"),
                Val::C(b"SEVENTEEN"),
                Val::C(b"EIGHTEEN"),
                Val::C(b"NINETEEN"),
            ]
            .into_iter();
            NUMBER
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { NUMBER, TENS }
    }
}

/// Convert an integer to text
///
/// Convert an integer to an equivalent written phrase.
/// For example, convert 121 to 'ONE HUNDRED TWENTY-ONE'.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  N          I   An integer (less than 10**12 in absolute value).
///  STRING     O   An English string representing the cardinal of N.
/// ```
///
/// # Detailed Input
///
/// ```text
///  N        is any integer (less than 10**12 in absolute value).
///           If N is less than 0, -N must be a legitimate number.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STRING   is the English cardinal equivalent of N. STRING will
///           contain only upper case letters.
///
///           The longest possible output string contains 145
///           characters. One such string is:
///
///              'NEGATIVE '                                  //
///              'SEVEN HUNDRED SEVENTY-SEVEN BILLION '       //
///              'SEVEN HUNDRED SEVENTY-SEVEN MILLION '       //
///              'SEVEN HUNDRED SEVENTY-SEVEN THOUSAND '      //
///              'SEVEN HUNDRED SEVENTY-SEVEN'
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the resulting text is longer than the output string,
///      it will be truncated on the right, leaving only the most
///      significant portion of the number.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is used primarily for constructing error messages.
///  For example, an overflow message might look like the following:
///
///     'An excess of seventy-four parameters was detected.'
///
///  A second use might be to write dollar amounts: it's much harder
///  to tamper with a string like
///
///     'Two thousand four hundred seventy-one dollars'
///
///  than with the equivalent string
///
///     '$ 2471.00'
/// ```
///
/// # Examples
///
/// ```text
///  N           STRING
///  ------      ------------------------------------------
///  -43         NEGATIVE FORTY-THREE
///   1          ONE
///   2          TWO
///   3          THREE
///   4          FOUR
///   20         TWENTY
///   21         TWENTY-ONE
///   99         NINETY-NINE
///   82131      EIGHTY-TWO THOUSAND ONE HUNDRED THIRTY-ONE
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that N will always be less than
///      a trillion (10**12) in absolute value.
///
///  2)  In the event that N is less than zero, this routine assumes
///      that -N is a legitimate integer on the host machine.
///
///  3)  This routine assumes that an integer as large as 10**9
///      (one billion) is representable on the host machine.
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
pub fn inttxt(ctx: &mut SpiceContext, n: i32, string: &mut str) {
    INTTXT(n, fstr::StrBytes::new(string).as_mut(), ctx.raw_context());
}

//$Procedure INTTXT ( Convert an integer to text )
pub fn INTTXT(N: i32, STRING: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SUFF = [b' '; LNGEST as usize];
    let mut NUM: i32 = 0;
    let mut PAD: i32 = 0;
    let mut SPACE: i32 = 0;
    let mut X: i32 = 0;
    let mut Y: i32 = 0;

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Zero is easy.
    //
    if (N == 0) {
        fstr::assign(STRING, b"ZERO");
        return;
    }

    //
    // If the number is negative, the string begins with the word
    // `NEGATIVE', and otherwise the number can be treated as though
    // it were positive.
    //
    if (N < 0) {
        NUM = -N;
        fstr::assign(STRING, b"NEGATIVE");
    } else {
        NUM = N;
        fstr::assign(STRING, b" ");
    }

    //
    // Construct the number portion, from left to right: billions,
    // then millions, and so on. In case of overflow, SUFFIX simply
    // leaves the output string unchanged, so there is no need to
    // check explicitly for truncation.
    //
    while (NUM > 0) {
        //
        // Find the right unit (billion, million, or whatever),
        // and the number (X) of those units. X should always
        // be between zero and 999, regardless of the units.
        //
        if (NUM >= 1000000000) {
            X = (NUM / 1000000000);
            fstr::assign(&mut SUFF, b"BILLION");
            NUM = (NUM - (X * 1000000000));
        } else if (NUM >= 1000000) {
            X = (NUM / 1000000);
            fstr::assign(&mut SUFF, b"MILLION");
            NUM = (NUM - (X * 1000000));
        } else if (NUM >= 1000) {
            X = (NUM / 1000);
            fstr::assign(&mut SUFF, b"THOUSAND");
            NUM = (NUM - (X * 1000));
        } else {
            X = NUM;
            fstr::assign(&mut SUFF, b" ");
            NUM = 0;
        }

        //
        // Convert X to text, ...
        //
        SPACE = 1;

        while (X > 0) {
            if fstr::eq(STRING, b" ") {
                PAD = 0;
            } else {
                PAD = 1;
            }

            if (X >= 100) {
                Y = (X / 100);
                X = (X - (Y * 100));

                SUFFIX(&save.NUMBER[Y], PAD, STRING);
                SUFFIX(b"HUNDRED", 1, STRING);
            } else if (X >= 20) {
                Y = (X / 10);
                X = (X - (Y * 10));

                SUFFIX(&save.TENS[Y], PAD, STRING);

                if (X != 0) {
                    SUFFIX(b"-", 0, STRING);
                    SPACE = 0;
                }
            } else {
                Y = X;
                X = 0;

                if fstr::eq(STRING, b" ") {
                    SPACE = 0;
                }

                SUFFIX(&save.NUMBER[Y], SPACE, STRING);
            }
        }

        //
        // ... then add the units. Repeat as necessary.
        //
        SUFFIX(&SUFF, 1, STRING);
    }
}
