//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CURRNT: i32 = 1;
const BEGIN: i32 = CURRNT;
const NONAME: i32 = (CURRNT + 1);
const YEAR: i32 = (NONAME + 1);
const YR: i32 = (YEAR + 1);
const UMON: i32 = (YR + 1);
const MMON: i32 = (UMON + 1);
const LMON: i32 = (MMON + 1);
const UMNTH: i32 = (LMON + 1);
const MMNTH: i32 = (UMNTH + 1);
const LMNTH: i32 = (MMNTH + 1);
const MONTH: i32 = (LMNTH + 1);
const DOY: i32 = (MONTH + 1);
const UWKD: i32 = (DOY + 1);
const MWKD: i32 = (UWKD + 1);
const LWKD: i32 = (MWKD + 1);
const UWEEKD: i32 = (LWKD + 1);
const MWEEKD: i32 = (UWEEKD + 1);
const LWEEKD: i32 = (MWEEKD + 1);
const DAY: i32 = (LWEEKD + 1);
const MINUTE: i32 = (DAY + 1);
const HOUR: i32 = (MINUTE + 1);
const SEC: i32 = (HOUR + 1);
const POINT: i32 = (SEC + 1);
const PLACE: i32 = (POINT + 1);
const JULIAN: i32 = (PLACE + 1);
const UTC: i32 = (JULIAN + 1);
const TDB: i32 = (UTC + 1);
const TDT: i32 = (TDB + 1);
const TT: i32 = (TDT + 1);
const SP2000: i32 = (TT + 1);
const SP1950: i32 = (SP2000 + 1);
const ROUND: i32 = (SP1950 + 1);
const TRUNC: i32 = (ROUND + 1);
const UERA: i32 = (TRUNC + 1);
const LERA: i32 = (UERA + 1);
const UERAX: i32 = (LERA + 1);
const LERAX: i32 = (UERAX + 1);
const UAMPM: i32 = (LERAX + 1);
const LAMPM: i32 = (UAMPM + 1);
const UTCP: i32 = (LAMPM + 1);
const UTCM: i32 = (UTCP + 1);
const JCAL: i32 = (UTCM + 1);
const GCAL: i32 = (JCAL + 1);
const MCAL: i32 = (GCAL + 1);
const TIMSYS: i32 = (MCAL + 1);
const CALNDR: i32 = (TIMSYS + 1);
const AMPM: i32 = (CALNDR + 1);
const MON: i32 = (AMPM + 1);
const WKDAY: i32 = (MON + 1);
const ERA: i32 = (WKDAY + 1);
const NOON: i32 = (ERA + 1);
const RLYEAR: i32 = (NOON + 1);
const FINISH: i32 = RLYEAR;
const YMD: i32 = 1;
const CONTIN: i32 = (YMD + 1);
const MAXMRK: i32 = 43;
const ROOM: i32 = 100;
const MAXLEN: i32 = 256;
const MRKSIZ: i32 = 8;
const LOCLEN: i32 = 32;

struct SaveVars {
    FIRST: bool,
    MARKS: ActualCharArray,
    CLASS: StackArray<i32, 44>,
    NMARKS: i32,
    MRKLEN: StackArray<i32, 43>,
    PNTRS: StackArray<i32, 100>,
    DUMP: StackArray<i32, 11>,
    NDUMP: i32,
    TIMFMT: i32,
    TYPE: i32,
    NUMTYP: i32,
    ORIGNL: ActualCharArray,
    VALUES: StackArray<f64, 52>,
    PAD: StackArray<f64, 52>,
    LENGTH: StackArray<i32, 52>,
    ID: StackArray<i32, 52>,
    HAVE: StackArray<bool, 52>,
    COMPNT: StackArray2D<i32, 16>,
    MYSTR: Vec<u8>,
    SUBSTR: Vec<u8>,
    STRING: Vec<u8>,
    MYET: f64,
    B: i32,
    E: i32,
    START: i32,
    NTOKNS: i32,
    IDENT: StackArray<i32, 100>,
    BEG: StackArray<i32, 100>,
    END: StackArray<i32, 100>,
    FMT: Vec<u8>,
    WIDTH: i32,
    APPND: i32,
    TIMTYP: i32,
    YMDFMT: Vec<u8>,
    YWFMT: Vec<u8>,
    BASTYP: Vec<u8>,
    INTYP: Vec<u8>,
    TSYS: Vec<u8>,
    CAL: Vec<u8>,
    ZON: Vec<u8>,
    HOFF: f64,
    MOFF: f64,
    CALTYP: i32,
    LAST: i32,
    OK: bool,
    DOERA: bool,
    DOZONE: bool,
    I: i32,
    STOPAT: i32,
    TRNCAT: i32,
    DELTA: f64,
    FACTOR: f64,
    FRAC: f64,
    INCR: f64,
    INTMED: f64,
    NTVEC: StackArray<f64, 8>,
    PTVEC: StackArray<f64, 8>,
    TEMPD: f64,
    TIMPAD: f64,
    TVEC: StackArray<f64, 8>,
    VALUE: f64,
    POWER: StackArray<f64, 15>,
    X: f64,
    JYEAR: i32,
    JMONTH: i32,
    JDAY: i32,
    JDOY: i32,
    GYEAR: i32,
    GMONTH: i32,
    GDAY: i32,
    GDOY: i32,
    GO2JUL: bool,
    MONTHS: ActualCharArray,
    MYMON: Vec<u8>,
    MYLEN: i32,
    MONTYP: i32,
    MLEN: StackArray<i32, 12>,
    WKDAYS: ActualCharArray,
    MYWKD: Vec<u8>,
    WKLEN: StackArray<i32, 7>,
    WKTYP: i32,
    INDX: i32,
    UNKNWN: bool,
    MAKING: bool,
    PUMPUP: bool,
    VANISH: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIRST: bool = false;
        let mut MARKS = ActualCharArray::new(MRKSIZ, 1..=MAXMRK);
        let mut CLASS = StackArray::<i32, 44>::new(0..=MAXMRK);
        let mut NMARKS: i32 = 0;
        let mut MRKLEN = StackArray::<i32, 43>::new(1..=MAXMRK);
        let mut PNTRS = StackArray::<i32, 100>::new(1..=100);
        let mut DUMP = StackArray::<i32, 11>::new(1..=11);
        let mut NDUMP: i32 = 0;
        let mut TIMFMT: i32 = 0;
        let mut TYPE: i32 = 0;
        let mut NUMTYP: i32 = 0;
        let mut ORIGNL = ActualCharArray::new(32, BEGIN..=FINISH);
        let mut VALUES = StackArray::<f64, 52>::new(BEGIN..=FINISH);
        let mut PAD = StackArray::<f64, 52>::new(BEGIN..=FINISH);
        let mut LENGTH = StackArray::<i32, 52>::new(BEGIN..=FINISH);
        let mut ID = StackArray::<i32, 52>::new(BEGIN..=FINISH);
        let mut HAVE = StackArray::<bool, 52>::new(BEGIN..=FINISH);
        let mut COMPNT = StackArray2D::<i32, 16>::new(1..=8, 1..=CONTIN);
        let mut MYSTR = vec![b' '; MAXLEN as usize];
        let mut SUBSTR = vec![b' '; MAXLEN as usize];
        let mut STRING = vec![b' '; MAXLEN as usize];
        let mut MYET: f64 = 0.0;
        let mut B: i32 = 0;
        let mut E: i32 = 0;
        let mut START: i32 = 0;
        let mut NTOKNS: i32 = 0;
        let mut IDENT = StackArray::<i32, 100>::new(1..=ROOM);
        let mut BEG = StackArray::<i32, 100>::new(1..=ROOM);
        let mut END = StackArray::<i32, 100>::new(1..=ROOM);
        let mut FMT = vec![b' '; LOCLEN as usize];
        let mut WIDTH: i32 = 0;
        let mut APPND: i32 = 0;
        let mut TIMTYP: i32 = 0;
        let mut YMDFMT = vec![b' '; 8 as usize];
        let mut YWFMT = vec![b' '; 8 as usize];
        let mut BASTYP = vec![b' '; 16 as usize];
        let mut INTYP = vec![b' '; 16 as usize];
        let mut TSYS = vec![b' '; 16 as usize];
        let mut CAL = vec![b' '; 16 as usize];
        let mut ZON = vec![b' '; 32 as usize];
        let mut HOFF: f64 = 0.0;
        let mut MOFF: f64 = 0.0;
        let mut CALTYP: i32 = 0;
        let mut LAST: i32 = 0;
        let mut OK: bool = false;
        let mut DOERA: bool = false;
        let mut DOZONE: bool = false;
        let mut I: i32 = 0;
        let mut STOPAT: i32 = 0;
        let mut TRNCAT: i32 = 0;
        let mut DELTA: f64 = 0.0;
        let mut FACTOR: f64 = 0.0;
        let mut FRAC: f64 = 0.0;
        let mut INCR: f64 = 0.0;
        let mut INTMED: f64 = 0.0;
        let mut NTVEC = StackArray::<f64, 8>::new(1..=8);
        let mut PTVEC = StackArray::<f64, 8>::new(1..=8);
        let mut TEMPD: f64 = 0.0;
        let mut TIMPAD: f64 = 0.0;
        let mut TVEC = StackArray::<f64, 8>::new(1..=8);
        let mut VALUE: f64 = 0.0;
        let mut POWER = StackArray::<f64, 15>::new(0..=14);
        let mut X: f64 = 0.0;
        let mut JYEAR: i32 = 0;
        let mut JMONTH: i32 = 0;
        let mut JDAY: i32 = 0;
        let mut JDOY: i32 = 0;
        let mut GYEAR: i32 = 0;
        let mut GMONTH: i32 = 0;
        let mut GDAY: i32 = 0;
        let mut GDOY: i32 = 0;
        let mut GO2JUL: bool = false;
        let mut MONTHS = ActualCharArray::new(9, 1..=12);
        let mut MYMON = vec![b' '; 9 as usize];
        let mut MYLEN: i32 = 0;
        let mut MONTYP: i32 = 0;
        let mut MLEN = StackArray::<i32, 12>::new(1..=12);
        let mut WKDAYS = ActualCharArray::new(9, 1..=7);
        let mut MYWKD = vec![b' '; 9 as usize];
        let mut WKLEN = StackArray::<i32, 7>::new(1..=7);
        let mut WKTYP: i32 = 0;
        let mut INDX: i32 = 0;
        let mut UNKNWN: bool = false;
        let mut MAKING: bool = false;
        let mut PUMPUP: bool = false;
        let mut VANISH: bool = false;

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
            MONTHS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(7),
                Val::I(8),
                Val::I(5),
                Val::I(5),
                Val::I(3),
                Val::I(4),
                Val::I(4),
                Val::I(6),
                Val::I(9),
                Val::I(7),
                Val::I(8),
                Val::I(8),
            ]
            .into_iter();
            MLEN.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"Sunday"),
                Val::C(b"Monday"),
                Val::C(b"Tuesday"),
                Val::C(b"Wednesday"),
                Val::C(b"Thursday"),
                Val::C(b"Friday"),
                Val::C(b"Saturday"),
            ]
            .into_iter();
            WKDAYS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(6),
                Val::I(6),
                Val::I(7),
                Val::I(9),
                Val::I(8),
                Val::I(6),
                Val::I(8),
            ]
            .into_iter();
            WKLEN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(10.0),
                Val::D(100.0),
                Val::D(1000.0),
                Val::D(10000.0),
                Val::D(100000.0),
                Val::D(1000000.0),
                Val::D(10000000.0),
                Val::D(100000000.0),
                Val::D(1000000000.0),
                Val::D(10000000000.0),
                Val::D(100000000000.0),
                Val::D(1000000000000.0),
                Val::D(10000000000000.0),
                Val::D(100000000000000.0),
            ]
            .into_iter();
            POWER
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            FIRST,
            MARKS,
            CLASS,
            NMARKS,
            MRKLEN,
            PNTRS,
            DUMP,
            NDUMP,
            TIMFMT,
            TYPE,
            NUMTYP,
            ORIGNL,
            VALUES,
            PAD,
            LENGTH,
            ID,
            HAVE,
            COMPNT,
            MYSTR,
            SUBSTR,
            STRING,
            MYET,
            B,
            E,
            START,
            NTOKNS,
            IDENT,
            BEG,
            END,
            FMT,
            WIDTH,
            APPND,
            TIMTYP,
            YMDFMT,
            YWFMT,
            BASTYP,
            INTYP,
            TSYS,
            CAL,
            ZON,
            HOFF,
            MOFF,
            CALTYP,
            LAST,
            OK,
            DOERA,
            DOZONE,
            I,
            STOPAT,
            TRNCAT,
            DELTA,
            FACTOR,
            FRAC,
            INCR,
            INTMED,
            NTVEC,
            PTVEC,
            TEMPD,
            TIMPAD,
            TVEC,
            VALUE,
            POWER,
            X,
            JYEAR,
            JMONTH,
            JDAY,
            JDOY,
            GYEAR,
            GMONTH,
            GDAY,
            GDOY,
            GO2JUL,
            MONTHS,
            MYMON,
            MYLEN,
            MONTYP,
            MLEN,
            WKDAYS,
            MYWKD,
            WKLEN,
            WKTYP,
            INDX,
            UNKNWN,
            MAKING,
            PUMPUP,
            VANISH,
        }
    }
}

/// Time Output
///
/// Convert an input epoch represented in TDB seconds past the TDB
/// epoch of J2000 to a character string formatted to the
/// specifications of a user's format picture.
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
///  ET         I   Epoch in seconds past the ephemeris epoch J2000.
///  PICTUR     I   A format specification for the output string.
///  OUTPUT     O   A string representation of the input epoch.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ET       is a double precision representation of time in seconds
///           past the ephemeris epoch J2000.
///
///  PICTUR   is a string that specifies how the output should be
///           presented. The string is made up of various markers
///           that stand for various components associated with
///           a time.
///
///           There are five types of markers that may appear in a
///           format picture. These are String Markers, Numeric
///           Markers, Meta markers, Modifier Markers and Literal
///           Markers.
///
///           The PICTUR string is examined and the various markers
///           are identified. The output time string is constructed
///           by replacing each of the identified markers with
///           an appropriate time component.
///
///           The various markers and their meanings are discussed
///           in the $Particulars section below.
///
///           Note that leading and trailing blanks in PICTUR are
///           ignored.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUTPUT   is a time string equivalent to the input epoch ET,
///           matching the format specified by PICTUR.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  A leapseconds kernel must be loaded via the routine FURNSH
///      before calling this routine. If a leapsecond kernel has not
///      been loaded, an error is signaled by a routine in the call
///      tree of this routine.
///
///  2)  If PICTUR contains the numeric marker 'YYYY' and the
///      magnitude of year is too large to be displayed as a four-digit
///      integer, TIMOUT will replace it by '****'.
///
///  3)  If the requested precision is higher than 12 decimal places,
///      TIMOUT will truncate the decimal part down to 12, and OUTPUT
///      will have all the remaining digits in the decimal part set to
///      zero.
///
///  4)  Double colon (::), when is not part of one of the supported
///      markers, has no effect and will be presented as is on the
///      output string.
/// ```
///
/// # Files
///
/// ```text
///  A leapseconds kernel must be "loaded" via the routine FURNSH
///  prior to calling TIMOUT.
/// ```
///
/// # Particulars
///
/// ```text
///  A format picture is simply a string of letters that lets
///  TIMOUT know where various components of a time representation
///  should be placed during creation of the time string.
///  Here's an example of such a picture:
///
///     MON DD,YYYY  HR:MN:SC.#### (TDB) ::TDB
///
///  Here is a sample of times that would be created by using this
///  format.
///
///     JAN 12,1992  12:28:18.2772 (TDB)
///     FEB 13,1994  23:18:25.2882 (TDB)
///     AUG 21,1995  00:02:00.1881 (TDB)
///
///  As you can see from the samples above, the format picture
///  specifies that every time string created should begin with a
///  three-letter abbreviation for the month, followed by a space and
///  the day of the month. The day of month is followed immediately by
///  a comma and the year. The year component is followed by two
///  spaces. The next outputs are hours represented as a two digit
///  integer, a colon, minutes represented as a two digit integer,
///  another colon, and seconds truncated to 4 decimal places and
///  having a two digit integer part (rounding can be commanded; see
///  the discussion of truncation and rounding below). This is
///  followed by a space and the string '(TDB)'. The special marker
///  '::TDB' in the time picture is an ``invisible'' marker. It is
///  used to specify the time system that should be used in creating
///  the time string (in this case Barycentric Dynamical Time).
///
///  TIMOUT does not recognize all of the parts of the time format
///  picture in the example above. The list of recognized parts and
///  unrecognized parts is shown in the table below.
///
///    Recognized       Unrecognized
///    ----------       ------------
///    'MON'            ' '
///    'DD'             ','
///    'YYYY'           '  '
///    'HR'             ':'
///    'MN'             '(TDB)'
///    'SC'
///    '.####'
///    '::TDB'
///
///  The unrecognized parts are called literal markers. They are
///  copied exactly as they appear in PICTUR into the output string.
///  The recognized parts of the picture are replaced by a
///  component of time or, as in the case of '::TDB' are used
///  as instructions about the overall properties of the time
///  string.
///
///  The full list of recognized markers, their classification
///  and meaning are given below.
///
///  MARKER       CLASS     MEANING
///  -----------  --------  -----------------------------------------
///  '.##...'     modifier  represents a numeric component that
///                         immediately precedes this in a decimal
///                         format. Number of decimal places
///                         equals the number of '#' characters
///  '::GCAL'     meta      dates are reported in Gregorian calendar
///  '::JCAL'     meta      dates are reported in Julian calendar
///  '::MCAL'     meta      dates after 15 October, 1582 are reported
///                         in Gregorian calendar; before that
///                         dates are reported in Julian calendar
///
///  '::RND'      meta      round output to places specified by
///                         least significant component
///
///  '::TDB'      meta      all components should be TDB
///
///  '::TDT'      meta      all components should be TT (TDT)
///
///  '::TT'       meta      all components should be TT (TDT)
///
///  '::TRNC'     meta      truncate all output components (default)
///  '::UTC'      meta      all components should be UTC (default)
///  '::UTC+h:m'  meta      all components in UTC offset by +h (hours)
///                         and +m (minutes) so as to allow time zones.
///  '::UTC-h:m'  meta      all components in UTC offset by -h (hours)
///                         and -m (minutes) so as to allow time zones.
///  'AMPM'       string    String (either 'A.M.' or 'P.M.')
///                         indicating whether hours are before
///                         or after noon.
///  'ampm'       string    String (either 'a.m.' or 'p.m.')
///                         indicating whether hours are before
///                         or after noon.
///  'AP'         numeric   AM/PM equivalents of the hour component
///                         of a time.
///  'DD'         numeric   Day of month
///  'DOY'        numeric   Day of year
///  'ERA'        string    String (either 'B.C.' or 'A.D.') giving
///                         era associated with an epoch.
///  '?ERA?'      string    String: either ' B.C. ' or ' A.D. ' if the
///                         year is before 1000 A.D. otherwise a
///                         blank: ' '.
///  'era'        string    String (either 'b.c.' or 'a.d.') giving
///                         era associated with an epoch.
///  '?era?'      string    String: either ' b.c. ' or ' a.d. ' if the
///                         year is before 1000 A.D. otherwise a
///                         blank: ' '.
///  'HR'         numeric   hour component of time
///  'JULIAND'    numeric   Julian date component of time
///  'MM'         numeric   numeric representation of month component
///  'MN'         numeric   minute component of time
///  'MON'        string    upper case three letter abbreviation for
///                         month
///  'Mon'        string    capitalized three letter abbreviation for
///                         month
///  'mon'        string    lower case three letter abbreviation for
///                         month
///  'MONTH'      string    upper case full name of month
///  'Month'      string    capitalized full name of month
///  'month'      string    lower case full name of month
///  'SC'         numeric   seconds component of time
///  'SP1950'     numeric   seconds past 1950 component of time
///  'SP2000'     numeric   seconds past 2000 component of time
///  'YR'         numeric   last two digits of year component of time
///  'YYYY'       numeric   year component of time
///  'WEEKDAY'    string    upper case day of week
///  'Weekday'    string    capitalized day of week
///  'weekday'    string    lower case day of week
///  'WKD'        string    upper case three letter abbreviation for
///                         day of week.
///  'Wkd'        string    capitalized three letter abbreviation for
///                         day of week.
///  'wkd'        string    lower case three letter abbreviation for
///                         day of week.
///
///  String Markers
///
///     String markers are portions of the format picture that will
///     be replaced with a character string that represents the
///     corresponding component of a time.
///
///  Numeric Markers
///
///     Numeric markers are portions of the format picture that will
///     be replaced with a decimal string that represents the
///     corresponding component of a time.
///
///  Meta Markers
///
///     Meta markers (listed under the class ``meta'' in the
///     table above) are used to indicate "global" properties of
///     your time string. You may specify time scale and how
///     rounding should be performed on the components of time
///     in your output string. Meta markers may be placed anywhere
///     in your format picture. They do not contribute to placement
///     of characters in output time strings. Also there are no
///     restrictions on how many meta markers you may place in
///     the format picture. However, if you supply conflicting
///     `meta' markers (for example '::TDT' and '::TDB') in your
///     picture the first marker listed (in left to right order)
///     overrules the conflicting marker that appears later in
///     the picture.
///
///  Default Meta Markers
///
///     If you do not specify a time system, calendar, or time
///     zone through the use of a Meta Marker, TIMOUT uses the
///     values returned by the SPICE routine TIMDEF. The default
///     time system, calendar returned by TIMDEF are UTC and
///     the Gregorian calendar. The default time zone returned
///     by TIMDEF is a blank indicating that no time zone offset
///     should be used.
///
///     See the header for the routine TIMDEF for a more complete
///     discussion of setting and retrieving default values.
///
///  Modifier Markers
///
///     The numeric markers listed in the table above stand
///     for integers unless they are modified through use of a
///     modifier marker. The strings
///
///        .#
///        .##
///        .###
///        .####
///
///     are used to this end. When a numeric marker is followed
///     immediately by one of these modifiers, the corresponding time
///     component will be written with the number of decimal places
///     indicated by the number of successive occurrences of the
///     character '#'. Any numeric token may be modified.
///
///  Rounding vs. Truncation
///
///     The meta markers ::TRNC and ::RND allow you to control
///     how the output time picture is rounded. If you specify
///     ::TRNC all components of time are simply truncated to
///     the precision specified by the marker and any modifier.
///     If you specify ::RND the output time is rounded to the
///     least significant component of the format picture. The
///     default action is truncation.
///
///     Whether an output time string should be rounded or
///     truncated depends upon what you plan to do with the
///     string. For example suppose you simply want to get the
///     calendar date associated with a time and not the time of
///     day. Then you probably do not want to round your output.
///     Rounding 1992 Dec 31, 13:12:00 to the nearest day
///     produces 1993 Jan 1. Thus in this case rounding is probably
///     not appropriate.
///
///     However, if you are producing output for plotting using
///     Julian Date, seconds past 1950 or seconds past 2000, you will
///     probably want your output rounded so as to produce a smoother
///     plot.
///
///  Time Systems
///
///     TIMOUT can produce output strings for epochs relative to
///     any of the systems UTC, TT or TDT, or TDB. If you do not
///     explicitly specify a time system, TIMOUT will produce strings
///     relative to the time system returned by the SPICE routine
///     TIMDEF. Unless you call TIMDEF and change it, the default time
///     system is UTC. However, by using one of the Meta Markers
///     ::UTC, ::TT, ::TDT, or ::TDB you may specify that TIMOUT
///     produce time strings relative to the UTC, TT or TDT, or TDB
///     system respectively.
///
///  Time Zones
///
///     The meta markers ::UTC+h:m  and ::UTC-h:m  allow you to
///     offset UTC times so that you may represent times in a time
///     zone other than GMT. For example you can output times in
///     Pacific Standard time by placing the meta-marker ::UTC-8 in
///     your format picture.
///
///     For instance, if you use the picture
///
///        YYYY Mon DD, HR:MN:SC ::UTC
///
///     you will get output strings such as:
///
///        1995 Jan 03, 12:00:00
///
///     If you use the picture
///
///
///        YYYY Mon DD, HR:MN:SC ::UTC-8
///
///     you will get output strings such as:
///
///        1995 Jan 03, 04:00:00
///
///     Finally, if you use the picture
///
///        YYYY Mon DD, HR:MN:SC ::UTC-8:15
///
///     you will get output string
///
///        1995 Jan 03, 03:45:00
///
///     Note that the minutes are always added or subtracted based on
///     the sign present in the time zone specifier. In the case of
///     ::UTC+h:m, minutes are added. In the case ::UTC-h:m, minutes
///     are subtracted.
///
///     The unsigned part of the hours component can be no more than
///     12. The unsigned part of the minutes component can be no
///     more than 59.
///
///  Calendars
///
///     The calendar currently used by western countries is the
///     Gregorian calendar. This calendar begins on Oct 15, 1582.
///     Prior to Gregorian calendar the Julian calendar was used. The
///     last Julian calendar date prior to the beginning of the
///     Gregorian calendar is Oct 5, 1582.
///
///     The primary difference between the Julian and Gregorian
///     calendars is in the determination of leap years. Nevertheless,
///     both can be formally extended backward and forward in time
///     indefinitely.
///
///     By default TIMOUT uses the default calendar returned by
///     TIMDEF. Under most circumstances this will be the Gregorian
///     calendar (::GCAL). However you may specify that TIMOUT use a
///     specific calendar through use of one of the calendar Meta
///     Markers. You may specify that TIMOUT use the Julian calendar
///     (::JCAL), the Gregorian calendar (::GCAL)  or a mixture of
///     both (::MCAL).
///
///     If you specify ::MCAL, epochs that occur after the beginning
///     of the Gregorian calendar will be represented using the
///     Gregorian calendar, and epochs prior to the beginning of the
///     Gregorian calendar will be represented using the Julian
///     calendar.
///
///  Getting Software to Construct Pictures for You.
///
///     Although it is not difficult to construct time format
///     pictures, you do need to be aware of the various markers that
///     may appear in a format picture.
///
///     There is an alternative means for getting a format picture.
///     The routine TPICTR constructs format pictures from a sample
///     time string. For example, suppose you would like your time
///     strings to look like the basic pattern of the string below.
///
///        'Fri Jul 26 12:22:09 PDT 1996'
///
///     You can call TPICTR with this string, and it will create the
///     appropriate PICTUR for use with TIMOUT.
///
///        CALL TPICTR ( 'Fri Jul 26 12:22:09 PDT 1996',
///       .              PICTUR, OK, ERRMSG             )
///
///     The result will be:
///
///        'Wkd Mon DD HR:MN:SC (PDT) ::UTC-7'
///
///     Note: not every date that you can read is interpretable by
///     TPICTR. For example, you might be able to understand that
///     19960212121116 is Feb 2 1996, 12:11:16. However, TPICTR
///     cannot recognize this string. Thus it is important to check
///     the logical output OK to make sure that TPICTR was able to
///     understand the time picture you provided.
///
///     Even thought TPICTR can not recognize every time pattern that
///     has been used by various people, it does recognize nearly all
///     patterns that you use when you want to communicate outside
///     your particular circle of colleagues.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///  1) Given a sample with the format of the UNIX date string
///     local to California, create a SPICE time picture for use
///     in TIMOUT.
///
///     Using that SPICE time picture, convert a series of ephemeris
///     times to that picture format.
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
///           PROGRAM TIMOUT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN  = 400 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN  = 64  )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(ERRLEN)    ERR
///           CHARACTER*(TIMLEN)    PICTUR
///           CHARACTER*(TIMLEN)    SAMPLE
///           CHARACTER*(TIMLEN)    TIMSTR
///           CHARACTER*(TIMLEN)    UTCSTR
///
///           DOUBLE PRECISION      ET
///
///           LOGICAL               OK
///
///     C
///     C     Load LSK file.
///     C
///           CALL FURNSH ( 'naif0012.tls' )
///
///     C
///     C     Create the required time picture.
///     C
///           SAMPLE = 'Thu Oct 01 11:11:11 PDT 1111'
///
///           CALL TPICTR ( SAMPLE, PICTUR, OK, ERR )
///
///           IF ( .NOT. OK ) THEN
///
///              WRITE(*,*) 'Invalid time picture.'
///              WRITE(*,*) ERR
///
///           ELSE
///
///     C
///     C        Convert the input UTC time to ephemeris time.
///     C
///              UTCSTR = '26 Nov 2018  23:23:00 UTC'
///              CALL STR2ET ( UTCSTR, ET )
///
///     C
///     C         Now convert ET to the desired output format.
///     C
///               CALL TIMOUT ( ET, PICTUR, TIMSTR )
///               WRITE (*,*) 'Sample format: ', SAMPLE
///               WRITE (*,*) 'Time picture : ', PICTUR
///               WRITE (*,*)
///               WRITE (*,*) 'Input UTC    : ', UTCSTR
///               WRITE (*,*) 'Output       : ', TIMSTR
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
///      Sample format: Thu Oct 01 11:11:11 PDT 1111
///      Time picture : Wkd Mon DD HR:MN:SC PDT YYYY ::UTC-7
///
///      Input UTC    : 26 Nov 2018  23:23:00 UTC
///      Output       : Mon Nov 26 16:23:00 PDT 2018
///
///
///  2) Convert a UTC time to a string that contains both the
///     calendar representations of the date as well as the Julian
///     date; for example a string of the form:
///
///        "Thu Aug 01 09:47:16 PDT 1996 (2450297.1994 JDUTC)"
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
///           PROGRAM TIMOUT_EX2
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN  = 80 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(TIMLEN)    PICTUR
///           CHARACTER*(TIMLEN)    TIMSTR
///           CHARACTER*(TIMLEN)    UTCSTR
///
///           DOUBLE PRECISION      ET
///
///     C
///     C     Load LSK file.
///     C
///           CALL FURNSH ( 'naif0012.tls' )
///
///     C
///     C     Convert the input UTC time to ephemeris time.
///     C
///           UTCSTR = '26 Nov 2018  16:23:00 UTC'
///           CALL STR2ET ( UTCSTR, ET )
///
///     C
///     C     Create the required time picture. This could be done
///     C     using TPICTR.
///     C
///           PICTUR = 'Wkd Mon DD HR:MN ::UTC-7 YYYY '
///          .      // '(JULIAND.#### JDUTC)'
///
///     C
///     C      Now convert ET to the desired output format.
///     C
///            CALL TIMOUT ( ET, PICTUR, TIMSTR )
///            WRITE (*,*) 'Input UTC: ', UTCSTR
///            WRITE (*,*) 'Output   : ', TIMSTR
///
///            END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Input UTC: 26 Nov 2018  16:23:00 UTC
///      Output   : Mon Nov 26 09:23  2018 (2458449.1826 JDUTC)
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  M.J. Spencer       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.4.0, 07-AUG-2021 (EDW) (JDR) (NJB)
///
///         Corrected typo preventing correct calculation of decimal
///         values for HR.###... and MN.###... markers with ::UTC+N:M
///         and ::UTC-N:M meta tags.
///
///         Added "::TT" as a time system meta marker equivalent-to/
///         alias-for "::TDT". No change to functionality.
///
///         Corrected OUTPUT argument name in $Brief_I/O section (it was
///         STRING) and improved its description in $Detailed_Output.
///         Fixed call to TPICTR in $Particulars.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary entries in $Revisions section. Converted the
///         existing code fragments into complete examples and added
///         references to required LSKs.
///
///         Updated $Exceptions section, rewording the existing entry
///         and adding three additional cases.
///
/// -    SPICELIB Version 3.3.1, 31-JAN-2017 (NJB)
///
///         Updated header comments to draw attention to the fact that
///         rounding can be commanded.
///
/// -    SPICELIB Version 3.3.0, 23-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in RMAIND call. Replaced header references to LDPOOL with
///         references to FURNSH.
///
/// -    SPICELIB Version 3.2.0, 09-APR-2002 (WLT)
///
///         Added code to bracket the fractional part of a time component
///         so that it cannot become negative due to inability to invert
///         arithmetic operations with double precision arithmetic.
///
/// -    SPICELIB Version 3.1.0, 21-JUN-2001 (WLT)
///
///         Added the format picture components ?ERA? and ?era? which
///         vanish for years after 999 A.D.
///
/// -    SPICELIB Version 3.0.2, 10-APR-2000 (WLT)
///
///         Declared SCAN to be external.
///
/// -    SPICELIB Version 3.0.1, 22-JUN-1998 (WLT)
///
///         A number of typographical and grammatical errors
///         were corrected in the header.
///
/// -    SPICELIB Version 3.0.0, 30-DEC-1997 (WLT)
///
///         The previous version of this routine did not output
///         fractional components for epochs prior to 1 A.D.
///
///         In addition, the default time system, calendar and time zone
///         are obtained from TIMDEF.
///
/// -    SPICELIB Version 2.0.0, 01-APR-1997 (WLT)
///
///         In the event that the format picture requested 'YR' as
///         the first component of a time string, the previous edition
///         of this routine used the year value corresponding to the
///         last call to this routine (or whatever happened to be in
///         memory on the first call). This error has been corrected.
///
/// -    SPICELIB Version 1.0.0, 26-JUL-1996 (WLT) (MJS) (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 3.0.0, 30-DEC-1997 (WLT)
///
///         The previous version of this routine did not output
///         fractional components for epochs prior to 1 A.D.
///
///         This error was due to overuse of the original year
///         component returned from TTRANS. The original year
///         component is now saved for use in computing the fractional
///         component. The modified year (used in printing B.C. epochs)
///         is stored in a separate variable.
///
/// -    SPICELIB Version 2.0.0, 01-APR-1997 (WLT)
///
///         In the event that the format picture requested 'YR' as
///         the first component of a time string, the previous edition
///         of this routine used the year value corresponding to the
///         last call to this routine (or whatever happened to be in
///         memory on the first call). This error has been corrected.
///
///
///         The error was fixed by recoding the following IF THEN statement
///
///               IF (       TYPE .EQ. YEAR
///     .               .OR. TYPE .EQ. MONTH
///     .               .OR. TYPE .EQ. MON
///     .               .OR. TYPE .EQ. DAY
///     .               .OR. TYPE .EQ. DOY
///     .               .OR. TYPE .EQ. NOON
///     .               .OR. TYPE .EQ. HOUR
///     .               .OR. TYPE .EQ. ERA
///     .               .OR. TYPE .EQ. AMPM
///     .               .OR. TYPE .EQ. MINUTE
///     .               .OR. TYPE .EQ. SEC   ) THEN
///
///         as
///
///               IF (       TYPE .EQ. YEAR
///     .               .OR. TYPE .EQ. YR
///     .               .OR. TYPE .EQ. MONTH
///     .               .OR. TYPE .EQ. MON
///     .               .OR. TYPE .EQ. DAY
///     .               .OR. TYPE .EQ. DOY
///     .               .OR. TYPE .EQ. NOON
///     .               .OR. TYPE .EQ. HOUR
///     .               .OR. TYPE .EQ. ERA
///     .               .OR. TYPE .EQ. AMPM
///     .               .OR. TYPE .EQ. MINUTE
///     .               .OR. TYPE .EQ. SEC   ) THEN
///
///
/// -    Beta Version 2.1.0, 17-MAR-1994 (MJS) (NJB)
///
///         Integer argument to BRCKTD changed from 0 to 0.0D0.
/// ```
pub fn timout(
    ctx: &mut SpiceContext,
    et: f64,
    pictur: &str,
    output: &mut str,
) -> crate::Result<()> {
    TIMOUT(
        et,
        pictur.as_bytes(),
        fstr::StrBytes::new(output).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TIMOUT ( Time Output )
pub fn TIMOUT(
    ET: f64,
    PICTUR: &[u8],
    OUTPUT: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // The parameters below act essentially as an enumeration
    // of the various kinds of components we will be looking at in the
    // input time string.
    //

    //
    // The following parameters serve as an enumeration of the various
    // time formats that are recognized.
    //

    //
    // The parameters below are used to declare the space needed for
    // scanning the input format string.
    //

    //
    // The length of the local string that we will use for copying
    // the format string.
    //

    //
    // Local variables
    //

    //
    // The next set of variables holds the marks and auxiliary
    // arrays used for scanning the format string.
    //

    //
    // The variables below are used to hold, base formats, values of
    // time vector components, adjustments to use when rounding,
    // the lengths of the format pictures and whether or not various
    // components have already been computed.
    //

    //
    // The array below contains the indexes of the various values
    // associated with the three different times of time vectors
    // that we will be using YMD, YD, CONTIN.
    //

    //
    // We will be making a local copy of the input format string
    // and the input time.
    //

    //
    // The integers below are used to mark substring boundaries.
    //

    //
    // Times come in three flavors: TT or TDT, TDB, UTC.  The one used
    // on this particular invocation of TIMOUT is stored in TIMTYP.
    // The routine TTRANS needs to have input and output time vector
    // types.  The one used based upon the input PICTUR is stored
    // in BASTYP.
    //

    //
    // Loop counters and delimiters
    //

    //
    // Utility double precision numbers
    //

    //
    // The array power is used to assist in the truncation of double
    // precision values.
    //

    //
    // calendar variables.
    //

    //
    // Character string representations for months and week days.
    //

    //
    // Save everything.
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"TIMOUT", ctx)?;

    //
    // Chapter 1. Initializations.
    // =================================================================
    //
    // On the first pass, we need to set up the recognized tokens
    // that will be used for scanning, the classes of these tokens
    // and the array of ID's for time systems.
    //
    if save.FIRST {
        save.FIRST = false;

        fstr::assign(save.MARKS.get_mut(1), b"YYYY");
        fstr::assign(save.MARKS.get_mut(2), b"YR");
        fstr::assign(save.MARKS.get_mut(3), b"MON");
        fstr::assign(save.MARKS.get_mut(4), b"Mon");
        fstr::assign(save.MARKS.get_mut(5), b"mon");
        fstr::assign(save.MARKS.get_mut(6), b"MONTH");
        fstr::assign(save.MARKS.get_mut(7), b"Month");
        fstr::assign(save.MARKS.get_mut(8), b"month");
        fstr::assign(save.MARKS.get_mut(9), b"MM");
        fstr::assign(save.MARKS.get_mut(10), b"DOY");
        fstr::assign(save.MARKS.get_mut(11), b"WKD");
        fstr::assign(save.MARKS.get_mut(12), b"Wkd");
        fstr::assign(save.MARKS.get_mut(13), b"wkd");
        fstr::assign(save.MARKS.get_mut(14), b"WEEKDAY");
        fstr::assign(save.MARKS.get_mut(15), b"Weekday");
        fstr::assign(save.MARKS.get_mut(16), b"weekday");
        fstr::assign(save.MARKS.get_mut(17), b"DD");
        fstr::assign(save.MARKS.get_mut(18), b"MN");
        fstr::assign(save.MARKS.get_mut(19), b"HR");
        fstr::assign(save.MARKS.get_mut(20), b"SC");
        fstr::assign(save.MARKS.get_mut(21), b".#");
        fstr::assign(save.MARKS.get_mut(22), b"#");
        fstr::assign(save.MARKS.get_mut(23), b"JULIAND");
        fstr::assign(save.MARKS.get_mut(24), b"::UTC");
        fstr::assign(save.MARKS.get_mut(25), b"::TDB");
        fstr::assign(save.MARKS.get_mut(26), b"::TDT");
        fstr::assign(save.MARKS.get_mut(27), b"::TT");
        fstr::assign(save.MARKS.get_mut(28), b"SP2000");
        fstr::assign(save.MARKS.get_mut(29), b"SP1950");
        fstr::assign(save.MARKS.get_mut(30), b"::RND");
        fstr::assign(save.MARKS.get_mut(31), b"::TRNC");
        fstr::assign(save.MARKS.get_mut(32), b"ERA");
        fstr::assign(save.MARKS.get_mut(33), b"era");
        fstr::assign(save.MARKS.get_mut(34), b"AMPM");
        fstr::assign(save.MARKS.get_mut(35), b"ampm");
        fstr::assign(save.MARKS.get_mut(36), b"::UTC+");
        fstr::assign(save.MARKS.get_mut(37), b"::UTC-");
        fstr::assign(save.MARKS.get_mut(38), b"::JCAL");
        fstr::assign(save.MARKS.get_mut(39), b"::GCAL");
        fstr::assign(save.MARKS.get_mut(40), b"::MCAL");
        fstr::assign(save.MARKS.get_mut(41), b"AP");
        fstr::assign(save.MARKS.get_mut(42), b"?ERA?");
        fstr::assign(save.MARKS.get_mut(43), b"?era?");

        save.NMARKS = 43;

        SCANPR(
            &mut save.NMARKS,
            save.MARKS.as_arg_mut(),
            save.MRKLEN.as_slice_mut(),
            save.PNTRS.as_slice_mut(),
        );

        //
        // Now that we've prepared our recognized substrings and
        // auxiliary arrays for scanning, collect the id's of the
        // various marks and classify the various marks.
        // substrings.
        //
        save.ID[NONAME] = 0;
        save.ID[YEAR] = BSRCHC(b"YYYY", save.NMARKS, save.MARKS.as_arg());
        save.ID[YR] = BSRCHC(b"YR", save.NMARKS, save.MARKS.as_arg());
        save.ID[UMON] = BSRCHC(b"MON", save.NMARKS, save.MARKS.as_arg());
        save.ID[MMON] = BSRCHC(b"Mon", save.NMARKS, save.MARKS.as_arg());
        save.ID[LMON] = BSRCHC(b"mon", save.NMARKS, save.MARKS.as_arg());
        save.ID[UMNTH] = BSRCHC(b"MONTH", save.NMARKS, save.MARKS.as_arg());
        save.ID[MMNTH] = BSRCHC(b"Month", save.NMARKS, save.MARKS.as_arg());
        save.ID[LMNTH] = BSRCHC(b"month", save.NMARKS, save.MARKS.as_arg());
        save.ID[MONTH] = BSRCHC(b"MM", save.NMARKS, save.MARKS.as_arg());
        save.ID[DOY] = BSRCHC(b"DOY", save.NMARKS, save.MARKS.as_arg());
        save.ID[UWKD] = BSRCHC(b"WKD", save.NMARKS, save.MARKS.as_arg());
        save.ID[MWKD] = BSRCHC(b"Wkd", save.NMARKS, save.MARKS.as_arg());
        save.ID[LWKD] = BSRCHC(b"wkd", save.NMARKS, save.MARKS.as_arg());
        save.ID[UWEEKD] = BSRCHC(b"WEEKDAY", save.NMARKS, save.MARKS.as_arg());
        save.ID[MWEEKD] = BSRCHC(b"Weekday", save.NMARKS, save.MARKS.as_arg());
        save.ID[LWEEKD] = BSRCHC(b"weekday", save.NMARKS, save.MARKS.as_arg());
        save.ID[DAY] = BSRCHC(b"DD", save.NMARKS, save.MARKS.as_arg());
        save.ID[MINUTE] = BSRCHC(b"MN", save.NMARKS, save.MARKS.as_arg());
        save.ID[HOUR] = BSRCHC(b"HR", save.NMARKS, save.MARKS.as_arg());
        save.ID[SEC] = BSRCHC(b"SC", save.NMARKS, save.MARKS.as_arg());
        save.ID[POINT] = BSRCHC(b".#", save.NMARKS, save.MARKS.as_arg());
        save.ID[PLACE] = BSRCHC(b"#", save.NMARKS, save.MARKS.as_arg());
        save.ID[JULIAN] = BSRCHC(b"JULIAND", save.NMARKS, save.MARKS.as_arg());
        save.ID[UTC] = BSRCHC(b"::UTC", save.NMARKS, save.MARKS.as_arg());
        save.ID[TDB] = BSRCHC(b"::TDB", save.NMARKS, save.MARKS.as_arg());
        save.ID[TDT] = BSRCHC(b"::TDT", save.NMARKS, save.MARKS.as_arg());
        save.ID[TT] = BSRCHC(b"::TT", save.NMARKS, save.MARKS.as_arg());
        save.ID[SP2000] = BSRCHC(b"SP2000", save.NMARKS, save.MARKS.as_arg());
        save.ID[SP1950] = BSRCHC(b"SP1950", save.NMARKS, save.MARKS.as_arg());
        save.ID[ROUND] = BSRCHC(b"::RND", save.NMARKS, save.MARKS.as_arg());
        save.ID[TRUNC] = BSRCHC(b"::TRNC", save.NMARKS, save.MARKS.as_arg());
        save.ID[UERA] = BSRCHC(b"ERA", save.NMARKS, save.MARKS.as_arg());
        save.ID[LERA] = BSRCHC(b"era", save.NMARKS, save.MARKS.as_arg());
        save.ID[UERAX] = BSRCHC(b"?ERA?", save.NMARKS, save.MARKS.as_arg());
        save.ID[LERAX] = BSRCHC(b"?era?", save.NMARKS, save.MARKS.as_arg());
        save.ID[UAMPM] = BSRCHC(b"AMPM", save.NMARKS, save.MARKS.as_arg());
        save.ID[LAMPM] = BSRCHC(b"ampm", save.NMARKS, save.MARKS.as_arg());
        save.ID[UTCP] = BSRCHC(b"::UTC+", save.NMARKS, save.MARKS.as_arg());
        save.ID[UTCM] = BSRCHC(b"::UTC-", save.NMARKS, save.MARKS.as_arg());
        save.ID[JCAL] = BSRCHC(b"::JCAL", save.NMARKS, save.MARKS.as_arg());
        save.ID[GCAL] = BSRCHC(b"::GCAL", save.NMARKS, save.MARKS.as_arg());
        save.ID[MCAL] = BSRCHC(b"::MCAL", save.NMARKS, save.MARKS.as_arg());
        save.ID[AMPM] = BSRCHC(b"AP", save.NMARKS, save.MARKS.as_arg());

        save.CLASS[save.ID[NONAME]] = NONAME;
        save.CLASS[save.ID[YEAR]] = YEAR;
        save.CLASS[save.ID[YR]] = YR;
        save.CLASS[save.ID[UMON]] = MON;
        save.CLASS[save.ID[MMON]] = MON;
        save.CLASS[save.ID[LMON]] = MON;
        save.CLASS[save.ID[UMNTH]] = MON;
        save.CLASS[save.ID[MMNTH]] = MON;
        save.CLASS[save.ID[LMNTH]] = MON;
        save.CLASS[save.ID[MONTH]] = MONTH;
        save.CLASS[save.ID[DOY]] = DOY;
        save.CLASS[save.ID[UWKD]] = WKDAY;
        save.CLASS[save.ID[MWKD]] = WKDAY;
        save.CLASS[save.ID[LWKD]] = WKDAY;
        save.CLASS[save.ID[UWEEKD]] = WKDAY;
        save.CLASS[save.ID[MWEEKD]] = WKDAY;
        save.CLASS[save.ID[LWEEKD]] = WKDAY;
        save.CLASS[save.ID[DAY]] = DAY;
        save.CLASS[save.ID[MINUTE]] = MINUTE;
        save.CLASS[save.ID[HOUR]] = HOUR;
        save.CLASS[save.ID[SEC]] = SEC;
        save.CLASS[save.ID[POINT]] = POINT;
        save.CLASS[save.ID[PLACE]] = PLACE;
        save.CLASS[save.ID[JULIAN]] = JULIAN;
        save.CLASS[save.ID[UTC]] = TIMSYS;
        save.CLASS[save.ID[TDB]] = TIMSYS;
        save.CLASS[save.ID[TDT]] = TIMSYS;
        save.CLASS[save.ID[TT]] = TIMSYS;
        save.CLASS[save.ID[SP2000]] = SP2000;
        save.CLASS[save.ID[SP1950]] = SP1950;
        save.CLASS[save.ID[ROUND]] = ROUND;
        save.CLASS[save.ID[TRUNC]] = TRUNC;
        save.CLASS[save.ID[UERA]] = ERA;
        save.CLASS[save.ID[LERA]] = ERA;
        save.CLASS[save.ID[UERAX]] = ERA;
        save.CLASS[save.ID[LERAX]] = ERA;
        save.CLASS[save.ID[UAMPM]] = NOON;
        save.CLASS[save.ID[LAMPM]] = NOON;
        save.CLASS[save.ID[UTCP]] = TIMSYS;
        save.CLASS[save.ID[UTCM]] = TIMSYS;
        save.CLASS[save.ID[JCAL]] = CALNDR;
        save.CLASS[save.ID[GCAL]] = CALNDR;
        save.CLASS[save.ID[MCAL]] = CALNDR;
        save.CLASS[save.ID[AMPM]] = AMPM;

        {
            let m1__: i32 = BEGIN;
            let m2__: i32 = FINISH;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.PAD[save.I] = 0.0;
                save.I += m3__;
            }
        }

        save.PAD[SEC] = 0.5;
        save.PAD[MINUTE] = (60.0 * save.PAD[SEC]);
        save.PAD[HOUR] = (60.0 * save.PAD[MINUTE]);
        save.PAD[DAY] = (24.0 * save.PAD[HOUR]);
        save.PAD[MONTH] = (30.0 * save.PAD[DAY]);
        save.PAD[DOY] = save.PAD[DAY];
        save.PAD[MON] = save.PAD[MONTH];
        save.PAD[YEAR] = (365.0 * save.PAD[DAY]);
        save.PAD[YR] = (365.0 * save.PAD[DAY]);
        save.PAD[JULIAN] = save.PAD[DAY];
        save.PAD[SP2000] = save.PAD[SEC];
        save.PAD[SP1950] = save.PAD[SEC];
        save.PAD[AMPM] = save.PAD[HOUR];

        //
        // After we've made the initial scan for tokens and determined
        // the time system requested, we will want to get rid of the
        // time system tokens.
        //
        save.DUMP[1] = save.ID[UTC];
        save.DUMP[2] = save.ID[TDT];
        save.DUMP[3] = save.ID[TT];
        save.DUMP[4] = save.ID[TDB];
        save.DUMP[5] = save.ID[ROUND];
        save.DUMP[6] = save.ID[TRUNC];
        save.DUMP[7] = save.ID[UTCM];
        save.DUMP[8] = save.ID[UTCP];
        save.DUMP[9] = save.ID[JCAL];
        save.DUMP[10] = save.ID[GCAL];
        save.DUMP[11] = save.ID[MCAL];

        save.NDUMP = 11;

        //
        // Set up the default formats for the various time components
        //
        fstr::assign(save.ORIGNL.get_mut(YEAR), b"YYYY");
        save.LENGTH[YEAR] = 4;

        fstr::assign(save.ORIGNL.get_mut(YR), b"0Y");
        save.LENGTH[YR] = 2;

        fstr::assign(save.ORIGNL.get_mut(DOY), b"0DD");
        save.LENGTH[DOY] = 3;

        fstr::assign(save.ORIGNL.get_mut(DAY), b"0D");
        save.LENGTH[DAY] = 2;

        fstr::assign(save.ORIGNL.get_mut(MONTH), b"0M");
        save.LENGTH[MONTH] = 2;

        fstr::assign(save.ORIGNL.get_mut(HOUR), b"0H");
        save.LENGTH[HOUR] = 2;

        fstr::assign(save.ORIGNL.get_mut(AMPM), b"0H");
        save.LENGTH[AMPM] = 2;

        fstr::assign(save.ORIGNL.get_mut(MINUTE), b"0M");
        save.LENGTH[MINUTE] = 2;

        fstr::assign(save.ORIGNL.get_mut(SEC), b"0S");
        save.LENGTH[SEC] = 2;

        fstr::assign(save.ORIGNL.get_mut(JULIAN), b"XXXXXXX");
        save.LENGTH[JULIAN] = 7;

        fstr::assign(save.ORIGNL.get_mut(SP2000), b"XXXXXXXXXXX");
        save.LENGTH[SP2000] = 11;

        fstr::assign(save.ORIGNL.get_mut(SP1950), b"XXXXXXXXXXX");
        save.LENGTH[SP1950] = 11;

        //
        // Finally set up the component pointers...
        //
        save.COMPNT[[1, YMD]] = RLYEAR;
        save.COMPNT[[2, YMD]] = MONTH;
        save.COMPNT[[3, YMD]] = DAY;
        save.COMPNT[[4, YMD]] = HOUR;
        save.COMPNT[[5, YMD]] = MINUTE;
        save.COMPNT[[6, YMD]] = SEC;

        save.COMPNT[[1, CONTIN]] = CURRNT;
    }

    //
    // Chapter 2.  Parsing the input picture.
    // ==============================================================
    //
    // First let's copy the input picture into local storage
    // (left justified) and get just past the end of the
    // significant portion (this way the loop that constructs the
    // output string will terminate with no unfinished business
    // left to resolve).
    //
    fstr::assign(&mut save.MYSTR, b" ");
    LJUST(PICTUR, fstr::substr_mut(&mut save.MYSTR, 1..=(MAXLEN - 1)));
    save.E = (RTRIM(&save.MYSTR) + 1);
    save.START = 1;

    //
    // Scan the input string.
    //
    SCAN(
        fstr::substr(&save.MYSTR, 1..=save.E),
        save.MARKS.as_arg(),
        save.MRKLEN.as_slice(),
        save.PNTRS.as_slice(),
        ROOM,
        &mut save.START,
        &mut save.NTOKNS,
        save.IDENT.as_slice_mut(),
        save.BEG.as_slice_mut(),
        save.END.as_slice_mut(),
    );

    //
    // Locate the time system that will be used.  This must
    // be one of the following: UTC, TDB, TT, TDT
    //
    save.UNKNWN = true;
    save.GO2JUL = false;
    save.DOZONE = false;
    save.I = 1;
    save.HOFF = 0.0;
    save.MOFF = 0.0;

    //
    // Get the default time type from TIMDEF
    //
    TIMDEF(b"GET", b"SYSTEM", &mut save.TSYS, ctx)?;

    if fstr::eq(&save.TSYS, b"UTC") {
        save.TIMTYP = save.ID[UTC];
    } else if fstr::eq(&save.TSYS, b"TDB") {
        save.TIMTYP = save.ID[TDB];
    } else if (fstr::eq(&save.TSYS, b"TDT") || fstr::eq(&save.TSYS, b"TT")) {
        save.TIMTYP = save.ID[TDT];
    } else {
        save.TIMTYP = save.ID[UTCP];

        TIMDEF(b"GET", b"ZONE", &mut save.ZON, ctx)?;

        PREFIX(b"::", 0, &mut save.ZON);
        ZZUTCPM(
            &save.ZON,
            1,
            &mut save.HOFF,
            &mut save.MOFF,
            &mut save.LAST,
            &mut save.OK,
            ctx,
        );

        save.DOZONE = save.OK;

        //
        // The routine TIMDEF uses ZZUTCPM to determine whether
        // or not a time zone is legitimate before it stores it
        // to be "GOTTEN."  As a result the value of OK should
        // always be .TRUE.  However, just in case TIMDEF should
        // someday use something other that ZZUTCPM for checking
        // we put in the unneeded check below.
        //
        if !save.OK {
            save.TIMTYP = save.ID[UTC];
        }
    }

    while (save.UNKNWN && (save.I <= save.NTOKNS)) {
        if (save.CLASS[save.IDENT[save.I]] == TIMSYS) {
            save.TIMTYP = save.IDENT[save.I];
            save.UNKNWN = false;
            save.DOZONE = false;

            if ((save.IDENT[save.I] == save.ID[UTCP]) || (save.IDENT[save.I] == save.ID[UTCM])) {
                //
                // We've got a time zone specification. Parse it and
                // store the offsets from UTC.
                //
                ZZUTCPM(
                    &save.MYSTR,
                    save.BEG[save.I],
                    &mut save.HOFF,
                    &mut save.MOFF,
                    &mut save.LAST,
                    &mut save.OK,
                    ctx,
                );

                if save.OK {
                    save.DOZONE = true;
                    save.TIMTYP = save.ID[UTCP];

                    //
                    // If we ran all the way up to the end of the next
                    // token, we simply reset the identity of the next
                    // token to be a zone type and increment  I.
                    //
                    // This way we never see the next token in this loop
                    // and it gets removed later when time systems and
                    // other meta markers from  our copy of the time
                    // format string.
                    //
                    if (save.LAST == save.END[(save.I + 1)]) {
                        save.IDENT[(save.I + 1)] = save.IDENT[save.I];
                        save.I = (save.I + 1);
                    } else {
                        save.END[save.I] = save.LAST;
                        save.BEG[(save.I + 1)] = (save.LAST + 1);
                    }
                }
            }
        }

        save.I = (save.I + 1);
    }

    //
    // Determine whether we should use the Julian or gregorian (default)
    // calendar
    //
    save.UNKNWN = true;
    save.I = 1;

    //
    // Get the default calendar from TIMDEF.
    //
    TIMDEF(b"GET", b"CALENDAR", &mut save.CAL, ctx)?;

    if fstr::eq(&save.CAL, b"GREGORIAN") {
        save.CALTYP = save.ID[GCAL];
    } else if fstr::eq(&save.CAL, b"JULIAN") {
        save.CALTYP = save.ID[JCAL];
    } else {
        save.CALTYP = save.ID[MCAL];
    }

    while (save.UNKNWN && (save.I <= save.NTOKNS)) {
        if (save.CLASS[save.IDENT[save.I]] == CALNDR) {
            save.CALTYP = save.IDENT[save.I];
            save.UNKNWN = false;
        }

        save.I = (save.I + 1);
    }

    //
    // Next determine whether or not we shall be performing rounding
    // on output.
    //
    save.PUMPUP = (ISRCHI(save.ID[ROUND], save.NTOKNS, save.IDENT.as_slice()) != 0);

    //
    // Determine if we have an Era specification
    //
    save.DOERA = ((((ISRCHI(save.ID[LERA], save.NTOKNS, save.IDENT.as_slice()) != 0)
        || (ISRCHI(save.ID[UERA], save.NTOKNS, save.IDENT.as_slice()) != 0))
        || (ISRCHI(save.ID[UERAX], save.NTOKNS, save.IDENT.as_slice()) != 0))
        || (ISRCHI(save.ID[LERAX], save.NTOKNS, save.IDENT.as_slice()) != 0));

    //
    // Until we've examined the year, we assume that the era is not
    // supposed to vanish.
    //
    save.VANISH = false;
    //
    // Next remove all of the time system dudes from the list of
    // tokens.
    //
    SCANRJ(
        save.DUMP.as_slice(),
        save.NDUMP,
        &mut save.NTOKNS,
        save.IDENT.as_slice_mut(),
        save.BEG.as_slice_mut(),
        save.END.as_slice_mut(),
    );

    //
    // If the user wants to round the output, we need to pump up ET
    // by the smallest significant part of the input picture.  But
    // in either case we are going to pad the input time.  For now
    // we pad it by zero.
    //
    save.TIMPAD = 0.0;

    if save.PUMPUP {
        //
        // We need to determine the amount to pad ET by.  So we need
        // to look at the string and find the least significant component
        // that has been requested.  Keep in mind that the last token
        // is of type NONAME (its a blank) by construction.
        //
        save.I = 1;

        while (save.I <= save.NTOKNS) {
            save.TYPE = save.CLASS[save.IDENT[save.I]];

            if (((((((save.TYPE == NONAME) || (save.TYPE == POINT)) || (save.TYPE == PLACE))
                || (save.TYPE == NOON))
                || (save.TYPE == ERA))
                || (save.TYPE == MON))
                || (save.TYPE == WKDAY))
            {
                //
                // Don't do anything, just go on to the next token.
                //
                save.I = (save.I + 1);
            } else {
                //
                // Look up the amount we should pad our time by.
                //
                save.FACTOR = 1.0;
                save.INCR = save.PAD[save.TYPE];

                //
                // Examine the next token.  If it's not a decimal point
                // and marker, we have the least significant part of
                // this component.
                //
                save.I = (save.I + 1);
                save.TYPE = save.CLASS[save.IDENT[save.I]];

                if (save.TYPE == POINT) {
                    save.FACTOR = (save.FACTOR * 0.1);
                    save.I = (save.I + 1);

                    //
                    // Now just look for the end of the string of place
                    // holders
                    //
                    while (save.IDENT[save.I] == save.ID[PLACE]) {
                        save.FACTOR = (save.FACTOR * 0.1);
                        save.I = (save.I + 1);
                    }
                }

                //
                // Now compute the time pad for this component of the
                // time string.
                //
                save.INCR = (save.INCR * save.FACTOR);

                if (save.TIMPAD != 0.0) {
                    save.TIMPAD = intrinsics::DMIN1(&[save.TIMPAD, save.INCR]);
                } else {
                    save.TIMPAD = save.INCR;
                }
            }
        }
    }

    //
    // Right now we don't have any components of the time format
    // and we don't need any of them so far.
    //
    for PART in BEGIN..=FINISH {
        save.HAVE[PART] = false;
    }

    //
    // Set up the input time format and the output time format that will
    // be used later.
    //
    // The input time format is used to convert the basic ET we have now
    // to one of the various time formats that are supported by the
    // routine TTRANS.  If we are going to construct a string in one of
    // the dynamical time systems we will call the input time a formal
    // time in seconds past a formal calendar epoch of J2000.  If on the
    // other hand we are going to construct a UTC based string, we will
    // convert our ET to an earth based epoch (TT, TDT) and use this as
    // our base input system.
    //
    //
    save.MYET = ET;

    if (save.TIMTYP == save.ID[TDB]) {
        //
        // Since we are likely to need SP2000, SP1950 or JD, we
        // compute them now.
        //
        save.MYET = (save.MYET + save.TIMPAD);
        save.VALUES[SP2000] = save.MYET;
        save.VALUES[JULIAN] = UNITIM(save.MYET, b"TDB", b"JDTDB", ctx)?;
        save.VALUES[SP1950] = (save.VALUES[SP2000] + (SPD() * (J2000() - J1950())));

        fstr::assign(&mut save.BASTYP, b"FORMAL");
        fstr::assign(&mut save.YMDFMT, b"YMDF");
        fstr::assign(&mut save.YWFMT, b"YMWDF");
        save.HAVE[SP2000] = true;
        save.HAVE[SP1950] = true;
        save.HAVE[JULIAN] = true;
    } else if ((save.TIMTYP == save.ID[TDT]) || (save.TIMTYP == save.ID[TT])) {
        save.MYET = (UNITIM(save.MYET, b"TDB", b"TDT", ctx)? + save.TIMPAD);
        save.VALUES[SP2000] = save.MYET;
        save.VALUES[JULIAN] = UNITIM(save.MYET, b"TDT", b"JDTDT", ctx)?;
        save.VALUES[SP1950] = (save.VALUES[SP2000] + (SPD() * (J2000() - J1950())));

        fstr::assign(&mut save.BASTYP, b"FORMAL");
        fstr::assign(&mut save.YMDFMT, b"YMDF");
        fstr::assign(&mut save.YWFMT, b"YMWDF");
        save.HAVE[SP2000] = true;
        save.HAVE[SP1950] = true;
        save.HAVE[JULIAN] = true;
    } else {
        //
        // In this case we convert to an earth based frame for our
        // working epoch.  This rounds properly when it's time to get
        // fractional components.
        //
        save.MYET = (UNITIM(save.MYET, b"TDB", b"TDT", ctx)? + save.TIMPAD);
        fstr::assign(&mut save.BASTYP, b"TDT");
        fstr::assign(&mut save.YMDFMT, b"YMD");
        fstr::assign(&mut save.YWFMT, b"YMWD");
    }

    //
    // Chapter 3.  Building the Output String
    // ==================================================================
    //

    //
    // Now we are ready to go, we need to fetch the tokens
    // and construct the output string.  We will
    // put the next portion of the output at APPND
    //
    save.APPND = 1;
    save.MAKING = false;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NTOKNS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.TYPE = save.CLASS[save.IDENT[save.I]];
            save.TVEC[1] = save.MYET;

            //
            // If the next marker is not one we use as a place holder
            // in the fractional part of decimal formats AND we
            // are in the process of building a format, then the format
            // building is done.  We can construct the component and
            // append it to the string we are building.
            //
            if ((save.TYPE != PLACE) && save.MAKING) {
                //
                // We also need to be sure this isn't a decimal point
                // marker before we add on to the output string.
                //
                if ((save.TYPE != POINT) || save.HAVE[POINT]) {
                    //
                    // We are going to truncate the number to the number of
                    // places requested NOT round.
                    //
                    save.TRNCAT = BRCKTI(((save.WIDTH - save.LENGTH[save.NUMTYP]) - 1), 0, 14);
                    save.FRAC = (save.VALUE - f64::trunc(save.VALUE));

                    if (save.FRAC < 0 as f64) {
                        save.VALUE = (save.VALUE - 1.0);
                        save.FRAC = (save.FRAC + 1.0);
                    }

                    save.INTMED = ((f64::trunc((save.FRAC * save.POWER[save.TRNCAT])) - 0.125)
                        / save.POWER[save.TRNCAT]);

                    save.FRAC = BRCKTD(save.INTMED, 0.0, 1.0);

                    save.VALUE = (f64::trunc(save.VALUE) + save.FRAC);

                    DPFMT(save.VALUE, &save.FMT, &mut save.SUBSTR, ctx)?;

                    fstr::assign(
                        fstr::substr_mut(&mut save.STRING, save.APPND..),
                        &save.SUBSTR,
                    );
                    save.APPND = (save.APPND + save.WIDTH);
                    save.HAVE[POINT] = false;
                    save.MAKING = false;
                }
            }

            //
            // If the token isn't recognized we can just
            // append it to the string we are constructing and
            // adjust the point at which the next substring is
            // to be appended.
            //
            if (save.TYPE == NONAME) {
                fstr::assign(
                    fstr::substr_mut(&mut save.STRING, save.APPND..),
                    fstr::substr(&save.MYSTR, save.BEG[save.I]..=save.END[save.I]),
                );
                save.APPND = (((save.APPND - save.BEG[save.I]) + save.END[save.I]) + 1);

            //
            // If the token is a place holder, we either just append it
            // or tack it on to a format string we are creating..
            //
            } else if (save.TYPE == PLACE) {
                if save.MAKING {
                    save.B = (save.WIDTH + 1);
                    save.E = ((save.B - save.BEG[save.I]) + save.END[save.I]);
                    fstr::assign(
                        fstr::substr_mut(&mut save.FMT, save.B..=save.E),
                        fstr::substr(&save.MYSTR, save.BEG[save.I]..=save.END[save.I]),
                    );
                    save.WIDTH = (((save.WIDTH - save.BEG[save.I]) + save.END[save.I]) + 1);
                } else {
                    fstr::assign(
                        fstr::substr_mut(&mut save.STRING, save.APPND..=save.APPND),
                        fstr::substr(&save.MYSTR, save.BEG[save.I]..=save.END[save.I]),
                    );
                    save.APPND = (((save.APPND - save.BEG[save.I]) + save.END[save.I]) + 1);
                }

            //
            // If the token is the decimal point plus place holder
            // AND we are making a format, we append it to the current
            // format and determine the fractional part of the current
            // quantity.
            //
            } else if (save.TYPE == POINT) {
                if !save.MAKING {
                    save.B = save.APPND;
                    save.E = ((save.APPND - save.BEG[save.I]) + save.END[save.I]);
                    fstr::assign(
                        fstr::substr_mut(&mut save.STRING, save.B..=save.E),
                        fstr::substr(&save.MYSTR, save.BEG[save.I]..=save.END[save.I]),
                    );
                    save.APPND = (save.E + 1);
                    save.HAVE[POINT] = false;
                } else if (save.TIMFMT == CONTIN) {
                    save.B = (save.WIDTH + 1);
                    save.E = ((save.B - save.BEG[save.I]) + save.END[save.I]);
                    fstr::assign(
                        fstr::substr_mut(&mut save.FMT, save.B..=save.E),
                        fstr::substr(&save.MYSTR, save.BEG[save.I]..=save.END[save.I]),
                    );
                    save.WIDTH = save.E;
                    save.HAVE[POINT] = true;
                } else {
                    save.B = (save.WIDTH + 1);
                    save.E = ((save.B - save.BEG[save.I]) + save.END[save.I]);
                    fstr::assign(
                        fstr::substr_mut(&mut save.FMT, save.B..=save.E),
                        fstr::substr(&save.MYSTR, save.BEG[save.I]..=save.END[save.I]),
                    );
                    save.WIDTH = save.E;
                    save.HAVE[POINT] = true;

                    //
                    // Since we obviously are going to be needing
                    // the fractional part of this component we fetch it
                    // now and add it to whatever the integer part of the
                    // current value is.  Here's how we do this.
                    // If we truncated the input time to this component
                    // we'd have a value on an "integer" portion of the
                    // time scale.
                    // .
                    // .               current
                    // .               time
                    // .               truncated    .---MYET
                    //                      |       |
                    //                      v       v
                    // time scale: ---------+-------X-----------+-----
                    //                                          ^
                    //                                          |
                    // .                               truncated time
                    // .                               plus 1 in this
                    // .                               component
                    // .
                    // Add one to the truncated component to get the
                    // next integer component.  Finally we convert these
                    // two constructed stings to seconds so that we can
                    // get the "fractional part" of the current component.
                    // Fortunately, when we computed the integer value
                    // for this component we constructed the time
                    // vectors we need, so we don't have to go to a lot
                    // of trouble now.
                    //

                    TTRANS(&save.INTYP, &save.BASTYP, save.PTVEC.as_slice_mut(), ctx)?;
                    TTRANS(&save.INTYP, &save.BASTYP, save.NTVEC.as_slice_mut(), ctx)?;

                    save.DELTA = intrinsics::DMAX1(&[1.0, (save.NTVEC[1] - save.PTVEC[1])]);
                    save.FRAC = BRCKTD(0.0, 1.0, ((save.MYET - save.PTVEC[1]) / save.DELTA));

                    save.VALUE = (save.VALUE + save.FRAC);
                }
            } else {
                //
                // If we get to this point we have an honest time
                // string component to fetch.  We might already have
                // this guy.  If so we can just collect him from the
                // values buffer (although this collection is performed
                // after the next long IF-THEN block that gets the value
                // if we don't already have it).
                //
                save.MAKING = true;
                save.HAVE[POINT] = false;
                fstr::assign(&mut save.FMT, save.ORIGNL.get(save.TYPE));
                save.WIDTH = save.LENGTH[save.TYPE];
                save.NUMTYP = save.TYPE;

                if !save.HAVE[save.TYPE] {
                    save.TVEC[1] = save.MYET;

                    //
                    // Most components are handled in the next block.
                    //
                    if ((((((((((((save.TYPE == YEAR) || (save.TYPE == YR))
                        || (save.TYPE == MONTH))
                        || (save.TYPE == MON))
                        || (save.TYPE == DAY))
                        || (save.TYPE == DOY))
                        || (save.TYPE == NOON))
                        || (save.TYPE == HOUR))
                        || (save.TYPE == ERA))
                        || (save.TYPE == AMPM))
                        || (save.TYPE == MINUTE))
                        || (save.TYPE == SEC))
                    {
                        TTRANS(&save.BASTYP, &save.YMDFMT, save.TVEC.as_slice_mut(), ctx)?;

                        //
                        // The seconds component is finished.  Regardless
                        // of any zone or calendar modifications, we just
                        // don't have to deal with this component any more.
                        //
                        save.VALUES[SEC] = save.TVEC[6];

                        //
                        // If we need to deal with time zones, this is
                        // the time to do it.
                        //
                        if (save.TIMTYP == save.ID[UTCP]) {
                            save.TVEC[4] = (save.TVEC[4] + save.HOFF);
                            save.TVEC[5] = (save.TVEC[5] + save.MOFF);
                            save.TVEC[6] = 0.0;

                            TTRANS(b"YMDF", b"YMDF", save.TVEC.as_slice_mut(), ctx)?;
                        }

                        //
                        // One way or the other the hours and minutes components
                        // are finished.  Record their values.
                        //
                        save.VALUES[HOUR] = save.TVEC[4];
                        save.VALUES[MINUTE] = save.TVEC[5];

                        if (save.VALUES[HOUR] == 0.0) {
                            save.VALUES[AMPM] = 12.0;
                        } else if (save.VALUES[HOUR] > 12.0) {
                            save.VALUES[AMPM] = (save.VALUES[HOUR] - 12.0);
                        } else {
                            save.VALUES[AMPM] = save.VALUES[HOUR];
                        }

                        //
                        // Finally, if we need to change the calendar to
                        // Julian this is the place to handle it.
                        //
                        save.JYEAR = intrinsics::IDNINT(save.TVEC[1]);
                        save.JMONTH = intrinsics::IDNINT(save.TVEC[2]);
                        save.JDAY = intrinsics::IDNINT(save.TVEC[3]);

                        GR2JUL(
                            &mut save.JYEAR,
                            &mut save.JMONTH,
                            &mut save.JDAY,
                            &mut save.JDOY,
                            ctx,
                        )?;

                        save.GYEAR = save.JYEAR;
                        save.GMONTH = save.JMONTH;
                        save.GDAY = save.JDAY;

                        JUL2GR(
                            &mut save.GYEAR,
                            &mut save.GMONTH,
                            &mut save.GDAY,
                            &mut save.GDOY,
                            ctx,
                        )?;

                        if (save.CALTYP == save.ID[GCAL]) {
                            save.VALUES[YEAR] = (save.GYEAR as f64);
                            save.VALUES[MONTH] = (save.GMONTH as f64);
                            save.VALUES[DAY] = (save.GDAY as f64);
                            save.VALUES[DOY] = (save.GDOY as f64);
                            save.GO2JUL = false;
                        } else if (save.CALTYP == save.ID[JCAL]) {
                            save.VALUES[YEAR] = (save.JYEAR as f64);
                            save.VALUES[MONTH] = (save.JMONTH as f64);
                            save.VALUES[DAY] = (save.JDAY as f64);
                            save.VALUES[DOY] = (save.JDOY as f64);
                            save.GO2JUL = true;
                        } else if (save.CALTYP == save.ID[MCAL]) {
                            if (save.GYEAR < 1582) {
                                save.GO2JUL = true;
                            } else if (save.GYEAR > 1582) {
                                save.GO2JUL = false;
                            } else if (save.GMONTH < 10) {
                                save.GO2JUL = true;
                            } else if (save.GMONTH > 10) {
                                save.GO2JUL = false;
                            } else if (save.GDAY >= 15) {
                                save.GO2JUL = false;
                            } else {
                                save.GO2JUL = true;
                            }

                            if save.GO2JUL {
                                save.VALUES[YEAR] = (save.JYEAR as f64);
                                save.VALUES[MONTH] = (save.JMONTH as f64);
                                save.VALUES[DAY] = (save.JDAY as f64);
                                save.VALUES[DOY] = (save.JDOY as f64);
                            } else {
                                save.VALUES[YEAR] = (save.GYEAR as f64);
                                save.VALUES[MONTH] = (save.GMONTH as f64);
                                save.VALUES[DAY] = (save.GDAY as f64);
                                save.VALUES[DOY] = (save.GDOY as f64);
                            }
                        }

                        //
                        // Determine the era associated with the epoch.  Also
                        // if the year component is negative, we handle  that
                        // now.
                        //
                        // We store the actual value of the year so that
                        // it can be used when determining rounding of
                        // other components.
                        //
                        save.VALUES[RLYEAR] = save.VALUES[YEAR];

                        if save.DOERA {
                            if (save.VALUES[YEAR] < 1 as f64) {
                                save.VALUES[YEAR] = (1.0 - save.VALUES[YEAR]);
                                save.VALUES[ERA] = 1.0;
                            } else {
                                save.VALUES[ERA] = 2.0;
                            }

                            save.VANISH = (save.VALUES[YEAR] >= 1000.0);
                        }

                        //
                        // Fetch the last two digits of the year.
                        //
                        RMAIND(save.VALUES[YEAR], 100.0, &mut save.X, &mut save.TEMPD, ctx)?;
                        save.VALUES[YR] = save.TEMPD;

                        save.HAVE[YEAR] = true;
                        save.HAVE[YR] = true;
                        save.HAVE[DOY] = true;
                        save.HAVE[MONTH] = true;
                        save.HAVE[MON] = true;
                        save.HAVE[DAY] = true;
                        save.HAVE[HOUR] = true;
                        save.HAVE[MINUTE] = true;
                        save.HAVE[SEC] = true;
                        save.HAVE[AMPM] = true;
                        save.HAVE[ERA] = true;
                    } else if (save.TYPE == WKDAY) {
                        save.TVEC[1] = save.MYET;

                        TTRANS(&save.BASTYP, &save.YWFMT, save.TVEC.as_slice_mut(), ctx)?;

                        //
                        // Weekday. If we need to deal with time zones, this is
                        // the time to do it.
                        //
                        if (save.TIMTYP == save.ID[UTCP]) {
                            save.TVEC[5] = (save.TVEC[5] + save.HOFF);
                            save.TVEC[6] = (save.TVEC[6] + save.MOFF);
                            save.TVEC[7] = 0.0;

                            TTRANS(b"YMWDF", b"YMWDF", save.TVEC.as_slice_mut(), ctx)?;
                        }

                        save.VALUES[WKDAY] = save.TVEC[4];
                        save.HAVE[WKDAY] = true;
                    } else if ((save.TYPE == SP1950) || (save.TYPE == SP2000)) {
                        //
                        // The only way to get here is if the output time
                        // type is UTC or a time zone (otherwise we'd
                        // already HAVE SP2000 and SP1950).
                        //
                        save.TVEC[1] = save.MYET;

                        TTRANS(&save.BASTYP, b"FORMAL", save.TVEC.as_slice_mut(), ctx)?;

                        save.VALUES[SP2000] = save.TVEC[1];
                        save.VALUES[SP1950] = (save.VALUES[SP2000] + (SPD() * (J2000() - J1950())));

                        save.HAVE[SP2000] = true;
                        save.HAVE[SP1950] = true;
                    } else if (save.TYPE == JULIAN) {
                        //
                        // The same tale can be told here as in the last
                        // case.  We can only get here if this is UTC
                        // output.
                        //
                        save.TVEC[1] = save.MYET;

                        TTRANS(&save.BASTYP, b"JDUTC", save.TVEC.as_slice_mut(), ctx)?;

                        save.VALUES[JULIAN] = save.TVEC[1];
                        save.HAVE[JULIAN] = true;
                    }
                }

                //
                // O.K. whatever thing we are about to construct, we now
                // have it's numeric value.  It's time to construct its
                // string  value.
                //
                //
                // We need to treat character months, weekdays, eras, a.m.'s
                // and p.m.'s specially.
                //
                if (save.TYPE == MON) {
                    save.INDX = intrinsics::IDNINT(save.VALUES[MONTH]);
                    fstr::assign(&mut save.MYMON, save.MONTHS.get(save.INDX));
                    save.MONTYP = save.IDENT[save.I];

                    //
                    // There is no ELSE case in the block below because all of
                    // the possible MONTYP values are checked explicitly.
                    //
                    if (save.MONTYP == save.ID[UMON]) {
                        UCASE(&save.MYMON.to_vec(), &mut save.MYMON, ctx);
                        fstr::assign(fstr::substr_mut(&mut save.MYMON, 4..), b" ");
                        save.MYLEN = 3;
                    } else if (save.MONTYP == save.ID[MMON]) {
                        fstr::assign(fstr::substr_mut(&mut save.MYMON, 4..), b" ");
                        save.MYLEN = 3;
                    } else if (save.MONTYP == save.ID[LMON]) {
                        LCASE(&save.MYMON.to_vec(), &mut save.MYMON, ctx);
                        fstr::assign(fstr::substr_mut(&mut save.MYMON, 4..), b" ");
                        save.MYLEN = 3;
                    } else if (save.MONTYP == save.ID[MMNTH]) {
                        save.MYLEN = save.MLEN[save.INDX];
                    } else if (save.MONTYP == save.ID[UMNTH]) {
                        UCASE(&save.MYMON.to_vec(), &mut save.MYMON, ctx);
                        save.MYLEN = save.MLEN[save.INDX];
                    } else if (save.MONTYP == save.ID[LMNTH]) {
                        LCASE(&save.MYMON.to_vec(), &mut save.MYMON, ctx);
                        save.MYLEN = save.MLEN[save.INDX];
                    }

                    fstr::assign(
                        fstr::substr_mut(&mut save.STRING, save.APPND..),
                        &save.MYMON,
                    );
                    save.APPND = (save.APPND + save.MYLEN);
                    save.MAKING = false;
                } else if (save.TYPE == WKDAY) {
                    save.INDX = intrinsics::IDNINT(save.VALUES[WKDAY]);
                    fstr::assign(&mut save.MYWKD, save.WKDAYS.get(save.INDX));
                    save.WKTYP = save.IDENT[save.I];

                    //
                    // There is no ELSE case in the block below because all of
                    // the possible WKTYP values are checked explicitly.
                    //
                    if (save.WKTYP == save.ID[UWKD]) {
                        UCASE(&save.MYWKD.to_vec(), &mut save.MYWKD, ctx);
                        fstr::assign(fstr::substr_mut(&mut save.MYWKD, 4..), b" ");
                        save.MYLEN = 3;
                    } else if (save.WKTYP == save.ID[MWKD]) {
                        fstr::assign(fstr::substr_mut(&mut save.MYWKD, 4..), b" ");
                        save.MYLEN = 3;
                    } else if (save.WKTYP == save.ID[LWKD]) {
                        LCASE(&save.MYWKD.to_vec(), &mut save.MYWKD, ctx);
                        fstr::assign(fstr::substr_mut(&mut save.MYWKD, 4..), b" ");
                        save.MYLEN = 3;
                    } else if (save.WKTYP == save.ID[MWEEKD]) {
                        save.MYLEN = save.WKLEN[save.INDX];
                    } else if (save.WKTYP == save.ID[UWEEKD]) {
                        UCASE(&save.MYWKD.to_vec(), &mut save.MYWKD, ctx);
                        save.MYLEN = save.WKLEN[save.INDX];
                    } else if (save.WKTYP == save.ID[LWEEKD]) {
                        LCASE(&save.MYWKD.to_vec(), &mut save.MYWKD, ctx);
                        save.MYLEN = save.WKLEN[save.INDX];
                    }

                    fstr::assign(
                        fstr::substr_mut(&mut save.STRING, save.APPND..),
                        &save.MYWKD,
                    );
                    save.APPND = (save.APPND + save.MYLEN);
                    save.MAKING = false;
                } else if (save.TYPE == ERA) {
                    if ((save.VALUES[ERA] == 2.0)
                        && ((save.IDENT[save.I] == save.ID[UERA])
                            || (save.IDENT[save.I] == save.ID[UERAX])))
                    {
                        fstr::assign(fstr::substr_mut(&mut save.STRING, save.APPND..), b" A.D.");
                    } else if ((save.VALUES[ERA] == 2.0)
                        && ((save.IDENT[save.I] == save.ID[LERA])
                            || (save.IDENT[save.I] == save.ID[LERAX])))
                    {
                        fstr::assign(fstr::substr_mut(&mut save.STRING, save.APPND..), b" a.d.");
                    } else if ((save.VALUES[ERA] == 1.0)
                        && ((save.IDENT[save.I] == save.ID[UERA])
                            || (save.IDENT[save.I] == save.ID[UERAX])))
                    {
                        fstr::assign(fstr::substr_mut(&mut save.STRING, save.APPND..), b" B.C.");
                    } else {
                        fstr::assign(fstr::substr_mut(&mut save.STRING, save.APPND..), b" b.c.");
                    }
                    //
                    // If we have the vanishing kind of era, and we've
                    // determined that it needs to vanish, then blank out the
                    // portion of the string we just filled in. and don't
                    // increment the place holder.
                    //
                    if ((save.IDENT[save.I] == save.ID[UERAX])
                        || (save.IDENT[save.I] == save.ID[LERAX]))
                    {
                        if save.VANISH {
                            fstr::assign(fstr::substr_mut(&mut save.STRING, save.APPND..), b" ");
                            save.APPND = (save.APPND + 1);
                        } else {
                            save.APPND = (save.APPND + 6);
                        }
                    } else {
                        LJUST(
                            &fstr::substr(&save.STRING, save.APPND..).to_vec(),
                            fstr::substr_mut(&mut save.STRING, save.APPND..),
                        );
                        save.APPND = (save.APPND + 4);
                    }

                    save.MAKING = false;
                } else if (save.TYPE == NOON) {
                    if ((save.IDENT[save.I] == save.ID[UAMPM]) && (save.VALUES[HOUR] >= 12.0)) {
                        fstr::assign(fstr::substr_mut(&mut save.STRING, save.APPND..), b"P.M.");
                    } else if ((save.IDENT[save.I] == save.ID[UAMPM]) && (save.VALUES[HOUR] < 12.0))
                    {
                        fstr::assign(fstr::substr_mut(&mut save.STRING, save.APPND..), b"A.M.");
                    } else if ((save.IDENT[save.I] == save.ID[LAMPM])
                        && (save.VALUES[HOUR] >= 12.0))
                    {
                        fstr::assign(fstr::substr_mut(&mut save.STRING, save.APPND..), b"p.m.");
                    } else {
                        fstr::assign(fstr::substr_mut(&mut save.STRING, save.APPND..), b"a.m.");
                    }

                    save.APPND = (save.APPND + 4);
                    save.MAKING = false;
                } else {
                    save.VALUE = save.VALUES[save.TYPE];
                }

                //
                // If we are now creating a format string, we should
                // construct the previous time representation and
                // the next for this component (just in case we need it
                // later).
                //
                if save.MAKING {
                    //
                    // We store the value of our current type in the
                    // CURRNT slot of the values array.  This value
                    // is used by the single numeric types, JD, SP2000,
                    // and SP1950.
                    //
                    save.VALUES[CURRNT] = save.VALUES[save.TYPE];

                    //
                    // Here's how this works:  We will copy all of
                    // the components of the time representation up to
                    // the current one.  This is the truncated representation
                    // of our epoch.  We then copy these same components into
                    // another time vector, but add an increment to the
                    // component corresponding to the one we are dealing with
                    // now.  We use an increment of 0 for those components that
                    // already contain their fractional part. We use an
                    // increment of 1 for the components that typically have
                    // integer representations.
                    //
                    //
                    // Zero out the previous and next time vectors so we won't
                    // have to do it when we are filling in the truncated
                    // portions.
                    //
                    for J in 1..=7 {
                        save.PTVEC[J] = 0.0;
                        save.NTVEC[J] = 0.0;
                    }

                    if ((save.TYPE == YEAR) || (save.TYPE == YR)) {
                        save.STOPAT = 1;
                        save.TIMFMT = YMD;
                        fstr::assign(&mut save.INTYP, &save.YMDFMT);
                        save.INCR = 1.0;
                    } else if (save.TYPE == MONTH) {
                        save.STOPAT = 2;
                        save.TIMFMT = YMD;
                        fstr::assign(&mut save.INTYP, &save.YMDFMT);
                        save.INCR = 1.0;
                    } else if ((save.TYPE == DAY) || (save.TYPE == DOY)) {
                        save.STOPAT = 3;
                        save.TIMFMT = YMD;
                        fstr::assign(&mut save.INTYP, &save.YMDFMT);
                        save.INCR = 1.0;
                    } else if ((save.TYPE == HOUR) || (save.TYPE == AMPM)) {
                        //
                        // Note that in this case (and the next 2) that if we
                        // an HOUR component, we had to get it either from
                        // a Day of Year format or from a Year Month Day
                        // format. Thus we have all of the more significant
                        // components for this format.
                        //
                        save.STOPAT = 4;
                        save.TIMFMT = YMD;
                        fstr::assign(&mut save.INTYP, &save.YMDFMT);
                        save.INCR = 1.0;
                    } else if (save.TYPE == MINUTE) {
                        save.STOPAT = 5;
                        save.TIMFMT = YMD;
                        fstr::assign(&mut save.INTYP, &save.YMDFMT);

                        save.INCR = 1.0;
                    } else if (save.TYPE == SEC) {
                        save.STOPAT = 6;
                        save.TIMFMT = YMD;
                        fstr::assign(&mut save.INTYP, &save.YMDFMT);
                        save.INCR = 0.0;
                    } else if (save.TYPE == JULIAN) {
                        save.STOPAT = 1;
                        save.TIMFMT = CONTIN;
                        save.INCR = 0.0;

                        if ((save.TIMTYP == save.ID[TDT]) || (save.TIMTYP == save.ID[TT])) {
                            fstr::assign(&mut save.INTYP, b"JDTDT");
                        } else if (save.TIMTYP == save.ID[TDB]) {
                            fstr::assign(&mut save.INTYP, b"JDTDB");
                        } else if ((save.TIMTYP == save.ID[UTC]) || (save.TIMTYP == save.ID[UTCP]))
                        {
                            fstr::assign(&mut save.INTYP, b"JDUTC");
                        }
                    } else {
                        //
                        // The only types left are the continuous (numeric)
                        // types.
                        //
                        save.STOPAT = 1;
                        save.TIMFMT = CONTIN;
                        save.INCR = 0.0;
                        fstr::assign(&mut save.INTYP, &save.BASTYP);
                    }

                    //
                    // Ok.  We are now ready to construct the previous
                    // and next time vectors.
                    //
                    for J in 1..=save.STOPAT {
                        save.PTVEC[J] = save.VALUES[save.COMPNT[[J, save.TIMFMT]]];
                        save.NTVEC[J] = save.PTVEC[J];
                    }

                    save.NTVEC[save.STOPAT] = (save.NTVEC[save.STOPAT] + save.INCR);

                    //
                    // If the type is a year or month, then we need to set
                    // the month to 1, so that we will be working with
                    // beginnings of years not beginning of last months of
                    // the previous year.
                    //
                    if ((save.TYPE == YEAR) || (save.TYPE == YR)) {
                        save.PTVEC[2] = 1.0;
                        save.NTVEC[2] = 1.0;
                        save.PTVEC[3] = 1.0;
                        save.NTVEC[3] = 1.0;
                    } else if (save.TYPE == MONTH) {
                        save.PTVEC[3] = 1.0;
                        save.NTVEC[3] = 1.0;
                    }

                    if (save.GO2JUL && (save.TIMFMT != CONTIN)) {
                        //
                        // Convert both PTVEC and NTVEC to the gregorian
                        // calendar
                        //
                        save.JYEAR = intrinsics::IDNINT(save.PTVEC[1]);
                        save.JMONTH = intrinsics::IDNINT(save.PTVEC[2]);
                        save.JDAY = intrinsics::IDNINT(save.PTVEC[3]);

                        JUL2GR(
                            &mut save.JYEAR,
                            &mut save.JMONTH,
                            &mut save.JDAY,
                            &mut save.JDOY,
                            ctx,
                        )?;

                        save.PTVEC[1] = (save.JYEAR as f64);
                        save.PTVEC[2] = (save.JMONTH as f64);
                        save.PTVEC[3] = (save.JDAY as f64);

                        save.JYEAR = intrinsics::IDNINT(save.NTVEC[1]);
                        save.JMONTH = intrinsics::IDNINT(save.NTVEC[2]);
                        save.JDAY = intrinsics::IDNINT(save.NTVEC[3]);

                        JUL2GR(
                            &mut save.JYEAR,
                            &mut save.JMONTH,
                            &mut save.JDAY,
                            &mut save.JDOY,
                            ctx,
                        )?;

                        save.NTVEC[1] = (save.JYEAR as f64);
                        save.NTVEC[2] = (save.JMONTH as f64);
                        save.NTVEC[3] = (save.JDAY as f64);
                    }

                    //
                    // Handle the +/- time zone shifts.
                    //
                    if (save.DOZONE && (save.TIMFMT != CONTIN)) {
                        save.PTVEC[4] = (save.PTVEC[4] - save.HOFF);
                        save.NTVEC[4] = (save.NTVEC[4] - save.HOFF);

                        save.PTVEC[5] = (save.PTVEC[5] - save.MOFF);
                        save.NTVEC[5] = (save.NTVEC[5] - save.MOFF);

                        save.PTVEC[6] = 0.0;
                        save.NTVEC[6] = 0.0;

                        TTRANS(b"YMDF", b"YMDF", save.PTVEC.as_slice_mut(), ctx)?;
                        TTRANS(b"YMDF", b"YMDF", save.NTVEC.as_slice_mut(), ctx)?;

                        if (save.TYPE == SEC) {
                            save.PTVEC[6] = save.VALUES[SEC];
                            save.NTVEC[6] = save.VALUES[SEC];
                        }
                    }
                }
            }

            save.I += m3__;
        }
    }

    //
    // All that's left to do is to copy the constructed string
    // to the output string.
    //
    fstr::assign(OUTPUT, &save.STRING);

    CHKOUT(b"TIMOUT", ctx)?;
    Ok(())
}
