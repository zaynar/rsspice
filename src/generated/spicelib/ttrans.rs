//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const MXCOMP: i32 = 7;
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
const MAXLP: i32 = 140;
const MAXVAR: i32 = 1;
const DAYSEC: i32 = 1;
const DAYP2: i32 = (DAYSEC + 1);
const ET: i32 = (DAYP2 + 1);
const FRML: i32 = (ET + 1);
const JDTDB: i32 = (FRML + 1);
const JDTDT: i32 = (JDTDB + 1);
const JDUTC: i32 = (JDTDT + 1);
const JED: i32 = (JDUTC + 1);
const TTAI: i32 = (JED + 1);
const TDB: i32 = (TTAI + 1);
const TDT: i32 = (TDB + 1);
const YD: i32 = (TDT + 1);
const YDD: i32 = (YD + 1);
const YDDF: i32 = (YDD + 1);
const YDF: i32 = (YDDF + 1);
const YMD: i32 = (YDF + 1);
const YMDF: i32 = (YMD + 1);
const YMWD: i32 = (YMDF + 1);
const YMWDF: i32 = (YMWD + 1);
const YWD: i32 = (YMWDF + 1);
const YWDF: i32 = (YWD + 1);
const NREC: i32 = YWDF;
const NO: bool = false;
const YES: bool = true;
const LBCELL: i32 = -5;
const TYPLEN: i32 = 8;

struct SaveVars {
    UNIFRM: ActualCharArray,
    RECOG: ActualCharArray,
    PARSED: StackArray<i32, 21>,
    NEEDY: StackArray<bool, 21>,
    FORML: StackArray<bool, 21>,
    ORDVEC: StackArray<i32, 21>,
    REST: Vec<u8>,
    MYFROM: Vec<u8>,
    MYTO: Vec<u8>,
    VARS: ActualCharArray,
    DAYDP: f64,
    DAYLEN: f64,
    DP2000: f64,
    DT: f64,
    EXSECS: f64,
    FORMAL: f64,
    FRAC: f64,
    HALFD: f64,
    HOURS: f64,
    JD1101: f64,
    JDSECS: f64,
    LASTDT: f64,
    MINS: f64,
    SECS: f64,
    SECSPD: f64,
    TAI: f64,
    TAITAB: ActualArray<f64>,
    TEMPD: f64,
    TSECS: f64,
    DAY: i32,
    DAYNUM: i32,
    DAYPTR: i32,
    DAYTAB: ActualArray<i32>,
    DN2000: i32,
    DOFFST: i32,
    DOFYR: i32,
    DPSUN: i32,
    DYEAR: i32,
    FMDAY: i32,
    FYRDAY: i32,
    I: i32,
    MONTH: i32,
    NREF: i32,
    OFFSET: i32,
    PFROM: i32,
    PTO: i32,
    QINT: i32,
    REFPTR: i32,
    REM: i32,
    SUNDAY: i32,
    TAIPTR: i32,
    TEMPI: i32,
    USRCTR: StackArray<i32, 2>,
    WEEK: i32,
    WKDAY: i32,
    YEAR: i32,
    YR1: i32,
    YR100: i32,
    YR4: i32,
    YR400: i32,
    FIRST: bool,
    FOUND: bool,
    NODATA: bool,
    UPDATE: bool,
    EXTRA: StackArray<i32, 12>,
    DPBEGL: StackArray<i32, 12>,
    DPJAN0: StackArray<i32, 12>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut UNIFRM = ActualCharArray::new(TYPLEN, LBCELL..=NREC);
        let mut RECOG = ActualCharArray::new(TYPLEN, 1..=NREC);
        let mut PARSED = StackArray::<i32, 21>::new(1..=NREC);
        let mut NEEDY = StackArray::<bool, 21>::new(1..=NREC);
        let mut FORML = StackArray::<bool, 21>::new(1..=NREC);
        let mut ORDVEC = StackArray::<i32, 21>::new(1..=NREC);
        let mut REST = vec![b' '; 32 as usize];
        let mut MYFROM = vec![b' '; 32 as usize];
        let mut MYTO = vec![b' '; 32 as usize];
        let mut VARS = ActualCharArray::new(32, 1..=MAXVAR);
        let mut DAYDP: f64 = 0.0;
        let mut DAYLEN: f64 = 0.0;
        let mut DP2000: f64 = 0.0;
        let mut DT: f64 = 0.0;
        let mut EXSECS: f64 = 0.0;
        let mut FORMAL: f64 = 0.0;
        let mut FRAC: f64 = 0.0;
        let mut HALFD: f64 = 0.0;
        let mut HOURS: f64 = 0.0;
        let mut JD1101: f64 = 0.0;
        let mut JDSECS: f64 = 0.0;
        let mut LASTDT: f64 = 0.0;
        let mut MINS: f64 = 0.0;
        let mut SECS: f64 = 0.0;
        let mut SECSPD: f64 = 0.0;
        let mut TAI: f64 = 0.0;
        let mut TAITAB = ActualArray::<f64>::new(1..=(2 * MAXLP));
        let mut TEMPD: f64 = 0.0;
        let mut TSECS: f64 = 0.0;
        let mut DAY: i32 = 0;
        let mut DAYNUM: i32 = 0;
        let mut DAYPTR: i32 = 0;
        let mut DAYTAB = ActualArray::<i32>::new(1..=(2 * MAXLP));
        let mut DN2000: i32 = 0;
        let mut DOFFST: i32 = 0;
        let mut DOFYR: i32 = 0;
        let mut DPSUN: i32 = 0;
        let mut DYEAR: i32 = 0;
        let mut FMDAY: i32 = 0;
        let mut FYRDAY: i32 = 0;
        let mut I: i32 = 0;
        let mut MONTH: i32 = 0;
        let mut NREF: i32 = 0;
        let mut OFFSET: i32 = 0;
        let mut PFROM: i32 = 0;
        let mut PTO: i32 = 0;
        let mut QINT: i32 = 0;
        let mut REFPTR: i32 = 0;
        let mut REM: i32 = 0;
        let mut SUNDAY: i32 = 0;
        let mut TAIPTR: i32 = 0;
        let mut TEMPI: i32 = 0;
        let mut USRCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut WEEK: i32 = 0;
        let mut WKDAY: i32 = 0;
        let mut YEAR: i32 = 0;
        let mut YR1: i32 = 0;
        let mut YR100: i32 = 0;
        let mut YR4: i32 = 0;
        let mut YR400: i32 = 0;
        let mut FIRST: bool = false;
        let mut FOUND: bool = false;
        let mut NODATA: bool = false;
        let mut UPDATE: bool = false;
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
        NODATA = true;

        Self {
            UNIFRM,
            RECOG,
            PARSED,
            NEEDY,
            FORML,
            ORDVEC,
            REST,
            MYFROM,
            MYTO,
            VARS,
            DAYDP,
            DAYLEN,
            DP2000,
            DT,
            EXSECS,
            FORMAL,
            FRAC,
            HALFD,
            HOURS,
            JD1101,
            JDSECS,
            LASTDT,
            MINS,
            SECS,
            SECSPD,
            TAI,
            TAITAB,
            TEMPD,
            TSECS,
            DAY,
            DAYNUM,
            DAYPTR,
            DAYTAB,
            DN2000,
            DOFFST,
            DOFYR,
            DPSUN,
            DYEAR,
            FMDAY,
            FYRDAY,
            I,
            MONTH,
            NREF,
            OFFSET,
            PFROM,
            PTO,
            QINT,
            REFPTR,
            REM,
            SUNDAY,
            TAIPTR,
            TEMPI,
            USRCTR,
            WEEK,
            WKDAY,
            YEAR,
            YR1,
            YR100,
            YR4,
            YR400,
            FIRST,
            FOUND,
            NODATA,
            UPDATE,
            EXTRA,
            DPBEGL,
            DPJAN0,
        }
    }
}

fn YDAYS(YEAR: i32) -> i32 {
    ((((365 * (YEAR - 1)) + ((YEAR - 1) / 4)) - ((YEAR - 1) / 100)) + ((YEAR - 1) / 400))
}

fn DIVBLE(YEAR: i32, I: i32) -> i32 {
    intrinsics::MAX0(&[0, ((1 + ((i32::abs(YEAR) / I) * I)) - i32::abs(YEAR))])
}

fn LDAYS(YEAR: i32) -> i32 {
    ((DIVBLE(YEAR, 4) - DIVBLE(YEAR, 100)) + DIVBLE(YEAR, 400))
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

fn HMSSEC(HOURS: f64, MINS: f64, SECS: f64) -> f64 {
    (((HOURS * 3600.0) + (MINS * 60.0)) + SECS)
}

/// Time transformation
///
/// Transform a time vector from one representation and system
/// to another.
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
///  MXCOMP     P    maximum number of components allowed for TVEC.
///  TO         I    description of a time vector.
///  FROM       I    description of a time vector.
///  TVEC      I-O   time vector representing an epoch.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TVEC     is called a time vector. It is an array of double
///           precision numbers that represent some epoch. To
///           determine its meaning you must examine the string
///           FROM. Note that the number of significant entries
///           in TVEC is implied by FROM.
///
///  FROM,
///  TO       are two strings used to describe the type of time vector
///           TVEC. FROM is the type of the input vector TVEC and
///           TO is the type of the output TVEC
///
///           The interpretation of TVEC  is as follows:
///
///              TYPE      Interpretation of TVEC
///              ------    -------------------------------------------
///              YMD(F)  - year, month, day,   hour, minutes, seconds
///              YD(F)   - year,  day-of-year, hour, minutes, seconds
///              YD.D(F) - year, number of days past beginning of year
///              DAYSEC  - calendar days past 1 jan 1 AD,
///                        seconds past beg day
///              DP2000  - calendar days past 1 jan 2000,
///                        seconds past beg day
///              JDUTC   - julian date UTC.
///              FORMAL  - seconds in the formal calendar since J2000.
///              YWD(F)  - year, week, day, hour, minutes, seconds
///              YMWD(F) - year, month, week, day, hour, minutes,
///                        seconds
///              TAI     - atomic seconds past Atomic J2000.
///              TDT     - Terrestrial Dynamical Time
///              TDB     - Barycentric Dynamical Time
///              JED     - Julian Ephemeris Date (based on TDB)
///              ET      - Ephemeris time (same as TDB)
///              JDTDB   - Julian Date based on TDB (same as JED)
///              JDTDT   - Julian Date based on TDT
///
///           The number of components of TVEC implied by TYPE is
///           as follows:
///
///              YMD     - 6
///              YD      - 5
///              JDUTC   - 1
///              FORMAL  - 1
///              YD.D    - 2
///              DAYSEC  - 2
///              DP2000  - 2
///              YWD     - 6
///              YMWD    - 7
///              TAI     - 1
///              TDT     - 1
///              TDB     - 1
///              JED     - 1
///              ET      - 1
///              JDTDB   - 1
///              JDTDT   - 1
///
///
///           For all types, only the last component of the
///           time vector may be non-integer. If other components
///           have fractional parts only their truncated integer
///           components will be recognized.
///
///           YMD and YD
///
///              These types are assumed to be different
///              representations on UTC time markers. Thus
///              the hour, minutes and seconds portions all
///              represent time elapsed
///              since the beginning of a day. As such the
///              seconds portion of HMS may range up to (but
///              not include) 61 on days when positive leap
///              seconds occur and may range up to (but not
///              include) 59 on days during which negative
///              leapseconds occur.
///
///           YD.D type.
///
///              Y is the calendar year used in civil time keeping
///              D is the day of the calendar year --- for any time
///                during the first of January, the integer portion
///                of the day will be 1.
///
///                The fractional portion is the fractional part of
///                the specific day. Thus the amount of time
///                specified by the fractional portion of the day
///                depends upon whether or not the day has a leap
///                second.  ".D" can be computed from the formula
///
///                      number of seconds past beginning of day
///                .D = ---------------------------------------
///                         number of UTC seconds in the day.
///
///           FORMAL type.
///
///              The FORMAL type for TVEC gives the number of
///              seconds past the epoch J2000 (noon Jan 1 2000)
///              on the formal calendar (no leap seconds ---
///              all days contain 86400 seconds)  The formal clock
///              is simply held still for one second during
///              positive leap seconds. Times during leap seconds
///              cannot be represented in this system.
///
///              This system is converted internally to a
///              calendar days past epoch and seconds
///              past beginning of day form. For this reason,
///              times that occur during a positive leap second
///              can never be represented. Moreover, if a negative
///              leapsecond occurs, times that occur during the
///              ``missing'' leapsecond will simply be placed
///              at the beginning of the next day. Thus two
///              different FORMAL times can represent the
///              same time around a negative leap second.
///
///              FORMAL time is equivalent to somewhat parochial
///              ``UTC seconds past J2000'' that is produced
///              by the SPICE routine TPARSE.
///
///           JDUTC type.
///
///              This system is similar to the FORMAL system
///              described above. All days are assumed to have
///              86400 seconds. All numbers of the form
///
///                 integer + 0.5
///
///              fall at the beginning of calendar UTC days.
///
///              There is no way to represent times during a
///              positive leapsecond. Times during missing
///              negative leap seconds are represented in two ways.
///
///           DAYSEC type.
///
///              This time vector has the form of calendar
///              days since January 1, of the year 1 A.D.
///              and number of seconds past the beginning of the
///              calendar day.
///              (January 2 of the year 1 A.D. is 1 calendar
///              day past January 1, 1 A.D.)
///
///           DP2000 type.
///
///              This time vector has the same form as DAYSEC
///              time vectors. The only difference is that
///              the reference epoch is JAN 1, 2000.
///
///           YWD and YMWD types.
///
///              These time vectors are used to specify a time
///              that are most conveniently expressed by phrases
///              such as "the third Monday of every month" or
///              "Beginning with the second Wednesday of the new
///              year and every 4th Wednesday thereafter."
///
///              The hours, minutes and seconds components of
///              these time vectors are the
///              same as for the Year-Month-Day and Year-Day UTC
///              time vectors.
///
///              The Y component refers to the calendar year, and
///              in the YMWD vector, the M component refers to
///              the calendar month.
///
///              The W component refers to the week of the
///              Year (YWD) or Month (YMWD).  The first week
///              begins on the first day of the year or the first
///              day of the month. The D component is the day of the
///              week with 1 corresponding to Sunday, 2 to Monday,
///              and so on with 7 corresponding to Saturday.
///
///              Thus the YMWD time vector
///
///                 1991
///                   11
///                    3
///                    5
///                   12
///                    0
///                    0
///
///              refers to 12:00:00 on the third Thursday of
///              November of 1991.
///
///              The YWD time vector
///
///                 1997
///                   11
///                    4
///                   13
///                    5
///                   11
///
///              refers to 12:05:11 on the eleventh Wednesday
///              of 1997.
///
///           Formal Calendar Time Vectors
///           ============================
///           The types YMDF, YDF, YD.D(F), YWDF, YMWDF are similar
///           to the corresponding base types: YMD, YD, YD.D, YWD
///           and YMWD. However, these types represent formal
///           time vectors. Each day contains exactly 86400 seconds.
///           The difference between formal and non-formal systems
///           can only be seen during a positive leapsecond or
///           during the second following a negative leapsecond.
///
///           Epochs during a positive leapsecond on input are
///           placed in the first second of the next day. Epochs
///           during a positive leapsecond on output are held
///           at 00:00:00 of the next day.
///
///           Epochs during the first second following a negative
///           leapsecond are counted as belonging to the previous
///           day if both the input and output types are formal
///           types.
///
///
///           Calendars
///           =====================
///           In all time vectors for which a year is specified,
///           the year is assumed to belong to the Gregorian
///           Calendar---every 4th year is a leapyear except
///           for centuries (such as 1900) that are not divisible
///           by 400. This calendar is formally extended
///           indefinitely backward and forward in time.
///
///           Note that the Gregorian Calendar did not
///           formally exist prior to October 15, 1582. Prior to
///           that time the Julian Calendar was used (in the
///           Julian Calendar every 4th year is a leapyear, including
///           all centuries).
///
///           If you have epochs relative to the Julian calendar,
///           the SPICE routine JUL2GR is available for converting
///           to the formal Gregorian Calendar.
///
///
///           Epochs Prior to 1972
///           =====================
///           UTC as it exists today, was adopted in 1972. For
///           epochs prior to 1972, it is assumed that the difference
///           between TAI and UTC is a constant value.
///
///           Years prior to 1 A.D.
///           =====================
///           A year belonging to the B.C. era,  may be
///           represented by subtracting the year from 1.
///           Thus to specify 27 B.C (Gregorian) set the
///           year component of the time vector to -26.
///
///
///           Notes:
///           ======
///           The FORMAL and JDUTC types should not be used
///           for times near a leap second. However, for times
///           removed from leap seconds they pose no problems.
///
///           The DAYSEC and DP2000 are useful for representing
///           times that are given in atomic seconds past some
///           reference epoch other than J2000.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TVEC     is the time vector corresponding to the input
///           time vector but with components consistent with
///           the type specified by input variable TO.
/// ```
///
/// # Parameters
///
/// ```text
///  MXCOMP   is the maximum number of components that can appear in
///           TVEC.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the type of either FROM or TO is not recognized, the
///      error SPICE(UNKNONWNTIMESYSTEM) is signaled.
///
///  2)  If a leapseconds kernel has not been loaded prior a call
///      to TTRANS, the error  SPICE(NOLEAPSECONDS) is signaled.
///
///  3)  If epochs associated with leapseconds in the leapseconds
///      kernel are not in increasing order, the error
///      SPICE(BADLEAPSECONDS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is the fundamental translator between various
///  representations of time in the SPICE system. However, it
///  is intended to be a mid-level routine that few user's should
///  have need of calling.
///
///  In addition to translating between time systems, this routine
///  can be used to normalize the components of a time string
///  so that they are in the normal range for a particular
///  representation. This allows you to easily do arithmetic
///  with epochs.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you need to convert a time expressed as seconds
///  past J2000 (TDB) to Pacific Daylight time. The following
///  example shows how you might use TTRANS to accomplish this
///  task.
///
///   TVEC(1) = ET
///
///   CALL TTRANS ( 'TDB', 'YMD', TVEC )
///
///   The seconds component of PDT is the same as the seconds
///   component of UTC. We save and add the UTC-PDT offset
///   to the hours and minutes component of the time vector.
///
///   SECNDS  = TVEC(6)
///   TVEC(6) = 0.0D0
///
///   TVEC(4) = TVEC(4) - 7.0D0
///   TVEC(5) = TVEC(5) + 0.0D0
///
///   CALL TTRANS ( 'YMDF', 'YMDF', TVEC )
///
///   Now reset the seconds component to the original value
///   and pass the time vector to some formatting routine.
///
///   TVEC(6) = SECNDS
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.6.0, 05-SEP-2021 (EDW) (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Removed INT casts in HMSSEC calls. The casts prevent
///         correct calculation of TDB time for non integer hour
///         and minute values in time strings from STR2ET.
///
///         Removed reference to FURNSH from the "LSK variable
///         not present" long error message.
///
/// -    SPICELIB Version 1.5.0, 09-SEP-2013 (BVS)
///
///         Updated to keep track of the POOL counter and call ZZCVPOOL.
///
/// -    SPICELIB Version 1.4.0, 05-MAR-2009 (NJB)
///
///         Bug fix: this routine now keeps track of whether its
///         kernel pool look-up succeeded. If not, a kernel pool
///         lookup is attempted on the next call to this routine.
///
/// -    SPICELIB Version 1.3.0, 15-NOV-2006 (NJB)
///
///         A reference to RTPOOL was replaced by a reference
///         to GDPOOL.
///
/// -    SPICELIB Version 1.2.0, 24-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in RMAIND and RMAINI calls. Changed reference to LDPOOL to
///         reference to FURNSH in an error message.
///
/// -    SPICELIB Version 1.1.0, 09-JUN-1999 (WLT)
///
///         The routine was modified so that uniform time system
///         transformations (see UNITIM) are handled without
///         performing intermediate computations. This gives a slight
///         improvement in the accuracy of some computations.
///
///         In addition, two unused variables were removed.
///
/// -    SPICELIB Version 1.0.0, 17-SEP-1996 (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.2.0, 24-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in RMAIND and RMAINI calls. Changed reference to LDPOOL to
///         reference to FURNSH in an error message.
/// ```
pub fn ttrans(
    ctx: &mut SpiceContext,
    from: &str,
    to: &str,
    tvec: &mut [f64; 7],
) -> crate::Result<()> {
    TTRANS(from.as_bytes(), to.as_bytes(), tvec, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TTRANS ( Time transformation )
pub fn TTRANS(
    FROM: &[u8],
    TO: &[u8],
    TVEC: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TVEC = DummyArrayMut::new(TVEC, 1..=MXCOMP);

    //
    // SPICELIB functions
    //

    //
    // Local (in-line) functions
    //

    //
    // Local parameters
    //
    //
    // Parameters
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
    // MAXLP is the maximum number of leap seconds that can be
    // stored internally.   The value of 140 should be sufficient
    // to store leap seconds through the year 2100.
    //

    //
    // MAXVAR is the number of kernel pool variables required by this
    // routine.
    //

    //
    //
    // The following gives us an "enumeration" for all of the
    // various types of time vectors that are recognized.
    //
    // DAYSEC
    // DAYP2
    // ET
    // FRML
    // JDTDB
    // JDTDT
    // JDUTC
    // JED
    // TAI
    // TDB
    // TDT
    // YD
    // YDD
    // YDDF
    // YDF
    // YMD
    // YMDF
    // YMWD
    // YMWDF
    // YWD
    // YWDF
    //

    //
    // The following parameters just make the code seem a bit
    // more natural.
    //

    //
    // Local variables
    //

    //
    // The array EXTRA contains the number of many additional days that
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
    // The number of days elapsed since Jan 1, of year 1 A.D. to
    // Jan 1 of YEAR is given by:
    //

    //
    // Return 1 if YEAR is divisible by N, otherwise return 0.
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

    //
    // The number of seconds represented by HOURS hours MINS minutes
    // and SECS seconds.
    //

    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"TTRANS", ctx)?;
    }

    //
    // The first time any of the entry points are called we
    // must set up the "watcher" for the kernel pool variables
    // that will be needed by this routine.
    //
    if save.FIRST {
        save.FIRST = false;

        save.SECSPD = SPD();
        save.HALFD = (SPD() / 2.0);

        save.DN2000 = DP0001(2000, 1, 1, save.DPJAN0.as_slice(), save.EXTRA.as_slice());
        save.SUNDAY = DP0001(1991, 1, 6, save.DPJAN0.as_slice(), save.EXTRA.as_slice());

        save.JD1101 = ((J2000() - (save.DN2000 as f64)) - 0.5);

        //
        // Initialize the list of Uniform time systems.
        //
        SSIZEC(NREC, save.UNIFRM.as_arg_mut(), ctx)?;

        //
        // Set up the set of recognized time vectors.
        //
        // The following 4 parallel arrays are here
        // to assist in the task of classifying the
        // FROM and TO time representations. The arrays
        // contain:
        //
        // RECOG   the strings that are recognized as legitimate
        //         time representations
        //
        // PARSED  a unique integer that can be used to stand
        //         for each recognized format.  This is used
        //         in the various IF THEN blocks to decide
        //         how a time vector should be processed instead
        //         of the name because integer compares are
        //         much faster than string comparisons.
        //
        // FORML   is a logical that indicates whether or not the
        //         corresponding time system is a formal system
        //         or UTC based system.  FORML(I) = YES implies
        //         the time system is formal.  FORML(I) means it
        //         isn't.
        //
        // NEEDY   is a logical that indicates whether or not
        //         there is a YEAR in the time system.  It should
        //         be read "NEED Y" for "need year"  not "needy"
        //         as when you are destitute.  NEEDY(I) = YES means
        //         the time system has a year.  NEEDY(I) = NO means
        //         it doesn't
        //
        fstr::assign(save.RECOG.get_mut(1), b"DAYSEC ");
        save.PARSED[1] = DAYSEC;
        save.FORML[1] = NO;
        save.NEEDY[1] = NO;

        fstr::assign(save.RECOG.get_mut(2), b"DP2000 ");
        save.PARSED[2] = DAYP2;
        save.FORML[2] = NO;
        save.NEEDY[2] = NO;

        fstr::assign(save.RECOG.get_mut(3), b"ET ");
        save.PARSED[3] = ET;
        save.FORML[3] = NO;
        save.NEEDY[3] = NO;

        INSRTC(b"ET", save.UNIFRM.as_arg_mut(), ctx)?;

        fstr::assign(save.RECOG.get_mut(4), b"FORMAL ");
        save.PARSED[4] = FRML;
        save.FORML[4] = YES;
        save.NEEDY[4] = NO;

        fstr::assign(save.RECOG.get_mut(5), b"JDTDB ");
        save.PARSED[5] = JDTDB;
        save.FORML[5] = NO;
        save.NEEDY[5] = NO;

        INSRTC(b"JDTDB", save.UNIFRM.as_arg_mut(), ctx)?;

        fstr::assign(save.RECOG.get_mut(6), b"JDTDT ");
        save.PARSED[6] = JDTDT;
        save.FORML[6] = NO;
        save.NEEDY[6] = NO;

        INSRTC(b"JDTDT", save.UNIFRM.as_arg_mut(), ctx)?;

        fstr::assign(save.RECOG.get_mut(7), b"JDUTC ");
        save.PARSED[7] = JDUTC;
        save.FORML[7] = YES;
        save.NEEDY[7] = NO;

        fstr::assign(save.RECOG.get_mut(8), b"JED ");
        save.PARSED[8] = JED;
        save.FORML[8] = NO;
        save.NEEDY[8] = NO;

        INSRTC(b"JED", save.UNIFRM.as_arg_mut(), ctx)?;

        fstr::assign(save.RECOG.get_mut(9), b"TAI ");
        save.PARSED[9] = TTAI;
        save.FORML[9] = NO;
        save.NEEDY[9] = NO;

        INSRTC(b"TAI", save.UNIFRM.as_arg_mut(), ctx)?;

        fstr::assign(save.RECOG.get_mut(10), b"TDB ");
        save.PARSED[10] = TDB;
        save.FORML[10] = NO;
        save.NEEDY[10] = NO;

        INSRTC(b"TDB", save.UNIFRM.as_arg_mut(), ctx)?;

        fstr::assign(save.RECOG.get_mut(11), b"TDT ");
        save.PARSED[11] = TDT;
        save.FORML[11] = NO;
        save.NEEDY[11] = NO;

        INSRTC(b"TDT", save.UNIFRM.as_arg_mut(), ctx)?;

        fstr::assign(save.RECOG.get_mut(12), b"YD ");
        save.PARSED[12] = YD;
        save.FORML[12] = NO;
        save.NEEDY[12] = YES;

        fstr::assign(save.RECOG.get_mut(13), b"YD.D ");
        save.PARSED[13] = YDD;
        save.FORML[13] = NO;
        save.NEEDY[13] = YES;

        fstr::assign(save.RECOG.get_mut(14), b"YD.DF ");
        save.PARSED[14] = YDDF;
        save.FORML[14] = YES;
        save.NEEDY[14] = YES;

        fstr::assign(save.RECOG.get_mut(15), b"YDF ");
        save.PARSED[15] = YDF;
        save.FORML[15] = YES;
        save.NEEDY[15] = YES;

        fstr::assign(save.RECOG.get_mut(16), b"YMD ");
        save.PARSED[16] = YMD;
        save.FORML[16] = NO;
        save.NEEDY[16] = YES;

        fstr::assign(save.RECOG.get_mut(17), b"YMDF ");
        save.PARSED[17] = YMDF;
        save.FORML[17] = YES;
        save.NEEDY[17] = YES;

        fstr::assign(save.RECOG.get_mut(18), b"YMWD ");
        save.PARSED[18] = YMWD;
        save.FORML[18] = NO;
        save.NEEDY[18] = YES;

        fstr::assign(save.RECOG.get_mut(19), b"YMWDF ");
        save.PARSED[19] = YMWDF;
        save.FORML[19] = YES;
        save.NEEDY[19] = YES;

        fstr::assign(save.RECOG.get_mut(20), b"YWD ");
        save.PARSED[20] = YWD;
        save.FORML[20] = NO;
        save.NEEDY[20] = YES;

        fstr::assign(save.RECOG.get_mut(21), b"YWDF ");
        save.PARSED[21] = YWDF;
        save.FORML[21] = YES;
        save.NEEDY[21] = YES;

        ORDERC(save.RECOG.as_arg(), NREC, save.ORDVEC.as_slice_mut());
        REORDC(save.ORDVEC.as_slice_mut(), NREC, save.RECOG.as_arg_mut());
        REORDI(save.ORDVEC.as_slice_mut(), NREC, save.PARSED.as_slice_mut());
        REORDL(save.ORDVEC.as_slice_mut(), NREC, save.FORML.as_slice_mut());
        REORDL(save.ORDVEC.as_slice_mut(), NREC, save.NEEDY.as_slice_mut());

        //
        // Initialize the local POOL counter to user value.
        //
        ZZCTRUIN(save.USRCTR.as_slice_mut(), ctx);

        //
        // Set up the kernel pool watchers
        //
        fstr::assign(save.VARS.get_mut(1), b"DELTET/DELTA_AT");

        SWPOOL(b"TTRANS", 1, save.VARS.as_arg(), ctx)?;
    }

    //
    // Check to see if any of the kernel items required by this
    // routine have been updated since the last call to this
    // entry point.
    //
    ZZCVPOOL(b"TTRANS", save.USRCTR.as_slice_mut(), &mut save.UPDATE, ctx)?;

    if (save.UPDATE || save.NODATA) {
        //
        // We load the TAI-UTC offsets and formal leapsecond epochs
        // into the TAITAB.  (We will modify this array in a minute).
        //
        GDPOOL(
            b"DELTET/DELTA_AT",
            1,
            (2 * MAXLP),
            &mut save.NREF,
            save.TAITAB.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;

        //
        // Make sure all of the requested data was there.
        //
        if !save.FOUND {
            save.NODATA = true;

            SETMSG(b"The variable that points to the leapseconds (DELTET/DELTA_AT) could not be located in the kernel pool.  It is likely that the leapseconds kernel has not been loaded.", ctx);
            SIGERR(b"SPICE(NOLEAPSECONDS)", ctx)?;
            CHKOUT(b"TTRANS", ctx)?;
            return Ok(());
        }

        //
        // Transform the TAITAB in place to give the TAI time tag
        // at the beginning of the UTC day in which a leap
        // second occurred and the TAI time tag at the beginning
        // of the next day.  Pictorially, the table is transformed
        //
        //        +----------------------+         +-------------------+
        //        | DELTA_1 (TAI to UTC) |         | TAI at start of   |
        //        |                      |         | day before TAI-UTC|
        //        |                      |         | change occurred   |
        //        +----------------------+         +-------------------+
        // from:  | First Formal time    |     to: | TAI time at start |
        //        | associated with      |         | of next day UTC.  |
        //        | DELTA_1              |         | after DELTA_1 jump|
        //        +----------------------+         +-------------------+
        //        | DELTA_2 (TAI to UTC) |         | TAI at start of   |
        //        |                      |         | day before TAI-UTC|
        //        |                      |         | jump occurred     |
        //        +----------------------+         +-------------------+
        //        | First Formal time    |         | TAI time at start |
        //        | associated with      |         | of next day UTC.  |
        //        | DELTA_2              |         | after DELTA_2 jump|
        //        +----------------------+         +-------------------+
        //                 .                                .
        //                 .                                .
        //                 .                                .
        //
        //
        // At the same time, load the table DAYTAB. It contains the
        // the day number past 1 Jan 1 AD for the beginning of the
        // days loaded in TAITAB.
        //
        save.LASTDT = (save.TAITAB[1] - 1.0);

        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NREF;
            let m3__: i32 = 2;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.OFFSET = save.I;
                save.REFPTR = (save.I + 1);

                save.DT = save.TAITAB[save.OFFSET];
                save.FORMAL = save.TAITAB[save.REFPTR];
                save.TAITAB[save.OFFSET] = ((save.FORMAL - save.SECSPD) + save.LASTDT);
                save.TAITAB[save.REFPTR] = (save.FORMAL + save.DT);

                save.DAYNUM = ((((save.FORMAL + save.HALFD) / save.SECSPD) as i32) + save.DN2000);

                save.DAYTAB[save.OFFSET] = (save.DAYNUM - 1);
                save.DAYTAB[save.REFPTR] = save.DAYNUM;

                save.LASTDT = save.DT;

                save.I += m3__;
            }
        }

        //
        // Since we don't have to do it very often, make sure the
        // times in the TAI table are in increasing order.
        //
        {
            let m1__: i32 = 2;
            let m2__: i32 = save.NREF;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.NODATA = true;

                if (save.TAITAB[(save.I - 1)] >= save.TAITAB[save.I]) {
                    SETMSG(b"Either the leapsecond epochs taken from the kernel pool are not properly ordered or the UTC - TAI offsets are completely out of range. ", ctx);
                    SIGERR(b"SPICE(BADLEAPSECONDS)", ctx)?;
                    CHKOUT(b"TTRANS", ctx)?;
                    return Ok(());
                }

                save.I += m3__;
            }
        }

        //
        // At this point, we've completed all checks on kernel data.
        //
        save.NODATA = false;
    }

    //
    // Make local normalized copies of FROM and TO.
    //
    NEXTWD(FROM, &mut save.MYFROM, &mut save.REST);
    NEXTWD(TO, &mut save.MYTO, &mut save.REST);
    UCASE(&save.MYFROM.to_vec(), &mut save.MYFROM, ctx);
    UCASE(&save.MYTO.to_vec(), &mut save.MYTO, ctx);

    //
    // Make sure that the FROM and TO are recognized time types.
    //
    save.PTO = BSRCHC(&save.MYTO, NREC, save.RECOG.as_arg());
    save.PFROM = BSRCHC(&save.MYFROM, NREC, save.RECOG.as_arg());

    //
    // Eventually, we need to handle SCLKs.  When that happens
    // we will do it here and in a similarly marked spot at
    // the end of this routine.  First see if we know how to
    // handle the FROM system.
    //
    // IF ( PFROM .EQ. 0 ) THEN
    //
    //    CALL ISSCLK ( FROM,ERROR, FOUND )
    //
    //    IF ( .NOT. FOUND ) THEN
    //       IF ( ERROR .NE. ' ' ) THEN
    //          CALL SETMSG ( ERROR )
    //          CALL SIGERR ( 'SPICE(TIMESYSTEMPROBLEM)' )
    //          CALL CHKOUT ( 'TTRANS' )
    //          RETURN
    //       END IF
    //    ELSE
    //       CALL SCLKTV ( FROM, TVEC )
    //       PFROM = TDB
    //    END IF
    //
    // END IF
    //
    // Now check to see if we know how to handle the  TO system.
    //
    // IF ( PTO .EQ. 0 ) THEN
    //
    //    CALL ISSCLK ( TO, ERROR, FOUND )
    //
    //    IF ( .NOT. FOUND ) THEN
    //
    //       IF ( ERROR .NE. ' ' ) THEN
    //          CALL SETMSG ( ERROR )
    //          CALL SIGERR ( 'SPICE(TIMESYSTEMPROBLEM)' )
    //          CALL CHKOUT ( 'TTRANS' )
    //       END IF
    //
    //    ELSE
    //
    //       MKSCLK = .TRUE.
    //       PTO    =  TDB
    //
    //    END IF
    //
    // END IF
    //
    //
    // For now we are NOT going to deal with SCLK so if something
    // isn't recognized, we can just signal an error and quit.
    //
    if (save.PFROM == 0) {
        SETMSG(
            b"The FROM time representation \'#\' is not recognized. ",
            ctx,
        );
        ERRCH(b"#", FROM, ctx);
        SIGERR(b"SPICE(UNKNONWNTIMESYSTEM)", ctx)?;
        CHKOUT(b"TTRANS", ctx)?;
        return Ok(());
    } else if (save.PTO == 0) {
        SETMSG(b"The TO time representation \'#\' is not recognized. ", ctx);
        ERRCH(b"#", FROM, ctx);
        SIGERR(b"SPICE(UNKNONWNTIMESYSTEM)", ctx)?;
        CHKOUT(b"TTRANS", ctx)?;
        return Ok(());
    }

    //
    // OK.  We have made our last attempt at diagnosing a user error.
    // From this point on we assume that the user input exactly what
    // was intended.
    //
    // We convert the time vector to days past 1 jan 01 and seconds
    // past the beginning of the day.  None of the cases below
    // are particularly tricky.  There's just a lot of cases.
    //
    if ((save.PFROM == YMD) || (save.PFROM == YMDF)) {
        save.YEAR = (TVEC[1] as i32);
        save.MONTH = (TVEC[2] as i32);
        save.DAY = (TVEC[3] as i32);

        RMAINI((save.MONTH - 1), 12, &mut save.DYEAR, &mut save.MONTH, ctx)?;

        save.YEAR = (save.YEAR + save.DYEAR);
        save.MONTH = (save.MONTH + 1);
        save.DOFFST = 0;

        if (save.YEAR <= 0) {
            RMAINI(save.YEAR, 400, &mut save.YR400, &mut save.TEMPI, ctx)?;
            save.YEAR = save.TEMPI;

            if (save.YEAR == 0) {
                save.YEAR = (save.YEAR + 400);
                save.YR400 = (save.YR400 - 1);
            }

            save.DOFFST = (DP400Y * save.YR400);
        }

        save.DAYNUM = (DP0001(
            save.YEAR,
            save.MONTH,
            save.DAY,
            save.DPJAN0.as_slice(),
            save.EXTRA.as_slice(),
        ) + save.DOFFST);

        //
        // Calculate seconds from midnight, 00:00:00.
        //
        save.SECS = HMSSEC(TVEC[4], TVEC[5], TVEC[6]);
    } else if ((save.PFROM == YD) || (save.PFROM == YDF)) {
        save.YEAR = (TVEC[1] as i32);
        save.DAY = (TVEC[2] as i32);
        save.MONTH = 1;

        save.DOFFST = 0;

        if (save.YEAR <= 0) {
            RMAINI(save.YEAR, 400, &mut save.YR400, &mut save.TEMPI, ctx)?;
            save.YEAR = save.TEMPI;

            if (save.YEAR == 0) {
                save.YEAR = (save.YEAR + 400);
                save.YR400 = (save.YR400 - 1);
            }

            save.DOFFST = (DP400Y * save.YR400);
        }

        save.DAYNUM = (DP0001(
            save.YEAR,
            save.MONTH,
            save.DAY,
            save.DPJAN0.as_slice(),
            save.EXTRA.as_slice(),
        ) + save.DOFFST);

        //
        // Calculate seconds from midnight, 00:00:00.
        //
        save.SECS = HMSSEC(TVEC[3], TVEC[4], TVEC[5]);
    } else if ((save.PFROM == YDD) || (save.PFROM == YDDF)) {
        save.YEAR = (TVEC[1] as i32);
        save.DAY = (TVEC[2] as i32);
        save.MONTH = 1;

        save.DOFFST = 0;

        if (save.YEAR <= 0) {
            RMAINI(save.YEAR, 400, &mut save.YR400, &mut save.TEMPI, ctx)?;
            save.YEAR = save.TEMPI;

            if (save.YEAR == 0) {
                save.YEAR = (save.YEAR + 400);
                save.YR400 = (save.YR400 - 1);
            }

            save.DOFFST = (DP400Y * save.YR400);
        }

        save.FRAC = (TVEC[2] - (save.DAY as f64));
        save.DAYNUM = (DP0001(
            save.YEAR,
            save.MONTH,
            save.DAY,
            save.DPJAN0.as_slice(),
            save.EXTRA.as_slice(),
        ) + save.DOFFST);

        //
        // Normally the length of a day is 86400 seconds, but this day
        // might be a leapsecond day.  We will set DAYLEN to SECSPD and
        // change it if it turns out this is a day with a leapsecond.
        //
        if (save.PFROM == YDDF) {
            save.SECS = (save.FRAC * save.SECSPD);
        } else {
            save.DAYLEN = save.SECSPD;
            save.DAYPTR = LSTLEI(save.DAYNUM, save.NREF, save.DAYTAB.as_slice());

            if ODD(save.DAYPTR) {
                save.DAYLEN = (save.TAITAB[(save.DAYPTR + 1)] - save.TAITAB[save.DAYPTR]);
            }

            save.SECS = (save.FRAC * save.DAYLEN);
        }
    } else if (save.PFROM == FRML) {
        //
        // First lets get the number of days since 1-Jan-2000 00:00:00
        //
        RMAIND(
            (TVEC[1] + save.HALFD),
            save.SECSPD,
            &mut save.DP2000,
            &mut save.SECS,
            ctx,
        )?;

        save.DAYNUM = ((save.DP2000 as i32) + save.DN2000);
    } else if (save.PFROM == JDUTC) {
        //
        // JD1101 is the julian date UTC of Jan 1, 1 AD.
        //
        save.JDSECS = ((TVEC[1] - save.JD1101) * save.SECSPD);

        RMAIND(
            save.JDSECS,
            save.SECSPD,
            &mut save.DAYDP,
            &mut save.SECS,
            ctx,
        )?;

        save.DAYNUM = (save.DAYDP as i32);
    } else if (save.PFROM == DAYSEC) {
        save.DAYNUM = (TVEC[1] as i32);
        save.SECS = TVEC[2];
    } else if (save.PFROM == DAYP2) {
        save.DAYNUM = ((TVEC[1] as i32) + save.DN2000);
        save.SECS = TVEC[2];
    } else if ((save.PFROM == YWD) || (save.PFROM == YWDF)) {
        save.YEAR = (TVEC[1] as i32);
        save.WEEK = ((TVEC[2] as i32) - 1);
        save.WKDAY = (TVEC[3] as i32);
        save.MONTH = 1;

        //
        // Compute the days past 1 jan 1 of the beginning of this
        // year and month.
        //

        save.DOFFST = 0;

        if (save.YEAR <= 0) {
            RMAINI(save.YEAR, 400, &mut save.YR400, &mut save.TEMPI, ctx)?;
            save.YEAR = save.TEMPI;

            if (save.YEAR == 0) {
                save.YEAR = (save.YEAR + 400);
                save.YR400 = (save.YR400 - 1);
            }

            save.DOFFST = (DP400Y * save.YR400);
        }

        save.DAYNUM = (DP0001(
            save.YEAR,
            save.MONTH,
            1,
            save.DPJAN0.as_slice(),
            save.EXTRA.as_slice(),
        ) + save.DOFFST);

        RMAINI(
            (save.DAYNUM - save.SUNDAY),
            7,
            &mut save.QINT,
            &mut save.DPSUN,
            ctx,
        )?;

        save.FYRDAY = (save.DPSUN + 1);

        RMAINI(
            (save.WKDAY - save.FYRDAY),
            7,
            &mut save.QINT,
            &mut save.OFFSET,
            ctx,
        )?;

        save.DAYNUM = ((save.DAYNUM + (save.WEEK * 7)) + save.OFFSET);

        //
        // Calculate seconds from midnight, 00:00:00.
        //
        save.SECS = HMSSEC(TVEC[4], TVEC[5], TVEC[6]);
    } else if ((save.PFROM == YMWD) || (save.PFROM == YMWDF)) {
        save.YEAR = (TVEC[1] as i32);
        save.MONTH = (TVEC[2] as i32);
        save.WEEK = ((TVEC[3] as i32) - 1);
        save.DAY = (TVEC[4] as i32);

        save.DOFFST = 0;

        if (save.YEAR <= 0) {
            RMAINI(save.YEAR, 400, &mut save.YR400, &mut save.TEMPI, ctx)?;
            save.YEAR = save.TEMPI;

            if (save.YEAR == 0) {
                save.YEAR = (save.YEAR + 400);
                save.YR400 = (save.YR400 - 1);
            }

            save.DOFFST = (DP400Y * save.YR400);
        }

        save.DAYNUM = (DP0001(
            save.YEAR,
            save.MONTH,
            1,
            save.DPJAN0.as_slice(),
            save.EXTRA.as_slice(),
        ) + save.DOFFST);

        RMAINI(
            (save.DAYNUM - save.SUNDAY),
            7,
            &mut save.QINT,
            &mut save.DPSUN,
            ctx,
        )?;
        save.FMDAY = (save.DPSUN + 1);

        RMAINI(
            (save.DAY - save.FMDAY),
            7,
            &mut save.QINT,
            &mut save.OFFSET,
            ctx,
        )?;

        save.DAYNUM = ((save.DAYNUM + (save.WEEK * 7)) + save.OFFSET);

        //
        // Calculate seconds from midnight, 00:00:00.
        //
        save.SECS = HMSSEC(TVEC[5], TVEC[6], TVEC[7]);

    //
    // If we get to this point the type must be one of the continuous
    // time types: 'TAI', 'TDT', 'TDB', 'JED', 'ET', 'JDTDT', 'JDTDB'.
    //
    } else {
        //
        // If the output time is one of the continuous time systems
        // we can take a short cut and just perform the computation
        // directly.
        //
        if ELEMC(&save.MYTO, save.UNIFRM.as_arg(), ctx)? {
            TVEC[1] = UNITIM(TVEC[1], &save.MYFROM, &save.MYTO, ctx)?;
            CHKOUT(b"TTRANS", ctx)?;
            return Ok(());
        }

        //
        // The output time system isn't one of the uniform time systems.
        // Convert what we have to TAI and then to the DAYNUM, SECOND
        // representation.
        //
        save.TAI = UNITIM(TVEC[1], &save.MYFROM, b"TAI", ctx)?;
        save.TAIPTR = LSTLED(save.TAI, save.NREF, save.TAITAB.as_slice());

        //
        // If the TAIPTR value is odd, then the TAI time falls during
        // a day with a leap second.  We can just look up the day
        // number and compute the number of seconds into that
        // day directly ...
        //
        if ODD(save.TAIPTR) {
            save.DAYNUM = save.DAYTAB[save.TAIPTR];
            save.SECS = (save.TAI - save.TAITAB[save.TAIPTR]);

        //
        // ...Otherwise, all days since the reference TAI time have
        // the same number of seconds (SECSPD).  (This statement applies
        // to days that precede the first reference TAI time too.)
        // Thus we can simply compute the number of days and seconds
        // that have elapsed since the reference TAI time.
        //
        } else {
            //
            // If TAI is before the first time in the table, we can
            // compute the number of days and seconds before the first
            // entry in the TAI table.
            //
            save.TAIPTR = intrinsics::MAX0(&[save.TAIPTR, 1]);

            RMAIND(
                (save.TAI - save.TAITAB[save.TAIPTR]),
                save.SECSPD,
                &mut save.DAYDP,
                &mut save.SECS,
                ctx,
            )?;

            save.DAYNUM = ((save.DAYDP as i32) + save.DAYTAB[save.TAIPTR]);
        }
    }

    if save.FORML[save.PFROM] {
        RMAIND(
            save.SECS,
            save.SECSPD,
            &mut save.DAYDP,
            &mut save.TSECS,
            ctx,
        )?;
        save.DAYNUM = (save.DAYNUM + (save.DAYDP as i32));
        save.SECS = save.TSECS;
    }

    // ==================================================================
    //
    // Force the seconds into the range 0 to 86401 or 86400
    // depending upon whether or not the output system is a formal
    // time system or not.
    //
    if (save.FORML[save.PTO] && save.FORML[save.PFROM]) {
        //
        // We don't have to do anything here.
        //
    } else {
        if ((save.SECS > (save.SECSPD - 1.0)) || (save.SECS < 0.0)) {
            //
            // First convert to TAI...
            //
            save.DAYPTR =
                intrinsics::MAX0(&[1, LSTLEI(save.DAYNUM, save.NREF, save.DAYTAB.as_slice())]);
            save.SECS =
                (save.SECS + (((save.DAYNUM - save.DAYTAB[save.DAYPTR]) as f64) * save.SECSPD));
            save.TAI = (save.TAITAB[save.DAYPTR] + save.SECS);

            //
            // ...then back to DAYNUM and SECS
            //
            save.TAIPTR = LSTLED(save.TAI, save.NREF, save.TAITAB.as_slice());

            if ODD(save.TAIPTR) {
                save.DAYNUM = save.DAYTAB[save.TAIPTR];
                save.SECS = (save.TAI - save.TAITAB[save.TAIPTR]);
            } else {
                save.TAIPTR = intrinsics::MAX0(&[1, save.TAIPTR]);
                save.DAYNUM = save.DAYTAB[save.TAIPTR];

                RMAIND(
                    (save.TAI - save.TAITAB[save.TAIPTR]),
                    save.SECSPD,
                    &mut save.DAYDP,
                    &mut save.SECS,
                    ctx,
                )?;

                save.DAYNUM = (save.DAYNUM + (save.DAYDP as i32));
            }
        }
    }
    //
    // One last thing.  If we are going to a formal time vector,
    // we want to ignore positive leapseconds. (Negative ones
    // were handled above, the clock jumped ahead one second
    // when the second hand got to 59.)
    //
    // The idea is that we want the clock
    // to stand still during the leapsecond.  Yeah this is bogus,
    // but people with analog clocks don't have any other choice.
    //
    // We are in a positive leapsecond only if SECS is greater than
    // the number of seconds in a normal day.  In that case we
    // increment the day number by one and set SECS to zero.
    //

    if (save.FORML[save.PTO] && (save.SECS > save.SECSPD)) {
        save.DAYNUM = (save.DAYNUM + 1);
        save.SECS = 0 as f64;
    }
    //
    // OK. Now we have DAYNUM and SECS,  convert this form to the
    // one requested.
    //
    // If there is a 'Y' in the form we are to convert to, then we
    // will need some form of year, etc.  Do the work now and sort it
    // it all out at the appropriate time later on.
    //
    if save.NEEDY[save.PTO] {
        save.YR400 = (save.DAYNUM / DP400Y);
        save.REM = (save.DAYNUM - (DP400Y * save.YR400));
        //
        // We want to be able to deal with years prior to  1 Jan 1
        // So we make sure the remainder is positive.
        //
        if (save.REM < 0) {
            save.YR400 = (save.YR400 - 1);
            save.REM = (save.REM + DP400Y);
        }

        save.YR100 = intrinsics::MIN0(&[3, (save.REM / DP100Y)]);
        save.REM = (save.REM - (save.YR100 * DP100Y));

        save.YR4 = intrinsics::MIN0(&[24, (save.REM / DP4Y)]);
        save.REM = (save.REM - (save.YR4 * DP4Y));

        save.YR1 = intrinsics::MIN0(&[3, (save.REM / DP1Y)]);
        save.REM = (save.REM - (save.YR1 * DP1Y));

        save.DOFYR = (save.REM + 1);

        save.YEAR = (((((save.YR400 * 400) + (save.YR100 * 100)) + (save.YR4 * 4)) + save.YR1) + 1);

        if (LDAYS(save.YEAR) == 0) {
            save.MONTH = LSTLTI(save.DOFYR, 12, save.DPJAN0.as_slice());
            save.DAY = (save.DOFYR - save.DPJAN0[save.MONTH]);
        } else {
            save.MONTH = LSTLTI(save.DOFYR, 12, save.DPBEGL.as_slice());
            save.DAY = (save.DOFYR - save.DPBEGL[save.MONTH]);
        }

        //
        // We only want to convert that portion of seconds less than
        // 86399 to hours, minutes and seconds.  Take anything extra
        // and put it in EXSECS.
        //
        save.EXSECS = intrinsics::DMAX1(&[0.0, ((save.SECS - save.SECSPD) + 1 as f64)]);
        save.TSECS = (save.SECS - save.EXSECS);

        RMAIND(save.TSECS, 3600.0, &mut save.HOURS, &mut save.TEMPD, ctx)?;
        RMAIND(save.TEMPD, 60.0, &mut save.MINS, &mut save.TSECS, ctx)?;

        save.TSECS = (save.TSECS + save.EXSECS);
    }

    //=====================================================================
    //
    //     Finally, we convert to the requested output.
    //
    if ((save.PTO == YMD) || (save.PTO == YMDF)) {
        TVEC[1] = (save.YEAR as f64);
        TVEC[2] = (save.MONTH as f64);
        TVEC[3] = (save.DAY as f64);
        TVEC[4] = save.HOURS;
        TVEC[5] = save.MINS;
        TVEC[6] = save.TSECS;
    } else if ((save.PTO == YD) || (save.PTO == YDF)) {
        TVEC[1] = (save.YEAR as f64);
        TVEC[2] = (save.DOFYR as f64);
        TVEC[3] = save.HOURS;
        TVEC[4] = save.MINS;
        TVEC[5] = save.TSECS;
    } else if ((save.PTO == YDD) || (save.PTO == YDDF)) {
        TVEC[1] = (save.YEAR as f64);

        if (save.PTO == YDD) {
            save.DAYPTR = LSTLEI(save.DAYNUM, save.NREF, save.DAYTAB.as_slice());
            save.DAYLEN = save.SECSPD;

            if ODD(save.DAYPTR) {
                save.DAYLEN = (save.TAITAB[(save.DAYPTR + 1)] - save.TAITAB[save.DAYPTR]);
            }

            TVEC[2] = ((save.DOFYR as f64) + (save.SECS / save.DAYLEN));
        } else {
            TVEC[2] = ((save.DOFYR as f64) + (save.SECS / save.SECSPD));
        }
    } else if (save.PTO == FRML) {
        TVEC[1] = (((((save.DAYNUM - save.DN2000) as f64) * save.SECSPD) - save.HALFD) + save.SECS);
    } else if (save.PTO == JDUTC) {
        TVEC[1] = ((save.JD1101 + (save.DAYNUM as f64)) + (save.SECS / save.SECSPD));
    } else if (save.PTO == DAYSEC) {
        TVEC[1] = (save.DAYNUM as f64);
        TVEC[2] = save.SECS;
    } else if (save.PTO == DAYP2) {
        TVEC[1] = ((save.DAYNUM - save.DN2000) as f64);
        TVEC[2] = save.SECS;
    } else if ((save.PTO == YWD) || (save.PTO == YWDF)) {
        //
        // First compute the day of the week, and the week number
        //
        RMAINI(
            (save.DAYNUM - save.SUNDAY),
            7,
            &mut save.QINT,
            &mut save.DAY,
            ctx,
        )?;
        save.WEEK = (1 + ((save.DOFYR - 1) / 7));

        //
        // Now just put everything where it belongs.
        //
        TVEC[1] = (save.YEAR as f64);
        TVEC[2] = (save.WEEK as f64);
        TVEC[3] = ((save.DAY as f64) + 1.0);
        TVEC[4] = save.HOURS;
        TVEC[5] = save.MINS;
        TVEC[6] = save.TSECS;
    } else if ((save.PTO == YMWD) || (save.PTO == YMWDF)) {
        //
        // First compute how many weeks into the month DAYNUM is,
        // and compute the day of week number.
        //
        TVEC[1] = (save.YEAR as f64);
        save.DOFFST = 0;

        if (save.YEAR <= 0) {
            RMAINI(save.YEAR, 400, &mut save.YR400, &mut save.TEMPI, ctx)?;
            save.YEAR = save.TEMPI;

            if (save.YEAR == 0) {
                save.YEAR = (save.YEAR + 400);
                save.YR400 = (save.YR400 - 1);
            }

            save.DOFFST = (DP400Y * save.YR400);
        }

        save.WEEK = (1
            + (((save.DAYNUM
                - DP0001(
                    save.YEAR,
                    save.MONTH,
                    1,
                    save.DPJAN0.as_slice(),
                    save.EXTRA.as_slice(),
                ))
                - save.DOFFST)
                / 7));
        RMAINI(
            (save.DAYNUM - save.SUNDAY),
            7,
            &mut save.QINT,
            &mut save.DAY,
            ctx,
        )?;

        //
        // Now just move the remaining stuff into TVEC.
        //
        TVEC[2] = (save.MONTH as f64);
        TVEC[3] = (save.WEEK as f64);
        TVEC[4] = ((save.DAY as f64) + 1.0);
        TVEC[5] = save.HOURS;
        TVEC[6] = save.MINS;
        TVEC[7] = save.TSECS;

    //
    // If we get to this point the type must be one of the continuous
    // time types: 'TAI', 'TDT', 'TDB', 'JED', 'ET', 'JDTDT', 'JDTDB'.
    //
    // First convert to TAI and then to the appropriate output type.
    //
    } else {
        save.DAYPTR =
            intrinsics::MAX0(&[1, LSTLEI(save.DAYNUM, save.NREF, save.DAYTAB.as_slice())]);
        save.SECS = (save.SECS + (((save.DAYNUM - save.DAYTAB[save.DAYPTR]) as f64) * save.SECSPD));
        save.TAI = (save.TAITAB[save.DAYPTR] + save.SECS);

        TVEC[1] = UNITIM(save.TAI, b"TAI", &save.MYTO, ctx)?;
    }

    //
    // Here's where we will handle conversion to SCLK when
    // we get around to implementing that portion of TTRANS
    //
    //
    // IF ( MKSCLK ) THEN
    //    CALL TVSCLK ( TO, TVEC )
    // END IF
    //
    // END IF
    //

    CHKOUT(b"TTRANS", ctx)?;
    Ok(())
}
