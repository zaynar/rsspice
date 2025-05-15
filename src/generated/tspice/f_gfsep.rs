//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CNVTOL: f64 = 0.000001;
const NWMAX: i32 = 15;
const NWDIST: i32 = 5;
const NWSEP: i32 = 5;
const NWRR: i32 = 5;
const NWUDS: i32 = 5;
const NWPA: i32 = 5;
const NWILUM: i32 = 5;
const ADDWIN: f64 = 0.5;
const FRMNLN: i32 = 32;
const FOVTLN: i32 = 40;
const FTCIRC: &[u8] = b"CIRCLE";
const FTELLI: &[u8] = b"ELLIPSE";
const FTPOLY: &[u8] = b"POLYGON";
const FTRECT: &[u8] = b"RECTANGLE";
const ANNULR: &[u8] = b"ANNULAR";
const ANY: &[u8] = b"ANY";
const PARTL: &[u8] = b"PARTIAL";
const FULL: &[u8] = b"FULL";
const DSSHAP: &[u8] = b"DSK";
const EDSHAP: &[u8] = b"ELLIPSOID";
const PTSHAP: &[u8] = b"POINT";
const RYSHAP: &[u8] = b"RAY";
const SPSHAP: &[u8] = b"SPHERE";
const NOCTYP: i32 = 4;
const OCLLN: i32 = 7;
const SHPLEN: i32 = 9;
const MAXVRT: i32 = 10000;
const CIRFOV: &[u8] = b"CIRCLE";
const ELLFOV: &[u8] = b"ELLIPSE";
const POLFOV: &[u8] = b"POLYGON";
const RECFOV: &[u8] = b"RECTANGLE";
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const PCK: &[u8] = b"gfsep.pck";
const LSK: &[u8] = b"gfsep.tls";
const SPK1: &[u8] = b"nat.bsp";
const PCK1: &[u8] = b"nat.pck";
const MEDIUM: f64 = 0.00000001;
const LNSIZE: i32 = 80;
const TIMLEN: i32 = 50;
const LBCELL: i32 = -5;
const MAXWIN: i32 = 10000;
const NCORR: i32 = 9;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORR: ActualCharArray,
    SHAPES: ActualCharArray,
    TARG1: Vec<u8>,
    TARG2: Vec<u8>,
    SHAPE1: Vec<u8>,
    SHAPE2: Vec<u8>,
    FRAME1: Vec<u8>,
    FRAME2: Vec<u8>,
    TITLE: Vec<u8>,
    TIME0: Vec<u8>,
    TIME1: Vec<u8>,
    LEFT: f64,
    RIGHT: f64,
    POSA: StackArray<f64, 3>,
    POSB: StackArray<f64, 3>,
    RADA: StackArray<f64, 3>,
    RADB: StackArray<f64, 3>,
    HANGA: f64,
    HANGB: f64,
    ADJUST: f64,
    CNFINE: ActualArray<f64>,
    PI_: f64,
    ET0: f64,
    ET1: f64,
    BEG: f64,
    END: f64,
    LT: f64,
    REFVAL: f64,
    RESULT: ActualArray<f64>,
    STEP: f64,
    WORK: ActualArray2D<f64>,
    SEP: f64,
    HAN1: i32,
    COUNT: i32,
    DIM: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut SHAPES = ActualCharArray::new(LNSIZE, 1..=2);
        let mut TARG1 = vec![b' '; LNSIZE as usize];
        let mut TARG2 = vec![b' '; LNSIZE as usize];
        let mut SHAPE1 = vec![b' '; LNSIZE as usize];
        let mut SHAPE2 = vec![b' '; LNSIZE as usize];
        let mut FRAME1 = vec![b' '; LNSIZE as usize];
        let mut FRAME2 = vec![b' '; LNSIZE as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TIME0 = vec![b' '; TIMLEN as usize];
        let mut TIME1 = vec![b' '; TIMLEN as usize];
        let mut LEFT: f64 = 0.0;
        let mut RIGHT: f64 = 0.0;
        let mut POSA = StackArray::<f64, 3>::new(1..=3);
        let mut POSB = StackArray::<f64, 3>::new(1..=3);
        let mut RADA = StackArray::<f64, 3>::new(1..=3);
        let mut RADB = StackArray::<f64, 3>::new(1..=3);
        let mut HANGA: f64 = 0.0;
        let mut HANGB: f64 = 0.0;
        let mut ADJUST: f64 = 0.0;
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut PI_: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut BEG: f64 = 0.0;
        let mut END: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut STEP: f64 = 0.0;
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=NWSEP);
        let mut SEP: f64 = 0.0;
        let mut HAN1: i32 = 0;
        let mut COUNT: i32 = 0;
        let mut DIM: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"lt"),
                Val::C(b" lt+s"),
                Val::C(b" cn"),
                Val::C(b" cn + s"),
                Val::C(b"XLT"),
                Val::C(b"XLT + S"),
                Val::C(b"XCN"),
                Val::C(b"XCN+S"),
            ]
            .into_iter();
            CORR.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"POINT"), Val::C(b"SPHERE")].into_iter();
            SHAPES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CORR,
            SHAPES,
            TARG1,
            TARG2,
            SHAPE1,
            SHAPE2,
            FRAME1,
            FRAME2,
            TITLE,
            TIME0,
            TIME1,
            LEFT,
            RIGHT,
            POSA,
            POSB,
            RADA,
            RADB,
            HANGA,
            HANGB,
            ADJUST,
            CNFINE,
            PI_,
            ET0,
            ET1,
            BEG,
            END,
            LT,
            REFVAL,
            RESULT,
            STEP,
            WORK,
            SEP,
            HAN1,
            COUNT,
            DIM,
        }
    }
}

//$Procedure F_GFSEP ( GFSEP family tests )
pub fn F_GFSEP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Save everything
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFSEP", ctx)?;

    save.PI_ = spicelib::PI(ctx);

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    //
    // Create an LSK, load using FURNSH.
    //
    testutil::ZZTSTLSK(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a PCK, load using FURNSH.
    //
    testutil::ZZTSTPCK(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the PCK for Nat's Solar System.
    //
    testutil::NATPCK(PCK1, false, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create the SPK for Nat's Solar System.
    //
    testutil::NATSPK(SPK1, true, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a confinement window from ET0 and ET1.
    //
    fstr::assign(&mut save.TIME0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.TIME1, b"2000 APR 1  00:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.TIME1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Error cases
    //

    //
    // Case 1
    //
    testutil::TCASE(b"Non-positive step size", ctx)?;

    save.STEP = 0.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARG1, b"MOON");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"IAU_MOON");
    fstr::assign(&mut save.TARG2, b"EARTH");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"NULL");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"SUN",
        b"=",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    // Case 2
    //
    testutil::TCASE(b"Non unique body IDs.", ctx)?;

    save.STEP = spicelib::SPD();
    fstr::assign(&mut save.TARG1, b"MOON");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"IAU_MOON");
    fstr::assign(&mut save.TARG2, b"MOON");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"IAU_MOON");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"SUN",
        b"=",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    fstr::assign(&mut save.TARG1, b"EARTH");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"MOON");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"IAU_MOON");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"MOON",
        b"=",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    fstr::assign(&mut save.TARG1, b"EARTH");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"MOON");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"IAU_MOON");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"EARTH",
        b"=",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // Case 3
    //
    testutil::TCASE(b"Invalid aberration correction specifier", ctx)?;

    fstr::assign(&mut save.TARG1, b"EARTH");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"MOON");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"IAU_MOON");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        b"X",
        b"SUN",
        b"=",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Case 4
    //
    testutil::TCASE(b"Invalid relations operator", ctx)?;

    fstr::assign(&mut save.TARG1, b"EARTH");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"MOON");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"IAU_MOON");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"SUN",
        b"==",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    // Case 5
    //
    testutil::TCASE(b"Invalid body names", ctx)?;

    fstr::assign(&mut save.TARG1, b"X");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"MOON");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"IAU_MOON");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"SUN",
        b"ABSMAX",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    fstr::assign(&mut save.TARG1, b"EARTH");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"X");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"NULL");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"MOON",
        b"ABSMAX",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    fstr::assign(&mut save.TARG1, b"EARTH");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"MOON");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"NULL");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"X",
        b"ABSMAX",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // Case 6
    //
    testutil::TCASE(b"Negative adjustment value", ctx)?;

    save.ADJUST = -1.0;
    fstr::assign(&mut save.TARG1, b"EARTH");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"MOON");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"NULL");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"SUN",
        b"ABSMAX",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Case 7
    //
    testutil::TCASE(b"Ephemeris data unavailable", ctx)?;

    save.ADJUST = 0.0;

    //
    // A vehicle/probe has a point shape so frame name not processed.
    //
    fstr::assign(&mut save.TARG1, b"DAWN");
    fstr::assign(&mut save.SHAPE1, b"POINT");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"MARS");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"IAU_MARS");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"SUN",
        b"ABSMAX",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // Case 8
    //
    testutil::TCASE(b"Unknown shape name.", ctx)?;

    save.STEP = spicelib::SPD();
    fstr::assign(&mut save.TARG1, b"MOON");
    fstr::assign(&mut save.SHAPE1, b"PANCAKE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"EARTH");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"NULL");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"SUN",
        b"=",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    fstr::assign(&mut save.TARG1, b"MOON");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"EARTH");
    fstr::assign(&mut save.SHAPE2, b"BLOB");
    fstr::assign(&mut save.FRAME2, b"NULL");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"SUN",
        b"=",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    // Case 9
    //
    testutil::TCASE(b"Invalid value for MW", ctx)?;

    save.STEP = spicelib::SPD();
    fstr::assign(&mut save.TARG1, b"MOON");
    fstr::assign(&mut save.SHAPE1, b"SPHERE");
    fstr::assign(&mut save.FRAME1, b"NULL");
    fstr::assign(&mut save.TARG2, b"EARTH");
    fstr::assign(&mut save.SHAPE2, b"SPHERE");
    fstr::assign(&mut save.FRAME2, b"NULL");

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"SUN",
        b"=",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        1,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    spicelib::GFSEP(
        &save.TARG1,
        &save.SHAPE1,
        &save.FRAME1,
        &save.TARG2,
        &save.SHAPE2,
        &save.FRAME2,
        &save.CORR[1],
        b"SUN",
        b"=",
        0.0,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        3,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Case 10
    //
    testutil::TCASE(b"ABSMIN", ctx)?;

    //
    // Perform a simple search using ALPHA and BETA for times
    // of minimum angular separation as seen from the sun.
    // Recall ALPHA - BETA occultation occurs every day
    // at 12:00 PM TDB.
    //

    save.STEP = 60.0;
    save.ADJUST = 0.0;
    save.REFVAL = 0.0;

    //
    // Store the time bounds of our search interval in
    // the CNFINE confinement window.
    //
    spicelib::STR2ET(b"2000 JAN 01 TDB", &mut save.ET0, ctx)?;
    spicelib::STR2ET(b"2000 JAN 02 TDB", &mut save.ET1, ctx)?;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;

    fstr::assign(&mut save.FRAME1, b"ALPHAFIXED");
    fstr::assign(&mut save.FRAME2, b"BETAFIXED");

    for I in 1..=2 {
        for J in 1..=2 {
            spicelib::GFSEP(
                b"ALPHA",
                &save.SHAPES[I],
                &save.FRAME1,
                b"BETA",
                &save.SHAPES[J],
                &save.FRAME2,
                b"NONE",
                b"SUN",
                b"ABSMIN",
                save.REFVAL,
                save.ADJUST,
                save.STEP,
                save.CNFINE.as_slice(),
                MAXWIN,
                NWSEP,
                save.WORK.as_slice_mut(),
                save.RESULT.as_slice_mut(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check the number of intervals in the result window.
            //
            save.COUNT = 0;
            save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
            testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;

            if (save.COUNT == 1) {
                spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

                spicelib::SPKPOS(
                    b"ALPHA",
                    save.BEG,
                    b"J2000",
                    b"NONE",
                    b"SUN",
                    save.POSA.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                spicelib::SPKPOS(
                    b"BETA",
                    save.BEG,
                    b"J2000",
                    b"NONE",
                    b"SUN",
                    save.POSB.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;

                //
                // The angular separation should be within the MEDIUM
                // tolerance of 0.D0.
                //
                testutil::CHCKSD(
                    b"LOCMIN",
                    spicelib::VSEP(save.POSA.as_slice(), save.POSB.as_slice(), ctx),
                    b"~",
                    0.0,
                    MEDIUM,
                    OK,
                    ctx,
                )?;
            }
        }
    }

    spicelib::BODVRD(
        b"ALPHA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RADA.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKSI(b"BODVRD ALPHA", save.DIM, b"=", 3, 0, OK, ctx)?;

    spicelib::BODVRD(
        b"BETA",
        b"RADII",
        3,
        &mut save.DIM,
        save.RADB.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKSI(b"BODVRD ALPHA", save.DIM, b"=", 3, 0, OK, ctx)?;

    //
    // Perform the test blocks for each aberration correction value.
    //

    fstr::assign(&mut save.TARG1, b"ALPHA");
    fstr::assign(&mut save.FRAME1, b"ALPHAFIXED");
    fstr::assign(&mut save.TARG2, b"BETA");
    fstr::assign(&mut save.FRAME2, b"BETAFIXED");

    for J in 1..=NCORR {
        fstr::assign(&mut save.ABCORR, save.CORR.get(J));

        //
        // Case 11
        //
        spicelib::REPMC(
            b"Relative =, sphere/sphere #",
            b"#",
            &save.ABCORR,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        fstr::assign(&mut save.SHAPE1, b"SPHERE");
        fstr::assign(&mut save.SHAPE2, b"SPHERE");

        //
        // Perform a simple search using ALPHA and BETA for times
        // angular separation equal zero for the allowed body shapes.
        //
        save.REFVAL = 0.0;

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"=",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;

        //
        // Two events should exist during the defined time interval.
        // We test one of the two.
        //
        if (save.COUNT == 2) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::SPKPOS(
                &save.TARG1,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSA.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            spicelib::SPKPOS(
                &save.TARG2,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSB.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            save.HANGA = f64::asin((save.RADA[1] / spicelib::VNORM(save.POSA.as_slice())));
            save.HANGB = f64::asin((save.RADB[1] / spicelib::VNORM(save.POSB.as_slice())));

            //
            // The angular separation should be within the MEDIUM
            // tolerance of the sum of body's half angles.
            //
            testutil::CHCKSD(
                &save.TITLE,
                spicelib::VSEP(save.POSA.as_slice(), save.POSB.as_slice(), ctx),
                b"~",
                (save.HANGA + save.HANGB),
                MEDIUM,
                OK,
                ctx,
            )?;
        }

        //
        // Case 12
        //
        spicelib::REPMC(
            b"Relative =, sphere/point #",
            b"#",
            &save.ABCORR,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        fstr::assign(&mut save.SHAPE1, b"SPHERE");
        fstr::assign(&mut save.SHAPE2, b"POINT");

        save.REFVAL = 0.0;

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"=",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;

        //
        // Two events should exist during the defined time interval.
        // We test one of the two.
        //
        if (save.COUNT == 2) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::SPKPOS(
                &save.TARG1,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSA.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            spicelib::SPKPOS(
                &save.TARG2,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSB.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            save.HANGA = f64::asin((save.RADA[1] / spicelib::VNORM(save.POSA.as_slice())));

            //
            // The angular separation should be within the MEDIUM
            // tolerance of body ALPHA's half angle.
            //
            testutil::CHCKSD(
                &save.TITLE,
                spicelib::VSEP(save.POSA.as_slice(), save.POSB.as_slice(), ctx),
                b"~",
                save.HANGA,
                MEDIUM,
                OK,
                ctx,
            )?;
        }

        //
        // Case 13
        //
        spicelib::REPMC(
            b"Relative =, point/sphere #",
            b"#",
            &save.ABCORR,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        fstr::assign(&mut save.SHAPE1, b"POINT");
        fstr::assign(&mut save.SHAPE2, b"SPHERE");

        save.REFVAL = 0.0;

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"=",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 2, 0, OK, ctx)?;

        //
        // Two events should exist during the defined time interval.
        // We test one of the two.
        //
        if (save.COUNT == 2) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::SPKPOS(
                &save.TARG1,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSA.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            spicelib::SPKPOS(
                &save.TARG2,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSB.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            save.HANGB = f64::asin((save.RADB[1] / spicelib::VNORM(save.POSB.as_slice())));

            //
            // The angular separation should be within the MEDIUM
            // tolerance of body BETA's half angle.
            //
            testutil::CHCKSD(
                &save.TITLE,
                spicelib::VSEP(save.POSA.as_slice(), save.POSB.as_slice(), ctx),
                b"~",
                save.HANGB,
                MEDIUM,
                OK,
                ctx,
            )?;
        }

        //
        // Case 14
        //
        testutil::TCASE(b"Relative =, point/point", ctx)?;

        fstr::assign(&mut save.SHAPE1, b"POINT");
        fstr::assign(&mut save.SHAPE2, b"POINT");

        save.REFVAL = 0.0;

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"ABSMIN",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;

        //
        // One event should exist during the defined time interval
        // (as we search for a non-zero separation).
        //
        if (save.COUNT == 1) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::SPKPOS(
                &save.TARG1,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSA.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            spicelib::SPKPOS(
                &save.TARG2,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSB.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            //
            // The angular separation should be within the MEDIUM
            // tolerance of zero.
            //
            testutil::CHCKSD(
                b"Relative =, point/point",
                spicelib::VSEP(save.POSA.as_slice(), save.POSB.as_slice(), ctx),
                b"~",
                0.0,
                MEDIUM,
                OK,
                ctx,
            )?;
        }

        //
        // Case 15
        //
        spicelib::REPMC(
            b"ABSMAX #, sphere/sphere",
            b"#",
            &save.ABCORR,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        fstr::assign(&mut save.SHAPE1, b"SPHERE");
        fstr::assign(&mut save.SHAPE2, b"SPHERE");

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"ABSMAX",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;

        //
        // One events should exist during the defined time interval.
        //
        if (save.COUNT == 1) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::SPKPOS(
                &save.TARG1,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSA.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            spicelib::SPKPOS(
                &save.TARG2,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSB.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            save.HANGA = f64::asin((save.RADA[1] / spicelib::VNORM(save.POSA.as_slice())));
            save.HANGB = f64::asin((save.RADB[1] / spicelib::VNORM(save.POSB.as_slice())));

            save.SEP = spicelib::ZZSEPQ(
                save.BEG,
                1000,
                2000,
                save.RADA[1],
                save.RADB[1],
                10,
                &save.ABCORR,
                b"J2000",
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The angular separation should be within the MEDIUM
            // tolerance of PI - HANGA - HANGB.
            //
            testutil::CHCKSD(
                &save.TITLE,
                (save.PI_ - (save.HANGA + save.HANGB)),
                b"~",
                save.SEP,
                MEDIUM,
                OK,
                ctx,
            )?;
        }

        //
        // Case 16
        //
        spicelib::REPMC(
            b"ABSMAX #, sphere/point",
            b"#",
            &save.ABCORR,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        fstr::assign(&mut save.SHAPE1, b"SPHERE");
        fstr::assign(&mut save.SHAPE2, b"POINT");

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"ABSMAX",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;

        //
        // Once events should exist during the defined time interval.
        //
        if (save.COUNT == 1) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::SPKPOS(
                &save.TARG1,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSA.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            save.HANGA = f64::asin((save.RADA[1] / spicelib::VNORM(save.POSA.as_slice())));

            //
            // The angular separation should be within the MEDIUM
            // tolerance of body BETA's half angle.
            //

            save.SEP = spicelib::ZZSEPQ(
                save.BEG,
                1000,
                2000,
                save.RADA[1],
                0.0,
                10,
                &save.ABCORR,
                b"J2000",
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The angular separation should be within the MEDIUM
            // tolerance  of PI - HANGA.
            //
            testutil::CHCKSD(
                &save.TITLE,
                (save.PI_ - save.HANGA),
                b"~",
                save.SEP,
                MEDIUM,
                OK,
                ctx,
            )?;
        }

        //
        // Case 17
        //
        spicelib::REPMC(
            b"ABSMAX #, point/sphere",
            b"#",
            &save.ABCORR,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        fstr::assign(&mut save.SHAPE1, b"POINT");
        fstr::assign(&mut save.SHAPE2, b"SPHERE");

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"ABSMAX",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;

        //
        // Once events should exist during the defined time interval.
        //
        if (save.COUNT == 1) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::SPKPOS(
                &save.TARG2,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSB.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            save.HANGB = f64::asin((save.RADB[1] / spicelib::VNORM(save.POSB.as_slice())));

            //
            // The angular separation should be within the MEDIUM
            // tolerance of body BETA's half angle.
            //

            save.SEP = spicelib::ZZSEPQ(
                save.BEG,
                1000,
                2000,
                0.0,
                save.RADB[1],
                10,
                &save.ABCORR,
                b"J2000",
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The angular separation should be within the MEDIUM
            // tolerance of PI - HANGB.
            //
            testutil::CHCKSD(
                &save.TITLE,
                (save.PI_ - save.HANGB),
                b"~",
                save.SEP,
                MEDIUM,
                OK,
                ctx,
            )?;
        }

        //
        // Case 18
        //
        spicelib::REPMC(
            b"ABSMAX #, point/point",
            b"#",
            &save.ABCORR,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        fstr::assign(&mut save.SHAPE1, b"POINT");
        fstr::assign(&mut save.SHAPE2, b"POINT");

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"ABSMAX",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;

        //
        // Once events should exist during the defined time interval.
        //
        if (save.COUNT == 1) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            //
            // The angular separation should be within the MEDIUM
            // tolerance of body BETA's half angle.
            //

            save.SEP = spicelib::ZZSEPQ(
                save.BEG,
                1000,
                2000,
                0.0,
                0.0,
                10,
                &save.ABCORR,
                b"J2000",
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The angular separation should be within the MEDIUM
            // tolerance of PI.
            //
            testutil::CHCKSD(&save.TITLE, save.PI_, b"~", save.SEP, MEDIUM, OK, ctx)?;
        }

        //
        // Case 19
        //
        spicelib::REPMC(
            b"ABSMIN #, sphere/sphere",
            b"#",
            &save.ABCORR,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        fstr::assign(&mut save.SHAPE1, b"SPHERE");
        fstr::assign(&mut save.SHAPE2, b"SPHERE");

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"ABSMIN",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;

        //
        // One events should exist during the defined time interval.
        //
        if (save.COUNT == 1) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::SPKPOS(
                &save.TARG1,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSA.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            spicelib::SPKPOS(
                &save.TARG2,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSB.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            save.HANGA = f64::asin((save.RADA[1] / spicelib::VNORM(save.POSA.as_slice())));
            save.HANGB = f64::asin((save.RADB[1] / spicelib::VNORM(save.POSB.as_slice())));

            //
            // The angular separation should be within the MEDIUM
            // tolerance of body BETA's half angle.
            //

            save.SEP = spicelib::ZZSEPQ(
                save.BEG,
                1000,
                2000,
                save.RADA[1],
                save.RADB[1],
                10,
                &save.ABCORR,
                b"J2000",
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The angular separation should be within the MEDIUM
            // tolerance of - HANGA - HANGB.
            //
            testutil::CHCKSD(
                &save.TITLE,
                -(save.HANGA + save.HANGB),
                b"~",
                save.SEP,
                MEDIUM,
                OK,
                ctx,
            )?;
        }

        //
        // Case 20
        //
        spicelib::REPMC(
            b"ABSMIN #, sphere/point",
            b"#",
            &save.ABCORR,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        fstr::assign(&mut save.SHAPE1, b"SPHERE");
        fstr::assign(&mut save.SHAPE2, b"POINT");

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"ABSMIN",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;

        //
        // One events should exist during the defined time interval.
        //
        if (save.COUNT == 1) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::SPKPOS(
                &save.TARG1,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSA.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            save.HANGA = f64::asin((save.RADA[1] / spicelib::VNORM(save.POSA.as_slice())));

            //
            // The angular separation should be within the MEDIUM
            // tolerance of body BETA's half angle.
            //

            save.SEP = spicelib::ZZSEPQ(
                save.BEG,
                1000,
                2000,
                save.RADA[1],
                0.0,
                10,
                &save.ABCORR,
                b"J2000",
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The angular separation should be within the MEDIUM
            // tolerance of -HANGA.
            //
            testutil::CHCKSD(&save.TITLE, -save.HANGA, b"~", save.SEP, MEDIUM, OK, ctx)?;
        }

        //
        // Case 21
        //
        spicelib::REPMC(
            b"ABSMIN #, point/sphere",
            b"#",
            &save.ABCORR,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        fstr::assign(&mut save.SHAPE1, b"POINT");
        fstr::assign(&mut save.SHAPE2, b"SPHERE");

        spicelib::GFSEP(
            &save.TARG1,
            &save.SHAPE1,
            &save.FRAME1,
            &save.TARG2,
            &save.SHAPE2,
            &save.FRAME2,
            &save.ABCORR,
            b"SUN",
            b"ABSMIN",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWSEP,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", 1, 0, OK, ctx)?;

        //
        // Once events should exist during the defined time interval.
        //
        if (save.COUNT == 1) {
            spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::SPKPOS(
                &save.TARG2,
                save.BEG,
                b"J2000",
                &save.ABCORR,
                b"SUN",
                save.POSB.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;

            save.HANGB = f64::asin((save.RADB[1] / spicelib::VNORM(save.POSB.as_slice())));

            //
            // The angular separation should be within the MEDIUM
            // tolerance of body BETA's half angle.
            //

            save.SEP = spicelib::ZZSEPQ(
                save.BEG,
                1000,
                2000,
                0.0,
                save.RADB[1],
                10,
                &save.ABCORR,
                b"J2000",
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // The angular separation should be within the MEDIUM
            // tolerance of - HANGB.
            //
            testutil::CHCKSD(&save.TITLE, -save.HANGB, b"~", save.SEP, MEDIUM, OK, ctx)?;
        }
    }

    //
    // Case 22
    //
    fstr::assign(&mut save.TITLE, b"Check the GF call uses the GFSTOL value");
    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Re-run a valid search after using GFSTOL to set the convergence
    // tolerance to a value that should cause a numerical error signal.
    //

    save.STEP = 60.0;
    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.FRAME1, b"ALPHAFIXED");
    fstr::assign(&mut save.FRAME2, b"BETAFIXED");

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFSEP(
        b"ALPHA",
        &save.SHAPES[1],
        &save.FRAME1,
        b"BETA",
        &save.SHAPES[1],
        &save.FRAME2,
        b"NONE",
        b"SUN",
        b"ABSMIN",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.COUNT = 0;
    save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"COUNT", save.COUNT, b"!=", 0, 0, OK, ctx)?;

    spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Reset tol.
    //

    spicelib::GFSTOL(0.0001, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFSEP(
        b"ALPHA",
        &save.SHAPES[1],
        &save.FRAME1,
        b"BETA",
        &save.SHAPES[1],
        &save.FRAME2,
        b"NONE",
        b"SUN",
        b"ABSMIN",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWSEP,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.COUNT = 0;
    save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
    testutil::CHCKSI(b"COUNT", save.COUNT, b"!=", 0, 0, OK, ctx)?;

    spicelib::WNFETD(
        save.RESULT.as_slice(),
        1,
        &mut save.LEFT,
        &mut save.RIGHT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The values in the time window should not match
    // as the search used different tolerances. Check
    // the first value in the first interval.
    //
    testutil::CHCKSD(&save.TITLE, save.BEG, b"!=", save.LEFT, 0.0, OK, ctx)?;

    //
    // Reset the convergence tolerance.
    //
    spicelib::GFSTOL(CNVTOL, ctx)?;

    //
    // Case n
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::KCLEAR(ctx)?;

    spicelib::DELFIL(PCK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(LSK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
