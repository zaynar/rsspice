//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const DP400Y: i32 = ((365 * 400) + 97);
const DP100Y: i32 = ((365 * 100) + 24);
const DP4Y: i32 = ((365 * 4) + 1);
const DP1Y: i32 = 365;
const JAN: i32 = 31;
const FEB: i32 = 28;
const MAR: i32 = 31;
const APR: i32 = 30;
const MAY: i32 = 31;
const JUN: i32 = 30;
const JUL: i32 = 31;
const AUG: i32 = 31;
const SEP: i32 = 30;
const OCT: i32 = 31;
const NOV: i32 = 30;
const DEC: i32 = 31;
const JAN0: i32 = 0;
const FEB0: i32 = (JAN + JAN0);
const MAR0: i32 = (FEB + FEB0);
const APR0: i32 = (MAR + MAR0);
const MAY0: i32 = (APR + APR0);
const JUN0: i32 = (MAY + MAY0);
const JUL0: i32 = (JUN + JUN0);
const AUG0: i32 = (JUL + JUL0);
const SEP0: i32 = (AUG + AUG0);
const OCT0: i32 = (SEP + SEP0);
const NOV0: i32 = (OCT + OCT0);
const DEC0: i32 = (NOV + NOV0);
const JANL0: i32 = JAN0;
const FEBL0: i32 = FEB0;
const MARL0: i32 = (MAR0 + 1);
const APRL0: i32 = (APR0 + 1);
const MAYL0: i32 = (MAY0 + 1);
const JUNL0: i32 = (JUN0 + 1);
const JULL0: i32 = (JUL0 + 1);
const AUGL0: i32 = (AUG0 + 1);
const SEPL0: i32 = (SEP0 + 1);
const OCTL0: i32 = (OCT0 + 1);
const NOVL0: i32 = (NOV0 + 1);
const DECL0: i32 = (DEC0 + 1);
const STRSIZ: i32 = 16;
const LNGSIZ: i32 = 180;

struct SaveVars {
    ERA: Vec<u8>,
    YSTR: Vec<u8>,
    DSTR: Vec<u8>,
    HSTR: Vec<u8>,
    MSTR: Vec<u8>,
    SSTR: Vec<u8>,
    MESSGE: Vec<u8>,
    DATE: Vec<u8>,
    MONTHS: ActualCharArray,
    DMNINT: f64,
    DMXINT: f64,
    DP2000: f64,
    FRAC: f64,
    HALFD: f64,
    MYDNOM: f64,
    MYNUM: f64,
    Q: f64,
    REMD: f64,
    SECS: f64,
    SECSPD: f64,
    BH: i32,
    BM: i32,
    DAY: i32,
    DAYNUM: i32,
    DN2000: i32,
    DOFYR: i32,
    HOURS: i32,
    IQ: i32,
    MINS: i32,
    MONTH: i32,
    OFFSET: i32,
    REM: i32,
    TSECS: i32,
    YEAR: i32,
    YR1: i32,
    YR100: i32,
    YR4: i32,
    YR400: i32,
    FIRST: bool,
    ADJUST: bool,
    EXTRA: StackArray<i32, 12>,
    DPBEGL: StackArray<i32, 12>,
    DPJAN0: StackArray<i32, 12>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ERA = vec![b' '; STRSIZ as usize];
        let mut YSTR = vec![b' '; STRSIZ as usize];
        let mut DSTR = vec![b' '; STRSIZ as usize];
        let mut HSTR = vec![b' '; STRSIZ as usize];
        let mut MSTR = vec![b' '; STRSIZ as usize];
        let mut SSTR = vec![b' '; STRSIZ as usize];
        let mut MESSGE = vec![b' '; STRSIZ as usize];
        let mut DATE = vec![b' '; LNGSIZ as usize];
        let mut MONTHS = ActualCharArray::new(3, 1..=12);
        let mut DMNINT: f64 = 0.0;
        let mut DMXINT: f64 = 0.0;
        let mut DP2000: f64 = 0.0;
        let mut FRAC: f64 = 0.0;
        let mut HALFD: f64 = 0.0;
        let mut MYDNOM: f64 = 0.0;
        let mut MYNUM: f64 = 0.0;
        let mut Q: f64 = 0.0;
        let mut REMD: f64 = 0.0;
        let mut SECS: f64 = 0.0;
        let mut SECSPD: f64 = 0.0;
        let mut BH: i32 = 0;
        let mut BM: i32 = 0;
        let mut DAY: i32 = 0;
        let mut DAYNUM: i32 = 0;
        let mut DN2000: i32 = 0;
        let mut DOFYR: i32 = 0;
        let mut HOURS: i32 = 0;
        let mut IQ: i32 = 0;
        let mut MINS: i32 = 0;
        let mut MONTH: i32 = 0;
        let mut OFFSET: i32 = 0;
        let mut REM: i32 = 0;
        let mut TSECS: i32 = 0;
        let mut YEAR: i32 = 0;
        let mut YR1: i32 = 0;
        let mut YR100: i32 = 0;
        let mut YR4: i32 = 0;
        let mut YR400: i32 = 0;
        let mut FIRST: bool = false;
        let mut ADJUST: bool = false;
        let mut EXTRA = StackArray::<i32, 12>::new(1..=12);
        let mut DPBEGL = StackArray::<i32, 12>::new(1..=12);
        let mut DPJAN0 = StackArray::<i32, 12>::new(1..=12);

        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(1),
                Val::I(1),
                Val::I(1),
                Val::I(1),
                Val::I(1),
                Val::I(1),
                Val::I(1),
                Val::I(1),
                Val::I(1),
            ]
            .into_iter();
            EXTRA
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(JAN0),
                Val::I(FEB0),
                Val::I(MAR0),
                Val::I(APR0),
                Val::I(MAY0),
                Val::I(JUN0),
                Val::I(JUL0),
                Val::I(AUG0),
                Val::I(SEP0),
                Val::I(OCT0),
                Val::I(NOV0),
                Val::I(DEC0),
            ]
            .into_iter();
            DPJAN0
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(JANL0),
                Val::I(FEBL0),
                Val::I(MARL0),
                Val::I(APRL0),
                Val::I(MAYL0),
                Val::I(JUNL0),
                Val::I(JULL0),
                Val::I(AUGL0),
                Val::I(SEPL0),
                Val::I(OCTL0),
                Val::I(NOVL0),
                Val::I(DECL0),
            ]
            .into_iter();
            DPBEGL
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
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
            MONTHS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ERA,
            YSTR,
            DSTR,
            HSTR,
            MSTR,
            SSTR,
            MESSGE,
            DATE,
            MONTHS,
            DMNINT,
            DMXINT,
            DP2000,
            FRAC,
            HALFD,
            MYDNOM,
            MYNUM,
            Q,
            REMD,
            SECS,
            SECSPD,
            BH,
            BM,
            DAY,
            DAYNUM,
            DN2000,
            DOFYR,
            HOURS,
            IQ,
            MINS,
            MONTH,
            OFFSET,
            REM,
            TSECS,
            YEAR,
            YR1,
            YR100,
            YR4,
            YR400,
            FIRST,
            ADJUST,
            EXTRA,
            DPBEGL,
            DPJAN0,
        }
    }
}

fn YDAYS(YEAR: i32) -> i32 {
    ((((365 * (YEAR - 1)) + ((YEAR - 1) / 4)) - ((YEAR - 1) / 100)) + ((YEAR - 1) / 400))
}

fn LDAYS(YEAR: i32) -> i32 {
    (((((YEAR / 4) * 4) / YEAR) - (((YEAR / 100) * 100) / YEAR)) + (((YEAR / 400) * 400) / YEAR))
}

fn DOY(YEAR: i32, MONTH: i32, DAY: i32, DPJAN0: &[i32], EXTRA: &[i32]) -> i32 {
    let DPJAN0 = DummyArray::new(DPJAN0, 1..=12);
    let EXTRA = DummyArray::new(EXTRA, 1..=12);
    ((DPJAN0[MONTH] + (EXTRA[MONTH] * LDAYS(YEAR))) + DAY)
}

fn DP0001(YEAR: i32, MONTH: i32, DAY: i32, DPJAN0: &[i32], EXTRA: &[i32]) -> i32 {
    let DPJAN0 = DummyArray::new(DPJAN0, 1..=12);
    let EXTRA = DummyArray::new(EXTRA, 1..=12);
    ((YDAYS(YEAR) + DOY(YEAR, MONTH, DAY, DPJAN0.as_slice(), EXTRA.as_slice())) - 1)
}

/// Convert ET to Calendar format
///
/// Convert from an ephemeris epoch measured in seconds past
/// the epoch of J2000 to a calendar string format using a
/// formal calendar free of leapseconds.
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
///  ET         I   Ephemeris time measured in seconds past J2000.
///  CALSTR     O   A standard calendar representation of ET.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is an epoch measured in ephemeris seconds
///           past the epoch of J2000.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CALSTR   is a calendar string representing the input ephemeris
///           epoch. This string is based upon extending the
///           Gregorian Calendar backward and forward indefinitely
///           keeping the same rules for determining leap years.
///           Moreover, there is no accounting for leapseconds.
///
///           To be sure that all of the date can be stored in
///           CALSTR, it should be declared to have length at
///           least 48 characters.
///
///           The string will have the following format
///
///              year (era) mon day hr:mn:sc.sss
///
///           Where:
///
///              year --- is the year
///              era  --- is the chronological era associated with
///                       the date. For years after 999 A.D.
///                       the era is omitted. For years
///                       between 1 A.D. and 999 A.D. (inclusive)
///                       era is the string 'A.D.' For epochs
///                       before 1 A.D. Jan 1 00:00:00, era is
///                       given as 'B.C.' and the year is converted
///                       to years before the "Christian Era".
///                       The last B.C. epoch is
///
///                         1 B.C. DEC 31 23:59:59.999
///
///                       The first A.D. epoch (which occurs .001
///                       seconds after the last B.C. epoch) is:
///
///                          1 A.D. JAN 1 00:00:00.000
///
///                       Note: there is no year 0 A.D. or 0 B.C.
///              mon  --- is a 3-letter abbreviation for the month
///                       in all capital letters.
///              day  --- is the day of the month
///              hr   --- is the hour of the day (between 0 and 23)
///                       leading zeros are added to hr if the
///                       numeric value is less than 10.
///              mn   --- is the minute of the hour (0 to 59)
///                       leading zeros are added to mn if the
///                       numeric value is less than 10.
///              sc.sss   is the second of the minute to 3 decimal
///                       places ( 0 to 59.999). Leading zeros
///                       are added if the numeric value is less
///                       than 10. Seconds are truncated, not
///                       rounded.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the input ET is so large that the corresponding
///      number of days since 1 A.D. Jan 1, 00:00:00 is
///      within 1 of overflowing or underflowing an integer,
///      ET will not be converted to the correct string
///      representation rather, the string returned will
///      state that the epoch was before or after the day
///      that is INTMIN +1 or INTMAX - 1 days after
///      1 A.D. Jan 1, 00:00:00.
///
///  2)  If the output string is not sufficiently long to hold
///      the full date, it will be truncated on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  This is an error free routine for converting ephemeris epochs
///  represented as seconds past the J2000 epoch to formal
///  calendar strings based upon the Gregorian Calendar. This formal
///  time is often useful when one needs a human recognizable
///  form of an ephemeris epoch. There is no accounting for leap
///  seconds in the output times produced.
///
///  Note: The calendar epochs produced are not the same as the
///        UTC calendar epochs that correspond to ET. The strings
///        produced by this routine may vary from the corresponding
///        UTC epochs by more than 1 minute.
///
///  This routine can be used in creating error messages or
///  in routines and programs in which one prefers to report
///  times without employing leapseconds to produce exact UTC
///  epochs.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you wish to  report that no data is
///  available at a particular ephemeris epoch ET. The following
///  code shows how you might accomplish this task:
///
///        CALL DPSTRF ( ET,  6, 'F', ETSTR  )
///        CALL ETCAL  ( ET,          CALSTR )
///
///        E1 = RTRIM   (             CALSTR )
///        E2 = RTRIM   (             ETSTR  )
///
///        WRITE (*,*) 'There is no data available for the body '
///        WRITE (*,*) 'at requested time: '
///        WRITE (*,*) '   ', CALSTR(1:E1), ' (', ETSTR(1:E2), ')'
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  One must keep in mind when using this routine that
///      ancient times are not based upon the Gregorian
///      calendar. For example the 0 point of the Julian
///      Date system is 4713 B.C. Jan 1, 12:00:00 on the Julian
///      Calendar. If one formalized the Gregorian calendar
///      and extended it indefinitely, the zero point of the Julian
///      date system corresponds to 4714 B.C. NOV 24 12:00:00 on
///      the Gregorian calendar. There are several reasons for this.
///      Leap years in the Julian calendar occur every
///      4 years (including *all* centuries). Moreover,  the
///      Gregorian calendar "effectively" begins on 15 Oct, 1582 A.D.
///      which is 5 Oct, 1582 A.D. in the Julian Calendar.
///
///      Therefore you must be careful in your interpretation
///      of ancient dates produced by this routine.
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
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.3.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement. Changed output argument name
///         STRING to CALSTR for consistency with other routines.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section. Added TIME required reading.
///
/// -    SPICELIB Version 2.2.0, 05-MAR-1998 (WLT)
///
///         The documentation concerning the appearance of the output
///         time string was corrected so that it does not suggest
///         a comma is inserted after the day of the month. The
///         comma was removed from the output string in Version 2.0.0
///         (see the note below) but the documentation was not upgraded
///         accordingly.
///
/// -    SPICELIB Version 2.1.0, 20-MAY-1996 (WLT)
///
///         Two arrays that were initialized but never used were
///         removed.
///
/// -    SPICELIB Version 2.0.0, 16-AUG-1995 (KRG)
///
///         If the day number was less than 10, the spacing was off for
///         the rest of the time by one space, that for the "tens" digit.
///         This has been fixed by using a leading zero when the number of
///         days is < 10.
///
///         Also, the comma that appeared between the month/day/year
///         and the hour:minute:seconds tokens has been removed. This was
///         done in order to make the calendar date format of ETCAL
///         consistent with the calendar date format of ET2UTC.
///
///
/// -    SPICELIB Version 1.0.0, 14-DEC-1993 (WLT)
/// ```
pub fn etcal(ctx: &mut SpiceContext, et: f64, calstr: &mut str) {
    ETCAL(et, fstr::StrBytes::new(calstr).as_mut(), ctx.raw_context());
}

//$Procedure ETCAL ( Convert ET to Calendar format )
pub fn ETCAL(ET: f64, CALSTR: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB Functions.
    //

    //
    // We declare the variables that contain the number of days in
    // 400 years, 100 years, 4 years and 1 year.
    //

    //
    // The following integers give the number of days during the
    // associated month of a non-leap year.
    //

    //
    // The integers that follow give the number of days in a normal
    // year that precede the first of the month.
    //

    //
    // The integers that follow give the number of days in a leap
    // year that precede the first of the month.
    //

    //
    // The variables below hold the components of the output string
    // before they are put together.
    //

    //
    // We will construct our string using the local variable DATE
    // and transfer the results to the output CALSTR when we are
    // done.
    //

    //
    // MONTHS contains 3-letter abbreviations for the months of the year
    //

    //
    // The array EXTRA contains the number of additional days that
    // appear before the first of a month during a leap year (as opposed
    // to a non-leap year).
    //

    //
    // DPJAN0(I) gives the number of days that occur before the I'th
    // month of a normal year.
    //

    //
    // Definitions of statement functions.
    //
    //
    // The number of days elapsed since Jan 1, of year 1 A.D. to
    // Jan 1 of YEAR is given by:
    //

    //
    // The number of leap days in a year is given by:
    //

    //
    // To compute the day of the year we
    //
    //    look up the number of days to the beginning of the month,
    //
    //    add on the number leap days that occurred prior to that
    //    time
    //
    //    add on the number of days into the month
    //

    //
    // The number of days since 1 Jan 1 A.D. is given by:
    //

    if save.FIRST {
        save.FIRST = false;
        save.HALFD = (SPD() / 2.0);
        save.SECSPD = SPD();
        save.DN2000 = DP0001(2000, 1, 1, save.DPJAN0.as_slice(), save.EXTRA.as_slice());
        save.DMXINT = INTMAX() as f64;
        save.DMNINT = INTMIN() as f64;
    }
    //
    // Now we "in-line" compute the following call.
    //
    //    call rmaind ( et + halfd, secspd, dp2000, secs )
    //
    // because we can't make a call to rmaind.
    //
    // The reader may wonder why we use et + halfd.  The value
    // et is seconds past the ephemeris epoch of J2000 which
    // is at 2000 Jan 1, 12:00:00.  We want to compute days past
    // 2000 Jan 1, 00:00:00.  The seconds past THAT epoch is et + halfd.
    // We add on 0.0005 seconds so that the string produced will be
    // rounded to the nearest millisecond.
    //
    save.MYDNOM = save.SECSPD;
    save.MYNUM = (ET + save.HALFD);

    save.Q = f64::trunc((save.MYNUM / save.MYDNOM));
    save.REMD = (save.MYNUM - (save.Q * save.MYDNOM));

    if (save.REMD < 0.0) {
        save.Q = (save.Q - 1.0);
        save.REMD = (save.REMD + save.MYDNOM);
    }

    save.SECS = save.REMD;
    save.DP2000 = save.Q;

    //
    // Do something about the problem when ET is vastly
    // out of range.  (Day number outside MAX and MIN integer).
    //
    if ((save.DP2000 + save.DN2000 as f64) < (save.DMNINT + 1 as f64)) {
        save.DP2000 = ((save.DMNINT - save.DN2000 as f64) + 1 as f64);
        fstr::assign(&mut save.MESSGE, b"Epoch before ");
        save.SECS = 0.0;
    } else if ((save.DP2000 + save.DN2000 as f64) > (save.DMXINT - 1 as f64)) {
        save.DP2000 = ((save.DMXINT - save.DN2000 as f64) - 1 as f64);
        fstr::assign(&mut save.MESSGE, b"Epoch after ");
        save.SECS = 0.0;
    } else {
        fstr::assign(&mut save.MESSGE, b" ");
    }

    //
    // Compute the number of days since 1 .A.D. Jan 1, 00:00:00.
    // From the tests in the previous IF-ELSE IF-ELSE block this
    // addition is guaranteed not to overflow.
    //
    save.DAYNUM = ((save.DP2000 + (save.DN2000 as f64)) as i32);

    //
    // If the number of days is negative, we need to do a little
    // work so that we can represent the date in the B.C. era.
    // We add enough multiples of 400 years so that the year will
    // be positive and then we subtract off the appropriate multiple
    // of 400 years later.
    //
    if (save.DAYNUM < 0) {
        //
        // Since we can't make the call below and remain
        // error free, we compute it ourselves.
        //
        // call rmaini ( daynum, dp400y, offset, daynum )
        //
        save.IQ = (save.DAYNUM / DP400Y);
        save.REM = (save.DAYNUM - (DP400Y * save.IQ));

        if (save.REM < 0) {
            save.IQ = (save.IQ - 1);
            save.REM = (save.REM + DP400Y);
        }

        save.OFFSET = save.IQ;
        save.DAYNUM = save.REM;

        save.ADJUST = true;
    } else {
        save.ADJUST = false;
    }

    //
    // Next we compute the year.  Divide out multiples of 400, 100
    // 4 and 1 year.  Finally combine these to get the correct
    // value for year.  (Note this is all integer arithmetic.)
    //
    // Recall that DP1Y   =    365
    //             DP4Y   =  4*DPY    + 1
    //             DP100Y = 25*DP4Y   - 1
    //             DP400Y =  4*DP100Y + 1
    //
    save.YR400 = (save.DAYNUM / DP400Y);
    save.REM = (save.DAYNUM - (DP400Y * save.YR400));

    save.YR100 = intrinsics::MIN0(&[3, (save.REM / DP100Y)]);
    save.REM = (save.REM - (save.YR100 * DP100Y));

    save.YR4 = intrinsics::MIN0(&[24, (save.REM / DP4Y)]);
    save.REM = (save.REM - (save.YR4 * DP4Y));

    save.YR1 = intrinsics::MIN0(&[3, (save.REM / DP1Y)]);
    save.REM = (save.REM - (save.YR1 * DP1Y));

    save.DOFYR = (save.REM + 1);

    save.YEAR = (((((save.YR400 * 400) + (save.YR100 * 100)) + (save.YR4 * 4)) + save.YR1) + 1);
    //
    // Get the month, and day of month (depending upon whether
    // we have a leap year or not).
    //
    if (LDAYS(save.YEAR) == 0) {
        save.MONTH = LSTLTI(save.DOFYR, 12, save.DPJAN0.as_slice());
        save.DAY = (save.DOFYR - save.DPJAN0[save.MONTH]);
    } else {
        save.MONTH = LSTLTI(save.DOFYR, 12, save.DPBEGL.as_slice());
        save.DAY = (save.DOFYR - save.DPBEGL[save.MONTH]);
    }
    //
    // If we had to adjust the year to make it positive, we now
    // need to correct it and then convert it to a B.C. year.
    //
    if save.ADJUST {
        save.YEAR = (save.YEAR + (save.OFFSET * 400));
        save.YEAR = (-save.YEAR + 1);
        fstr::assign(&mut save.ERA, b" B.C. ");
    } else {
        //
        // If the year is less than 1000, we can't just write it
        // out.  We need to add the era.  If we don't do this
        // the dates look very confusing.
        //
        if (save.YEAR < 1000) {
            fstr::assign(&mut save.ERA, b" A.D. ");
        } else {
            fstr::assign(&mut save.ERA, b" ");
        }
    }
    //
    // Convert Seconds to Hours, Minute and Seconds.
    // We work with thousandths of a second in integer arithmetic
    // so that all of the truncation work with seconds will already
    // be done.  (Note that we already know that SECS is greater than
    // or equal to zero so we'll have no problems with HOURS, MINS
    // or SECS becoming negative.)
    //
    save.TSECS = ((save.SECS * 1000.0) as i32);
    save.FRAC = (save.SECS - (save.TSECS as f64));

    save.HOURS = (save.TSECS / 3600000);
    save.TSECS = (save.TSECS - (3600000 * save.HOURS));

    save.MINS = (save.TSECS / 60000);
    save.TSECS = (save.TSECS - (60000 * save.MINS));

    save.SECS = ((save.TSECS as f64) / 1000.0);
    //
    // We round seconds if we can do so without getting seconds to be
    // bigger than 60.
    //
    if ((save.SECS + 0.0005) < 60.0) {
        save.SECS = (save.SECS + 0.0005);
    }

    //
    // Finally, get the components of our date string.
    //
    INTSTR(save.YEAR, &mut save.YSTR, ctx);
    if (save.DAY >= 10) {
        INTSTR(save.DAY, &mut save.DSTR, ctx);
    } else {
        fstr::assign(&mut save.DSTR, b"0");
        INTSTR(save.DAY, fstr::substr_mut(&mut save.DSTR, 2..), ctx);
    }

    //
    // We want to zero pad the hours minutes and seconds.
    //
    if (save.HOURS < 10) {
        save.BH = 2;
    } else {
        save.BH = 1;
    }

    if (save.MINS < 10) {
        save.BM = 2;
    } else {
        save.BM = 1;
    }

    fstr::assign(&mut save.MSTR, b"00");
    fstr::assign(&mut save.HSTR, b"00");
    fstr::assign(&mut save.SSTR, b" ");

    //
    // Now construct the string components for hours, minutes and
    // seconds.
    //
    save.SECS = ((((save.SECS * 1000.0) as i32) as f64) / 1000.0);

    INTSTR(save.HOURS, fstr::substr_mut(&mut save.HSTR, save.BH..), ctx);
    INTSTR(save.MINS, fstr::substr_mut(&mut save.MSTR, save.BM..), ctx);
    DPSTRF(save.SECS, 6, b"F", &mut save.SSTR, ctx);
    //
    // The form of the output for SSTR has a leading blank followed by
    // the first significant digit.  If a decimal point is in the
    // third slot, then SSTR is of the form ' x.xxxxx'  and we need
    // to insert a leading zero.
    //
    if fstr::eq(fstr::substr(&save.SSTR, 3..=3), b".") {
        fstr::assign(fstr::substr_mut(&mut save.SSTR, 1..=1), b"0");
    }
    //
    // We don't want any leading spaces in SSTR, (HSTR and MSTR don't
    // have leading spaces by construction.
    //
    LJUST(&save.SSTR.to_vec(), &mut save.SSTR);
    //
    // Now form the date string, squeeze out extra spaces and
    // left justify the whole thing.
    //
    fstr::assign(
        &mut save.DATE,
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(
                                &fstr::concat(
                                    &fstr::concat(
                                        &fstr::concat(
                                            &fstr::concat(
                                                &fstr::concat(&save.MESSGE, &save.YSTR),
                                                &save.ERA,
                                            ),
                                            save.MONTHS.get(save.MONTH),
                                        ),
                                        b" ",
                                    ),
                                    fstr::substr(&save.DSTR, 1..=3),
                                ),
                                b" ",
                            ),
                            fstr::substr(&save.HSTR, 1..=2),
                        ),
                        b":",
                    ),
                    fstr::substr(&save.MSTR, 1..=2),
                ),
                b":",
            ),
            fstr::substr(&save.SSTR, 1..=6),
        ),
    );

    CMPRSS(b" ", 1, &save.DATE.to_vec(), &mut save.DATE);
    LJUST(&save.DATE.to_vec(), &mut save.DATE);

    fstr::assign(CALSTR, &save.DATE);
}
