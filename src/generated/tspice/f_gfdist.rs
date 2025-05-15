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
const SPK: &[u8] = b"gfdist.bsp";
const MEDIUM: f64 = 0.0001;
const VTIGHT: f64 = 0.000000000001;
const TIMTOL: f64 = 0.000003;
const TDELTA: f64 = 30.0;
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 80;
const TIMLEN: i32 = 50;
const LBCELL: i32 = -5;
const MAXWIN: i32 = 20000;
const NCORR: i32 = 9;
const NSAMP: i32 = 100;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORR: ActualCharArray,
    OBSRVR: Vec<u8>,
    TARGET: Vec<u8>,
    TITLE: Vec<u8>,
    TIME0: Vec<u8>,
    TIME1: Vec<u8>,
    ABSMAX: f64,
    BEG: f64,
    END: f64,
    ABSMIN: f64,
    ADJUST: f64,
    SPAN: f64,
    CNFIN1: ActualArray<f64>,
    CNFIN2: ActualArray<f64>,
    CNFINE: ActualArray<f64>,
    COMP: ActualArray<f64>,
    DIST: f64,
    ET0: f64,
    ET1: f64,
    LEFT: f64,
    LT: f64,
    MAXDST: f64,
    MINDST: f64,
    POS: StackArray<f64, 3>,
    REFVAL: f64,
    RES2: ActualArray<f64>,
    RESULT: ActualArray<f64>,
    RESGT: ActualArray<f64>,
    RESLT: ActualArray<f64>,
    RIGHT: f64,
    STEPSZ: f64,
    STEP: f64,
    T: f64,
    WORK: ActualArray2D<f64>,
    XLEFT: f64,
    XRIGHT: f64,
    XTIME: f64,
    HANDLE: i32,
    COUNT: i32,
    IXMAX: i32,
    IXMIN: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TIME0 = vec![b' '; TIMLEN as usize];
        let mut TIME1 = vec![b' '; TIMLEN as usize];
        let mut ABSMAX: f64 = 0.0;
        let mut BEG: f64 = 0.0;
        let mut END: f64 = 0.0;
        let mut ABSMIN: f64 = 0.0;
        let mut ADJUST: f64 = 0.0;
        let mut SPAN: f64 = 0.0;
        let mut CNFIN1 = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut CNFIN2 = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut COMP = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut DIST: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut LEFT: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut MAXDST: f64 = 0.0;
        let mut MINDST: f64 = 0.0;
        let mut POS = StackArray::<f64, 3>::new(1..=3);
        let mut REFVAL: f64 = 0.0;
        let mut RES2 = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut RESGT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut RESLT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut RIGHT: f64 = 0.0;
        let mut STEPSZ: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut T: f64 = 0.0;
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=NWMAX);
        let mut XLEFT: f64 = 0.0;
        let mut XRIGHT: f64 = 0.0;
        let mut XTIME: f64 = 0.0;
        let mut HANDLE: i32 = 0;
        let mut COUNT: i32 = 0;
        let mut IXMAX: i32 = 0;
        let mut IXMIN: i32 = 0;

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

        Self {
            ABCORR,
            CORR,
            OBSRVR,
            TARGET,
            TITLE,
            TIME0,
            TIME1,
            ABSMAX,
            BEG,
            END,
            ABSMIN,
            ADJUST,
            SPAN,
            CNFIN1,
            CNFIN2,
            CNFINE,
            COMP,
            DIST,
            ET0,
            ET1,
            LEFT,
            LT,
            MAXDST,
            MINDST,
            POS,
            REFVAL,
            RES2,
            RESULT,
            RESGT,
            RESLT,
            RIGHT,
            STEPSZ,
            STEP,
            T,
            WORK,
            XLEFT,
            XRIGHT,
            XTIME,
            HANDLE,
            COUNT,
            IXMAX,
            IXMIN,
        }
    }
}

//$Procedure F_GFDIST ( GFDIST family tests )
pub fn F_GFDIST(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Saved everything.
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFDIST", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load an SPK file as well.
    //
    testutil::TSTSPK(SPK, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    // CALL FURNSH ( 'de421.bsp' )
    // CALL CHCKXC ( .FALSE., ' ', OK )

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Invalid result window size", ctx)?;

    spicelib::SSIZED(1, save.RESULT.as_slice_mut(), ctx)?;

    save.STEP = 10.0;
    save.ADJUST = 0.0;

    save.REFVAL = 0.0;

    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    spicelib::SSIZED(0, save.RESULT.as_slice_mut(), ctx)?;

    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive step size", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 0.0;
    save.ADJUST = 0.0;
    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    save.STEP = -1.0;
    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Workspace too small", ctx)?;

    save.STEP = ((7 as f64) * spicelib::SPD());

    save.LEFT = 0.0;
    save.RIGHT = ((60 as f64) * spicelib::SPD());

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Size is zero.
    //
    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b"LOCMAX",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        0,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Size is positive but below limit.
    //
    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b"LOCMAX",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        1,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Size is positive but is too small (error discovery at
    // search time).
    //
    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b"LOCMAX",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        2,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WINDOWEXCESS)", OK, ctx)?;

    //
    // Window count is positive but below limit.
    //
    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b"LOCMAX",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        (NWDIST - 1),
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Result window too small", ctx)?;

    save.STEP = ((7 as f64) * spicelib::SPD());

    save.LEFT = 0.0;
    save.RIGHT = ((60 as f64) * spicelib::SPD());

    spicelib::SSIZED(2, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b"LOCMAX",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(OUTOFROOM)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Invalid operator", ctx)?;

    save.STEP = 10.0;
    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b"!=",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Negative adjustment value", ctx)?;

    save.STEP = 10.0;
    save.ADJUST = -1.0;

    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"EARTH",
        b"ABSMAX",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Target and observer are identical", ctx)?;

    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"MOON",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized target or observer", ctx)?;

    save.STEP = 1.0;

    spicelib::GFDIST(
        b"MOOON",
        b"NONE",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    spicelib::GFDIST(
        b"MOON",
        b"NONE",
        b"ERTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Ephemeris data unavailable", ctx)?;

    save.LEFT = 0.0;
    save.RIGHT = ((60 as f64) * spicelib::SPD());

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = ((7 as f64) * spicelib::SPD());
    save.ADJUST = 0.0;

    spicelib::GFDIST(
        b"GASPRA",
        b"NONE",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Bad aberration correction specifiers", ctx)?;

    save.LEFT = 0.0;
    save.RIGHT = ((60 as f64) * spicelib::SPD());

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = ((7 as f64) * spicelib::SPD());
    save.ADJUST = 0.0;

    spicelib::GFDIST(
        b"MOON",
        b"S",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFDIST(
        b"MOON",
        b"XS",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFDIST(
        b"MOON",
        b"RLT",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFDIST(
        b"MOON",
        b"XRLT",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    spicelib::GFDIST(
        b"MOON",
        b"z",
        b"EARTH",
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //

    //
    // To speed things up, we're first going to create a couple of
    // "smarter" confinement windows. The first is the set of times when
    // the uncorrected distance is at least 404000 km. We'll call this
    // CNFIN1.
    //
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.OBSRVR, b"EARTH");

    fstr::assign(
        &mut save.TITLE,
        b"Find times when #-# distance > # km. ABCORR = #.",
    );
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
    spicelib::REPMD(
        &save.TITLE.to_vec(),
        b"#",
        save.REFVAL,
        6,
        &mut save.TITLE,
        ctx,
    );
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

    testutil::TCASE(&save.TITLE, ctx)?;

    fstr::assign(&mut save.TIME0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.TIME1, b"2000 APR 1  00:00:00 TDB");

    spicelib::STR2ET(&save.TIME0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.TIME1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFIN1.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.CNFIN2.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a confinement window from ET0 and ET1.
    //
    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = spicelib::SPD();
    save.REFVAL = 404000.0;
    save.ADJUST = 0.0;

    spicelib::GFDIST(
        &save.TARGET,
        b"NONE",
        &save.OBSRVR,
        b">",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.CNFIN1.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"CNFIN1 cardinality",
        spicelib::WNCARD(save.CNFIN1.as_slice(), ctx)?,
        b"=",
        4,
        0,
        OK,
        ctx,
    )?;

    for J in 1..=spicelib::WNCARD(save.CNFIN1.as_slice(), ctx)? {
        spicelib::WNFETD(
            save.CNFIN1.as_slice(),
            J,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;
    }

    //
    // The second confinement window is used for searches for minima.
    // This will be the window over which the uncorrected distance is at
    // less than 375000 km. We'll call this CNFIN2.
    //
    save.REFVAL = 375000.0;
    save.ADJUST = 0.0;

    spicelib::GFDIST(
        &save.TARGET,
        b"NONE",
        &save.OBSRVR,
        b"<",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
        save.WORK.as_slice_mut(),
        save.CNFIN2.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"CNFIN2 cardinality",
        spicelib::WNCARD(save.CNFIN2.as_slice(), ctx)?,
        b"=",
        3,
        0,
        OK,
        ctx,
    )?;

    for J in 1..=spicelib::WNCARD(save.CNFIN2.as_slice(), ctx)? {
        spicelib::WNFETD(
            save.CNFIN2.as_slice(),
            J,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;

        // WRITE (*,*) RIGHT-LEFT
    }

    //
    // Check equality constraint for each aberration correction.
    //
    for I in 1..=NCORR {
        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(&mut save.ABCORR, save.CORR.get(I));

        save.STEP = spicelib::SPD();
        save.REFVAL = 404100.0;
        save.ADJUST = 0.0;

        fstr::assign(
            &mut save.TITLE,
            b"Find times when #-# distance = # km. ABCORR = #.",
        );
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.REFVAL,
            6,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFDIST(
            b"MOON",
            &save.ABCORR,
            b"EARTH",
            b"=",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFIN1.as_slice(),
            MAXWIN,
            NWDIST,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We need to have at least two intervals for
        // a reasonable test case.
        //
        testutil::CHCKSI(
            b"RESULT cardinality",
            spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
            b">",
            1,
            0,
            OK,
            ctx,
        )?;

        //
        // Check distance at the endpoint of the
        // intervals of the result window.
        //
        for J in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                J,
                &mut save.LEFT,
                &mut save.RIGHT,
                ctx,
            )?;

            //
            // Check the distance at the interval's endpoints.
            //
            spicelib::SPKPOS(
                &save.TARGET,
                save.LEFT,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DIST = spicelib::VNORM(save.POS.as_slice());

            //        CALL SPKEZR ( TARGET, LEFT,  'J2000', ABCORR,
            // .                    OBSRVR, STATE, LT               )
            //        CALL CHCKXC ( .FALSE., ' ',   OK )
            //        CALL VHAT ( POS, UHAT )
            //        RR = VDOT ( STATE(4), UHAT )
            //        WRITE (*,*) 'Range rate = ', RR

            fstr::assign(&mut save.TITLE, b"Interval # start: distance");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
            testutil::CHCKSD(&save.TITLE, save.DIST, b"~", save.REFVAL, MEDIUM, OK, ctx)?;

            spicelib::SPKPOS(
                &save.TARGET,
                save.RIGHT,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DIST = spicelib::VNORM(save.POS.as_slice());

            fstr::assign(&mut save.TITLE, b"Interval # stop: distance");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
            testutil::CHCKSD(&save.TITLE, save.DIST, b"~", save.REFVAL, MEDIUM, OK, ctx)?;
        }
    }

    //
    // Check > and < constraints for each aberration correction.
    //
    spicelib::SSIZED(MAXWIN, save.RESLT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESGT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.COMP.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NCORR {
        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(&mut save.ABCORR, save.CORR.get(I));

        save.STEP = spicelib::SPD();
        save.REFVAL = 404100.0;
        save.ADJUST = 0.0;

        fstr::assign(
            &mut save.TITLE,
            b"Find times when #-# distance > # km. ABCORR = #.",
        );
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.REFVAL,
            6,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Search using the > operator.
        //
        spicelib::GFDIST(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            b">",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFIN1.as_slice(),
            MAXWIN,
            NWDIST,
            save.WORK.as_slice_mut(),
            save.RESGT.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We need to have at least two intervals for
        // a reasonable test case.
        //
        testutil::CHCKSI(
            b"RESGT cardinality",
            spicelib::WNCARD(save.RESGT.as_slice(), ctx)?,
            b">",
            2,
            0,
            OK,
            ctx,
        )?;

        //
        // Check distance at the endpoints of the
        // intervals of the RESGT result window.
        //
        // At the first start and the last stop, simply
        // check that the inequality is satisfied, since
        // these bounds may be imposed by the confinement
        // window.
        //
        // Also check distance at NSAMP interior points of the
        // each interval of the RESGT result window.
        //
        for J in 1..=spicelib::WNCARD(save.RESGT.as_slice(), ctx)? {
            //
            // Check the distance at the interval's endpoints.
            //
            spicelib::WNFETD(
                save.RESGT.as_slice(),
                J,
                &mut save.LEFT,
                &mut save.RIGHT,
                ctx,
            )?;

            //        WRITE (*,*) 'RESGT interval: ', J,
            // .                  LEFT, RIGHT, RIGHT-LEFT

            spicelib::SPKPOS(
                &save.TARGET,
                save.LEFT,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DIST = spicelib::VNORM(save.POS.as_slice());

            fstr::assign(&mut save.TITLE, b"RESGT Interval # start: distance");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);

            if (J == 1) {
                testutil::CHCKSD(
                    &save.TITLE,
                    save.DIST,
                    b">",
                    (save.REFVAL - MEDIUM),
                    MEDIUM,
                    OK,
                    ctx,
                )?;
            } else {
                testutil::CHCKSD(&save.TITLE, save.DIST, b"~", save.REFVAL, MEDIUM, OK, ctx)?;
            }

            spicelib::SPKPOS(
                &save.TARGET,
                save.RIGHT,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DIST = spicelib::VNORM(save.POS.as_slice());

            fstr::assign(&mut save.TITLE, b"RESGT Interval # stop: distance");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);

            if (J == spicelib::WNCARD(save.RESGT.as_slice(), ctx)?) {
                testutil::CHCKSD(
                    &save.TITLE,
                    save.DIST,
                    b">",
                    (save.REFVAL - MEDIUM),
                    MEDIUM,
                    OK,
                    ctx,
                )?;
            } else {
                testutil::CHCKSD(&save.TITLE, save.DIST, b"~", save.REFVAL, MEDIUM, OK, ctx)?;
            }

            //
            // Sample interior points in the current interval.
            //
            save.SPAN = (save.RIGHT - save.LEFT);
            save.STEPSZ = (save.SPAN / (NSAMP + 1) as f64);

            for K in 1..=NSAMP {
                save.T = (save.LEFT + ((K as f64) * save.STEPSZ));

                spicelib::SPKPOS(
                    &save.TARGET,
                    save.T,
                    b"J2000",
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.POS.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.DIST = spicelib::VNORM(save.POS.as_slice());

                fstr::assign(&mut save.TITLE, b"RESGT Interval # sample # distance");
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                spicelib::REPMI(&save.TITLE.to_vec(), b"#", K, &mut save.TITLE, ctx);

                testutil::CHCKSD(
                    &save.TITLE,
                    save.DIST,
                    b">",
                    (save.REFVAL - MEDIUM),
                    MEDIUM,
                    OK,
                    ctx,
                )?;
            }
            //
            // We've completed tests for the current interval
            // of RESGT.
            //
        }

        //
        //---- Case -------------------------------------------------------------
        //
        //
        //        Search using the < constraint.
        //
        //        Use the settings of
        //
        //           OSERVR
        //           TARGET
        //           REFVAL
        //           ABCORR
        //           STEP
        //
        //        from the previous > test case.
        //

        fstr::assign(
            &mut save.TITLE,
            b"Find times when #-# distance < # km. ABCORR = #.",
        );
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.REFVAL,
            6,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;
        //
        // Find the complement of the result window relative
        // to the confinement window.
        //
        spicelib::WNDIFD(
            save.CNFIN1.as_slice(),
            save.RESGT.as_slice(),
            save.COMP.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Search using the < operator.
        //
        spicelib::GFDIST(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            b"<",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFIN1.as_slice(),
            MAXWIN,
            NWDIST,
            save.WORK.as_slice_mut(),
            save.RESLT.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We need to have at least two intervals for
        // a reasonable test case.
        //
        testutil::CHCKSI(
            b"RESLT cardinality",
            spicelib::WNCARD(save.RESLT.as_slice(), ctx)?,
            b">",
            1,
            0,
            OK,
            ctx,
        )?;

        //
        // Test RESLT: we expect this window to be very close
        // to the same window as COMP.
        //
        for J in 1..=spicelib::WNCARD(save.RESLT.as_slice(), ctx)? {
            //        WRITE (*,*) 'RESLT Interval: ', J, LEFT, RIGHT,
            // .                  RIGHT-LEFT

            spicelib::WNFETD(
                save.RESLT.as_slice(),
                J,
                &mut save.LEFT,
                &mut save.RIGHT,
                ctx,
            )?;
            spicelib::WNFETD(
                save.COMP.as_slice(),
                J,
                &mut save.XLEFT,
                &mut save.XRIGHT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"RESLT Interval # start time");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);

            testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XLEFT, 0.000003, OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"RESLT Interval # stop time");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);

            testutil::CHCKSD(
                &save.TITLE,
                save.RIGHT,
                b"~",
                save.XRIGHT,
                0.000003,
                OK,
                ctx,
            )?;
        }

        //
        // We've completed tests for the current aberration
        // correction.
        //
    }
    //
    // We've completed tests of the > and < operators for
    // each aberration correction.
    //

    //
    // Check LOCMAX and ABSMAX, and ABSMAX with adjustment
    // constraints for each aberration correction.
    //

    for I in 1..=NCORR {
        //
        //---- Case -------------------------------------------------------------
        //

        //
        // Switch observer and target.
        //
        fstr::assign(&mut save.TARGET, b"EARTH");
        fstr::assign(&mut save.OBSRVR, b"MOON");

        fstr::assign(&mut save.ABCORR, save.CORR.get(I));

        save.STEP = spicelib::SPD();
        save.REFVAL = 404000.0;
        save.ADJUST = 0.0;

        fstr::assign(
            &mut save.TITLE,
            b"Find times when #-# distance is a local maximum. ABCORR = #.",
        );
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Search using the LOCMAX operator.
        //
        spicelib::GFDIST(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            b"LOCMAX",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFIN1.as_slice(),
            MAXWIN,
            NWDIST,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"RESULT cardinality",
            spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
            b">",
            2,
            0,
            OK,
            ctx,
        )?;

        save.ABSMAX = 0.0;
        save.IXMAX = 0;

        for J in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
            //
            // Fetch the endpoints of the current
            // interval. We expect them to match within 2*CNVTOL.
            //
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                J,
                &mut save.LEFT,
                &mut save.RIGHT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"Interval # left/right endpoint");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);

            testutil::CHCKSD(
                &save.TITLE,
                save.LEFT,
                b"~",
                save.RIGHT,
                ((2 as f64) * CNVTOL),
                OK,
                ctx,
            )?;

            //
            // Check the distance at TDELTA before and after
            // the interval's left endpoint.
            //

            spicelib::SPKPOS(
                &save.TARGET,
                save.LEFT,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.MAXDST = spicelib::VNORM(save.POS.as_slice());

            if (save.MAXDST > save.ABSMAX) {
                save.ABSMAX = save.MAXDST;
                save.IXMAX = J;
            }

            for K in 1..=2 {
                if (K == 1) {
                    save.T = (save.LEFT - TDELTA);
                    fstr::assign(&mut save.TITLE, b"Interval # start-# sec: distance");
                } else {
                    save.T = (save.LEFT + TDELTA);
                    fstr::assign(&mut save.TITLE, b"Interval # start+# sec: distance");
                }

                spicelib::SPKPOS(
                    &save.TARGET,
                    save.T,
                    b"J2000",
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.POS.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.DIST = spicelib::VNORM(save.POS.as_slice());

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                spicelib::REPMD(&save.TITLE.to_vec(), b"#", TDELTA, 6, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(&save.TITLE, save.DIST, b"<", save.MAXDST, 0.0, OK, ctx)?;
            }
            //
            // We've completed tests for the current interval
            // of RESULT.
            //
        }

        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Find times when #-# distance is an absolute maximum. ABCORR = #.",
        );

        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Now search for the absolute maximum distance.
        //
        spicelib::SSIZED(MAXWIN, save.RES2.as_slice_mut(), ctx)?;

        spicelib::GFDIST(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            b"ABSMAX",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFIN1.as_slice(),
            MAXWIN,
            NWDIST,
            save.WORK.as_slice_mut(),
            save.RES2.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect a result window containing just 1 interval.
        //
        testutil::CHCKSI(
            b"RES2 cardinality (0)",
            spicelib::WNCARD(save.RES2.as_slice(), ctx)?,
            b"=",
            1,
            0,
            OK,
            ctx,
        )?;

        //
        // Fetch the endpoints of the result window's single
        // interval. We expect them to match within 2*CNVTOL.
        //
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.TITLE, b"Interval 1 left/right endpoint");

        testutil::CHCKSD(
            &save.TITLE,
            save.LEFT,
            b"~",
            save.RIGHT,
            ((2 as f64) * CNVTOL),
            OK,
            ctx,
        )?;

        //
        // Check the distance and time of the maximum against those
        // obtained from the local maximum search.
        //
        save.T = save.RES2[1];

        spicelib::SPKPOS(
            &save.TARGET,
            save.T,
            b"J2000",
            &save.ABCORR,
            &save.OBSRVR,
            save.POS.as_slice_mut(),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.DIST = spicelib::VNORM(save.POS.as_slice());

        testutil::CHCKSD(
            b"Abs max distance",
            save.DIST,
            b"~/",
            save.ABSMAX,
            VTIGHT,
            OK,
            ctx,
        )?;

        save.XTIME = save.RESULT[((2 * (save.IXMAX - 1)) + 1)];

        testutil::CHCKSD(b"Abs max epoch", save.T, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        //        Now search for the absolute maximum distance, using a
        //        non-zero adjustment value.
        //
        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Find times when #-# distance is within # of an absolute maximum. ABCORR = #.",
        );

        save.ADJUST = 100.0;

        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.ADJUST,
            6,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::SCARDD(0, save.RES2.as_slice_mut(), ctx)?;

        spicelib::GFDIST(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            b"ABSMAX",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFIN1.as_slice(),
            MAXWIN,
            NWDIST,
            save.WORK.as_slice_mut(),
            save.RES2.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect a result window containing at least 1 interval.
        //
        testutil::CHCKSI(
            b"RES2 cardinality (1)",
            spicelib::WNCARD(save.RES2.as_slice(), ctx)?,
            b">",
            0,
            0,
            OK,
            ctx,
        )?;

        //
        // Check the distance the result window's endpoints
        // against that obtained from the absolute maximum search.
        //
        for J in 1..=spicelib::WNCARD(save.RES2.as_slice(), ctx)? {
            spicelib::WNFETD(
                save.RES2.as_slice(),
                J,
                &mut save.LEFT,
                &mut save.RIGHT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SPKPOS(
                &save.TARGET,
                save.LEFT,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DIST = spicelib::VNORM(save.POS.as_slice());

            testutil::CHCKSD(
                b"Adjusted ABSMAX distance, left",
                save.DIST,
                b"~",
                (save.ABSMAX - save.ADJUST),
                MEDIUM,
                OK,
                ctx,
            )?;

            spicelib::SPKPOS(
                &save.TARGET,
                save.RIGHT,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DIST = spicelib::VNORM(save.POS.as_slice());

            testutil::CHCKSD(
                b"Adjusted ABSMAX distance, right",
                save.DIST,
                b"~",
                (save.ABSMAX - save.ADJUST),
                MEDIUM,
                OK,
                ctx,
            )?;
        }

        //
        // We've completed tests for the current aberration
        // correction.
        //
    }

    //
    // Check LOCMIN and ABSMIN, and ABSMIN with adjustment
    // constraints for each aberration correction.
    //
    for I in 1..=NCORR {
        //
        //---- Case -------------------------------------------------------------
        //

        fstr::assign(&mut save.TARGET, b"EARTH");
        fstr::assign(&mut save.OBSRVR, b"MOON");

        fstr::assign(&mut save.ABCORR, save.CORR.get(I));

        save.STEP = spicelib::SPD();
        save.REFVAL = 374000.0;
        save.ADJUST = 0.0;

        fstr::assign(
            &mut save.TITLE,
            b"Find times when #-# distance is a local minimum. ABCORR = #.",
        );
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Search using the LOCMIN operator.
        //
        spicelib::GFDIST(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            b"LOCMIN",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFIN2.as_slice(),
            MAXWIN,
            NWDIST,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"RESULT cardinality",
            spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
            b">",
            2,
            0,
            OK,
            ctx,
        )?;

        save.ABSMIN = spicelib::DPMAX();
        save.IXMIN = 0;

        for J in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
            //
            // Fetch the endpoints of the current
            // interval. We expect them to match within 2*CNVTOL.
            //
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                J,
                &mut save.LEFT,
                &mut save.RIGHT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.TITLE, b"Interval # left/right endpoint");
            spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);

            testutil::CHCKSD(
                &save.TITLE,
                save.LEFT,
                b"~",
                save.RIGHT,
                ((2 as f64) * CNVTOL),
                OK,
                ctx,
            )?;

            //
            // Check the distance at TDELTA before and after
            // the interval's left endpoint.
            //

            spicelib::SPKPOS(
                &save.TARGET,
                save.LEFT,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.MINDST = spicelib::VNORM(save.POS.as_slice());

            if (save.MINDST < save.ABSMIN) {
                save.ABSMIN = save.MINDST;
                save.IXMIN = J;
            }

            for K in 1..=2 {
                if (K == 1) {
                    save.T = (save.LEFT - TDELTA);
                    fstr::assign(&mut save.TITLE, b"Interval # start-# sec: distance");
                } else {
                    save.T = (save.LEFT + TDELTA);
                    fstr::assign(&mut save.TITLE, b"Interval # start+# sec: distance");
                }

                spicelib::SPKPOS(
                    &save.TARGET,
                    save.T,
                    b"J2000",
                    &save.ABCORR,
                    &save.OBSRVR,
                    save.POS.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.DIST = spicelib::VNORM(save.POS.as_slice());

                spicelib::REPMI(&save.TITLE.to_vec(), b"#", J, &mut save.TITLE, ctx);
                spicelib::REPMD(&save.TITLE.to_vec(), b"#", TDELTA, 6, &mut save.TITLE, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(&save.TITLE, save.DIST, b">", save.MINDST, 0.0, OK, ctx)?;
            }
            //
            // We've completed tests for the current interval
            // of RESULT.
            //
        }

        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Find times when #-# distance is an absolute minimum. ABCORR = #.",
        );

        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Now search for the absolute minimum distance.
        //
        spicelib::SSIZED(MAXWIN, save.RES2.as_slice_mut(), ctx)?;

        spicelib::GFDIST(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            b"ABSMIN",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFIN2.as_slice(),
            MAXWIN,
            NWDIST,
            save.WORK.as_slice_mut(),
            save.RES2.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect a result window containing just 1 interval.
        //
        testutil::CHCKSI(
            b"RES2 cardinality (2)",
            spicelib::WNCARD(save.RES2.as_slice(), ctx)?,
            b"=",
            1,
            0,
            OK,
            ctx,
        )?;

        //
        // Fetch the endpoints of the result window's single
        // interval. We expect them to match within 2*CNVTOL.
        //
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            1,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.TITLE, b"Interval 1 left/right endpoint");

        testutil::CHCKSD(
            &save.TITLE,
            save.LEFT,
            b"~",
            save.RIGHT,
            ((2 as f64) * CNVTOL),
            OK,
            ctx,
        )?;

        //
        // Check the distance and time of the minimum against those
        // obtained from the local minimum search.
        //
        save.T = save.RES2[1];

        spicelib::SPKPOS(
            &save.TARGET,
            save.T,
            b"J2000",
            &save.ABCORR,
            &save.OBSRVR,
            save.POS.as_slice_mut(),
            &mut save.LT,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.DIST = spicelib::VNORM(save.POS.as_slice());

        testutil::CHCKSD(
            b"Abs min distance",
            save.DIST,
            b"~/",
            save.ABSMIN,
            VTIGHT,
            OK,
            ctx,
        )?;

        save.XTIME = save.RESULT[((2 * (save.IXMIN - 1)) + 1)];

        testutil::CHCKSD(b"Abs min epoch", save.T, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        //        Now search for the absolute minimum distance, using a
        //        non-zero adjustment value.
        //
        //
        //---- Case -------------------------------------------------------------
        //
        fstr::assign(
            &mut save.TITLE,
            b"Find times when #-# distance is within # of an absolute minimum. ABCORR = #.",
        );

        save.ADJUST = 100.0;

        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.OBSRVR, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.ADJUST,
            6,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);

        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::SCARDD(0, save.RES2.as_slice_mut(), ctx)?;

        spicelib::GFDIST(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            b"ABSMIN",
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFIN2.as_slice(),
            MAXWIN,
            NWDIST,
            save.WORK.as_slice_mut(),
            save.RES2.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We expect a result window containing at least 1 interval.
        //
        testutil::CHCKSI(
            b"RES2 cardinality (3)",
            spicelib::WNCARD(save.RES2.as_slice(), ctx)?,
            b">",
            0,
            0,
            OK,
            ctx,
        )?;

        //
        // Check the distance the result window's endpoints
        // against that obtained from the absolute minimum search.
        //
        for J in 1..=spicelib::WNCARD(save.RES2.as_slice(), ctx)? {
            spicelib::WNFETD(
                save.RES2.as_slice(),
                J,
                &mut save.LEFT,
                &mut save.RIGHT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::SPKPOS(
                &save.TARGET,
                save.LEFT,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DIST = spicelib::VNORM(save.POS.as_slice());

            testutil::CHCKSD(
                b"Adjusted ABSMIN distance, left",
                save.DIST,
                b"~",
                (save.ABSMIN + save.ADJUST),
                MEDIUM,
                OK,
                ctx,
            )?;

            spicelib::SPKPOS(
                &save.TARGET,
                save.RIGHT,
                b"J2000",
                &save.ABCORR,
                &save.OBSRVR,
                save.POS.as_slice_mut(),
                &mut save.LT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            save.DIST = spicelib::VNORM(save.POS.as_slice());

            testutil::CHCKSD(
                b"Adjusted ABSMIN distance, right",
                save.DIST,
                b"~",
                (save.ABSMIN + save.ADJUST),
                MEDIUM,
                OK,
                ctx,
            )?;
        }

        //
        // We've completed tests for the current aberration
        // correction.
        //
    }

    //
    // Case
    //
    fstr::assign(&mut save.TITLE, b"Check the GF call uses the GFSTOL value");
    testutil::TCASE(&save.TITLE, ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.ABCORR, b"NONE");
    save.STEP = spicelib::SPD();
    save.REFVAL = 374000.0;
    save.ADJUST = 0.0;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFDIST(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        b"ABSMIN",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
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

    spicelib::GFDIST(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        b"ABSMIN",
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWDIST,
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
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
