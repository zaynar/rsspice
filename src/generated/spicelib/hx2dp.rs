//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const MAXMAN: i32 = 31;
const EXPCHR: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"^");
const BASE: f64 = 16.0;
const INVBAS: f64 = (1.0 / BASE);
const MAXDIG: i32 = 16;

struct SaveVars {
    DPVAL: StackArray<f64, 16>,
    MAXDP: f64,
    MINDP: f64,
    SCALES: StackArray<f64, 31>,
    DIGBEG: i32,
    DIGEND: i32,
    IEXPCH: i32,
    IMINUS: i32,
    IPLUS: i32,
    ISPACE: i32,
    LCCBEG: i32,
    LCCEND: i32,
    UCCBEG: i32,
    UCCEND: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DPVAL = StackArray::<f64, 16>::new(0..=(MAXDIG - 1));
        let mut MAXDP: f64 = 0.0;
        let mut MINDP: f64 = 0.0;
        let mut SCALES = StackArray::<f64, 31>::new(1..=MAXMAN);
        let mut DIGBEG: i32 = 0;
        let mut DIGEND: i32 = 0;
        let mut IEXPCH: i32 = 0;
        let mut IMINUS: i32 = 0;
        let mut IPLUS: i32 = 0;
        let mut ISPACE: i32 = 0;
        let mut LCCBEG: i32 = 0;
        let mut LCCEND: i32 = 0;
        let mut UCCBEG: i32 = 0;
        let mut UCCEND: i32 = 0;
        let mut FIRST: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(1.0),
                Val::D(2.0),
                Val::D(3.0),
                Val::D(4.0),
                Val::D(5.0),
                Val::D(6.0),
                Val::D(7.0),
                Val::D(8.0),
                Val::D(9.0),
                Val::D(10.0),
                Val::D(11.0),
                Val::D(12.0),
                Val::D(13.0),
                Val::D(14.0),
                Val::D(15.0),
            ]
            .into_iter();
            DPVAL
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;

        Self {
            DPVAL,
            MAXDP,
            MINDP,
            SCALES,
            DIGBEG,
            DIGEND,
            IEXPCH,
            IMINUS,
            IPLUS,
            ISPACE,
            LCCBEG,
            LCCEND,
            UCCBEG,
            UCCEND,
            FIRST,
        }
    }
}

/// Hexadecimal string to d.p. number
///
/// Convert a string representing a double precision number in a
/// base 16 "scientific notation" into its equivalent double
/// precision number.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MAXMAN     P   Maximum number of digits in a hex mantissa.
///  STRING     I   Hex form string to convert to double precision.
///  NUMBER     O   Double precision value to be returned.
///  ERROR      O   A logical flag which is .TRUE. on error.
///  ERRMSG     O   A descriptive error message.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is a character string containing a base 16 "scientific
///           notation" representation of a double precision number
///           which is to be converted to a double precision number.
///           Examples of such a string are:
///
///              '2A^3' = ( 2/16 + 10/( 16**2 ) ) * 16**3 = 672.0
///
///           and
///
///              '-B^1' = - ( 11/16 ) * 16**1             = -11.0
///
///           The following table describes the character set used to
///           represent the hexadecimal digits and their corresponding
///           values.
///
///              Character     Value         Character     Value
///              ---------    -------        ---------    -------
///                 '0'         0.0D0           '8'         8.0D0
///                 '1'         1.0D0           '9'         9.0D0
///                 '2'         2.0D0         'A','a'      10.0D0
///                 '3'         3.0D0         'B','b'      11.0D0
///                 '4'         4.0D0         'C','c'      12.0D0
///                 '5'         5.0D0         'D','d'      13.0D0
///                 '6'         6.0D0         'E','e'      14.0D0
///                 '7'         7.0D0         'F','f'      15.0D0
///
///           The caret, or hat, character, '^', is used to
///           distinguish the exponent.
///
///           The plus sign, '+', and the minus sign, '-', are used,
///           and they have their usual meanings.
///
///           A base 16 "scientific notation" character string which
///           is to be parsed by this routine should consist of a sign,
///           '+' or '-' (the plus sign is optional for nonnegative
///           numbers), followed immediately by a contiguous sequence
///           of hexadecimal digits, the exponent character, and a
///           signed hexadecimal exponent. The exponent is required,
///           but the sign is optional for a nonnegative exponent.
///
///           A number in base 16 "scientific notation" consists of
///           a contiguous sequence of characters with one of the
///           following formats:
///
///              (1)   h h h h  ... h ^H H  ... H
///                     1 2 3 4      n  1 2      m
///
///              (2)   +h h h h  ... h ^H H  ... H
///                      1 2 3 4      n  1 2      m
///
///              (3)   -h h h h  ... h ^H H  ... H
///                      1 2 3 4      n  1 2      m
///
///              (4)    h h h h  ... h ^+H H  ... H
///                      1 2 3 4      n   1 2      m
///
///              (5)   +h h h h  ... h ^+H H  ... H
///                      1 2 3 4      n   1 2      m
///
///              (6)   -h h h h  ... h ^+H H  ... H
///                      1 2 3 4      n   1 2      m
///
///              (7)   h h h h  ... h ^-H H  ... H
///                     1 2 3 4      n   1 2      m
///
///              (8)   +h h h h  ... h ^-H H  ... H
///                      1 2 3 4      n   1 2      m
///
///              (9)   -h h h h  ... h ^-H H  ... H
///                      1 2 3 4      n   1 2      m
///
///           where
///
///              h  and H  denote hexadecimal digits;
///               i      j
///
///              ^         denotes exponentiation;
///
///           and
///
///              + and - have their usual interpretations.
///
///           STRING may have leading and trailing blanks, but blanks
///           embedded within the significant portion of the input
///           string are not allowed.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NUMBER   is the double precision value to be returned. The value
///           of this argument is not changed if an error occurs while
///           parsing the input string.
///
///  ERROR    is a logical flag which indicates whether an error
///           occurred while attempting to parse NUMBER from the input
///           character string STRING. ERROR will have the value
///           .TRUE. if an error occurs. It will have the value
///           .FALSE. otherwise.
///
///  ERRMSG   is a descriptive error message if an error occurs while
///           attempting to parse the number NUMBER from the
///           hexadecimal character string STRING, blank otherwise.
/// ```
///
/// # Parameters
///
/// ```text
///  MAXMAN   is the maximum number of digits in a hexadecimal
///           mantissa. The value of MAXMAN is 31.
///
///           The current value of MAXMAN is more than sufficient for
///           most double precision implementations, providing almost
///           twice as many digits as can actually be produced. This
///           value may be changed when a greater precision is known
///           to exist among all of the supported platforms.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If an unexpected character is encountered, an appropriate
///      error message will be set, and the routine will exit. The
///      value of NUMBER will be unchanged.
///
///  2)  If the input string represents a number that is larger in
///      absolute magnitude than the maximum representable
///      double precision number an appropriate error message
///      will be set, and the routine will exit. The value of
///      NUMBER will be unchanged.
///
///  3)  If the input string is blank, an appropriate error message
///      will be set, and the routine will exit. The value of
///      NUMBER will be unchanged.
///
///  4)  If the string has too many digits in the mantissa, then an
///      appropriate error message will be set, and the routine will
///      exit. The value of NUMBER will be unchanged.
///
///  5)  If the output error message string is not long enough to
///      contain the entire error message, the error message will be
///      truncated on the right.
///
///  6)  This routine does NOT check for underflow errors when
///      constructing a double precision number.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will convert a character string containing a number
///  in base 16 "scientific notation" into its equivalent double
///  precision number.
///
///  This routine is one of a pair of routines which are used to
///  perform conversions between double precision numbers and
///  an equivalent base 16 "scientific notation" character string
///  representation:
///
///        DP2HX  -- Convert a double precision number into a base 16
///                  "scientific notation" character string.
///
///        HX2DP  -- Convert a base 16 "scientific notation"
///                  character string into a double precision number.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Convert a set of character strings containing a base 16
///     "scientific notation" representation of a double precision
///     number, to their double precision values.
///
///
///     Example code begins here.
///
///
///           PROGRAM HX2DP_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local constants.
///     C
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN = 80 )
///
///           INTEGER               STRLEN
///           PARAMETER           ( STRLEN = 17 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(ERRLEN)    ERRMSG
///           CHARACTER*(STRLEN)    NUMBER ( 16 )
///
///           DOUBLE PRECISION      VALUE
///
///           INTEGER               I
///
///           LOGICAL               ERROR
///
///     C
///     C     Assign an array of strings representing, in base 16
///     C     "scientific notation", double precision numbers.
///     C     Not all of them are valid representations.
///     C
///           DATA                  NUMBER /
///          .                  '89705F4136B4A6^-7', '12357898765X34',
///          .                  '1^1',               '-1^1',
///          .                  '4^3',               '-4^3',
///          .                  '7F5EB^5',           '7F5eb^5',
///          .                  '1B^2',              '+1B^2',
///          .                  '+1B^+2',            '0^0',
///          .                  ' ',                 '-AB238Z^2',
///          .                  '234ABC',            '234ABC^'    /
///
///     C
///     C     Loop over the NUMBER array, call HX2DP for each
///     C     element of NUMBER.
///     C
///           WRITE(*,'(A)') 'string             number'
///           WRITE(*,'(A)') '-----------------  ----------------'
///
///           DO I= 1, 16
///
///              CALL HX2DP ( NUMBER(I), VALUE, ERROR, ERRMSG )
///
///              IF ( ERROR ) THEN
///
///                 WRITE(*,'(A17,2X,A)') NUMBER(I), ERRMSG
///
///              ELSE
///
///                 WRITE(*,'(A17,X,E17.9)') NUMBER(I), VALUE
///
///              END IF
///
///           END DO
///
///     C
///     C     Finally, try with a number that has too many digits in
///     C     the mantissa.
///     C
///           CALL HX2DP ( '4ABC123AB346523BDC568798C2473678^1',
///          .             VALUE, ERROR, ERRMSG )
///
///           WRITE(*,*)
///           WRITE(*,*) 'String 4ABC123AB346523BDC568798C2473678^1 '
///          .        // 'produces:'
///           WRITE(*,*) '   ', ERRMSG
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     string             number
///     -----------------  ----------------
///     89705F4136B4A6^-7   0.200000000E-08
///     12357898765X34     ERROR: Illegal character 'X' encountered.
///     1^1                 0.100000000E+01
///     -1^1               -0.100000000E+01
///     4^3                 0.102400000E+04
///     -4^3               -0.102400000E+04
///     7F5EB^5             0.521707000E+06
///     7F5eb^5             0.521707000E+06
///     1B^2                0.270000000E+02
///     +1B^2               0.270000000E+02
///     +1B^+2              0.270000000E+02
///     0^0                 0.000000000E+00
///                        ERROR: A blank input string is not allowed.
///     -AB238Z^2          ERROR: Illegal character 'Z' encountered.
///     234ABC             ERROR: Missing exponent.
///     234ABC^            ERROR: Missing exponent.
///
///      String 4ABC123AB346523BDC568798C2473678^1 produces:
///         ERROR: Too many digits in the mantissa (> 31).
///
///
///     Note: The hat or caret, '^', signals an exponent.
///
///     Note that some errors are machine dependent. For example,
///     for a VAX using D_floating arithmetic we get:
///
///        STRING = '23BCE^30'
///        NUMBER = ( Not defined )
///        ERROR  = .TRUE.
///        ERRMSG = 'ERROR: Number is too large to be represented.'
///
///        STRING = '-2abc3^22'
///        NUMBER = ( Not defined )
///        ERROR  = .TRUE.
///        ERRMSG = 'ERROR: Number is too small to be represented.'
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The current value of MAXMAN is more than sufficient for most
///      double precision implementations, providing almost twice as
///      many digits as can actually be produced.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR) (BVS)
///
///         Added IMPLICIT NONE statement.
///
///         The declaration of MAXMAN has been promoted to the
///         $Declarations section and the error produced when the maximum
///         number of digits for the mantissa is exceeded has been updated
///         to inform about MAXMAN value.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example.
///
///         Updated $Brief_I/O, $Parameters, $Exceptions and $Restrictions
///         sections to properly describe MAXMAN.
///
///         Corrected $Revisions entries.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1994 (KRG)
///
///         Fixed a typo in the description of the input argument STRING.
///         The example showing the expansion of 160 into hexadecimal
///         was incorrect. 160 was replaced with 672 which makes the
///         example correct.
///
/// -    SPICELIB Version 1.0.0, 26-OCT-1992 (KRG)
/// ```
pub fn hx2dp(
    ctx: &mut SpiceContext,
    string: &str,
    number: &mut f64,
    error: &mut bool,
    errmsg: &mut str,
) {
    HX2DP(
        string.as_bytes(),
        number,
        error,
        fstr::StrBytes::new(errmsg).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure HX2DP ( Hexadecimal string to d.p. number )
pub fn HX2DP(
    STRING: &[u8],
    NUMBER: &mut f64,
    ERROR: &mut bool,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TMPNUM: f64 = 0.0;
    let mut IEXPON: i32 = 0;
    let mut IVAL = StackArray::<i32, 32>::new(1..=(MAXMAN + 1));
    let mut LETTER: i32 = 0;
    let mut NDIGIT: i32 = 0;
    let mut POSITN: i32 = 0;
    let mut STRBEG: i32 = 0;
    let mut STREND: i32 = 0;
    let mut MORE: bool = false;
    let mut NEGTIV: bool = false;
    let mut FNDEXP: bool = false;

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
    // Here is a brief outline of the algorithm used to convert the
    // character string into its equivalent double precision number.
    //
    //    The input hexadecimal string is scanned from left to right.
    //
    //    0) Any leading white space is skipped.
    //
    //    1) The length of the significant portion of the string
    //       is determined.
    //
    //    2) The sign of the mantissa is determined.
    //
    //    3) The digits of the hexadecimal mantissa are parsed.
    //
    //    4) The exponent of the number is parsed.
    //
    //    5) The mantissa of the double precision number is generated
    //       by summing appropriately scaled values of the hexadecimal
    //       mantissa digits which were collected in step 2. The
    //       summation is performed so that the summands are added
    //       in order of increasing magnitude to eliminate a potential
    //       loss of significance which might occur otherwise. This
    //       yields a number in the range of 1/BASE and 1.0 or zero.
    //
    //    6) The double precision number is then scaled by the exponent
    //       obtained in step 3.
    //
    if save.FIRST {
        //
        // If this is the first call, set up the array that is used to
        // properly scale each of the hexadecimal digits when summing
        // them to build a double precision number. Right now, the value
        // of MAXMAN, the maximum number of digits in a hexadecimal
        // mantissa, is 31. MAXMAN = 31 is more than sufficient for most
        // current double precision implementations, providing almost
        // twice as many digits as can actually be produced. This value
        // may be changed when a greater precision is known to exist on
        // any of the supported platforms.
        //
        save.FIRST = false;

        save.SCALES[1] = INVBAS;
        for I in 2..=MAXMAN {
            save.SCALES[I] = (INVBAS * save.SCALES[(I - 1)]);
        }
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
        // Also get the integer value for the exponent character.
        //
        save.IEXPCH = intrinsics::ICHAR(EXPCHR);
        //
        // Initialize some boundary values for error checking while
        // constructing the desired double precision number. These
        // are used to help determine whether an overflow condition
        // is imminent due to the overly large magnitude of a positive
        // or negative number.
        //
        save.MINDP = (DPMIN() * INVBAS);
        save.MAXDP = (DPMAX() * INVBAS);
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
    TMPNUM = 0.0;
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
    // the input string and the position of the exponent character.
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
    POSITN = STRBEG;
    //
    // The first character should be either a plus sign, '+', a
    // minus sign, '-', or a digit, '0' - '9', 'A' - 'F', or
    // 'a' - 'f'. Anything else is bogus and we will catch it in
    // the main loop below.
    //
    // If the character is a minus sign, we want to set the value of
    // NEGTIV to .TRUE. and increment the position.
    //
    // If the character is a plus sign, we want to increment the
    // position.
    //
    if (intrinsics::ICHAR(fstr::substr(STRING, POSITN..=POSITN)) == save.IMINUS) {
        NEGTIV = true;
        POSITN = (POSITN + 1);
    } else if (intrinsics::ICHAR(fstr::substr(STRING, POSITN..=POSITN)) == save.IPLUS) {
        POSITN = (POSITN + 1);
    }
    //
    // Collect all of the digits in the mantissa, storing them
    // for later conversion. We do this because we want to add
    // the digits of the mantissa in increasing order so that we
    // do not lose any significance.
    //
    // A normalized hexadecimal number must have an exponent,
    // which is represented by the hat character, EXPCHR, which
    // s why that test is part of the loop termination.
    //
    // We currently have no digits, and we have not found the
    // exponent character yet.
    //
    NDIGIT = 0;
    FNDEXP = false;
    while ((POSITN <= STREND) && !FNDEXP) {
        LETTER = intrinsics::ICHAR(fstr::substr(STRING, POSITN..=POSITN));

        if ((LETTER >= save.DIGBEG) && (LETTER <= save.DIGEND)) {
            POSITN = (POSITN + 1);
            NDIGIT = (NDIGIT + 1);
            IVAL[NDIGIT] = (LETTER - save.DIGBEG);
        } else if ((LETTER >= save.UCCBEG) && (LETTER <= save.UCCEND)) {
            POSITN = (POSITN + 1);
            NDIGIT = (NDIGIT + 1);
            IVAL[NDIGIT] = ((10 + LETTER) - save.UCCBEG);
        } else if ((LETTER >= save.LCCBEG) && (LETTER <= save.LCCEND)) {
            POSITN = (POSITN + 1);
            NDIGIT = (NDIGIT + 1);
            IVAL[NDIGIT] = ((10 + LETTER) - save.LCCBEG);
        } else if (LETTER == save.IEXPCH) {
            //
            // We have found the exponent character, so set the
            // indicator and increment the position.
            //
            FNDEXP = true;
            POSITN = (POSITN + 1);
        } else {
            *ERROR = true;
            fstr::assign(ERRMSG, b"ERROR: Illegal character \'#\' encountered.");
            REPMC(&ERRMSG.to_vec(), b"#", &intrinsics::CHAR(LETTER), ERRMSG);
            return;
        }
        //
        // We need to make sure that the number of mantissa digits
        // remains less than or equal to the number of mantissa
        // digits that we declared, see the MAXMAN parameter.
        //
        if (NDIGIT > MAXMAN) {
            *ERROR = true;
            fstr::assign(ERRMSG, b"ERROR: Too many digits in the mantissa (> #).");
            REPMI(&ERRMSG.to_vec(), b"#", MAXMAN, ERRMSG, ctx);
            return;
        }
    }
    //
    // At this point, we have found an exponent character, and:
    //
    //    1) We are beyond the end of the significant portion of the
    //       string, which is an error: no exponent digits were found.
    //
    //    2) We are positioned on the first digit of the exponent,
    //       and are ready to try and parse it.
    //
    if (POSITN <= STREND) {
        //
        // If there is at least one significant character left in the
        // string, we need to try and parse it as an exponent.
        //
        HX2INT(
            fstr::substr(STRING, POSITN..),
            &mut IEXPON,
            ERROR,
            ERRMSG,
            ctx,
        );

        if *ERROR {
            //
            // If an error occurred while attempting to parse the
            // exponent, we simply want to exit. The error message
            // is already set.
            //
            return;
        }
    } else {
        *ERROR = true;
        fstr::assign(ERRMSG, b"ERROR: Missing exponent.");
        return;
    }
    //
    // We now have everything that we need to build the double
    // precision number, a mantissa and an exponent. So, let's
    // start building the number. We need to be careful that we
    // do not overflow when we scale the number using the exponent.
    //
    // First, we build up the mantissa ...
    //
    if NEGTIV {
        while (NDIGIT > 0) {
            TMPNUM = (TMPNUM - (save.DPVAL[IVAL[NDIGIT]] * save.SCALES[NDIGIT]));
            NDIGIT = (NDIGIT - 1);
        }
    } else {
        while (NDIGIT > 0) {
            TMPNUM = (TMPNUM + (save.DPVAL[IVAL[NDIGIT]] * save.SCALES[NDIGIT]));
            NDIGIT = (NDIGIT - 1);
        }
    }
    //
    // At this point, one of the following is true:
    //
    //    1)  -1     <  TMPNUM <= -1/BASE
    //
    //    2)  1/BASE <= TMPNUM <  1
    //
    // or
    //
    //    3) TMPNUM = 0.0D0
    //
    // Now we to scale the normalized number using the exponent. If
    // the exponent is zero, we will simply fall through the loop
    // structures below at no greater cost than a few comparisons.
    //
    if (IEXPON < 0) {
        //
        // We do not check for any sort of underflow conditions.
        //
        for I in 1..=-IEXPON {
            TMPNUM = (TMPNUM * INVBAS);
        }
    } else {
        if NEGTIV {
            for I in 1..=IEXPON {
                if (TMPNUM >= save.MINDP) {
                    TMPNUM = (TMPNUM * BASE);
                } else {
                    *ERROR = true;
                    fstr::assign(ERRMSG, b"ERROR: Number is too small to be represented.");
                    return;
                }
            }
        } else {
            for I in 1..=IEXPON {
                if (TMPNUM <= save.MAXDP) {
                    TMPNUM = (TMPNUM * BASE);
                } else {
                    *ERROR = true;
                    fstr::assign(ERRMSG, b"ERROR: Number is too large to be represented.");
                    return;
                }
            }
        }
    }
    //
    // If we got to here, we have successfully parsed the hexadecimal
    // string into a double precision number. So, set the value and
    // return.
    //
    *NUMBER = TMPNUM;
}
