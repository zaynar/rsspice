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
const PCK: &[u8] = b"nat.tpc";
const PCK2: &[u8] = b"generic.tpc";
const SPK1: &[u8] = b"nat.bsp";
const SPK2: &[u8] = b"generic.bsp";
const TIMTOL: f64 = CNVTOL;
const LNSIZE: i32 = 80;
const TIMLEN: i32 = 50;
const LBCELL: i32 = -5;
const MAXWIN: i32 = 100;

struct SaveVars {
    TITLE: Vec<u8>,
    TIMSTR: Vec<u8>,
    UTC0: Vec<u8>,
    UTC1: Vec<u8>,
    CNFINE: StackArray<f64, 206>,
    ET0: f64,
    ET1: f64,
    LEFT: f64,
    RESULT: StackArray<f64, 206>,
    RIGHT: f64,
    STEP: f64,
    TOL: f64,
    XTIME: f64,
    HAN1: i32,
    HAN2: i32,
    BAIL: bool,
    RPT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TIMSTR = vec![b' '; TIMLEN as usize];
        let mut UTC0 = vec![b' '; TIMLEN as usize];
        let mut UTC1 = vec![b' '; TIMLEN as usize];
        let mut CNFINE = StackArray::<f64, 206>::new(LBCELL..=(2 * MAXWIN));
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut LEFT: f64 = 0.0;
        let mut RESULT = StackArray::<f64, 206>::new(LBCELL..=(2 * MAXWIN));
        let mut RIGHT: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XTIME: f64 = 0.0;
        let mut HAN1: i32 = 0;
        let mut HAN2: i32 = 0;
        let mut BAIL: bool = false;
        let mut RPT: bool = false;

        Self {
            TITLE,
            TIMSTR,
            UTC0,
            UTC1,
            CNFINE,
            ET0,
            ET1,
            LEFT,
            RESULT,
            RIGHT,
            STEP,
            TOL,
            XTIME,
            HAN1,
            HAN2,
            BAIL,
            RPT,
        }
    }
}

//$Procedure      F_GFOCCE ( GFOCCE family tests )
pub fn F_GFOCCE(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // EXTERNAL declarations
    //
    //
    // SPICELIB default functions for
    //
    //    - Interrupt handling (no-op function):   GFBAIL
    //    - Search refinement:                     GFREFN
    //    - Progress report termination:           GFREPF
    //    - Progress report initialization:        GFREPI
    //    - Progress report update:                GFREPU
    //    - Search step size "get" function:       GFSTEP
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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFOCCE", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    testutil::NATSPK(SPK1, true, &mut save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTSPK(SPK2, true, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATPCK(PCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTPCK(PCK2, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Set up a confinement window. Initialize this and
    // the result window.
    //
    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    //
    // Create a confinement window with 4 intervals.
    //
    for I in 1..=4 {
        save.LEFT = ((save.ET0 + (((I - 1) as f64) * spicelib::SPD())) + 3600.0);
        save.RIGHT = (save.LEFT + (3600.0 * 22 as f64));

        spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    }

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Result window too small (detected before search)", ctx)?;

    spicelib::SSIZED(1, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 5 TDB", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BAIL = false;
    save.RPT = false;

    save.STEP = 300.0;
    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = CNVTOL;

    spicelib::GFOCCE(
        b"ANY",
        b"MOON",
        b"ellipsoid",
        b"IAU_MOON",
        b"SUN",
        b"ellipsoid",
        b"IAU_SUN",
        b"LT",
        b"EARTH",
        CNVTOL,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WINDOWTOOSMALL)", OK, ctx)?;

    //
    // Restore original result window size.
    //
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad convergence tolerance.", ctx)?;

    spicelib::STR2ET(b"2000 JAN 1 TDB", &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(b"2000 JAN 5 TDB", &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BAIL = false;
    save.RPT = false;

    save.STEP = 300.0;
    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = 0.0;

    spicelib::GFOCCE(
        b"ANY",
        b"MOON",
        b"ellipsoid",
        b"IAU_MOON",
        b"SUN",
        b"ellipsoid",
        b"IAU_SUN",
        b"LT",
        b"EARTH",
        save.TOL,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDTOLERANCE)", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized shape for front target", ctx)?;

    save.TOL = CNVTOL;

    spicelib::GFOCCE(
        b"ANY",
        b"MOON",
        b"ray",
        b"IAU_MOON",
        b"SUN",
        b"ellipsoid",
        b"IAU_SUN",
        b"LT",
        b"EARTH",
        save.TOL,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Unrecognized shape for back target", ctx)?;

    save.TOL = CNVTOL;

    spicelib::GFOCCE(
        b"ANY",
        b"MOON",
        b"point",
        b"IAU_MOON",
        b"SUN",
        b"ray",
        b"IAU_SUN",
        b"LT",
        b"EARTH",
        save.TOL,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(BADMETHODSYNTAX)", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Any occultation of ALPHA by BETA, abcorr = LT+S", ctx)?;

    //
    // Note: the stellar aberration correction spec should be ignored.
    //
    fstr::assign(&mut save.UTC0, b"2000 JAN 1  00:00:00 TDB");
    fstr::assign(&mut save.UTC1, b"2000 JAN 05 00:00:00 TDB");

    spicelib::STR2ET(&save.UTC0, &mut save.ET0, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::STR2ET(&save.UTC1, &mut save.ET1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.BAIL = false;
    save.RPT = false;

    save.STEP = 300.0;
    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.TOL = CNVTOL;

    spicelib::GFOCCE(
        b"ANY",
        b"beta",
        b"ellipsoid",
        b"betafixed",
        b"alpha",
        b"ellipsoid",
        b"alphafixed",
        b"LT+S",
        b"sun",
        save.TOL,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        spicelib::GFREPI,
        spicelib::GFREPU,
        spicelib::GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(
        b"RESULT cardinality",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        4,
        0,
        OK,
        ctx,
    )?;

    for I in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
        spicelib::WNFETD(
            save.RESULT.as_slice(),
            I,
            &mut save.LEFT,
            &mut save.RIGHT,
            ctx,
        )?;

        fstr::assign(&mut save.TITLE, b"Occ # beg time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        //
        // Check occultation start time.
        //
        save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.LEFT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        //
        // Check occultation end time.
        //
        fstr::assign(&mut save.TITLE, b"Occ # end time");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);

        save.XTIME = (((((I - 1) as f64) * spicelib::SPD()) + ((10 as f64) * 60.0)) + 1.0);
        testutil::CHCKSD(&save.TITLE, save.RIGHT, b"~", save.XTIME, TIMTOL, OK, ctx)?;

        spicelib::TIMOUT(
            save.LEFT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
        spicelib::TIMOUT(
            save.RIGHT,
            b"YYYY MON DD HR:MN:SC.###### ::TDB (TDB)",
            &mut save.TIMSTR,
            ctx,
        )?;
    }

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::TCASE(b"Clean up. Unload and delete kernels.", ctx)?;

    //
    // Clean up SPK files.
    //
    spicelib::SPKUEF(save.HAN1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
