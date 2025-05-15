//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const WDSIZE: i32 = 16;
const NZONES: i32 = 8;

struct SaveVars {
    DEFZON: Vec<u8>,
    DEFSYS: Vec<u8>,
    DEFCAL: Vec<u8>,
    ZONES: ActualCharArray,
    TRNSLT: ActualCharArray,
    MYACTN: Vec<u8>,
    MYITEM: Vec<u8>,
    MYVAL: Vec<u8>,
    LAST: i32,
    ZONE: i32,
    HOFF: f64,
    MOFF: f64,
    SUCCES: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DEFZON = vec![b' '; WDSIZE as usize];
        let mut DEFSYS = vec![b' '; WDSIZE as usize];
        let mut DEFCAL = vec![b' '; WDSIZE as usize];
        let mut ZONES = ActualCharArray::new(WDSIZE, 1..=NZONES);
        let mut TRNSLT = ActualCharArray::new(WDSIZE, 1..=NZONES);
        let mut MYACTN = vec![b' '; WDSIZE as usize];
        let mut MYITEM = vec![b' '; WDSIZE as usize];
        let mut MYVAL = vec![b' '; WDSIZE as usize];
        let mut LAST: i32 = 0;
        let mut ZONE: i32 = 0;
        let mut HOFF: f64 = 0.0;
        let mut MOFF: f64 = 0.0;
        let mut SUCCES: bool = false;

        fstr::assign(&mut DEFSYS, b"UTC");
        fstr::assign(&mut DEFZON, b" ");
        fstr::assign(&mut DEFCAL, b"GREGORIAN");
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"EST"),
                Val::C(b"EDT"),
                Val::C(b"CST"),
                Val::C(b"CDT"),
                Val::C(b"MST"),
                Val::C(b"MDT"),
                Val::C(b"PST"),
                Val::C(b"PDT"),
            ]
            .into_iter();
            ZONES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"UTC-5"),
                Val::C(b"UTC-4"),
                Val::C(b"UTC-6"),
                Val::C(b"UTC-5"),
                Val::C(b"UTC-7"),
                Val::C(b"UTC-6"),
                Val::C(b"UTC-8"),
                Val::C(b"UTC-7"),
            ]
            .into_iter();
            TRNSLT
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            DEFZON,
            DEFSYS,
            DEFCAL,
            ZONES,
            TRNSLT,
            MYACTN,
            MYITEM,
            MYVAL,
            LAST,
            ZONE,
            HOFF,
            MOFF,
            SUCCES,
        }
    }
}

/// Time Software Defaults
///
/// Set and retrieve the defaults associated with calendar
/// input strings.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ACTION     I   Kind of action to take 'SET' or 'GET'.
///  ITEM       I   Default item of interest.
///  VALUE     I-O  Value associated with the default item.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ACTION   is a word that specifies whether TIMDEF sets the
///           value associated with ITEM or retrieves the value
///           associated with ITEM. The allowed values for
///           ACTION are 'SET' and 'GET'. The routine is not
///           sensitive to the case of the letters in ACTION.
///
///  ITEM     is the default items whose value should be set or
///           retrieved. The items that may be requested are:
///
///              ITEM        Allowed Values
///              ---------   --------------
///              CALENDAR    GREGORIAN
///                          JULIAN
///                          MIXED
///
///              SYSTEM      TDB
///                          TDT
///                          TT
///                          UTC
///
///              ZONE        EST, EDT, CST, CDT, MST, MDT, PST, PDT
///                          UTC+HR
///                          UTC-HR       ( 0 <= HR < 13 )
///                          UTC+HR:MN    ( 0 <= MN < 60 )
///                          UTC-HR:MN
///
///           The case of ITEM is not significant.
///
///  VALUE    if the action is 'SET' then VALUE is an input and
///           is the value to be associated with ITEM. Note that
///           VALUE is checked to ensure it is within the range
///           of allowed values for ITEM. If it is not within
///           the expected range and appropriate error message
///           is signaled. The case of VALUE is not significant.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VALUE    if the action is 'GET' then VALUE will be the
///           value associated with the requested ITEM. Note that
///           when time zones are set, they are translated to the
///           UTC offset form ( UTC(+/-)HR[:MN] ). When VALUE is
///           an output it will be in upper case.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the ACTION specified is not 'SET' or 'GET', the error
///      SPICE(BADACTION) is signaled.
///
///  2)  If the ITEM specified is not one the recognized items,
///      the error SPICE(BADTIMEITEM) is signaled.
///
///  3)  If the value associated with a 'SET' item input
///      is not one of the recognized items, the error
///      SPICE(BADDEFAULTVALUE) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine exists to allow SPICE toolkit users to alter
///  the default interpretation of time strings made by the
///  routine STR2ET.
///
///  Normally, unlabelled time strings are assumed to belong to
///  the Gregorian Calendar and are UTC times. However, you
///  may alter the default behavior by calling TIMDEF.
///
///  Calendar
///  --------
///
///  You may set the calendar to be one of the following
///
///  Gregorian   --- This is the calendar used daily the
///                  Western Hemisphere. Leap years occur in this
///                  calendar every 4 years except on centuries
///                  such as 1900 that are not divisible by 400.
///
///  Julian      --- This is the calendar that was in use prior
///                  to October 15, 1582. Leap years occur every
///                  4 years on the Julian Calendar (including all
///                  centuries.) October 5, 1582 on the Julian
///                  calendar corresponds to October 15, 1582 of the
///                  Gregorian Calendar.
///
///  Mixed       --- This calendar uses the Julian calendar
///                  for days prior to October 15, 1582 and
///                  the Gregorian calendar for days on or after
///                  October 15, 1582.
///
///  To set the default calendar, select on of the above for VALUE
///  and make the following call.
///
///     CALL TIMDEF ( 'SET', 'CALENDAR', VALUE )
///
///
///  System
///  -------
///
///  You may set the system used for keeping time to be UTC (default)
///  TDB (barycentric Dynamical Time), TDT (Terrestrial Dynamical
///  Time), or TT (Terrestrial Time). TDT and TT represent the same
///  time system. Both TDB and TT (TDT) have no leapseconds. As such
///  the time elapsed between any two epochs on these calendars does
///  not depend upon when leapseconds occur.
///
///  To set the default time system, select TDT, TT, TDB or UTC for
///  VALUE and make the following call.
///
///     CALL TIMDEF ( 'SET', 'SYSTEM', VALUE )
///
///  Note that such a call has the side effect of setting the value
///  associated with ZONE to a blank.
///
///  Zone
///  -----
///
///  You may alter the UTC system by specifying a time zone (UTC
///  offset). For example you may specify that epochs are referred
///  to Pacific Standard Time (PST --- UTC-7). The standard
///  abbreviations for U.S. time zones are recognized:
///
///     EST   UTC-5
///     EDT   UTC-4
///     CST   UTC-6
///     CDT   UTC-5
///     MST   UTC-7
///     MDT   UTC-6
///     PST   UTC-8
///     PDT   UTC-7
///
///  In addition you may specify any commercial time zone by using
///  "offset" notation. This notation starts with the letters 'UTC'
///  followed by a + for time zones east of Greenwich and - for time
///  zones west of Greenwich. This is followed by the number of hours
///  to add or subtract from UTC. This is optionally followed by a
///  colon ':' and the number of minutes to add or subtract (based on
///  the sign that follows 'UTC') to get the local time zone. Thus to
///  specify the time zone of Calcutta you would specify the time zone
///  to be UTC+5:30. To specify the time zone of Newfoundland use the
///  time zone UTC-3:30.
///
///  To set a default time zone, select one of the "built-in" U.S.
///  zones or construct an offset as discussed above. Then make the
///  call
///
///     CALL TIMDEF ( 'SET', 'ZONE', VALUE )
///
///  If you 'GET' a 'ZONE' it will either be blank, or have the
///  form 'UTC+/-HR[:MN]'
///
///  Note that such a call has the side effect of setting the value
///  associated with SYSTEM to a blank.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you wish to modify the behavior of STR2ET so that
///  it interprets unlabeled time strings as being times in
///  Pacific Daylight Time and that you want the calendar to use
///  to be the "Mixed" calendar. The following two calls will
///  make the desired changes to the behavior of STR2ET
///
///      CALL TIMDEF ( 'SET', 'CALENDAR', 'MIXED' )
///      CALL TIMDEF ( 'SET', 'ZONE',     'PDT'   )
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 14-OCT-2021 (EDW) (JDR)
///
///         UCASE and LJUST called on VALUE only in 'SET' block.
///
///         Add time system name 'TT' (Terrestrial Time) as alternate
///         assignment of 'TDT' (Terrestrial Dynamical Time).
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 26-MAY-1998 (WLT)
///
///         The previous version did not check out and return
///         when an error was detected in the if block that
///         begins with
///
///            ELSE IF ( MYITEM .EQ. 'ZONE' ) THEN
///
///         The routine did eventually check out and return so
///         that the trace stack was maintained correctly, but
///         the default time zone would be modified which was not
///         the desired behavior.
///
/// -    SPICELIB Version 1.1.0, 27-JUN-1997 (WLT)
///
///         The previous version failed to check out when
///         the default value was set.
///
/// -    SPICELIB Version 1.0.0, 13-NOV-1996 (WLT)
/// ```
pub fn timdef(
    ctx: &mut SpiceContext,
    action: &str,
    item: &str,
    value: &mut str,
) -> crate::Result<()> {
    TIMDEF(
        action.as_bytes(),
        item.as_bytes(),
        fstr::StrBytes::new(value).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TIMDEF ( Time Software Defaults )
pub fn TIMDEF(
    ACTION: &[u8],
    ITEM: &[u8],
    VALUE: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables.
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"TIMDEF", ctx)?;

    //
    // Normalize the input.
    //
    LJUST(ACTION, &mut save.MYACTN);
    UCASE(&save.MYACTN.to_vec(), &mut save.MYACTN, ctx);

    LJUST(ITEM, &mut save.MYITEM);
    UCASE(&save.MYITEM.to_vec(), &mut save.MYITEM, ctx);

    //
    // Admittedly, the decision making below is not very elegant.
    // However, this works and is simpler than anything that comes
    // to mind at the moment and allows us to give quite specific
    // diagnostic messages easily.
    //
    if fstr::eq(&save.MYACTN, b"SET") {
        LJUST(VALUE, &mut save.MYVAL);
        UCASE(&save.MYVAL.to_vec(), &mut save.MYVAL, ctx);

        if fstr::eq(&save.MYITEM, b"SYSTEM") {
            //
            // Recognize only the know time systems. Note, 'UTC' not
            // actually a time system, more a time representation.
            //
            if (((fstr::eq(&save.MYVAL, b"TDB") || fstr::eq(&save.MYVAL, b"TDT"))
                || fstr::eq(&save.MYVAL, b"TT"))
                || fstr::eq(&save.MYVAL, b"UTC"))
            {
                fstr::assign(&mut save.DEFZON, b" ");
                fstr::assign(&mut save.DEFSYS, &save.MYVAL);
            } else {
                SETMSG(b"The default value assigned to the time system must be one of \'UTC\', \'TDT\', \'TT\', or \'TDB\'. The value supplied was \'#\'. ", ctx);

                ERRCH(b"#", VALUE, ctx);
                SIGERR(b"SPICE(BADDEFAULTVALUE)", ctx)?;
                CHKOUT(b"TIMDEF", ctx)?;
                return Ok(());
            }
        } else if fstr::eq(&save.MYITEM, b"ZONE") {
            save.ZONE = ISRCHC(&save.MYVAL, NZONES, save.ZONES.as_arg());

            //
            // If MYVAL was one of the recognized time zones, we
            // translate it to the UTC offset form.
            //
            if (save.ZONE > 0) {
                fstr::assign(&mut save.MYVAL, save.TRNSLT.get(save.ZONE));
            }

            PREFIX(b"::", 0, &mut save.MYVAL);
            ZZUTCPM(
                &save.MYVAL,
                1,
                &mut save.HOFF,
                &mut save.MOFF,
                &mut save.LAST,
                &mut save.SUCCES,
                ctx,
            );

            if !save.SUCCES {
                SETMSG(b"The input value for a time zone \"#\" was not recognized as known time zone and could not be parsed according to the pattern UTC(+/-)HR[:MN]. Known time zones are: \'EST\', \'EDT\', \'CST\', \'CDT\', \'MST\', \'MDT\', \'PST\', and \'PDT\'. ", ctx);

                ERRCH(b"#", VALUE, ctx);
                SIGERR(b"SPICE(BADDEFAULTVALUE)", ctx)?;
                CHKOUT(b"TIMDEF", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut save.DEFZON, fstr::substr(&save.MYVAL, 3..));
            fstr::assign(&mut save.DEFSYS, b" ");
        } else if fstr::eq(&save.MYITEM, b"CALENDAR") {
            if ((fstr::eq(&save.MYVAL, b"JULIAN") || fstr::eq(&save.MYVAL, b"GREGORIAN"))
                || fstr::eq(&save.MYVAL, b"MIXED"))
            {
                fstr::assign(&mut save.DEFCAL, &save.MYVAL);
            } else {
                SETMSG(b"The input value for \'#\' is not a recognized calendar type.  The recognized calendars are \'GREGORIAN\', \'JULIAN\', and \'MIXED\'. ", ctx);
                ERRCH(b"#", VALUE, ctx);
                SIGERR(b"SPICE(BADDEFAULTVALUE)", ctx)?;
                CHKOUT(b"TIMDEF", ctx)?;
                return Ok(());
            }
        } else {
            SETMSG(b"The specified item \'#\' is not a recognized time default item.  The items that you may \"SET\" via the routine TIMDEF are \'CALENDAR\', \'SYSTEM\', or \'ZONE\' ", ctx);

            ERRCH(b"#", ITEM, ctx);
            SIGERR(b"SPICE(BADTIMEITEM)", ctx)?;
            CHKOUT(b"TIMDEF", ctx)?;
            return Ok(());
        }

        CHKOUT(b"TIMDEF", ctx)?;
        return Ok(());
    } else if fstr::eq(&save.MYACTN, b"GET") {
        if fstr::eq(&save.MYITEM, b"CALENDAR") {
            fstr::assign(VALUE, &save.DEFCAL);
        } else if fstr::eq(&save.MYITEM, b"SYSTEM") {
            fstr::assign(VALUE, &save.DEFSYS);
        } else if fstr::eq(&save.MYITEM, b"ZONE") {
            fstr::assign(VALUE, &save.DEFZON);
        } else {
            SETMSG(b"The specified item \'#\' is not a recognized time default item.  The items that you may \"SET\" via the routine TIMDEF are \'CALENDAR\', \'SYSTEM\', or \'ZONE\' ", ctx);
            ERRCH(b"#", ITEM, ctx);
            SIGERR(b"SPICE(BADTIMEITEM)", ctx)?;
            CHKOUT(b"TIMDEF", ctx)?;
            return Ok(());
        }
    } else {
        SETMSG(b"The action specified to TIMDEF was \'#\'.  This is not a recognized action. The recognized actions are \'SET\' and \'GET\'. ", ctx);

        ERRCH(b"#", ACTION, ctx);
        SIGERR(b"SPICE(BADACTION)", ctx)?;
        CHKOUT(b"TIMDEF", ctx)?;
        return Ok(());
    }

    CHKOUT(b"TIMDEF", ctx)?;
    Ok(())
}
