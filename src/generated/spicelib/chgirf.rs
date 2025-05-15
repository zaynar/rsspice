//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NINERT: i32 = 21;
const MAXF: i32 = NINERT;

struct SaveVars {
    BASES: ActualCharArray,
    DEFS: ActualCharArray,
    ERROR: Vec<u8>,
    FRAMES: ActualCharArray,
    WORD: Vec<u8>,
    ANGLE: f64,
    RADANG: f64,
    TMPMAT: StackArray2D<f64, 9>,
    TRANS: StackArray2D<f64, 189>,
    AXIS: i32,
    B: i32,
    DFRAME: i32,
    LOC: i32,
    P: i32,
    READY: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BASES = ActualCharArray::new(16, 1..=MAXF);
        let mut DEFS = ActualCharArray::new(80, 1..=MAXF);
        let mut ERROR = vec![b' '; 25];
        let mut FRAMES = ActualCharArray::new(16, 1..=MAXF);
        let mut WORD = vec![b' '; 25];
        let mut ANGLE: f64 = 0.0;
        let mut RADANG: f64 = 0.0;
        let mut TMPMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut TRANS = StackArray2D::<f64, 189>::new(1..=9, 1..=MAXF);
        let mut AXIS: i32 = 0;
        let mut B: i32 = 0;
        let mut DFRAME: i32 = 0;
        let mut LOC: i32 = 0;
        let mut P: i32 = 0;
        let mut READY: bool = false;

        READY = false;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000")].into_iter();
            fstr::assign(FRAMES.get_mut(1), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000")].into_iter();
            fstr::assign(BASES.get_mut(1), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.0  1")].into_iter();
            fstr::assign(DEFS.get_mut(1), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(FRAMES.get_mut(2), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000")].into_iter();
            fstr::assign(BASES.get_mut(2), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(
                b"1152.84248596724 3  -1002.26108439117  2  1153.04066200330  3",
            )]
            .into_iter();
            fstr::assign(DEFS.get_mut(2), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"FK4")].into_iter();
            fstr::assign(FRAMES.get_mut(3), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(3), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.525  3")].into_iter();
            fstr::assign(DEFS.get_mut(3), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-118")].into_iter();
            fstr::assign(FRAMES.get_mut(4), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(4), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.53155  3")].into_iter();
            fstr::assign(DEFS.get_mut(4), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-96")].into_iter();
            fstr::assign(FRAMES.get_mut(5), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(5), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.4107  3")].into_iter();
            fstr::assign(DEFS.get_mut(5), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-102")].into_iter();
            fstr::assign(FRAMES.get_mut(6), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(6), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.1359  3")].into_iter();
            fstr::assign(DEFS.get_mut(6), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-108")].into_iter();
            fstr::assign(FRAMES.get_mut(7), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(7), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.4775  3")].into_iter();
            fstr::assign(DEFS.get_mut(7), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-111")].into_iter();
            fstr::assign(FRAMES.get_mut(8), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(8), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.5880  3")].into_iter();
            fstr::assign(DEFS.get_mut(8), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-114")].into_iter();
            fstr::assign(FRAMES.get_mut(9), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(9), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.5529  3")].into_iter();
            fstr::assign(DEFS.get_mut(9), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-122")].into_iter();
            fstr::assign(FRAMES.get_mut(10), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(10), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.5316  3")].into_iter();
            fstr::assign(DEFS.get_mut(10), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-125")].into_iter();
            fstr::assign(FRAMES.get_mut(11), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(11), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.5754  3")].into_iter();
            fstr::assign(DEFS.get_mut(11), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-130")].into_iter();
            fstr::assign(FRAMES.get_mut(12), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(12), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.5247  3")].into_iter();
            fstr::assign(DEFS.get_mut(12), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"GALACTIC")].into_iter();
            fstr::assign(FRAMES.get_mut(13), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"FK4")].into_iter();
            fstr::assign(BASES.get_mut(13), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"1177200.0  3  225360.0  1  1016100.0  3")].into_iter();
            fstr::assign(DEFS.get_mut(13), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-200")].into_iter();
            fstr::assign(FRAMES.get_mut(14), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000")].into_iter();
            fstr::assign(BASES.get_mut(14), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.0  3")].into_iter();
            fstr::assign(DEFS.get_mut(14), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-202")].into_iter();
            fstr::assign(FRAMES.get_mut(15), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000")].into_iter();
            fstr::assign(BASES.get_mut(15), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"0.0  3")].into_iter();
            fstr::assign(DEFS.get_mut(15), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"MARSIAU")].into_iter();
            fstr::assign(FRAMES.get_mut(16), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000")].into_iter();
            fstr::assign(BASES.get_mut(16), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"324000.0D0 3 133610.4D0 2 -152348.4D0 3")].into_iter();
            fstr::assign(DEFS.get_mut(16), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ECLIPJ2000")].into_iter();
            fstr::assign(FRAMES.get_mut(17), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000")].into_iter();
            fstr::assign(BASES.get_mut(17), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"84381.448 1")].into_iter();
            fstr::assign(DEFS.get_mut(17), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ECLIPB1950")].into_iter();
            fstr::assign(FRAMES.get_mut(18), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"B1950")].into_iter();
            fstr::assign(BASES.get_mut(18), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"84404.836 1")].into_iter();
            fstr::assign(DEFS.get_mut(18), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-140")].into_iter();
            fstr::assign(FRAMES.get_mut(19), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000")].into_iter();
            fstr::assign(BASES.get_mut(19), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(
                b"1152.71013777252 3  -1002.25042010533  2  1153.75719544491  3",
            )]
            .into_iter();
            fstr::assign(DEFS.get_mut(19), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-142")].into_iter();
            fstr::assign(FRAMES.get_mut(20), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000")].into_iter();
            fstr::assign(BASES.get_mut(20), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(
                b"1152.72061453864 3  -1002.25052830351  2  1153.74663857521  3",
            )]
            .into_iter();
            fstr::assign(DEFS.get_mut(20), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DE-143")].into_iter();
            fstr::assign(FRAMES.get_mut(21), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000")].into_iter();
            fstr::assign(BASES.get_mut(21), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(
                b"1153.03919093833, 3, -1002.24822382286, 2, 1153.42900222357, 3",
            )]
            .into_iter();
            fstr::assign(DEFS.get_mut(21), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        DFRAME = 0;

        Self {
            BASES,
            DEFS,
            ERROR,
            FRAMES,
            WORD,
            ANGLE,
            RADANG,
            TMPMAT,
            TRANS,
            AXIS,
            B,
            DFRAME,
            LOC,
            P,
            READY,
        }
    }
}

/// Change inertial reference frames
///
/// Support changes among a standard set of inertial coordinate
/// reference frames.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  REFA       I   IRFROT
///  REFB       I   IRFROT
///  ROTAB      O   IRFROT
///  NAME      I-O  IRFNUM, IRFNAM, IRFDEF
///  INDEX     I-O  IRFNUM, IRFNAM
/// ```
///
/// # Detailed Input
///
/// ```text
///  See entry points IRFROT, IRFNUM, IRFNAM, and IRFDEF.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See entry points IRFROT, IRFNUM, and IRFNAM.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If CHGIRF is called directly, the signal, the error
///      SPICE(BOGUSENTRY) is signaled.
///
///  2)  See entry points IRFROT, IRFNUM, IRFNAM, and IRFDEF for
///      exceptions specific to those routines.
/// ```
///
/// # Particulars
///
/// ```text
///  CHGIRF exists only as an umbrella for data to be shared
///  by its entry points (IRFROT, IRFNUM, IRFNAM, and IRFDEF).
///  It should never be called directly.
/// ```
///
/// # Examples
///
/// ```text
///  See entry points IRFROT, IRFNUM, IRFNAM, and IRFDEF.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Lieske, "Precession Matrix Based on IAU (1976) System of
///       Astronomical Constants," Astron. Astrophys. 73, 282-284,
///       1979.
///
///  [2]  E. M. Standish, Jr., "Orientation of the JPL Ephemerides,
///       DE 200/LE 200, to the Dynamical Equinox of J2000," Astron.
///       Astrophys. 114, 297-302, 1982.
///
///  [3]  E. M. Standish, Jr., "Conversion of Ephemeris Coordinates
///       from the B1950 System to the J2000 System," JPL IOM
///       314.6-581, 24 June 1985.
///
///  [4]  E. M. Standish, Jr., "The Equinox Offsets of the JPL
///       Ephemeris," JPL IOM 314.6-929, 26 February 1988.
///
///  [5]  J. Lieske, "Expressions for the Precession Quantities Based
///       upon the IAU (1976) System of Astronomical Constants,"
///       Astron. Astrophys. 58, 1-16, 1977.
///
///  [6]  L. Bass and R. Cesarone, "Mars Observer Planetary Constants
///       and Models," JPL D-3444, November 1990.
///
///  [7]  P. Kenneth Seidelmann (Ed.), "Explanatory Supplement to the
///       Astronomical Almanac," University Science Books, 1992.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.5.0, 06-NOV-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Corrected sign error in comments describing the angles used to
///         convert between the J2000 and B1950 reference frames. Also
///         corrected a typo in the comments.
///
/// -    SPICELIB Version 4.4.0, 24-SEP-2013 (BVS)
///
///         Updated entry point IRFNUM to treat J2000 as a special case
///         and to not participate of CHKIN/CHOUT to increase efficiency.
///
/// -    SPICELIB Version 4.3.0, 25-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in CONVRT, ROTMAT and MXM calls.
///
/// -    SPICELIB Version 4.2.1, 04-JAN-2002 (EDW)
///
///         Added DE-143 to header description for IRFROT.
///
/// -    SPICELIB Version 4.2.0, 10-APR-1997 (WLT)
///
///         A descriptive diagnostic was added to the entry points
///         IRFROT and IRFDEF. Before they simply signaled the error
///         with no diagnostic.
///
/// -    SPICELIB Version 4.1.0, 14-OCT-1996 (WLT)
///
///         The number of inertial frames recognized is now stored
///         in the include file ninert.inc.
///
/// -    SPICELIB Version 4.0.0, 20-MAY-1996 (WLT)
///
///         The inertial frame DE-143 was added to the list of recognized
///         inertial frames.
///
/// -    SPICELIB Version 3.0.0, 20-MAR-1995 (WLT)
///
///         The inertial frames DE-140 and DE-142  were added to the
///         list of recognized inertial frames.
///
/// -    SPICELIB Version 2.0.0, 30-JUL-1993 (WLT)
///
///         The transformation from J2000 to B1950 was upgraded
///         so that the transformation matrix produced matches
///         the matrix given in [1].
///
///         The frame MARSIAU was added to the list
///         of recognized frames. This is the standard mars
///         referenced inertial frame used by the Mars Observer
///         project.
///
///         Values for the obliquity of the ecliptic were taken
///         from the Explanatory Supplement [7] to the Astronomical
///         Almanac (1992) at both the epochs J2000 and B1950 and
///         used to define the mean ecliptic and equinox frames
///         ECLIPJ2000 and ECLIPB1950.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.3.0, 25-AUG-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in CONVRT, ROTMAT  and MXM calls.
/// ```
pub fn chgirf(
    ctx: &mut SpiceContext,
    refa: i32,
    refb: i32,
    rotab: &[[f64; 3]; 3],
    name: &str,
    index: i32,
) -> crate::Result<()> {
    CHGIRF(
        refa,
        refb,
        rotab.as_flattened(),
        name.as_bytes(),
        index,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CHGIRF ( Change inertial reference frames )
pub fn CHGIRF(
    REFA: i32,
    REFB: i32,
    ROTAB: &[f64],
    NAME: &[u8],
    INDEX: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //
    //
    // Each frame is defined in terms of another frame, except for
    // the root frame, which is defined in terms of itself. For now,
    // the root frame is the standard IAU reference frame, J2000,
    // defined by the Earth mean equator and dynamical equinox of
    // Julian year 2000.
    //
    // Each definition consists of a series of rotations, each
    // through some angle (in arc seconds) and about some axis.
    // The rotations are listed in the opposite order in which
    // they are to be performed, so as to correspond more closely
    // to conventional notation. For example, the definition
    //
    //    FRAMES(i) = 'F2'
    //    BASES(i)  = 'F1'
    //    DEFS(i)   = '22.34  3   31.21  2   0.449  1'
    //
    // means that a vector in frame F1 is converted to the equivalent
    // vector in frame F2 by applying the following rotation:
    //
    //    -                                            -
    //    v    = ( [ 22.34 ]  [ 31.21 ]  [ 0.449 ]  )  v
    //     F2               3          2          1     F1
    //
    // where the notation
    //
    //    [ theta ]
    //             a
    //
    // means ``rotate through angle theta about axis a.''
    //
    // New frames may be added by:
    //
    //    1) Increasing the value of MAXF.
    //
    //    2) Adding new values for FRAMES, BASES, and DEFS.
    //
    // The actual transformations (TRANS) will be computed during
    // initialization.
    //
    // Note that BASES must be the name of a previously defined
    // reference frame, and that no frame should appear more than
    // once in FRAMES.
    //
    // Note also that the list of valid reference frames maintained
    // by CHGIRF must be updated whenever new frames are added.
    //

    //
    // The root frame is mostly for show. Rotate by 0 arc seconds
    // about the x-axis to obtain the identity matrix.
    //

    //
    // The B1950 reference frame is obtained by precessing the J2000
    // frame backwards from Julian year 2000 to Besselian year 1950,
    // using the 1976 IAU precession model.
    //
    // The rotation from B1950 to J2000 is
    //
    //    [ -z ]  [ theta ]  [ -zeta ]
    //          3          2          3
    //
    // So the rotation from J2000 to B1950 is the transpose,
    //
    //    [ zeta ]  [ -theta ]  [ z ]
    //            3           2      3
    //
    // The values for z, theta, and zeta are computed from the formulas
    // given in table 5 of [5].
    //
    //    z     =  1153.04066200330"
    //    theta =  1002.26108439117"
    //    zeta  =  1152.84248596724"
    //

    //
    // The FK4 reference frame is derived from the B1950 frame by
    // applying the equinox offset determined by Fricke. This is just
    // the rotation
    //
    //    [ 0.525" ]
    //              3
    //

    //
    // The DE-118 reference frame is nearly identical to the FK4
    // reference frame. It is also derived from the B1950 frame.
    // Only the offset is different:
    //
    //    [ 0.53155" ]
    //                3
    //
    // In [2], Standish uses two separate rotations,
    //
    //    [ 0.00073" ]  P [ 0.5316" ]
    //                3              3
    //
    // (where P is the precession matrix used above to define the
    // B1950 frame). The major effect of the second rotation is to
    // correct for truncating the magnitude of the first rotation.
    // At his suggestion, we will use the untruncated value, and
    // stick to a single rotation.
    //

    //
    // Most of the other DE reference frames may be defined relative
    // to either the DE-118 or B1950 frames. The values below are taken
    // from [4].
    //
    //    DE number   Offset from DE-118   Offset from B1950
    //    ---------   ------------------   -----------------
    //           96             +0.1209"            +0.4107"
    //          102             +0.3956"            +0.1359"
    //          108             +0.0541"            +0.4775"
    //          111             -0.0564"            +0.5880"
    //          114             -0.0213"            +0.5529"
    //          122             +0.0000"            +0.5316"
    //          125             -0.0438"            +0.5754"
    //          130             +0.0069"            +0.5247"
    //
    // We will use B1950 for now, since the offsets generally have
    // more significant digits.
    //

    //
    // The Galactic System II reference frame is defined by the
    // following rotations:
    //
    //         o          o            o
    //    [ 327  ]  [ 62.6  ]  [ 282.25  ]
    //            3          1            3
    //
    //  In the absence of better information, we will assume that
    //  it is derived from the FK4 frame. Converting the angles from
    //  degrees to arc seconds,
    //
    //       o
    //    327      = 1177200"
    //        o
    //    62.6     =  225360"
    //          o
    //    282.25   = 1016100"
    //

    //
    // According to Standish, the various DE-200 frames are identical
    // with J2000, because he rotates the ephemerides before releasing
    // them (in order to avoid problems like the one that this routine
    // is designed to solve). Because we have to have something, we
    // will use
    //
    //         o
    //    [ 0.0 ]
    //           3
    //

    //
    // The values for the transformation from J2000 to MARSIAU_MO
    // are derived from the constants given for the pole of Mars
    // on page 8-2 of reference [6].
    //

    //
    // The value for the obliquity of the ecliptic at J2000  is
    // taken from page  114 of [7] equation 3.222-1.  This agrees
    // with the expression given in [5]
    //

    //
    // The value for the obliquity of the ecliptic at B1950  is
    // taken from page  171 of [7].
    //

    //
    // The frame for DE-140 is simply DE-400 rotated by the rotation:
    //
    //  0.9999256765384668  0.0111817701197967  0.0048589521583895
    // -0.0111817701797229  0.9999374816848701 -0.0000271545195858
    // -0.0048589520204830 -0.0000271791849815  0.9999881948535965
    //
    // Note that the DE-400 frame is J2000.
    //
    // The transpose of this is the frame from DE140 to DE400. To get
    // the euler angles below, the matrix given above was copied into
    // a matrix XFORM.
    //
    // This matrix was transposed to give the transformation from
    // DE-140 to J2000.
    //
    //    CALL XPOSE ( XFORM, XFORM )
    //
    // Using the SPICE routine M2EUL, the euler representation of the
    // transformation from DE140 to J2000 was constructed.
    //
    //    CALL M2EUL ( XFORM, 3, 2, 3, A1, A2, A3 )
    //
    // Angles were converted to the range from -180 to 180 degrees
    // and converted to arcseconds.  At this point we have the
    // euler representation from DE-140 to J2000.
    //
    //    [ A1 ]  [ A2 ]  [ A3 ]
    //          3       2       3
    //
    // To get the Euler representation of the transformation from
    // J2000 to DE-140  we use.
    //
    //    [ -A3 ]  [ -A2 ] [ -A1 ]
    //           3        2       3
    //
    // This method was used because it yields a nicer form of
    // representation than the straight forward transformation.
    // Note that these numbers are quite close to the values used
    // for the transformation from J2000 to B1950
    //

    //
    // The frame for DE-142 is simply DE-402 rotated by the rotation:
    //
    //  0.9999256765402605  0.0111817697320531  0.0048589526815484
    // -0.0111817697907755  0.9999374816892126 -0.0000271547693170
    // -0.0048589525464121 -0.0000271789392288  0.9999881948510477
    //
    // Note that the DE-402 frame is J2000.
    //
    // The Euler angles giving the transformation for J2000 to
    // DE-142 were constructed in the same way as the transformation
    // from J2000 to DE140.  Only the input matrix changed to use the
    // one given above.
    //

    //
    // The frame for DE-143 is simply DE-403 rotated by the rotation:
    //
    //  0.9999256765435852  0.0111817743077255  0.0048589414674762
    // -0.0111817743300355  0.9999374816382505 -0.0000271622115251
    // -0.0048589414161348 -0.0000271713942366  0.9999881949053349
    //
    // Note that the DE-403 frame is J2000.
    //
    // The Euler angles giving the transformation for J2000 to
    // DE-143 were constructed in the same way as the transformation
    // from J2000 to DE140.  Only the input matrix changed to use the
    // one given above.
    //

    //
    // Until defined (by a call to IRFDEF), the default frame is
    // undefined.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CHGIRF", ctx)?;
    }

    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"CHGIRF", ctx)?;
    Ok(())
}

/// Inertial reference frame rotation
///
/// Compute the matrix needed to rotate vectors between two
/// standard inertial reference frames.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  REFA,
///  REFB       I   Indices of target reference frames (A,B).
///  ROTAB      O   Rotation from frame A to frame B.
/// ```
///
/// # Detailed Input
///
/// ```text
///  REFA,
///  REFB     are the indices of two standard inertial reference
///           frames. The complete set of supported frames is shown
///           below.
///
///              Index  Name      Description
///              -----  --------  --------------------------------
///               1    J2000      Earth mean equator, dynamical
///                               equinox of J2000
///
///               2    B1950      Earth mean equator, dynamical
///                               equinox of B1950
///
///               3    FK4        Fundamental Catalog (4)
///
///               4    DE-118     JPL Developmental Ephemeris (118)
///
///               5    DE-96      JPL Developmental Ephemeris ( 96)
///
///               6    DE-102     JPL Developmental Ephemeris (102)
///
///               7    DE-108     JPL Developmental Ephemeris (108)
///
///               8    DE-111     JPL Developmental Ephemeris (111)
///
///               9    DE-114     JPL Developmental Ephemeris (114)
///
///              10    DE-122     JPL Developmental Ephemeris (122)
///
///              11    DE-125     JPL Developmental Ephemeris (125)
///
///              12    DE-130     JPL Developmental Ephemeris (130)
///
///              13    GALACTIC   Galactic System II
///
///              14    DE-200     JPL Developmental Ephemeris (200)
///
///              15    DE-202     JPL Developmental Ephemeris (202)
///
///              16    MARSIAU    Mars Observer inertial frame
///                               defined relative to MARS.
///
///              17    ECLIPJ2000 Earth mean ecliptic and equinox
///                               of the epoch J2000
///
///              18    ECLIPB1950 Earth mean ecliptic and equinox
///                               of the Besselian date 1950.
///
///              19    DE-140    JPL Developmental Ephemeris (140)
///
///              20    DE-142    JPL Developmental Ephemeris (142)
///
///              21    DE-143    JPL Developmental Ephemeris (143)
/// ```
///
/// # Detailed Output
///
/// ```text
///  ROTAB    is the rotation which, when applied to a vector v
///           in reference frame A,
///              _            _
///              v  = (ROTAB) v
///               B            A
///
///           yields the same vector in reference frame B. The
///           inverse rotation is performed by applying the
///           transpose,
///              _           T _
///              v  = (ROTAB)  v
///               A             B
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If either REFA or REFB is outside the range [1,MAXF],
///      where MAXF is the number of supported frames, the error
///      SPICE(IRFNOTREC) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  IRFROT exists primarily for use by the ephemeris and star
///  catalog readers in the SPICELIB toolkit library.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, IRFROT is used to rotate
///  vectors originally referenced to the DE-118 coordinate frame
///  to equivalent vectors referenced to the IAU standard J2000
///  reference frame.
///
///     CALL IRFROT ( 4, 1, R )
///
///     CALL MXV ( R, SC1950, SC2000 )
///     CALL MXV ( R, MP1950, MP2000 )
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Lieske, "Precession Matrix Based on IAU (1976) System of
///       Astronomical Constants," Astron. Astrophys. 73, 282-284,
///       1979.
///
///  [2]  E. M. Standish, Jr., "Orientation of the JPL Ephemerides,
///       DE 200/LE 200, to the Dynamical Equinox of J2000," Astron.
///       Astrophys. 114, 297-302, 1982.
///
///  [3]  E. M. Standish, Jr., "Conversion of Ephemeris Coordinates
///       from the B1950 System to the J2000 System," JPL IOM
///       314.6-581, 24 June 1985.
///
///  [4]  E. M. Standish, Jr., "The Equinox Offsets of the JPL
///       Ephemeris," JPL IOM 314.6-929, 26 February 1988.
///
///  [5]  J. Lieske, "Expressions for the Precession Quantities Based
///       upon the IAU (1976) System of Astronomical Constants,"
///       Astron. Astrophys. 58, 1-16, 1977.
///
///  [6]  L. Bass and R. Cesarone, "Mars Observer Planetary Constants
///       and Models," JPL D-3444, November 1990.
///
///  [7]  P. Kenneth Seidelmann (Ed.), "Explanatory Supplement to the
///       Astronomical Almanac," University Science Books, 1992.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.4.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Corrected
///         argument name in $Brief_I/O section: ROTAB was MATRIX. Added
///         $Literature_References.
///
/// -    SPICELIB Version 4.3.0, 24-SEP-2013 (BVS)
///
///         Updated to do discovery check-in/check-out.
///
/// -    SPICELIB Version 4.2.1, 04-JAN-2002 (EDW)
///
///         Added DE-143 to header description for IRFROT.
///
/// -    SPICELIB Version 4.2.0, 10-APR-1997 (WLT)
///
///         A descriptive diagnostic was added to the entry points
///         IRFROT and IRFDEF. Before they simply signaled the error
///         with no diagnostic.
///
/// -    SPICELIB Version 4.1.0, 14-OCT-1996 (WLT)
///
///         The number of inertial frames recognized is now stored
///         in the include file ninert.inc.
///
/// -    SPICELIB Version 4.0.0, 20-MAY-1996 (WLT)
///
///         The inertial frame DE-143 was added to the list of recognized
///         inertial frames.
///
/// -    SPICELIB Version 3.0.0, 20-MAR-1995 (WLT)
///
///         The inertial frames DE-140 and DE-142  were added to the
///         list of recognized inertial frames.
///
/// -    SPICELIB Version 2.0.0, 30-JUL-1993 (WLT)
///
///         The transformation from J2000 to B1950 was upgraded
///         so that the transformation matrix produced matches
///         the matrix given in [1].
///
///         The frame MARSIAU was added to the list
///         of recognized frames. This is the standard mars
///         referenced inertial frame used by the Mars Observer
///         project.
///
///         Values for the obliquity of the ecliptic were taken
///         from the Explanatory Supplement [7] to the Astronomical
///         Almanac (1992) at both the epochs J2000 and B1950 and
///         used to define the mean ecliptic and equinox frames
///         ECLIPJ2000 and ECLIPB1950.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn irfrot(
    ctx: &mut SpiceContext,
    refa: i32,
    refb: i32,
    rotab: &mut [[f64; 3]; 3],
) -> crate::Result<()> {
    IRFROT(refa, refb, rotab.as_flattened_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure IRFROT ( Inertial reference frame rotation )
pub fn IRFROT(
    REFA: i32,
    REFB: i32,
    ROTAB: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ROTAB = DummyArrayMut2D::new(ROTAB, 1..=3, 1..=3);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // If it has not been done already, construct the transformation
    // from the root frame to each supported reference frame.
    //
    // Begin by constructing the identity matrix (rotating by zero
    // radians about the x-axis). Apply the rotations indicated in
    // the frame definition (from right to left) to get the incremental
    // rotation from the base frame. The final rotation is
    //
    //    R             = (R           ) (R          )
    //     root->frame      base->frame    root->base
    //
    if !save.READY {
        CHKIN(b"IRFROT", ctx)?;

        for I in 1..=MAXF {
            ROTATE(0.0, 1, save.TRANS.subarray_mut([1, I]), ctx);

            for J in intrinsics::range(WDCNT(&save.DEFS[I]), 2, -2) {
                NTHWD(&save.DEFS[I], J, &mut save.WORD, &mut save.LOC);
                NPARSI(
                    &save.WORD,
                    &mut save.AXIS,
                    &mut save.ERROR,
                    &mut save.P,
                    ctx,
                );

                NTHWD(&save.DEFS[I], (J - 1), &mut save.WORD, &mut save.LOC);
                NPARSD(
                    &save.WORD,
                    &mut save.ANGLE,
                    &mut save.ERROR,
                    &mut save.P,
                    ctx,
                );

                CONVRT(save.ANGLE, b"ARCSECONDS", b"RADIANS", &mut save.RADANG, ctx)?;

                ROTMAT(
                    save.TRANS.subarray([1, I]),
                    save.RADANG,
                    save.AXIS,
                    save.TMPMAT.as_slice_mut(),
                    ctx,
                );
                MOVED(save.TMPMAT.as_slice(), 9, save.TRANS.subarray_mut([1, I]));
            }

            save.B = ISRCHC(&save.BASES[I], I, save.FRAMES.as_arg());

            MXM(
                save.TRANS.subarray([1, I]),
                save.TRANS.subarray([1, save.B]),
                save.TMPMAT.as_slice_mut(),
            );
            MOVED(save.TMPMAT.as_slice(), 9, save.TRANS.subarray_mut([1, I]));
        }

        CHKOUT(b"IRFROT", ctx)?;

        save.READY = true;
    }

    //
    // If the transformations have been defined, we can proceed with
    // the business at hand: determining the rotation from one frame
    // to another. To get from frame A to frame B, the rotation is
    //
    //                                 T
    //    R     = (R       ) (R       )
    //     A->B     root->B    root->A
    //
    // If A and B are the same frame, the rotation is just the identity.
    // In theory, computing
    //
    //                                 T
    //    R     = (R       ) (R       )
    //     A->A     root->A    root->A
    //
    // should work, but why risk roundoff problems?
    //
    if ((REFA < 1) || (REFA > MAXF)) {
        CHKIN(b"IRFROT", ctx)?;
        SETMSG(b"A request has been made to obtain the transformation from inertial reference frame # to inertial reference frame #. Unfortunately # is not the id-code of a known inertial frame. ", ctx);
        ERRINT(b"#", REFA, ctx);
        ERRINT(b"#", REFB, ctx);
        ERRINT(b"#", REFA, ctx);
        SIGERR(b"SPICE(IRFNOTREC)", ctx)?;
        CHKOUT(b"IRFROT", ctx)?;
    } else if ((REFB < 1) || (REFB > MAXF)) {
        CHKIN(b"IRFROT", ctx)?;
        SETMSG(b"A request has been made to obtain the transformation from inertial reference frame # to inertial reference frame #. Unfortunately # is not the id-code of a known inertial frame. ", ctx);
        ERRINT(b"#", REFA, ctx);
        ERRINT(b"#", REFB, ctx);
        ERRINT(b"#", REFB, ctx);
        SIGERR(b"SPICE(IRFNOTREC)", ctx)?;
        CHKOUT(b"IRFROT", ctx)?;
    } else if (REFA == REFB) {
        ROTATE(0.0, 1, ROTAB.as_slice_mut(), ctx);
    } else {
        MXMT(
            save.TRANS.subarray([1, REFB]),
            save.TRANS.subarray([1, REFA]),
            ROTAB.as_slice_mut(),
        );
    }

    Ok(())
}

/// Inertial reference frame number
///
/// Return the index of one of the standard inertial reference
/// frames supported by IRFROT.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NAME       I   Name of standard inertial reference frame.
///  INDEX      O   Index of frame.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of one of the standard inertial
///           reference frames supported by IRFROT, or
///           'DEFAULT'.
/// ```
///
/// # Detailed Output
///
/// ```text
///  INDEX    is the index of the frame specified by NAME.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If NAME is not recognized, INDEX is zero.
///
///  2)  If no default frame has been specified, INDEX is zero.
/// ```
///
/// # Particulars
///
/// ```text
///  IRFNUM is supplied as a convenience, to allow users to refer to
///  the various standard inertial reference frames by name.
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, the rotation from DE-118 to FK4 is
///  computed without knowing the indices of these frames.
///
///     CALL IRFNUM ( 'DE-118', A )
///     CALL IRFNUM ( 'FK4',    B )
///
///     CALL IRFROT ( A, B, ROTAB )
///
///  IRFNUM can be used to rotate vectors into the default frame,
///  as illustrated by the following code fragment.
///
///     CALL IRFNUM ( 'FK4',     A )
///     CALL IRFNUM ( 'DEFAULT', B )
///
///     CALL IRFROT ( A, B, ROTAB                 )
///     CALL MXV    (       ROTAB, OLDVEC, NEWVEC )
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Lieske, "Precession Matrix Based on IAU (1976) System of
///       Astronomical Constants," Astron. Astrophys. 73, 282-284,
///       1979.
///
///  [2]  E. M. Standish, Jr., "Orientation of the JPL Ephemerides,
///       DE 200/LE 200, to the Dynamical Equinox of J2000," Astron.
///       Astrophys. 114, 297-302, 1982.
///
///  [3]  E. M. Standish, Jr., "Conversion of Ephemeris Coordinates
///       from the B1950 System to the J2000 System," JPL IOM
///       314.6-581, 24 June 1985.
///
///  [4]  E. M. Standish, Jr., "The Equinox Offsets of the JPL
///       Ephemeris," JPL IOM 314.6-929, 26 February 1988.
///
///  [5]  J. Lieske, "Expressions for the Precession Quantities Based
///       upon the IAU (1976) System of Astronomical Constants,"
///       Astron. Astrophys. 58, 1-16, 1977.
///
///  [6]  L. Bass and R. Cesarone, "Mars Observer Planetary Constants
///       and Models," JPL D-3444, November 1990.
///
///  [7]  P. Kenneth Seidelmann (Ed.), "Explanatory Supplement to the
///       Astronomical Almanac," University Science Books, 1992.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.4.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Literature_References.
///
/// -    SPICELIB Version 4.3.0, 24-SEP-2013 (BVS)
///
///         Updated to treat J2000 as a special case and to not
///         CHKIN/CHOUT to increase efficiency.
///
/// -    SPICELIB Version 4.2.1, 04-JAN-2002 (EDW)
///
///         Added DE-143 to header description for IRFROT.
///
/// -    SPICELIB Version 4.2.0, 10-APR-1997 (WLT)
///
///         A descriptive diagnostic was added to the entry points
///         IRFROT and IRFDEF. Before they simply signaled the error
///         with no diagnostic.
///
/// -    SPICELIB Version 4.1.0, 14-OCT-1996 (WLT)
///
///         The number of inertial frames recognized is now stored
///         in the include file ninert.inc.
///
/// -    SPICELIB Version 4.0.0, 20-MAY-1996 (WLT)
///
///         The inertial frame DE-143 was added to the list of recognized
///         inertial frames.
///
/// -    SPICELIB Version 3.0.0, 20-MAR-1995 (WLT)
///
///         The inertial frames DE-140 and DE-142  were added to the
///         list of recognized inertial frames.
///
/// -    SPICELIB Version 2.0.0, 30-JUL-1993 (WLT)
///
///         The transformation from J2000 to B1950 was upgraded
///         so that the transformation matrix produced matches
///         the matrix given in [1].
///
///         The frame MARSIAU was added to the list
///         of recognized frames. This is the standard mars
///         referenced inertial frame used by the Mars Observer
///         project.
///
///         Values for the obliquity of the ecliptic were taken
///         from the Explanatory Supplement [7] to the Astronomical
///         Almanac (1992) at both the epochs J2000 and B1950 and
///         used to define the mean ecliptic and equinox frames
///         ECLIPJ2000 and ECLIPB1950.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn irfnum(ctx: &mut SpiceContext, name: &str, index: &mut i32) {
    IRFNUM(name.as_bytes(), index, ctx.raw_context());
}

//$Procedure IRFNUM ( Inertial reference frame number )
pub fn IRFNUM(NAME: &[u8], INDEX: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return;
    }

    if (fstr::eq(NAME, b"J2000") || fstr::eq(NAME, b"j2000")) {
        *INDEX = 1;
        return;
    }

    if EQSTR(NAME, b"DEFAULT") {
        *INDEX = save.DFRAME;
    } else {
        *INDEX = ESRCHC(NAME, MAXF, save.FRAMES.as_arg());
    }
}

/// Inertial reference frame name
///
/// Return the name of one of the standard inertial reference
/// frames supported by IRFROT.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  INDEX      I   Index of standard inertial reference frame.
///  NAME       O   Name of frame.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INDEX    is the index of one of the standard inertial
///           reference frames supported by IRFROT.
/// ```
///
/// # Detailed Output
///
/// ```text
///  NAME     is the name of the frame specified by INDEX.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If INDEX is not the index of a supported frame, NAME is blank.
/// ```
///
/// # Particulars
///
/// ```text
///  IRFNAM is supplied as a convenience, to allow users to determine
///  the names of standard inertial reference frames referred to only
///  by index (as in the segment descriptors of a GEF ephemeris file).
/// ```
///
/// # Examples
///
/// ```text
///  In the following example, the identity of a rotation from DE-118
///  to FK4 is deduced from the indices used to create the rotation.
///
///     CALL IRFROT ( A, B, ROTAB )
///
///     CALL IRFNAM ( A, NAME(1) )
///     CALL IRFNAM ( B, NAME(2) )
///
///     WRITE (6,*) 'Rotation from ' // NAME(1) // ' to ' // NAME(2)
///
///  Note that the name of the default reference frame can only be
///  recovered from the number:
///
///     CALL IRFNUM ( 'DEFAULT', DINDEX        )
///     CALL IRFNAM (            DINDEX, DNAME )
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Lieske, "Precession Matrix Based on IAU (1976) System of
///       Astronomical Constants," Astron. Astrophys. 73, 282-284,
///       1979.
///
///  [2]  E. M. Standish, Jr., "Orientation of the JPL Ephemerides,
///       DE 200/LE 200, to the Dynamical Equinox of J2000," Astron.
///       Astrophys. 114, 297-302, 1982.
///
///  [3]  E. M. Standish, Jr., "Conversion of Ephemeris Coordinates
///       from the B1950 System to the J2000 System," JPL IOM
///       314.6-581, 24 June 1985.
///
///  [4]  E. M. Standish, Jr., "The Equinox Offsets of the JPL
///       Ephemeris," JPL IOM 314.6-929, 26 February 1988.
///
///  [5]  J. Lieske, "Expressions for the Precession Quantities Based
///       upon the IAU (1976) System of Astronomical Constants,"
///       Astron. Astrophys. 58, 1-16, 1977.
///
///  [6]  L. Bass and R. Cesarone, "Mars Observer Planetary Constants
///       and Models," JPL D-3444, November 1990.
///
///  [7]  P. Kenneth Seidelmann (Ed.), "Explanatory Supplement to the
///       Astronomical Almanac," University Science Books, 1992.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.3.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Literature_References.
///
/// -    SPICELIB Version 4.2.1, 04-JAN-2002 (EDW)
///
///         Added DE-143 to header description for IRFROT.
///
/// -    SPICELIB Version 4.2.0, 10-APR-1997 (WLT)
///
///         A descriptive diagnostic was added to the entry points
///         IRFROT and IRFDEF. Before they simply signaled the error
///         with no diagnostic.
///
/// -    SPICELIB Version 4.1.0, 14-OCT-1996 (WLT)
///
///         The number of inertial frames recognized is now stored
///         in the include file ninert.inc.
///
/// -    SPICELIB Version 4.0.0, 20-MAY-1996 (WLT)
///
///         The inertial frame DE-143 was added to the list of recognized
///         inertial frames.
///
/// -    SPICELIB Version 3.0.0, 20-MAR-1995 (WLT)
///
///         The inertial frames DE-140 and DE-142  were added to the
///         list of recognized inertial frames.
///
/// -    SPICELIB Version 2.0.0, 30-JUL-1993 (WLT)
///
///         The transformation from J2000 to B1950 was upgraded
///         so that the transformation matrix produced matches
///         the matrix given in [1].
///
///         The frame MARSIAU was added to the list
///         of recognized frames. This is the standard mars
///         referenced inertial frame used by the Mars Observer
///         project.
///
///         Values for the obliquity of the ecliptic were taken
///         from the Explanatory Supplement [7] to the Astronomical
///         Almanac (1992) at both the epochs J2000 and B1950 and
///         used to define the mean ecliptic and equinox frames
///         ECLIPJ2000 and ECLIPB1950.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn irfnam(ctx: &mut SpiceContext, index: i32, name: &mut str) -> crate::Result<()> {
    IRFNAM(index, fstr::StrBytes::new(name).as_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure IRFNAM ( Inertial reference frame name )
pub fn IRFNAM(INDEX: i32, NAME: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"IRFNAM", ctx)?;
    }

    if ((INDEX < 1) || (INDEX > MAXF)) {
        fstr::assign(NAME, b" ");
    } else {
        fstr::assign(NAME, save.FRAMES.get(INDEX));
    }

    CHKOUT(b"IRFNAM", ctx)?;
    Ok(())
}

/// Inertial reference frame, default
///
/// Specify a standard inertial reference frame as the default
/// frame for a program.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  INDEX      I   Index of default frame.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INDEX    is the index of one of the standard inertial
///           reference frames supported by IRFROT.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If INDEX is outside the range [1,MAXF], where MAXF is the
///      number of supported frames, the error SPICE(IRFNOTREC) is
///      signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  IRFDEF allows tools to be written at a relatively high level
///  without requiring the reference frame to be tramp coupled or
///  placed in global memory.
/// ```
///
/// # Examples
///
/// ```text
///  Typically, the calling program will select a default frame
///  during initialization,
///
///     C
///     C     Use J2000 for all ephemeris, star data.
///     C
///           CALL IRFDEF ( 1 )
///
///  and recover the default frame at lower levels,
///
///     C
///     C     Rotate all vectors into the default frame.
///     C
///           CALL IRFNUM ( 'DEFAULT', REFD   )
///
///           DO I = 1, NVEC
///              CALL IRFROT ( REFIN, REFD, ROT           )
///              CALL MXV                   ROT, VEC, VEC )
///           END DO
///
///  Note that many utilities accept 'DEFAULT' as the name of
///  an inertial reference frame,
///
///     CALL SPKEZ ( TARGET, ..., 'DEFAULT', ... )
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  J. Lieske, "Precession Matrix Based on IAU (1976) System of
///       Astronomical Constants," Astron. Astrophys. 73, 282-284,
///       1979.
///
///  [2]  E. M. Standish, Jr., "Orientation of the JPL Ephemerides,
///       DE 200/LE 200, to the Dynamical Equinox of J2000," Astron.
///       Astrophys. 114, 297-302, 1982.
///
///  [3]  E. M. Standish, Jr., "Conversion of Ephemeris Coordinates
///       from the B1950 System to the J2000 System," JPL IOM
///       314.6-581, 24 June 1985.
///
///  [4]  E. M. Standish, Jr., "The Equinox Offsets of the JPL
///       Ephemeris," JPL IOM 314.6-929, 26 February 1988.
///
///  [5]  J. Lieske, "Expressions for the Precession Quantities Based
///       upon the IAU (1976) System of Astronomical Constants,"
///       Astron. Astrophys. 58, 1-16, 1977.
///
///  [6]  L. Bass and R. Cesarone, "Mars Observer Planetary Constants
///       and Models," JPL D-3444, November 1990.
///
///  [7]  P. Kenneth Seidelmann (Ed.), "Explanatory Supplement to the
///       Astronomical Almanac," University Science Books, 1992.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.3.0, 17-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added
///         $Literature_References.
///
/// -    SPICELIB Version 4.2.1, 04-JAN-2002 (EDW)
///
///         Added DE-143 to header description for IRFROT.
///
/// -    SPICELIB Version 4.2.0, 10-APR-1997 (WLT)
///
///         A descriptive diagnostic was added to the entry points
///         IRFROT and IRFDEF. Before they simply signaled the error
///         with no diagnostic.
///
/// -    SPICELIB Version 4.1.0, 14-OCT-1996 (WLT)
///
///         The number of inertial frames recognized is now stored
///         in the include file ninert.inc.
///
/// -    SPICELIB Version 4.0.0, 20-MAY-1996 (WLT)
///
///         The inertial frame DE-143 was added to the list of recognized
///         inertial frames.
///
/// -    SPICELIB Version 3.0.0, 20-MAR-1995 (WLT)
///
///         The inertial frames DE-140 and DE-142  were added to the
///         list of recognized inertial frames.
///
/// -    SPICELIB Version 2.0.0, 30-JUL-1993 (WLT)
///
///         The transformation from J2000 to B1950 was upgraded
///         so that the transformation matrix produced matches
///         the matrix given in [1].
///
///         The frame MARSIAU was added to the list
///         of recognized frames. This is the standard mars
///         referenced inertial frame used by the Mars Observer
///         project.
///
///         Values for the obliquity of the ecliptic were taken
///         from the Explanatory Supplement [7] to the Astronomical
///         Almanac (1992) at both the epochs J2000 and B1950 and
///         used to define the mean ecliptic and equinox frames
///         ECLIPJ2000 and ECLIPB1950.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn irfdef(ctx: &mut SpiceContext, index: i32) -> crate::Result<()> {
    IRFDEF(index, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure IRFDEF ( Inertial reference frame, default )
pub fn IRFDEF(INDEX: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"IRFDEF", ctx)?;
    }

    //
    // There's not much to do, except save the value for later use.
    //
    if ((INDEX < 1) || (INDEX > MAXF)) {
        SETMSG(
            b"The reference frame with id-code # is not a recognized inertial reference frame. ",
            ctx,
        );
        ERRINT(b"#", INDEX, ctx);

        SIGERR(b"SPICE(IRFNOTREC)", ctx)?;
    } else {
        save.DFRAME = INDEX;
    }

    CHKOUT(b"IRFDEF", ctx)?;
    Ok(())
}
