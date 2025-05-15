//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const STRLEN: i32 = 255;
const EXPCHR: &[u8; 1 as usize] = &fstr::extend_const::<{ 1 as usize }>(b"^");
const BASE: f64 = 16.0;
const INVBAS: f64 = (1.0 / BASE);
const EXPINB: i32 = 1;
const FACTR1: f64 = 4294967296.0;
const INVFC1: f64 = (1.0 / FACTR1);
const EXPIN1: i32 = 8;
const FACTR2: f64 = 65536.0;
const INVFC2: f64 = (1.0 / FACTR2);
const EXPIN2: i32 = 4;
const MAXDIG: i32 = 16;

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

/// D.p. number to hexadecimal string
///
/// Convert a double precision number to an equivalent character
/// string using a base 16 "scientific notation."
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRLEN     P   Max number of characters allowed in output string.
///  NUMBER     I   D.p. number to be converted.
///  HXSTR      O   Equivalent character string, left justified.
///  HXSSIZ     O   Length of the character string produced.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NUMBER   is the double precision number to be converted to a
///           character string representation.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HXSTR    is the character string produced by this routine that
///           represents NUMBER in a base 16 "scientific notation,"
///           e.g.:
///
///              672.0 = '2A^3' = ( 2/16 + 10/( 16**2 ) ) * 16**3
///
///           and
///
///              -11.0 = '-B^1' = - ( 11/16 ) * 16**1.
///
///           The following table describes the character set used to
///           represent the hexadecimal digits and their corresponding
///           values.
///
///              Character    Value         Character    Value
///              ---------    ------        ---------    ------
///                '0'         0.0D0          '8'         8.0D0
///                '1'         1.0D0          '9'         9.0D0
///                '2'         2.0D0          'A'        10.0D0
///                '3'         3.0D0          'B'        11.0D0
///                '4'         4.0D0          'C'        12.0D0
///                '5'         5.0D0          'D'        13.0D0
///                '6'         6.0D0          'E'        14.0D0
///                '7'         7.0D0          'F'        15.0D0
///
///           The caret, or hat, character, '^', is used to
///           distinguish the exponent.
///
///           The plus sign, '+', and the minus sign, '-', are used,
///           and they have their usual meanings.
///
///           In order to obtain the entire character string produced
///           by this routine, the output character string should be
///           at least N characters long, where
///
///
///                       # of bits per double precision mantissa + 3
///              N = 3 + ---------------------------------------------
///                                            4
///
///                       # of bits per double precision exponent + 3
///                    + ---------------------------------------------
///                                            4
///
///           There should be one character position for the sign of
///           the mantissa, one for the sign of the exponent, one for
///           the exponentiation character, and one for each
///           hexadecimal digit that could be produced from a mantissa
///           and an exponent.
///
///           The following table contains minimum output string
///           lengths necessary to obtain the complete character
///           string produced by this routine for some typical
///           implementations of double precision numbers.
///
///              Double precision number
///              Size Mantissa Exponent   Minimum output string
///              bits   bits     bits     length
///              ---- -------- --------   ----------------------
///              64   48       15         3 + 12 + 4 = 19
///              64   55+1     8          3 + 14 + 2 = 19 (VAX)
///              64   52       11         3 + 13 + 3 = 19 (IEEE)
///
///           The base 16 "scientific notation" character string
///           produced by this routine will be left justified and
///           consist of a contiguous sequence of characters with one
///           of the following formats:
///
///              (1)   h h h h  ... h ^H H  ... H
///                     1 2 3 4      n  1 2      m
///
///              (2)   -h h h h  ... h ^H H  ... H
///                      1 2 3 4      n  1 2      m
///
///              (3)   h h h h  ... h ^-H H  ... H
///                     1 2 3 4      n   1 2      m
///
///              (4)   -h h h h  ... h ^-H H  ... H
///                      1 2 3 4      n   1 2      m
///
///           where
///
///              h   and  H   denote hexadecimal digits
///               i        j
///
///              '^'          denotes exponentiation ( base 16 )
///
///           and
///
///              '+' and '-'  have their usual interpretations.
///
///           The character string produced will be blank padded on
///           the right if HXSSIZ < LEN( HXSTR ).
///
///  HXSSIZ   is the length of the base 16 "scientific notation"
///           character string produced by this routine.
/// ```
///
/// # Parameters
///
/// ```text
///  STRLEN   is the maximum number of characters permitted in the
///           output string. The value of STRLEN is 255.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the output character string is not long enough to
///      contain the entire character string that was produced,
///      the string will be truncated on the right.
///
///  2)  If LEN( HXSTR ) > HXSSIZ, the output character string will
///      be blank padded on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine converts a double precision number into an equivalent
///  character string using a base 16 "scientific notation." This
///  representation allows the full precision of a number to be placed
///  in a format that is suitable for porting or archival storage.
///
///  This routine is one of a pair of routines which are used to
///  perform conversions between double precision numbers and
///  an equivalent base 16 "scientific notation" character string
///  representation:
///
///     DP2HX  -- Convert a double precision number into a base 16
///               "scientific notation" character string.
///
///     HX2DP  -- Convert a base 16 "scientific notation"
///               character string into a double precision number.
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
///  1) Convert a set of double precision numbers to their equivalent
///     character string using a base 16 "scientific notation."
///
///
///     Example code begins here.
///
///
///           PROGRAM DP2HX_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local constants.
///     C
///           INTEGER               HXSLEN
///           PARAMETER           ( HXSLEN = 40 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(HXSLEN)    STRVAL
///
///           DOUBLE PRECISION      NUMBER (8)
///
///           INTEGER               I
///           INTEGER               SVALLN
///
///     C
///     C     Assign an array of double precision numbers.
///     C
///           DATA                  NUMBER /    2.0D-9,       1.0D0,
///          .                                  -1.0D0,    1024.0D0,
///          .                               -1024.0D0,  521707.0D0,
///          .                                  27.0D0,       0.0D0 /
///
///     C
///     C     Loop over the NUMBER array, call DP2HX for each
///     C     element of NUMBER.
///     C
///           WRITE(*,*) 'number       string             length'
///           WRITE(*,*) '-----------  -----------------  ------'
///
///           DO I= 1, 8
///
///              CALL DP2HX ( NUMBER(I), STRVAL, SVALLN )
///              WRITE(*,'(E12.4,2X,A17,2X,I5)') NUMBER(I), STRVAL,
///          .                                              SVALLN
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      number       string             length
///      -----------  -----------------  ------
///       0.2000E-08  89705F4136B4A8^-7     17
///       0.1000E+01  1^1                    3
///      -0.1000E+01  -1^1                   4
///       0.1024E+04  4^3                    3
///      -0.1024E+04  -4^3                   4
///       0.5217E+06  7F5EB^5                7
///       0.2700E+02  1B^2                   4
///       0.0000E+00  0^0                    3
///
///
///     Note: the hat or caret, '^', signals an exponent.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The maximum number of characters permitted in the output
///      string is specified by the parameter STRLEN.
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
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Changed the argument names "STRING" and "LENGTH" to "HXSTR"
///         and "HXSSIZ" for consistency with other routines.
///
///         Added IMPLICIT NONE statement.
///
///         The declaration of STRLEN has been promoted to the
///         $Declarations section.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Added complete example code.
///
///         Updated $Brief_I/O, $Parameters and $Restrictions sections to
///         properly describe STRLEN.
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
pub fn dp2hx(ctx: &mut SpiceContext, number: f64, hxstr: &mut str, hxssiz: &mut i32) {
    DP2HX(
        number,
        fstr::StrBytes::new(hxstr).as_mut(),
        hxssiz,
        ctx.raw_context(),
    );
}

//$Procedure DP2HX ( D.p. number to hexadecimal string )
pub fn DP2HX(NUMBER: f64, HXSTR: &mut [u8], HXSSIZ: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut EXPSTR = [b' '; STRLEN as usize];
    let mut TMPSTR = [b' '; STRLEN as usize];
    let mut REMNDR: f64 = 0.0;
    let mut TMPNUM: f64 = 0.0;
    let mut EXPLEN: i32 = 0;
    let mut INTEXP: i32 = 0;
    let mut POSITN: i32 = 0;
    let mut RESULT: i32 = 0;
    let mut NEGTIV: bool = false;
    let mut POSTIV: bool = false;

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
    // Make a copy of the input so that it will not be changed by this
    // routine. Also, assume that we do not know the sign of the number.
    //
    TMPNUM = NUMBER;
    NEGTIV = false;
    POSTIV = false;
    //
    // Check to see what the sign of the number is, because we treat
    // negative numbers, positive numbers and zero separately. This
    // simplifies the testing in the loop boundaries a bit, and removes
    // calls to DABS() that would otherwise have been necessary.
    //
    // Set the appropriate logical flag for the sign of the input number.
    //
    if (TMPNUM < 0.0) {
        NEGTIV = true;
    } else if (TMPNUM > 0.0) {
        POSTIV = true;
    }
    //
    // If nonzero, a double precision number is first normalized,
    // so that it has a value between 1.0D0/BASE and 1.0D0 or -1.0D0
    // and -1/BASE. The hexadecimal digits in the mantissa  are found
    // by repeated applications of multiplication and truncation
    // operations. The hexadecimal digits will be in the correct order
    // when finished. The string will be left justified, and its length
    // will be set before returning.
    //
    // Calculate the exponent of the number using multiple scaling
    // levels. The different scale factors, 16**8, 16**4, and 16,
    // provide a significant speed improvement for the normalization
    // process.
    //
    INTEXP = 0;
    if NEGTIV {
        if (TMPNUM > -1.0) {
            //
            // ABS(TMPNUM) .LT. 1.0
            //
            while ((FACTR1 * TMPNUM) > -1.0) {
                //
                // Scale the number and decrement the exponent.
                //
                TMPNUM = (TMPNUM * FACTR1);
                INTEXP = (INTEXP - EXPIN1);
            }

            while ((FACTR2 * TMPNUM) > -1.0) {
                //
                // Scale the number and decrement the exponent.
                //
                TMPNUM = (TMPNUM * FACTR2);
                INTEXP = (INTEXP - EXPIN2);
            }

            while ((BASE * TMPNUM) > -1.0) {
                //
                // Scale the number and decrement the exponent.
                //
                TMPNUM = (TMPNUM * BASE);
                INTEXP = (INTEXP - EXPINB);
            }
        //
        // At this point, -1 < TMPNUM <= -1/BASE.
        //
        } else {
            //
            // ABS(TMPNUM) .GE. 1.0
            //
            while ((INVFC1 * TMPNUM) <= -1.0) {
                //
                // Scale the number and increment the exponent.
                //
                TMPNUM = (TMPNUM * INVFC1);
                INTEXP = (INTEXP + EXPIN1);
            }

            while ((INVFC2 * TMPNUM) <= -1.0) {
                //
                // Scale the number and increment the exponent.
                //
                TMPNUM = (TMPNUM * INVFC2);
                INTEXP = (INTEXP + EXPIN2);
            }

            while (TMPNUM <= -1.0) {
                //
                // Scale the number and increment the exponent.
                //
                TMPNUM = (TMPNUM * INVBAS);
                INTEXP = (INTEXP + EXPINB);
            }
            //
            // At this point, -1 < TMPNUM <= -1/BASE.
            //
        }
    } else if POSTIV {
        if (TMPNUM < 1.0) {
            //
            // ABS(TMPNUM) .LT. 1.0
            //
            while ((FACTR1 * TMPNUM) < 1.0) {
                //
                // Scale the number and decrement the exponent.
                //
                TMPNUM = (TMPNUM * FACTR1);
                INTEXP = (INTEXP - EXPIN1);
            }

            while ((FACTR2 * TMPNUM) < 1.0) {
                //
                // Scale the number and decrement the exponent.
                //
                TMPNUM = (TMPNUM * FACTR2);
                INTEXP = (INTEXP - EXPIN2);
            }

            while ((BASE * TMPNUM) < 1.0) {
                //
                // Scale the number and decrement the exponent.
                //
                TMPNUM = (TMPNUM * BASE);
                INTEXP = (INTEXP - EXPINB);
            }
        //
        // At this point, 1/BASE <= TMPNUM < 1
        //
        } else {
            //
            // ABS(TMPNUM) .GE. 1.0
            //
            while ((INVFC1 * TMPNUM) >= 1.0) {
                //
                // Scale the number and increment the exponent.
                //
                TMPNUM = (TMPNUM * INVFC1);
                INTEXP = (INTEXP + EXPIN1);
            }

            while ((INVFC2 * TMPNUM) >= 1.0) {
                //
                // Scale the number and increment the exponent.
                //
                TMPNUM = (TMPNUM * INVFC2);
                INTEXP = (INTEXP + EXPIN2);
            }

            while (TMPNUM >= 1.0) {
                //
                // Scale the number and increment the exponent.
                //
                TMPNUM = (TMPNUM * INVBAS);
                INTEXP = (INTEXP + EXPINB);
            }
            //
            // At this point, 1/BASE <= TMPNUM < 1
            //
        }
    }
    //
    // We do different things for the cases where the number to be
    // converted is positive, negative, or zero.
    //
    if NEGTIV {
        //
        // Set the beginning position.
        //
        POSITN = 1;
        //
        // Put the minus sign in place.
        //
        fstr::assign(fstr::substr_mut(&mut TMPSTR, POSITN..=POSITN), b"-");
        //
        // Start with the remainder equal to the normalized value of the
        // original number.
        //
        REMNDR = TMPNUM;
        //
        // Collect all of the digits in the string.
        //
        // This stopping test works because the base is a power of
        // 2 and the mantissa is composed of a sum of powers of 2.
        //
        while (REMNDR != 0.0) {
            //
            // -1 < REMNDR <= -1/BASE
            //
            POSITN = (POSITN + 1);
            TMPNUM = (REMNDR * BASE);
            RESULT = (TMPNUM as i32);
            REMNDR = (TMPNUM - (RESULT as f64));
            fstr::assign(
                fstr::substr_mut(&mut TMPSTR, POSITN..=POSITN),
                save.DIGITS.get(-RESULT),
            );
        }
        //
        // Put the exponent on the end of the number and update the
        // position.
        //
        INT2HX(INTEXP, &mut EXPSTR, &mut EXPLEN, ctx);
        fstr::assign(
            fstr::substr_mut(&mut TMPSTR, (POSITN + 1)..),
            &fstr::concat(EXPCHR, fstr::substr(&EXPSTR, 1..=EXPLEN)),
        );
        POSITN = ((POSITN + EXPLEN) + 1);
    } else if POSTIV {
        //
        // Set the beginning position.
        //
        POSITN = 0;
        //
        // Start with the remainder equal to the normalized value of the
        // original number.
        //
        REMNDR = TMPNUM;
        //
        // Collect all of the digits in the string.
        //
        // This stopping test works because the base is a power of
        // 2 and the mantissa is composed of a sum of powers of 2.
        //
        while (REMNDR != 0.0) {
            //
            // 1/BASE <= REMNDR < 1
            //
            POSITN = (POSITN + 1);
            TMPNUM = (REMNDR * BASE);
            RESULT = (TMPNUM as i32);
            REMNDR = (TMPNUM - (RESULT as f64));
            fstr::assign(
                fstr::substr_mut(&mut TMPSTR, POSITN..=POSITN),
                save.DIGITS.get(RESULT),
            );
        }
        //
        // Put the exponent on the end of the number and update the
        // position.
        //
        INT2HX(INTEXP, &mut EXPSTR, &mut EXPLEN, ctx);
        fstr::assign(
            fstr::substr_mut(&mut TMPSTR, (POSITN + 1)..),
            &fstr::concat(EXPCHR, fstr::substr(&EXPSTR, 1..=EXPLEN)),
        );
        POSITN = ((POSITN + EXPLEN) + 1);
    } else {
        //
        // Treat zero as a special case, because it's easier.
        //
        POSITN = 3;
        fstr::assign(fstr::substr_mut(&mut TMPSTR, 1..=3), b"0^0");
    }
    //
    // Set the value for the length of the character string produced
    // before returning.
    //
    *HXSSIZ = POSITN;
    //
    // Set the value of the output string before returning. Let the
    // Fortran string assignment deal with the left justification, and
    // the truncation on the right if HXSTR is not long enough to
    // contain all of the characters produced.
    //
    fstr::assign(HXSTR, fstr::substr(&TMPSTR, 1..=*HXSSIZ));
}
