//
// GENERATED FILE
//

use super::*;
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

//$Procedure CHGIRF_O ( Change inertial reference frames )
pub fn CHGIRF_O(
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
    // by CHGIRF_O must be updated whenever new frames are added.
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
    // The values for z, theta, and zeta are taken directly from
    // are computed from the formulas given in table 5 of [5].
    //
    //    z     =  1153.04066200330"
    //    theta = -1002.26108439117"
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
    // Until defined (by a call to IRFDEF_O), the default frame is
    // undefined.
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"CHGIRF_O", ctx)?;
    }

    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    spicelib::CHKOUT(b"CHGIRF_O", ctx)?;
    Ok(())
}

//$Procedure IRFROT_O ( Inertial reference frame rotation )
pub fn IRFROT_O(
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
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"IRFROT_O", ctx)?;
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
        for I in 1..=MAXF {
            spicelib::ROTATE(0.0, 1, save.TRANS.subarray_mut([1, I]), ctx);

            for J in intrinsics::range(spicelib::WDCNT(&save.DEFS[I]), 2, -2) {
                spicelib::NTHWD(&save.DEFS[I], J, &mut save.WORD, &mut save.LOC);
                spicelib::NPARSI(
                    &save.WORD,
                    &mut save.AXIS,
                    &mut save.ERROR,
                    &mut save.P,
                    ctx,
                );

                spicelib::NTHWD(&save.DEFS[I], (J - 1), &mut save.WORD, &mut save.LOC);
                spicelib::NPARSD(
                    &save.WORD,
                    &mut save.ANGLE,
                    &mut save.ERROR,
                    &mut save.P,
                    ctx,
                );

                spicelib::CONVRT(save.ANGLE, b"ARCSECONDS", b"RADIANS", &mut save.RADANG, ctx)?;

                spicelib::ROTMAT(
                    save.TRANS.subarray([1, I]),
                    save.RADANG,
                    save.AXIS,
                    save.TMPMAT.as_slice_mut(),
                    ctx,
                );
                spicelib::MOVED(save.TMPMAT.as_slice(), 9, save.TRANS.subarray_mut([1, I]));
            }

            save.B = spicelib::ISRCHC(&save.BASES[I], I, save.FRAMES.as_arg());

            spicelib::MXM(
                save.TRANS.subarray([1, I]),
                save.TRANS.subarray([1, save.B]),
                save.TMPMAT.as_slice_mut(),
            );
            spicelib::MOVED(save.TMPMAT.as_slice(), 9, save.TRANS.subarray_mut([1, I]));
        }

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
        spicelib::SETMSG(b"A request has been made to obtain the transformation from inertial reference frame # to inertial reference frame #. Unfortunately # is not the id-code of a known inertial frame. ", ctx);
        spicelib::ERRINT(b"#", REFA, ctx);
        spicelib::ERRINT(b"#", REFB, ctx);
        spicelib::ERRINT(b"#", REFA, ctx);
        spicelib::SIGERR(b"SPICE(IRFNOTREC)", ctx)?;
    } else if ((REFB < 1) || (REFB > MAXF)) {
        spicelib::SETMSG(b"A request has been made to obtain the transformation from inertial reference frame # to inertial reference frame #. Unfortunately # is not the id-code of a known inertial frame. ", ctx);
        spicelib::ERRINT(b"#", REFA, ctx);
        spicelib::ERRINT(b"#", REFB, ctx);
        spicelib::ERRINT(b"#", REFB, ctx);
        spicelib::SIGERR(b"SPICE(IRFNOTREC)", ctx)?;
    } else if (REFA == REFB) {
        spicelib::ROTATE(0.0, 1, ROTAB.as_slice_mut(), ctx);
    } else {
        spicelib::MXMT(
            save.TRANS.subarray([1, REFB]),
            save.TRANS.subarray([1, REFA]),
            ROTAB.as_slice_mut(),
        );
    }

    spicelib::CHKOUT(b"IRFROT_O", ctx)?;
    Ok(())
}

//$Procedure IRFNUM_O ( Inertial reference frame number )
pub fn IRFNUM_O(NAME: &[u8], INDEX: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"IRFNUM_O", ctx)?;
    }

    if spicelib::EQSTR(NAME, b"DEFAULT") {
        *INDEX = save.DFRAME;
    } else {
        *INDEX = spicelib::ESRCHC(NAME, MAXF, save.FRAMES.as_arg());
    }

    spicelib::CHKOUT(b"IRFNUM_O", ctx)?;
    Ok(())
}

//$Procedure IRFNAM_O ( Inertial reference frame name )
pub fn IRFNAM_O(INDEX: i32, NAME: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"IRFNAM_O", ctx)?;
    }

    if ((INDEX < 1) || (INDEX > MAXF)) {
        fstr::assign(NAME, b" ");
    } else {
        fstr::assign(NAME, save.FRAMES.get(INDEX));
    }

    spicelib::CHKOUT(b"IRFNAM_O", ctx)?;
    Ok(())
}

//$Procedure IRFDEF_O ( Inertial reference frame, default )
pub fn IRFDEF_O(INDEX: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"IRFDEF_O", ctx)?;
    }

    //
    // There's not much to do, except save the value for later use.
    //
    if ((INDEX < 1) || (INDEX > MAXF)) {
        spicelib::SETMSG(
            b"The reference frame with id-code # is not a recognized inertial reference frame. ",
            ctx,
        );
        spicelib::ERRINT(b"#", INDEX, ctx);

        spicelib::SIGERR(b"SPICE(IRFNOTREC)", ctx)?;
    } else {
        save.DFRAME = INDEX;
    }

    spicelib::CHKOUT(b"IRFDEF_O", ctx)?;
    Ok(())
}
