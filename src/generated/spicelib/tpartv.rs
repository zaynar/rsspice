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
const WDSIZE: i32 = 12;
const ROOM: i32 = 300;
const ZSIZE: i32 = 3;
const OSIZE: i32 = 6;
const NZONES: i32 = 8;

struct SaveVars {
    BEGS: StackArray<i32, 5>,
    ENDS: StackArray<i32, 5>,
    HAVERA: bool,
    HAVWDY: bool,
    HAVZON: bool,
    HAVAPM: bool,
    HAVSYS: bool,
    NKNOWN: i32,
    KNOWN: ActualCharArray,
    MEANNG: ActualCharArray,
    REP: Vec<u8>,
    ZONES: ActualCharArray,
    OFFSET: ActualCharArray,
    DELIM: ActualCharArray,
    B: i32,
    B1: i32,
    B2: i32,
    E: i32,
    E1: i32,
    E2: i32,
    FROM: i32,
    MAPTO: i32,
    TO: i32,
    USE: i32,
    R: i32,
    RESOLV: bool,
    L2R: bool,
    R2L: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BEGS = StackArray::<i32, 5>::new(1..=NMODS);
        let mut ENDS = StackArray::<i32, 5>::new(1..=NMODS);
        let mut HAVERA: bool = false;
        let mut HAVWDY: bool = false;
        let mut HAVZON: bool = false;
        let mut HAVAPM: bool = false;
        let mut HAVSYS: bool = false;
        let mut NKNOWN: i32 = 0;
        let mut KNOWN = ActualCharArray::new(WDSIZE, 1..=ROOM);
        let mut MEANNG = ActualCharArray::new(WDSIZE, 1..=ROOM);
        let mut REP = vec![b' '; WDSIZE as usize];
        let mut ZONES = ActualCharArray::new(ZSIZE, 1..=NZONES);
        let mut OFFSET = ActualCharArray::new(OSIZE, 1..=NZONES);
        let mut DELIM = ActualCharArray::new(1, 1..=3);
        let mut B: i32 = 0;
        let mut B1: i32 = 0;
        let mut B2: i32 = 0;
        let mut E: i32 = 0;
        let mut E1: i32 = 0;
        let mut E2: i32 = 0;
        let mut FROM: i32 = 0;
        let mut MAPTO: i32 = 0;
        let mut TO: i32 = 0;
        let mut USE: i32 = 0;
        let mut R: i32 = 0;
        let mut RESOLV: bool = false;
        let mut L2R: bool = false;
        let mut R2L: bool = false;
        let mut FIRST: bool = false;

        FIRST = true;
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
            OFFSET
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BEGS,
            ENDS,
            HAVERA,
            HAVWDY,
            HAVZON,
            HAVAPM,
            HAVSYS,
            NKNOWN,
            KNOWN,
            MEANNG,
            REP,
            ZONES,
            OFFSET,
            DELIM,
            B,
            B1,
            B2,
            E,
            E1,
            E2,
            FROM,
            MAPTO,
            TO,
            USE,
            R,
            RESOLV,
            L2R,
            R2L,
            FIRST,
        }
    }
}

/// Time string ---parse to a time vector
///
/// Parse the components of a time string and return a vector of the
/// components of that string. Also return an array of any modifiers
/// present in the input string; these may alter the interpretation
/// of the components.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  STRING     I   A string to be parsed as a time
///  TVEC       O   A vector giving the components of the time.
///  NTVEC      O   The number of components supplied for TVEC
///  TYPE       O   The type of the "time vector" TVEC
///  MODIFY     O   A list of modifiers present in STRING.
///  MODS       O   A logical indicating the presence of a modifier
///  YABBRV     O   A logical indicating that a year was abbreviated
///  SUCCES     O   A logical indicating whether STRING was parsed.
///  PICTUR     O   A time format picture associated with STRING
///  ERROR      O   A diagnostic message if STRING couldn't be parsed
///
///  The function returns
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is a character string that represents some
///           julian or calendar epoch.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TVEC     is a vector of double precision numbers that represent
///           the input string. The number and meaning of the
///           components of TVEC depend upon the input string. This
///           meaning can be determined from the output variable
///           TYPE.
///
///              TYPE     NTVEC     TVEC Components
///              -----------------------------------------------------
///              YMD      3 to 6    TVEC(1) is the calendar year
///                                 TVEC(2) is the numeric value of the
///                                         month (1-12)
///                                 TVEC(3) is the day of the month
///                                 TVEC(4) is the hour of the day
///                                 TVEC(5) is the minute of the hour
///                                 TVEC(6) is the second of the minute
///
///              YD       2 to 5    TVEC(1) is the calendar year
///                                 TVEC(2) is the day of the year
///                                 TVEC(3) is the hour of the day
///                                 TVEC(4) is the minute of the hour
///                                 TVEC(5) is the second of the minute
///
///              JD       1         TVEC(1) is the julian date
///
///           Note that the values of TVEC are not forced into the
///           normal ranges used in daily conversation.  TPARTV
///           simply reports what's found in the string and does
///           not pass judgement on the "correctness" of these
///           components.
///
///  NTVEC    is the actual number of components that were present
///           in the string. For example a user might have
///           supplied only year, month and day of an epoch.
///           In such a case NTVEC will be set to 3. The components
///           actually supplied will be 1 through NTVEC. Values
///           not supplied are set to zero.
///
///  TYPE     is the type of time string supplied. This is a function
///           of whether the string contains year, month and day,
///           day of year, or julian date.
///
///  MODIFY   is an array of character strings that indicate
///           whether a modifier to the calendar string was supplied.
///           If a particular modifier was not supplied, the
///           value of that component of MODIFY will be set to
///           a blank. Modifiers are used to change the meaning
///           of time strings.
///
///           For example 12:12:29 Jan 1, 1996  means 12 hours past
///           midnight on Jan 1, 1996 in the UTC time system. But
///           if we modify the string to be:
///
///              12:12:29 A.M. Jan 1, Tuesday PDT 1996 B.C.
///
///           the string takes on an entirely different meaning.
///
///           Five different modifiers are recognized by TPARTV:
///           the era associated with the epoch, day of week of
///           the epoch, time zone of an epoch,  AM/PM used in
///           daily time usage, and the system (UTC, TDB, TT, or TDT).
///
///           Again whether or not modifiers are compatible with the
///           time and date components or with each other is not
///           determined by TPARTV. TPARTV simply reports what is
///           present in the string, leaving the task of deciding
///           the meaning of the string to the calling routine.
///
///           The components of MODIFY, their meaning and possible
///           values are given below.
///
///                                     Possible
///              Component   Meaning    Non-blank Modifier Values
///              ---------   ---------  -------------------------
///              1           ERA        'A.D.', 'B.C.'
///              2           Weekday    'SUN', 'MON', ... etc.
///              3           Time Zone  'UTC+i:i', 'UTC-i:i'
///              4           AM/PM      'A.M.', 'P.M.'
///              5           System     'UTC', 'TDB', 'TT', 'TDT'
///
///           TPARTV recognizes the standard abbreviations of
///           all continental U.S. time zones.
///
///              PDT --- Pacific  Daylight Time  (UTC-07:00)
///              PST --- Pacific  Standard Time  (UTC-08:00)
///              MDT --- Mountain Daylight Time  (UTC-06:00)
///              MST --- Mountain Standard Time  (UTC-07:00)
///              CDT --- Central  Daylight Time  (UTC-05:00)
///              CST --- Central  Standard Time  (UTC-06:00)
///              EDT --- Eastern  Daylight Time  (UTC-04:00)
///              EST --- Eastern  Standard Time  (UTC-05:00)
///
///           In addition it recognizes offsets from UTC expressed
///           as UTC+/-HR:MN. Note that through out SPICELIB
///           the minutes component of the UTC offset are always
///           regarded as positive offsets from the hour offset.
///
///           All Time zones are returned in MODIFY as UTC offsets
///           as indicated in the table above.
///
///  MODS     is .TRUE. if some non-blank modifier was supplied.
///
///  YABBRV   is .TRUE. if a year was supplied in the abbreviated
///           form 'YR  where YR is a two digit integer.
///
///  SUCCES   is .TRUE. if the string was successfully parsed.
///           Otherwise it is set to .FALSE. and a diagnostic
///           is supplied in the argument ERROR.
///
///  PICTUR   is a string that gives a format picture that can
///           be used by the routine TIMOUT to construct a time
///           string of the same form as the input time string.
///
///           If some component of the input string could not be
///           identified, PICTUR is returned as a blank. However,
///           if all components of the input string could be
///           identified and the string is simply ambiguous, PICTUR
///           will contain a format picture that corresponds to
///           the ambiguous input. Consequently, you must check
///           the value of PICTUR to determine if TPARTV has
///           been able to construct a format picture.
///
///  ERROR    is blank if the string was successfully parsed.
///           Otherwise a human readable diagnostic is returned
///           in ERROR.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  All problems detected by this routine are reported via the
///      variables SUCCES and ERROR.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine parses in input string that represents some
///  epoch in some time system. In addition it constructs a
///  format picture that describes the position and meaning
///  of the various components of the string.
///
///  This routine is intended to be used in close conjunction with
///  the routines TTRANS and TIMOUT.
///
///  The string is parsed by first determining its recognizable
///  substrings (integers, punctuation marks, names of months,
///  names of weekdays, time systems, time zones, etc.) These
///  recognizable substrings are called the tokens of the input
///  string. The meaning of some tokens are immediately determined.
///  For example named months, weekdays, time systems have clear
///  meanings. However, the meanings of numeric components must
///  be deciphered from their magnitudes and location in
///  the string relative to the immediately recognized components
///  of the input string.
///
///  To determine the meaning of the numeric tokens in the input
///  string, a set of "productions rules" and transformations are
///  applied to the full set of tokens in the string. These
///  transformations are repeated until the meaning of every token
///  has been determined or until further transformations yield
///  no new clues into the meaning of the numeric tokens.
///
///  1)  Unless the substring 'JD' or 'jd' is present, the string is
///      assumed to be a calendar format (day-month-year or year and
///      day of year). If the substring JD or jd is present, the
///      string is assumed to represent a Julian date.
///
///  2)  If the Julian date specifier is not present, any integer
///      greater than 999 is regarded as being a year specification.
///
///  3)  A dash '-' can represent a minus sign only if it precedes
///      the first digit in the string and the string contains
///      the Julian date specifier (JD). (No negative years,
///      months, days, etc. are allowed).
///
///  4)  Numeric components of a time string must be separated
///      by a character that is not a digit or decimal point.
///      Only one decimal component is allowed. For example
///      1994219.12819 is sometimes interpreted as the
///      219th day of 1994 + 0.12819 days. TPARTV does not
///      support such strings.
///
///  5)  No exponential components are allowed. For example you
///      can't specify the Julian date of J2000 as 2.451545E6.
///      You also can't input 1993 Jun 23 23:00:01.202E-4 and have
///      to explicitly list all zeros that follow the decimal
///      point: i.e. 1993 Jun 23 23:00:00.0001202.
///
///  6)  The single colon (:) when used to separate numeric
///      components of a string is interpreted as separating
///      Hours, Minutes, and Seconds of time.
///
///  7)  If a double slash (//) or double colon (::) follows
///      a pair of integers, those integers are assumed  to
///      represent the year and day of year.
///
///  8)  A quote followed by an integer less than 100 is regarded
///      as an abbreviated year. For example: '93 would be regarded
///      as the 93rd year of the reference century. See the SPICELIB
///      routine TEXPYR for further discussion of abbreviated years.
///
///  9)  An integer followed by 'B.C.' or 'A.D.' is regarded as
///      a year in the era associated with that abbreviation.
///
///  10) All dates are regarded as belonging to the extended
///      Gregorian Calendar (the Gregorian calendar is the calendar
///      currently used by western society). See the SPICELIB routine
///      JUL2GR for converting from Julian Calendar to the Gregorian
///      Calendar.
///
///  11) If the ISO date-time separator (T) is present in the string
///      ISO allowed token patterns are examined for a match
///      with the current token list. If no match is found the
///      search is abandoned and appropriate diagnostic messages
///      are generated. Historically the interpretation of ISO
///      formatted time strings deviates from the ISO standard in
///      allowing two digit years and expanding years in the 0 to 99
///      range the same way as is done for non ISO formatted strings.
///      Due to this interpretation it is impossible to specify
///      times in years in the 0 A.D. to 99 A.D. range using ISO
///      formatted strings on the input.
///
///  12) If two delimiters are found in succession in the time
///      string, the time string is diagnosed as an erroneous string.
///      (Delimiters are comma, white space, dash, slash, period, or
///      day of year mark. The day of year mark is a pair of forward
///      slashes or a pair of colons.)
///
///      Note the delimiters do not have to be the same. The pair
///      of characters ",-" counts as two successive delimiters.
///
///  13) White space and commas serve only to delimit tokens in the
///      input string. They do not affect the meaning of any
///      of the tokens.
///
///  14) If an integer is greater than 1000 (and the 'JD' label
///      is not present, the integer is regarded as a year.
///
///  15) When the size of the integer components does not clearly
///      specify a year the following patterns are assumed
///
///      Calendar Format
///
///         Year Month Day
///         Month Day Year
///         Year Day Month
///
///         where Month is the name of a month, not its numeric
///         value.
///
///         When integer components are separated by slashes (/)
///         as in 3/4/5. Month, Day, Year is assumed (2005 March 4)
///
///      Day of Year Format.
///
///         If a day of year marker is present (// or ::) the
///         pattern
///
///           I-I// or I-I:: (where I stands for an integer)
///
///         is interpreted as Year Day-of-Year. However, I-I/ is
///         regarded as ambiguous.
///
///  The table below gives a list of abbreviations used to
///  classify tokens.
///
///             /   ---  slash punctuation mark
///             H   ---  hour
///             M   ---  Minute
///             S   ---  Second
///             Y   ---  year
///             d   ---  day of year marker
///             i   ---  unsigned integer
///             m   ---  month
///             n   ---  unsigned decimal number
///             y   ---  day of year
///             -   ---  dash punctuation mark
///             D   ---  day of month
///             :   ---  colon punctuation mark
///
///   Given these abbreviations the following (rather lengthy)
///   table gives the set of built in token patterns that
///   are recognized and the associated interpretation of that
///   pattern.
///
///      Pattern         Meaning         Pattern         Meaning
///      ------------------------        -------------------------
///      Y-i-it......... YmD             i/i/ii:i:n..... mDYHMS
///      Y-i-iti........ YmDH            i/i/ii:n....... mDYHM
///      Y-i-iti:i...... YmDHM           i/i/ii:n....... mDYHM
///      Y-i-iti:i:i.... YmDHMS          i:i:ii-i-Y..... HMSmDY
///      Y-i-iti:i:n.... YmDHMS          i:i:ii/i/Y..... HMSmDY
///      Y-i-iti:n...... YmDHM           i:i:ii/i/i..... HMSmDY
///      Y-i-itn........ YmDH            i:i:iimY....... HMSDmY
///      Y-i/........... Yy              i:i:imiY....... HMSmDY
///      Y-i/i:i........ YyHM            i:i:ni-i-Y..... HMSmDY
///      Y-i/i:i:i...... YyHMS           i:i:ni/i/Y..... HMSmDY
///      Y-i/i:i:n...... YyHMS           i:i:ni/i/i..... HMSmDY
///      Y-i/i:n........ YyHM            i:i:nimY....... HMSDmY
///      Y-id........... Yy              i:i:nmiY....... HMSmDY
///      Y-idi:i........ YyHM            i:ii-i-Y....... HMmDY
///      Y-idi:i:i...... YyHMS           i:ii/i/Y....... HMmDY
///      Y-idi:i:n...... YyHMS           i:ii/i/i....... HMmDY
///      Y-idi:n........ YyHM            i:iimY......... HMDmY
///      Y-it........... Yy              i:imiY......... HMmDY
///      Y-iti.......... YyH             i:ni-i-Y....... HMmDY
///      Y-iti:i........ YyHM            i:ni/i/Y....... HMmDY
///      Y-iti:i:i...... YyHMS           i:ni/i/i....... HMmDY
///      Y-iti:i:n...... YyHMS           i:nimY......... HMDmY
///      Y-iti:n........ YyHM            i:nmiY......... HMmDY
///      Y-itn.......... YyH             iYd............ yY
///      Yid............ Yy              iYdi:i......... yYHM
///      Yidi:i......... YyHM            iYdi:i:i....... yYHMS
///      Yidi:i:i....... YyHMS           iYdi:i:n....... yYHMS
///      Yidi:i:n....... YyHMS           iYdi:n......... yYHM
///      Yidi:n......... YyHM            iiY............ mDY
///      Yii............ YmD             iiYi........... mDYH
///      Yiii........... YmDH            iiYi:i......... mDYHM
///      Yiii:i......... YmDHM           iiYi:i:i....... mDYHMS
///      Yiii:i:i....... YmDHMS          iiYi:i:n....... mDYHMS
///      Yiii:i:n....... YmDHMS          iiYi:n......... mDYHM
///      Yiii:n......... YmDHM           iiYn........... mDYH
///      Yiiii.......... YmDHM           iid............ Yy
///      Yiiiii......... YmDHMS          iidi:i......... YyHM
///      Yiiiin......... YmDHMS          iidi:i:i....... YyHMS
///      Yiiin.......... YmDHM           iidi:i:n....... YyHMS
///      Yiin........... YmDH            iidi:n......... YyHM
///      Yim............ YDm             iim............ YDm
///      Yimi........... YDmH            iimi........... YDmH
///      Yimi:i......... YDmHM           iimi:i......... YDmHM
///      Yimi:i:i....... YDmHMS          iimi:i:i....... YDmHMS
///      Yimi:i:n....... YDmHMS          iimi:i:n....... YDmHMS
///      Yimi:n......... YDmHM           iimi:n......... YDmHM
///      Yimn........... YDmH            iimii.......... YDmHM
///      Yin............ YmD             iimiii......... YDmHMS
///      Ymi............ YmD             iimiin......... YDmHMS
///      Ymii........... YmDH            iimin.......... YDmHM
///      Ymii:i......... YmDHM           iimn........... YDmH
///      Ymii:i:i....... YmDHMS          imY............ DmY
///      Ymii:i:n....... YmDHMS          imYi........... DmYH
///      Ymii:n......... YmDHM           imYi:i......... DmYHM
///      Ymin........... YmDH            imYi:i:i....... DmYHMS
///      Ymn............ YmD             imYi:i:n....... DmYHMS
///      Ynm............ YDm             imYi:n......... DmYHM
///      i-Y/........... yY              imYn........... DmYH
///      i-Y/i:i........ yYHM            imi............ YmD
///      i-Y/i:i:i...... yYHMS           imi:i:iY....... DmHMSY
///      i-Y/i:i:n...... yYHMS           imi:i:nY....... DmHMSY
///      i-Y/i:n........ yYHM            imi:iY......... DmHMY
///      i-Yd........... yY              imi:nY......... DmHMY
///      i-Ydi:i........ yYHM            imii........... YmDH
///      i-Ydi:i:i...... yYHMS           imii:i......... YmDHM
///      i-Ydi:i:n...... yYHMS           imii:i:i....... YmDHMS
///      i-Ydi:n........ yYHM            imii:i:n....... YmDHMS
///      i-i-Y.......... mDY             imii:n......... YmDHM
///      i-i-Yi:i....... mDYHM           imiii.......... YmDHM
///      i-i-Yi:i:i..... mDYHMS          imiiii......... YmDHMS
///      i-i-Yi:i:n..... mDYHMS          imiiin......... YmDHMS
///      i-i-Yi:n....... mDYHM           imiin.......... YmDHM
///      i-i-it......... YmD             imin........... YmDH
///      i-i-iti........ YmDH            imn............ YmD
///      i-i-iti:i...... YmDHM           inY............ mDY
///      i-i-iti:i:i.... YmDHMS          inm............ YDm
///      i-i-iti:i:n.... YmDHMS          miY............ mDY
///      i-i-iti:n...... YmDHM           miYi........... mDYH
///      i-i-itn........ YmDH            miYi:i......... mDYHM
///      i-i/i:i........ YyHM            miYi:i:i....... mDYHMS
///      i-i/i:i:i...... YyHMS           miYi:i:n....... mDYHMS
///      i-i/i:i:n...... YyHMS           miYi:n......... mDYHM
///      i-i/i:n........ YyHM            miYn........... mDYH
///      i-idi:i........ YyHM            mii............ mDY
///      i-idi:i:i...... YyHMS           mii:i:iY....... mDHMSY
///      i-idi:i:n...... YyHMS           mii:i:nY....... mDHMSY
///      i-idi:n........ YyHM            mii:iY......... mDHMY
///      i-it........... Yy              mii:nY......... mDHMY
///      i-iti.......... YyH             miii........... mDYH
///      i-iti:i........ YyHM            miii:i......... mDYHM
///      i-iti:i:i...... YyHMS           miii:i:i....... mDYHMS
///      i-iti:i:n...... YyHMS           miii:i:n....... mDYHMS
///      i-iti:n........ YyHM            miii:n......... mDYHM
///      i-itn.......... YyH             miiii.......... mDYHM
///      i/i/Y.......... mDY             miiiii......... mDYHMS
///      i/i/Y/i:n...... mDYHM           miiiin......... mDYHMS
///      i/i/Yi:i....... mDYHM           miiin.......... mDYHM
///      i/i/Yi:i:i..... mDYHMS          miin........... mDYH
///      i/i/Yi:i:n..... mDYHMS          mnY............ mDY
///      i/i/i.......... mDY             mni............ mDY
///      i/i/ii:i....... mDYHM           nmY............ DmY
///      i/i/ii:i:i..... mDYHMS
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you need to convert various time strings to ephemeris
///  seconds past J2000. The following pair of calls shows
///  how you would use this routine together with the routines
///  TCHECK and TTRANS to perform this task.
///
///
///      CALL TPARTV ( STRING,
///     .              TVEC,   NTVEC, TYPE,
///     .              MODIFY, MODS,  YABBRV, SUCCES,
///     .              PICTUR, ERROR )
///
///
///      IF ( .NOT. SUCCES ) THEN
///
///         Use the SPICE error handling facility to post an
///         error message and signal an error.
///
///         CALL SETMSG ( ERROR )
///         CALL SIGERR ( 'MYCHECK(BADTIME)' )
///         CALL CHKOUT ( 'MYROUTINE' )
///         RETURN
///      END IF
///
///      Check the components of TVEC to make sure everything
///      makes sense.
///
///      CALL TCHECK( TVEC, TYPE, MODS, MODIFY, OK, ERROR )
///
///      IF ( .NOT. OK ) THEN
///
///         Use the SPICE error handling facility to post an
///         error message and signal an error.
///
///         CALL SETMSG ( ERROR )
///         CALL SIGERR ( 'MYCHECK(BADTIME)' )
///         CALL CHKOUT ( 'MYROUTINE' )
///         RETURN
///      END IF
///
///      CALL TTRANS ( TYPE, 'ET', TVEC )
///
///      ET = TVEC(1)
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
/// -    SPICELIB Version 3.2.0, 23-DEC-2021 (EDW) (BVS) (JDR)
///
///         ISO format logic recognizes/evaluates ISO time strings
///         with or without trailing 'Z'.
///
///         Reordered header sections. Edited the header to comply with
///         NAIF standard.
///
///         Updated $Exceptions entry #1 wording.
///
/// -    SPICELIB Version 3.1.0, 15-AUG-2002 (WLT)
///
///         Replaced the call to INSSUB with ZZINSSUB so that this
///         routine can legitimately be called error free.
///
/// -    SPICELIB Version 3.0.0, 10-MAY-1999 (WLT)
///
///         The routine was modified so that weekday followed by a comma
///         is recognized as a legitimate pattern when parsing.
///
/// -    SPICELIB Version 2.0.0, 16-APR-1997 (WLT)
///
///         The routine was modified so that last-chance removal of
///         delimiters ',', '-', and '/' are removed one at a time
///         (instead of all at once as in version 1.0.0) and the
///         resulting representation checked against
///         the built-in list.
///
///         In addition the set of built-in patterns was increased
///         from 185 to 203. See ZZTPATS for more details.
///
/// -    SPICELIB Version 1.0.0, 10-AUG-1996 (WLT)
/// ```
pub fn tpartv(
    ctx: &mut SpiceContext,
    string: &str,
    tvec: &mut [f64],
    ntvec: &mut i32,
    type_: &mut str,
    modify: CharArrayMut,
    mods: &mut bool,
    yabbrv: &mut bool,
    succes: &mut bool,
    pictur: &mut str,
    error: &mut str,
) {
    TPARTV(
        string.as_bytes(),
        tvec,
        ntvec,
        fstr::StrBytes::new(type_).as_mut(),
        modify,
        mods,
        yabbrv,
        succes,
        fstr::StrBytes::new(pictur).as_mut(),
        fstr::StrBytes::new(error).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure TPARTV ( Time string ---parse to a time vector)
pub fn TPARTV(
    STRING: &[u8],
    TVEC: &mut [f64],
    NTVEC: &mut i32,
    TYPE: &mut [u8],
    MODIFY: CharArrayMut,
    MODS: &mut bool,
    YABBRV: &mut bool,
    SUCCES: &mut bool,
    PICTUR: &mut [u8],
    ERROR: &mut [u8],
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TVEC = DummyArrayMut::new(TVEC, 1..);
    let mut MODIFY = DummyCharArrayMut::new(MODIFY, None, 1..);

    //
    // SPICELIB Functions
    //

    //
    // Private Functions
    //

    //
    // Parameters
    //
    //
    // ERA
    // WDAY
    // ZONE
    // AMPM
    // SYSTEM
    //

    //
    // Local Variables.
    //
    // The number of known time patterns NKNOWN comes from the include
    // file timepars.inc
    //

    //
    // Time Zone Variables
    //

    //
    // Standard SPICE error handling.
    //
    //
    // So far there are no modifiers to the time string.
    //
    *MODS = false;
    *YABBRV = false;

    for I in 1..=NMODS {
        fstr::assign(MODIFY.get_mut(I), b" ");
    }

    //
    // On the first call to this routine we load the built in
    // representation patterns.
    //
    if save.FIRST {
        if ZZTPATS(
            ROOM,
            &mut save.NKNOWN,
            save.KNOWN.as_arg_mut(),
            save.MEANNG.as_arg_mut(),
            ctx,
        ) {
            save.FIRST = false;
        } else {
            fstr::assign(PICTUR, b" ");
            *SUCCES = false;
            fstr::assign(ERROR, b"There is an incompatibility between ZZTPATS and the room allocated for KNOWN in TPARTV.");
            return;
        }
    }

    //
    // First step is to tokenize the string.  The new representation
    // is maintained in ZZTIME.  We'll get it later if we need it.
    //
    save.RESOLV = ZZTOKNS(STRING, ERROR, ctx);

    if !save.RESOLV {
        *SUCCES = false;
        *NTVEC = 0;
        fstr::assign(TYPE, b" ");
        fstr::assign(PICTUR, b" ");
        return;
    }

    //
    // The result of tokenizing the string will be a representation
    // that contains the following letters.
    //
    //       '        The quote character
    //       [        The left parenthesis
    //       ]        The right parenthesis
    //       ,        The comma
    //       -        The dash
    //       .        The decimal point
    //       /        The slash---used to separate date components.
    //       :        The colon (used to separate time components)
    //       N  ---   stands for one of the symbols A.M. or P.M.
    //       O        stands for the symbol UTC+
    //       Z  ---   stands for a time zone such as PDT, PSD, CDT, etc.
    //       b        stands for a block of white space
    //       d        stands for the day of year marker (// or ::)
    //       e  ---   stands for the era (B.C. or A.D.)
    //       j        stands for julian date
    //       m        stands for a month
    //       o        stands for the symbol UTC-
    //       s  ---   stands for a time system (UTC, TDB, TT, TDT)
    //       t        stands the ISO date-T-time separator.
    //       w  ---   stands for the day of the week.
    //       i        stands for a sequence of digits
    //
    // We will gradually remove many of these and replace the i, i.
    // and i.i with the following items
    //
    //       n       stands for a decimal number
    //       Y       stands for a year
    //       D       stands for the day in a month
    //       y       stands for the day of the year
    //       H       stands for hours
    //       M       stands for minutes
    //       S       stands for seconds
    //
    //
    // We will use the following logical functions to modify
    // the tokenized representation:
    //
    //    ZZTOKNS --- breaks the string down into a list of recognized
    //                tokens and stores an internal model for this
    //                list.  The begins and ends of the substrings
    //                associated with the tokenization are maintained
    //                inside the routine ZZTIME (which ZZTOKNS is an
    //                entry point to).  If some substring cannot be
    //                recognized, ZZTOKNS returns the value FALSE
    //                together with a diagnostic indicating what
    //                was wrong with the input string.
    //
    //    ZZCMBT  --- combines one or more tokens into a single token.
    //                this is performed only once and is done either
    //                scanning left to right or right to left.
    //                It returns TRUE if a combination is performed.
    //
    //    ZZREMT  --- removes all instances of a token from the tokenized
    //                representation.  It returns TRUE is an item
    //                is removed.
    //
    //    ZZSUBT  --- substitutes the first occurrence of a
    //                subpattern (scanning left to right or right to
    //                left) with another pattern of the same length.
    //                This is where we attach new meaning to the
    //                tokenized pattern.  It returns TRUE if a
    //                substitution is performed.
    //
    //    ZZREPT  --- is a combination of the ZZSUBT and ZZREMT
    //                This performs ZZSUBT on the string, but then
    //                remove all occurrences of the special character
    //                * from the tokenized list. It returns TRUE
    //                is a substitution is performed.
    //
    //    ZZNOTE  --- returns the begin and end of the first occurrence
    //                of some token, and then removes the token
    //                from the tokenized representation.  We use this
    //                primarily to extract modifiers from the tokenized
    //                string.  These should occur only once and once
    //                removed allow us to more easily attach meaning
    //                to the remaining tokens. The value of ZZNOTE
    //                is true if the requested item could be found,
    //                otherwise it is false and the begin and end
    //                of the requested substring are set to 0.
    //
    //    ZZIST   --- returns TRUE if the specified token is present
    //                in the tokenized substring.
    //
    //    ZZISPT  --- returns true is a pair of consecutive tokens
    //                from a list are located in the representation
    //                of the tokenized string.  This is used to
    //                locate consecutive pairs of delimiters in the
    //                input string. It returns TRUE if a pair of
    //                consecutive items is located.  Otherwise
    //                it returns FALSE.
    //
    //    ZZVALT  --- allows you to substitute a new token for any
    //                integer (i) that lies within a specified range
    //                of values.  This is primarily used to recognize
    //                years in the input string.
    //
    //    ZZGREP  --- is used to get the current representation of the
    //                tokenized string (with all processing resulting
    //                from use of the manipulation routines taken into
    //                account).
    //
    //    ZZTPATS --- is used to set up the large list of canned patterns
    //                that are recognized as legitimate tokenizations.
    //                Almost all legitimate time strings when tokenized
    //                will match one of these patterns.
    //
    //    ZZUNPCK --- uses STRING together with the current
    //                representation of it's tokens to return a
    //                time vector.  If a problem is encountered with
    //                the current tokens, it returns a diagnostic
    //                message that indicates why the string
    //                could not be parsed.  Note ZZUNPCK should be
    //                called only after all string modifiers have
    //                been retrieved via a call to ZZNOTE (or by
    //                manually removing them).
    //
    // Next Step is to combine some tokens so that we won't run
    // into problems later on.  We may introduce some new components
    // in the process.
    //
    save.L2R = true;
    save.R2L = !save.L2R;

    if ZZCMBT(b"Oi", b"z", save.L2R, ctx) {
        save.RESOLV = ZZCMBT(b"z:i", b"Z", save.L2R, ctx);
        save.RESOLV = ZZSUBT(b"z", b"Z", save.L2R, ctx);
    }

    if ZZCMBT(b"oi", b"z", save.L2R, ctx) {
        save.RESOLV = ZZCMBT(b"z:i", b"Z", save.L2R, ctx);
        save.RESOLV = ZZSUBT(b"z", b"Z", save.L2R, ctx);
    }

    //
    // Next we resolve any months, or weekdays that are followed
    // by periods.
    //
    save.RESOLV = ZZREPT(b"m.", b"m*", save.L2R, ctx);
    save.RESOLV = ZZREPT(b"w.", b"w*", save.L2R, ctx);
    save.RESOLV = ZZREPT(b"w,", b"w*", save.L2R, ctx);

    //
    // Now convert the right most integer-decimal-point pair to the
    // number representation.
    //
    if ZZCMBT(b"i.i", b"n", save.R2L, ctx) {
        //
        // We aren't going to do anything here.  We are simply
        // using the IF-THEN...ELSE IF ... ENDIF  to make sure
        // we only replace one decimal place.
        //
    } else if ZZCMBT(b"i.", b"n", save.R2L, ctx) {
        //
        // Same as the previous comment.
        //
    }
    //
    // Remove any white space from the tokenization.
    //
    save.RESOLV = ZZREMT(b"b", ctx);

    //
    // User Custom Formats (this still needs a modicum of work).
    // ----------------------------------------------------------------
    // ================================================================
    //
    //
    // RESOLV = ZZGREP ( REP )
    // USE    = ISRCHC ( REP, NCUSTM, CUSTOM )
    //
    // IF ( USE .GT. 0 ) THEN
    //    RESOLV = ZZREPT ( CUSTM(USE), CMEANS(USE), L2R )
    // ELSE
    //    RESOLV =  .FALSE.
    // END IF
    //
    // IF ( RESOLV ) THEN
    //
    //    SUCCES = ZZUNPCK ( STRING, YABBRV, ...
    //                       TVEC,   NTVEC, TYPE, PICTUR, ERROR )
    //    ERROR  = ' '
    //
    //    RETURN
    // END IF
    //
    //

    //
    // Julian Date
    // ----------------------------------------------------------------
    // ================================================================
    //
    if ZZIST(b"j", ctx) {
        //
        // This is some form of Julian Date. Handle this case
        // right here and return.
        //
        save.RESOLV = ZZREPT(b"[s]", b"*s*", save.L2R, ctx);
        *MODS = (*MODS || ZZNOTE(b"s", &mut save.B, &mut save.E, ctx));

        if *MODS {
            UCASE(
                fstr::substr(STRING, save.B..=save.E),
                &mut MODIFY[SYSTEM],
                ctx,
            );
        }

        save.RESOLV = ZZREPT(b"[j]", b"*j*", save.L2R, ctx);
        save.RESOLV = ZZREMT(b"j", ctx);

        if !ZZIST(b"n", ctx) {
            save.RESOLV = ZZSUBT(b"i", b"n", save.L2R, ctx);
        }

        save.RESOLV = ZZCMBT(b"-n", b"n", save.L2R, ctx);
        save.RESOLV = ZZSUBT(b"n", b"J", save.L2R, ctx);

        //
        // We let ZZUNPK handle the parsing or diagnosis of any problems.
        //
        *SUCCES = ZZUNPCK(
            STRING,
            *YABBRV,
            TVEC.as_slice_mut(),
            NTVEC,
            TYPE,
            PICTUR,
            ERROR,
            ctx,
        );

        if (intrinsics::INDEX(PICTUR, b"JULIAND.") > 0) {
            SUFFIX(b"::RND", 1, PICTUR);
        }

        if fstr::ne(MODIFY.get(SYSTEM), b" ") {
            SUFFIX(b"::", 1, PICTUR);
            SUFFIX(&MODIFY[SYSTEM], 0, PICTUR);
        }

        return;
    }

    //
    // Calendar Date Formats.
    // ----------------------------------------------------------------
    // ================================================================
    //
    // Replace any integers greater than 1000 by Y.
    //
    save.B = 1000;
    save.E = INTMAX();
    save.RESOLV = ZZVALT(STRING, save.B, save.E, b"Y", ctx);

    //
    // If the ISO time delimiter 't' is present we don't perform
    // any further simplifications.
    //
    if ZZIST(b"t", ctx) {
        save.RESOLV = ZZGREP(&mut save.REP, ctx);
        save.USE = BSRCHC(&save.REP, save.NKNOWN, save.KNOWN.as_arg());

        if (save.USE != 0) {
            save.RESOLV = ZZREPT(&save.KNOWN[save.USE], &save.MEANNG[save.USE], save.L2R, ctx);

            *SUCCES = ZZUNPCK(
                STRING,
                *YABBRV,
                TVEC.as_slice_mut(),
                NTVEC,
                TYPE,
                PICTUR,
                ERROR,
                ctx,
            );

            //
            // If you tag an ISO as A.D., other logic in the Time
            // subsystem will not intepret the two digit year value as a
            // two digit abbreviation.
            //
            //  MODIFY(ERA) = 'A.D.'
            //

            if (intrinsics::INDEX(PICTUR, b".#") != 0) {
                SUFFIX(b"::RND", 1, PICTUR);
            }

            if fstr::ne(MODIFY.get(ZONE), b" ") {
                SUFFIX(b"::", 1, PICTUR);
                SUFFIX(&MODIFY[ZONE], 0, PICTUR);
            }

            if fstr::ne(MODIFY.get(SYSTEM), b" ") {
                SUFFIX(b"::", 1, PICTUR);
                SUFFIX(&MODIFY[SYSTEM], 0, PICTUR);
            }
        } else {
            *SUCCES = false;
            *NTVEC = 0;
            *MODS = false;
            fstr::assign(TYPE, b" ");
            fstr::assign(PICTUR, b" ");

            fstr::assign(ERROR, b"The input string uses the ISO  \"T\" date/time delimiter but does not match any of the accepted ISO formats. ");
        }

        return;
    }

    //
    // If we reach this point, either we didn't have any custom
    // formats supplied or we didn't match any of them.
    // Resolve any abbreviated years.  We've already set integers
    // that are 1000 or greater to 'Y'  Only 1 or 2 digit integers
    // can be year abbreviations.  We replace the 3 digit integers
    // with I temporarily; locate any abbreviated years; reset all
    // the 3-digit back to 'i'.  (Note 3-digit means value between
    // 100 and 999.  003 is not regarded as a 3 digit number).
    //
    save.B = 100;
    save.E = 1000;
    save.RESOLV = ZZVALT(STRING, save.B, save.E, b"I", ctx);
    *YABBRV = ZZREPT(b"\'i", b"*Y", save.L2R, ctx);

    while ZZSUBT(b"I", b"i", save.L2R, ctx) {
        save.B = (save.B + 1);
    }

    //
    // Resolve the system, and other text components.
    //
    save.RESOLV = ZZREPT(b"[e]", b"*e*", save.L2R, ctx);
    save.RESOLV = ZZREPT(b"[w]", b"*w*", save.L2R, ctx);
    save.RESOLV = ZZREPT(b"[N]", b"*N*", save.L2R, ctx);
    save.RESOLV = ZZREPT(b"[Z]", b"*Z*", save.L2R, ctx);
    save.RESOLV = ZZREPT(b"[s]", b"*s*", save.L2R, ctx);
    save.RESOLV = ZZSUBT(b"ie", b"Ye", save.L2R, ctx);
    //
    // Note the positions of ERA, WEEKDAY, TIME-ZONE, AMPM marker
    // and time SYSTEM.
    //

    save.HAVERA = ZZNOTE(b"e", &mut save.BEGS[ERA], &mut save.ENDS[ERA], ctx);
    save.HAVWDY = ZZNOTE(b"w", &mut save.BEGS[WDAY], &mut save.ENDS[WDAY], ctx);
    save.HAVZON = ZZNOTE(b"Z", &mut save.BEGS[ZONE], &mut save.ENDS[ZONE], ctx);
    save.HAVAPM = ZZNOTE(b"N", &mut save.BEGS[AMPM], &mut save.ENDS[AMPM], ctx);
    save.HAVSYS = ZZNOTE(b"s", &mut save.BEGS[SYSTEM], &mut save.ENDS[SYSTEM], ctx);

    *MODS = ((((save.HAVERA || save.HAVWDY) || save.HAVZON) || save.HAVAPM) || save.HAVSYS);

    if *MODS {
        for I in 1..=NMODS {
            if (save.BEGS[I] != 0) {
                UCASE(
                    fstr::substr(STRING, save.BEGS[I]..=save.ENDS[I]),
                    &mut MODIFY[I],
                    ctx,
                );
            }
        }

        if save.HAVERA {
            if fstr::eq(fstr::substr(MODIFY.get(ERA), 1..=1), b"A") {
                fstr::assign(MODIFY.get_mut(ERA), b"A.D.");
            } else {
                fstr::assign(MODIFY.get_mut(ERA), b"B.C.");
            }
        }

        if save.HAVAPM {
            if fstr::eq(fstr::substr(MODIFY.get(AMPM), 1..=1), b"A") {
                fstr::assign(MODIFY.get_mut(AMPM), b"A.M.");
            } else {
                fstr::assign(MODIFY.get_mut(AMPM), b"P.M.");
            }
        }

        fstr::assign(fstr::substr_mut(MODIFY.get_mut(WDAY), 4..), b" ");

        if save.HAVZON {
            save.MAPTO = ISRCHC(&MODIFY[ZONE], NZONES, save.ZONES.as_arg());

            if (save.MAPTO != 0) {
                fstr::assign(MODIFY.get_mut(ZONE), save.OFFSET.get(save.MAPTO));
            }
        }
    }

    //
    // Try our built in formats without any further substitution.
    //
    save.RESOLV = ZZGREP(&mut save.REP, ctx);
    save.USE = BSRCHC(&save.REP, save.NKNOWN, save.KNOWN.as_arg());

    if (save.USE > 0) {
        save.RESOLV = ZZREPT(&save.KNOWN[save.USE], &save.MEANNG[save.USE], save.L2R, ctx);
        *SUCCES = ZZUNPCK(
            STRING,
            *YABBRV,
            TVEC.as_slice_mut(),
            NTVEC,
            TYPE,
            PICTUR,
            ERROR,
            ctx,
        );

        if (intrinsics::INDEX(PICTUR, b".#") != 0) {
            SUFFIX(b"::RND", 1, PICTUR);
        }

        if fstr::ne(MODIFY.get(ZONE), b" ") {
            SUFFIX(b"::", 1, PICTUR);
            SUFFIX(&MODIFY[ZONE], 0, PICTUR);
        }

        if fstr::ne(MODIFY.get(SYSTEM), b" ") {
            SUFFIX(b"::", 1, PICTUR);
            SUFFIX(&MODIFY[SYSTEM], 0, PICTUR);
        }

        return;
    }
    //
    // Make sure we don't have a pair of successive delimiters
    // or a delimiter at either end of the input string.
    //
    if ZZISPT(b",/-:d.", &mut save.FROM, &mut save.TO, ctx) {
        *SUCCES = false;
        *NTVEC = 0;
        fstr::assign(TYPE, b" ");

        fstr::assign(ERROR, STRING);
        ZZINSSUB(&ERROR.to_vec(), b">", (save.TO + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.FROM, ERROR);
        PREFIX(b"There are two successive delimiters <#> in the input string.  This is an ambiguous input. \' ", 0, ERROR);
        REPMC(
            &ERROR.to_vec(),
            b"#",
            fstr::substr(STRING, save.FROM..=save.TO),
            ERROR,
        );
        SUFFIX(b"\'", 0, ERROR);
        fstr::assign(PICTUR, b" ");
        return;
    }

    //
    // A delimiter hanging at either end of the string shall be
    // regarded as an error.
    //
    save.RESOLV = ZZGREP(&mut save.REP, ctx);
    save.R = RTRIM(&save.REP);

    if (intrinsics::INDEX(b",/-:.", fstr::substr(&save.REP, 1..=1)) > 0) {
        save.RESOLV = ZZSUBT(fstr::substr(&save.REP, 1..=1), b"Q", save.L2R, ctx);
        save.RESOLV = false;
    } else if (intrinsics::INDEX(b",/-:.", fstr::substr(&save.REP, save.R..=save.R)) > 0) {
        save.RESOLV = ZZSUBT(
            fstr::substr(&save.REP, save.R..=save.R),
            b"Q",
            save.L2R,
            ctx,
        );
        save.RESOLV = false;
    }

    if !save.RESOLV {
        save.RESOLV = ZZNOTE(b"Q", &mut save.FROM, &mut save.TO, ctx);
        fstr::assign(ERROR, STRING);
        ZZINSSUB(&ERROR.to_vec(), b">", (save.TO + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.FROM, ERROR);
        PREFIX(
            b"An unexpected delimiter (\'#\') was encountered in the input string. \' ",
            0,
            ERROR,
        );
        SUFFIX(b"\'", 0, ERROR);
        REPMC(
            &ERROR.to_vec(),
            b"#",
            fstr::substr(STRING, save.FROM..=save.TO),
            ERROR,
        );
        fstr::assign(PICTUR, b" ");
        *SUCCES = false;
        return;
    }
    //
    // We probably made it unscathed through the check above.
    // Remove delimiters ',', '/', and '-' and retry the built-in
    // patterns.
    //
    fstr::assign(save.DELIM.get_mut(1), b",");
    fstr::assign(save.DELIM.get_mut(2), b"-");
    fstr::assign(save.DELIM.get_mut(3), b"/");

    for I in 1..=3 {
        save.RESOLV = ZZREMT(&save.DELIM[I], ctx);

        save.RESOLV = ZZGREP(&mut save.REP, ctx);
        save.USE = BSRCHC(&save.REP, save.NKNOWN, save.KNOWN.as_arg());

        if (save.USE > 0) {
            save.RESOLV = ZZREPT(&save.KNOWN[save.USE], &save.MEANNG[save.USE], save.L2R, ctx);
            *SUCCES = ZZUNPCK(
                STRING,
                *YABBRV,
                TVEC.as_slice_mut(),
                NTVEC,
                TYPE,
                PICTUR,
                ERROR,
                ctx,
            );

            if (intrinsics::INDEX(PICTUR, b".#") != 0) {
                SUFFIX(b"::RND", 1, PICTUR);
            }

            if fstr::ne(MODIFY.get(ZONE), b" ") {
                SUFFIX(b"::", 1, PICTUR);
                SUFFIX(&MODIFY[ZONE], 0, PICTUR);
            }

            if fstr::ne(MODIFY.get(SYSTEM), b" ") {
                SUFFIX(b"::", 1, PICTUR);
                SUFFIX(&MODIFY[SYSTEM], 0, PICTUR);
            }

            return;
        }
    }

    //
    // If we make it to this point, we must have a pretty funky
    // time string.  There are some obvious incompatibilities. We
    // check them now
    //

    if ZZNOTE(b"e", &mut save.B, &mut save.E, ctx) {
    } else if ZZNOTE(b"s", &mut save.B, &mut save.E, ctx) {
    } else if ZZNOTE(b"Z", &mut save.B, &mut save.E, ctx) {
    } else if ZZNOTE(b"w", &mut save.B, &mut save.E, ctx) {
    } else if ZZNOTE(b"N", &mut save.B, &mut save.E, ctx) {
    }
    //
    // If B is non-zero the item in question is a duplicate
    // modifier.
    //
    if (save.B > 0) {
        *SUCCES = false;
        *NTVEC = 0;
        fstr::assign(TYPE, b" ");
        fstr::assign(ERROR, STRING);

        ZZINSSUB(&ERROR.to_vec(), b">", (save.E + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.B, ERROR);
        PREFIX(
            b"The substring \"#\" is a duplicate modifier of the input string: \' ",
            0,
            ERROR,
        );
        SUFFIX(b"\'", 0, ERROR);
        REPMC(
            &ERROR.to_vec(),
            b"#",
            fstr::substr(STRING, save.B..=save.E),
            ERROR,
        );
        fstr::assign(PICTUR, b" ");
        return;
    }
    //
    // Look for unresolved markers
    //
    if ZZNOTE(b"[", &mut save.B, &mut save.E, ctx) {
    } else if ZZNOTE(b"]", &mut save.B, &mut save.E, ctx) {
    } else if ZZNOTE(b"O", &mut save.B, &mut save.E, ctx) {
    } else if ZZNOTE(b"o", &mut save.B, &mut save.E, ctx) {
    } else if ZZNOTE(b"z", &mut save.B, &mut save.E, ctx) {
    }

    if (save.B > 0) {
        *SUCCES = false;
        *NTVEC = 0;
        fstr::assign(TYPE, b" ");
        fstr::assign(ERROR, STRING);

        ZZINSSUB(&ERROR.to_vec(), b">", (save.E + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.B, ERROR);
        PREFIX(
            b"The substring \"#\" could not be resolved in the input string: \' ",
            0,
            ERROR,
        );
        SUFFIX(b"\'", 0, ERROR);
        REPMC(
            &ERROR.to_vec(),
            b"#",
            fstr::substr(STRING, save.B..=save.E),
            ERROR,
        );
        fstr::assign(PICTUR, b" ");
        return;
    }

    if (ZZIST(b"m", ctx) && ZZIST(b"d", ctx)) {
        *SUCCES = false;
        *NTVEC = 0;
        fstr::assign(TYPE, b" ");
        fstr::assign(ERROR, STRING);
        save.RESOLV = ZZNOTE(b"m", &mut save.B1, &mut save.E1, ctx);
        save.RESOLV = ZZNOTE(b"d", &mut save.B2, &mut save.E2, ctx);

        save.B = intrinsics::MAX0(&[save.B1, save.B2]);
        save.E = intrinsics::MAX0(&[save.E1, save.E2]);

        ZZINSSUB(&ERROR.to_vec(), b">", (save.E + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.B, ERROR);

        save.B = intrinsics::MIN0(&[save.B1, save.B2]);
        save.E = intrinsics::MIN0(&[save.E1, save.E2]);

        ZZINSSUB(&ERROR.to_vec(), b">", (save.E + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.B, ERROR);

        PREFIX(
            b"Both a month \"#\" and day of year delimiter \"#\" appear in the input string: \' ",
            0,
            ERROR,
        );
        SUFFIX(b"\'", 0, ERROR);
        REPMC(
            &ERROR.to_vec(),
            b"#",
            fstr::substr(STRING, save.B1..=save.E1),
            ERROR,
        );
        REPMC(
            &ERROR.to_vec(),
            b"#",
            fstr::substr(STRING, save.B2..=save.E2),
            ERROR,
        );
        fstr::assign(PICTUR, b" ");
        return;
    }

    //
    // Make the remaining obvious substitutions for hours,
    // minutes, and seconds
    //
    if ZZREPT(b"i:i:i:n", b"D*H*M*S", save.R2L, ctx) {
    } else if ZZREPT(b"i:i:i:i", b"D*H*M*S", save.R2L, ctx) {
    } else if ZZREPT(b"i:i:n", b"H*M*S", save.R2L, ctx) {
    } else if ZZREPT(b"i:i:i", b"H*M*S", save.R2L, ctx) {
    } else if ZZREPT(b"i:n", b"H*M", save.R2L, ctx) {
    } else if ZZREPT(b"i:i", b"H*M", save.R2L, ctx) {
    }

    save.RESOLV = ZZREMT(b":", ctx);
    //
    // Handle the obvious substitutions of an integer next to
    // a Month.
    //
    if ZZSUBT(b"<miiH", b"mDY", save.L2R, ctx) {
    } else if ZZSUBT(b"<mi", b"mD", save.L2R, ctx) {
    } else if ZZSUBT(b"Siim>", b"SYDm", save.L2R, ctx) {
    } else if ZZSUBT(b"im>", b"Dm", save.L2R, ctx) {
    } else if ZZSUBT(b"miY>", b"mDY", save.L2R, ctx) {
    } else if ZZSUBT(b"Ymi", b"YmD", save.L2R, ctx) {
    } else if ZZSUBT(b"Smi", b"SmD", save.L2R, ctx) {
    } else if ZZSUBT(b"Mmi", b"MmD", save.L2R, ctx) {
    } else if ZZSUBT(b"imY", b"DmY", save.L2R, ctx) {
    } else if ZZSUBT(b"imH", b"DmH", save.L2R, ctx) {
    } else if ZZREPT(b"Yid", b"Yy*", save.L2R, ctx) {
    } else if ZZREPT(b"iYd", b"yY*", save.L2R, ctx) {
    } else if ZZREPT(b"Ydi", b"Y*y", save.L2R, ctx) {
    }

    //
    // That's it we let ZZUNPCK handle the problem of diagnosing
    // or decoding the current representation.
    //

    *SUCCES = ZZUNPCK(
        STRING,
        *YABBRV,
        TVEC.as_slice_mut(),
        NTVEC,
        TYPE,
        PICTUR,
        ERROR,
        ctx,
    );

    if fstr::ne(PICTUR, b" ") {
        if (intrinsics::INDEX(PICTUR, b".#") != 0) {
            SUFFIX(b"::RND", 1, PICTUR);
        }

        if fstr::ne(MODIFY.get(ZONE), b" ") {
            SUFFIX(b"::", 1, PICTUR);
            SUFFIX(&MODIFY[ZONE], 0, PICTUR);
        }

        if fstr::ne(MODIFY.get(SYSTEM), b" ") {
            SUFFIX(b"::", 1, PICTUR);
            SUFFIX(&MODIFY[SYSTEM], 0, PICTUR);
        }
    }
}
