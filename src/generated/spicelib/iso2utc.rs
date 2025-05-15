//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NMNTHS: i32 = 12;

struct SaveVars {
    MONTHS: ActualCharArray,
    IMONTH: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MONTHS = ActualCharArray::new(3, 0..=NMNTHS);
        let mut IMONTH = ActualCharArray::new(2, 1..=NMNTHS);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"???"),
                Val::C(b"JAN"),
                Val::C(b"FEB"),
                Val::C(b"MAR"),
                Val::C(b"APR"),
                Val::C(b"MAY"),
                Val::C(b"JUN"),
                Val::C(b"JUL"),
                Val::C(b"AUG"),
                Val::C(b"SEP"),
                Val::C(b"OCT"),
                Val::C(b"NOV"),
                Val::C(b"DEC"),
            ]
            .into_iter();
            MONTHS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"01"),
                Val::C(b"02"),
                Val::C(b"03"),
                Val::C(b"04"),
                Val::C(b"05"),
                Val::C(b"06"),
                Val::C(b"07"),
                Val::C(b"08"),
                Val::C(b"09"),
                Val::C(b"10"),
                Val::C(b"11"),
                Val::C(b"12"),
            ]
            .into_iter();
            IMONTH
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { MONTHS, IMONTH }
    }
}

fn DIGIT(I: i32, ASCII: &[u8]) -> bool {
    let ASCII = &ASCII[..100 as usize];
    ((intrinsics::ICHAR(fstr::substr(ASCII, I..=I)) >= intrinsics::ICHAR(b"0"))
        && (intrinsics::ICHAR(fstr::substr(ASCII, I..=I)) <= intrinsics::ICHAR(b"9")))
}

fn HYPHEN(I: i32, ASCII: &[u8]) -> bool {
    let ASCII = &ASCII[..100 as usize];
    fstr::eq(fstr::substr(ASCII, I..=I), b"-")
}

fn COLON(I: i32, ASCII: &[u8]) -> bool {
    let ASCII = &ASCII[..100 as usize];
    fstr::eq(fstr::substr(ASCII, I..=I), b":")
}

fn T(I: i32, ASCII: &[u8]) -> bool {
    let ASCII = &ASCII[..100 as usize];
    (fstr::eq(fstr::substr(ASCII, I..=I), b"T") || fstr::eq(fstr::substr(ASCII, I..=I), b"t"))
}

/// Convert ISO time strings to UTC strings.
///
/// Deprecated: This routine is deprecated because all high-level
/// time conversion routines (STR2ET, UTC2ET, TPARSE) were updated
/// to accept ISO formatted times on input.  This routine is
/// supported for purposes of backward compatibility only.
///
/// Convert date-time strings represented in the format adopted by the
/// International Standards Organization (ISO) to equivalent UTC time
/// strings recognized by the SPICELIB routine TPARSE.
///
/// # Required Reading
///
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TSTRNG     I   String representing a calendar or julian date epoch
///  UTCSTR     O   SPICELIB UTC string corresponding to TSTRNG
///  ERROR      O   Error message if something went wrong.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TSTRNG   is an input time string, containing a time string
///           in ISO format. This routine is not sensitive to
///           the case of the characters that make up TSTRNG.
///           Thus 1992-192t12:29:28 and 1992-192T12:29:28
///           are equivalent.
///
///           The ISO standard time formats are:
///
///              Year Month Day    yyyy-mm-ddThh:mm:ss[.sss...]
///                                yyyy-mm-dd
///
///              Day of Year       yyyy-dddThh:mm:ss[.sss...]
///                                yyyy-ddd
///
///           The letters y,m,d,h,m,s can stand for any digit.
///           All digits are required in these formats. Moreover
///           the year portion of these strings must be between
///           1000 and 2999 inclusive.
///
///           The length of TSTRNG should not exceed 80 characters.
///
///           We point out that the format yyyy-ddd may be
///           interpreted very differently by routine UTC2ET.
///           1992-003 is interpreted by UTC2ET as March 1, 1992
///           whereas it is interpret as January 3, 1992 by ISO2ET.
///
///           User's should be aware of these differences in
///           interpretation and exercise adequate care in their
///           programs to avoid this possible confusion.
/// ```
///
/// # Detailed Output
///
/// ```text
///  UTCSTR   is the equivalent of TSTRNG, expressed in a UTC
///           time string that can be parsed by the SPICELIB
///           routine TPARSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the string is interpreted as an ISO format string and
///      the year portion is not within the range [1000, 2999], the
///      error SPICE(YEAROUTOFBOUNDS) is signaled. UTCSTR is
///      not changed.
///
///  2)  If the string does not clearly match the ISO format,
///      the error SPICE(NOTISOFORMAT) is signaled. UTCSTR is not
///      changed.
/// ```
///
/// # Particulars
///
/// ```text
///  The input string is converted to a UTC time string as defined
///  by the SPICELIB routine TPARSE.
/// ```
///
/// # Examples
///
/// ```text
///  To convert the time string 1992-04-03T14:12:28 to the
///  corresponding ephemeris time, execute the following instructions:
///
///     TSTRNG = '1992-04-03T14:12:28'
///
///     CALL ISO2UTC ( TSTRNG, UTCSTR, ERROR )
///
///     CALL TPARSE ( UTCSTR, UTCSEC, ERROR )
///
///     CALL DELTET ( UTCSEC, 'UTC', DELTA )
///
///     ET = DELTA + UTCSEC
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Jespersen and J. Fitz-Randolph, "From Sundials to Atomic
///       Clocks, Understanding Time and Frequency," Dover
///       Publications, Inc. New York, 1877.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  B.V. Semenov       (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 23-DEC-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Remove obsolete
///         reference to NAIF document 268 from $Literature_References.
///
/// -    SPICELIB Version 1.1.2, 28-FEB-2008 (BVS)
///
///         Corrected the contents of the $Required_Reading section.
///
/// -    SPICELIB Version 1.1.1, 19-SEP-2006 (EDW)
///
///         Added text to previously empty $Restrictions section.
///
/// -    SPICELIB Version 1.0.0, 11-JUL-1995 (KRG) (JML)
///
///         Based on
///
///         EKLIB Version 1.1.0, 11-JUL-1995 (KRG)
///
///            Fixed a typo in the $Detailed_Output section of the header.
///            The output variable was listed as ET when it should have
///            been UTCSTR.
///
///            Changed the length of ASCII to be 100 rather than 128. This
///            removes possible compiler warning messages for truncating
///            character variables on assignments. The maximum nonblank
///            length for an input time ISO string is 80 characters, so
///            placing it into a temporary array of 100 characters should
///            pose no difficulties.
///
///         EKLIB Version 1.0.0, 25-FEB-1993 (JML)
/// ```
pub fn iso2utc(
    ctx: &mut SpiceContext,
    tstrng: &str,
    utcstr: &mut str,
    error: &mut str,
) -> crate::Result<()> {
    ISO2UTC(
        tstrng.as_bytes(),
        fstr::StrBytes::new(utcstr).as_mut(),
        fstr::StrBytes::new(error).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ISO2UTC ( Convert ISO time strings to UTC strings. )
pub fn ISO2UTC(
    TSTRNG: &[u8],
    UTCSTR: &mut [u8],
    ERROR: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ASCII = [b' '; 100 as usize];
    let mut MYSTR = [b' '; 128 as usize];
    let mut L: i32 = 0;
    let mut M: i32 = 0;
    let mut CHANGE: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // In-line functions.
    //

    //
    // Local Variables
    //

    //
    // Initial Values
    //

    //
    // In-line Function Definitions
    //

    //
    // Standard SPICELIB exception handling
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ISO2UTC", ctx)?;
    }

    //
    // Left justify the input time string, and determine the location of
    // it's last non-blank character.  Finally make some local copies.
    //
    LJUST(TSTRNG, &mut ASCII);
    L = RTRIM(&ASCII);
    fstr::assign(&mut MYSTR, &ASCII);
    CHANGE = false;

    //
    // Next check for one of the ISO allowed formats.
    //
    if (L == 8) {
        //
        // The possible format is: yyyy-ddd.  If we get a
        // match construct the corresponding SPICE day of
        // year format using JAN  (e.g. 1991-JAN-261).
        //
        if (((((((DIGIT(1, &ASCII) && DIGIT(2, &ASCII)) && DIGIT(3, &ASCII))
            && DIGIT(4, &ASCII))
            && HYPHEN(5, &ASCII))
            && DIGIT(6, &ASCII))
            && DIGIT(7, &ASCII))
            && DIGIT(8, &ASCII))
        {
            fstr::assign(
                &mut MYSTR,
                &fstr::concat(
                    &fstr::concat(fstr::substr(&ASCII, 1..=5), b"JAN"),
                    fstr::substr(&ASCII, 5..),
                ),
            );
            CHANGE = true;
        }
    } else if (L == 10) {
        //
        // The possible format is: yyyy-mm-dd. If we get a match
        // construct the corresponding SPICE yyyy-mm-dd format.
        //
        if (((((((((DIGIT(1, &ASCII) && DIGIT(2, &ASCII)) && DIGIT(3, &ASCII))
            && DIGIT(4, &ASCII))
            && HYPHEN(5, &ASCII))
            && DIGIT(6, &ASCII))
            && DIGIT(7, &ASCII))
            && HYPHEN(8, &ASCII))
            && DIGIT(9, &ASCII))
            && DIGIT(10, &ASCII))
        {
            M = BSRCHC(fstr::substr(&ASCII, 6..=7), NMNTHS, save.IMONTH.as_arg());
            fstr::assign(
                &mut MYSTR,
                &fstr::concat(
                    &fstr::concat(fstr::substr(&ASCII, 1..=5), save.MONTHS.get(M)),
                    fstr::substr(&ASCII, 8..),
                ),
            );
            CHANGE = true;
        }
    } else if (L >= 17) {
        //
        // There are two possible formats yyyy-dddThh:mm:ss.ssssss
        //                                yyyy-mm-ddThh:mm:ss.ssssss
        // As above, if we get a match up to the first character following
        // a 'T', convert this to a standard SPICE time string.
        //
        if ((((((((((((((((DIGIT(1, &ASCII) && DIGIT(2, &ASCII)) && DIGIT(3, &ASCII))
            && DIGIT(4, &ASCII))
            && HYPHEN(5, &ASCII))
            && DIGIT(6, &ASCII))
            && DIGIT(7, &ASCII))
            && DIGIT(8, &ASCII))
            && T(9, &ASCII))
            && DIGIT(10, &ASCII))
            && DIGIT(11, &ASCII))
            && COLON(12, &ASCII))
            && DIGIT(13, &ASCII))
            && DIGIT(14, &ASCII))
            && COLON(15, &ASCII))
            && DIGIT(16, &ASCII))
            && DIGIT(17, &ASCII))
        {
            fstr::assign(
                &mut MYSTR,
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(fstr::substr(&ASCII, 1..=5), b"JAN"),
                            fstr::substr(&ASCII, 5..=8),
                        ),
                        b" ",
                    ),
                    fstr::substr(&ASCII, 10..),
                ),
            );

            CHANGE = true;
        } else if ((((((((((((((((((DIGIT(1, &ASCII) && DIGIT(2, &ASCII))
            && DIGIT(3, &ASCII))
            && DIGIT(4, &ASCII))
            && HYPHEN(5, &ASCII))
            && DIGIT(6, &ASCII))
            && DIGIT(7, &ASCII))
            && HYPHEN(8, &ASCII))
            && DIGIT(9, &ASCII))
            && DIGIT(10, &ASCII))
            && T(11, &ASCII))
            && DIGIT(12, &ASCII))
            && DIGIT(13, &ASCII))
            && COLON(14, &ASCII))
            && DIGIT(15, &ASCII))
            && DIGIT(16, &ASCII))
            && COLON(17, &ASCII))
            && DIGIT(18, &ASCII))
            && DIGIT(19, &ASCII))
        {
            M = BSRCHC(fstr::substr(&ASCII, 6..=7), NMNTHS, save.IMONTH.as_arg());
            fstr::assign(
                &mut MYSTR,
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(fstr::substr(&ASCII, 1..=5), save.MONTHS.get(M)),
                            fstr::substr(&ASCII, 8..=10),
                        ),
                        b" ",
                    ),
                    fstr::substr(&ASCII, 12..),
                ),
            );

            CHANGE = true;
        }
    }

    //
    // If we didn't make some change to the input string, it's NOT
    // an ISO format string. Say so in an error message and return.
    //
    if !CHANGE {
        fstr::assign(ERROR, b"The input string does not match the format expected of ISO time strings. The acceptable formats are: yyyy-ddd, yyyy-mm-dd, yyyy-dddThh:mm:ss[.ss...], and yyyy-mm-ddThh:mm:ss[.ss...].  The input string was #. ");

        REPMC(&ERROR.to_vec(), b"#", fstr::substr(&MYSTR, 1..=L), ERROR);
        CHKOUT(b"ISO2UTC", ctx)?;
        return Ok(());
    }

    //
    // Check for a year out of the range from 1000 to 2999
    //
    if (CHANGE
        && (fstr::lt(fstr::substr(&ASCII, 1..=4), b"1000")
            || fstr::gt(fstr::substr(&ASCII, 1..=4), b"2999")))
    {
        fstr::assign(ERROR, b"Years outside the range from 1000 to 2999 are not supported in SPICE-ISO format. You\'ve supplied a time string of the form # ... ");
        REPMC(&ERROR.to_vec(), b"#", fstr::substr(&ASCII, 1..=7), ERROR);
        CHKOUT(b"ISO2UTC", ctx)?;
        return Ok(());
    }

    //
    // That's it.
    //
    fstr::assign(ERROR, b" ");

    fstr::assign(UTCSTR, &MYSTR);

    CHKOUT(b"ISO2UTC", ctx)?;

    Ok(())
}
