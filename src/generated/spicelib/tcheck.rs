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
const NMODS: i32 = SYSTEM;
const LNSIZE: i32 = 200;

struct SaveVars {
    MESSGE: Vec<u8>,
    MNAMES: ActualCharArray,
    CNAME: ActualCharArray,
    DINMON: StackArray<f64, 12>,
    DOY: f64,
    DINYR: f64,
    JUN30: f64,
    HUBND: f64,
    HLBND: f64,
    COMP: i32,
    DAY: i32,
    HOUR: i32,
    I: i32,
    K: i32,
    LEAPDY: i32,
    MINUTE: i32,
    MONTH: i32,
    MYEAR: i32,
    SECOND: i32,
    YEAR: i32,
    DOCHCK: bool,
    MODTRU: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MESSGE = vec![b' '; LNSIZE as usize];
        let mut MNAMES = ActualCharArray::new(10, 1..=12);
        let mut CNAME = ActualCharArray::new(7, 1..=4);
        let mut DINMON = StackArray::<f64, 12>::new(1..=12);
        let mut DOY: f64 = 0.0;
        let mut DINYR: f64 = 0.0;
        let mut JUN30: f64 = 0.0;
        let mut HUBND: f64 = 0.0;
        let mut HLBND: f64 = 0.0;
        let mut COMP: i32 = 0;
        let mut DAY: i32 = 0;
        let mut HOUR: i32 = 0;
        let mut I: i32 = 0;
        let mut K: i32 = 0;
        let mut LEAPDY: i32 = 0;
        let mut MINUTE: i32 = 0;
        let mut MONTH: i32 = 0;
        let mut MYEAR: i32 = 0;
        let mut SECOND: i32 = 0;
        let mut YEAR: i32 = 0;
        let mut DOCHCK: bool = false;
        let mut MODTRU: bool = false;

        DOCHCK = false;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(31.0),
                Val::D(28.0),
                Val::D(31.0),
                Val::D(30.0),
                Val::D(31.0),
                Val::D(30.0),
                Val::D(31.0),
                Val::D(31.0),
                Val::D(30.0),
                Val::D(31.0),
                Val::D(30.0),
                Val::D(31.0),
            ]
            .into_iter();
            DINMON
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"January"),
                Val::C(b"February"),
                Val::C(b"March"),
                Val::C(b"April"),
                Val::C(b"May"),
                Val::C(b"June"),
                Val::C(b"July"),
                Val::C(b"August"),
                Val::C(b"September"),
                Val::C(b"October"),
                Val::C(b"November"),
                Val::C(b"December"),
            ]
            .into_iter();
            MNAMES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"days"),
                Val::C(b"hours"),
                Val::C(b"minutes"),
                Val::C(b"seconds"),
            ]
            .into_iter();
            CNAME
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            MESSGE,
            MNAMES,
            CNAME,
            DINMON,
            DOY,
            DINYR,
            JUN30,
            HUBND,
            HLBND,
            COMP,
            DAY,
            HOUR,
            I,
            K,
            LEAPDY,
            MINUTE,
            MONTH,
            MYEAR,
            SECOND,
            YEAR,
            DOCHCK,
            MODTRU,
        }
    }
}

fn DIVBLE(YEAR: i32, I: i32) -> i32 {
    intrinsics::MAX0(&[0, ((1 + ((i32::abs(YEAR) / I) * I)) - i32::abs(YEAR))])
}

/// Time Check
///
/// Determine whether the components of a time vector are in the
/// "usual" range for the components, if component checking is
/// enabled.
///
/// If component checking is not enabled, this routine simply
/// returns after setting the outputs.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TVEC       I   A vector of time components
///  TYPE       I   The type of time vector.
///  MODS       I   A logical indicating the presence of modifiers
///  MODIFY     I   The values of the modifiers
///  OK         O   Indicates success or failure of component checks.
///  ERROR      O   Diagnostic message if .NOT. OK.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TVEC     is an array of double precision numbers that
///           represent the components of some calendar epoch.
///
///  TYPE     is kind of calendar epoch represented by TVEC
///           legitimate values are 'YMD' and 'YD'
///
///  MODS     is a logical flag indicating whether any of the
///           items in MODIFY are non-blank. If some item
///           in MODIFY is non-blank, MODS will be .TRUE. If
///           all items in MODIFY are blank, MODS will be .FALSE.
///
///  MODIFY   is an array of strings indicating how the
///           interpretation of the various components of TVEC
///           should be modified. Blank values indicate that
///           the default interpretation should be applied.
///           Non-blank components will have the following values
///           and meanings.
///
///
///            Component   Meaning   Possible Non-blank Modifier Values
///            ---------   -------   ----------------------------------
///            1           ERA       'A.D.', 'B.C.'
///            2           Weekday   'SUN',  'MON', ... etc.
///            3           AM/PM     'A.M.', 'P.M.'
///            4           System    'UTC',  'TDB', 'TDT'
///            5           Time Zone 'UTC+i:i', 'UTC-i:i'
/// ```
///
/// # Detailed Output
///
/// ```text
///  OK       is returned .TRUE. if all components of TVEC are within
///           the normal range of values. If some problem arises,
///           OK will be returned with the value .FALSE. Note that
///           component checking has not been enabled by a call
///           to TPARCH, the value of OK is automatically set to
///           .TRUE.
///
///  ERROR    if OK is returned with the value .TRUE., ERROR will be
///           returned as a blank. However, if OK is .FALSE., ERROR
///           will contain a diagnostic indicating what was wrong
///           with the components of TVEC. Note that
///           component checking has not been enabled by a call
///           to TPARCH, the value of ERROR is automatically set to
///           a blank.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  All problems with TVEC are reported via the logical OK
///      and the message ERROR.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine works in conjunction with the entry point TPARCH.
///  If TPARCH has not been called with the input value 'YES' this
///  routine simply sets the outputs as indicated above and returns.
///
///  Usually strings such as February 32, 1997 are regarded as
///  erroneous. However, the SPICE time subsystem is capable
///  of attaching meaning to such strings. The routines TPARCH and
///  TCHECK allow you to treat such strings as erroneous throughout
///  the SPICE time sub-system.
///
///  This routine examines the components of a time vector and
///  determines whether or not all of the values in the vector
///  are within the normal bounds.
///
///  To pass inspection:
///
///     Years must be integers.
///
///     Months must be in the range from 1 to 12 and must be integers.
///
///     Days of the month must be in the normal ranges. For example
///          if the month specified is January, the day of the month
///          must be greater than or equal to 1.0D0 and strictly less
///          than 32.0D0 (The normal range for February is a function
///          of whether the year specified is a leap year. The
///          Gregorian calendar is used to determine leap years.)
///
///     Day of the year must be greater than or equal to 1.0D0
///          and strictly less than 366.0D0  (367.0D0 in a leap year.
///          The Gregorian calendar is used to determine leap years.)
///
///     Hours must be greater than or equal to 0.0D0 and strictly
///          less than 24.0D0. If the AMPM modifier is included
///          hours must be greater than or equal to 1.0D0 and strictly
///          less than 13.0D0.
///
///     Minutes must be greater than or equal to 0.0D0 and must
///          be strictly less than 60.0D0
///
///     Seconds must be greater than or equal to 0.0D0 and strictly
///          less than 60.0D0 (61.0D0 during the last minute of the
///          30th of June and the 31st of December).
///
///     If some component other than the seconds component is
///     not an integer, all components of lesser significance must
///     be zero.
///
///  This routine  is designed to work in conjunction
///  with the SPICE routine TPARTV and it is anticipated that
///  it will be called in the following fashion
///
///     CALL TPARTV ( STRING, TVEC, NTVEC,  TYPE,
///    .              MODIFY, MODS, YABBRV, SUCCES, ERROR )
///
///     IF ( .NOT. SUCCES ) THEN
///
///        communicate the diagnostic message and
///        take other actions as appropriate
///
///        RETURN
///
///     END IF
///
///     IF ( SUCCES .AND. CHECK ) THEN
///         CALL TCHECK ( TVEC, TYPE, MODS, MODIFY, OK, ERROR )
///     END IF
///
///     IF ( .NOT. OK ) THEN
///
///        communicate the diagnostic message and
///        take other actions as appropriate
///
///        RETURN
///
///     END IF
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have parsed a string (via TPARTV) and want
///  to enforce normal ranges of the components. The following
///  sequence of calls will perform the checks on components.
///
///     get the current checking setting
///
///     CALL TCHCKD ( CURNT )
///
///     turn on component checking.
///
///     CALL TPARCH ( 'YES' )
///
///     Check the components.
///
///     CALL TCHECK ( TVEC, TYPE, MODS, MODIFY, OK, ERROR )
///
///     Reset the checking setting to the original value.
///
///     CALL TPARCH ( CURNT )
///
///
///     Now handle any problems that were diagnosed by TCHECK
///
///     IF ( .NOT. OK ) THEN
///
///        do something
///
///     END IF
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
/// -    SPICELIB Version 1.2.0, 01-NOV-2021 (JDR) (EDW)
///
///         Added logic to prevent evaluation of MODIFY when MODS false.
///
///         Added text listing routines affected and not affected by
///         explicit assignments to TPARCH.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.0, 31-JAN-2017 (NJB)
///
///         Bug fix: updated logic so that B.C. leap years are recognized.
///
/// -    SPICELIB Version 1.0.1, 10-FEB-2014 (BVS)
///
///         Fixed typo in the $Declarations section in the TPARCH header:
///         STRING -> TYPE.
///
/// -    SPICELIB Version 1.0.0, 26-JUL-1996 (WLT)
/// ```
pub fn tcheck(
    ctx: &mut SpiceContext,
    tvec: &[f64],
    type_: &str,
    mods: bool,
    modify: CharArray,
    ok: &mut bool,
    error: &mut str,
) {
    TCHECK(
        tvec,
        type_.as_bytes(),
        mods,
        modify,
        ok,
        fstr::StrBytes::new(error).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure TCHECK ( Time Check )
pub fn TCHECK(
    TVEC: &[f64],
    TYPE: &[u8],
    MODS: bool,
    MODIFY: CharArray,
    OK: &mut bool,
    ERROR: &mut [u8],
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let TVEC = DummyArray::new(TVEC, 1..);
    let MODIFY = DummyCharArray::new(MODIFY, None, 1..);

    //
    // SPICELIB functions
    //

    //
    // In-line Functions
    //

    //
    // Local Variables
    //

    //
    // The in-line function DIVBLE returns 1 if YEAR is divisible
    // by I,  it returns 0 otherwise.
    //
    //
    // If checking isn't enabled, there is nothing to do.
    //
    if !save.DOCHCK {
        *OK = true;
        fstr::assign(ERROR, b" ");
        return;
    }
    //
    // Ok.  Checking has been enabled.  Proceed with the various
    // checks.
    //
    save.YEAR = intrinsics::IDNINT(TVEC[1]);

    save.MODTRU = false;
    if MODS {
        save.MODTRU = fstr::eq(MODIFY.get(ERA), b"B.C.");
    }

    if save.MODTRU {
        save.MYEAR = (1 - save.YEAR);
    } else {
        save.MYEAR = save.YEAR;
    }

    save.LEAPDY = ((DIVBLE(save.MYEAR, 4) - DIVBLE(save.MYEAR, 100)) + DIVBLE(save.MYEAR, 400));

    save.DINMON[2] = (28.0 + (save.LEAPDY as f64));
    save.DINYR = (365.0 + (save.LEAPDY as f64));
    save.JUN30 = (181.0 + (save.LEAPDY as f64));
    //
    // The error message that will be attached to an out of range
    // problem for hours depends upon whether the AMPM modifier
    // was specified.  We set up valid range as well as the out
    // of range messages here.
    //
    save.MODTRU = false;
    if MODS {
        save.MODTRU = fstr::ne(MODIFY.get(AMPM), b" ");
    }

    if save.MODTRU {
        save.HUBND = 13.0;
        save.HLBND = 1.0;
        fstr::assign(&mut save.MESSGE, b"The hours component of the time specified was #. When either A.M. or P.M. is specified with the time the hours component must be at least 1.0D0 and less than 13.0D0. ");
    } else {
        save.HUBND = 24.0;
        save.HLBND = 0.0;
        fstr::assign(&mut save.MESSGE, b"The hours component of the time specified was #.  The hours component must be greater than or equal to 0.0D0 and less than 24.0D0. ");
    }
    //
    // We only check YD and YMD anything else is out of the
    // province of this routine.
    //
    if (fstr::ne(TYPE, b"YD") && fstr::ne(TYPE, b"YMD")) {
        *OK = false;
        fstr::assign(ERROR, b"The type of the time vector specified was #, only \'YD\' and \'YMD\' are recognized. ");
        REPMC(&ERROR.to_vec(), b"#", TYPE, ERROR);
        return;
    }

    //
    // First check.  The year must be an integer.
    //
    if (TVEC[1] != (save.YEAR as f64)) {
        *OK = false;
        fstr::assign(
            ERROR,
            b"The year value was #.  This must be an integral value. ",
        );
        REPMD(&ERROR.to_vec(), b"#", TVEC[1], 8, ERROR, ctx);

        return;
    }

    if fstr::eq(TYPE, b"YD") {
        save.DAY = 2;
        save.HOUR = 3;
        save.MINUTE = 4;
        save.SECOND = 5;
        save.DOY = TVEC[2];

        if ((TVEC[2] >= (save.DINYR + 1.0)) || (TVEC[2] < 1.0)) {
            *OK = false;
            fstr::assign(ERROR, b"Day # has been specified for the year #. The correct range for the day of year for this year is from 1 to #. ");

            REPMD(&ERROR.to_vec(), b"#", TVEC[2], 8, ERROR, ctx);
            REPMI(&ERROR.to_vec(), b"#", save.YEAR, ERROR, ctx);
            REPMI(&ERROR.to_vec(), b"#", (365 + save.LEAPDY), ERROR, ctx);
            return;
        }
    } else if fstr::eq(TYPE, b"YMD") {
        save.MONTH = intrinsics::IDNINT(TVEC[2]);
        save.DAY = 3;
        save.HOUR = 4;
        save.MINUTE = 5;
        save.SECOND = 6;
        save.DOY = 0.0;

        if (TVEC[2] != (save.MONTH as f64)) {
            *OK = false;
            fstr::assign(ERROR, b"The month specified, #, was not an integer. The month must be an integer in the range from 1 to 12. ");
            REPMD(&ERROR.to_vec(), b"#", TVEC[2], 3, ERROR, ctx);
            return;
        } else if ((TVEC[2] < 1.0) || (TVEC[2] > 12.0)) {
            *OK = false;
            fstr::assign(ERROR, b"The month specified was #.  The month must be an integer in the range from 1 to 12 (inclusive). ");
            REPMI(&ERROR.to_vec(), b"#", save.MONTH, ERROR, ctx);
            return;
        } else if ((TVEC[3] < 1.0) || (TVEC[3] >= (save.DINMON[save.MONTH] + 1.0))) {
            *OK = false;
            fstr::assign(ERROR, b"The day of the month specified for the month of # was #.  For # the day must be at least 1.0D0 and less than #. ");
            REPMC(&ERROR.to_vec(), b"#", &save.MNAMES[save.MONTH], ERROR);
            REPMD(&ERROR.to_vec(), b"#", TVEC[3], 3, ERROR, ctx);
            REPMC(&ERROR.to_vec(), b"#", &save.MNAMES[save.MONTH], ERROR);
            REPMD(
                &ERROR.to_vec(),
                b"#",
                (save.DINMON[save.MONTH] + 1.0),
                2,
                ERROR,
                ctx,
            );
            return;
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = (save.MONTH - 1);
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.DOY = (save.DOY + save.DINMON[save.I]);
                save.I += m3__;
            }
        }

        save.DOY = (save.DOY + TVEC[3]);
    }

    //
    // Make sure the hours, minutes and seconds are all in range.
    //
    if ((TVEC[save.HOUR] >= save.HUBND) || (TVEC[save.HOUR] < save.HLBND)) {
        *OK = false;
        fstr::assign(ERROR, &save.MESSGE);
        REPMD(&ERROR.to_vec(), b"#", TVEC[save.HOUR], 2, ERROR, ctx);
        return;
    } else if ((TVEC[save.MINUTE] >= 60.0) || (TVEC[save.MINUTE] < 0.0)) {
        *OK = false;
        fstr::assign(ERROR, b"The minutes component of the time specified was #. This value must be greater than or equal to 0.0 and less than 60.0. ");
        REPMD(&ERROR.to_vec(), b"#", TVEC[save.MINUTE], 2, ERROR, ctx);
        return;
    }

    if ((TVEC[save.SECOND] >= 60.0) || (TVEC[save.SECOND] < 0.0)) {
        //
        // We allow for the possibility that we might have a leapsecond.
        //
        save.MODTRU = false;
        if MODS {
            save.MODTRU = fstr::eq(MODIFY.get(AMPM), b"P.M.");
        }

        if (((((TVEC[save.SECOND] < 61.0) && (TVEC[save.SECOND] > 0.0))
            && (TVEC[save.MINUTE] == 59.0))
            && (TVEC[save.HOUR] == 23.0))
            && ((save.DOY == save.DINYR) || (save.DOY == save.JUN30)))
        {
            //
            // Don't do anything.
            //
        } else if ((((((TVEC[save.SECOND] < 61.0) && (TVEC[save.SECOND] > 0.0))
            && (TVEC[save.MINUTE] == 59.0))
            && (TVEC[save.HOUR] == 11.0))
            && save.MODTRU)
            && ((save.DOY == save.DINYR) || (save.DOY == save.JUN30)))
        {
            //
            // Don't do anything.
            //
        } else {
            *OK = false;
            fstr::assign(ERROR, b"The seconds component of time must be at least 0.0D0 and less than 60.0D0 (61.0D0 during the last minute of June 30 and December 31). The value supplied was #. ");

            REPMD(&ERROR.to_vec(), b"#", TVEC[save.SECOND], 8, ERROR, ctx);
            return;
        }
    }

    //
    // One final check.  If some component is not an integer
    // the remaining components must be zero.
    //
    save.COMP = 0;

    {
        let m1__: i32 = save.DAY;
        let m2__: i32 = save.MINUTE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.COMP = (save.COMP + 1);
            save.K = save.COMP;

            if (TVEC[save.I] != intrinsics::IDNINT(TVEC[save.I]) as f64) {
                for J in (save.I + 1)..=save.SECOND {
                    save.K = (save.K + 1);

                    if (TVEC[J] != 0.0) {
                        *OK = false;
                        fstr::assign(ERROR, b"The \'#\' component of the date has a fractional component.  This is allowed only if all components of lesser significance have value 0.0D0. However the \'#\' component has value #. ");

                        REPMC(&ERROR.to_vec(), b"#", &save.CNAME[save.COMP], ERROR);
                        REPMC(&ERROR.to_vec(), b"#", &save.CNAME[save.K], ERROR);
                        REPMD(&ERROR.to_vec(), b"#", TVEC[J], 2, ERROR, ctx);
                        return;
                    }
                }
            }

            save.I += m3__;
        }
    }

    //
    // If we make it this far, all components pass the reasonableness
    // tests.
    //
    *OK = true;
    fstr::assign(ERROR, b" ");
}

/// Parse check---check format of strings
///
/// Restrict the set of strings that are recognized by SPICE time
/// parsing routines to those that have standard values for all time
/// components.
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
///  TYPE       I   String: Use 'YES' to restrict time inputs.
/// ```
///
/// # Detailed Input
///
/// ```text
///  TYPE     is a character string that is used to adjust the set of
///           strings that will be regarded as valid time strings by
///           SPICE time parsing routines.
///
///           The default behavior of SPICE time software is to allow
///           an extended range of values for the various components
///           (tokens) of a time string. For example, using its default
///           behavior, TPARSE would regard 1993 JAN 367 as a valid
///           time string and return the UTC seconds past the J2000
///           epoch value that corresponds to Jan 2, 1994.
///
///           While this is a "reasonable" interpretation of such a
///           string, there may be occasions when such a string should
///           be regarded as an error.
///
///           By calling TPARCH with a value of 'YES', the action of
///           the time software will be modified. Strings that have
///           components that are out of the range of values used in
///           most English discourse will be regarded as errors. Thus
///           the numeric values of MONTH, DAY, HOUR, MINUTE, and
///           SECOND must satisfy the following conditions to be
///           regarded as legitimate calendar time strings.
///
///              ITEM     Valid Range
///              ------   -----------------------------------------
///              MONTH    1 to 13
///              DAY      1 to 365 (366 for leap years) when
///                       DAY is interpreted as the day of year
///                       i.e. the month token is empty.
///                       1 to 31  if month is January
///                       1 to 28  (29 in leap years) if month is
///                                February
///                       1 to 31  if month is March
///                       1 to 30  if month is April
///                       1 to 31  if month is May
///                       1 to 31  if month is June
///                       1 to 30  if month is July
///                       1 to 31  if month is August
///                       1 to 30  if month is September
///                       1 to 31  if month is October
///                       1 to 30  if month is November
///                       1 to 31  if month is December
///              HOUR     0 to 23
///              MINUTE   0 to 59
///              SECOND   0 up to but not including 60 on days that
///                       can not have a leapsecond.
///                       0 up to but not including 61 for times
///                       that are the last second of June or
///                       December. In other words,
///                            JUN 30, 23:59:60.xxxxxx...x
///                       and  DEC 31, 23:59:60.xxxxxx...x
///
///           To reset the action of time software to the default
///           action, set TYPE to a value that is not equivalent to
///           'YES' when case and spaces are ignored.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is used to alter the collections of strings
///  that SPICE software regards as legitimate calendar strings. The
///  default behavior of SPICE software is to accept strings such
///  as FEB 34, 1993 and to interpret these in a "natural way"
///  (FEB 34, 1993 is regarded as MARCH 6, 1993.) This behavior
///  is sometimes useful for "private" programs that you write.
///  However, such a string may be a typo (a finger accidentally hit
///  two keys for the day instead of one). Given that this string
///  does not appear in common usage, you may want to consider
///  that it is more likely the result of erroneous input. You
///  can alter the behavior of SPICE software so that it will
///  treat such a string as an error. To do this call this entry
///  point with TYPE having the value 'YES'.
///
///     CALL TPARCH ( 'YES' )
///
///  Until the behavior is reset by calling TPARCH with a value
///  other than 'YES' (such as 'NO'), SPICE software will treat all
///  out-of-bound components of time strings as errors.
///
///  If you are happy with the SPICE default interpretation of
///  strings, you do not need to make any calls to TPARCH.
///
///  All time parsing routines --including the top-level APIs TPARSE
///  and UTC2ET-- respect the setting assigned by TPARCH, except the
///  SPICELIB routine STR2ET.
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
///  1) When accepting times as input interactively, you usually
///     read a string typed at a keyboard and then pass that string
///     to the SPICE time system to convert it to an ephemeris time.
///     The default behavior of SPICE software is to accept strings
///     such as FEB 34, 1993 and to interpret these in a "natural way"
///     (FEB 34, 1993 is regarded as MARCH 6, 1993.) The following
///     example code demonstrates how to modify this behavior.
///
///
///     Example code begins here.
///
///
///           PROGRAM TPARCH_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         TIMSTR
///           PARAMETER           ( TIMSTR = 'FEB 34, 1993' )
///
///           INTEGER               ERRMLN
///           PARAMETER           ( ERRMLN = 1000 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(ERRMLN)    ERRMSG
///
///           DOUBLE PRECISION      SP2000
///
///     C
///     C     First, demonstrate the default behavior of SPICE.
///     C     Let's get the number of UTC seconds past J2000 epoch.
///     C
///           CALL TPARSE ( TIMSTR, SP2000, ERRMSG )
///
///           IF ( ERRMSG .EQ. ' ' ) THEN
///
///              WRITE(*,'(A,F18.6)') 'UTC (s): ', SP2000
///
///           ELSE
///
///              WRITE(*,'(2A)') 'Error  : ', ERRMSG
///
///           END IF
///
///     C
///     C     Now, turn error checking on and parse the time string
///     C     again.
///     C
///           CALL TPARCH ( 'YES' )
///           CALL TPARSE ( TIMSTR, SP2000, ERRMSG )
///
///           IF ( ERRMSG .EQ. ' ' ) THEN
///
///              WRITE(*,'(A,F18.6)') 'UTC (s): ', SP2000
///
///           ELSE
///
///              WRITE(*,'(2A)') 'Error  : ', ERRMSG
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     UTC (s):  -215352000.000000
///     Error  : The day of the month specified for the month of Feb***
///
///
///     Warning: incomplete output. 1 line extended past the right
///     margin of the header and has been truncated. This line is
///     marked by "***" at the end of the line.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 01-NOV-2021 (JDR) (EDW)
///
///         Added text listing routines affected and not affected by
///         explicit assignments to TPARCH.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Added TIME to $Required_Reading list.
///
/// -    SPICELIB Version 1.0.1, 10-FEB-2014 (BVS)
///
///         Fixed typo in the $Declarations section: STRING -> TYPE.
///
/// -    SPICELIB Version 1.0.0, 07-APR-1996 (WLT)
///
///         The entry point TPARCH was moved from TPARSE to the routine
///         TCHECK so that all time parsing actions could be centralized.
/// ```
pub fn tparch(ctx: &mut SpiceContext, type_: &str) {
    TPARCH(type_.as_bytes(), ctx.raw_context());
}

//$Procedure TPARCH ( Parse check---check format of strings )
pub fn TPARCH(TYPE: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.DOCHCK = EQSTR(TYPE, b"YES");
}

/// Time components are checked
///
/// Determine whether component checking is enabled for time strings.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  TYPE       O   Answer to the question: "Is checking enabled?"
/// ```
///
/// # Detailed Output
///
/// ```text
///  TYPE     is a string that gives the answer to the question
///           "Is checking of components enabled?"  If checking
///           is enabled, the value returned will be "YES" if
///           checking is not enabled, the value returned will
///           be "NO".
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point allows you to "fetch" the current settings
///  regarding the checking of components of a time string. This
///  allows you to temporarily set the action to whatever is desired
///  in a particular piece of code and then reset the action to
///  the setting in effect prior to the routines activities.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you'd like to write a routine that always applies
///  component checking to the components of a time string.
///
///  Use this entry point together with TPARCH and TCHECK to
///  make use of the built-in SPICE capabilities
///
///     get the current setting.
///
///     CALL TCHCKD ( CURNT )
///     CALL TPARCH ( 'YES' )
///
///        perform some time
///        parsing activities.
///
///        check the components.
///
///     CALL TCHECK ( TVEC, TYPE, MODS, MODIFY, OK, ERROR )
///
///     Set the checking activity back to the value prior
///     to the work done here.
///
///     CALL TPARCH ( CURNT )
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
/// -    SPICELIB Version 1.0.1, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 07-APR-1996 (WLT)
/// ```
pub fn tchckd(ctx: &mut SpiceContext, type_: &mut str) {
    TCHCKD(fstr::StrBytes::new(type_).as_mut(), ctx.raw_context());
}

//$Procedure TCHCKD ( Time components are checked )
pub fn TCHCKD(TYPE: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.DOCHCK {
        fstr::assign(TYPE, b"YES");
    } else {
        fstr::assign(TYPE, b"NO");
    }
}
