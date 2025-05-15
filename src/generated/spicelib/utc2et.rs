//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const ERA: i32 = 1;
const WDAY: i32 = (ERA + 1);
const ZONE: i32 = (WDAY + 1);
const AMPM: i32 = (ZONE + 1);
const SYSTEM: i32 = (AMPM + 1);
const LONGLN: i32 = 480;
const LNSIZE: i32 = 80;
const SMWDSZ: i32 = 8;

/// UTC to Ephemeris Time
///
/// Convert an input time from Calendar or Julian Date format, UTC,
/// to ephemeris seconds past J2000.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UTCSTR     I   Input time string, UTC.
///  ET         O   Output epoch, ephemeris seconds past J2000.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UTCSTR   is an input time string, containing a Calendar or Julian
///           Date, UTC. Any input string acceptable to the SPICELIB
///           routine TPARTV are acceptable to UTC2ET. The length of
///           UTCSTR should not exceed 80 characters.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ET       is the equivalent of UTCSTR, expressed in ephemeris
///           seconds past J2000. If an error occurs, or if the
///           input string is ambiguous, ET is not changed.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input time string is ambiguous, the error
///      SPICE(INVALIDTIMESTRING) is signaled.
///
///  2)  This routine does not attempt to account for variations
///      in the length of the second that were in effect prior
///      to Jan 1, 1972. For days prior to that date, we assume
///      there are exactly 86400 ephemeris seconds.
/// ```
///
/// # Particulars
///
/// ```text
///  Note: NAIF recommends the use of STR2ET instead of UTC2ET.
///
///  This routine handles that task of converting strings
///  representing epochs in the UTC system to ephemeris seconds
///  (TDB) past the epoch of the J2000 frame.
///
///  Although this routine is intended primarily for the
///  conversion of epochs during the "space age" it may also
///  be used to convert much earlier epochs. However, before
///  using this routine to convert epochs prior to 1972
///  you must be sure that the assumptions made by in the
///  implementation are consistent with the accuracy of
///  the input calendar string.
///
///  As noted in the $Exceptions section, this routine does not attempt
///  to account for variations in the length of the second that were
///  used prior to Jan 1, 1972. Instead each "UTC" day prior to
///  Jan 1, 1972 is assumed to have exactly 86400 TDT seconds.
///
///  Ancient Epochs
///  --------------
///
///  The calendar used today, the Gregorian calendar, has its
///  initial epoch on 15 October, 1582. Prior to that epoch the
///  Julian calendar was used for the recording of epochs.
///  October 15, 1582 (Gregorian) corresponds to
///  October 05, 1582 (Julian). From the point of view of the
///  implementation of this routine, all epochs belong to the
///  Gregorian calendar extended indefinitely backward in time.
///  If you need to obtain ephemeris seconds past the J2000 epoch
///  from Julian Calendar strings, we suggest that
///  you make use of the SPICELIB routine STR2ET.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Below is a sampling of some of the time formats that
///     are acceptable as inputs to UTC2ET. A complete discussion
///     of permissible formats is given in the SPICELIB routine
///     TPARTV as well as the User's reference file time.req
///     located in the "doc" directory of the toolkit.
///
///
///     ISO (T) Formats (with or without trailing "Z").
///
///     String                        Year Mon  DOY DOM  HR Min Sec
///     ----------------------------  ---- ---  --- ---  -- --- ------
///     1996-12-18T12:28:28           1996 Dec   na  18  12  28 28
///     1986-01-18T12                 1986 Jan   na  18  12  00 00
///     1986-01-18T12:19              1986 Jan   na  18  12  19 00
///     1986-01-18T12:19:52.18Z       1986 Jan   na  18  12  19 52.18
///     1995-08T18:28:12Z             1995  na  008  na  18  28 12
///     1995-18T                      1995  na  018  na  00  00 00
///     0000-01-01T                   1 BC Jan   na  01  00  00 00
///
///
///     Calendar Formats.
///
///     String                        Year   Mon DOM  HR Min  Sec
///     ----------------------------  ----   --- ---  -- ---  ------
///     Tue Aug  6 11:10:57  1996     1996   Aug  06  11  10  57
///     1 DEC 1997 12:28:29.192       1997   Dec  01  12  28  29.192
///     2/3/1996 17:18:12.002         1996   Feb  03  17  18  12.002
///     Mar 2 12:18:17.287 1993       1993   Mar  02  12  18  17.287
///     1992 11:18:28  3 Jul          1992   Jul  03  11  18  28
///     June 12, 1989 01:21           1989   Jun  12  01  21  00
///     1978/3/12 23:28:59.29         1978   Mar  12  23  28  59.29
///     17JUN1982 18:28:28            1982   Jun  17  18  28  28
///     13:28:28.128 1992 27 Jun      1992   Jun  27  13  28  28.128
///     1972 27 jun 12:29             1972   Jun  27  12  29  00
///     '93 Jan 23 12:29:47.289       1993*  Jan  23  12  29  47.289
///     27 Jan 3, 19:12:28.182        2027*  Jan  03  19  12  28.182
///     23 A.D. APR 4, 18:28:29.29    0023** Apr  04  18  28  29.29
///     18 B.C. Jun 3, 12:29:28.291   -017** Jun  03  12  29  28.291
///     29 Jun  30 12:29:29.298       2029+  Jun  30  12  29  29.298
///     29 Jun '30 12:29:29.298       2030*  Jun  29  12  29  29.298
///
///
///     Day of Year Formats
///
///     String                        Year  DOY HR Min Sec
///     ----------------------------  ----  --- -- --- ------
///     1997-162::12:18:28.827        1997  162 12  18 28.827
///     162-1996/12:28:28.287         1996  162 12  28 28.287
///     1993-321/12:28:28.287         1993  231 12  28 28.287
///     1992 183// 12 18 19           1992  183 12  18 19
///     17:28:01.287 1992-272//       1992  272 17  28 01.287
///     17:28:01.282 272-1994//       1994  272 17  28 01.282
///     '92-271/ 12:28:30.291         1992* 271 12  28 30.291
///     92-182/ 18:28:28.281          1992* 182 18  28 28.281
///     182-92/ 12:29:29.192          0182+ 092 12  29 29.192
///     182-'92/ 12:28:29.182         1992  182 12  28 29.182
///
///
///     Julian Date Strings
///
///     jd 28272.291                  Julian Date   28272.291
///     2451515.2981 (JD)             Julian Date 2451515.2981
///     2451515.2981 JD               Julian Date 2451515.2981
///
///                                   Abbreviations Used in Tables
///
///                                      na    --- Not Applicable
///                                      Mon   --- Month
///                                      DOY   --- Day of Year
///                                      DOM   --- Day of Month
///                                      Wkday --- Weekday
///                                      Hr    --- Hour
///                                      Min   --- Minutes
///                                      Sec   --- Seconds
///
///     *  The default interpretation of a year that has been
///        abbreviated with a leading quote as in 'xy (such as '92) is
///        to treat the year as 19xy if xy > 68 and to treat it is
///        20xy otherwise. Thus '69 is interpreted as 1969 and '68 is
///        treated as 2068. However, you may change the "split point"
///        and centuries through use of the SPICELIB routine TSETYR
///        which is an entry point in the SPICELIB module TEXPYR. See
///        that routine for a discussion of how you may reset the
///        split point.
///
///     ** All epochs are regarded as belonging to the Gregorian
///        calendar. We formally extend the Gregorian calendar
///        backward and forward in time for all epochs. If you have
///        epochs belonging to the Julian Calendar, consult the
///        SPICELIB routines TPARTV and JUL2GR for a discussion
///        concerning conversions to the Gregorian calendar and ET.
///
///     +  When a day of year format or calendar format string is
///        input and neither of the integer components of the date is
///        greater than 1000, the first integer is regarded as being
///        the year.
///
///  2) Convert a calendar date in UTC string format to Julian
///     Ephemeris date.
///
///     Use the LSK kernel below to load the leap seconds and time
///     constants required for the conversions.
///
///        naif0012.tls
///
///
///     Example code begins here.
///
///
///           PROGRAM UTC2ET_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      UNITIM
///
///     C
///     C     Local constants.
///     C
///           CHARACTER*(*)         UTCSTR
///           PARAMETER           ( UTCSTR = 'Dec 19 2003  16:48:00' )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      JED
///
///     C
///     C     Load the LSK file.
///     C
///           CALL FURNSH ( 'naif0012.tls' )
///
///     C
///     C     Convert input UTC string to Ephemeris Time.
///     C
///           CALL UTC2ET ( UTCSTR, ET )
///           WRITE(*,'(2A)') 'UTC time             : ', UTCSTR
///           WRITE(*,'(A,F22.9)') 'Ephemeris time       :', ET
///
///     C
///     C     Convert the Ephemeris Time to Julian ephemeris date,
///     C     i.e. Julian date relative to TDB time scale.
///     C
///           JED = UNITIM( ET, 'ET', 'JED' )
///           WRITE(*,'(A,F22.9)') 'Julian Ephemeris Date:', JED
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     UTC time             : Dec 19 2003  16:48:00
///     Ephemeris time       :   125124544.183560610
///     Julian Ephemeris Date:     2452993.200742865
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The conversion between ET and UTC depends on the values in the
///      input kernel pool. The kernel pool should be loaded prior to
///      calling this routine.
///
///  2)  Before using this routine for epochs prior to Jan 1, 1972
///      be sure to check the $Particulars section to make sure
///      that the assumptions made in this routine are consistent
///      with the accuracy you require for your application.
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
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.1, 13-AUG-2021 (JDR) (EDW)
///
///         Header edits to expand description of ISO format.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Added note to $Particulars section.
///
/// -    SPICELIB Version 2.1.0, 05-JAN-1998 (WLT)
///
///         Comments concerning the default century for abbreviated
///         years were updated to reflect changes to TEXPYR.
///
/// -    SPICELIB Version 2.0.0, 20-NOV-1996 (WLT)
///
///         About the only thing that is the same in this routine
///         from the previous editions, is that the interface is
///         unchanged. Nearly everything else has been modified.
///         The routine was modified to make use of TPARTV
///         and TTRANS to handle the task of parsing and
///         computing seconds past 2000 TDB. This version
///         now handles leap seconds correctly.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WMO) (IMU)
/// ```
pub fn utc2et(ctx: &mut SpiceContext, utcstr: &str, et: &mut f64) -> crate::Result<()> {
    UTC2ET(utcstr.as_bytes(), et, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure UTC2ET ( UTC to Ephemeris Time )
pub fn UTC2ET(UTCSTR: &[u8], ET: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut PICTUR = [b' '; LNSIZE as usize];
    let mut ERROR = [b' '; LONGLN as usize];
    let mut TYPE = [b' '; SMWDSZ as usize];
    let mut MODIFY = ActualCharArray::new(SMWDSZ, 1..=5);
    let mut TVEC = StackArray::<f64, 10>::new(1..=10);
    let mut NTVEC: i32 = 0;
    let mut YEAR: i32 = 0;
    let mut SUCCES: bool = false;
    let mut OK: bool = false;
    let mut MODS: bool = false;
    let mut YABBRV: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"UTC2ET", ctx)?;
    //
    // So far we have no errors, the type of input is unknown.
    //
    fstr::assign(&mut ERROR, b" ");
    fstr::assign(&mut TYPE, b" ");

    //
    // First parse the string and perform the various tests on
    // the validity of its components.
    //
    TPARTV(
        UTCSTR,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    if !SUCCES {
        SETMSG(&ERROR, ctx);
        SIGERR(b"SPICE(INVALIDTIMESTRING)", ctx)?;
        CHKOUT(b"UTC2ET", ctx)?;
        return Ok(());
    }

    //
    // We are not going to allow most of the modifiers in strings.
    //
    if MODS {
        if (fstr::ne(MODIFY.get(SYSTEM), b" ") && fstr::ne(MODIFY.get(SYSTEM), b"UTC")) {
            fstr::assign(&mut ERROR, b"UTC2ET does not support the specification of a time system in a string.  The time system # was specified. Try the routine STR2ET.");
            REPMC(&ERROR.clone(), b"#", &MODIFY[SYSTEM], &mut ERROR);

            SETMSG(&ERROR, ctx);
            SIGERR(b"SPICE(INVALIDTIMESTRING)", ctx)?;
            CHKOUT(b"UTC2ET", ctx)?;
            return Ok(());
        } else if fstr::ne(MODIFY.get(ZONE), b" ") {
            fstr::assign(&mut ERROR, b"UTC2ET does not support the specification of a time zone in a time string.  The time zone \'#\' was specified. Try the routine STR2ET.");
            REPMC(&ERROR.clone(), b"#", &MODIFY[ZONE], &mut ERROR);
            SETMSG(&ERROR, ctx);
            SIGERR(b"SPICE(INVALIDTIMESTRING)", ctx)?;
            CHKOUT(b"UTC2ET", ctx)?;
            return Ok(());
        } else if fstr::ne(MODIFY.get(AMPM), b" ") {
            fstr::assign(&mut ERROR, b"UTC2ET does not support the AM/PM conventions for time strings. Try the routine STR2ET.");
            SETMSG(&ERROR, ctx);
            SIGERR(b"SPICE(INVALIDTIMESTRING)", ctx)?;
            CHKOUT(b"UTC2ET", ctx)?;
            return Ok(());
        }
    }

    //
    // If parsing the time string went well, we let TTRANS handle
    // the problem of transforming the time vector to TDB.
    //
    if (fstr::eq(&TYPE, b"YMD") || fstr::eq(&TYPE, b"YD")) {
        //
        // Check the components of the time vector for reasonableness.
        //
        TCHECK(
            TVEC.as_slice(),
            &TYPE,
            MODS,
            MODIFY.as_arg(),
            &mut OK,
            &mut ERROR,
            ctx,
        );

        if !OK {
            SETMSG(&ERROR, ctx);
            SIGERR(b"SPICE(INVALIDTIMESTRING)", ctx)?;
        }

        //
        // Fix up the year as needed.
        //
        YEAR = intrinsics::IDNINT(TVEC[1]);

        if fstr::eq(MODIFY.get(ERA), b"B.C.") {
            YEAR = (1 - YEAR);
        } else if fstr::eq(MODIFY.get(ERA), b"A.D.") {
            //
            // Do nothing.
            //
        } else if (YEAR < 100) {
            TEXPYR(&mut YEAR, ctx);
        }

        TVEC[1] = (YEAR as f64);
        //
        // We are ready for launch, convert the time vector.
        //
        TTRANS(&TYPE, b"TDB", TVEC.as_slice_mut(), ctx)?;
        *ET = TVEC[1];
    } else if fstr::eq(&TYPE, b"JD") {
        TTRANS(b"JDUTC", b"TDB", TVEC.as_slice_mut(), ctx)?;
        *ET = TVEC[1];
    } else {
        //
        // The only way to get here is if we got some unexpected
        // type of time string. Signal an error.
        //
        SETMSG(b"# time strings are not handled by UTC2ET. ", ctx);
        ERRCH(b"#", &TYPE, ctx);
        SIGERR(b"SPICE(INVALIDTIMESTRING)", ctx)?;
        CHKOUT(b"UTC2ET", ctx)?;
        return Ok(());
    }

    CHKOUT(b"UTC2ET", ctx)?;
    Ok(())
}
