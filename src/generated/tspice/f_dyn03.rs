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
const DM14: f64 = 0.00000000000001;
const DM12: f64 = 0.000000000001;
const DM5: f64 = 0.00001;
const NAMLEN: i32 = 32;
const LNSIZE: i32 = 80;
const MAXDEF: i32 = 50;
const MAXTOK: i32 = 7;
const MSGLEN: i32 = 240;
const TIMLEN: i32 = 50;
const NDIMS: i32 = 3;
const NRSTAT: i32 = 3;
const RSTIDX: i32 = 1;
const NBASEF: i32 = 3;
const BFRIDX: i32 = (RSTIDX + 1);
const NCENTR: i32 = 1;
const CTRIDX: i32 = (BFRIDX + 1);

struct SaveVars {
    BASEFR: ActualCharArray,
    RSTATE: ActualCharArray,
    ZR: StackArray2D<f64, 9>,
    CTRCDE: StackArray<i32, 1>,
    DIMS: StackArray<i32, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BASEFR = ActualCharArray::new(NAMLEN, 1..=NBASEF);
        let mut RSTATE = ActualCharArray::new(LNSIZE, 1..=NRSTAT);
        let mut ZR = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut CTRCDE = StackArray::<i32, 1>::new(1..=NCENTR);
        let mut DIMS = StackArray::<i32, 3>::new(1..=NDIMS);

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

            let mut clist = [Val::C(b"J2000"), Val::C(b"IAU_MARS"), Val::C(b"GSE")].into_iter();
            BASEFR
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(399)].into_iter();
            CTRCDE
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(NRSTAT), Val::I(NBASEF), Val::I(NCENTR)].into_iter();
            DIMS.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BASEFR,
            RSTATE,
            ZR,
            CTRCDE,
            DIMS,
        }
    }
}

//$Procedure F_DYN03 ( Dynamic Frame Test Family 03 )
pub fn F_DYN03(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut BFRAME = [b' '; NAMLEN as usize];
    let mut CASMSG = [b' '; MSGLEN as usize];
    let mut DEFTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut FRNAME = [b' '; NAMLEN as usize];
    let mut FRZEPC = [b' '; TIMLEN as usize];
    let mut KEYWRD = [b' '; LNSIZE as usize];
    let mut RSTSTR = [b' '; LNSIZE as usize];
    let mut FRSTEM = [b' '; 18 as usize];
    let mut DELTA: f64 = 0.0;
    let mut DETERR: f64 = 0.0;
    let mut DMAG: f64 = 0.0;
    let mut DMOB: f64 = 0.0;
    let mut DRDIFF = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DRLERR: f64 = 0.0;
    let mut DRV2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DRVBLK = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DRVERR: f64 = 0.0;
    let mut DVNUT = StackArray::<f64, 4>::new(1..=4);
    let mut ET: f64 = 0.0;
    let mut MOB: f64 = 0.0;
    let mut NMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut NRMERR: f64 = 0.0;
    let mut NULON: f64 = 0.0;
    let mut NUOBL: f64 = 0.0;
    let mut PRECM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut R = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut R2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut RJ2B = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut RMINUS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut RPLUS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TODMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut T: f64 = 0.0;
    let mut T0: f64 = 0.0;
    let mut TOL: f64 = 0.0;
    let mut XFB2J = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFF2J = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut COORDS = StackArray::<i32, 3>::new(1..=NDIMS);
    let mut CENTER: i32 = 0;
    let mut DX: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut NCART: i32 = 0;
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
    //     1) Rotation state
    //     2) Base frame
    //     3) Frame center
    //
    // Number of dimensions of the test parameter space:
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
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Rotation states:
    //

    //
    // We use both inertial, PCK-based, and dynamic base frames.
    //

    //
    // ID codes of frame centers:
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
    testutil::TOPEN(b"F_DYN03", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create test inputs for comprehensive true-of-date test.",
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
        // Set the rotation state.
        //
        fstr::assign(&mut RSTSTR, save.RSTATE.get(COORDS[RSTIDX]));

        //
        // Set the base frame.
        //
        fstr::assign(&mut BFRAME, save.BASEFR.get(COORDS[BFRIDX]));

        //
        // Set the frame center.
        //
        CENTER = save.CTRCDE[COORDS[CTRIDX]];

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
            // Create a text buffer containing a frame definition that
            // uses the current frame definition
            //
            // First comes the frame name-to-frame code assignment.
            //
            fstr::assign(&mut FRNAME, b"TRUE_OF_DATE_DYN_FRAME");
            FRCODE = -2000000000;

            fstr::assign(DEFTXT.get_mut(1), b"FRAME_#    = #");
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
            // The frame center must be specified via a code.
            //
            fstr::assign(
                DEFTXT.get_mut(5),
                &fstr::concat(&FRSTEM, b"CENTER           = #"),
            );

            spicelib::REPMI(&DEFTXT[5].to_vec(), b"#", CENTER, &mut DEFTXT[5], ctx);
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
                        KVTEQT,
                    ),
                    b"\'",
                ),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::REPMI(&DEFTXT[8].to_vec(), b"#", FRCODE, &mut DEFTXT[8], ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Set the frame's precession model.
            //
            fstr::assign(
                DEFTXT.get_mut(9),
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(&fstr::concat(&FRSTEM, KWPRCM), b" = \'"),
                        KVM001,
                    ),
                    b"\'",
                ),
            );

            //
            // Set the frame's nutation model.
            //
            fstr::assign(
                DEFTXT.get_mut(10),
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(&fstr::concat(&FRSTEM, KWNUTM), b" = \'"),
                        KVM002,
                    ),
                    b"\'",
                ),
            );

            //
            // Set the frame's rotation state or freeze epoch.
            //
            //
            // Set the frame's rotation state or freeze epoch.
            //
            if fstr::eq(&RSTSTR, b"FROZEN") {
                fstr::assign(
                    DEFTXT.get_mut(11),
                    &fstr::concat(
                        &fstr::concat(&fstr::concat(&FRSTEM, KWFREZ), b"     = "),
                        FRZTOK,
                    ),
                );
            } else {
                fstr::assign(
                    DEFTXT.get_mut(11),
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(&fstr::concat(&FRSTEM, KWRSTA), b"     = \'"),
                            fstr::substr(&RSTSTR, 1..=spicelib::RTRIM(&RSTSTR)),
                        ),
                        b"\'",
                    ),
                );
            }

            DX = 11;

            //
            // Enter the frame definition into the kernel pool.
            //
            spicelib::LMPOOL(DEFTXT.as_arg(), DX, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // WRITE (*,*) '------------------------------------'
            //
            // DO I = 1, DX
            //    WRITE (*,*) DEFTXT(I)
            // END DO
            //
            // WRITE (*,*) '------------------------------------'
            //

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
            // Perform "sanity checks" on the returned matrix. Make sure
            // the diagonal blocks are identical rotations. Make sure the
            // upper right block is a zero matrix.
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
            // If the frame is considered to be rotating, compare the
            // discrete derivative to the lower left block.
            //
            if spicelib::EQSTR(&RSTSTR, KVROTG) {
                //
                // Check the derivative absolute error.
                //
                TOL = DM12;
                testutil::CHCKSD(b"DRVERR", DRVERR, b"~", 0.0, TOL, OK, ctx)?;

                //
                // Check the derivative relative error.
                //
                TOL = DM5;
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
            spicelib::REPMI(b"PXFORM test number #C. Construct position transformation locally; compare to PXFORM output.", b"#", CASE, &mut CASMSG, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::TCASE(&CASMSG, ctx)?;

            //
            // Pick a time value.  If the rotation state is
            // frozen, use the "freeze" value.
            //
            if spicelib::EQSTR(&RSTSTR, b"FROZEN") {
                T = T0;
            } else {
                T = ET;
            }

            //
            // Get precession matrix.
            //
            spicelib::ZZEPRCSS(T, PRECM.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Get mean obliquity.
            //
            spicelib::ZZMOBLIQ(T, &mut MOB, &mut DMOB, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Get nutation angles.  Form nutation matrix.
            //
            spicelib::ZZWAHR(T, DVNUT.as_slice_mut(), ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            NULON = DVNUT[1];
            NUOBL = DVNUT[2];

            spicelib::EUL2M(
                (-MOB - NUOBL),
                -NULON,
                MOB,
                1,
                3,
                1,
                NMAT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Form the J2000 to true-of-date matrix.
            //
            spicelib::MXM(NMAT.as_slice(), PRECM.as_slice(), TODMAT.as_slice_mut());

            //
            // Now form the transformation from the true-of-date
            // frame to the base frame.
            //
            spicelib::PXFORM(b"J2000", &BFRAME, T, RJ2B.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::MXMT(RJ2B.as_slice(), TODMAT.as_slice(), R2.as_slice_mut());

            //
            // Compare to the rotation from the true-of-date frame to
            // the base frame returned by PXFORM.
            //
            spicelib::PXFORM(&FRNAME, &BFRAME, ET, R.as_slice_mut(), ctx)?;

            TOL = DM14;
            testutil::CHCKAD(b"R2", R2.as_slice(), b"~", R.as_slice(), 9, TOL, OK, ctx)?;
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
