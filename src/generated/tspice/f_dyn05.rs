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
const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const IAU76: &[u8] = b"EARTH_IAU_1976";
const IAU80: &[u8] = b"EARTH_IAU_1980";
const TXTPCK: &[u8] = b"test_dyn.tpc";
const SPK: &[u8] = b"test_dyn.bsp";
const SPK2: &[u8] = b"tst_dyn2.bsp";
const CKNM: &[u8] = b"test_ck3.bc";
const CK2: &[u8] = b"tst_dyn2.bc";
const SCLK: &[u8] = b"test_ck3.tsc";
const EX13: f64 = 0.0000000000001;
const SIDLEN: i32 = 40;
const NAMLEN: i32 = 32;
const LNSIZE: i32 = 80;
const MAXDEF: i32 = 50;
const MAXTOK: i32 = 7;
const DSCSIZ: i32 = 5;
const ND: i32 = 2;
const NI: i32 = 6;

//$Procedure F_DYN05 ( Dynamic Frame Test Family 05 )
pub fn F_DYN05(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DEFTXT = ActualCharArray::new(LNSIZE, 1..=MAXDEF);
    let mut FRNAME = [b' '; NAMLEN as usize];
    let mut IDENT = [b' '; SIDLEN as usize];
    let mut AV0 = StackArray::<f64, 3>::new(1..=3);
    let mut AV1 = StackArray::<f64, 3>::new(1..=3);
    let mut AV2 = StackArray::<f64, 3>::new(1..=3);
    let mut CLKOUT: f64 = 0.0;
    let mut CMAT0 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut CMAT1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut CMAT2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DLT: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut ETCORR: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut POS0 = StackArray::<f64, 3>::new(1..=3);
    let mut POS1 = StackArray::<f64, 3>::new(1..=3);
    let mut POS2 = StackArray::<f64, 3>::new(1..=3);
    let mut R = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SCLKDP: f64 = 0.0;
    let mut STATE0 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE1 = StackArray::<f64, 6>::new(1..=6);
    let mut STATE2 = StackArray::<f64, 6>::new(1..=6);
    let mut STOBS = StackArray::<f64, 6>::new(1..=6);
    let mut SUM = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut XF2 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XF3 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XF4 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut CENTER: i32 = 0;
    let mut CLASS: i32 = 0;
    let mut CLKID: i32 = 0;
    let mut CLSSID: i32 = 0;
    let mut FRCODE: i32 = 0;
    let mut HAN: i32 = 0;
    let mut HANDLE = StackArray::<i32, 4>::new(1..=4);
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut INSTID: i32 = 0;
    let mut RC2CDE: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Note:  the name CKNM was picked because the more consistent
    // name "CK" is taken---it's the name of a frame class.
    //

    //
    // Tolerance levels for various tests.
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_DYN05", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Create test inputs for comprehensive mean-of-date test.",
        ctx,
    )?;

    //
    // Create and load kernels.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK, true, &mut HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_PCK08(TXTPCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTCK3(CKNM, SCLK, true, true, false, &mut HANDLE[2], ctx)?;

    //
    // Define a CK frame associated with instrument -10001; the pointing
    // for this instrument is given by the CK created by the test
    // utility TSTCK3.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_CK_-10001                  =  -10001",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_-10001_NAME                = \'CK_-10001\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_-10001_CLASS               =  3");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_-10001_CLASS_ID            =  -10001",
    );
    fstr::assign(DEFTXT.get_mut(5), b"FRAME_-10001_CENTER              =  -9");
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_-10001_RELATIVE            = \'J2000\'",
    );

    //
    // Load the CK_-10001 frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 6, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define a CK frame associated with instrument -9901; the pointing
    // for this instrument is given by the CK created by the test
    // utility TSTCK3.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_CK_-9901                   =  -9901",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_-9901_NAME                 = \'CK_-9901\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_-9901_CLASS                =  3");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_-9901_CLASS_ID             =  -9901",
    );
    fstr::assign(DEFTXT.get_mut(5), b"FRAME_-9901_CENTER               =  -9");
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_-9901_RELATIVE             = \'GALACTIC\'",
    );

    //
    // Load the CK_-9901 frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 6, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We'll need to several dynamic frames. Since the purpose of the
    // frames is to test the ability of the frame subsystem to handle
    // varying levels of recursive calls, we'll name the frames based on
    // the required recursion levels associated with transformations
    // between the frames and the J2000 frame.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_RECUR_1                    =  2399001",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2399001_NAME               = \'RECUR_1\'",
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
                KVNEAR,
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
    // Load the RECUR_1 frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 20, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define a two-vector frame using a constant vector and
    // requiring one level of recursion for evaluation.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_RECUR_1B                    =  2399002",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2399002_NAME               = \'RECUR_1B\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_2399002_CLASS              =  5");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_2399002_CLASS_ID           =  2399002",
    );
    fstr::assign(
        DEFTXT.get_mut(5),
        b"FRAME_2399002_CENTER             =  399",
    );
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_2399002_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        DEFTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSTYL), b"       = \'"),
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
                    &fstr::concat(b"FRAME_2399002_", KWFFAM),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVAXI),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVCDF),
                    b"       = \'",
                ),
                KVCONS,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVFRM),
            b"          = \'IAU_SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVOBS),
            b"       = \'EARTH\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVABC),
            b"         = \'LT\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWVSPC),
                    b"         = \'",
                ),
                KVLATC,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(15),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWUNIT),
                    b"         = \'",
                ),
                KVDEGR,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(16),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWLON),
            b"          =  60.0",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(17),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWPRI), KWLAT),
            b"          = -30.0",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(18),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWVAXI),
                    b"       = \'-",
                ),
                KVY,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(19),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWVCDF),
                    b"       =  \'",
                ),
                KVVELV,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(20),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWVOBS),
            b"       = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(21),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWVTRG),
            b"       = \'EARTH\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(22),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(23),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWSEC), KWVFRM),
            b"          = \'J2000\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(24),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399002_", KWRSTA), b"       =  \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    //
    // Load the RECUR_1B frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 24, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define the RECUR_2 frame.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_RECUR_2                    =  2401000",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2401000_NAME               = \'RECUR_2\'",
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
            b"          = \'RECUR_1\'",
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
    // Load the RECUR_2 frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 20, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define a two-vector frame using a constant vector and
    // requiring two levels of recursion for evaluation.
    //
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_RECUR_2B                    =  2399003",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2399003_NAME               = \'RECUR_2B\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_2399003_CLASS              =  5");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_2399003_CLASS_ID           =  2399003",
    );
    fstr::assign(
        DEFTXT.get_mut(5),
        b"FRAME_2399003_CENTER             =  399",
    );
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_2399003_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        DEFTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWSTYL), b"       = \'"),
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
                    &fstr::concat(b"FRAME_2399003_", KWFFAM),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWPRI), KWVAXI),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWPRI), KWVCDF),
                    b"       = \'",
                ),
                KVCONS,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWPRI), KWVFRM),
            b"          = \'RECUR_1B\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWPRI), KWVOBS),
            b"       = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWPRI), KWVABC),
            b"         = \'XS\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWPRI), KWVSPC),
                    b"         = \'",
                ),
                KVLATC,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(15),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWPRI), KWUNIT),
                    b"         = \'",
                ),
                KVDEGR,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(16),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWPRI), KWLON),
            b"          =  60.0",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(17),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWPRI), KWLAT),
            b"          = -30.0",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(18),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWSEC), KWVAXI),
                    b"       = \'-",
                ),
                KVY,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(19),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWSEC), KWVCDF),
                    b"       =  \'",
                ),
                KVVELV,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(20),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWSEC), KWVOBS),
            b"       = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(21),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWSEC), KWVTRG),
            b"       = \'EARTH\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(22),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWSEC), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(23),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWSEC), KWVFRM),
            b"          = \'J2000\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(24),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399003_", KWRSTA), b"       =  \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    //
    // Load the RECUR_2B frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 24, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define the RECUR_3 frame.  This frame is used to test
    // error detection when excessive recursion depth is required
    // to evaluate a dynamic frame.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_RECUR_3                    =  2499000",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2499000_NAME               = \'RECUR_3\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_2499000_CLASS              =  5");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_2499000_CLASS_ID           =  2499000",
    );
    fstr::assign(
        DEFTXT.get_mut(5),
        b"FRAME_2499000_CENTER             =  401",
    );
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_2499000_RELATIVE           = \'IAU_MARS\'",
    );
    fstr::assign(
        DEFTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWSTYL), b"       = \'"),
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
                    &fstr::concat(b"FRAME_2499000_", KWFFAM),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWPRI), KWVAXI),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWPRI), KWVCDF),
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
            &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWPRI), KWVOBS),
            b"       = \'MARS\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWPRI), KWVTRG),
            b"         = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWPRI), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWSEC), KWVAXI),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWSEC), KWVCDF),
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
            &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWSEC), KWVOBS),
            b"       = \'MARS\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(17),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWSEC), KWVTRG),
            b"       = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(18),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWSEC), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(19),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWSEC), KWVFRM),
            b"          = \'RECUR_2\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(20),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2499000_", KWRSTA), b"       =  \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    //
    // Load the RECUR_3 frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 20, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define the RECUR_3B frame.  This frame is used to test
    // error detection when excessive recursion depth is required
    // to evaluate a dynamic frame.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_RECUR_3B                   =  2499001",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2499001_NAME               = \'RECUR_3B\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_2499001_CLASS              =  5");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_2499001_CLASS_ID           =  2499001",
    );
    fstr::assign(
        DEFTXT.get_mut(5),
        b"FRAME_2499001_CENTER             =  401",
    );
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_2499001_RELATIVE           = \'IAU_MARS\'",
    );
    fstr::assign(
        DEFTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWSTYL), b"       = \'"),
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
                    &fstr::concat(b"FRAME_2499001_", KWFFAM),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWPRI), KWVAXI),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWPRI), KWVCDF),
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
            &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWPRI), KWVOBS),
            b"       = \'MARS\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWPRI), KWVTRG),
            b"         = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWPRI), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWSEC), KWVAXI),
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
                    &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWSEC), KWVCDF),
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
            &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWSEC), KWVOBS),
            b"       = \'MARS\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(17),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWSEC), KWVTRG),
            b"       = \'SUN\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(18),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWSEC), KWVABC),
            b"         = \'NONE\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(19),
        &fstr::concat(
            &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWSEC), KWVFRM),
            b"          = \'RECUR_2B\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(20),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2499001_", KWRSTA), b"       =  \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    //
    // Load the RECUR_3B frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 20, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define the EARTH_MEAN_OF_DATE frame.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_EARTH_MEAN_OF_DATE         =  2399004",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2399004_NAME               = \'EARTH_MEAN_OF_DATE\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_2399004_CLASS              =  5");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_2399004_CLASS_ID           =  2399004",
    );
    fstr::assign(
        DEFTXT.get_mut(5),
        b"FRAME_2399004_CENTER             =  399",
    );
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_2399004_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        DEFTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399004_", KWSTYL), b"   = \'"),
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
                    &fstr::concat(b"FRAME_2399004_", KWFFAM),
                    b"             = \'",
                ),
                KVMEQT,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399004_", KWPRCM), b"   = \'"),
                KVM001,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399004_", KWRSTA), b"    = \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    //
    // Load the EARTH_MEAN_OF_DATE frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 10, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define the EARTH_TRUE_OF_DATE frame.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_EARTH_TRUE_OF_DATE         =  2399005",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2399005_NAME               = \'EARTH_TRUE_OF_DATE\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_2399005_CLASS              =  5");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_2399005_CLASS_ID           =  2399005",
    );
    fstr::assign(
        DEFTXT.get_mut(5),
        b"FRAME_2399005_CENTER             =  399",
    );
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_2399005_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        DEFTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399005_", KWSTYL), b"   = \'"),
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
                    &fstr::concat(b"FRAME_2399005_", KWFFAM),
                    b"             = \'",
                ),
                KVTEQT,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399005_", KWPRCM), b"   = \'"),
                KVM001,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399005_", KWNUTM), b"   = \'"),
                KVM002,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2399005_", KWRSTA), b"    = \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    //
    // Load the EARTH_TRUE_OF_DATE frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 11, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Define the MARS_EULER frame.
    //
    fstr::assign(
        DEFTXT.get_mut(1),
        b"FRAME_MARS_EULER                 =  2499002",
    );
    fstr::assign(
        DEFTXT.get_mut(2),
        b"FRAME_2499002_NAME               = \'MARS_EULER\'",
    );
    fstr::assign(DEFTXT.get_mut(3), b"FRAME_2499002_CLASS              =  5");
    fstr::assign(
        DEFTXT.get_mut(4),
        b"FRAME_2499002_CLASS_ID           =  2499002",
    );
    fstr::assign(
        DEFTXT.get_mut(5),
        b"FRAME_2499002_CENTER             =  499",
    );
    fstr::assign(
        DEFTXT.get_mut(6),
        b"FRAME_2499002_RELATIVE           = \'J2000\'",
    );
    fstr::assign(
        DEFTXT.get_mut(7),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2499002_", KWSTYL), b"   = \'"),
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
                    &fstr::concat(b"FRAME_2499002_", KWFFAM),
                    b"             = \'",
                ),
                KVEULR,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(9),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(b"FRAME_2499002_", KWEPOC),
                b"              =  ",
            ),
            b"@2000-NOV-23/04:25:00",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(10),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(b"FRAME_2499002_", KWUNIT),
                    b"              =  \'",
                ),
                KVDEGR,
            ),
            b"\'",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(11),
        &fstr::concat(
            &fstr::concat(b"FRAME_2499002_", KWEUAX),
            b"               = ( 1  3  2 )",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(12),
        &fstr::concat(
            &fstr::concat(b"FRAME_2499002_", KWEAC1),
            b"     = (  1      2.D-10 )",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(13),
        &fstr::concat(
            &fstr::concat(b"FRAME_2499002_", KWEAC2),
            b"     = (  3      4.D-10 )",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(14),
        &fstr::concat(
            &fstr::concat(b"FRAME_2499002_", KWEAC3),
            b"     = (  5      6.D-10 )",
        ),
    );
    fstr::assign(
        DEFTXT.get_mut(15),
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(&fstr::concat(b"FRAME_2499002_", KWRSTA), b"    = \'"),
                KVROTG,
            ),
            b"\'",
        ),
    );

    //
    // Load the MARS_EULER frame definition.
    //
    spicelib::LMPOOL(DEFTXT.as_arg(), 15, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //     Start out by checking the FRAMEX interfaces.  These
    //     tests are of a trivial nature; their purpose is to verify
    //     that class 5 (dynamic) frames are handled properly.
    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Map name RECUR_2 to an ID code, and back.", ctx)?;

    RC2CDE = 2401000;
    spicelib::NAMFRM(b"RECUR_2", &mut FRCODE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", RC2CDE, 0, OK, ctx)?;

    spicelib::FRMNAM(RC2CDE, &mut FRNAME, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"RECUR_2", OK, ctx)?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look up frame info for RECUR_2", ctx)?;

    spicelib::FRINFO(
        RC2CDE,
        &mut CENTER,
        &mut CLASS,
        &mut CLSSID,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // The frame class is "dynamic."  The parameter DYN is defined
    // in frmtyp.inc.
    //
    testutil::CHCKSI(b"CLASS", CLASS, b"=", DYN, 0, OK, ctx)?;

    //
    // The class ID is just the frame ID in this case.
    //
    testutil::CHCKSI(b"CLASS ID", CLSSID, b"=", RC2CDE, 0, OK, ctx)?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Get the center of frame RECUR_2", ctx)?;

    //
    // This case is quite obscure:  we're making RECUR_2 the
    // default frame associated with Phobos, and we want to
    // make sure the FRAMEX API handles this.  Note that changing
    // the default frame associated with a planet or satellite
    // is a bad idea, unless the new frame is an improved body-fixed
    // frame for the object in question.
    //
    spicelib::PCPOOL(b"OBJECT_401_FRAME", 1, CharArray::from_ref(b"RECUR_2"), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the center using CIDFRM first, then CNMFRM.
    //
    spicelib::CIDFRM(401, &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", RC2CDE, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"RECUR_2", OK, ctx)?;

    spicelib::CNMFRM(b"PHOBOS", &mut FRCODE, &mut FRNAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"FRCODE", FRCODE, b"=", RC2CDE, 0, OK, ctx)?;
    testutil::CHCKSC(b"FRNAME", &FRNAME, b"=", b"RECUR_2", OK, ctx)?;

    //
    // Delete the OBJECT_401_FRAME kernel variable.
    //
    spicelib::DVPOOL(b"OBJECT_401_FRAME", ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //     Now we want to perform an SXFORM test:  transform between
    //     two frames which both require two levels of recursion
    //     for evaluation.
    //
    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"SXFORM test:  create and test transformation between RECUR_2 and RECUR_2B.",
        ctx,
    )?;

    spicelib::STR2ET(b"2005 JAN 1", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the transformation matrix.
    //
    spicelib::SXFORM(b"RECUR_2", b"RECUR_2B", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create transformations from RECUR_2 to J2000 and from
    // J2000 to RECUR_2B.  Compose and compare to XFORM.
    //
    spicelib::SXFORM(b"RECUR_2", b"J2000", ET, XF2.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"RECUR_2B", ET, XF3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXMG(XF3.as_slice(), XF2.as_slice(), 6, 6, 6, XF4.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare results.
    //
    testutil::CHCKAD(
        b"XFORM",
        XFORM.as_slice(),
        b"~/",
        XF4.as_slice(),
        36,
        EX13,
        OK,
        ctx,
    )?;

    //
    //     Now test the SPK interfaces.
    //
    //     We'll test the state and corresponding position-only routines
    //     together, to the extent possible.
    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check SPKEZR/SPKPOS using the frame RECUR_2.", ctx)?;
    //
    // For each SPK API routine, we'll verify that the SPK routine
    // can do the same transformation that we can do explicitly
    // by looking up the state relative to the J2000 frame and
    // then using SXFORM or PXFORM to convert the state to the
    // dynamic frame.
    //
    spicelib::STR2ET(b"2005 JAN 1", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the light time from the center of the frame RECUR_2
    // to the observer.  Determine the "aberration corrected epoch"
    // ETCORR.  The inertial to dynamic frame transformation XFORM will
    // be computed at this epoch.
    //
    spicelib::SPKSSB(499, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        401,
        ET,
        b"J2000",
        b"LT+S",
        STOBS.as_slice(),
        STATE0.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the transformation from J2000 to the dynamic frame at
    // the aberration corrected epoch.
    //
    spicelib::ZZCOREPC(b"LT+S", ET, LT, &mut ETCORR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"RECUR_2", ETCORR, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(b"J2000", b"RECUR_2", ETCORR, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the dynamic frame.
    //
    spicelib::SPKEZR(
        b"EARTH",
        ET,
        b"RECUR_2",
        b"LT+S",
        b"MARS",
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"EARTH",
        ET,
        b"RECUR_2",
        b"LT+S",
        b"MARS",
        POS0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the inertial frame, then
    // transform to the dynamic frame.
    //
    spicelib::SPKEZR(
        b"EARTH",
        ET,
        b"J2000",
        b"LT+S",
        b"MARS",
        STATE1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"EARTH",
        ET,
        b"J2000",
        b"LT+S",
        b"MARS",
        POS1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Adjust the derivative of the state transformation to reflect
    // the rate of change of light time to the frame's center.
    //
    for ROW in 4..=6 {
        for COL in 1..=3 {
            XFORM[[ROW, COL]] = (XFORM[[ROW, COL]] * (1.0 - DLT));
        }
    }

    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );
    spicelib::MXV(R.as_slice(), POS1.as_slice(), POS2.as_slice_mut());

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Check position from SPKPOS.
    //
    testutil::CHCKAD(
        b"POS0",
        POS0.as_slice(),
        b"~/",
        POS2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check SPKEZR/SPKPOS using the frame RECUR_2B.", ctx)?;
    //
    // This test is a repeat of the previous one, but we use a
    // frame whose constant vector is defined relative to a
    // dynamic frame.
    //
    spicelib::STR2ET(b"2005 JAN 1", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the light time from the center of the frame RECUR_2
    // to the observer.  Determine the "aberration corrected epoch"
    // ETCORR.  The inertial to dynamic frame transformation XFORM will
    // be computed at this epoch.
    //
    // RECUR_2B is centered at the earth.
    //
    //
    spicelib::SPKSSB(499, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        399,
        ET,
        b"J2000",
        b"XLT+S",
        STOBS.as_slice(),
        STATE0.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the transformation from J2000 to the dynamic frame at
    // the aberration corrected epoch.
    //
    spicelib::ZZCOREPC(b"XLT+S", ET, LT, &mut ETCORR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"RECUR_2B", ETCORR, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(b"J2000", b"RECUR_2B", ETCORR, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the dynamic frame.
    //
    spicelib::SPKEZR(
        b"EARTH",
        ET,
        b"RECUR_2B",
        b"XLT+S",
        b"MARS",
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"EARTH",
        ET,
        b"RECUR_2B",
        b"XLT+S",
        b"MARS",
        POS0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the inertial frame, then
    // transform to the dynamic frame.
    //
    spicelib::SPKEZR(
        b"EARTH",
        ET,
        b"J2000",
        b"XLT+S",
        b"MARS",
        STATE1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"EARTH",
        ET,
        b"J2000",
        b"XLT+S",
        b"MARS",
        POS1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Adjust the derivative of the state transformation to reflect
    // the rate of change of light time to the frame's center.
    //
    for ROW in 4..=6 {
        for COL in 1..=3 {
            XFORM[[ROW, COL]] = (XFORM[[ROW, COL]] * (1.0 + DLT));
        }
    }

    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );
    spicelib::MXV(R.as_slice(), POS1.as_slice(), POS2.as_slice_mut());

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Check position from SPKPOS.
    //
    testutil::CHCKAD(
        b"POS0",
        POS0.as_slice(),
        b"~/",
        POS2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //     Exercise the code for evaluting frames of the remaining
    //     families.
    //
    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check SPKEZR/SPKPOS using the frame EARTH_MEAN_OF_DATE.",
        ctx,
    )?;

    //
    // Compute the light time from the center of the frame
    // EARTH_MEAN_OF_DATE (namely the earth) to the observer.  Determine
    // the "aberration corrected epoch" ETCORR.  The inertial to dynamic
    // frame transformation XFORM will be computed at this epoch.
    //
    spicelib::SPKSSB(499, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        399,
        ET,
        b"J2000",
        b"LT",
        STOBS.as_slice(),
        STATE0.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the transformation from J2000 to the dynamic frame at
    // the aberration corrected epoch.
    //
    spicelib::ZZCOREPC(b"LT", ET, LT, &mut ETCORR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"EARTH_MEAN_OF_DATE",
        ETCORR,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        b"J2000",
        b"EARTH_MEAN_OF_DATE",
        ETCORR,
        R.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the dynamic frame.
    //
    spicelib::SPKEZR(
        b"EARTH",
        ET,
        b"EARTH_MEAN_OF_DATE",
        b"LT",
        b"MARS",
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"EARTH",
        ET,
        b"EARTH_MEAN_OF_DATE",
        b"LT",
        b"MARS",
        POS0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the inertial frame, then
    // transform to the dynamic frame.
    //
    spicelib::SPKEZR(
        b"EARTH",
        ET,
        b"J2000",
        b"LT",
        b"MARS",
        STATE1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"EARTH",
        ET,
        b"J2000",
        b"LT",
        b"MARS",
        POS1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Adjust the derivative of the state transformation to reflect
    // the rate of change of light time to the frame's center.
    //
    for ROW in 4..=6 {
        for COL in 1..=3 {
            XFORM[[ROW, COL]] = (XFORM[[ROW, COL]] * (1.0 - DLT));
        }
    }

    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );
    spicelib::MXV(R.as_slice(), POS1.as_slice(), POS2.as_slice_mut());

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Check position from SPKPOS.
    //
    testutil::CHCKAD(
        b"POS0",
        POS0.as_slice(),
        b"~/",
        POS2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Check SPKEZR/SPKPOS using the frame EARTH_TRUE_OF_DATE.",
        ctx,
    )?;

    //
    // Compute the light time from the center of the frame
    // EARTH_TRUE_OF_DATE (namely the earth) to the observer.  Determine
    // the "aberration corrected epoch" ETCORR.  The inertial to dynamic
    // frame transformation XFORM will be computed at this epoch.
    //
    spicelib::SPKSSB(499, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        399,
        ET,
        b"J2000",
        b"LT+S",
        STOBS.as_slice(),
        STATE0.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the transformation from J2000 to the dynamic frame at
    // the aberration corrected epoch.
    //
    spicelib::ZZCOREPC(b"LT", ET, LT, &mut ETCORR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(
        b"J2000",
        b"EARTH_TRUE_OF_DATE",
        ETCORR,
        XFORM.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(
        b"J2000",
        b"EARTH_TRUE_OF_DATE",
        ETCORR,
        R.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the dynamic frame.
    //
    spicelib::SPKEZR(
        b"EARTH",
        ET,
        b"EARTH_TRUE_OF_DATE",
        b"LT",
        b"MARS",
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"EARTH",
        ET,
        b"EARTH_TRUE_OF_DATE",
        b"LT",
        b"MARS",
        POS0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the inertial frame, then
    // transform to the dynamic frame.
    //
    spicelib::SPKEZR(
        b"EARTH",
        ET,
        b"J2000",
        b"LT",
        b"MARS",
        STATE1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"EARTH",
        ET,
        b"J2000",
        b"LT",
        b"MARS",
        POS1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Adjust the derivative of the state transformation to reflect
    // the rate of change of light time to the frame's center.
    //
    for ROW in 4..=6 {
        for COL in 1..=3 {
            XFORM[[ROW, COL]] = (XFORM[[ROW, COL]] * (1.0 - DLT));
        }
    }

    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );
    spicelib::MXV(R.as_slice(), POS1.as_slice(), POS2.as_slice_mut());

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Check position from SPKPOS.
    //
    testutil::CHCKAD(
        b"POS0",
        POS0.as_slice(),
        b"~/",
        POS2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check SPKEZR/SPKPOS using the frame MARS_EULER.", ctx)?;

    //
    // Compute the light time from the center of the frame MARS_EULER to
    // the observer.  Determine the "aberration corrected epoch" ETCORR.
    // The inertial to dynamic frame transformation XFORM will be
    // computed at this epoch.
    //
    spicelib::SPKSSB(399, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        499,
        ET,
        b"J2000",
        b"LT+S",
        STOBS.as_slice(),
        STATE0.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the transformation from J2000 to the dynamic frame at
    // the aberration corrected epoch.
    //
    spicelib::ZZCOREPC(b"CN+S", ET, LT, &mut ETCORR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"MARS_EULER", ETCORR, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(b"J2000", b"MARS_EULER", ETCORR, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the inertial frame, then
    // transform to the dynamic frame.
    //
    spicelib::SPKEZR(
        b"MARS",
        ET,
        b"MARS_EULER",
        b"CN+S",
        b"EARTH",
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"MARS",
        ET,
        b"MARS_EULER",
        b"CN+S",
        b"EARTH",
        POS0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state in the inertial frame, then transform to
    // the dynamic frame.
    //
    spicelib::SPKEZR(
        b"MARS",
        ET,
        b"J2000",
        b"CN+S",
        b"EARTH",
        STATE1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"MARS",
        ET,
        b"J2000",
        b"CN+S",
        b"EARTH",
        POS1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Adjust the derivative of the state transformation to reflect
    // the rate of change of light time to the frame's center.
    //
    for ROW in 4..=6 {
        for COL in 1..=3 {
            XFORM[[ROW, COL]] = (XFORM[[ROW, COL]] * (1.0 - DLT));
        }
    }

    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );
    spicelib::MXV(R.as_slice(), POS1.as_slice(), POS2.as_slice_mut());

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Check position from SPKPOS.
    //
    testutil::CHCKAD(
        b"POS0",
        POS0.as_slice(),
        b"~/",
        POS2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //     Suppose we do a lookup requiring three levels of recursion?
    //
    //--- Case: ------------------------------------------------------
    //
    //
    //     SPKEZR:  catch error when evaluating two-vector frame
    //     defined using velocity vector.
    //
    testutil::TCASE(b"Check SPKEZR using the frame RECUR_3. In this test, the error should be caught in routine ZZSPKEZ1.", ctx)?;

    spicelib::STR2ET(b"2005 JAN 1", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZR(
        b"PHOBOS",
        ET,
        b"RECUR_3",
        b"LT+S",
        b"MARS",
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(RECURSIONTOODEEP)", OK, ctx)?;

    //
    //--- Case: ------------------------------------------------------
    //
    //
    //     SPKPOS:  catch error when evaluating two-vector frame defined
    //     using velocity vector. Because the error will occur when
    //     obtaining a state vector, the error will be trapped in ZZSPKEZ1
    //     rather than the position-only analog ZZSPKZP1.
    //
    testutil::TCASE(b"Check SPKEZR using the frame RECUR_3. In this test, the error should be caught in routine ZZSPKEZ1.", ctx)?;

    spicelib::STR2ET(b"2005 JAN 1", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"PHOBOS",
        ET,
        b"RECUR_3",
        b"LT+S",
        b"MARS",
        POS0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(RECURSIONTOODEEP)", OK, ctx)?;

    //
    // Make sure ZZSPKZP1 can trap the recursion error.  Note:
    // this case should be excluded in Icy testing.
    //
    spicelib::ZZSPKZP1(
        401,
        ET,
        b"RECUR_1",
        b"LT+S",
        499,
        POS0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(RECURSIONTOODEEP)", OK, ctx)?;

    //
    //--- Case: ------------------------------------------------------
    //
    //
    //     SPKEZR:  catch error when evaluating two-vector frame
    //     defined using constant vector.
    //
    testutil::TCASE(b"Check SPKEZR using the frame RECUR_3B. In this test, the error should be caught in routine ZZFRMCH1.", ctx)?;

    spicelib::STR2ET(b"2005 JAN 1", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZR(
        b"PHOBOS",
        ET,
        b"RECUR_3B",
        b"LT+S",
        b"MARS",
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(RECURSIONTOODEEP)", OK, ctx)?;

    //
    //--- Case: ------------------------------------------------------
    //
    //     SPKPOS:  catch error when evaluating two-vector frame
    //     defined using constant vector.
    //
    testutil::TCASE(b"Check SPKPOS using the frame RECUR_3B. In this test, the error should be caught in routine ZZFRMCH1.", ctx)?;

    spicelib::STR2ET(b"2005 JAN 1", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKPOS(
        b"PHOBOS",
        ET,
        b"RECUR_3B",
        b"LT+S",
        b"MARS",
        POS0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(RECURSIONTOODEEP)", OK, ctx)?;

    //
    //     Now test the rest of the SPK API routines.  We'll use just
    //     one dynamic frame for each case, since we've already exercised
    //     the low-level code needed to evaluate each family of dynamic
    //     frames.  (Note, however, that we haven't covered every
    //     possible logic branch in the low-level code---the other
    //     dynamic frame test families are responsible for that.)
    //
    //
    //--- Case: ------------------------------------------------------
    //

    testutil::TCASE(b"Check SPKEZ/SPKEZP using the frame RECUR_1.", ctx)?;
    //
    // Compute the light time from the center of the frame RECUR_2
    // to the observer.  Determine the "aberration corrected epoch"
    // ETCORR.  The inertial to dynamic frame transformation XFORM will
    // be computed at this epoch.
    //
    spicelib::SPKSSB(499, ET, b"J2000", STOBS.as_slice_mut(), ctx)?;
    spicelib::SPKLTC(
        399,
        ET,
        b"J2000",
        b"XLT+S",
        STOBS.as_slice(),
        STATE0.as_slice_mut(),
        &mut LT,
        &mut DLT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compute the transformation from J2000 to the dynamic frame at
    // the aberration corrected epoch.
    //
    spicelib::ZZCOREPC(b"XLT+S", ET, LT, &mut ETCORR, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"RECUR_1", ETCORR, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(b"J2000", b"RECUR_1", ETCORR, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the dynamic frame.
    //
    spicelib::SPKEZ(
        399,
        ET,
        b"RECUR_1",
        b"XLT+S",
        499,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZP(
        399,
        ET,
        b"RECUR_1",
        b"XLT+S",
        499,
        POS0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the inertial frame, then
    // transform to the dynamic frame.
    //
    spicelib::SPKEZ(
        399,
        ET,
        b"J2000",
        b"XLT+S",
        499,
        STATE1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKEZP(
        399,
        ET,
        b"J2000",
        b"XLT+S",
        499,
        POS1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Adjust the derivative of the state transformation to reflect
    // the rate of change of light time to the frame's center.
    //
    for ROW in 4..=6 {
        for COL in 1..=3 {
            XFORM[[ROW, COL]] = (XFORM[[ROW, COL]] * (1.0 + DLT));
        }
    }

    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );
    spicelib::MXV(R.as_slice(), POS1.as_slice(), POS2.as_slice_mut());

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Check position from SPKEZP.
    //
    testutil::CHCKAD(
        b"POS0",
        POS0.as_slice(),
        b"~/",
        POS2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check SPKGEO/SPKGPS using the frame RECUR_1.", ctx)?;

    spicelib::SXFORM(b"J2000", b"RECUR_1", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(b"J2000", b"RECUR_1", ET, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the dynamic frame.
    //
    spicelib::SPKGEO(
        399,
        ET,
        b"RECUR_1",
        499,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKGPS(399, ET, b"RECUR_1", 499, POS0.as_slice_mut(), &mut LT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state and position in the inertial frame, then
    // transform to the dynamic frame.
    //
    spicelib::SPKGEO(399, ET, b"J2000", 499, STATE1.as_slice_mut(), &mut LT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKGPS(399, ET, b"J2000", 499, POS1.as_slice_mut(), &mut LT, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );
    spicelib::MXV(R.as_slice(), POS1.as_slice(), POS2.as_slice_mut());

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Check position from SPKGPS. Measure error in relative terms. We
    // have a slightly larger relative error here (presumably due to
    // implementation differences between the two computation paths), so
    // use a slightly larger tolerance.
    //
    testutil::CHCKAD(
        b"POS0",
        POS0.as_slice(),
        b"~/",
        POS2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //     Check SPKSSB.  Note:  there's no position-only analog of SPKSSB.
    //
    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check SPKSSB using the frame RECUR_2.", ctx)?;

    //
    // SPKSSB computes only geometric states, so the dynamic frame
    // is evaluated at ET.
    //
    spicelib::SXFORM(b"J2000", b"RECUR_2", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state in the dynamic frame.
    //
    spicelib::SPKSSB(399, ET, b"RECUR_2", STATE0.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state in the inertial frame, then transform to
    // the dynamic frame.
    //
    spicelib::SPKSSB(399, ET, b"J2000", STATE1.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check SPKAPP/SPKAPO using the frame RECUR_2.", ctx)?;

    //
    // SPKAPP supports only inertial frames.  However, we want to make
    // sure this routine rejects dynamic frames if they're provided as
    // inputs.
    //
    spicelib::SPKAPP(
        499,
        ET,
        b"RECUR_2",
        STATE0.as_slice(),
        b"LT+S",
        STATE1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    // Ditto for SPKAPO.
    //
    spicelib::SPKAPO(
        499,
        ET,
        b"RECUR_2",
        STATE0.as_slice(),
        b"LT+S",
        POS1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADFRAME)", OK, ctx)?;

    //
    //     Check SPKPV.  Note:  this test need not be done for Icy.
    //
    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Check SPKPV using the frame RECUR_2.", ctx)?;

    //
    // SPKPV computes only geometric states, so the dynamic frame
    // is evaluated at ET.
    //
    spicelib::SXFORM(b"J2000", b"RECUR_2", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state in the dynamic frame.
    //
    spicelib::SPKSFS(
        399,
        ET,
        &mut HAN,
        SUM.as_slice_mut(),
        &mut IDENT,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::SPKPV(
        HAN,
        SUM.as_slice(),
        ET,
        b"RECUR_2",
        STATE0.as_slice_mut(),
        &mut CENTER,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state in the inertial frame, then transform to
    // the dynamic frame.
    //
    spicelib::SPKPV(
        HAN,
        SUM.as_slice(),
        ET,
        b"J2000",
        STATE1.as_slice_mut(),
        &mut CENTER,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //     Check the CK interfaces.
    //
    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Look up attitude of object -10001 relative to frame RECUR_2 using CKGP.",
        ctx,
    )?;

    INSTID = -10001;
    CLKID = -9;

    spicelib::STR2ET(b"2005 JAN 1 12:00 TDB", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCE2C(CLKID, ET, &mut SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up pointing relative to frame RECUR_2.
    //
    spicelib::CKGP(
        INSTID,
        SCLKDP,
        0.0,
        b"RECUR_2",
        CMAT0.as_slice_mut(),
        &mut CLKOUT,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // Look up pointing relative to frame J2000; convert to
    // pointing relative to RECUR_2.
    //
    spicelib::CKGP(
        INSTID,
        SCLKDP,
        0.0,
        b"J2000",
        CMAT1.as_slice_mut(),
        &mut CLKOUT,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    spicelib::PXFORM(b"J2000", b"RECUR_2", ET, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXMT(CMAT1.as_slice(), R.as_slice(), CMAT2.as_slice_mut());

    //
    // Compare C-matrices.
    //
    testutil::CHCKAD(
        b"CMAT0",
        CMAT0.as_slice(),
        b"~",
        CMAT2.as_slice(),
        9,
        EX13,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Look up attitude of object -10001 relative to frame RECUR_2 using CKGPAV.",
        ctx,
    )?;

    //
    // Look up pointing relative to frame RECUR_2.
    //
    spicelib::CKGPAV(
        INSTID,
        SCLKDP,
        0.0,
        b"RECUR_2",
        CMAT0.as_slice_mut(),
        AV0.as_slice_mut(),
        &mut CLKOUT,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up pointing relative to frame J2000; convert to
    // pointing relative to RECUR_2.
    //
    spicelib::CKGPAV(
        INSTID,
        SCLKDP,
        0.0,
        b"J2000",
        CMAT1.as_slice_mut(),
        AV1.as_slice_mut(),
        &mut CLKOUT,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::PXFORM(b"J2000", b"RECUR_2", ET, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXMT(CMAT1.as_slice(), R.as_slice(), CMAT2.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Compare C-matrices.
    //
    testutil::CHCKAD(
        b"CMAT0",
        CMAT0.as_slice(),
        b"~",
        CMAT2.as_slice(),
        9,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Compare angular velocity vectors.  To do this, we first
    // must find the the angular velocity of frame CK_-10001 relative
    // to RECUR_2, given the angular velocity of frame CK_-10001 relative
    // to J2000.  We start by looking up the state transformation
    // from RECUR_2 to J2000.
    //
    spicelib::SXFORM(b"RECUR_2", b"J2000", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the state transformation from J2000 to CK_-10001.
    //
    spicelib::RAV2XF(CMAT1.as_slice(), AV1.as_slice(), XF2.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Get the state transformation from RECUR_2 to CK_-10001.
    // Extract the angular velocity from this transformation.
    //
    spicelib::MXMG(
        XF2.as_slice(),
        XFORM.as_slice(),
        6,
        6,
        6,
        XF3.as_slice_mut(),
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::XF2RAV(XF3.as_slice(), CMAT2.as_slice_mut(), AV2.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check AV0.
    //
    testutil::CHCKAD(
        b"AV0",
        AV0.as_slice(),
        b"~",
        AV2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //     Check the PCK interfaces.
    //
    //
    //--- Case: ------------------------------------------------------
    //
    //
    //     TISBOD is supposed to support only inertial base frames.
    //     Make sure that a dynamic base frame is rejected.
    //
    testutil::TCASE(
        b"Look up attitude of Mars relative to frame RECUR_1 using TISBOD.",
        ctx,
    )?;

    spicelib::TISBOD(b"RECUR_1", 499, ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(IRFNOTREC)", OK, ctx)?;

    //
    // Same deal for TIPBOD.
    //
    testutil::TCASE(
        b"Look up attitude of Mars relative to frame RECUR_1 using TIPBOD.",
        ctx,
    )?;

    spicelib::TIPBOD(b"RECUR_1", 499, ET, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(IRFNOTREC)", OK, ctx)?;

    //
    //     Now we're going to perform tests using SPK and CK files that use
    //     dynamic frames as base frames.  First we'll create these kernels.
    //     We'll use the existing test utilities to do most of our work,
    //     then we'll change some segment descriptors to make selected
    //     segments refer to new bodies and dynamic frames.
    //
    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create an SPK for tests using dynamic base frames.", ctx)?;

    //
    // Start with the SPK file.  Create but don't load the file.
    //
    testutil::TSTSPK(SPK2, false, &mut HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Locate the segment for the Jupiter barycenter; change the
    // descriptor to make the base frame RECUR_2.  Make the base frame
    // for the Saturn barycenter RECUR_2B.  Change the ID codes of the
    // objects at the same time, so we can compare states from the new
    // segments with those from the old.
    //
    // Change the frame for the Uranus barycenter to RECUR_3.  This
    // segment should become unusable; we'll check this later.
    //
    spicelib::DAFOPW(SPK2, &mut HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    while FOUND {
        spicelib::DAFGS(SUM.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DAFUS(SUM.as_slice(), ND, NI, DC.as_slice_mut(), IC.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (IC[1] == 5) {
            //
            // Update the descriptor.
            //
            spicelib::NAMFRM(b"RECUR_2", &mut IC[3], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            IC[1] = (IC[1] * 100);

            spicelib::DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), SUM.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Write the new descriptor.
            //
            spicelib::DAFRS(SUM.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        } else if (IC[1] == 6) {
            //
            // Update the descriptor.
            //
            spicelib::NAMFRM(b"RECUR_2B", &mut IC[3], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            IC[1] = (IC[1] * 100);

            spicelib::DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), SUM.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Write the new descriptor.
            //
            spicelib::DAFRS(SUM.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        } else if (IC[1] == 7) {
            //
            // Update the descriptor.
            //
            spicelib::NAMFRM(b"RECUR_3", &mut IC[3], ctx)?;
            IC[1] = (IC[1] * 100);

            spicelib::DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), SUM.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Write the new descriptor.
            //
            spicelib::DAFRS(SUM.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        spicelib::DAFFNA(&mut FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close the modified SPK file.
    //
    spicelib::DAFCLS(HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look up state of body 500 relative to the sun, in frame RECUR_2. Compare to state of Jupiter barycenter in frame DE-96", ctx)?;
    //
    // Load our modified SPK file.
    //
    spicelib::SPKLEF(SPK2, &mut HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2005 JAN 1", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the state of body 500 relative to the solar system
    // barycenter in the dynamic frame RECUR_2. The state should match
    // that of the Jupiter barycenter relative to the solar system
    // barycenter in frame DE-96.
    //
    spicelib::SPKEZ(
        500,
        ET,
        b"RECUR_2",
        b"NONE",
        10,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The frame DE-96 is what's used for this segment by TSTSPK.
    //
    spicelib::SPKEZ(
        5,
        ET,
        b"DE-96",
        b"NONE",
        10,
        STATE1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE1.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE1.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Look up state of body 500 relative to  body 600 in frame RECUR_1. ",
        ctx,
    )?;

    //
    // Look up the state of body 600 relative to the solar system
    // barycenter in the dynamic frame RECUR_1. The state should match
    // that of the Jupiter barycenter relative to the solar system
    // barycenter in frame DE-102.
    //
    spicelib::SPKEZ(
        600,
        ET,
        b"RECUR_1",
        b"NONE",
        500,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the same state using the J2000 frame.  Map to
    // RECUR_1 and compare.
    //
    spicelib::SPKEZ(
        600,
        ET,
        b"J2000",
        b"NONE",
        500,
        STATE1.as_slice_mut(),
        &mut LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SXFORM(b"J2000", b"RECUR_1", ET, XFORM.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXVG(
        XFORM.as_slice(),
        STATE1.as_slice(),
        6,
        6,
        STATE2.as_slice_mut(),
    );

    //
    // Check position and velocity separately.  Measure error in
    // relative terms.
    //
    testutil::CHCKAD(
        b"STATE0 position",
        STATE0.as_slice(),
        b"~/",
        STATE2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    testutil::CHCKAD(
        b"STATE0 velocity",
        STATE0.subarray(4),
        b"~/",
        STATE2.subarray(4),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look up state of the Uranus barycenter relative to the sun, in frame J2000. Our Uranus barycenter segment has a dynamic base frame requiring three levels of recursion for evaluation.", ctx)?;

    //
    // Look up the state of body 700 relative to the solar system
    // barycenter in the dynamic frame RECUR_3.  We expect an error
    // to be signaled.
    //
    spicelib::SPKEZ(
        700,
        ET,
        b"J2000",
        b"NONE",
        10,
        STATE0.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(RECURSIONTOODEEP)", OK, ctx)?;

    //
    //     Now create a CK containing a segment with dynamic base
    //     frame.
    //
    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Create a CK for tests using dynamic base frames.", ctx)?;

    //
    // Start with the CK file.  Create but don't load the file.
    //
    testutil::TSTCK3(CK2, SCLK, false, false, false, &mut HANDLE[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Locate the segment for the instrument -9901; change the descriptor
    // to make the base frame RECUR_2. Change the ID code of the
    // instrument at the same time, so we can compare pointing from the
    // new segments with those from the old.
    //
    // Change the frame for the instrument -10000 to RECUR_3.  This
    // segment should become unusable; we'll check this later.
    //
    spicelib::DAFOPW(CK2, &mut HANDLE[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFBFS(HANDLE[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DAFFNA(&mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    while FOUND {
        spicelib::DAFGS(SUM.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::DAFUS(SUM.as_slice(), ND, NI, DC.as_slice_mut(), IC.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (IC[1] == -10001) {
            //
            // Update the descriptor.  For CK descriptors, the frame
            // ID is in the second element of the integer component.
            //
            spicelib::NAMFRM(b"RECUR_2", &mut IC[2], ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            IC[1] = (IC[1] + 100);

            spicelib::DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), SUM.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Write the new descriptor.
            //
            spicelib::DAFRS(SUM.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        } else if (IC[1] == -10000) {
            //
            // Update the descriptor.
            //
            spicelib::NAMFRM(b"RECUR_3", &mut IC[2], ctx)?;
            IC[1] = (IC[1] + 100);

            spicelib::DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), SUM.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            //
            // Write the new descriptor.
            //
            spicelib::DAFRS(SUM.as_slice(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }

        spicelib::DAFFNA(&mut FOUND, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Close the modified CK file.
    //
    spicelib::DAFCLS(HANDLE[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Look up pointing of instrument -9901 in frame RECUR_2.",
        ctx,
    )?;

    //
    // Load the CK containing data of interest.
    //
    spicelib::CKLPF(CK2, &mut HANDLE[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the pointing and angular velocity of instrument -9901 in
    // the dynamic frame RECUR_2. The data should match those of the
    // instrument -10001 in frame J2000.
    //
    INSTID = -9901;
    CLKID = -9;

    spicelib::STR2ET(b"2005 JAN 1 12:00 TDB", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCE2C(CLKID, ET, &mut SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up pointing relative to frame RECUR_2.
    //
    spicelib::CKGPAV(
        INSTID,
        SCLKDP,
        0.0,
        b"RECUR_2",
        CMAT0.as_slice_mut(),
        AV0.as_slice_mut(),
        &mut CLKOUT,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"-9901 FOUND", FOUND, true, OK, ctx)?;

    //
    // Look up the same data for instrument -10001 using the
    // J2000 frame (base frame for instrument -10001).
    // Compare.
    //
    spicelib::CKGPAV(
        -10001,
        SCLKDP,
        0.0,
        b"J2000",
        CMAT1.as_slice_mut(),
        AV1.as_slice_mut(),
        &mut CLKOUT,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"-10001 FOUND", FOUND, true, OK, ctx)?;

    //
    // Compare C-matrices.
    //
    testutil::CHCKAD(
        b"CMAT0",
        CMAT0.as_slice(),
        b"~",
        CMAT1.as_slice(),
        9,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Compare angular velocity vectors.
    //
    testutil::CHCKAD(
        b"AV0",
        AV0.as_slice(),
        b"~",
        AV1.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look up pointing of instrument -9901 in frame J2000.  This forces the CK subsystem to do a dynamic frame transformation.", ctx)?;

    //
    // Load the CK containing data of interest.
    //
    spicelib::CKLPF(CK2, &mut HANDLE[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up the pointing and angular velocity of instrument -9901 in
    // the dynamic frame J2000.  The data should match those of the
    // instrument -10001 after the RECUR_2 to J2000 state transformation
    // is applied.
    //
    INSTID = -9901;
    CLKID = -9;

    spicelib::STR2ET(b"2005 JAN 1 12:00 TDB", &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCE2C(CLKID, ET, &mut SCLKDP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Look up pointing relative to frame J2000.
    //
    spicelib::CKGPAV(
        INSTID,
        SCLKDP,
        0.0,
        b"J2000",
        CMAT0.as_slice_mut(),
        AV0.as_slice_mut(),
        &mut CLKOUT,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"-9901 FOUND", FOUND, true, OK, ctx)?;

    //
    // Look up the same data for instrument -10001 using the
    // J2000 frame (base frame for instrument -10001).
    // Compare.
    //
    spicelib::CKGPAV(
        -10001,
        SCLKDP,
        0.0,
        b"J2000",
        CMAT1.as_slice_mut(),
        AV1.as_slice_mut(),
        &mut CLKOUT,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"-10001 FOUND", FOUND, true, OK, ctx)?;

    //
    // Get the  J2000 to RECUR_2 position transformation matrix.
    // Right-multiply CMAT1 by this.
    //
    spicelib::PXFORM(b"J2000", b"RECUR_2", ET, R.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXM(CMAT1.as_slice(), R.as_slice(), CMAT2.as_slice_mut());

    //
    // Compare C-matrices.
    //
    testutil::CHCKAD(
        b"CMAT0",
        CMAT0.as_slice(),
        b"~",
        CMAT2.as_slice(),
        9,
        EX13,
        OK,
        ctx,
    )?;

    //
    // Compare angular velocity vectors. To simplify the computations,
    // we'll work with state transformations. Construct the J2000 to
    // CK_-10001 state transformation XF2 from the corresponding
    // C-matrix and angular velocity vector. Right-multiply this by the
    // J2000 to RECUR_2 state transformation XF3. This should yield a
    // transformation XF4 equivalent to XFORM, since XFORM was
    // effectively created by the same right-multiplication of the state
    // transformation matrix mapping RECUR_2 to CK_-9901.  The angular
    // velocity vector AV2 extracted from XF4 can then be compared to
    // the original angular velocity vector AV0.
    //
    spicelib::RAV2XF(CMAT1.as_slice(), AV1.as_slice(), XF2.as_slice_mut());

    spicelib::SXFORM(b"J2000", b"RECUR_2", ET, XF3.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::MXMG(XF2.as_slice(), XF3.as_slice(), 6, 6, 6, XF4.as_slice_mut());

    //
    // Extract angular velocity from XF4.
    //
    spicelib::XF2RAV(XF4.as_slice(), CMAT2.as_slice_mut(), AV2.as_slice_mut());

    //
    // Check AV0.
    //
    testutil::CHCKAD(
        b"AV0",
        AV0.as_slice(),
        b"~",
        AV2.as_slice(),
        3,
        EX13,
        OK,
        ctx,
    )?;

    //
    //--- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look up pointing of instrument -9900 in frame J2000.  The segment for -9900 has a dynamic base frame requiring three levels of recursion for evaluation.", ctx)?;

    //
    // Look up the state of body 700 relative to the solar system
    // barycenter in the dynamic frame RECUR_3.  We expect an error
    // to be signaled.
    //
    spicelib::CKGPAV(
        -9900,
        SCLKDP,
        0.0,
        b"J2000",
        CMAT0.as_slice_mut(),
        AV0.as_slice_mut(),
        &mut CLKOUT,
        &mut FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(RECURSIONTOODEEP)", OK, ctx)?;

    //
    // End of test cases.
    //

    //
    // Clean up the SPK and CK files.
    //
    spicelib::SPKUEF(HANDLE[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKUPF(HANDLE[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(CKNM, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(HANDLE[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::CKUPF(HANDLE[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    spicelib::DELFIL(CK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
