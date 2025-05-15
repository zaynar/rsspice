//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const PCK: &[u8] = b"test_dyn.tpc";
const SPK: &[u8] = b"test_dyn.bsp";
const FRZTOK: &[u8] = b"@2010-JAN-1/00:00:00";
const DM19: f64 = 0.0000000000000000001;
const DM16: f64 = 0.0000000000000001;
const DM14: f64 = 0.00000000000001;
const DM12: f64 = 0.000000000001;
const DM11: f64 = 0.00000000001;
const DM9: f64 = 0.000000001;
const DM6: f64 = 0.000001;
const DM4: f64 = 0.0001;
const NAMLEN: i32 = 32;
const LNSIZE: i32 = 80;
const MAXDEF: i32 = 50;
const MAXTOK: i32 = 7;
const TIMLEN: i32 = 50;
const MSGLEN: i32 = 240;
const NDIMS: i32 = 7;
const NCOSET: i32 = 3;
const COFIDX: i32 = 1;
const MCOIDX: i32 = 3;
const NSEQ: i32 = 2;
const SEQIDX: i32 = (COFIDX + 1);
const MSQIDX: i32 = 1;
const NUNITS: i32 = 2;
const UNTIDX: i32 = (SEQIDX + 1);
const NEPOCS: i32 = 2;
const EPCIDX: i32 = (UNTIDX + 1);
const NBASEF: i32 = 3;
const BFRIDX: i32 = (EPCIDX + 1);
const NCENTR: i32 = 2;
const CTRIDX: i32 = (BFRIDX + 1);
const NRSTAT: i32 = 3;
const RSTIDX: i32 = (CTRIDX + 1);

struct SaveVars {
    BASEFR: ActualCharArray,
    COFSTR: ActualCharArray2D,
    EPCSTR: ActualCharArray,
    RSTATE: ActualCharArray,
    UNITS: ActualCharArray,
    ZR: StackArray2D<f64, 9>,
    AXESEQ: StackArray2D<i32, 6>,
    CTRCDE: StackArray<i32, 2>,
    DIMS: StackArray<i32, 7>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BASEFR = ActualCharArray::new(NAMLEN, 1..=NBASEF);
        let mut COFSTR = ActualCharArray2D::new(LNSIZE, 1..=3, 1..=NCOSET);
        let mut EPCSTR = ActualCharArray::new(LNSIZE, 1..=NEPOCS);
        let mut RSTATE = ActualCharArray::new(LNSIZE, 1..=NRSTAT);
        let mut UNITS = ActualCharArray::new(LNSIZE, 1..=NUNITS);
        let mut ZR = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut AXESEQ = StackArray2D::<i32, 6>::new(1..=3, 1..=NSEQ);
        let mut CTRCDE = StackArray::<i32, 2>::new(1..=NCENTR);
        let mut DIMS = StackArray::<i32, 7>::new(1..=NDIMS);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(3),
                Val::I(1),
                Val::I(3),
                Val::I(2),
                Val::I(3),
                Val::I(1),
            ]
            .into_iter();
            AXESEQ
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"J2000"), Val::C(b"IAU_MARS"), Val::C(b"GSE")].into_iter();
            BASEFR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"( 1           2E-8        3E-13 )"),
                Val::C(b"( 10          2E-7        3E-12 )"),
                Val::C(b"( 100         2E-6        3E-11 )"),
                Val::C(b"( 2                             )"),
                Val::C(b"( 4           5E-7              )"),
                Val::C(b"( 6           7E-7        8E-12 )"),
                Val::C(b"(  -47.68143  0.33621061170684714E-10 )"),
                Val::C(b"(  -37.1135  -0.19298045478743630E-10 )"),
                Val::C(b"( -176.630   -0.40612497946759260E-02 )"),
            ]
            .into_iter();
            COFSTR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(399), Val::I(499)].into_iter();
            CTRCDE
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist =
                [Val::C(b"@2000-JAN-1/12:00"), Val::C(b"@2001-JAN-1/12:00")].into_iter();
            EPCSTR
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

            let mut clist = [Val::C(KVROTG), Val::C(KVINRT), Val::C(b"FROZEN")].into_iter();
            RSTATE
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(NCOSET),
                Val::I(NSEQ),
                Val::I(NUNITS),
                Val::I(NEPOCS),
                Val::I(NBASEF),
                Val::I(NCENTR),
                Val::I(NRSTAT),
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
            BASEFR,
            COFSTR,
            EPCSTR,
            RSTATE,
            UNITS,
            ZR,
            AXESEQ,
            CTRCDE,
            DIMS,
        }
    }
}

//$Procedure F_DYN04 ( Dynamic Frame Test Family 04 )
pub fn F_DYN04(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ANGCOF = ActualCharArray::new(LNSIZE, 1..=3);
    let mut AXESTR = [b' '; LNSIZE as usize];
    let mut BFRAME = [b' '; NAMLEN as usize];
    let mut CASMSG = [b' '; MSGLEN as usize];
    let mut CTRNAM = [b' '; NAMLEN as usize];
    let mut DEFTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut EPOCH = [b' '; LNSIZE as usize];
    let mut FRZEPC = [b' '; TIMLEN as usize];
    let mut FRNAME = [b' '; NAMLEN as usize];
    let mut INUNIT = [b' '; LNSIZE as usize];
    let mut KEYWRD = [b' '; LNSIZE as usize];
    let mut RSTSTR = [b' '; NAMLEN as usize];
    let mut TIMSTR = [b' '; LNSIZE as usize];
    let mut TOKENS = ActualCharArray::new(LNSIZE, 1..=MAXTOK);
    let mut UNTSTR = [b' '; LNSIZE as usize];
    let mut FRSTEM = [b' '; 18 as usize];
    let mut COEFFS = StackArray::<f64, 6>::new(0..=(MAXTOK - 2));
    let mut DELTA: f64 = 0.0;
    let mut DETERR: f64 = 0.0;
    let mut DMAG: f64 = 0.0;
    let mut DP: f64 = 0.0;
    let mut DRDIFF = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DRLERR: f64 = 0.0;
    let mut DRV2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DRVBLK = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DRVERR: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut EULANG = StackArray::<f64, 6>::new(1..=6);
    let mut NRMERR: f64 = 0.0;
    let mut P: f64 = 0.0;
    let mut R = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut R2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut REFEPC: f64 = 0.0;
    let mut RMINUS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut RPLUS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut T: f64 = 0.0;
    let mut T0: f64 = 0.0;
    let mut TIPM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TOL: f64 = 0.0;
    let mut TSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XF2 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFB2J = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFF2J = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut AXES = StackArray::<i32, 3>::new(1..=3);
    let mut COORDS = StackArray::<i32, 7>::new(1..=NDIMS);
    let mut CENTER: i32 = 0;
    let mut DX: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut N: i32 = 0;
    let mut NCART: i32 = 0;
    let mut FOUND: bool = false;
    let mut ISMARS: bool = false;
    let mut GO: bool = false;

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
    // The dimensions of the test parameter space are:
    //
    //     1) Coefficient set
    //     2) Axis sequence
    //     3) Units
    //     4) Epoch
    //     5) Base frame
    //     6) Center
    //     7) Rotation state
    //
    // Number of dimensions of the test parameter space:
    //

    //
    // Number of coefficient sets:
    //

    //
    // The constant COFIDX refers to the ordinal position of the
    // dimension corresponding to the coefficient set in the parameter
    // space.
    //

    //
    // Index of coefficient set for Mars test case:
    //

    //
    // Number of axis sequences:
    //

    //
    // Index of axis sequence for Mars test case:
    //

    //
    // Number of unit selections:
    //

    //
    // Number of epoch selections:
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
    // Number of frame center cases:
    //

    //
    // Number of rotation states:
    //

    //
    // The constant RSTIDX refers to the ordinal position of the
    // dimension corresponding to the rotation state in the parameter
    // space.
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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Euler angle axis sequences:
    //

    //
    // We use both inertial, PCK-based, and dynamic base frames.
    //

    //
    // Polynomial coefficient sets:
    //
    // The last set is based on the coefficient for Mars from
    // pck00008.tpc.  That PCK contains the assignments
    //
    //    BODY499_POLE_RA          = (  317.68143   -0.1061      0.  )
    //    BODY499_POLE_DEC         = (   52.88650   -0.0609      0.  )
    //    BODY499_PM               = (  176.630    350.89198226  0.  )
    //
    // The angles shown here are obtained from those angles via
    // the transformations
    //
    //    Angle 1 is -Pi/2 - RA, mapped into the range [0, 360).
    //    Angle 2 is -Pi/2 + Dec.
    //    Angle 3 is -PM.
    //
    // Units have been changed as well:
    //
    //    RA/Dec terms in the PCK are in degrees and degrees/century;
    //    the rates here have been converted to degrees/sec. Prime
    //    meridian terms in the PCK are in degrees and degrees/day; the
    //    rate here has been converted to degrees/sec.
    //
    //

    //
    // ID codes of frame centers:
    //

    //
    // Reference epochs for polynomial definitions:
    //

    //
    // Angular units:
    //

    //
    // Rotation states:
    //

    //
    // DIMS defines the dimensions of the cartesian product of the
    // input parameters.  The cardinality of the set comprising the
    // Nth "factor" of the cartesian product is DIMS(N).  The cardinality
    // of the product itself is the product of the elements of DIMS.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_DYN04", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create test inputs for comprehensive Euler frame test.",
        ctx,
    )?;

    //
    // Create and load kernels.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK, true, &mut HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_PCK08(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Parse the constant string used for the freeze epoch.
    // First remove the '@' and '/' characters from the string.
    // Add TDB token.
    //
    spicelib::CMPRSS(b"@", 0, FRZTOK, &mut FRZEPC);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::REPLCH(&FRZEPC.clone(), b"/", b" ", &mut FRZEPC);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SUFFIX(b"TDB", 1, &mut FRZEPC);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&FRZEPC, &mut T0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define the GSE frame.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_GSE                        =  2399001",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2399001_NAME               = \'GSE\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_2399001_CLASS              =  5");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_2399001_CLASS_ID           =  2399001",
    );
    fstr::assign(
        DEFTXT.get_mut(5),
        b"FRAME_2399001_CENTER             =  399",
    );
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_2399001_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        DEFTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSTYL), b"       = \'"),
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
                    &fstr::concat(b"FRAME_2399001_", KWFFAM),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWPRI), KWVAXI),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWPRI), KWVCDF),
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
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWPRI), KWVOBS),
            b"       = \'EARTH\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWPRI), KWVTRG),
            b"         = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWPRI), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVAXI),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVCDF),
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
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVOBS),
            b"       = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(17),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVTRG),
            b"       = \'EARTH\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(18),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(19),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWSEC), KWVFRM),
            b"          = \'J2000\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(20),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399001_", KWRSTA), b"       =  \'"),
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
    // We use MULTIX and the array DIMS to define a mapping that allows
    // us to map a test case number to a set of input parameters.
    //
    // Compute the number of test cases in the input cartesian product
    // space.
    //
    NCART = spicelib::PRODAI(save.DIMS.as_slice(), NDIMS);

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
        // Start out with the coefficient sets.
        //
        for I in 1..=3 {
            fstr::assign(ANGCOF.get_mut(I), save.COFSTR.get([I, COORDS[COFIDX]]));
        }

        //
        // Set the axis sequence.
        //
        for I in 1..=3 {
            AXES[I] = save.AXESEQ[[I, COORDS[SEQIDX]]];
        }

        //
        // Set the angular units.
        //
        fstr::assign(&mut UNTSTR, save.UNITS.get(COORDS[UNTIDX]));

        //
        // Set the epoch.
        //
        fstr::assign(&mut EPOCH, save.EPCSTR.get(COORDS[EPCIDX]));

        //
        // Set the base frame.
        //
        fstr::assign(&mut BFRAME, save.BASEFR.get(COORDS[BFRIDX]));

        //
        // Set the frame center.  Obtain the center as a name.
        //
        CENTER = save.CTRCDE[COORDS[CTRIDX]];

        spicelib::BODC2N(CENTER, &mut CTRNAM, &mut FOUND, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        //
        // Set the rotation state.
        //
        fstr::assign(&mut RSTSTR, save.RSTATE.get(COORDS[RSTIDX]));

        //
        // Decide whether the current test case is the special case
        // for Mars.
        //
        ISMARS = ((((((CENTER == 499) && fstr::eq(&BFRAME, b"J2000"))
            && fstr::eq(&EPOCH, b"@2000-JAN-1/12:00"))
            && fstr::eq(&UNTSTR, b"\'DEGREES\'"))
            && (COORDS[SEQIDX] == MSQIDX))
            && (COORDS[COFIDX] == MCOIDX));

        //
        // For this test family, there are no cases that must be
        // rejected, so the "GO" flag is always .TRUE.  We keep
        // this currently unused flag to simplify maintenance of
        // this code.
        //
        GO = true;

        if GO {
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
            fstr::assign(&mut FRNAME, b"EULER_DYN_FRAME");
            FRCODE = -2000000000;

            fstr::assign(DEFTXT.get_mut(1), b"FRAME_#           = #");
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
            // The frame center is specified via a name.
            //
            fstr::assign(
                DEFTXT.get_mut(5),
                &fstr::concat(&FRSTEM, b"CENTER           = \'#\'"),
            );

            spicelib::REPMC(&DEFTXT[5].to_vec(), b"#", &CTRNAM, &mut DEFTXT[5]);
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
                        &fstr::concat(&fstr::concat(&FRSTEM, KWSTYL), b"    = \'"),
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
                        &fstr::concat(&fstr::concat(&FRSTEM, KWFFAM), b"           = \'"),
                        KVEULR,
                    ),
                    b"\'",
                ),
            );

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&DEFTXT[8].to_vec(), b"#", FRCODE, &mut DEFTXT[8], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Set the epoch for the polynomials.
            //
            fstr::assign(
                DEFTXT.get_mut(9),
                &fstr::concat(
                    &fstr::concat(&fstr::concat(&FRSTEM, KWEPOC), b"            = "),
                    &EPOCH,
                ),
            );

            //
            // Set the units for the polynomial coefficients.
            //
            fstr::assign(
                DEFTXT.get_mut(10),
                &fstr::concat(
                    &fstr::concat(&fstr::concat(&FRSTEM, KWUNIT), b"            = "),
                    &UNTSTR,
                ),
            );

            //
            // Set the axis sequence.
            //
            fstr::assign(&mut AXESTR, b"( # # # )");

            for I in 1..=3 {
                spicelib::REPMI(&AXESTR.clone(), b"#", AXES[I], &mut AXESTR, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;
            }

            fstr::assign(
                DEFTXT.get_mut(11),
                &fstr::concat(
                    &fstr::concat(&fstr::concat(&FRSTEM, KWEUAX), b"             = "),
                    &AXESTR,
                ),
            );

            //
            // Set the coefficients.
            //
            fstr::assign(
                DEFTXT.get_mut(12),
                &fstr::concat(
                    &fstr::concat(&fstr::concat(&FRSTEM, KWEAC1), b"   = "),
                    ANGCOF.get(1),
                ),
            );
            fstr::assign(
                DEFTXT.get_mut(13),
                &fstr::concat(
                    &fstr::concat(&fstr::concat(&FRSTEM, KWEAC2), b"   = "),
                    ANGCOF.get(2),
                ),
            );
            fstr::assign(
                DEFTXT.get_mut(14),
                &fstr::concat(
                    &fstr::concat(&fstr::concat(&FRSTEM, KWEAC3), b"   = "),
                    ANGCOF.get(3),
                ),
            );

            //
            // Set the frame's rotation state or freeze epoch.
            //
            if fstr::eq(&RSTSTR, b"FROZEN") {
                fstr::assign(
                    DEFTXT.get_mut(15),
                    &fstr::concat(
                        &fstr::concat(&fstr::concat(&FRSTEM, KWFREZ), b"     = "),
                        FRZTOK,
                    ),
                );
            } else {
                fstr::assign(
                    DEFTXT.get_mut(15),
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(&fstr::concat(&FRSTEM, KWRSTA), b"     = \'"),
                            fstr::substr(&RSTSTR, 1..=spicelib::RTRIM(&RSTSTR)),
                        ),
                        b"\'",
                    ),
                );
            }

            DX = 15;

            //
            // Enter the frame definition into the kernel pool.
            //
            spicelib::LMPOOL(DEFTXT.as_arg(), DX, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            //           Here's where the testing begins.
            //
            //
            //--- Case: ------------------------------------------------------
            //
            spicelib::REPMI(
                b"SXFORM/PXFORM test number #A.  Test results against those from PXFORM.",
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
            ET = (1000.0 * (CASE - 100) as f64);

            spicelib::SXFORM(&FRNAME, &BFRAME, ET, XFORM.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::PXFORM(&FRNAME, &BFRAME, ET, R.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Extract the derivative block from XFORM into DRVBLK.
            // We'll use this block later; we're extracting it here
            // to keep this code close to the associated SXFORM call.
            //
            for I in 1..=3 {
                for J in 1..=3 {
                    DRVBLK[[J, I]] = XFORM[[(J + 3), I]];
                }
            }

            DMAG = spicelib::VNORMG(DRVBLK.as_slice(), 9);

            //
            // Extract the rotation block from XFORM into R2.
            //
            for I in 1..=3 {
                for J in 1..=3 {
                    R2[[J, I]] = XFORM[[J, I]];
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
            //--- Case: ------------------------------------------------------
            //
            spicelib::REPMI(
                b"SXFORM test number #B.  Test derivative block.",
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
            ET = (1000.0 * (CASE - 100) as f64);

            spicelib::SXFORM(&FRNAME, &BFRAME, ET, XFORM.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Perform "sanity checks" on the returned matrix.
            // Make sure the diagonal blocks are identical rotations,
            // compute a discrete derivative and compare to the
            // lower left block, and make sure the upper right
            // block is a zero matrix.
            //
            // The value of DELTA (1/16) was determined by experimentation.
            //
            DELTA = 0.0625;

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
            // If the frame is considered to be rotating, compare the
            // discrete derivative to the lower left block.
            //
            if spicelib::EQSTR(&RSTSTR, KVROTG) {
                //
                // Check the derivative absolute error.
                //
                TOL = DM6;
                testutil::CHCKSD(b"DRVERR", DRVERR, b"~", 0.0, TOL, OK, ctx)?;

                //
                // Check the derivative relative error.
                //
                TOL = DM4;
                testutil::CHCKSD(b"DRLERR", DRLERR, b"~", 0.0, TOL, OK, ctx)?;
            } else if spicelib::EQSTR(&RSTSTR, KVINRT) {
                //
                // The frame is supposed to be inertial, so the
                // state transformation mapping from this frame to
                // J2000 ideally should have a zero derivative block.
                // See whether the derivative block of the latter
                // transformation is close to zero.
                //
                spicelib::SXFORM(&BFRAME, b"J2000", ET, XFB2J.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::MXMG(
                    XFB2J.as_slice(),
                    XFORM.as_slice(),
                    6,
                    6,
                    6,
                    XFF2J.as_slice_mut(),
                );

                for I in 1..=3 {
                    for J in 1..=3 {
                        DRV2[[J, I]] = XFF2J[[(J + 3), I]];
                    }
                }

                testutil::CHCKAD(
                    b"Derivative block",
                    DRV2.as_slice(),
                    b"~",
                    save.ZR.as_slice(),
                    9,
                    DM19,
                    OK,
                    ctx,
                )?;
            } else {
                //
                // The frozen case is quite simple:  the derivative
                // with respect to the base frame is zero.
                //
                testutil::CHCKAD(
                    b"Derivative block",
                    DRVBLK.as_slice(),
                    b"=",
                    save.ZR.as_slice(),
                    9,
                    0.0,
                    OK,
                    ctx,
                )?;
            }

            //
            //--- Case: ------------------------------------------------------
            //
            spicelib::REPMI(b"SXFORM test number #C. Construct state transformation locally; compare to XFORM output.", b"#", CASE, &mut CASMSG, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::TCASE(&CASMSG, ctx)?;

            //
            // Parse the reference epoch; compute the evaluation epoch. If
            // the rotation state is frozen, use the "freeze" value.
            //
            spicelib::CMPRSS(b"@", 0, &EPOCH, &mut TIMSTR);
            spicelib::SUFFIX(b"TDB", 1, &mut TIMSTR);

            spicelib::STR2ET(&TIMSTR, &mut REFEPC, ctx)?;

            if spicelib::EQSTR(&RSTSTR, b"FROZEN") {
                T = (T0 - REFEPC);
            } else {
                T = (ET - REFEPC);
            }

            //
            // Compute the Euler angles and their derivatives;
            // scale to radians and radians/sec.
            //
            for I in 1..=3 {
                //
                // Parse the string '( <coeff 0> <coeff 1> ... <coeff n> )'
                //
                spicelib::LPARSM(&ANGCOF[I], b" ()", MAXTOK, &mut N, TOKENS.as_arg_mut());

                //
                // The first token will be the null token preceding
                // the first parenthesis.  The next 1-3 tokens are
                // numbers.  The last token is also null.
                //
                for J in 2..=(N - 1) {
                    spicelib::PRSDP(&TOKENS[J], &mut COEFFS[(J - 2)], ctx)?;
                }

                // WRITE (*,*) '-------------------------------------'
                // WRITE (*,*) ( COEFFS(J), J = 0, N-3 )
                // WRITE (*,*) '-------------------------------------'

                //
                // Evaluate the polynomial P representing the angle
                // and its time derivative DP at T.
                //
                P = 0.0;

                for J in intrinsics::range((N - 3), 0, -1) {
                    P = (COEFFS[J] + (T * P));
                }

                DP = 0.0;

                for J in intrinsics::range((N - 3), 1, -1) {
                    DP = ((J as f64) * (COEFFS[J] + (T * DP)));
                }

                //
                // Convert angular units to radians.
                //
                spicelib::CMPRSS(b"\'", 0, &UNTSTR, &mut INUNIT);

                spicelib::CONVRT(P, &INUNIT, KVRADN, &mut EULANG[I], ctx)?;
                spicelib::CONVRT(DP, &INUNIT, KVRADN, &mut EULANG[(I + 3)], ctx)?;
            }

            //
            // Compute the state transformation matrix.
            //
            spicelib::EUL2XF(
                EULANG.as_slice(),
                AXES[1],
                AXES[2],
                AXES[3],
                XF2.as_slice_mut(),
                ctx,
            )?;

            //
            // Now compare XF2 to XFORM.
            //
            // Extract the derivative block from XF2.
            //
            for I in 1..=3 {
                for J in 1..=3 {
                    DRV2[[J, I]] = XF2[[(J + 3), I]];
                }
            }

            //
            // Extract the rotation block and compare to
            // that for our dynamic frame.
            //
            for I in 1..=3 {
                for J in 1..=3 {
                    R2[[J, I]] = XF2[[J, I]];
                }
            }

            //
            // Compare to the rotation from the Euler frame to
            // the base frame returned by PXFORM.
            //
            spicelib::PXFORM(&FRNAME, &BFRAME, ET, R.as_slice_mut(), ctx)?;

            TOL = DM14;

            testutil::CHCKAD(b"R2", R2.as_slice(), b"~", R.as_slice(), 9, TOL, OK, ctx)?;

            //
            // Check the derivative block of the state transformation
            // matrix.
            //
            spicelib::SXFORM(&FRNAME, &BFRAME, ET, XFORM.as_slice_mut(), ctx)?;

            for I in 1..=3 {
                for J in 1..=3 {
                    DRVBLK[[J, I]] = XFORM[[(J + 3), I]];
                }
            }

            //
            // Our locally derivative is relevant only if the frame's
            // rotation state is KVROTG.
            //
            if fstr::eq(&RSTSTR, KVROTG) {
                TOL = DM14;
                testutil::CHCKAD(
                    b"d(R2)/dt",
                    DRV2.as_slice(),
                    b"~",
                    DRVBLK.as_slice(),
                    9,
                    TOL,
                    OK,
                    ctx,
                )?;
            }

            //
            //--- Case: ------------------------------------------------------
            //
            //
            //           We've finished the generic tests.  If our frame is
            //           the pseudo IAU_MARS frame, compare the frame to that
            //           defined by our PCK.
            //
            if (ISMARS && fstr::eq(&RSTSTR, KVROTG)) {
                testutil::TCASE(b"IAU_MARS test case", ctx)?;

                //
                // Get the PCK-defined state transformation matrix.
                // Set the epoch to ~2030; this will allow velocity
                // errors to accumulate.
                //
                ET = (spicelib::JYEAR() * 30 as f64);
                spicelib::SXFORM(b"J2000", b"IAU_MARS", ET, TSIPM.as_slice_mut(), ctx)?;

                //
                // Extract the derivative block.
                //
                for I in 1..=3 {
                    for J in 1..=3 {
                        DRV2[[J, I]] = TSIPM[[(J + 3), I]];
                    }
                }

                //
                // Extract the rotation block and compare to
                // that for our dynamic frame.
                //
                for I in 1..=3 {
                    for J in 1..=3 {
                        TIPM[[J, I]] = TSIPM[[J, I]];
                    }
                }

                spicelib::PXFORM(b"J2000", &FRNAME, ET, R.as_slice_mut(), ctx)?;

                TOL = DM11;
                testutil::CHCKAD(
                    b"Mars TIPM",
                    R.as_slice(),
                    b"~",
                    TIPM.as_slice(),
                    9,
                    TOL,
                    OK,
                    ctx,
                )?;

                //
                // Check the derivative block of the state transformation
                // matrix.
                //
                spicelib::SXFORM(b"J2000", &FRNAME, ET, XFORM.as_slice_mut(), ctx)?;

                for I in 1..=3 {
                    for J in 1..=3 {
                        DRVBLK[[J, I]] = XFORM[[(J + 3), I]];
                    }
                }

                testutil::CHCKAD(
                    b"Mars d(TIPM)/dt",
                    DRVBLK.as_slice(),
                    b"~",
                    DRV2.as_slice(),
                    9,
                    TOL,
                    OK,
                    ctx,
                )?;
            }
        }
        //
        // Expunge the optional keywords from the kernel pool.
        //
        if fstr::eq(&RSTSTR, b"FROZEN") {
            fstr::assign(&mut KEYWRD, &fstr::concat(&FRSTEM, KWFREZ));
        } else {
            fstr::assign(&mut KEYWRD, &fstr::concat(&FRSTEM, KWRSTA));
        }

        spicelib::DVPOOL(&KEYWRD, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Clean up the SPK file.
    //
    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
