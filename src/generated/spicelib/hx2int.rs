//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BASE: i32 = 16;

struct SaveVars {
    DIGBEG: i32,
    DIGEND: i32,
    IMINUS: i32,
    IPLUS: i32,
    ISPACE: i32,
    LCCBEG: i32,
    LCCEND: i32,
    MAXI: i32,
    MAXMOD: i32,
    MINI: i32,
    MINMOD: i32,
    UCCBEG: i32,
    UCCEND: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DIGBEG: i32 = 0;
        let mut DIGEND: i32 = 0;
        let mut IMINUS: i32 = 0;
        let mut IPLUS: i32 = 0;
        let mut ISPACE: i32 = 0;
        let mut LCCBEG: i32 = 0;
        let mut LCCEND: i32 = 0;
        let mut MAXI: i32 = 0;
        let mut MAXMOD: i32 = 0;
        let mut MINI: i32 = 0;
        let mut MINMOD: i32 = 0;
        let mut UCCBEG: i32 = 0;
        let mut UCCEND: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            DIGBEG,
            DIGEND,
            IMINUS,
            IPLUS,
            ISPACE,
            LCCBEG,
            LCCEND,
            MAXI,
            MAXMOD,
            MINI,
            MINMOD,
            UCCBEG,
            UCCEND,
            FIRST,
        }
    }
}

/// Signed hexadecimal string to integer
///
/// Convert a signed hexadecimal string representation of an integer
/// to its equivalent integer.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   Hexadecimal string to be converted to an integer.
///  NUMBER     O   Integer value to be returned.
///  ERROR      O   A logical flag which is .TRUE. on error.
///  ERRMSG     O   A descriptive error message.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is the hexadecimal string to be converted to an integer.
///
///           The following table describes the character set used
///           to represent the hexadecimal digits and their
///           corresponding values.
///
///           Character    Value           Character    Value
///           ---------    -----           ---------    -----
///             '0'          0                '8'          8
///             '1'          1                '9'          9
///             '2'          2              'A','a'       10
///             '3'          3              'B','b'       11
///             '4'          4              'C','c'       12
///             '5'          5              'D','d'       13
///             '6'          6              'E','e'       14
///             '7'          7              'F','f'       15
///
///          The plus sign, '+', and the minus sign, '-', are used as
///          well, and they have their usual meanings.
///
///          A hexadecimal character string parsed by this routine
///          should consist of a sign, '+' or '-' (the plus sign is
///          optional for nonnegative numbers), followed immediately
///          by a contiguous sequence of hexadecimal digits, e.g.:
///
///             (1)   +h h ... h
///                     1 2     n
///
///             (2)   -h h ... h
///                     1 2     n
///
///             (3)   h h ... h
///                    1 2     n
///
///          where h  represents an hexadecimal digit.
///                 i
///
///          STRING may have leading and trailing blanks, but blanks
///          embedded within the significant portion of the character
///          string are not allowed. This includes any blanks which
///          appear between the sign character and the first
///          hexadecimal digit.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NUMBER   is the integer value to be returned. The value of this
///           variable is not changed if an error occurs while parsing
///           the hexadecimal character string.
///
///  ERROR    is a logical flag which indicates whether an error
///           occurred while attempting to parse NUMBER from the
///           hexadecimal character string STRING. ERROR will have the
///           value .TRUE. if an error occurs. It will have the value
///           .FALSE. otherwise.
///
///  ERRMSG   contains a descriptive error message if an error
///           occurs while attempting to parse NUMBER from the
///           hexadecimal character string STRING, blank otherwise.
///           The error message will be left justified.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If an unexpected character is encountered while parsing the
///      hexadecimal character string, an appropriate error message
///      will be set, and the routine will exit. The value of NUMBER
///      will be unchanged.
///
///  2)  If the string represents a number that is larger than
///      the maximum representable integer an appropriate error
///      message will be set, and the routine will exit. The value
///      of NUMBER will be unchanged.
///
///  3)  If the string represents a number that is smaller than
///      the minimum representable integer, an appropriate error
///      message will be set, and the routine will exit. The value
///      of NUMBER will be unchanged.
///
///  4)  If the input string is blank, an appropriate error message
///      will be set, and the routine will exit. The value of NUMBER
///      will be unchanged.
///
///  5)  If the error message string is not long enough to contain
///      the entire error message, the error message will be
///      truncated on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will convert a signed hexadecimal character string
///  representation of an integer into its equivalent integer. This
///  provides a machine independent mechanism for storing or porting
///  integer values. This routine is used by the routine HX2DP which
///  converts a character string representation of a double precision
///  into its equivalent double precision value.
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
///  All of the values shown are for a two's complement 32 bit
///  representation for signed integers.
///
///  The following argument values illustrate the action of HX2INT for
///  various input values.
///
///      STRING                 NUMBER        ERROR   ERRMSG
///      ---------------------  ------------  ------  ------
///       '1'                    1            .FALSE.   ' '
///       '-1'                  -1            .FALSE.   ' '
///       'DF'                   223          .FALSE.   ' '
///       'Df'                   223          .FALSE.   ' '
///       '+3ABC'                15036        .FALSE.   ' '
///       'ff'                   255          .FALSE.   ' '
///       '-20'                 -32           .FALSE.   ' '
///       '0'                    0            .FALSE.   ' '
///
///       '7FFFFFFF'             2147483647   .FALSE.   ' '
///       (Maximum 32 bit integer)
///
///       '-7FFFFFFF'           -2147483647   .FALSE.   ' '
///       (Minimum 32 bit integer + 1)
///
///       '-80000000'           -2147483648   .FALSE.   ' '
///       (Minimum 32 bit integer)
///
///       STRING = ' '
///       NUMBER = ( Not defined )
///       ERROR  = .TRUE.
///       ERRMSG = 'ERROR: A blank input string is not allowed.'
///
///       STRING = '-AB238Q'
///       NUMBER = ( Not defined )
///       ERROR  = .TRUE.
///       ERRMSG = 'ERROR: Illegal character ''Q'' encountered.'
///
///       STRING = '- AAA'
///       NUMBER = ( Not defined )
///       ERROR  = .TRUE.
///       ERRMSG = 'ERROR: Illegal character '' '' encountered.'
///
///       STRING = '80000000'
///       NUMBER = ( Not defined )
///       ERROR  = .TRUE.
///       ERRMSG = 'ERROR: Integer too large to be represented.'
///
///       STRING = '-800F0000'
///       NUMBER = ( Not defined )
///       ERROR  = .TRUE.
///       ERRMSG = 'ERROR: Integer too small to be represented.'
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
/// -    SPICELIB Version 1.1.1, 20-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 10-MAR-1994 (KRG)
///
///         Changed an IF test operand from .LE. to .LT. so that
///         the ELSE IF clause could be reached. This change has
///         NO effect on the execution of the routine because it
///         makes use of a base that is a power of 2 (16), so the
///         ELSE IF clause never needs to be reached. The algorithm
///         was meant to be as general as possible, however, so that
///         only the base and digits would need to be changed in order to
///         implement a different number base.
///
/// -    SPICELIB Version 1.0.0, 22-OCT-1992 (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -     SPICELIB Version 1.1.0, 10-MAR-1994 (KRG)
///
///          Changed an IF test operand from .LE. to .LT. so that
///          the ELSE IF clause could be reached. This change has
///          NO effect on the execution of the routine because it
///          makes use of a base that is a power of 2 (16), so the
///          ELSE IF clause never needs to be reached. The algorithm
///          was meant to be as general as possible, however, so that
///          only the base and digits would need to be changed in order to
///          implement a different number base.
///
///          Old code was:
///
///             IF ( TMPNUM .LE. MAXI ) THEN
///
///                TMPNUM = TMPNUM * BASE + IDIGIT
///                POS    = POS + 1
///
///             ELSE IF ( ( TMPNUM .EQ. MAXI   ) .AND.
///      .                ( IDIGIT .LE. MAXMOD ) ) THEN
///
///                TMPNUM = TMPNUM * BASE + IDIGIT
///                POS    = POS + 1
///
///             ELSE ...
///
///          New code:
///
///             IF ( TMPNUM .LT. MAXI ) THEN
///
///                TMPNUM = TMPNUM * BASE + IDIGIT
///                POS    = POS + 1
///
///             ELSE IF ( ( TMPNUM .EQ. MAXI   ) .AND.
///      .                ( IDIGIT .LE. MAXMOD ) ) THEN
///
///                TMPNUM = TMPNUM * BASE + IDIGIT
///                POS    = POS + 1
///
///             ELSE ...
/// ```
pub fn hx2int(
    ctx: &mut SpiceContext,
    string: &str,
    number: &mut i32,
    error: &mut bool,
    errmsg: &mut str,
) {
    HX2INT(
        string.as_bytes(),
        number,
        error,
        fstr::StrBytes::new(errmsg).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure HX2INT  ( Signed hexadecimal string to integer )
pub fn HX2INT(
    STRING: &[u8],
    NUMBER: &mut i32,
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut IDIGIT: i32 = 0;
    let mut LETTER: i32 = 0;
    let mut POS: i32 = 0;
    let mut STRBEG: i32 = 0;
    let mut STREND: i32 = 0;
    let mut TMPNUM: i32 = 0;
    let mut MORE: bool = false;
    let mut NEGTIV: bool = false;

    //
    // SPICELIB functions
    //
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
    // Initial values
    //

    //
    // The input hexadecimal string is scanned from left to right, and
    // the integer is generated by repeated multiplications and additions
    // or subtractions.
    //
    // If this is the first time that this routine has been called,
    // we need to do some setup stuff.
    //
    if save.FIRST {
        save.FIRST = false;
        //
        // Initialize the upper and lower bounds for the decimal digits,
        // the upper and lower bounds for the uppercase hexadecimal
        // digits, the upper and lower bounds for the lowercase
        // hexadecimal digits, the space, the plus sign, and the minus
        // sign in the character sequence.
        //
        save.DIGBEG = intrinsics::ICHAR(b"0");
        save.DIGEND = intrinsics::ICHAR(b"9");
        save.UCCBEG = intrinsics::ICHAR(b"A");
        save.UCCEND = intrinsics::ICHAR(b"F");
        save.LCCBEG = intrinsics::ICHAR(b"a");
        save.LCCEND = intrinsics::ICHAR(b"f");
        save.IMINUS = intrinsics::ICHAR(b"-");
        save.IPLUS = intrinsics::ICHAR(b"+");
        save.ISPACE = intrinsics::ICHAR(b" ");
        //
        // Initialize some boundary values for error checking while
        // constructing the desired integer. These are used to help
        // determine integer overflow or integer underflow errors.
        //
        save.MINI = (INTMIN() / BASE);
        save.MINMOD = ((BASE * save.MINI) - INTMIN());
        save.MAXI = (INTMAX() / BASE);
        save.MAXMOD = (INTMAX() - (BASE * save.MAXI));
    }
    //
    // There are no errors initially, so set the error flag to
    // .FALSE.
    //
    *ERROR = false;
    //
    // If the string is blank, set the error flag and return immediately.
    //
    if fstr::eq(STRING, b" ") {
        *ERROR = true;
        fstr::assign(ERRMSG, b"ERROR: A blank input string is not allowed.");
        return;
    }
    //
    // Initialize a few other things.
    //
    fstr::assign(ERRMSG, b" ");
    TMPNUM = 0;
    //
    // Assume that the number is nonnegative.
    //
    NEGTIV = false;
    //
    // Skip any leading white space. We know that there is at least
    // one nonblank character at this point, so we will not loop
    // off the end of the string.
    //
    STRBEG = 1;

    while (intrinsics::ICHAR(fstr::substr(STRING, STRBEG..=STRBEG)) == save.ISPACE) {
        STRBEG = (STRBEG + 1);
    }
    //
    // Now, we want to find the end of the significant portion of
    // the input string.
    //
    STREND = (STRBEG + 1);
    MORE = true;

    while MORE {
        if (STREND <= intrinsics::LEN(STRING)) {
            if fstr::ne(fstr::substr(STRING, STREND..), b" ") {
                STREND = (STREND + 1);
            } else {
                MORE = false;
            }
        } else {
            MORE = false;
        }
    }
    //
    // At this point, STREND is one larger than the length of the
    // significant portion of the string because we incremented
    // its value after the test. We will subtract one from the
    // value of STREND so that it exactly represents the position
    // of the last significant character in the string.
    //
    STREND = (STREND - 1);
    //
    // Set the position pointer to the beginning of the significant
    // part, i.e., the nonblank part, of the string, because we are
    // now ready to try and parse the number.
    //
    POS = STRBEG;
    //
    // The first character should be a plus sign, '+', a minus sign,
    // '-', or a digit, '0' - '9', 'A' - 'F', or 'a' - 'f'. Anything
    // else is bogus, and we will catch it in the main loop below.
    //
    // If the character is a minus sign, we want to set the value of
    // NEGTIV to .TRUE. and increment the position.
    //
    // If the character is a plus sign, we want to increment the
    // position.
    //
    if (intrinsics::ICHAR(fstr::substr(STRING, POS..=POS)) == save.IMINUS) {
        NEGTIV = true;
        POS = (POS + 1);
    } else if (intrinsics::ICHAR(fstr::substr(STRING, POS..=POS)) == save.IPLUS) {
        POS = (POS + 1);
    }
    //
    // When we build up the number from the hexadecimal string we
    // need to treat nonnegative numbers differently from negative
    // numbers. This is because on many computers the minimum
    // integer is one less than the negation of the maximum integer.
    // Negative numbers are the ones which truly might cause
    // problems, because ABS(minimum integer) may equal ABS(maximum
    // integer) + 1, on some machines. For example, on many machines
    // with 32 bit numbers, INTMIN = -2147483648 and INTMAX =
    // 2147483647.
    //
    // Build up the number from the hexadecimal character string.
    //
    if NEGTIV {
        while (POS <= STREND) {
            LETTER = intrinsics::ICHAR(fstr::substr(STRING, POS..=POS));

            if ((LETTER >= save.DIGBEG) && (LETTER <= save.DIGEND)) {
                IDIGIT = (LETTER - save.DIGBEG);
            } else if ((LETTER >= save.UCCBEG) && (LETTER <= save.UCCEND)) {
                IDIGIT = ((10 + LETTER) - save.UCCBEG);
            } else if ((LETTER >= save.LCCBEG) && (LETTER <= save.LCCEND)) {
                IDIGIT = ((10 + LETTER) - save.LCCBEG);
            } else {
                *ERROR = true;
                fstr::assign(ERRMSG, b"ERROR: Illegal character \'#\' encountered.");
                REPMC(&ERRMSG.to_vec(), b"#", &intrinsics::CHAR(LETTER), ERRMSG);
                return;
            }

            if (TMPNUM > save.MINI) {
                TMPNUM = ((TMPNUM * BASE) - IDIGIT);
                POS = (POS + 1);
            } else if ((TMPNUM == save.MINI) && (IDIGIT <= save.MINMOD)) {
                TMPNUM = ((TMPNUM * BASE) - IDIGIT);
                POS = (POS + 1);
            } else {
                *ERROR = true;
                fstr::assign(ERRMSG, b"ERROR: Integer too small to be represented.");
                return;
            }
        }
    } else {
        while (POS <= STREND) {
            LETTER = intrinsics::ICHAR(fstr::substr(STRING, POS..=POS));

            if ((LETTER >= save.DIGBEG) && (LETTER <= save.DIGEND)) {
                IDIGIT = (LETTER - save.DIGBEG);
            } else if ((LETTER >= save.UCCBEG) && (LETTER <= save.UCCEND)) {
                IDIGIT = ((10 + LETTER) - save.UCCBEG);
            } else if ((LETTER >= save.LCCBEG) && (LETTER <= save.LCCEND)) {
                IDIGIT = ((10 + LETTER) - save.LCCBEG);
            } else {
                *ERROR = true;
                fstr::assign(ERRMSG, b"ERROR: Illegal character \'#\' encountered.");
                REPMC(&ERRMSG.to_vec(), b"#", &intrinsics::CHAR(LETTER), ERRMSG);
                return;
            }

            if (TMPNUM < save.MAXI) {
                TMPNUM = ((TMPNUM * BASE) + IDIGIT);
                POS = (POS + 1);
            } else if ((TMPNUM == save.MAXI) && (IDIGIT <= save.MAXMOD)) {
                TMPNUM = ((TMPNUM * BASE) + IDIGIT);
                POS = (POS + 1);
            } else {
                *ERROR = true;
                fstr::assign(ERRMSG, b"ERROR: Integer too large to be represented.");
                return;
            }
        }
    }
    //
    // If we got to here, we have successfully parsed the hexadecimal
    // string into an integer. Set the value and return.
    //
    *NUMBER = TMPNUM;
}
