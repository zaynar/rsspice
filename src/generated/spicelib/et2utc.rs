//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    FMT: Vec<u8>,
    ENDSTR: Vec<u8>,
    STR: Vec<u8>,
    FRACT: Vec<u8>,
    TAI: f64,
    FRCSEC: f64,
    SCALE: f64,
    TVEC: StackArray<f64, 8>,
    WHLSEC: f64,
    DAY: i32,
    HOUR: i32,
    MINUTE: i32,
    MONTH: i32,
    SECOND: i32,
    YEAR: i32,
    MYPREC: i32,
    BDAY: i32,
    BHR: i32,
    BMN: i32,
    BMONTH: i32,
    BSC: i32,
    EDAY: i32,
    EHR: i32,
    EMN: i32,
    EMONTH: i32,
    ESC: i32,
    I: i32,
    MTHNAM: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FMT = vec![b' '; 4 as usize];
        let mut ENDSTR = vec![b' '; 80 as usize];
        let mut STR = vec![b' '; 80 as usize];
        let mut FRACT = vec![b' '; 80 as usize];
        let mut TAI: f64 = 0.0;
        let mut FRCSEC: f64 = 0.0;
        let mut SCALE: f64 = 0.0;
        let mut TVEC = StackArray::<f64, 8>::new(1..=8);
        let mut WHLSEC: f64 = 0.0;
        let mut DAY: i32 = 0;
        let mut HOUR: i32 = 0;
        let mut MINUTE: i32 = 0;
        let mut MONTH: i32 = 0;
        let mut SECOND: i32 = 0;
        let mut YEAR: i32 = 0;
        let mut MYPREC: i32 = 0;
        let mut BDAY: i32 = 0;
        let mut BHR: i32 = 0;
        let mut BMN: i32 = 0;
        let mut BMONTH: i32 = 0;
        let mut BSC: i32 = 0;
        let mut EDAY: i32 = 0;
        let mut EHR: i32 = 0;
        let mut EMN: i32 = 0;
        let mut EMONTH: i32 = 0;
        let mut ESC: i32 = 0;
        let mut I: i32 = 0;
        let mut MTHNAM = ActualCharArray::new(3, 1..=12);

        {
            use f2rust_std::data::Val;

            let mut clist = [
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
            MTHNAM
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            FMT,
            ENDSTR,
            STR,
            FRACT,
            TAI,
            FRCSEC,
            SCALE,
            TVEC,
            WHLSEC,
            DAY,
            HOUR,
            MINUTE,
            MONTH,
            SECOND,
            YEAR,
            MYPREC,
            BDAY,
            BHR,
            BMN,
            BMONTH,
            BSC,
            EDAY,
            EHR,
            EMN,
            EMONTH,
            ESC,
            I,
            MTHNAM,
        }
    }
}

fn NDIGIT(DAY: i32) -> i32 {
    (((intrinsics::MIN0(&[1, (DAY / 1000)]) + intrinsics::MIN0(&[1, (DAY / 100)]))
        + intrinsics::MIN0(&[1, (DAY / 10)]))
        + 1)
}

/// Ephemeris Time to UTC
///
/// Convert an input time from ephemeris seconds past J2000
/// to Calendar, Day-of-Year, or Julian Date format, UTC.
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
///  ET         I   Epoch, given in ephemeris seconds past J2000.
///  FORMAT     I   Format of output epoch.
///  PREC       I   Digits of precision in fractional seconds or days.
///  UTCSTR     O   Output time string, UTC.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is the input epoch, ephemeris seconds past J2000.
///
///  FORMAT   is the format of the output time string. It may be
///           any of the following:
///
///              'C'      Calendar format, UTC.
///
///              'D'      Day-of-Year format, UTC.
///
///              'J'      Julian Date format, UTC.
///
///              'ISOC'   ISO Calendar format, UTC.
///
///              'ISOD'   ISO Day-of-Year format, UTC.
///
///  PREC     is the number of digits of precision to which
///           fractional seconds (for Calendar and Day-of-Year
///           formats) or days (for Julian Date format) are to
///           be computed. If PREC is zero or smaller, no decimal
///           point is appended to the output string. If PREC is
///           greater than 14, it is treated as 14.
/// ```
///
/// # Detailed Output
///
/// ```text
///  UTCSTR   is the output time string equivalent to the input
///           epoch, in the specified format. Some examples are
///           shown below.
///
///              'C'      '1986 APR 12 16:31:09.814'
///              'D'      '1986-102 // 16:31:12.814'
///              'J'      'JD 2446533.18834276'
///              'ISOC'   '1987-04-12T16:31:12.814'
///              'ISOD'   '1987-102T16:31:12.814'
///
///           If an error occurs, UTCSTR is not changed.
///
///           Fractional seconds, or for Julian dates, fractional
///           days, are rounded to the precision level specified
///           by the input argument PREC.
///
///           UTCSTR should be declared to be at least
///           20 + PREC characters in length to ensure
///           sufficient room to hold calendar strings
///           for modern epochs. For epochs prior to
///           1000 A.D. at least 24 + PREC characters in
///           length are required to hold the output
///           calendar string.
///
///           For epochs prior to 1000 A.D. Jan 1 calendar
///           and day of year formats are returned with the
///           era (A.D. or B.C.) attached to the year. For
///           example
///
///              '877 A.D. MAR 17 13:29:11.829'
///              '471 B.C. Jan 01 12:00:00.000'
///              '471 B.C. 001 // 12:00:00.000'
///
///           ISO formats do not support the inclusion of an era.
///           For years prior to 1 A.D. an error will be signaled
///           if ISO format has been requested.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the format for the output string is not recognized, the
///      error SPICE(INVALIDTIMEFORMAT) is signaled.
///
///  2)  If PREC is less than or equal to zero, it is treated as
///      zero. If PREC is greater than 14, it is treated as 14.
///
///  3)  If one of the ISO formats is specified (ISOC or ISOD) but the
///      year corresponding to ET is prior to 1 A.D. on the Gregorian
///      Calendar, the error SPICE(YEAROUTOFRANGE) is signaled.
///
///  4)  Epochs prior to 15 Oct, 1582 on the Gregorian calendar (the
///      calendar commonly used in western societies) are returned in
///      the "extended" Gregorian Calendar. To convert epochs to the
///      Julian calendar see the SPICELIB routine GR2JUL.
///
///  5)  This routine does not attempt to account for variations
///      in the length of the second that were in effect prior
///      to Jan 1, 1972. For days prior to that date, we assume
///      there are exactly 86400 ephemeris seconds. Consequently
///      the UTC Gregorian calendar strings produced for epochs
///      prior to Jan 1, 1972 differ from the corresponding
///      TDB calendar strings by approximately 41.18 seconds.
///      (TDB Gregorian calendar strings are produced by the
///      routine ETCAL).
///
///  6)  If a leapseconds kernel has not been loaded prior to calling
///      this routine, an error is signaled by a routine in the
///      call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  A leapseconds kernel must be loaded via FURNSH prior to calling
///  this routine. The kernel need be loaded only once during a program
///  run.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine handles the task of converting a double precision
///  representation of an epoch to a character string suitable for
///  human consumption. The more general routine TIMOUT may also be
///  used to convert ET to time strings.
/// ```
///
/// # Examples
///
/// ```text
///  Let the value of ET be -527644192.5403653 ephemeris seconds
///  past J2000. Assuming that the nominal values in the kernel pool
///  have not been altered, the following calls
///
///     CALL ET2UTC ( ET, 'C', 0, UTCSTR )
///     CALL ET2UTC ( ET, 'C', 3, UTCSTR )
///     CALL ET2UTC ( ET, 'D', 5, UTCSTR )
///     CALL ET2UTC ( ET, 'J', 7, UTCSTR )
///
///  produce the following output strings
///
///     '1983 APR 13 12:09:14'
///     '1983 APR 13 12:09:14.274'
///     '1983-103 // 12:09:14.27400'
///     'JD 2445438.0064152'
///
///  respectively.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Jespersen and J. Fitz-Randolph, "From Sundials to Atomic
///       Clocks, Understanding Time and Frequency," Dover
///       Publications, Inc. New York, 1982.
/// ```
///
/// # Author and Institution
///
/// ```text
///  C.H. Acton         (JPL)
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  J.M. Lynch         (JPL)
///  W.M. Owen          (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.0.5, 24-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Extended $Files
///         section.
///
/// -    SPICELIB Version 3.0.4, 06-APR-2009 (NJB)
///
///         Header was updated to state that fractional
///         seconds or days are rounded in the output
///         string.
///
/// -    SPICELIB Version 3.0.3, 28-JAN-2008 (BVS)
///
///         Fixed typo in the ISOC example string in $Detailed_Output.
///
/// -    SPICELIB Version 3.0.2, 29-JUL-2003 (NJB) (CHA)
///
///         Various header changes were made to improve clarity and
///         more fully explain the routine's functionality.
///
/// -    SPICELIB Version 3.0.1, 14-SEP-2000 (EDW)
///
///         Added FAILED check after TTRANS call during the calendar "C"
///         format processing to catch failure signal from TTRANS.
///         Lack of this check caused CSPICE based programs to core dump
///         if ET2UTC was called without a leapseconds kernel while
///         error action was set to RETURN.
///
/// -    SPICELIB Version 3.0.0, 13-MAR-1996 (WLT)
///
///         The construction of the numerical components of the
///         output string are now handled by the SPICELIB routine
///         TTRANS.
///
///         In addition the routine now supports the ISO formats and
///         the era associated with an epoch (B.C. or A.D.) in non
///         ISO formats.
///
/// -    SPICELIB Version 2.1.0, 11-JUL-1995 (KRG)
///
///         Removed some potential compile warnings that could be caused
///         by truncation of double precision values to integers through
///         a direct assignment. The direct assignment has been replaced
///         with a call to the intrinsic function IDINT.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 2.0.0, 21-MAR-1991 (NJB) (JML)
///
///         Two bugs involving rounding errors were corrected. One of
///         the bugs caused conversion errors of magnitude as large as
///         1 second. See $Revisions for details.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.0.0, 13-MAR-1996 (WLT)
///
///         The construction of the numerical components of the
///         output string are now handled by the SPICELIB routine
///         TTRANS.
///
///         In addition the routine now supports the era associated
///         with an epoch (B.C. or A.D.)
///
/// -    SPICELIB Version 2.1.0, 11-JUL-1995 (KRG)
///
///         Removed some potential compile warnings that could be
///         caused by truncation of double precision values to
///         integers through a direct assignment. The direct
///         assignment has been replaced with a call to the
///         intrinsic function IDINT.
///
/// -    SPICELIB Version 2.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was
///         added following the header.
///
/// -    SPICELIB Version 2.0.0, 21-MAR-1991 (NJB) (JML)
///
///         1) In the previous version of this routine, the algorithm
///         that was used permitted inconsistent conversion of the
///         integer and fractional parts of the UTC value
///         corresponding to the input ET value.
///
///            In the case where rounding the double precision UTC
///            time corresponding to the input ET value to PREC
///            decimal places resulted in a carry (to the integer
///            portion of the UTC value), the integer portion of the
///            UTC value was treated correctly, but the fraction was
///            not always rounded correctly. The specific case where
///            the problem occurred was when the input ET value
///            mapped to a UTC time having a fractional part that
///            rounded up to 1.0 when rounded PREC decimal places,
///            but that did not round up to 1.0 when rounded to the
///            nearest PREC+1 decimal places. The set of such
///            fractions can be represented as
///
///              { 1 - EPSILON :      EPSILON  <  5 * ( 10 **
///              -(PREC+1) ) - and
///
///                                   EPSILON  >  5 * ( 10 **
///                                   -(PREC+2) )
///
///                                                                      }
///
///            For example, if the input ET mapped to the UTC time
///
///               2 JAN 1991 00:34:12.99994,
///
///            then a call to this routine with PREC set to 3 would
///            result in the output
///
///               2 JAN 1991 00:34:13.999
///
///            instead of the correct value
///
///               2 JAN 1991 00:34:13.000
///
///            On the other hand, if the input ET mapped to the UTC
///            time
///
///               2 JAN 1991 00:34:12.99996,
///
///            then a call to this routine with PREC set to 3 would
///            result in the correct output.
///
///            This error was apparently difficult to generate: it
///            has never been reported by any SPICELIB users, and was
///            eventually discovered by NAIF staff.
///
///         2) The second bug is somewhat less severe, as far as the
///         magnitude of the error is concerned. However, it's
///         easier to generate this error. Namely, in some cases,
///         the fractional part of the input ET value is rounded to
///         PREC SIGNIFICANT DIGITS, rather than to PREC decimal
///         places. The effect of this is that the fraction is
///         occasionally truncated rather than rounded. For example,
///         the ET value equivalent to the UTC string
///
///               1991 JAN 2 00:34:12.0009
///
///            would be converted to
///
///               1991 JAN 2 00:34:12.000
///
///            instead of the correct value
///
///               1991 JAN 2 00:34:12.001
///
///            when the input argument PREC was set equal to 3.
///
///         The modifications made to solve these problems are as
///         follows:
///
///            1)  The input ET value, after conversion to `UTC
///                seconds past 2000', is broken up into the sum of a
///                whole number of seconds and a non-negative,
///                fractional number of seconds. The fact that the
///                fractional part is non-negative simplifies the
///                conversion of the fraction.
///
///            2)  The fraction is rounded to PREC decimal places---
///                that is, to the nearest integer multiple of
///                10**(-PREC). If the rounding results in a carry,
///                the whole number portion of the time value is
///                incremented by 1 second. After this step, the
///                whole number of seconds correctly accounts for
///                any necessary rounding of the fraction.
///
///            3)  The whole number portion of the time value is
///            passed through the inverse Muller-Wimberly algorithm
///            to obtain years, months, days, hours, minutes, and
///            whole seconds. A small fraction is added to the whole
///            number to prevent round-off error from occurring when
///            divisions are performed.
///
///            4)  The fraction is converted to a string using the
///                SPICELIB routine DPSTRF. To ensure that DPSTRF
///                produces an output string containing PREC decimal
///                places, an integer is added to the fraction value
///                before supplying it to DPSTRF. This integer
///                `anchors' the first significant digit of the input
///                value in the units place.
/// ```
pub fn et2utc(
    ctx: &mut SpiceContext,
    et: f64,
    format: &str,
    prec: i32,
    utcstr: &mut str,
) -> crate::Result<()> {
    ET2UTC(
        et,
        format.as_bytes(),
        prec,
        fstr::StrBytes::new(utcstr).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure ET2UTC ( Ephemeris Time to UTC )
pub fn ET2UTC(
    ET: f64,
    FORMAT: &[u8],
    PREC: i32,
    UTCSTR: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local Functions (Statement Functions)
    //

    //
    // Local variables
    //

    //
    // Save everything between calls
    //

    //
    // Initial values
    //

    //
    // The function NDIGIT gives the number of digits required to
    // display a non-negative integer that is less than 10000
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ET2UTC", ctx)?;

    //
    // Convert FORMAT to uppercase for ease of comparison. Make sure it's
    // one of the recognized formats.
    //
    UCASE(FORMAT, &mut save.FMT, ctx);

    if ((((fstr::ne(&save.FMT, b"J") && fstr::ne(&save.FMT, b"C")) && fstr::ne(&save.FMT, b"D"))
        && fstr::ne(&save.FMT, b"ISOD"))
        && fstr::ne(&save.FMT, b"ISOC"))
    {
        SETMSG(b"ET2UTC: Format specification for output time string is not recognized. Valid specifications are: \'C\', \'D\', \'J\', \'ISOC\', or \'ISOD\'. The supplied format was \'#\'. ", ctx);

        ERRCH(b"#", FORMAT, ctx);
        SIGERR(b"SPICE(INVALIDTIMEFORMAT)", ctx)?;
        CHKOUT(b"ET2UTC", ctx)?;
        return Ok(());
    }
    //
    // Force PREC into an acceptable range
    //
    save.MYPREC = intrinsics::MAX0(&[0, intrinsics::MIN0(&[14, PREC])]);

    //
    // If the output is Julian Date, we're ready to go. Remember that
    // the day part of Julian Date already has seven digits built in.
    //
    if fstr::eq(&save.FMT, b"J") {
        save.TVEC[1] = ET;

        TTRANS(b"TDB", b"JDUTC", save.TVEC.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ET2UTC", ctx)?;
            return Ok(());
        }

        DPSTRF(save.TVEC[1], (save.MYPREC + 7), b"F", &mut save.STR, ctx);
        PREFIX(b"JD", 0, &mut save.STR);

        fstr::assign(UTCSTR, &save.STR);

        CHKOUT(b"ET2UTC", ctx)?;
        return Ok(());
    }
    //
    // If we've dropped past the IF-THEN block above, we need
    // to construct a calendar format string. First thing to
    // do is convert from ET to TAI.
    //

    save.TAI = UNITIM(ET, b"TDB", b"TAI", ctx)?;

    //
    // We're going to break up TAI into an integer and a
    // fractional part.  The integer will be the greatest
    // integer less than or equal to TAI, and the fraction
    // will be the difference between TAI and the integer
    // part.  The fraction will always be in the interval
    //
    //    [0, 1)
    //
    // After making this decomposition, we'll adjust the integer
    // and fraction to take rounding into account.  The result
    // of the adjustment is that the fraction will be an integer
    // number of time units of length 10**(-MYPREC) seconds, where
    // the integer is in the range [0, (10**MYPREC)-1].  If the
    // fraction rounds up to 1, the fraction will be set to zero,
    // and the whole number portion of TAI will be incremented.
    //
    // Since the integers involved may be too large to represent
    // using the INTEGER data type, we'll represent them with
    // double precision numbers.  We'll use the intrinsic ANINT
    // function to keep round-off from creeping into these d.p.
    // numbers representing integers.
    //
    // Find the greatest integer less than or equal to TAI.
    // Recall that INT truncates toward the origin.  If TAI
    // is negative and is not already an integer, the result we
    // want is one less than AINT(TAI).
    //
    save.WHLSEC = f64::trunc(save.TAI);

    if ((save.TAI < 0.0) && (save.TAI != save.WHLSEC)) {
        save.WHLSEC = (save.WHLSEC - 1.0);
    }

    //
    // The fractional part of TAI must be rounded to the
    // nearest multiple of 10**(-MYPREC).  Fractions that are
    // equidistant from two multiples of 10**(-MYPREC) are
    // rounded up.
    //
    // To accomplish the rounding, we scale the fraction by
    // 10**MYPREC.
    //
    //
    save.SCALE = f64::round(f64::powi(10.0, save.MYPREC));
    save.FRCSEC = f64::round((save.SCALE * (save.TAI - save.WHLSEC)));

    //
    // If a carry occurred, the fraction becomes zero, and
    // we must increment WHLSEC.
    //
    if (save.FRCSEC == save.SCALE) {
        save.WHLSEC = (save.WHLSEC + 1.0);
        save.FRCSEC = 0.0;
    }

    save.FRCSEC = (save.FRCSEC / save.SCALE);

    //
    // Now, we let TTRANS handle the transformation to
    // the desired components for output.
    //
    // FRCSEC will be converted to a string containing MYPREC digits.
    // This will be done later on when the output string is
    // assembled.
    //
    save.TVEC[1] = save.WHLSEC;

    if (fstr::eq(&save.FMT, b"C") || fstr::eq(&save.FMT, b"ISOC")) {
        TTRANS(b"TAI", b"YMD", save.TVEC.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ET2UTC", ctx)?;
            return Ok(());
        }

        save.YEAR = intrinsics::IDNINT(save.TVEC[1]);
        save.MONTH = intrinsics::IDNINT(save.TVEC[2]);
        save.DAY = intrinsics::IDNINT(save.TVEC[3]);
        save.HOUR = intrinsics::IDNINT(save.TVEC[4]);
        save.MINUTE = intrinsics::IDNINT(save.TVEC[5]);
        save.SECOND = intrinsics::IDNINT(save.TVEC[6]);
        //
        // The beginning of the string is going to be the year.
        // Depending upon the size of the year, it may or
        // may not have an era label.  However the end of the
        // string has a fixed size.  We set up that portion of the
        // string now.  First fill in the month...
        //
        if fstr::eq(&save.FMT, b"C") {
            fstr::assign(&mut save.ENDSTR, b" MMM 00 00:00:00");
            fstr::assign(
                fstr::substr_mut(&mut save.ENDSTR, 2..=4),
                save.MTHNAM.get(save.MONTH),
            );
            //
            // ... and then fill in the day portion of the string.
            //
            save.EDAY = 7;
            save.BDAY = ((save.EDAY - NDIGIT(save.DAY)) + 1);

            INTSTR(
                save.DAY,
                fstr::substr_mut(&mut save.ENDSTR, save.BDAY..=save.EDAY),
                ctx,
            );
            save.EHR = 10;
            save.EMN = 13;
            save.ESC = 16;
        } else {
            fstr::assign(&mut save.ENDSTR, b"-0M-00T00:00:00");
            save.EDAY = 6;
            save.BDAY = ((save.EDAY - NDIGIT(save.DAY)) + 1);

            save.EMONTH = 3;
            save.BMONTH = ((save.EMONTH - NDIGIT(save.MONTH)) + 1);
            INTSTR(
                save.MONTH,
                fstr::substr_mut(&mut save.ENDSTR, save.BMONTH..=save.EMONTH),
                ctx,
            );
            INTSTR(
                save.DAY,
                fstr::substr_mut(&mut save.ENDSTR, save.BDAY..=save.EDAY),
                ctx,
            );
            save.EHR = 9;
            save.EMN = 12;
            save.ESC = 15;
        }
    } else {
        //
        // We must have day of year format.  Convert TAI to that
        // format.
        //

        TTRANS(b"TAI", b"YD", save.TVEC.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ET2UTC", ctx)?;
            return Ok(());
        }

        save.YEAR = intrinsics::IDNINT(save.TVEC[1]);
        save.MONTH = 1;
        save.DAY = intrinsics::IDNINT(save.TVEC[2]);
        save.HOUR = intrinsics::IDNINT(save.TVEC[3]);
        save.MINUTE = intrinsics::IDNINT(save.TVEC[4]);
        save.SECOND = intrinsics::IDNINT(save.TVEC[5]);
        //
        // As in the previous case, the end of the output string will
        // have a fixed size.  We fill in the day portion of the string
        // now.  Note that we have set things up so that the hour,
        // minutes and seconds appear in the same location in both
        // day of year and calendar format of strings.
        //
        if fstr::eq(&save.FMT, b"D") {
            fstr::assign(&mut save.ENDSTR, b"-000 // 00:00:00");
            save.EDAY = 4;
            save.BDAY = ((save.EDAY - NDIGIT(save.DAY)) + 1);

            INTSTR(
                save.DAY,
                fstr::substr_mut(&mut save.ENDSTR, save.BDAY..=save.EDAY),
                ctx,
            );
            save.EHR = 10;
            save.EMN = 13;
            save.ESC = 16;
        } else {
            fstr::assign(&mut save.ENDSTR, b"-000T00:00:00");
            save.EDAY = 4;
            save.BDAY = ((save.EDAY - NDIGIT(save.DAY)) + 1);

            INTSTR(
                save.DAY,
                fstr::substr_mut(&mut save.ENDSTR, save.BDAY..=save.EDAY),
                ctx,
            );
            save.EHR = 7;
            save.EMN = 10;
            save.ESC = 13;
        }
    }
    //
    // Fill out the hours, minutes and integer portion of
    // seconds in the output string.
    //
    save.BHR = ((save.EHR - NDIGIT(save.HOUR)) + 1);
    save.BMN = ((save.EMN - NDIGIT(save.MINUTE)) + 1);
    save.BSC = ((save.ESC - NDIGIT(save.SECOND)) + 1);

    INTSTR(
        save.HOUR,
        fstr::substr_mut(&mut save.ENDSTR, save.BHR..=save.EHR),
        ctx,
    );
    INTSTR(
        save.MINUTE,
        fstr::substr_mut(&mut save.ENDSTR, save.BMN..=save.EMN),
        ctx,
    );
    INTSTR(
        save.SECOND,
        fstr::substr_mut(&mut save.ENDSTR, save.BSC..=save.ESC),
        ctx,
    );
    //
    // Append the fractional part of the seconds component.
    //
    if (save.MYPREC > 0) {
        //
        // DPSTRF gives MYPREC significant digits in the output,
        // not necessarily MYPREC digits to the right of the
        // decimal point.  We will add a one-digit integer to
        // FRCSEC to `anchor' the first significant digit of
        // FRCSEC in a known place.  That way, we can get DPSTRF
        // to give us a known number of digits after the decimal
        // point.
        //
        // The integer part of FRCSEC will not affect the output
        // string.
        //
        save.FRCSEC = (save.FRCSEC + 1.0);

        DPSTRF(save.FRCSEC, (save.MYPREC + 1), b"F", &mut save.FRACT, ctx);

        save.I = intrinsics::INDEX(&save.FRACT, b".");
        fstr::assign(
            fstr::substr_mut(&mut save.ENDSTR, (save.ESC + 1)..),
            fstr::substr(&save.FRACT, save.I..=(save.I + save.MYPREC)),
        );
    }
    //
    // The end of the time string is now complete.  We need to
    // construct the year portion of the string. We are going to
    // append an era if the year is before 1000 A.D. Note that
    // we make sure the first character in the ending string
    // is a blank (' ') if the era is to be attached.  Otherwise
    // we'd get confusing day of year formats like
    // 999 A.D.-019 // 12:13:18.
    //
    if (save.YEAR >= 1000) {
        INTSTR(save.YEAR, &mut save.STR, ctx);
    } else if (save.YEAR > 0) {
        INTSTR(save.YEAR, &mut save.STR, ctx);

        if (fstr::eq(&save.FMT, b"C") || fstr::eq(&save.FMT, b"D")) {
            SUFFIX(b"A.D.", 1, &mut save.STR);
            fstr::assign(fstr::substr_mut(&mut save.ENDSTR, 1..=1), b" ");
        }
    } else if (save.YEAR <= 0) {
        if (fstr::eq(&save.FMT, b"C") || fstr::eq(&save.FMT, b"D")) {
            save.YEAR = (-save.YEAR + 1);

            INTSTR(save.YEAR, &mut save.STR, ctx);
            SUFFIX(b"B.C.", 1, &mut save.STR);

            fstr::assign(fstr::substr_mut(&mut save.ENDSTR, 1..=1), b" ");
        } else {
            save.YEAR = (-save.YEAR + 1);
            SETMSG(b"The year of the ET epoch supplied is # B.C.  Years in this era are not supported in ISO format. ", ctx);
            ERRINT(b"#", save.YEAR, ctx);
            SIGERR(b"SPICE(YEAROUTOFRANGE)", ctx)?;
            CHKOUT(b"ET2UTC", ctx)?;
            return Ok(());
        }
    }

    //
    // Finally append the ENDSTR to STR to get the fully formatted
    // string.
    //
    SUFFIX(&save.ENDSTR, 0, &mut save.STR);

    fstr::assign(UTCSTR, &save.STR);

    CHKOUT(b"ET2UTC", ctx)?;
    Ok(())
}
