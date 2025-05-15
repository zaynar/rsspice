//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BASE: i32 = 16;
const MAXDIG: i32 = 16;
const STRLEN: i32 = 255;

struct SaveVars {
    DIGITS: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DIGITS = ActualCharArray::new(1, 0..=(MAXDIG - 1));

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
                Val::C(b"A"),
                Val::C(b"B"),
                Val::C(b"C"),
                Val::C(b"D"),
                Val::C(b"E"),
                Val::C(b"F"),
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

/// Integer to signed hexadecimal string
///
/// Convert an integer to an equivalent signed hexadecimal string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NUMBER     I   Integer to be converted.
///  STRING     O   Equivalent hexadecimal string, left justified.
///  LENGTH     O   The length of the hexadecimal string produced.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NUMBER   is the integer to be converted.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STRING   is the signed hexadecimal string representing the integer
///           NUMBER.
///
///           The following table describes the character set used
///           to represent the hexadecimal digits and their
///           corresponding values.
///
///           Character    Value           Character    Value
///           ---------    -----           ---------    -----
///             '0'          0                '8'          8
///             '1'          1                '9'          9
///             '2'          2                'A'         10
///             '3'          3                'B'         11
///             '4'          4                'C'         12
///             '5'          5                'D'         13
///             '6'          6                'E'         14
///             '7'          7                'F'         15
///
///           In order to obtain the entire signed hexadecimal number,
///           the output character string should be at least N
///           characters long, where
///
///                           # of bits per integer + 3
///                 N = 1 + ---------------------------- .
///                                      4
///
///           There should be 1 character position for the sign, and
///           one character position for each hexadecimal digit that
///           could be produced from any integer which can be
///           represented by a particular computer system.
///
///           The following table contains minimum output string
///           lengths necessary to obtain the complete hexadecimal
///           string for various integer sizes.
///
///              Integer size in bits      Minimum output length
///              --------------------      ---------------------
///              8                         3
///              16                        5
///              32                        9
///              36 (really,it exists)     10
///              64                        17
///              etc.
///
///           The hexadecimal character string produced by this
///           routine will be left justified and consist of a
///           contiguous sequence of hexadecimal digits, or in the
///           case of a negative number, a contiguous sequence of
///           hexadecimal digits immediately preceded by a minus
///           sign, '-', e.g.:
///
///              (1)   h h ... h
///                     1 2     n
///
///              (2)   -h h ... h
///                      1 2     n
///
///           where h  represents an hexadecimal digit.
///                  i
///
///           The character string produced will be blank padded on
///           the right if LENGTH < LEN( STRING ).
///
///  LENGTH   is the length of the hexadecimal character string
///           produced by the conversion.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the output character string is not long enough to
///      contain the entire hexadecimal string that was produced,
///      the hexadecimal string will be truncated on the right.
///
///  2)  If LEN( STRING ) > LENGTH, the output character string will
///      be blank padded on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will convert a signed integer into an equivalent
///  signed hexadecimal character string. This provides a machine
///  independent mechanism for storing or porting integer values.
///  This routine is used by the routine DP2HX which converts a
///  double precision value into an equivalent character string.
///
///  This routine is one of a pair of routines which are used to
///  perform conversions between integers and equivalent signed
///  hexadecimal character strings:
///
///        INT2HX -- Convert an integer into a signed hexadecimal
///                  character string.
///
///        HX2INT -- Convert a signed hexadecimal character string
///                  into an integer.
/// ```
///
/// # Examples
///
/// ```text
///  All of the values shown are for a two's complement representation
///  for signed integers.
///
///  The following input and output argument values illustrate the
///  action of INT2HX for various input values of NUMBER.
///
///      NUMBER       STRING           LENGTH
///      -----------  ---------------  ------
///       1           '1'              1
///      -1           '-1'             2
///       223         'DF'             2
///      -32          '-20'            3
///       0           '0'              1
///
///       2147483647  '7FFFFFFF'       8
///       (Maximum 32 bit integer)
///
///      -2147483647  '-7FFFFFFF'      9
///       (Minimum 32 bit integer + 1)
///
///      -2147483648  '-80000000'      9
///       (Minimum 32 bit integer)
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The maximum number of characters permitted in the output
///      string is specified by the local parameter STRLEN.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
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
/// -    SPICELIB Version 1.0.0, 22-OCT-1992 (KRG)
/// ```
pub fn int2hx(ctx: &mut SpiceContext, number: i32, string: &mut str, length: &mut i32) {
    INT2HX(
        number,
        fstr::StrBytes::new(string).as_mut(),
        length,
        ctx.raw_context(),
    );
}

//$Procedure INT2HX  ( Integer to signed hexadecimal string )
pub fn INT2HX(NUMBER: i32, STRING: &mut [u8], LENGTH: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TMPSTR = [b' '; STRLEN as usize];
    let mut BEGIN: i32 = 0;
    let mut ITEMP: i32 = 0;
    let mut REMNDR: i32 = 0;
    let mut RESULT: i32 = 0;

    //
    // Local Parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //
    //
    // Local variables
    //

    //
    // The hexadecimal digits in the integer are found by repeated
    // applications of the "modulus" and division operations. We fill
    // the string in reverse order so that the digits are in the
    // correct order when we have finished building the string. We then
    // left justify the resulting string and set the value for its
    // length before returning.
    //
    // Make a copy of the input so that it will not be changed by this
    // routine.
    //
    ITEMP = NUMBER;
    //
    // We need to do different things for the cases where the integer to
    // be converted is positive, negative, or zero. ( Actually, the
    // positive case and the zero case are the same, but since we can
    // test for integer zero exactly it will save a few arithmetic
    // operations if we treat it as a special case. ) The case for a
    // negative integer is the only one which truly might cause problems,
    // because ABS(minimum integer) may equal ABS(maximum integer) + 1,
    // on some machines. For example, on many machines with 32 bit
    // integers, INTMIN = -2147483648 and INTMAX = 2147483647.
    //
    // Set the beginning position of the hexadecimal number to be
    // one past the end of the character string that will hold the
    // hexadecimal representation of the input number. Before each
    // digit of the hexadecimal number is inserted into the character
    // string, the beginning position is decremented, so we always know
    // exactly where the hexadecimal string begins. This simplifies the
    // calculation of the length of the hexadecimal character string at
    // the end of the routine.
    //
    BEGIN = (STRLEN + 1);

    if (ITEMP < 0) {
        //
        // Collect all of the digits in the string. We know we're done
        // when the value of ITEMP is equal to zero, thanks to the fact
        // that integer arithmetic operations are exact.
        //
        while (ITEMP != 0) {
            BEGIN = (BEGIN - 1);
            RESULT = (ITEMP / BASE);
            REMNDR = ((RESULT * BASE) - ITEMP);
            ITEMP = RESULT;
            fstr::assign(
                fstr::substr_mut(&mut TMPSTR, BEGIN..=BEGIN),
                save.DIGITS.get(REMNDR),
            );
        }
        //
        // Put the minus sign in place.
        //
        BEGIN = (BEGIN - 1);
        fstr::assign(fstr::substr_mut(&mut TMPSTR, BEGIN..=BEGIN), b"-");
    } else if (ITEMP > 0) {
        //
        // Collect all of the digits in the string. We know we're done
        // when the value of ITEMP is equal to zero, thanks to the fact
        // that integer arithmetic operations are exact.
        //
        while (ITEMP != 0) {
            BEGIN = (BEGIN - 1);
            RESULT = (ITEMP / BASE);
            REMNDR = (ITEMP - (RESULT * BASE));
            ITEMP = RESULT;
            fstr::assign(
                fstr::substr_mut(&mut TMPSTR, BEGIN..=BEGIN),
                save.DIGITS.get(REMNDR),
            );
        }
    } else {
        //
        // Treat zero as a special case, because it's easier.
        //
        BEGIN = (BEGIN - 1);
        fstr::assign(
            fstr::substr_mut(&mut TMPSTR, BEGIN..=BEGIN),
            save.DIGITS.get(0),
        );
    }
    //
    // Set the value of the output string before returning. Let the
    // Fortran string assignment deal with the left justification, and
    // the truncation on the right if the output string STRING is not
    // long enough to contain all of the characters in the string
    // that was produced.
    //
    fstr::assign(STRING, fstr::substr(&TMPSTR, BEGIN..));
    //
    // Also, set the value for the length of the hexadecimal string
    // before returning.
    //
    *LENGTH = ((STRLEN - BEGIN) + 1);
}
