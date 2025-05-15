//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BASE: i32 = 10;
const STRLEN: i32 = 80;

struct SaveVars {
    DIGITS: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DIGITS = ActualCharArray::new(1, 0..=9);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"0"),
                Val::C(b"1"),
                Val::C(b"2"),
                Val::C(b"3"),
                Val::C(b"4"),
                Val::C(b"5"),
                Val::C(b"6"),
                Val::C(b"7"),
                Val::C(b"8"),
                Val::C(b"9"),
            ]
            .into_iter();
            DIGITS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { DIGITS }
    }
}

/// Integer to character string
///
/// Convert an integer to an equivalent character string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NUMBER     I   Integer to be converted.
///  STRING     O   Equivalent character string, left justified.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NUMBER   is the integer to be converted into a character string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STRING   is the character string representing the integer NUMBER.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the output character string is not large enough to
///      contain the entire character string produced, the output
///      character string will be truncated on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will convert a signed integer into an equivalent
///  decimal character string representation. The decimal digits of
///  the integer NUMBER are found by repeated applications of
///  "modulus" and division operations.
/// ```
///
/// # Examples
///
/// ```text
///  The following argument values illustrate the use of INTSTR.
///
///      NUMBER        STRING
///      ------------  ---------------------
///       1            '-1'
///      -1            '-1'
///       223          '223'
///      -32           '-32'
///       0            '0'
///       2147483647   '2147483647'   ( Maximum 32 bit integer )
///      -2147483647   '-2147483647'  ( Minimum 32 bit integer + 1 )
///      -2147483647   '-2147483648'  ( Minimum 32 bit integer )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine assumes that all signed integers will fit into a
///      character string with LINLEN or fewer digits. See the
///      parameter LINLEN below for the current value.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.2.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.1.0, 11-MAY-1993 (HAN) (MJS)
///
///         DATA statement came before the SAVE statement. This is
///         a violation of the ANSI Standard. It is now the other way
///         around.
///
/// -    SPICELIB Version 2.0.0, 14-OCT-1992 (KRG)
///
///         The routine was rewritten to fix a bug concerning the minimum
///         representable integer.
///
///         This routine used to negate a negative number before it began
///         generating its digits. This was a bad thing to do, because on
///         many machines the minimum representable integer and the
///         maximum representable integer have the following relationship:
///
///            ABS( minimum integer ) = 1 + ABS( maximum integer ).
///
///         Changing the sign of a negative number before converting it
///         to a character string would cause a program to crash if it
///         were attempting to convert the minimum representable integer
///         into a character string.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 07-DEC-1990 (WLT)
///
///         References to the old name INT2CH were removed and
///         an exception added to that section.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (WLT)
/// ```
pub fn intstr(ctx: &mut SpiceContext, number: i32, string: &mut str) {
    INTSTR(
        number,
        fstr::StrBytes::new(string).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure INTSTR  ( Integer to character string )
pub fn INTSTR(NUMBER: i32, STRING: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TMPSTR = [b' '; STRLEN as usize];
    let mut TMPNUM: i32 = 0;
    let mut RESULT: i32 = 0;
    let mut REMNDR: i32 = 0;
    let mut I: i32 = 0;

    //
    // Local Parameters
    //

    //
    // Local variables
    //

    //
    // Saved values
    //

    //
    // The digits are generated in reverse order, so we fill the
    // character string in reverse order, from `right' to `left',
    // so that the digits are in the correct order when we are
    // done converting the integer. This is to avoid reversing the
    // character string before returning. The output character
    // string is then left justified upon exit.
    //
    // Make a copy of the input so that it will not be modified.
    //
    TMPNUM = NUMBER;
    //
    // Initialize the temporary character buffer used to store the
    // character string as it is generated to blanks.
    //
    fstr::assign(&mut TMPSTR, b" ");
    //
    // We need to do different things for the cases where the number to
    // be converted is positive, negative, or zero. ( Actually, the
    // positive case and the zero case are the same, but since we can
    // test for integer zero exactly it will save a few arithmetic
    // operations if we treat it as a special case. ) The case for a
    // negative number is the only one which truly might cause problems,
    // because ABS(minimum integer) may equal ABS(maximum integer) + 1.
    // For 32 bit numbers, INTMIN = -214748368 and INTMAX = 214748367.
    // You should be able to see the repercussions of this.
    //
    I = (intrinsics::LEN(&TMPSTR) + 1);
    if (TMPNUM < 0) {
        //
        // Collect all of the digits in the string.
        //
        while (TMPNUM != 0) {
            I = (I - 1);
            RESULT = (TMPNUM / BASE);
            REMNDR = ((RESULT * BASE) - TMPNUM);
            TMPNUM = RESULT;

            fstr::assign(
                fstr::substr_mut(&mut TMPSTR, I..=I),
                save.DIGITS.get(REMNDR),
            );
        }
        //
        // Put the minus sign in place.
        //
        I = (I - 1);
        fstr::assign(fstr::substr_mut(&mut TMPSTR, I..=I), b"-");
    } else if (TMPNUM > 0) {
        //
        // Collect all of the digits in the string.
        //
        while (TMPNUM != 0) {
            I = (I - 1);
            RESULT = (TMPNUM / BASE);
            REMNDR = (TMPNUM - (RESULT * BASE));
            TMPNUM = RESULT;

            fstr::assign(
                fstr::substr_mut(&mut TMPSTR, I..=I),
                save.DIGITS.get(REMNDR),
            );
        }
    } else {
        //
        // Treat zero as a special case, because it's easier.
        //
        I = (I - 1);
        fstr::assign(fstr::substr_mut(&mut TMPSTR, I..=I), save.DIGITS.get(0));
    }
    //
    // Set the value of the output string before returning. Let the
    // Fortran string equals deal with the left justification, and the
    // truncation on the right if the string STRING is not long enough
    // to contain all of the characters necessary.
    //
    fstr::assign(STRING, fstr::substr(&TMPSTR, I..=intrinsics::LEN(&TMPSTR)));
}
