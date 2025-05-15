//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const KVNMLN: i32 = 32;
const KVLEN: i32 = 80;
const FRNMLN: i32 = 32;
const BDNMLN: i32 = 36;
const MAXCOF: i32 = 20;
const MXNFAC: i32 = 10;
const LBSEP: f64 = 0.001;
const QEXP: i32 = -27;
const KWBFRM: &[u8] = b"RELATIVE";
const KWSTYL: &[u8] = b"DEF_STYLE";
const KVPARM: &[u8] = b"PARAMETERIZED";
const KWFREZ: &[u8] = b"FREEZE_EPOCH";
const KWRSTA: &[u8] = b"ROTATION_STATE";
const KVROTG: &[u8] = b"ROTATING";
const KVINRT: &[u8] = b"INERTIAL";
const KWFFAM: &[u8] = b"FAMILY";
const KVMEQT: &[u8] = b"MEAN_EQUATOR_AND_EQUINOX_OF_DATE";
const KVMECL: &[u8] = b"MEAN_ECLIPTIC_AND_EQUINOX_OF_DATE";
const KVTEQT: &[u8] = b"TRUE_EQUATOR_AND_EQUINOX_OF_DATE";
const KV2VEC: &[u8] = b"TWO-VECTOR";
const KVEULR: &[u8] = b"EULER";
const KVPROD: &[u8] = b"PRODUCT";
const KWPRCM: &[u8] = b"PREC_MODEL";
const KWNUTM: &[u8] = b"NUT_MODEL";
const KWOBQM: &[u8] = b"OBLIQ_MODEL";
const KVM001: &[u8] = b"EARTH_IAU_1976";
const KVM002: &[u8] = b"EARTH_IAU_1980";
const KVM003: &[u8] = b"EARTH_IAU_1980";
const KWVAXI: &[u8] = b"AXIS";
const KVX: &[u8] = b"X";
const KVY: &[u8] = b"Y";
const KVZ: &[u8] = b"Z";
const KWPRI: &[u8] = b"PRI_";
const KWSEC: &[u8] = b"SEC_";
const KWVCDF: &[u8] = b"VECTOR_DEF";
const KVPOSV: &[u8] = b"OBSERVER_TARGET_POSITION";
const KVVELV: &[u8] = b"OBSERVER_TARGET_VELOCITY";
const KVNEAR: &[u8] = b"TARGET_NEAR_POINT";
const KVCONS: &[u8] = b"CONSTANT";
const KWVOBS: &[u8] = b"OBSERVER";
const KWVTRG: &[u8] = b"TARGET";
const KWVFRM: &[u8] = b"FRAME";
const KWVABC: &[u8] = b"ABCORR";
const KWVSPC: &[u8] = b"SPEC";
const KVRECC: &[u8] = b"RECTANGULAR";
const KVLATC: &[u8] = b"LATITUDINAL";
const KVRADC: &[u8] = b"RA/DEC";
const KWVECT: &[u8] = b"VECTOR";
const KWLAT: &[u8] = b"LATITUDE";
const KWLON: &[u8] = b"LONGITUDE";
const KWRA: &[u8] = b"RA";
const KWDEC: &[u8] = b"DEC";
const KWATOL: &[u8] = b"ANGLE_SEP_TOL";
const KWEPOC: &[u8] = b"EPOCH";
const KWEUAX: &[u8] = b"AXES";
const KWEAC1: &[u8] = b"ANGLE_1_COEFFS";
const KWEAC2: &[u8] = b"ANGLE_2_COEFFS";
const KWEAC3: &[u8] = b"ANGLE_3_COEFFS";
const KWFFRM: &[u8] = b"FROM_FRAMES";
const KWTFRM: &[u8] = b"TO_FRAMES";
const KWUNIT: &[u8] = b"UNITS";
const KVRADN: &[u8] = b"RADIANS";
const KVDEGR: &[u8] = b"DEGREES";
const OTP: &[u8] = b"OBSERVER_TARGET_POSITION";
const OTV: &[u8] = b"OBSERVER_TARGET_VELOCITY";
const PCK: &[u8] = b"test_dyn.tpc";
const SPK: &[u8] = b"test_dyn.bsp";
const DM16: f64 = 0.0000000000000001;
const DM15: f64 = 0.000000000000001;
const DM14: f64 = 0.00000000000001;
const DM12: f64 = 0.000000000001;
const DM10: f64 = 0.0000000001;
const DM9: f64 = 0.000000001;
const DM7: f64 = 0.0000001;
const DM6: f64 = 0.000001;
const DM5: f64 = 0.00001;
const DM4: f64 = 0.0001;
const DM3: f64 = 0.001;
const DM2: f64 = 0.01;
const DM1: f64 = 0.1;
const ABCLEN: i32 = 15;
const NAMLEN: i32 = 32;
const LNSIZE: i32 = 80;
const MAXDEF: i32 = 50;
const MAXTOK: i32 = 7;
const NDIMS: i32 = 5;
const NABCRR: i32 = 11;
const NABPRS: i32 = 5;
const ABCIDX: i32 = 1;
const NAXDEF: i32 = 6;
const NBASEF: i32 = 2;
const BFRIDX: i32 = (ABCIDX + 1);
const NBODS: i32 = 5;
const NOTPRS: i32 = 5;
const OTPIDX: i32 = (BFRIDX + 1);
const NVECDF: i32 = 4;
const NVECFR: i32 = 5;
const NXVCDF: i32 = ((NVECDF - 2) + (2 * NVECFR));
const PRIIDX: i32 = (OTPIDX + 1);
const SECIDX: i32 = (PRIIDX + 1);
const ORDLEN: i32 = 4;
const NCOSYS: i32 = 3;
const NUNITS: i32 = 2;

struct SaveVars {
    ABCORR: ActualCharArray,
    ANGSTR: ActualCharArray2D,
    AXDEF: ActualCharArray2D,
    BASEFR: ActualCharArray,
    BODIES: ActualCharArray,
    CORSYS: ActualCharArray,
    UNITS: ActualCharArray,
    VECDEF: ActualCharArray,
    VECFRM: ActualCharArray,
    VECSTR: ActualCharArray,
    E: StackArray2D<f64, 9>,
    ZR: StackArray2D<f64, 9>,
    ABPAIR: StackArray2D<i32, 10>,
    DIMS: StackArray<i32, 5>,
    OTPAIR: StackArray2D<i32, 20>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = ActualCharArray::new(ABCLEN, 1..=NABCRR);
        let mut ANGSTR = ActualCharArray2D::new(LNSIZE, 1..=2, 1..=2);
        let mut AXDEF = ActualCharArray2D::new(LNSIZE, 1..=2, 1..=NAXDEF);
        let mut BASEFR = ActualCharArray::new(NAMLEN, 1..=NBASEF);
        let mut BODIES = ActualCharArray::new(NAMLEN, 1..=NBODS);
        let mut CORSYS = ActualCharArray::new(LNSIZE, 1..=NCOSYS);
        let mut UNITS = ActualCharArray::new(LNSIZE, 1..=NUNITS);
        let mut VECDEF = ActualCharArray::new(LNSIZE, 1..=NVECDF);
        let mut VECFRM = ActualCharArray::new(LNSIZE, 1..=NVECFR);
        let mut VECSTR = ActualCharArray::new(LNSIZE, 1..=3);
        let mut E = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut ZR = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut ABPAIR = StackArray2D::<i32, 10>::new(1..=2, 1..=NABPRS);
        let mut DIMS = StackArray::<i32, 5>::new(1..=NDIMS);
        let mut OTPAIR = StackArray2D::<i32, 20>::new(1..=4, 1..=NOTPRS);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"LT"),
                Val::C(b"LT+S"),
                Val::C(b"XLT"),
                Val::C(b"XLT+S"),
                Val::C(b"CN"),
                Val::C(b"CN+S"),
                Val::C(b"XCN"),
                Val::C(b"XCN+S"),
                Val::C(b"S"),
                Val::C(b"XS"),
            ]
            .into_iter();
            ABCORR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1),
                Val::I(1),
                Val::I(1),
                Val::I(5),
                Val::I(3),
                Val::I(1),
                Val::I(5),
                Val::I(3),
                Val::I(4),
                Val::I(4),
            ]
            .into_iter();
            ABPAIR
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"Z"),
                Val::C(b"X"),
                Val::C(b"-x"),
                Val::C(b" Y "),
                Val::C(b" Y "),
                Val::C(b"-z"),
                Val::C(b" -  y "),
                Val::C(b"  X"),
                Val::C(b" z"),
                Val::C(b" - x "),
                Val::C(b"- Z"),
                Val::C(b" - y "),
            ]
            .into_iter();
            AXDEF
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000"), Val::C(b"GSE")].into_iter();
            BASEFR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"PHOBOS"),
                Val::C(b"MARS"),
                Val::C(b"EARTH"),
                Val::C(b"MOON"),
                Val::C(b"SUN"),
            ]
            .into_iter();
            BODIES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"\'RECTANGULAR\'"),
                Val::C(b"\'LATITUDINAL\'"),
                Val::C(b"\'RA/DEC\'"),
            ]
            .into_iter();
            CORSYS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
            ]
            .into_iter();
            E.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1),
                Val::I(2),
                Val::I(1),
                Val::I(2),
                Val::I(1),
                Val::I(2),
                Val::I(1),
                Val::I(3),
                Val::I(1),
                Val::I(2),
                Val::I(3),
                Val::I(4),
                Val::I(3),
                Val::I(4),
                Val::I(3),
                Val::I(4),
                Val::I(3),
                Val::I(5),
                Val::I(3),
                Val::I(4),
            ]
            .into_iter();
            OTPAIR
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"OBSERVER_TARGET_POSITION"),
                Val::C(b"TARGET_NEAR_POINT"),
                Val::C(b"OBSERVER_TARGET_VELOCITY"),
                Val::C(b"CONSTANT"),
            ]
            .into_iter();
            VECDEF
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"J2000"),
                Val::C(b"GSE"),
                Val::C(b"PHOBOS_RADIAL"),
                Val::C(b"TARGET_FRAME"),
                Val::C(b"OBSERVER_FRAME"),
            ]
            .into_iter();
            VECFRM
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"( 1,  0,  1 )"),
                Val::C(b"( 1,  1,  0 )"),
                Val::C(b"( 0,  1,  1 )"),
            ]
            .into_iter();
            VECSTR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"-69.761"),
                Val::C(b" 78.565"),
                Val::C(b"-1"),
                Val::C(b" 1"),
            ]
            .into_iter();
            ANGSTR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"\'DEGREES\'"), Val::C(b"\'RADIANS\'")].into_iter();
            UNITS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(NABPRS),
                Val::I(NBASEF),
                Val::I(NOTPRS),
                Val::I(NXVCDF),
                Val::I(NXVCDF),
            ]
            .into_iter();
            DIMS.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 9 as usize))
                .chain([]);

            ZR.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            ANGSTR,
            AXDEF,
            BASEFR,
            BODIES,
            CORSYS,
            UNITS,
            VECDEF,
            VECFRM,
            VECSTR,
            E,
            ZR,
            ABPAIR,
            DIMS,
            OTPAIR,
        }
    }
}

//$Procedure F_DYN01 ( Dynamic Frame Test Family 01 )
pub fn F_DYN01(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut AXNAME = [b' '; LNSIZE as usize];
    let mut BFRAME = [b' '; NAMLEN as usize];
    let mut CASMSG = [b' '; LNSIZE as usize];
    let mut CENTER = [b' '; NAMLEN as usize];
    let mut COR = [b' '; ABCLEN as usize];
    let mut DEFTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut DEFTX2 = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut FRNAME = [b' '; NAMLEN as usize];
    let mut OBS = [b' '; NAMLEN as usize];
    let mut ORD = [b' '; ORDLEN as usize];
    let mut PRICOR = [b' '; ABCLEN as usize];
    let mut PRIFRM = [b' '; NAMLEN as usize];
    let mut PRIOBS = [b' '; NAMLEN as usize];
    let mut PRITRG = [b' '; NAMLEN as usize];
    let mut PRIVDF = [b' '; LNSIZE as usize];
    let mut PRIVF = [b' '; NAMLEN as usize];
    let mut SECCOR = [b' '; ABCLEN as usize];
    let mut SECFRM = [b' '; NAMLEN as usize];
    let mut SECOBS = [b' '; NAMLEN as usize];
    let mut SECTRG = [b' '; NAMLEN as usize];
    let mut SECVDF = [b' '; LNSIZE as usize];
    let mut SECVF = [b' '; NAMLEN as usize];
    let mut TARG = [b' '; NAMLEN as usize];
    let mut TMPSTR = [b' '; LNSIZE as usize];
    let mut TOKENS = ActualCharArray::new(LNSIZE, 1..=MAXTOK);
    let mut TRGFRM = [b' '; NAMLEN as usize];
    let mut VDF = [b' '; LNSIZE as usize];
    let mut VFR = [b' '; LNSIZE as usize];
    let mut VFRAME = [b' '; NAMLEN as usize];
    let mut FRSTEM = [b' '; 18 as usize];
    let mut ALT: f64 = 0.0;
    let mut AXSIGN: f64 = 0.0;
    let mut CLT: f64 = 0.0;
    let mut CONVEC = StackArray::<f64, 3>::new(1..=3);
    let mut CSTATE = StackArray::<f64, 6>::new(1..=6);
    let mut DELTA: f64 = 0.0;
    let mut DETERR: f64 = 0.0;
    let mut DMAG: f64 = 0.0;
    let mut DRDIFF = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DRLERR: f64 = 0.0;
    let mut DRVBLK = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DRVERR: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut ETCORR: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LAT0: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut LON0: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut MAXDER: f64 = 0.0;
    let mut NRMERR: f64 = 0.0;
    let mut POS = StackArray::<f64, 3>::new(1..=3);
    let mut PRIVEC = StackArray::<f64, 3>::new(1..=3);
    let mut R = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut R2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut R3 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut R4 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut RMINUS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut RPLUS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SECVEC = StackArray::<f64, 3>::new(1..=3);
    let mut SPOINT = StackArray::<f64, 3>::new(1..=3);
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut STOBS = StackArray::<f64, 6>::new(1..=6);
    let mut TOL: f64 = 0.0;
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut XF2000 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFB2J = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XF2 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut AXINDX: i32 = 0;
    let mut AXPAIR: i32 = 0;
    let mut CENT: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut COORDS = StackArray::<i32, 5>::new(1..=NDIMS);
    let mut CORIDX: i32 = 0;
    let mut DX: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut K: i32 = 0;
    let mut N: i32 = 0;
    let mut NCART: i32 = 0;
    let mut NCOR: i32 = 0;
    let mut NSKIP: i32 = 0;
    let mut PRIJ: i32 = 0;
    let mut SYSIDX: i32 = 0;
    let mut VFCODE: i32 = 0;
    let mut NPR: i32 = 0;
    let mut NOGO: i32 = 0;
    let mut BIGERR: bool = false;
    let mut FOUND: bool = false;
    let mut GO: bool = false;
    let mut ISCON = StackArray::<bool, 2>::new(1..=2);
    let mut ISVEL = StackArray::<bool, 2>::new(1..=2);
    let mut PRIBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut TSTDRV: bool = false;
    let mut SECBLK = StackArray::<bool, 15>::new(1..=NABCOR);

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Tolerance levels for various tests.
    //

    //
    // Parameters controlling frame definitions:
    //

    //
    // Number of dimensions of the test parameter space:
    //

    //
    // Number of aberration correction settings:
    //

    //
    // Number of aberration correction settings used for the
    // full cartesian product test:
    //
    // PARAMETER           ( NABPRS = 1 )

    //
    // The constant ABCIDX refers to the ordinal position of the
    // dimension corresponding to aberration correction in the parameter
    // space.
    //

    //
    // Number of axis definition cases:
    //

    //
    // Number of base frame cases:
    //

    //
    // The constant BFRIDX refers to the ordinal position of the
    // dimension corresponding to base frame in the parameter
    // space.
    //

    //
    // Number of bodies:
    //

    //
    // Number of observer-target body pairs:
    //

    //
    // Number of vector definitions:
    //

    //
    // Number of vector frames:
    //

    //
    // Number of extended vector definitions.  These definitions
    // include every combination of frame-dependent definition type
    // and frame (for example, velocity and observer frame, constant
    // and J2000 frame, etc.)
    //

    //
    // Other parameters
    //
    // Number of coordinate systems used to represent constant vectors:
    //

    //
    // Local Variables
    //

    //
    // We'll use the kernel variable name "stem"
    //
    //    FRAME_<ID code>_
    //
    // The length declared below (18) is the length of such a string
    // where the ID code contains 11 digits.
    //

    //
    // This variable is used for debugging.
    //

    //
    // OTPAIR is used to store observer-target pairs used to define
    // all vector types other than constant vectors.  The first
    // element of the ith column indicates the observer (the value
    // is an index into the BODIES array) for the primary vector;
    // the second element indicates the target for the primary vector.
    // The third and fourth elements play the same role for the
    // secondary vector.
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // The elements of ABPAIR are indices into the ABCORR array. ABPAIR
    // has dimensions 2 x NABPRS. Each column of ABPAIR indicates the
    // corrections to be used for the primary and secondary vectors.
    //
    // For the special case of constant vectors, the correction
    // 'LT+S' is replaced with the correction 'LT', the correction
    // 'XLT' is replaced with the correction 'S', and the correction
    // 'XLT+S' is replaced with the correction 'XS'.  These replacements
    // are done in-line.
    //

    //
    // When defining axes, we vary the spacing and case to test the
    // algorithm used to parse axis specifications.
    //

    //
    // We use both inertial and dynamic base frames.
    //

    //
    // PHOBOS_RADIAL is an alternative, or additional, dynamic base
    // frame.  At least one dynamic base frame must be used.
    //
    //
    // .                      'PHOBOS_RADIAL'                          /

    //
    // Bodies acting as observers and targets.
    //

    //
    // Coordinate systems used to represent constant vectors:
    //

    //
    // Euclidean basis vectors.
    //

    //
    // The primary vector is always defined using Phobos as the
    // observer and Mars as the target.  The secondary vector
    // may have the same observer and target (used when the second
    // vector is a velocity vector), the same observer and a different
    // target, or different observer and target.
    //

    //
    // VECDEF contains the possible vector definitions.
    //

    //
    // VECFRM contains frames associated with vectors.  Only velocity
    // and constant vectors actually use frame definitions.
    //

    //
    // When constant vectors are used and the coordinate system
    // is rectangular, the vectors are picked from this set.
    //

    //
    // These strings are used for latitudinal coordinates.
    //

    //
    // DIMS defines the dimensions of the cartesian product of the
    // input parameters.  The cardinality of the set comprising the
    // Nth "factor" of the cartesian product is DIMS(N).  The cardinality
    // of the product itself is the product of the elements of DIMS.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_DYN01", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create test inputs for comprehensive two-vector test.",
        ctx,
    )?;

    //
    // Create and load kernels.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Modify the prime meridian constants for Phobos to
    // make the rotation period deviate more from the orbital
    // period.
    //
    spicelib::VPACK(35.06, 100.0, 0.0, V.as_slice_mut());
    spicelib::PDPOOL(b"BODY401_PM", 3, V.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We'll need to create two non-inertial frames:  one to
    // use as a base frame and one to use as a frame associated
    // with velocity and constant vectors.  We'll the GSE
    // frame as one and a Mars radial frame as the other.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_GSE                        =  2399000",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2399000_NAME               = \'GSE\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_2399000_CLASS              =  5");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_2399000_CLASS_ID           =  2399000",
    );
    fstr::assign(
        DEFTXT.get_mut(5),
        b"FRAME_2399000_CENTER             =  399",
    );
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_2399000_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        DEFTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWSTYL), b"       = \'"),
                KVPARM,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(8),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(b"FRAME_2399000_", KWFFAM),
                    b"             = \'",
                ),
                KV2VEC,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWPRI), KWVAXI),
                    b"       = \'",
                ),
                KVX,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWPRI), KWVCDF),
                    b"       = \'",
                ),
                KVPOSV,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWPRI), KWVOBS),
            b"       = \'EARTH\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWPRI), KWVTRG),
            b"         = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWPRI), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWSEC), KWVAXI),
                    b"       = \'-",
                ),
                KVY,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(15),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWSEC), KWVCDF),
                    b"       =  \'",
                ),
                KVVELV,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(16),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWSEC), KWVOBS),
            b"       = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(17),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWSEC), KWVTRG),
            b"       = \'EARTH\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(18),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWSEC), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(19),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWSEC), KWVFRM),
            b"          = \'J2000\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(20),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399000_", KWRSTA), b"       =  \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    //
    // Load the GSE frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 20, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define the PHOBOS_RADIAL frame.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_PHOBOS_RADIAL              =  2401000",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2401000_NAME               = \'PHOBOS_RADIAL\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_2401000_CLASS              =  5");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_2401000_CLASS_ID           =  2401000",
    );
    fstr::assign(
        DEFTXT.get_mut(5),
        b"FRAME_2401000_CENTER             =  401",
    );
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_2401000_RELATIVE           = \'IAU_MARS\'",
    );
    fstr::assign(
        DEFTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWSTYL), b"       = \'"),
                KVPARM,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(8),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(b"FRAME_2401000_", KWFFAM),
                    b"             = \'",
                ),
                KV2VEC,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWPRI), KWVAXI),
                    b"       = \'",
                ),
                KVZ,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWPRI), KWVCDF),
                    b"       = \'",
                ),
                KVPOSV,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWPRI), KWVOBS),
            b"       = \'PHOBOS\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWPRI), KWVTRG),
            b"         = \'MARS\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWPRI), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWSEC), KWVAXI),
                    b"       = \'-",
                ),
                KVX,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(15),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWSEC), KWVCDF),
                    b"       =  \'",
                ),
                KVVELV,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(16),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWSEC), KWVOBS),
            b"       = \'PHOBOS\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(17),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWSEC), KWVTRG),
            b"       = \'MARS\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(18),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWSEC), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(19),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWSEC), KWVFRM),
            b"          = \'J2000\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(20),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2401000_", KWRSTA), b"       =  \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    //
    // Load the PHOBOS_RADIAL frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 20, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //--- Case: ------------------------------------------------------
    //
    //
    //     First test:  examine the GSE frame.
    //
    testutil::TCASE(b"Check GSE-to-J2000 frame transformation.", ctx)?;

    ET = 10000000.0;

    spicelib::SXFORM(b"GSE", b"J2000", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform "sanity checks" on the returned matrix. Make sure the
    // diagonal blocks are identical rotations, compute a discrete
    // derivative and compare to the lower left block, and make sure the
    // upper right block is a zero matrix.
    //
    DELTA = 1.0;

    spicelib::PXFORM(b"GSE", b"J2000", (ET - DELTA), RMINUS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PXFORM(b"GSE", b"J2000", (ET + DELTA), RPLUS.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_XFORM(
        XFORM.as_slice(),
        RMINUS.as_slice(),
        RPLUS.as_slice(),
        DELTA,
        &mut NRMERR,
        &mut DETERR,
        &mut DRVERR,
        &mut DRLERR,
        DRDIFF.as_slice_mut(),
        ctx,
    )?;

    //
    // Check the error measurements we've made. First up is the
    // determinant error.
    //
    TOL = DM14;
    testutil::CHCKSD(b"DETERR", DETERR, b"~", 0.0, TOL, OK, ctx)?;

    //
    // Check norms of rows and columns in the rotation blocks.
    //
    testutil::CHCKSD(b"NRMERR", NRMERR, b"~", 0.0, TOL, OK, ctx)?;

    //
    // Check the absolute derivative error.
    //
    TOL = ((8 as f64) * DM14);
    testutil::CHCKSD(b"DRVERR", DRVERR, b"~", 0.0, TOL, OK, ctx)?;

    //
    // Check the relative derivative error.
    //
    TOL = (0.6 * DM7);
    testutil::CHCKSD(b"DRLERR", DRLERR, b"~", 0.0, TOL, OK, ctx)?;

    //
    //--- Case: ------------------------------------------------------
    //
    //
    //     Second test:  examine the PHOBOS_RADIAL frame.
    //
    testutil::TCASE(
        b"Check PHOBOS_RADIAL-to-IAU_MARS frame transformation.",
        ctx,
    )?;

    ET = 10000000.0;

    spicelib::SXFORM(b"PHOBOS_RADIAL", b"IAU_MARS", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform "sanity checks" on the returned matrix. Make sure the
    // diagonal blocks are identical rotations, compute a discrete
    // derivative and compare to the lower left block, and make sure the
    // upper right block is a zero matrix.
    //
    DELTA = 1.0;

    spicelib::PXFORM(
        b"PHOBOS_RADIAL",
        b"IAU_MARS",
        (ET - DELTA),
        RMINUS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::PXFORM(
        b"PHOBOS_RADIAL",
        b"IAU_MARS",
        (ET + DELTA),
        RPLUS.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_XFORM(
        XFORM.as_slice(),
        RMINUS.as_slice(),
        RPLUS.as_slice(),
        DELTA,
        &mut NRMERR,
        &mut DETERR,
        &mut DRVERR,
        &mut DRLERR,
        DRDIFF.as_slice_mut(),
        ctx,
    )?;

    //
    // Check the error measurements we've made. First up is the
    // determinant error.
    //
    TOL = DM14;
    testutil::CHCKSD(b"DETERR", DETERR, b"~", 0.0, TOL, OK, ctx)?;

    //
    // Check norms of rows and columns in the rotation blocks.
    //
    testutil::CHCKSD(b"NRMERR", NRMERR, b"~", 0.0, TOL, OK, ctx)?;

    //
    // Check the absolute derivative error.
    //
    TOL = DM9;
    testutil::CHCKSD(b"DRVERR", DRVERR, b"~", 0.0, TOL, OK, ctx)?;

    //
    // Check the relative derivative error.
    //
    TOL = ((5 as f64) * DM7);
    testutil::CHCKSD(b"DRLERR", DRLERR, b"~", 0.0, TOL, OK, ctx)?;

    //
    //--- Case: ------------------------------------------------------
    //
    //
    //     Create an inertial version of the PHOBOS_RADIAL frame.
    //
    testutil::TCASE(b"Test INERTIAL version of PHOBOS_RADIAL frame.", ctx)?;

    spicelib::MOVEC(DEFTXT.as_arg(), 20, DEFTX2.as_arg_mut());

    spicelib::REPMC(&DEFTX2[20].to_vec(), KVROTG, KVINRT, &mut DEFTX2[20]);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTX2.as_arg(), 20, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"PHOBOS_RADIAL", b"IAU_MARS", ET, XF2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the PHOBOS_RADIAL to IAU_MARS transformation
    // defined using this frame to that using the standard
    // PHOBOS_RADIAL frame.  Just compare the rotation
    // blocks.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    R[[I, J]] = XFORM[[I, J]];
                    R2[[I, J]] = XF2[[I, J]];
                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    testutil::CHCKAD(
        b"Upper left block",
        R2.as_slice(),
        b"~",
        R.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    {
        let m1__: i32 = 4;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 4;
                let m2__: i32 = 6;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    R[[(I - 3), (J - 3)]] = XFORM[[I, J]];
                    R2[[(I - 3), (J - 3)]] = XF2[[I, J]];
                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    testutil::CHCKAD(
        b"Lower right block",
        R2.as_slice(),
        b"~",
        R.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check results from PXFORM.
    //
    spicelib::PXFORM(b"PHOBOS_RADIAL", b"IAU_MARS", ET, R3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"PXFORM rotation",
        R3.as_slice(),
        b"~",
        R2.as_slice(),
        9,
        DM14,
        OK,
        ctx,
    )?;

    //
    // Check the derivative matrix:  when we compose the transformation
    // XF2 with the IAU_MARS-to-J2000 transformation, we should
    // get a result with derivative block zero.
    //
    spicelib::SXFORM(b"IAU_MARS", b"J2000", ET, XF2000.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXMG(
        XF2000.as_slice(),
        XF2.as_slice(),
        6,
        6,
        6,
        XFB2J.as_slice_mut(),
    );

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    R[[I, J]] = XFB2J[[(3 + I), J]];
                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    testutil::CHCKAD(
        b"Derivative block",
        R.as_slice(),
        b"~",
        save.ZR.as_slice(),
        9,
        DM16,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    //
    //     Create a frozen version of the PHOBOS_RADIAL frame.
    //
    testutil::TCASE(b"Test FROZEN version of PHOBOS_RADIAL frame.", ctx)?;

    //
    // Expunge the optional ROTATN_STATE keyword from the kernel pool.
    //
    spicelib::DVPOOL(&fstr::concat(b"FRAME_2401000_", KWRSTA), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the frame definition.
    //
    spicelib::MOVEC(DEFTXT.as_arg(), 20, DEFTX2.as_arg_mut());
    //
    // Blank out the ROTATN_STATE assignment for this frame.
    //
    fstr::assign(DEFTX2.get_mut(20), b" ");

    //
    // Set the freeze epoch equal to 1.E7 seconds past J2000 TDB.
    //
    fstr::assign(
        DEFTX2.get_mut(21),
        &fstr::concat(
            &fstr::concat(b"FRAME_2401000_", KWFREZ),
            b"       = @2000-117/05:46:40",
        ),
    );

    spicelib::LMPOOL(DEFTX2.as_arg(), 21, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Although we look up XF2 at ET 0.D0, we should get the
    // same rotation we obtained from the standard PHOBOS_RADIAL
    // frame at ET 1.E7, except that the derivative block has
    // been zeroed out.
    //
    spicelib::CLEARD(36, XF2.as_slice_mut());
    spicelib::SXFORM(b"PHOBOS_RADIAL", b"IAU_MARS", 0.0, XF2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Compare the PHOBOS_RADIAL to IAU_MARS transformation
    // defined using this frame to that using the standard
    // PHOBOS_RADIAL frame.  Just compare the rotation
    // blocks.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    R[[I, J]] = XFORM[[I, J]];
                    R2[[I, J]] = XF2[[I, J]];
                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    testutil::CHCKAD(
        b"Upper left block",
        R2.as_slice(),
        b"~",
        R.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    {
        let m1__: i32 = 4;
        let m2__: i32 = 6;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 4;
                let m2__: i32 = 6;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    R[[(I - 3), (J - 3)]] = XFORM[[I, J]];
                    R2[[(I - 3), (J - 3)]] = XF2[[I, J]];
                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    testutil::CHCKAD(
        b"Lower right block",
        R2.as_slice(),
        b"~",
        R.as_slice(),
        9,
        0.0,
        OK,
        ctx,
    )?;

    //
    // Check results from PXFORM.
    //
    spicelib::PXFORM(b"PHOBOS_RADIAL", b"IAU_MARS", ET, R3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"PXFORM rotation",
        R3.as_slice(),
        b"~",
        R2.as_slice(),
        9,
        DM14,
        OK,
        ctx,
    )?;

    //
    // Check the derivative matrix:  we should have derivative block
    // zero.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    R[[I, J]] = XF2[[(3 + I), J]];
                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    testutil::CHCKAD(
        b"Derivative block",
        R.as_slice(),
        b"~",
        save.ZR.as_slice(),
        9,
        DM16,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    //
    //     Create a new frozen frame that doesn't use a velocity
    //     vector in its definition.  The secondary vector will be the
    //     Mars-Sun vector. Start with the PHOBOS_RADIAL frame as a
    //     template.
    //
    testutil::TCASE(b"Test FROZEN two-position vector frame.", ctx)?;

    //
    // Initialize the frame definition.
    //
    spicelib::MOVEC(DEFTXT.as_arg(), 20, DEFTX2.as_arg_mut());
    //
    // Blank out the ROTATN_STATE assignment for this frame.
    //
    fstr::assign(DEFTX2.get_mut(20), b" ");

    spicelib::REPMC(&DEFTX2[15].to_vec(), KVVELV, KVPOSV, &mut DEFTX2[15]);
    spicelib::REPMC(&DEFTX2[17].to_vec(), b"MARS", b"SUN", &mut DEFTX2[17]);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set the freeze epoch equal to 1.E7 seconds past J2000 TDB.
    //
    fstr::assign(
        DEFTX2.get_mut(21),
        &fstr::concat(
            &fstr::concat(b"FRAME_2401000_", KWFREZ),
            b"       = @2000-117/05:46:40",
        ),
    );

    spicelib::LMPOOL(DEFTX2.as_arg(), 21, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Although we look up XF2 at ET 0.D0, we should get the
    // same rotation we obtained from the standard PHOBOS_RADIAL
    // frame at ET 1.E7, except that the derivative block has
    // been zeroed out.
    //
    spicelib::CLEARD(36, XF2.as_slice_mut());
    spicelib::SXFORM(b"PHOBOS_RADIAL", b"IAU_MARS", 0.0, XF2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the upper left rotation block from XF2.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    R2[[I, J]] = XF2[[I, J]];
                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    //
    // Check results from PXFORM.
    //
    spicelib::PXFORM(b"PHOBOS_RADIAL", b"IAU_MARS", ET, R3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"PXFORM rotation",
        R3.as_slice(),
        b"~",
        R2.as_slice(),
        9,
        DM14,
        OK,
        ctx,
    )?;

    //
    // Check the derivative matrix:  we should have derivative block
    // zero.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    R[[I, J]] = XF2[[(3 + I), J]];
                    J += m3__;
                }
            }
            I += m3__;
        }
    }

    testutil::CHCKAD(
        b"Derivative block",
        R.as_slice(),
        b"~",
        save.ZR.as_slice(),
        9,
        DM16,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    //
    //     Create a new frozen frame with linearly dependent defining
    //     vectors.  Both primary and secondary vectors will be the
    //     Phobos-Mars position vector. Start with the PHOBOS_RADIAL frame
    //     as a template.
    //
    testutil::TCASE(b"Test linearly dependent two-position vector frame.", ctx)?;

    //
    // Initialize the frame definition.
    //
    spicelib::MOVEC(DEFTXT.as_arg(), 20, DEFTX2.as_arg_mut());
    //
    // Blank out the ROTATN_STATE assignment for this frame.
    //
    fstr::assign(DEFTX2.get_mut(20), b" ");

    spicelib::REPMC(&DEFTX2[15].to_vec(), KVVELV, KVPOSV, &mut DEFTX2[15]);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::LMPOOL(DEFTX2.as_arg(), 20, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Make sure error is caught by both SXFORM and PXFORM.
    //
    spicelib::SXFORM(b"PHOBOS_RADIAL", b"IAU_MARS", 0.0, XF2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    spicelib::PXFORM(b"PHOBOS_RADIAL", b"IAU_MARS", 0.0, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(DEGENERATECASE)", OK, ctx)?;

    //
    // Expunge the optional FREEZE_EPOCH keyword from the kernel pool.
    //
    spicelib::DVPOOL(&fstr::concat(b"FRAME_2401000_", KWFREZ), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Restore the original PHOBOS_RADIAL frame definition.
    //
    fstr::assign(DEFTXT.get_mut(21), b" ");
    spicelib::LMPOOL(DEFTXT.as_arg(), 21, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Proceed to the general test cases.
    //
    //
    // Because there are so many options that may be used to define
    // a two-vector frame, we can't conveniently loop over every
    // set of possibilities using nested loops:  the loops would be
    // nested too deeply.  Instead, we define a mapping that allows
    // us to map a test case number to a set of input parameters.
    //
    // First compute the number of test cases in the cartesian product
    // test.
    //

    NCART = spicelib::PRODAI(save.DIMS.as_slice(), NDIMS);

    //
    // All of the variables initialized here are used for debugging.
    //
    NOGO = 0;
    NPR = 0;
    // NNORML = 0
    NCOR = 0;
    NSKIP = 0;
    MAXDER = 0.0;

    for CASE in 1..=NCART {
        //
        // Find the multi-dimensional coordinates of the current
        // case in the cartesian product of the test input sets.
        //
        testutil::MULTIX(
            1,
            NDIMS,
            save.DIMS.as_slice(),
            CASE,
            COORDS.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Set frame definition parameters defined by COORDS.
        // Start out with the base frame.
        //
        fstr::assign(&mut BFRAME, save.BASEFR.get(COORDS[BFRIDX]));

        //
        // The second dimension refers to the observer-target pairs
        // for the primary and secondary vectors.  (For constant
        // vectors, only the observer is used, and this is only if
        // aberration corrections are used.)
        //
        I = save.OTPAIR[[1, COORDS[OTPIDX]]];
        J = save.OTPAIR[[2, COORDS[OTPIDX]]];
        fstr::assign(&mut PRIOBS, save.BODIES.get(I));
        fstr::assign(&mut PRITRG, save.BODIES.get(J));

        I = save.OTPAIR[[3, COORDS[OTPIDX]]];
        J = save.OTPAIR[[4, COORDS[OTPIDX]]];
        fstr::assign(&mut SECOBS, save.BODIES.get(I));
        fstr::assign(&mut SECTRG, save.BODIES.get(J));

        //
        // The third dimension refers to the extended vector
        // definition for the primary vector.  Extended definitions
        // include frame selections for velocity vectors and
        // constant vectors (those vectors requiring frames as
        // part of their definitions).  The ordering of extended
        // frame definitions is:
        //
        //     1           )  Observer-target position
        //     2           )  Observer-near point position
        //     3           )  Velocity vector, frame definition 1
        //                      ...
        //     2 + NVECFR  )  Velocity vector, frame definition NVECFR
        //     3 + NVECFR  )  Constant vector, frame definition 1
        //                      ...
        //     2 + 2*NVECFR)  Constant vector, frame definition NVECFR
        //
        // The fourth dimension provides the analogous information
        // for the secondary vector.
        //
        // We'll process the definitions for both vectors in the loop
        // below.
        //

        {
            let m1__: i32 = 1;
            let m2__: i32 = 2;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Indicate that we don't have a velocity vector or
                // constant vector until we know otherwise.
                //
                ISVEL[I] = false;
                ISCON[I] = false;

                if (I == 1) {
                    CORIDX = PRIIDX;
                } else {
                    CORIDX = SECIDX;
                }

                J = COORDS[CORIDX];

                if (I == 1) {
                    PRIJ = J;
                }

                if (J == 1) {
                    //
                    // The vector is an observer-target position.
                    //
                    fstr::assign(&mut VDF, save.VECDEF.get(1));
                    fstr::assign(&mut VFR, b" ");
                } else if (J == 2) {
                    //
                    // The vector is an observer-near point position.
                    //
                    fstr::assign(&mut VDF, save.VECDEF.get(2));
                    fstr::assign(&mut VFR, b" ");
                } else if ((J >= 3) && (J <= (2 + NVECFR))) {
                    //
                    // The vector is an observer-target velocity.
                    // The quantity J-2 is an index into the vector
                    // frame array.
                    //
                    fstr::assign(&mut VDF, save.VECDEF.get(3));
                    fstr::assign(&mut VFR, save.VECFRM.get((J - 2)));
                    ISVEL[I] = true;
                } else {
                    //
                    // The primary is constant in the specified frame.
                    // The quantity J-2-NVECFR is an index into the vector
                    // frame array.
                    //
                    fstr::assign(&mut VDF, save.VECDEF.get(4));
                    fstr::assign(&mut VFR, save.VECFRM.get(((J - 2) - NVECFR)));
                    ISCON[I] = true;
                }

                if (I == 1) {
                    fstr::assign(&mut PRIVDF, &VDF);
                    fstr::assign(&mut PRIFRM, &VFR);
                } else {
                    fstr::assign(&mut SECVDF, &VDF);
                    fstr::assign(&mut SECFRM, &VFR);
                }

                //
                // Set the aberration correction for the current vector.
                //
                J = save.ABPAIR[[I, COORDS[ABCIDX]]];

                //
                // If the current vector is constant, adjust the
                // aberration correction to make it valid for constant
                // vectors.
                //
                // For the special case of constant vectors, the correction
                // 'LT+S' is replaced with the correction 'LT', the correction
                // 'XLT' is replaced with the correction 'S', and the
                // correction 'XLT+S' is replaced with the correction 'XS'.
                //
                if ISCON[I] {
                    if fstr::eq(save.ABCORR.get(J), b"LT+S") {
                        //
                        // Set the correction for this vector to 'LT'
                        //
                        J = spicelib::ISRCHC(b"LT", NABCRR, save.ABCORR.as_arg());
                    } else if fstr::eq(save.ABCORR.get(J), b"XLT") {
                        //
                        // Set the correction for this vector to 'S'
                        //
                        J = spicelib::ISRCHC(b"S", NABCRR, save.ABCORR.as_arg());
                    } else if fstr::eq(save.ABCORR.get(J), b"XLT+S") {
                        //
                        // Set the correction for this vector to 'XS'
                        //
                        J = spicelib::ISRCHC(b"XS", NABCRR, save.ABCORR.as_arg());
                    } else if fstr::eq(save.ABCORR.get(J), b"NONE") {
                        //
                        // Set the correction for this vector to 'NONE'
                        //
                        J = spicelib::ISRCHC(b"NONE", NABCRR, save.ABCORR.as_arg());
                    } else {
                        spicelib::SETMSG(b"Unexpected aberration correction # was encountered when mapping correction for constant vector #.Case is #.", ctx);
                        spicelib::ERRCH(b"#", &save.ABCORR[J], ctx);
                        spicelib::ERRINT(b"#", I, ctx);
                        spicelib::ERRINT(b"#", CASE, ctx);
                        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;

                        testutil::CHCKXC(false, b" ", OK, ctx)?;

                        return Ok(());
                    }
                }

                if (I == 1) {
                    fstr::assign(&mut PRICOR, save.ABCORR.get(J));
                } else {
                    fstr::assign(&mut SECCOR, save.ABCORR.get(J));
                }

                I += m3__;
            }
        }

        //
        // Decide whether we need to execute the current case.
        //
        // There are certain cases we can skip:
        //
        //    1) Neither vector is a velocity vector and both
        //       vectors have the same observer/target sets.
        //       Such vector pairs will be too close to linearly
        //       dependent to work with.
        //
        //    2) Both vectors are velocity vectors.
        //
        GO = true;

        //
        // Use this code to disable testing of frames defined by
        // a pair of velocity vectors.
        //
        //  IF ( ISVEL(1) .AND. ISVEL(2) ) THEN
        //     GO = .FALSE.
        //  END IF

        J = COORDS[PRIIDX];
        K = COORDS[SECIDX];

        if ((ISVEL[1] && ISVEL[2]) || ((J <= 2) && (K <= 2))) {
            //
            // The primary and secondary vectors are either both
            //
            //    - velocity vectors
            //
            //    - in the set
            //
            //        { OBSERVER_TARGET_POSITION, TARGET_NEAR_POINT }
            //
            // In this case we don't want both vectors to be defined
            // by the same pair of objects, because the vectors will
            // be linearly dependent or close to it.
            //
            if ((fstr::eq(&PRIOBS, &SECOBS) || fstr::eq(&PRIOBS, &SECTRG))
                && (fstr::eq(&PRITRG, &SECOBS) || fstr::eq(&PRITRG, &SECTRG)))
            {
                GO = false;
            }
        }

        //
        // Another linear dependency problem:  if the vector frame
        // is PHOBOS_RADIAL, then we don't want cases where the observer
        // and target are from the set { Phobos, Mars } for both
        // vectors.
        //
        if (fstr::eq(&PRIFRM, b"PHOBOS_RADIAL") || fstr::eq(&SECFRM, b"PHOBOS_RADIAL")) {
            //
            // Note: this code must change if the observer-target pairs
            // are changed!
            //
            if (COORDS[OTPIDX] == 1) {
                GO = false;
            }
        }

        //
        // Similar problem for the earth and sun when the velocity
        // frame is GSE:
        //
        if (fstr::eq(&PRIFRM, b"GSE") || fstr::eq(&SECFRM, b"GSE")) {
            //
            // Note: this code must change if the observer-target pairs
            // are changed!
            //
            if (COORDS[OTPIDX] == 4) {
                GO = false;
            }
        }

        //
        // Bits used for debugging
        //
        if !GO {
            NOGO = (NOGO + 1);
        }

        if GO {
            //
            // Cycle over the vector-axis associations.  There's
            // no interaction between these parameters and the
            // other parameters, so we don't need to try every
            // possible combination of these values with the other
            // inputs.  We just need to use each association.
            //
            AXPAIR = (intrinsics::MOD(CASE, NAXDEF) + 1);

            //
            // We're ready to create a frame definition.
            //
            //
            // Create a text buffer containing a frame definition that
            // uses the current frame definition
            //
            //
            // First comes the frame name-to-frame code assignment.
            //
            fstr::assign(&mut FRNAME, b"TWOVEC_DYN_FRAME");
            FRCODE = -2000000000;

            fstr::assign(DEFTXT.get_mut(1), b"FRAME_#          = #");
            spicelib::REPMC(&DEFTXT[1].to_vec(), b"#", &FRNAME, &mut DEFTXT[1]);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&DEFTXT[1].to_vec(), b"#", FRCODE, &mut DEFTXT[1], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // We'll use the kernel variable name "stem"
            //
            //    FRAME_<ID code>_
            //
            // repeatedly, so instead of making thousands of REPMI
            // calls, we just create this stem here.
            //
            spicelib::REPMI(b"FRAME_#_", b"#", FRCODE, &mut FRSTEM, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(
                DEFTXT.get_mut(2),
                &fstr::concat(&FRSTEM, b"NAME             = \'#\'"),
            );
            spicelib::REPMC(&DEFTXT[2].to_vec(), b"#", &FRNAME, &mut DEFTXT[2]);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Frame class and class ID come next.
            //
            fstr::assign(
                DEFTXT.get_mut(3),
                &fstr::concat(&FRSTEM, b"CLASS            = 5"),
            );

            fstr::assign(
                DEFTXT.get_mut(4),
                &fstr::concat(&FRSTEM, b"CLASS_ID         = #"),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&DEFTXT[4].to_vec(), b"#", FRCODE, &mut DEFTXT[4], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The frame center is always the primary vector's
            // observer, for now.  The center must be specified
            // via a code.
            //
            fstr::assign(
                DEFTXT.get_mut(5),
                &fstr::concat(&FRSTEM, b"CENTER           = #"),
            );

            spicelib::BODN2C(&PRIOBS, &mut J, &mut FOUND, ctx)?;

            if !FOUND {
                spicelib::SETMSG(b"No ID code for #.", ctx);
                spicelib::ERRCH(b"#", &PRIOBS, ctx);
                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
            }

            spicelib::REPMI(&DEFTXT[5].to_vec(), b"#", J, &mut DEFTXT[5], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Set the base frame.
            //
            fstr::assign(
                DEFTXT.get_mut(6),
                &fstr::concat(&FRSTEM, b"RELATIVE         = \'#\'"),
            );
            spicelib::REPMC(&DEFTXT[6].to_vec(), b"#", &BFRAME, &mut DEFTXT[6]);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Set the frame definition style and family.
            //
            fstr::assign(
                DEFTXT.get_mut(7),
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(&fstr::concat(&FRSTEM, KWSTYL), b"       = \'"),
                        KVPARM,
                    ),
                    b"\'",
                ),
            );
            spicelib::REPMI(&DEFTXT[7].to_vec(), b"#", FRCODE, &mut DEFTXT[7], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(
                DEFTXT.get_mut(8),
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(&fstr::concat(&FRSTEM, KWFFAM), b"             = \'"),
                        KV2VEC,
                    ),
                    b"\'",
                ),
            );
            spicelib::REPMI(&DEFTXT[8].to_vec(), b"#", FRCODE, &mut DEFTXT[8], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(
                DEFTXT.get_mut(9),
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(&fstr::concat(&FRSTEM, KWRSTA), b"       =  \'"),
                        KVROTG,
                    ),
                    b"\'",
                ),
            );
            spicelib::REPMI(&DEFTXT[9].to_vec(), b"#", FRCODE, &mut DEFTXT[9], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Create definition assignments for the primary and secondary
            // axes.
            //
            // At this point, we'll have to abandon hard-coded array
            // indices for the DEFTXT array and instead use the variable
            // DX to represent the current index.
            //
            DX = 10;
            //
            // Blank out the variables containing associated frame names,
            // so we don't end up with residual values from other test
            // cases in these names.
            //
            fstr::assign(&mut PRIVF, b" ");
            fstr::assign(&mut SECVF, b" ");
            fstr::assign(&mut VFRAME, b" ");

            {
                let m1__: i32 = 1;
                let m2__: i32 = 2;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (I == 1) {
                        //
                        // On the first pass, set up definitions for the
                        // primary vector.
                        //
                        fstr::assign(&mut ORD, b"PRI_");
                        fstr::assign(&mut VDF, &PRIVDF);
                        fstr::assign(&mut VFR, &PRIFRM);
                        fstr::assign(&mut COR, &PRICOR);
                        fstr::assign(&mut OBS, &PRIOBS);
                        fstr::assign(&mut TARG, &PRITRG);
                    } else {
                        //
                        // On the second pass, set up definitions for the
                        // secondary vector.
                        //
                        fstr::assign(&mut ORD, b"SEC_");
                        fstr::assign(&mut VDF, &SECVDF);
                        fstr::assign(&mut VFR, &SECFRM);
                        fstr::assign(&mut COR, &SECCOR);
                        fstr::assign(&mut OBS, &SECOBS);
                        fstr::assign(&mut TARG, &SECTRG);
                    }

                    //
                    // Identify the axis { +/-X, +/-Y, +/-Z } associated
                    // with the current vector.
                    //
                    fstr::assign(
                        DEFTXT.get_mut(DX),
                        &fstr::concat(&fstr::concat(&FRSTEM, &ORD), b"AXIS         = \'#\'"),
                    );

                    spicelib::REPMC(
                        &DEFTXT[DX].to_vec(),
                        b"#",
                        &save.AXDEF[[I, AXPAIR]],
                        &mut DEFTXT[DX],
                    );

                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    DX = (DX + 1);

                    //
                    // Set the current vector definition.
                    //
                    fstr::assign(
                        DEFTXT.get_mut(DX),
                        &fstr::concat(
                            &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWVCDF),
                            b"     = \'#\'",
                        ),
                    );
                    spicelib::REPMC(&DEFTXT[DX].to_vec(), b"#", &VDF, &mut DEFTXT[DX]);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    DX = (DX + 1);

                    //
                    // Set the current vector observer.
                    //
                    fstr::assign(
                        DEFTXT.get_mut(DX),
                        &fstr::concat(
                            &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWVOBS),
                            b"     = \'#\'",
                        ),
                    );
                    spicelib::REPMC(&DEFTXT[DX].to_vec(), b"#", &OBS, &mut DEFTXT[DX]);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    DX = (DX + 1);

                    //
                    // Set the current vector target.  Constant vectors
                    // don't have targets.
                    //
                    if fstr::ne(&VDF, b"CONSTANT") {
                        fstr::assign(
                            DEFTXT.get_mut(DX),
                            &fstr::concat(
                                &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWVTRG),
                                b"     = \'#\'",
                            ),
                        );
                        spicelib::REPMC(&DEFTXT[DX].to_vec(), b"#", &TARG, &mut DEFTXT[DX]);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                        DX = (DX + 1);
                    }
                    //
                    // Set the current vector's aberration correction.
                    //
                    fstr::assign(
                        DEFTXT.get_mut(DX),
                        &fstr::concat(
                            &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWVABC),
                            b"     = \'#\'",
                        ),
                    );

                    spicelib::REPMC(&DEFTXT[DX].to_vec(), b"#", &COR, &mut DEFTXT[DX]);
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                    DX = (DX + 1);

                    //
                    // Set the current vector's frame, if necessary.
                    // Velocity vectors and constant vectors have associated
                    // frames.
                    //
                    if (fstr::eq(&VDF, b"OBSERVER_TARGET_VELOCITY") || fstr::eq(&VDF, b"CONSTANT"))
                    {
                        if ((((fstr::eq(&VFR, b"J2000") || fstr::eq(&VFR, b"IAU_JUPITER"))
                            || fstr::eq(&VFR, b"GSE"))
                            || fstr::eq(&VFR, b"IAU_PHOBOS"))
                            || fstr::eq(&VFR, b"PHOBOS_RADIAL"))
                        {
                            fstr::assign(&mut VFRAME, &VFR);
                        } else if fstr::eq(&VFR, b"BASE_FRAME") {
                            fstr::assign(&mut VFRAME, &BFRAME);
                        } else if fstr::eq(&VFR, b"OBSERVER_FRAME") {
                            spicelib::CNMFRM(&OBS, &mut VFCODE, &mut VFRAME, &mut FOUND, ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            if !FOUND {
                                spicelib::SETMSG(b"No frame info for #.", ctx);
                                spicelib::ERRCH(b"#", &OBS, ctx);
                                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                            }
                        } else if fstr::eq(&VFR, b"TARGET_FRAME") {
                            spicelib::CNMFRM(&TARG, &mut VFCODE, &mut VFRAME, &mut FOUND, ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            if !FOUND {
                                spicelib::SETMSG(b"No frame info for #.", ctx);
                                spicelib::ERRCH(b"#", &TARG, ctx);
                                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                            }
                        } else {
                            spicelib::SETMSG(b"Unexpected velocity frame token: #", ctx);
                            spicelib::ERRCH(b"#", &VFR, ctx);
                            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                        }

                        //
                        // Save the resolved frame names for later testing.
                        //
                        if (I == 1) {
                            fstr::assign(&mut PRIVF, &VFRAME);
                        } else {
                            fstr::assign(&mut SECVF, &VFRAME);
                        }

                        fstr::assign(
                            DEFTXT.get_mut(DX),
                            &fstr::concat(
                                &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWVFRM),
                                b"        = \'#\'",
                            ),
                        );

                        spicelib::REPMC(&DEFTXT[DX].to_vec(), b"#", &VFRAME, &mut DEFTXT[DX]);
                        testutil::CHCKXC(false, b" ", OK, ctx)?;
                        DX = (DX + 1);
                    }

                    //
                    // If the vector is constant, define it.  The vector
                    // definitions have no interaction with the other defining
                    // parameters other than the vector order, so all we need
                    // do is make sure all definitions are covered for the
                    // primary and secondary vectors.  We'll use the following
                    // definitions:
                    //
                    //    ... SPEC      = 'RECTANGULAR'
                    //    ... VECTOR    = ( <number1>, <number2>, <number3> )
                    //
                    //    ... SPEC      = 'RECTANGULAR'
                    //    ... VECTOR    = ( <number4>, <number5>, <number6> )
                    //
                    //    ... SPEC      = 'RECTANGULAR'
                    //    ... VECTOR    = ( <number7>, <number8>, <number9> )
                    //
                    //    ... SPEC      = 'LATITUDINAL'
                    //    ... UNITS     = 'DEGREES'
                    //    ... LONGITUDE = <number>
                    //    ... LATITUDE  = <number>
                    //
                    //    ... SPEC      = 'LATITUDINAL'
                    //    ... UNITS     = 'RADIANS'
                    //    ... LONGITUDE = <number>
                    //    ... LATITUDE  = <number>
                    //
                    //
                    //
                    if fstr::eq(&VDF, b"CONSTANT") {
                        if (I == 1) {
                            NCOR = (NCOR + 1);
                        }

                        //
                        // Make the vector selection depend on the case
                        // and whether the axis is primary or secondary.
                        //
                        J = (intrinsics::MOD((NCOR + I), 5) + 1);

                        if (J <= 3) {
                            //
                            // This is the rectangular coordinate case.
                            //
                            SYSIDX = 1;

                            fstr::assign(
                                DEFTXT.get_mut(DX),
                                &fstr::concat(
                                    &fstr::concat(
                                        &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWVSPC),
                                        b"         = ",
                                    ),
                                    save.CORSYS.get(SYSIDX),
                                ),
                            );
                            DX = (DX + 1);

                            fstr::assign(
                                DEFTXT.get_mut(DX),
                                &fstr::concat(
                                    &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWVECT),
                                    b"       = ",
                                ),
                            );

                            spicelib::SUFFIX(&save.VECSTR[J], 1, &mut DEFTXT[DX]);
                            DX = (DX + 1);

                            //
                            // Produce the d.p. constant vector; we'll use this
                            // in later testing.  Start by parsing the string
                            // representing the vector.  The first token will
                            // be a blank (representing the null token to the
                            // left of the left parenthesis), so start with
                            // token #2.
                            //
                            spicelib::LPARSM(
                                &save.VECSTR[J],
                                b"(),",
                                7,
                                &mut N,
                                TOKENS.as_arg_mut(),
                            );
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            for L in 2..=4 {
                                spicelib::PRSDP(&TOKENS[L], &mut CONVEC[(L - 1)], ctx)?;
                                testutil::CHCKXC(false, b" ", OK, ctx)?;
                            }
                        } else {
                            //
                            // This is one of the latitudinal or RA/DEC
                            // coordinate cases.
                            //
                            if (J == 4) {
                                //
                                // Use latitudinal coordinates.
                                //
                                SYSIDX = 2;
                            } else {
                                //
                                // Use RA/DEC coordinates.
                                //
                                SYSIDX = 3;
                            }

                            fstr::assign(
                                DEFTXT.get_mut(DX),
                                &fstr::concat(
                                    &fstr::concat(
                                        &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWVSPC),
                                        b"         = ",
                                    ),
                                    save.CORSYS.get(SYSIDX),
                                ),
                            );
                            DX = (DX + 1);

                            //
                            // Let K be the index of the units.
                            //
                            K = (J - 3);

                            fstr::assign(
                                DEFTXT.get_mut(DX),
                                &fstr::concat(
                                    &fstr::concat(
                                        &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWUNIT),
                                        b"        = ",
                                    ),
                                    save.UNITS.get(K),
                                ),
                            );
                            DX = (DX + 1);

                            if (J == 4) {
                                fstr::assign(
                                    DEFTXT.get_mut(DX),
                                    &fstr::concat(
                                        &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWLON),
                                        b"    = ",
                                    ),
                                );
                            } else {
                                fstr::assign(
                                    DEFTXT.get_mut(DX),
                                    &fstr::concat(
                                        &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWRA),
                                        b"           = ",
                                    ),
                                );
                            }

                            spicelib::SUFFIX(&save.ANGSTR[[1, K]], 1, &mut DEFTXT[DX]);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            //
                            // Save the longitude as a d.p. number.
                            //
                            spicelib::PRSDP(&save.ANGSTR[[1, K]], &mut LON0, ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            DX = (DX + 1);

                            if (J == 4) {
                                fstr::assign(
                                    DEFTXT.get_mut(DX),
                                    &fstr::concat(
                                        &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWLAT),
                                        b"     = ",
                                    ),
                                );
                            } else {
                                fstr::assign(
                                    DEFTXT.get_mut(DX),
                                    &fstr::concat(
                                        &fstr::concat(&fstr::concat(&FRSTEM, &ORD), KWDEC),
                                        b"          = ",
                                    ),
                                );
                            }

                            spicelib::SUFFIX(&save.ANGSTR[[2, K]], 1, &mut DEFTXT[DX]);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            DX = (DX + 1);
                            //
                            // Save the latitude as a d.p. number.
                            //
                            spicelib::PRSDP(&save.ANGSTR[[2, K]], &mut LAT0, ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // Convert the numeric lat/lon values to radians.
                            //
                            spicelib::CMPRSS(b"\'", 0, &save.UNITS[K], &mut TMPSTR);
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            spicelib::CONVRT(LAT0, &TMPSTR, KVRADN, &mut LAT, ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                            spicelib::CONVRT(LON0, &TMPSTR, KVRADN, &mut LON, ctx)?;
                            testutil::CHCKXC(false, b" ", OK, ctx)?;

                            //
                            // Produce a d.p. constant vector.
                            //
                            spicelib::LATREC(1.0, LON, LAT, CONVEC.as_slice_mut());
                            testutil::CHCKXC(false, b" ", OK, ctx)?;
                        }
                        //
                        // Save the constant vector as one of the defining
                        // vectors.
                        //
                        if (I == 1) {
                            spicelib::VEQU(CONVEC.as_slice(), PRIVEC.as_slice_mut());
                        } else {
                            spicelib::VEQU(CONVEC.as_slice(), SECVEC.as_slice_mut());
                        }
                    }

                    I += m3__;
                }
            }

            //
            // Enter the frame definition into the kernel pool.
            //
            spicelib::LMPOOL(DEFTXT.as_arg(), (DX - 1), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            //           Here's where the testing begins.
            //
            //
            //--- Case: ------------------------------------------------------
            //
            spicelib::REPMI(
                b"SXFORM test number #.  Test results against those from PXFORM.",
                b"#",
                CASE,
                &mut CASMSG,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::TCASE(&CASMSG, ctx)?;

            //
            // Pick a time value.
            //
            ET = (1000.0 * (CASE - 5000) as f64);

            // WRITE (*,*) 'F_DYN01:  SXFORM CALL <<<<<<<<<<<<<<<<<<'

            spicelib::SXFORM(&FRNAME, &BFRAME, ET, XFORM.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // IF ( .NOT. OK ) THEN
            //    DO I = 1, DX-1
            //       WRITE (*,*) DEFTXT(I)
            //    END DO
            //
            //    WRITE (*,*) '=================================== '
            // END IF
            //

            // WRITE (*,*) 'F_DYN01:  DONE <<<<<<<<<<<<<<<<<<'

            spicelib::PXFORM(&FRNAME, &BFRAME, ET, R.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Extract the derivative block from XFORM into DRVBLK.
            // We'll use this block later; we're extracting it here
            // to keep this code close to the associated SXFORM call.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = 3;
                        let m3__: i32 = 1;
                        J = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            DRVBLK[[J, I]] = XFORM[[(J + 3), I]];
                            J += m3__;
                        }
                    }
                    I += m3__;
                }
            }

            DMAG = spicelib::VNORMG(DRVBLK.as_slice(), 9);

            //
            // Extract the rotation block from XFORM into R2.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = 3;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = 3;
                        let m3__: i32 = 1;
                        J = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            R2[[J, I]] = XFORM[[J, I]];
                            J += m3__;
                        }
                    }
                    I += m3__;
                }
            }

            testutil::CHCKAD(
                b"SXFORM vs PXFORM",
                R2.as_slice(),
                b"~",
                R.as_slice(),
                9,
                DM12,
                OK,
                ctx,
            )?;

            //
            // These statements may be useful for debugging.
            //
            //
            // IF ( .NOT. OK ) THEN
            //
            //    WRITE (*,*) '=================================== '
            //    WRITE (*,*) 'CASE = ', CASE
            //    WRITE (*,*) COORDS
            //    WRITE (*,*) 'PRIFRM  = ', PRIFRM
            //    WRITE (*,*) 'PRIJ    = ', PRIJ
            //    WRITE (*,*) 'PRIVF   = ', PRIVF
            //    WRITE (*,*) 'VDF     = ', VDF
            //    WRITE (*,*) 'VFRAME  = ', VFRAME
            //    WRITE (*,*) 'DRVERR, DRLERR  = ', DRVERR, DRLERR
            //    WRITE (*,*) 'DMAG    = ', DMAG
            //    WRITE (*,*) 'DM16  /DMAG    = ', DM16  /DMAG
            //
            //   DO I = 1, DX-1
            //      WRITE (*,*) DEFTXT(I)
            //   END DO
            //
            //   WRITE (*,*) '=================================== '
            //
            // END IF
            //

            //
            //--- Case: ------------------------------------------------------
            //
            spicelib::REPMI(
                b"PXFORM test number #.  Primary vector def = #.",
                b"#",
                CASE,
                &mut CASMSG,
                ctx,
            );
            spicelib::REPMC(&CASMSG.clone(), b"#", &PRIVDF, &mut CASMSG);

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::TCASE(&CASMSG, ctx)?;

            //
            // Find the primary axis name and sign.
            //
            fstr::assign(&mut AXNAME, save.AXDEF.get([1, AXPAIR]));

            if (intrinsics::INDEX(&AXNAME, b"-") > 0) {
                AXSIGN = -1.0;
            } else {
                AXSIGN = 1.0;
            }

            spicelib::CMPRSS(b"-", 0, &AXNAME.clone(), &mut AXNAME);

            if spicelib::EQSTR(&AXNAME, b"X") {
                AXINDX = 1;
            } else if spicelib::EQSTR(&AXNAME, b"Y") {
                AXINDX = 2;
            } else {
                AXINDX = 3;
            }

            //
            // Create the primary vector, specified relative to the
            // base frame.  Map this vector to the current dynamic
            // frame.  The vector should match the current axis
            // specification for the primary defining vector.
            //
            if fstr::eq(&PRIVDF, b"OBSERVER_TARGET_POSITION") {
                spicelib::SPKPOS(
                    &PRITRG,
                    ET,
                    b"J2000",
                    &PRICOR,
                    &PRIOBS,
                    V.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::PXFORM(b"J2000", &FRNAME, ET, R2.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXV(R2.as_slice(), V.as_slice(), VTEMP.as_slice_mut());
                spicelib::VSCL(AXSIGN, VTEMP.as_slice(), V.as_slice_mut());
                spicelib::VHAT(V.as_slice(), PRIVEC.as_slice_mut());

                //
                // PRIVEC should match the axis indicated by AXINDX.
                //
                testutil::CHCKAD(
                    b"PRIVEC",
                    PRIVEC.as_slice(),
                    b"~",
                    save.E.subarray([1, AXINDX]),
                    3,
                    DM12,
                    OK,
                    ctx,
                )?;
            } else if fstr::eq(&PRIVDF, b"TARGET_NEAR_POINT") {
                //
                // Find the specified near point.
                //
                spicelib::SUBPT(
                    b"NEAR POINT",
                    &PRITRG,
                    ET,
                    &PRICOR,
                    &PRIOBS,
                    SPOINT.as_slice_mut(),
                    &mut ALT,
                    ctx,
                )?;

                //
                // We actually need to use the position transformation
                // from PRIVF to FRMNAM for this one.  NOTE:  the
                // epoch of this transformation is critical.  We must
                // use the same epoch as was used by SUBPT.
                //
                // Get the light time LT from the observer to target
                // center.
                //
                spicelib::SPKPOS(
                    &PRITRG,
                    ET,
                    b"J2000",
                    &PRICOR,
                    &PRIOBS,
                    POS.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Find the epoch associated with the target by
                // adjusting ET by LT.
                //
                spicelib::ZZCOREPC(&PRICOR, ET, LT, &mut ETCORR, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Find the position transformation from the target
                // body-fixed frame at ETCORR to J2000.  Convert
                // the body-fixed sub-point position to the J2000
                // frame.
                //
                spicelib::CNMFRM(&PRITRG, &mut J, &mut TRGFRM, &mut FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"TARGET FRAME FOUND", FOUND, true, OK, ctx)?;

                spicelib::PXFORM(&TRGFRM, b"J2000", ETCORR, R2.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXV(R2.as_slice(), SPOINT.as_slice(), V.as_slice_mut());

                //
                // Compute the observer-to-near point vector in the J2000
                // frame.
                //
                spicelib::VADD(V.as_slice(), POS.as_slice(), VTEMP.as_slice_mut());

                //
                // Find the position transformation J2000 the current frame
                // at ET.  Rotate the observer-near point vector into this
                // frame.
                //
                spicelib::PXFORM(b"J2000", &FRNAME, ET, R3.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXV(R3.as_slice(), VTEMP.as_slice(), V.as_slice_mut());

                //
                // Negate the vector if indicated, then unitize.
                //
                spicelib::VSCL(AXSIGN, V.as_slice(), VTEMP.as_slice_mut());
                spicelib::VHAT(VTEMP.as_slice(), PRIVEC.as_slice_mut());

                //
                // PRIVEC should match the axis indicated by AXINDX.
                //
                testutil::CHCKAD(
                    b"PRIVEC",
                    PRIVEC.as_slice(),
                    b"~",
                    save.E.subarray([1, AXINDX]),
                    3,
                    DM12,
                    OK,
                    ctx,
                )?;
            } else if fstr::eq(&PRIVDF, b"OBSERVER_TARGET_VELOCITY") {
                //
                // Look up the velocity vector in the specified frame.
                // Note this choice is critical:  any other frame,
                // unless related by a constant offset to the correct
                // frame, yields an invalid result.
                //
                spicelib::SPKEZR(
                    &PRITRG,
                    ET,
                    &PRIVF,
                    &PRICOR,
                    &PRIOBS,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // We actually need to use the position transformation
                // from PRIVF to FRMNAM for this one.  NOTE:  the
                // epoch of this transformation is critical.  We must
                // use the same epoch as was used by SPKEZR.  If
                // PRIVF is non-inertial, we need to adjust ET by
                // the light time from the observer to the center of PRIVF.
                // We can actually finesse the issue of whether the frame
                // is non-inertial, since performing the correction for
                // an inertial frame has no effect.
                //
                spicelib::NAMFRM(&PRIVF, &mut J, ctx)?;
                spicelib::FRINFO(J, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"FRAME CENTER FOUND", FOUND, true, OK, ctx)?;

                spicelib::BODC2N(CENT, &mut CENTER, &mut FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"CENTER NAME FOUND", FOUND, true, OK, ctx)?;

                //
                // Get the light time CLT from the observer to the center
                // of the frame in which we looked up the velocity.
                //
                spicelib::SPKEZR(
                    &CENTER,
                    ET,
                    &PRIVF,
                    &PRICOR,
                    &PRIOBS,
                    CSTATE.as_slice_mut(),
                    &mut CLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Find the epoch associated with the center of the
                // frame by adjusting ET by CLT.
                //
                spicelib::ZZCOREPC(&PRICOR, ET, CLT, &mut ETCORR, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Find the position transformation from PRIVF at
                // ETCORR to the current frame at ET.  We must do
                // this in two steps, passing through J2000.
                //
                spicelib::PXFORM(&PRIVF, b"J2000", ETCORR, R2.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::PXFORM(b"J2000", &FRNAME, ET, R3.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXM(R3.as_slice(), R2.as_slice(), R4.as_slice_mut());
                //
                // Extract the velocity vector.  Rotate to the current
                // frame, negate if indicated, then unitize.
                //
                spicelib::MXV(R4.as_slice(), STATE.subarray(4), V.as_slice_mut());
                spicelib::VSCL(AXSIGN, V.as_slice(), VTEMP.as_slice_mut());
                spicelib::VHAT(VTEMP.as_slice(), PRIVEC.as_slice_mut());

                //
                // PRIVEC should match the axis indicated by AXINDX.
                //
                testutil::CHCKAD(
                    b"PRIVEC",
                    PRIVEC.as_slice(),
                    b"~",
                    save.E.subarray([1, AXINDX]),
                    3,
                    DM12,
                    OK,
                    ctx,
                )?;
            } else if fstr::eq(&PRIVDF, b"CONSTANT") {
                //
                // The primary vector is a constant; the vector is
                // stored in PRIVEC.
                //
                // We need to use the position transformation from PRIVF
                // to FRMNAM for this one.  NOTE:  the epoch of this
                // transformation is critical. If PRIVF is non-inertial, we
                // need to adjust ET by the light time from the observer to
                // the center of PRIVF. We can actually finesse the issue
                // of whether the frame is non-inertial, since performing
                // the correction for an inertial frame has no effect.
                //
                spicelib::NAMFRM(&PRIVF, &mut J, ctx)?;
                spicelib::FRINFO(J, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"FRAME CENTER FOUND", FOUND, true, OK, ctx)?;

                spicelib::BODC2N(CENT, &mut CENTER, &mut FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"CENTER NAME FOUND", FOUND, true, OK, ctx)?;

                //
                // Parse the aberration correction for the primary
                // vector.  Get the light time to the frame center if
                // light time corrections are used.
                //
                spicelib::ZZPRSCOR(&PRICOR, PRIBLK.as_slice_mut(), ctx)?;

                if PRIBLK[LTIDX] {
                    //
                    // Get the light time CLT from the observer to the
                    // center of the frame in which the constant vector is
                    // defined.
                    //
                    spicelib::SPKEZR(
                        &CENTER,
                        ET,
                        &PRIVF,
                        &PRICOR,
                        &PRIOBS,
                        CSTATE.as_slice_mut(),
                        &mut CLT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    //
                    // The frame is not corrected for light time:
                    // the light time to the frame center is treated as
                    // zero.
                    //
                    CLT = 0.0;
                }

                //
                // Find the epoch associated with the center of the
                // frame by adjusting ET by CLT.
                //
                spicelib::ZZCOREPC(&PRICOR, ET, CLT, &mut ETCORR, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // We now want to express the constant vector in
                // the current dynamic frame at ET.  We must
                // transform the constant vector in two steps,
                // since the epoch associated with the frame in
                // which the constant vector is specified is ETCORR.
                // We first map the constant vector to the J2000
                // frame, then to the current frame.
                //
                // When the constant vector is expressed relative
                // to the J2000 frame, we apply the stellar aberration
                // correction if necessary.
                //
                // Find the position transformation from PRIVF at
                // ETCORR to J2000.  Apply to PRIVEC.
                //
                spicelib::PXFORM(&PRIVF, b"J2000", ETCORR, R2.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXV(R2.as_slice(), PRIVEC.as_slice(), V.as_slice_mut());

                if PRIBLK[STLIDX] {
                    //
                    // Stellar aberration correction is required.
                    //
                    // Find the state of the observer relative to the
                    // solar system barycenter at ET, in the J2000 frame.
                    //
                    spicelib::BODN2C(&PRIOBS, &mut J, &mut FOUND, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::SPKSSB(J, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if PRIBLK[XMTIDX] {
                        //
                        // Apply transmission stellar aberration correction.
                        //
                        spicelib::STLABX(
                            V.as_slice(),
                            STOBS.subarray(4),
                            VTEMP.as_slice_mut(),
                            ctx,
                        )?;
                    } else {
                        //
                        // Apply transmission stellar aberration correction.
                        //
                        spicelib::STELAB(
                            V.as_slice(),
                            STOBS.subarray(4),
                            VTEMP.as_slice_mut(),
                            ctx,
                        )?;
                    }

                    spicelib::VEQU(VTEMP.as_slice(), V.as_slice_mut());
                }
                //
                // The vector V contains our corrected (if necessary)
                // constant vector relative to J2000.
                //
                // Look up the rotation from J2000 to the current frame
                // at ET.  Apply to our constant vector.
                //
                spicelib::PXFORM(b"J2000", &FRNAME, ET, R3.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::MXV(R3.as_slice(), V.as_slice(), PRIVEC.as_slice_mut());

                //
                // Negate the constant vector if indicated, then unitize.
                //
                spicelib::VSCL(AXSIGN, PRIVEC.as_slice(), VTEMP.as_slice_mut());
                spicelib::VHAT(VTEMP.as_slice(), PRIVEC.as_slice_mut());

                //
                // PRIVEC should match the axis indicated by AXINDX.
                //
                testutil::CHCKAD(
                    b"PRIVEC",
                    PRIVEC.as_slice(),
                    b"~",
                    save.E.subarray([1, AXINDX]),
                    3,
                    DM12,
                    OK,
                    ctx,
                )?;
            } else {
                //
                // We have an unrecognized vector definition.
                //
                spicelib::SETMSG(b"F_DYN01 encountered vector definition #.", ctx);
                spicelib::ERRCH(b"#", &PRIVDF, ctx);
                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            //
            //
            //           We're now going to test the secondary vector.  We could
            //           have combined this code in a loop together with the tests
            //           for the primary vector, but "unrolling" the loop makes
            //           the test code a bit easier to look at.
            //
            //
            //--- Case: ------------------------------------------------------
            //
            spicelib::REPMI(
                b"PXFORM test number #.  Secondary vector def = #.",
                b"#",
                CASE,
                &mut CASMSG,
                ctx,
            );
            spicelib::REPMC(&CASMSG.clone(), b"#", &SECVDF, &mut CASMSG);

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::TCASE(&CASMSG, ctx)?;

            //
            // Find the secondary axis name and sign.
            //
            fstr::assign(&mut AXNAME, save.AXDEF.get([2, AXPAIR]));

            if (intrinsics::INDEX(&AXNAME, b"-") > 0) {
                AXSIGN = -1.0;
            } else {
                AXSIGN = 1.0;
            }

            spicelib::CMPRSS(b"-", 0, &AXNAME.clone(), &mut AXNAME);

            if spicelib::EQSTR(&AXNAME, b"X") {
                AXINDX = 1;
            } else if spicelib::EQSTR(&AXNAME, b"Y") {
                AXINDX = 2;
            } else {
                AXINDX = 3;
            }

            //
            // Create the secondary vector, specified relative to the base
            // frame.  Map this vector to the current dynamic frame.  The
            // component of this vector orthogonal to the primary vector
            // should match the current axis specification for the
            // secondary defining vector.
            //
            if fstr::eq(&SECVDF, b"OBSERVER_TARGET_POSITION") {
                spicelib::SPKPOS(
                    &SECTRG,
                    ET,
                    b"J2000",
                    &SECCOR,
                    &SECOBS,
                    V.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::PXFORM(b"J2000", &FRNAME, ET, R2.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXV(R2.as_slice(), V.as_slice(), VTEMP.as_slice_mut());
                spicelib::VSCL(AXSIGN, VTEMP.as_slice(), V.as_slice_mut());

                //
                // PRIVEC is left over from the tests on the primary
                // axis.  Replace SECVEC with its component orthogonal
                // to PRIVEC.  Unitize this component.
                //
                spicelib::VPERP(V.as_slice(), PRIVEC.as_slice(), VTEMP.as_slice_mut());
                spicelib::VHAT(VTEMP.as_slice(), SECVEC.as_slice_mut());

                //
                // SECVEC should match the axis indicated by AXINDX.
                //
                testutil::CHCKAD(
                    b"SECVEC",
                    SECVEC.as_slice(),
                    b"~",
                    save.E.subarray([1, AXINDX]),
                    3,
                    DM12,
                    OK,
                    ctx,
                )?;
            } else if fstr::eq(&SECVDF, b"TARGET_NEAR_POINT") {
                //
                // Find the specified near point.
                //
                spicelib::SUBPT(
                    b"NEAR POINT",
                    &SECTRG,
                    ET,
                    &SECCOR,
                    &SECOBS,
                    SPOINT.as_slice_mut(),
                    &mut ALT,
                    ctx,
                )?;

                //
                // We actually need to use the position transformation
                // from SECVF to FRMNAM for this one.  NOTE:  the
                // epoch of this transformation is critical.  We must
                // use the same epoch as was used by SUBPT.
                //
                // Get the light time LT from the observer to target
                // center.
                //
                spicelib::SPKPOS(
                    &SECTRG,
                    ET,
                    b"J2000",
                    &SECCOR,
                    &SECOBS,
                    POS.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Find the epoch associated with the target by
                // adjusting ET by LT.
                //
                spicelib::ZZCOREPC(&SECCOR, ET, LT, &mut ETCORR, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Find the position transformation from the target
                // body-fixed frame at ETCORR to J2000.  Convert
                // the body-fixed sub-point position to the J2000
                // frame.
                //
                spicelib::CNMFRM(&SECTRG, &mut J, &mut TRGFRM, &mut FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"TARGET FRAME FOUND", FOUND, true, OK, ctx)?;

                spicelib::PXFORM(&TRGFRM, b"J2000", ETCORR, R2.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXV(R2.as_slice(), SPOINT.as_slice(), V.as_slice_mut());

                //
                // Compute the observer-to-near point vector in the J2000
                // frame.
                //
                spicelib::VADD(V.as_slice(), POS.as_slice(), VTEMP.as_slice_mut());

                //
                // Find the position transformation J2000 the current frame
                // at ET.  Rotate the observer-near point vector into this
                // frame.
                //
                spicelib::PXFORM(b"J2000", &FRNAME, ET, R3.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXV(R3.as_slice(), VTEMP.as_slice(), V.as_slice_mut());

                //
                // Negate the vector if indicated, then unitize.
                //
                spicelib::VSCL(AXSIGN, V.as_slice(), VTEMP.as_slice_mut());

                //
                // PRIVEC is left over from the tests on the primary
                // axis.  Replace SECVEC with its component orthogonal
                // to PRIVEC.  Unitize this component.
                //
                spicelib::VPERP(VTEMP.as_slice(), PRIVEC.as_slice(), V.as_slice_mut());
                spicelib::VHAT(V.as_slice(), SECVEC.as_slice_mut());

                //
                // SECVEC should match the axis indicated by AXINDX.
                //
                testutil::CHCKAD(
                    b"SECVEC",
                    SECVEC.as_slice(),
                    b"~",
                    save.E.subarray([1, AXINDX]),
                    3,
                    DM12,
                    OK,
                    ctx,
                )?;
            } else if fstr::eq(&SECVDF, b"OBSERVER_TARGET_VELOCITY") {
                //
                // Look up the velocity vector in the specified frame.
                // Note this choice is critical:  any other frame,
                // unless related by a constant offset to the correct
                // frame, yields an invalid result.
                //
                spicelib::SPKEZR(
                    &SECTRG,
                    ET,
                    &SECVF,
                    &SECCOR,
                    &SECOBS,
                    STATE.as_slice_mut(),
                    &mut LT,
                    ctx,
                )?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;

                // WRITE (*,*) 'F_DYN01:'
                // WRITE (*,*) 'STATE = ', STATE
                // WRITE (*,*) 'SECVF = ', SECVF
                // WRITE (*,*) 'ET    = ', ET
                // CALL VHAT (STATE(4), UVEL )
                // WRITE (*, * ) 'UVEL = '
                // WRITE (*,'(1X,3E25.17)' ) UVEL

                //
                // We actually need to use the position transformation
                // from SECVF to FRMNAM for this one.  NOTE:  the
                // epoch of this transformation is critical.  We must
                // use the same epoch as was used by SPKEZR.  If
                // SECVF is non-inertial, we need to adjust ET by
                // the light time from the observer to the center of SECVF.
                // We can actually finesse the issue of whether the frame
                // is non-inertial, since performing the correction for
                // an inertial frame has no effect.
                //
                spicelib::NAMFRM(&SECVF, &mut J, ctx)?;
                spicelib::FRINFO(J, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"FRAME CENTER FOUND", FOUND, true, OK, ctx)?;

                spicelib::BODC2N(CENT, &mut CENTER, &mut FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"CENTER NAME FOUND", FOUND, true, OK, ctx)?;

                //
                // Get the light time CLT from the observer to the center
                // of the frame in which we looked up the velocity.
                //
                spicelib::SPKEZR(
                    &CENTER,
                    ET,
                    &SECVF,
                    &SECCOR,
                    &SECOBS,
                    CSTATE.as_slice_mut(),
                    &mut CLT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Find the epoch associated with the center of the
                // frame by adjusting ET by CLT.
                //
                spicelib::ZZCOREPC(&SECCOR, ET, CLT, &mut ETCORR, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // Find the position transformation from SECVF at
                // ETCORR to the current frame at ET.  We must do
                // this in two steps, passing through J2000.
                //
                spicelib::PXFORM(&SECVF, b"J2000", ETCORR, R2.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::PXFORM(b"J2000", &FRNAME, ET, R3.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXM(R3.as_slice(), R2.as_slice(), R4.as_slice_mut());
                //
                // Extract the velocity vector.  Rotate to the current
                // frame, negate if indicated, then unitize.
                //
                spicelib::MXV(R4.as_slice(), STATE.subarray(4), VTEMP.as_slice_mut());
                spicelib::VSCL(AXSIGN, VTEMP.as_slice(), V.as_slice_mut());

                //
                // PRIVEC is left over from the tests on the primary
                // axis.  Replace SECVEC with its component orthogonal
                // to PRIVEC.  Unitize this component.
                //
                spicelib::VPERP(V.as_slice(), PRIVEC.as_slice(), VTEMP.as_slice_mut());
                spicelib::VHAT(VTEMP.as_slice(), SECVEC.as_slice_mut());

                //
                // PRIVEC is left over from the tests on the primary
                // axis.  Replace SECVEC with its component orthogonal
                // to PRIVEC.
                //
                spicelib::VPERP(SECVEC.as_slice(), PRIVEC.as_slice(), VTEMP.as_slice_mut());
                spicelib::VEQU(VTEMP.as_slice(), SECVEC.as_slice_mut());

                //
                // SECVEC should match the axis indicated by AXINDX.
                //
                testutil::CHCKAD(
                    b"SECVEC",
                    SECVEC.as_slice(),
                    b"~",
                    save.E.subarray([1, AXINDX]),
                    3,
                    DM12,
                    OK,
                    ctx,
                )?;
            } else if fstr::eq(&SECVDF, b"CONSTANT") {
                //
                // The secondary vector is a constant; the vector is
                // stored in SECVEC.
                //
                // We need to use the position transformation from SECVF
                // to FRMNAM for this one.  NOTE:  the epoch of this
                // transformation is critical. If SECVF is non-inertial, we
                // need to adjust ET by the light time from the observer to
                // the center of SECVF. We can actually finesse the issue
                // of whether the frame is non-inertial, since performing
                // the correction for an inertial frame has no effect.
                //
                spicelib::NAMFRM(&SECVF, &mut J, ctx)?;
                spicelib::FRINFO(J, &mut CENT, &mut CLASS, &mut CLSSID, &mut FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"FRAME CENTER FOUND", FOUND, true, OK, ctx)?;

                spicelib::BODC2N(CENT, &mut CENTER, &mut FOUND, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSL(b"CENTER NAME FOUND", FOUND, true, OK, ctx)?;

                //
                // Parse the aberration correction for the secondary
                // vector.  Get the light time to the frame center if
                // light time corrections are used.
                //
                spicelib::ZZPRSCOR(&SECCOR, SECBLK.as_slice_mut(), ctx)?;

                if SECBLK[LTIDX] {
                    //
                    // Get the light time CLT from the observer to the
                    // center of the frame in which the constant vector is
                    // defined.
                    //
                    spicelib::SPKEZR(
                        &CENTER,
                        ET,
                        &SECVF,
                        &SECCOR,
                        &SECOBS,
                        CSTATE.as_slice_mut(),
                        &mut CLT,
                        ctx,
                    )?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;
                } else {
                    //
                    // The frame is not corrected for light time:
                    // the light time to the frame center is treated as
                    // zero.
                    //
                    CLT = 0.0;
                }

                //
                // Find the epoch associated with the center of the
                // frame by adjusting ET by CLT.
                //
                spicelib::ZZCOREPC(&SECCOR, ET, CLT, &mut ETCORR, ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                //
                // We now want to express the constant vector in
                // the current dynamic frame at ET.  We must
                // transform the constant vector in two steps,
                // since the epoch associated with the frame in
                // which the constant vector is specified is ETCORR.
                // We first map the constant vector to the J2000
                // frame, then to the current frame.
                //
                // When the constant vector is expressed relative
                // to the J2000 frame, we apply the stellar aberration
                // correction if necessary.
                //
                // Find the position transformation from SECVF at
                // ETCORR to J2000.  Apply to SECVEC.
                //
                spicelib::PXFORM(&SECVF, b"J2000", ETCORR, R2.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXV(R2.as_slice(), SECVEC.as_slice(), V.as_slice_mut());

                if SECBLK[STLIDX] {
                    //
                    // Stellar aberration correction is required.
                    //
                    // Find the state of the observer relative to the
                    // solar system barycenter at ET, in the J2000 frame.
                    //
                    spicelib::BODN2C(&SECOBS, &mut J, &mut FOUND, ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    spicelib::SPKSSB(J, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
                    testutil::CHCKXC(false, b" ", OK, ctx)?;

                    if SECBLK[XMTIDX] {
                        //
                        // Apply transmission stellar aberration correction.
                        //
                        spicelib::STLABX(
                            V.as_slice(),
                            STOBS.subarray(4),
                            VTEMP.as_slice_mut(),
                            ctx,
                        )?;
                    } else {
                        //
                        // Apply transmission stellar aberration correction.
                        //
                        spicelib::STELAB(
                            V.as_slice(),
                            STOBS.subarray(4),
                            VTEMP.as_slice_mut(),
                            ctx,
                        )?;
                    }

                    spicelib::VEQU(VTEMP.as_slice(), V.as_slice_mut());
                }

                //
                // The vector V contains our (corrected if necessary)
                // constant vector relative to J2000.
                //
                // Look up the rotation from J2000 to the current frame
                // at ET.  Apply to our constant vector.
                //
                spicelib::PXFORM(b"J2000", &FRNAME, ET, R3.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::MXV(R3.as_slice(), V.as_slice(), SECVEC.as_slice_mut());

                //
                // Negate the constant vector if indicated.
                //
                spicelib::VSCL(AXSIGN, SECVEC.as_slice(), VTEMP.as_slice_mut());

                //
                // PRIVEC is left over from the tests on the primary
                // axis.  Replace SECVEC with its component orthogonal
                // to PRIVEC.  Unitize this component.
                //
                spicelib::VPERP(VTEMP.as_slice(), PRIVEC.as_slice(), V.as_slice_mut());
                spicelib::VHAT(V.as_slice(), SECVEC.as_slice_mut());

                //
                // SECVEC should match the axis indicated by AXINDX.
                //
                testutil::CHCKAD(
                    b"SECVEC",
                    SECVEC.as_slice(),
                    b"~",
                    save.E.subarray([1, AXINDX]),
                    3,
                    DM12,
                    OK,
                    ctx,
                )?;
            } else {
                //
                // We have an unrecognized vector definition.
                //
                spicelib::SETMSG(b"F_DYN01 encountered vector definition #.", ctx);
                spicelib::ERRCH(b"#", &SECVDF, ctx);
                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            //
            //--- Case: ------------------------------------------------------
            //
            spicelib::REPMI(
                b"SXFORM test number #.  Test derivative block.",
                b"#",
                CASE,
                &mut CASMSG,
                ctx,
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::TCASE(&CASMSG, ctx)?;

            //
            // Perform "sanity checks" on the returned matrix.
            // Make sure the diagonal blocks are identical rotations,
            // compute a discrete derivative and compare to the
            // lower left block, and make sure the upper right
            // block is a zero matrix.
            //
            DELTA = 1.0;

            spicelib::PXFORM(&FRNAME, &BFRAME, (ET - DELTA), RMINUS.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::PXFORM(&FRNAME, &BFRAME, (ET + DELTA), RPLUS.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::T_XFORM(
                XFORM.as_slice(),
                RMINUS.as_slice(),
                RPLUS.as_slice(),
                DELTA,
                &mut NRMERR,
                &mut DETERR,
                &mut DRVERR,
                &mut DRLERR,
                DRDIFF.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check the error measurements we've made.  First
            // up is the determinant error.
            //
            TOL = DM14;
            testutil::CHCKSD(b"DETERR", DETERR, b"~", 0.0, TOL, OK, ctx)?;

            //
            // Check norms of rows and columns in the rotation
            // blocks.
            //
            testutil::CHCKSD(b"NRMERR", NRMERR, b"~", 0.0, TOL, OK, ctx)?;

            //
            // Check the derivative error when measured against
            // the discrete derivative.
            //
            MAXDER = intrinsics::DMAX1(&[MAXDER, DRVERR]);

            if ((intrinsics::INDEX(&PRICOR, b"S") > 0) || (intrinsics::INDEX(&SECCOR, b"S") > 0)) {
                TOL = DM4;
            // TOL = DM5
            } else {
                // TOL = DM5
                TOL = 0.0000005;
            }

            testutil::CHCKSD(b"DRVERR", DRVERR, b"~", 0.0, TOL, OK, ctx)?;

            //
            //    These statements may be useful for debugging.
            //
            //  IF ( .NOT. OK ) THEN
            //
            //     WRITE (*,*) '=================================== '
            //     WRITE (*,*) 'CASE = ', CASE
            //     WRITE (*,*) COORDS
            //     WRITE (*,*) 'PRIFRM  = ', PRIFRM
            //     WRITE (*,*) 'PRIJ    = ', PRIJ
            //     WRITE (*,*) 'PRIVF   = ', PRIVF
            //     WRITE (*,*) 'VDF     = ', VDF
            //     WRITE (*,*) 'SECVDF  = ', SECVDF
            //     WRITE (*,*) 'SECOBS  = ', SECOBS
            //     WRITE (*,*) 'SECTRG  = ', SECTRG
            //     WRITE (*,*) 'SECVF   = ', SECVF
            //     WRITE (*,*) 'VFRAME  = ', VFRAME
            //     WRITE (*,*) 'DRVERR  = ', DRVERR
            //     WRITE (*,*) 'PRIVEC  = ', PRIVEC
            //     WRITE (*,*) 'SECVEC  = ', SECVEC
            //     WRITE (*,*) 'DMAG    = ', DMAG
            //     WRITE (*,*) 'DM16  /DMAG    = ', DM16  /DMAG
            //
            //     CALL M33 ( 'DRDIFF', DRDIFF )
            //
            //    DO I = 1, DX-1
            //       WRITE (*,*) DEFTXT(I)
            //    END DO
            //
            //    WRITE (*,*) '=================================== '
            //
            // END IF

            //
            // Check the derivative relative error.  Perform this
            // test only if both aberration corrections are 'NONE'
            // and if the derivative error is not to close to
            // machine epsilon.
            //
            //

            //
            // Decide whether we'll do a derivative test.
            //
            TSTDRV = true;
            TOL = 0.0;

            if (fstr::eq(&PRICOR, b"NONE") && fstr::eq(&SECCOR, b"NONE")) {
                //
                // No aberration corrections are used.
                //
                // There are a few special cases where we must expect
                // quite inaccurate answers.  Set the tolerances for
                // these first.
                //
                // The Phobos-earth and earth-sun velocity vectors in the
                // IAU_PHOBOS and PHOBOS_RADIAL frames are particularly
                // problematic.
                //
                if (DRVERR < DM15) {
                    //
                    // The relative error is not meaningful; skip
                    // the test.
                    //
                    NSKIP = (NSKIP + 1);
                    TSTDRV = false;
                } else if fstr::eq(&PRIVF, b"PHOBOS_RADIAL") {
                    // TOL =  DM5
                    TOL = DM4;
                } else if (((fstr::eq(&PRIOBS, b"PHOBOS") && fstr::eq(&PRITRG, b"MARS"))
                    && fstr::eq(&PRIVF, b"IAU_PHOBOS"))
                    && fstr::eq(&PRIVDF, OTV))
                {
                    // TOL =  DM4
                    TOL = DM6;

                    NPR = (NPR + 1);

                    // WRITE (*,*) 'IAU_PHOBOS, NPR = ', NPR
                }

                if (((fstr::eq(&SECOBS, b"PHOBOS") && fstr::eq(&SECTRG, b"EARTH"))
                    && fstr::eq(&SECVF, b"IAU_PHOBOS"))
                    && fstr::eq(&SECVDF, OTV))
                {
                    TOL = intrinsics::DMAX1(&[TOL, DM4]);
                } else if (((fstr::eq(&SECOBS, b"EARTH") && fstr::eq(&SECTRG, b"SUN"))
                    && fstr::eq(&SECVF, b"PHOBOS_RADIAL"))
                    && fstr::eq(&SECVDF, OTV))
                {
                    TOL = intrinsics::DMAX1(&[TOL, DM3]);
                } else if (((fstr::eq(&SECOBS, b"EARTH") && fstr::eq(&SECTRG, b"SUN"))
                    && fstr::eq(&SECVF, b"IAU-EARTH"))
                    && fstr::eq(&SECVDF, OTV))
                {
                    TOL = intrinsics::DMAX1(&[TOL, DM3]);
                } else if (((fstr::eq(&SECOBS, b"PHOBOS") && fstr::eq(&SECTRG, b"EARTH"))
                    && fstr::eq(&SECVF, b"PHOBOS_RADIAL"))
                    && fstr::eq(&SECVDF, OTV))
                {
                    TOL = intrinsics::DMAX1(&[TOL, DM2]);
                }

                if (TOL == 0.0) {
                    if (DMAG > DM15) {
                        //
                        // This is the normal case.
                        //
                        // NNORML = NNORML + 1

                        // TOL = MAX ( 10.D0**( -16 - LOG10(DMAG) ),  DM6 )
                        TOL = intrinsics::DMAX1(&[
                            f64::powf(10.0, ((-16 as f64) - f64::log10(DMAG))),
                            DM5,
                        ]);
                    } else if (DMAG == 0.0) {
                        //
                        // This should happen only for constant frames,
                        // in which case estimated and actual derivatives
                        //  should---we think---be identically zero.
                        //
                        TOL = 0.0;
                    } else {
                        //
                        // No point in testing the relative error---the
                        // derivative is too small for the discrete estimate
                        // to have any validity.
                        //
                        NSKIP = (NSKIP + 1);
                        TSTDRV = false;
                    }
                }
            //
            // The tolerance has been set for the pure geometric
            // cases.
            //
            } else if ((((((intrinsics::INDEX(&PRICOR, b"LT") > 0)
                || (intrinsics::INDEX(&SECCOR, b"LT") > 0))
                || (intrinsics::INDEX(&PRICOR, b"CN") > 0))
                || (intrinsics::INDEX(&SECCOR, b"CN") > 0))
                || (intrinsics::INDEX(&PRICOR, b"S") > 0))
                || (intrinsics::INDEX(&SECCOR, b"S") > 0))
            {
                //
                // We're using light time corrections possibly
                // stellar aberration corrections).
                //
                // There are several special cases where we must expect
                // quite inaccurate answers.  Set the tolerances for
                // these first.
                //
                // The phobos-earth velocity vector in the IAU_PHOBOS
                // frame is particularly problematic.
                //
                BIGERR = false;

                if (((fstr::eq(&SECOBS, b"PHOBOS") && fstr::eq(&SECTRG, b"EARTH"))
                    && fstr::eq(&SECVF, b"IAU_PHOBOS"))
                    && fstr::eq(&SECVDF, OTV))
                {
                    BIGERR = true;
                    TOL = DM4;
                } else if (((fstr::eq(&PRIVDF, b"CONSTANT") && fstr::eq(&PRIVF, b"GSE"))
                    && fstr::eq(&SECOBS, b"EARTH"))
                    && fstr::eq(&SECTRG, b"SUN"))
                {
                    //
                    // The velocity is just noise for many of these
                    // cases, so the relative error is meaningless.
                    //
                    BIGERR = true;
                    TOL = 0.0;
                    TSTDRV = false;
                } else if (((fstr::eq(&SECOBS, b"EARTH") && fstr::eq(&SECTRG, b"SUN"))
                    && fstr::eq(&SECVF, b"PHOBOS_RADIAL"))
                    && fstr::eq(&SECVDF, OTV))
                {
                    BIGERR = true;
                    TOL = DM2;
                } else if (((fstr::eq(&SECOBS, b"PHOBOS") && fstr::eq(&SECTRG, b"EARTH"))
                    && fstr::eq(&SECVF, b"PHOBOS_RADIAL"))
                    && fstr::eq(&SECVDF, OTV))
                {
                    BIGERR = true;
                    TOL = DM2;
                } else if (fstr::eq(&PRIVDF, b"CONSTANT") && fstr::eq(&SECVDF, b"CONSTANT")) {
                    //
                    // The velocity is just noise for many of these
                    // cases, so the relative error is meaningless.
                    //
                    BIGERR = true;
                    TOL = 0.0;
                    TSTDRV = false;
                }

                //
                // Further tolerance adjustments may be needed if we're
                // using stellar aberration.
                //
                if TSTDRV {
                    if (intrinsics::INDEX(&PRICOR, b"S") > 0) {
                        if (fstr::eq(&PRIOBS, b"PHOBOS") && fstr::eq(&PRIVF, b"GSE")) {
                            BIGERR = true;
                            TOL = DM1;
                        } else if (fstr::eq(&PRIOBS, b"PHOBOS")
                            && fstr::eq(&PRIVF, b"PHOBOS_RADIAL"))
                        {
                            BIGERR = true;
                            TOL = DM3;
                        } else if (fstr::eq(&PRIOBS, b"PHOBOS") && fstr::eq(&PRIVDF, OTP)) {
                            BIGERR = true;
                            TOL = DM3;
                        }

                        if ((fstr::eq(&PRIOBS, b"EARTH") && fstr::eq(&PRITRG, b"SUN"))
                            && fstr::eq(&PRIVDF, OTV))
                        {
                            BIGERR = true;

                            if fstr::eq(&PRIVF, b"PHOBOS_RADIAL") {
                                //
                                // The derivative in this case suffers
                                // from total loss of precision.
                                //

                                TSTDRV = false;
                                TOL = 0.0;
                            } else {
                                //
                                // We have about one digit of information here.
                                //
                                TOL = DM1;
                            }
                        }
                    }

                    if (intrinsics::INDEX(&SECCOR, b"S") > 0) {
                        if ((fstr::eq(&SECOBS, b"EARTH") && fstr::eq(&SECTRG, b"SUN"))
                            && fstr::eq(&SECVDF, OTV))
                        {
                            BIGERR = true;

                            if fstr::eq(&SECVF, b"PHOBOS_RADIAL") {
                                //
                                // The derivative in this case suffers
                                // from total loss of precision.
                                //

                                TSTDRV = false;
                                TOL = 0.0;
                            } else {
                                //
                                // We have about one digit of information here.
                                //
                                TOL = DM1;
                            }
                        } else if (fstr::eq(&SECOBS, b"PHOBOS") && fstr::eq(&SECTRG, b"EARTH")) {
                            BIGERR = true;
                            TOL = DM1;
                        } else if (fstr::eq(&SECOBS, b"EARTH")
                            && fstr::eq(&SECVF, b"PHOBOS_RADIAL"))
                        {
                            BIGERR = true;
                            TOL = intrinsics::DMAX1(&[TOL, DM3]);
                        }
                    }
                }

                //
                // Set the tolerance for the "normal" cases.
                //
                if (TSTDRV && !BIGERR) {
                    if (DMAG > DM10) {
                        //
                        // The derivative magnitude is large enough to
                        // squeeze out about four digits of precision.
                        //
                        TOL = DM4;
                    } else if (DMAG > DM12) {
                        //
                        // We may expect about two digits of agreement
                        // between the computed as estimated derivative.
                        //
                        TOL = DM2;
                    } else if (DMAG == 0.0) {
                        //
                        // This should happen only for constant frames,
                        // in which case estimated and actual derivatives
                        // should---we think---be identically zero.
                        //
                        TOL = 0.0;
                    } else {
                        //
                        // No point in testing the relative error---the
                        // derivative is too small for the discrete estimate
                        // to have any validity.
                        //
                        NSKIP = (NSKIP + 1);
                        TSTDRV = false;
                    }
                }
                //
                // TOL has been set for the aberration correction cases.
                //
            }
            //
            // TOL has been set.
            //

            if TSTDRV {
                TOL = intrinsics::DMAX1(&[TOL, (0.0000001 / DMAG)]);

                // WRITE (*,*) 'USING TOL = ', TOL
                testutil::CHCKSD(b"DRLERR", DRLERR, b"~", 0.0, TOL, OK, ctx)?;

                //
                //     These statements may be useful for debugging.
                //
                // IF ( .NOT. OK ) THEN
                //
                //    WRITE (*,*) '=================================== '
                //    WRITE (*,*) 'CASE = ', CASE
                //    WRITE (*,*) COORDS
                //    WRITE (*,*) 'PRIFRM  = ', PRIFRM
                //    WRITE (*,*) 'PRIJ    = ', PRIJ
                //    WRITE (*,*) 'PRIVDF  = ', PRIVDF
                //    WRITE (*,*) 'PRIOBS  = ', PRIOBS
                //    WRITE (*,*) 'PRITRG  = ', PRITRG
                //    WRITE (*,*) 'PRIVF   = ', PRIVF
                //    WRITE (*,*) 'VDF     = ', VDF
                //    WRITE (*,*) 'SECVDF  = ', SECVDF
                //    WRITE (*,*) 'SECOBS  = ', SECOBS
                //    WRITE (*,*) 'SECTRG  = ', SECTRG
                //    WRITE (*,*) 'SECVF   = ', SECVF
                //    WRITE (*,*) 'VFRAME  = ', VFRAME
                // IF ( ISVEL(1) .AND. ISVEL(2) ) THEN
                //    WRITE (*,*) 'DRVERR, DRLERR  = ', DRVERR, DRLERR
                // END IF
                //    WRITE (*,*) 'PRIVEC  = ', PRIVEC
                //    WRITE (*,*) 'SECVEC  = ', SECVEC
                //     WRITE (*,*) 'DMAG    = ', DMAG
                //     WRITE (*,*) '1.D-7/DMAG    = ', 1.D-7/DMAG
                //     WRITE (*,*) '1.D-8/DMAG    = ', 1.D-8/DMAG
                //    WRITE (*,*) 'DM16  /DMAG    = ', DM16  /DMAG
                //
                //    CALL M33 ( 'DRDIFF', DRDIFF )
                //    CALL M66 ( 'XFORM',  XFORM  )

                // CALL XF2RAV ( XFORM, R, V )

                // WRITE (*,*) 'Frame angular velocity: ', V
                // WRITE (*,*) '||V|| = ', VNORM(V)
                //

                //  WRITE (*,*) 'F_DYN01:'
                //  WRITE (*,*) 'STATE = ', STATE
                // WRITE (*,*) 'SECVF = ', SECVF
                // WRITE (*,*) 'ET    = ', ET
                // CALL VHAT (STATE(4), UVEL )
                // WRITE (*, * ) 'UVEL = '
                // WRITE (*,'(1X,3E25.17)' ) UVEL

                //    DO I = 1, DX-1
                //       WRITE (*,*) DEFTXT(I)
                //    END DO
                //
                //    WRITE (*,*) '=================================== '
                //
                // END IF
                //
            }
        }
    }

    // WRITE (*,*) 'Number of cases rejected: ', NOGO
    // WRITE (*,*) 'MAXDER = ', MAXDER
    // WRITE (*,*) 'NSKIP  = ', NSKIP

    //  WRITE (*,*) 'NNORML = ', NNORML
    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
