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

struct SaveVars {
    DAYG: i32,
    DAYJ: i32,
    DOFYR: i32,
    DY: i32,
    M: i32,
    M100: i32,
    M4: i32,
    M400: i32,
    MON: i32,
    OFFSET: i32,
    OFFSTG: i32,
    OFFSTJ: i32,
    RDAYG: i32,
    RDAYJ: i32,
    TMPDAY: i32,
    TMPYR: i32,
    YR: i32,
    YROFF: i32,
    EXTRA: StackArray<i32, 12>,
    DPBEGL: StackArray<i32, 12>,
    DPJAN0: StackArray<i32, 12>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DAYG: i32 = 0;
        let mut DAYJ: i32 = 0;
        let mut DOFYR: i32 = 0;
        let mut DY: i32 = 0;
        let mut M: i32 = 0;
        let mut M100: i32 = 0;
        let mut M4: i32 = 0;
        let mut M400: i32 = 0;
        let mut MON: i32 = 0;
        let mut OFFSET: i32 = 0;
        let mut OFFSTG: i32 = 0;
        let mut OFFSTJ: i32 = 0;
        let mut RDAYG: i32 = 0;
        let mut RDAYJ: i32 = 0;
        let mut TMPDAY: i32 = 0;
        let mut TMPYR: i32 = 0;
        let mut YR: i32 = 0;
        let mut YROFF: i32 = 0;
        let mut EXTRA = StackArray::<i32, 12>::new(1..=12);
        let mut DPBEGL = StackArray::<i32, 12>::new(1..=12);
        let mut DPJAN0 = StackArray::<i32, 12>::new(1..=12);
        let mut FIRST: bool = false;

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

        Self {
            DAYG,
            DAYJ,
            DOFYR,
            DY,
            M,
            M100,
            M4,
            M400,
            MON,
            OFFSET,
            OFFSTG,
            OFFSTJ,
            RDAYG,
            RDAYJ,
            TMPDAY,
            TMPYR,
            YR,
            YROFF,
            EXTRA,
            DPBEGL,
            DPJAN0,
            FIRST,
        }
    }
}

fn GYDAYS(YR: i32) -> i32 {
    ((((365 * (YR - 1)) + ((YR - 1) / 4)) - ((YR - 1) / 100)) + ((YR - 1) / 400))
}

fn JYDAYS(YR: i32) -> i32 {
    ((365 * (YR - 1)) + ((YR - 1) / 4))
}

fn DIVBLE(YR: i32, M: i32) -> i32 {
    intrinsics::MAX0(&[0, ((1 + ((i32::abs(YR) / M) * M)) - i32::abs(YR))])
}

fn GLDAYS(YR: i32) -> i32 {
    ((DIVBLE(YR, 4) - DIVBLE(YR, 100)) + DIVBLE(YR, 400))
}

fn JLDAYS(YR: i32) -> i32 {
    DIVBLE(YR, 4)
}

fn GDOY(YR: i32, MON: i32, DY: i32, DPJAN0: &[i32], EXTRA: &[i32]) -> i32 {
    let DPJAN0 = DummyArray::new(DPJAN0, 1..=12);
    let EXTRA = DummyArray::new(EXTRA, 1..=12);
    ((DPJAN0[MON] + (EXTRA[MON] * GLDAYS(YR))) + DY)
}

fn JDOY(YR: i32, MON: i32, DY: i32, DPJAN0: &[i32], EXTRA: &[i32]) -> i32 {
    let DPJAN0 = DummyArray::new(DPJAN0, 1..=12);
    let EXTRA = DummyArray::new(EXTRA, 1..=12);
    ((DPJAN0[MON] + (EXTRA[MON] * JLDAYS(YR))) + DY)
}

fn GDP001(YR: i32, MON: i32, DY: i32, DPJAN0: &[i32], EXTRA: &[i32]) -> i32 {
    let DPJAN0 = DummyArray::new(DPJAN0, 1..=12);
    let EXTRA = DummyArray::new(EXTRA, 1..=12);
    ((GYDAYS(YR) + GDOY(YR, MON, DY, DPJAN0.as_slice(), EXTRA.as_slice())) - 1)
}

fn JDP001(YR: i32, MON: i32, DY: i32, DPJAN0: &[i32], EXTRA: &[i32]) -> i32 {
    let DPJAN0 = DummyArray::new(DPJAN0, 1..=12);
    let EXTRA = DummyArray::new(EXTRA, 1..=12);
    ((JYDAYS(YR) + JDOY(YR, MON, DY, DPJAN0.as_slice(), EXTRA.as_slice())) - 1)
}

/// Julian to Gregorian Calendar
///
/// Convert Year Month and Day on the Julian Calendar
/// to the Gregorian Calendar
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  YEAR      I-O  Year  of Julian Calendar/Gregorian Calendar
///  MONTH     I-O  Month of Julian Calendar/Gregorian Calendar
///  DAY       I-O  Day of Month in Julian Calendar/Gregorian Calendar
///  DOY        O   Day of Year in Gregorian Calendar
/// ```
///
/// # Detailed Input
///
/// ```text
///  YEAR     is an integer representing the year of an epoch, E, in
///           the Julian proleptic calendar. Note that the year 0
///           and negative values are required to represent
///           years in the pre-Christian era (B.C.) A year, Y B.C.,
///           should be represented as -(Y-1).  For example the year
///           435 B.C. should be input as -434.
///
///  MONTH    is an integer representing the month of some epoch, E,
///           in the Julian proleptic calendar. Months
///           outside the usual range from 1 to 12 are converted
///           to the standard range using modular arithmetic and
///           the input year is adjusted appropriately.
///
///
///  DAY      is the day of the month of some epoch, E, in the Julian
///           proleptic calendar.
///
///           Note to input an epoch as the day of a year, set MONTH
///           to 1 and DAY to the day of the year.
/// ```
///
/// # Detailed Output
///
/// ```text
///  YEAR     is an integer representing the year of the epoch, E,
///           above in the Gregorian calendar. Note that the year
///           0 (zero) and negative values are used to represent
///           years in the pre-Christian era (B.C.) A year, Y B.C.,
///           is be represented as -(Y-1).  For example the year
///           435 B.C. will be returned as -434.
///
///  MONTH    is an integer representing the month of the epoch, E,
///           above in the Gregorian Calendar calendar.
///
///  DAY      is the day of the month of the epoch, E, above in the
///           Gregorian Calendar
///
///  DOY      is the day of the year of the epoch, E, above in the
///           Gregorian Calendar.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a mid-level utility routine to assist in the assignment
///  and presentation of ancient epochs.
///
///  The SPICE software deals primarily with epochs represented on
///  in the Gregorian Calendar. However, the Gregorian calendar
///  was not adopted until October 15, 1582. As a result, epochs
///  prior to that time are usually represented in the Julian
///  proleptic calendar.
///
///  Formally, both calendars can be extended indefinitely forward
///  and backward in time due the algorithmic nature of the
///  determination of calendar representation.
///
///  When converting "parsed" calendar epochs in the SPICE system,
///  you need to first convert to the Gregorian Calendar. From that
///  point the SPICE toolkit can easily convert the epoch to Julian
///  date or seconds past the J2000 epoch.
///
///  This routine allows you to take a numeric representation of
///  an epoch represented in the Julian proleptic calendar and
///  convert that to an epoch in the Gregorian calendar.
///
///  To convert from Gregorian Calendar to Julian proleptic
///  calendar, use the entry point GR2JUL.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you need to find the epoch (in seconds past the
///  J2000) of some ancient epoch that occurred at
///  3:00 on March 4 of the year 121 B.C. And that this epoch
///  is based on the Julian proleptic calendar. We first need
///  to convert the Julian Calendar date to the Gregorian Calendar.
///
///  Here's the declarations we'll need
///
///     INTEGER               YEAR
///     INTEGER               MONTH
///     INTEGER               DAY
///     INTEGER               DOY
///
///     DOUBLE PRECISION      TVEC ( 6 )
///     DOUBLE PRECISION      TDB
///
///  You first need to convert the calendar date of this epoch
///  integers. (We don't need to worry about the hours for a moment).
///
///     YEAR  = -120
///     MONTH =  3
///     DAY   =  4
///
///  Convert this Year, Month and Day to the Gregorian Calendar.
///
///     CALL JUL2GR ( YEAR, MONTH, DAY, DOY )
///
///  Now construct a time vector for use in the routine TTRANS.
///  Note now we use the hour component of the epoch (the fourth
///  component of the time vector TVEC).
///
///     TVEC(1) = DBLE( YEAR )
///     TVEC(2) = DBLE( MONTH )
///     TVEC(3) = DBLE( DAY )
///     TVEC(4) = 3.0D0
///     TVEC(5) = 0.0D0
///     TVEC(6) = 0.0D0
///
///  Now the routine TTRANS can convert the time vector from
///  the input YMD format to barycentric dynamical time.
///
///     CALL TTRANS ( 'YDM', 'TDB', TVEC )
///
///     TDB = TVEC(1)
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.1, 02-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 26-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in RMAINI calls.
///
/// -    SPICELIB Version 1.1.1, 23-SEP-1999 (WLT)
///
///         Removed the unused variable DPMON.
///
/// -    SPICELIB Version 1.1.0, 23-FEB-1998 (WLT)
///
///         The routine was upgraded so that it will handle without
///         error months that are outside the range from 1 to 12.
///
/// -    SPICELIB Version 1.0.0, 13-MAR-1996 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 23-FEB-1998 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in RMAINI calls.
/// ```
pub fn jul2gr(
    ctx: &mut SpiceContext,
    year: &mut i32,
    month: &mut i32,
    day: &mut i32,
    doy: &mut i32,
) -> crate::Result<()> {
    JUL2GR(year, month, day, doy, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure JUL2GR (Julian to Gregorian Calendar)
pub fn JUL2GR(
    YEAR: &mut i32,
    MONTH: &mut i32,
    DAY: &mut i32,
    DOY: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Spicelib Functions
    //

    //
    // Local (in-line) Functions
    //

    //
    // Local parameters
    //
    // We declare the variables that contain the number of days in
    // 400 years (Gregorian), 100 years (Gregorian), 4 years and 1 year.
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
    // Local variables
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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Definitions of statement functions.
    //
    // The number of days elapsed since Gregorian Jan 1, of year 1 A.D.
    // to Jan 1 of YR is given by:
    //

    //
    // The number of days elapsed since Julian Jan 1, of year 1 A.D.
    // to Jan 1 of YR is given by:
    //

    //
    // Return 1 if YR is divisible by M, otherwise return 0.
    //

    //
    // The number of leap days in a Gregorian year is given by:
    //

    //
    // The number of leap days in a Julian year is given by:
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
    // The number of days since 1 Jan 1 A.D. (Gregorian) is given by:
    //

    //
    // The number of days since 1 Jan 1 A.D. (Julianis given by:
    //

    //
    // If this is the first pass through this entry point (or the
    // companion entry point) we need to set up some reference points.
    //
    // RDAYG   is the number of days past 1 A.D. Jan 1 of the Gregorian
    //         calendar of the date Oct 15, 1582
    //
    // RDAYJ   is the number of days past 1 A.D. Jan 1 of the Julian
    //         calendar of the date Oct 5, 1582.
    //
    // OFFSTJ and OFFSTG are just the offset from one count of days
    // to the other.
    //
    if save.FIRST {
        save.FIRST = false;

        save.RDAYG = GDP001(1582, 10, 15, save.DPJAN0.as_slice(), save.EXTRA.as_slice());
        save.RDAYJ = JDP001(1582, 10, 5, save.DPJAN0.as_slice(), save.EXTRA.as_slice());
        save.OFFSTJ = (save.RDAYJ - save.RDAYG);
        save.OFFSTG = (save.RDAYG - save.RDAYJ);
    }

    //
    // Make local copies of the year, month and day.  Then get the
    // YEARs into a positive range.
    //
    RMAINI((*MONTH - 1), 12, &mut save.YROFF, &mut save.MON, ctx)?;

    save.YR = (*YEAR + save.YROFF);
    save.MON = (save.MON + 1);
    save.DY = *DAY;

    if (save.YR <= 0) {
        RMAINI(save.YR, 4, &mut save.M4, &mut save.TMPYR, ctx)?;
        save.YR = save.TMPYR;

        if (save.YR == 0) {
            save.YR = (save.YR + 4);
            save.M4 = (save.M4 - 1);
        }

        save.OFFSET = (save.M4 * DP4Y);
    } else {
        save.OFFSET = 0;
    }

    //
    // First get the day number (Julian) for the input
    // year month and day.
    //
    save.DAYJ = (JDP001(
        save.YR,
        save.MON,
        save.DY,
        save.DPJAN0.as_slice(),
        save.EXTRA.as_slice(),
    ) + save.OFFSET);

    //
    // This day is DAYJ - RDAYJ days after 1582 Oct 5 on the
    // julian calendar.  But this is the same as the number
    // of days past 1582 Oct 15 on the Gregorian Calendar
    // So the Gregorian day number is DAYJ - RDAYJ + RDAYG
    // which is the same as DAYJ + OFFSTG.
    //
    save.DAYG = (save.DAYJ + save.OFFSTG);
    //
    // Now that we have the Gregorian day number it's a fairly
    // straight forward task to get the year, month and day
    // on the Gregorian calendar.
    //

    RMAINI(save.DAYG, DP400Y, &mut save.M400, &mut save.TMPDAY, ctx)?;
    save.DAYG = save.TMPDAY;

    save.M100 = intrinsics::MIN0(&[3, (save.DAYG / DP100Y)]);
    save.DAYG = (save.DAYG - (save.M100 * DP100Y));

    save.M4 = intrinsics::MIN0(&[24, (save.DAYG / DP4Y)]);
    save.DAYG = (save.DAYG - (save.M4 * DP4Y));

    save.M = intrinsics::MIN0(&[3, (save.DAYG / DP1Y)]);
    save.DAYG = (save.DAYG - (save.M * DP1Y));

    save.DOFYR = (save.DAYG + 1);
    save.YR = (((((400 * save.M400) + (100 * save.M100)) + (4 * save.M4)) + save.M) + 1);

    //
    // Now look up the month number and compute the day of the month.
    // How we do this depends on whether or not this is a leap year.
    //
    if (GLDAYS(save.YR) == 0) {
        save.MON = LSTLTI(save.DOFYR, 12, save.DPJAN0.as_slice());
        save.DY = (save.DOFYR - save.DPJAN0[save.MON]);
    } else {
        save.MON = LSTLTI(save.DOFYR, 12, save.DPBEGL.as_slice());
        save.DY = (save.DOFYR - save.DPBEGL[save.MON]);
    }

    *YEAR = save.YR;
    *MONTH = save.MON;
    *DAY = save.DY;
    *DOY = save.DOFYR;

    Ok(())
}

/// Gregorian to Julian Calendar
///
/// Convert Year Month and Day on the  Gregorian Calendar
/// to the Julian Calendar
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  YEAR      I-O  Year  of Gregorian Calendar/Julian Calendar
///  MONTH     I-O  Month of Gregorian Calendar/Julian Calendar
///  DAY       I-O  Day of Month in Gregorian Calendar/Julian Calendar
///  DOY        O   Day of Year in Julian Calendar
/// ```
///
/// # Detailed Input
///
/// ```text
///  YEAR     is an integer representing the year of an epoch, E, in
///           the Gregorian calendar. Note that the year 0 (zero)
///           and negative values are required to represent
///           years in the pre-Christian era (B.C.) A year, Y B.C.
///           should be represented as -(Y-1).  For example the year
///           435 B.C. should be input as -434.
///
///  MONTH    is an integer representing the month of some epoch, E,
///           in the Gregorian calendar. Months
///           outside the usual range from 1 to 12 are converted
///           to the standard range using modular arithmetic and
///           the input year is adjusted appropriately.
///
///  DAY      is the day of the month of some epoch, E, in the
///           Gregorian calendar.
///
///           Note to input an epoch as the day of a year, set MONTH
///           to 1 and DAY to the day of the year.
/// ```
///
/// # Detailed Output
///
/// ```text
///  YEAR     is an integer representing the year of the epoch, E,
///           above in the Julian calendar. Note that the year 0
///           (zero) and negative values are used to represent
///           years in the pre-Christian era (B.C.) A year, Y B.C.,
///           is be represented as -(Y-1).  For example the year
///           435 B.C. will be returned as -434.
///
///  MONTH    is an integer representing the month of the epoch, E,
///           above in the Julian Calendar calendar.
///
///  DAY      is the day of the month of the epoch, E, above in the
///           Julian Calendar
///
///  DOY      is the day of the year of the epoch, E, above in the
///           Julian Calendar.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a mid-level utility routine to assist in the assignment
///  and presentation of Ancient epochs.
///
///  The SPICE software deals primarily with epochs represented on
///  in the Gregorian Calendar. However, the Gregorian calendar
///  was not adopted until October 15, 1582. As a result, epochs
///  prior to that time are usually represented in the Julian
///  proleptic calendar.
///
///  Formally, both calendars can be extended indefinitely forward
///  and backward in time due the algorithmic nature of the
///  determination of calendar representation.
///
///  This routine allows you to take a numeric representation of
///  an epoch represented in the Gregorian calendar and
///  convert that to an epoch in the Julian calendar.
///
///  To convert from Julian Calendar to Gregorian
///  calendar, use the entry point JUL2GR.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you need to print an epoch (given in seconds past the
///  J2000 epoch) of some ancient epoch that occurred during
///  pre-Christian era, and that you want to represent this epoch
///  using the Julian proleptic calendar.
///
///  Here's the declarations we'll need
///
///     INTEGER               YEAR
///     INTEGER               MONTH
///     INTEGER               DAY
///     INTEGER               DOY
///
///     DOUBLE PRECISION      TVEC ( 6 )
///     DOUBLE PRECISION      TDB
///
///  You first need to convert TDB (the epoch in Seconds past J2000)
///  to a calendar representation.
///
///     TVEC(1) = TDB.
///
///     CALL TTRANS ( 'TDB', 'YMD', TVEC )
///
///  The output time vector will be relative to the Gregorian
///  Calendar. Collect the year, month and day from the time
///  vector.
///
///     YEAR    = INT ( TVEC(1) )
///     MONTH   = INT ( TVEC(2) )
///     DAY     = INT ( TVEC(3) )
///
///  The hours, minutes and seconds appear in components 4 through 6
///  of the time vector. We can ignore them in the conversion
///  of the calendar from Gregorian to Julian.
///
///     CALL GR2JUL ( YEAR, MONTH, DAY, DOY )
///
///  Now create a string from the YEAR, MONTH, DAY and TVEC(4)
///  through TVEC(6).
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
/// -    SPICELIB Version 1.1.1, 02-OCT-2021 (JDR)
///
///         Reordered header sections. Edited the header to comply with
///         NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 23-FEB-1998 (WLT)
///
///         The routine was upgraded so that it will handle without
///         error months that are outside the range from 1 to 12.
///
/// -    SPICELIB Version 1.0.0, 13-MAR-1996 (WLT)
/// ```
pub fn gr2jul(
    ctx: &mut SpiceContext,
    year: &mut i32,
    month: &mut i32,
    day: &mut i32,
    doy: &mut i32,
) -> crate::Result<()> {
    GR2JUL(year, month, day, doy, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure GR2JUL (Gregorian to Julian Calendar)
pub fn GR2JUL(
    YEAR: &mut i32,
    MONTH: &mut i32,
    DAY: &mut i32,
    DOY: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // If this is the first pass through this entry point (or the
    // companion entry point) we need to set up some reference points.
    //
    // RDAYG   is the number of days past 1 A.D. Jan 1 of the Gregorian
    //         calendar of the date Oct 15, 1582
    //
    // RDAYJ   is the number of days past 1 A.D. Jan 1 of the Julian
    //         calendar of the date Oct 5, 1582.
    //
    // OFFSTJ and OFFSTG are just the offset from one count of days
    // to the other.
    //
    if save.FIRST {
        save.FIRST = false;

        save.RDAYG = GDP001(1582, 10, 15, save.DPJAN0.as_slice(), save.EXTRA.as_slice());
        save.RDAYJ = JDP001(1582, 10, 5, save.DPJAN0.as_slice(), save.EXTRA.as_slice());
        save.OFFSTJ = (save.RDAYJ - save.RDAYG);
        save.OFFSTG = (save.RDAYG - save.RDAYJ);
    }
    //
    // Make Local Copies of YEAR, MONTH and DAY and get YEAR into
    // a positive range.
    //
    RMAINI((*MONTH - 1), 12, &mut save.YROFF, &mut save.MON, ctx)?;

    save.YR = (*YEAR + save.YROFF);
    save.MON = (save.MON + 1);
    save.DY = *DAY;

    if (save.YR <= 0) {
        RMAINI(save.YR, 400, &mut save.M400, &mut save.TMPYR, ctx)?;
        save.YR = save.TMPYR;

        if (save.YR == 0) {
            save.YR = (save.YR + 400);
            save.M400 = (save.M400 - 1);
        }

        save.OFFSET = (save.M400 * DP400Y);
    } else {
        save.OFFSET = 0;
    }

    //
    // First get the day number (Gregorian) for the input
    // year month and day.
    //
    save.DAYG = (GDP001(
        save.YR,
        save.MON,
        save.DY,
        save.DPJAN0.as_slice(),
        save.EXTRA.as_slice(),
    ) + save.OFFSET);

    //
    // This day is DAYG - RDAYG days after 1582 Oct 15 on the
    // Gregorian calendar.  But this is the same as the number
    // of days past 1582 Oct 5 on the Julian Calendar
    // So the Julian day number is DAYG - RDAYG + RDAYJ
    // which is the same as DAYG + OFFSTJ.
    //
    save.DAYJ = (save.DAYG + save.OFFSTJ);
    //
    // Now that we have the Julian day number it's a fairly
    // straight forward task to get the year, month and day
    // on the Julian calendar.
    //

    RMAINI(save.DAYJ, DP4Y, &mut save.M4, &mut save.TMPDAY, ctx)?;
    save.DAYJ = save.TMPDAY;

    save.M = intrinsics::MIN0(&[3, (save.DAYJ / DP1Y)]);
    save.DAYJ = (save.DAYJ - (save.M * DP1Y));

    save.DOFYR = (save.DAYJ + 1);
    save.YR = (((4 * save.M4) + save.M) + 1);

    //
    // Now look up the month number and compute the day of the month.
    // How we do this depends on whether or not this is a leap year.
    //
    if (JLDAYS(save.YR) == 0) {
        save.MON = LSTLTI(save.DOFYR, 12, save.DPJAN0.as_slice());
        save.DY = (save.DOFYR - save.DPJAN0[save.MON]);
    } else {
        save.MON = LSTLTI(save.DOFYR, 12, save.DPBEGL.as_slice());
        save.DY = (save.DOFYR - save.DPBEGL[save.MON]);
    }

    *YEAR = save.YR;
    *MONTH = save.MON;
    *DAY = save.DY;
    *DOY = save.DOFYR;

    Ok(())
}
