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
const RECSYS: &[u8] = b"RECTANGULAR";
const LATSYS: &[u8] = b"LATITUDINAL";
const SPHSYS: &[u8] = b"SPHERICAL";
const RADSYS: &[u8] = b"RA/DEC";
const CYLSYS: &[u8] = b"CYLINDRICAL";
const GEOSYS: &[u8] = b"GEODETIC";
const PGRSYS: &[u8] = b"PLANETOGRAPHIC";
const XCRD: &[u8] = b"X";
const YCRD: &[u8] = b"Y";
const ZCRD: &[u8] = b"Z";
const RADCRD: &[u8] = b"RADIUS";
const LONCRD: &[u8] = b"LONGITUDE";
const LATCRD: &[u8] = b"LATITUDE";
const RACRD: &[u8] = b"RIGHT ASCENSION";
const DECCRD: &[u8] = b"DECLINATION";
const RNGCRD: &[u8] = b"RANGE";
const CLTCRD: &[u8] = b"COLATITUDE";
const ALTCRD: &[u8] = b"ALTITUDE";
const POSDEF: &[u8] = b"POSITION";
const SOBDEF: &[u8] = b"SUB-OBSERVER POINT";
const SINDEF: &[u8] = b"SURFACE INTERCEPT POINT";
const NWREL: i32 = 5;
const NWLONG: i32 = 7;
const EXWIDX: i32 = ((NWREL + NWLONG) + 1);
const MXBEGM: i32 = 55;
const MXENDM: i32 = 13;
const MXMSG: i32 = ((MXBEGM + MXENDM) + 10);
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const PCK: &[u8] = b"zzgffvu.tpc";
const NPCK: &[u8] = b"nat.tpc";
const NSPK: &[u8] = b"nat.bsp";
const NIK: &[u8] = b"nat.ti";
const SPK: &[u8] = b"zzgffvu.bsp";
const TIMTOL: f64 = ((2 as f64) * CNVTOL);
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 200;
const FRNMLN: i32 = 32;
const MAXWIN: i32 = 20000;
const LBCELL: i32 = -5;
const LBLSIZ: i32 = 40;

struct SaveVars {
    ABCORR: Vec<u8>,
    INST: Vec<u8>,
    OBSRVR: Vec<u8>,
    QNAME: Vec<u8>,
    TARGET: Vec<u8>,
    TFRAME: Vec<u8>,
    TITLE: Vec<u8>,
    TSHAPE: Vec<u8>,
    CNFINE: ActualArray<f64>,
    ENDPT: StackArray<f64, 2>,
    RAYDIR: StackArray<f64, 3>,
    RESULT: ActualArray<f64>,
    STEP: f64,
    TOL: f64,
    XTIME: f64,
    HAN2: i32,
    HANDLE: i32,
    N: i32,
    BAIL: bool,
    RPT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut INST = vec![b' '; BDNMLN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut QNAME = vec![b' '; LBLSIZ as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TFRAME = vec![b' '; FRNMLN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut TSHAPE = vec![b' '; SHPLEN as usize];
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut ENDPT = StackArray::<f64, 2>::new(1..=2);
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut STEP: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XTIME: f64 = 0.0;
        let mut HAN2: i32 = 0;
        let mut HANDLE: i32 = 0;
        let mut N: i32 = 0;
        let mut BAIL: bool = false;
        let mut RPT: bool = false;

        Self {
            ABCORR,
            INST,
            OBSRVR,
            QNAME,
            TARGET,
            TFRAME,
            TITLE,
            TSHAPE,
            CNFINE,
            ENDPT,
            RAYDIR,
            RESULT,
            STEP,
            TOL,
            XTIME,
            HAN2,
            HANDLE,
            N,
            BAIL,
            RPT,
        }
    }
}

//$Procedure      F_GFFOVE ( Test GFFOVE )
pub fn F_GFFOVE(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Saved Variables
    //

    //
    // Initial values
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFFOVE", ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Setup: create and load SPK, PCK, LSK files.", ctx)?;

    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load a PCK.
    //
    testutil::TSTPCK(PCK, true, false, ctx)?;

    //
    // Load an SPK file as well.
    //
    testutil::TSTSPK(SPK, true, &mut save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create and load Nat's solar system SPK, PCK/FK, and IK
    // files.
    //
    testutil::NATPCK(NPCK, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATSPK(NSPK, true, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATIK(NIK, NSPK, NPCK, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    //
    // Actual DE-based ephemerides yield better comparisons,
    // since these ephemerides have less noise than do those
    // produced by TSTSPK.
    //
    //  CALL FURNSH ( 'de421.bsp' )
    //  CALL FURNSH ( 'jup230.bsp' )
    //

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // We exercise only those error checks that are actually
    // performed in GFOCCE.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Result window too small", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(1, save.RESULT.as_slice_mut(), ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 300.0;
    save.TOL = CNVTOL;

    save.RPT = false;
    save.BAIL = false;

    spicelib::GFSSTP(save.STEP, ctx)?;

    spicelib::GFFOVE(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    testutil::CHCKXC(true, b"SPICE(WINDOWTOOSMALL)", OK, ctx)?;

    //
    // Restore window sizes.
    //
    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bad tolerance value", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");

    spicelib::VPACK(0.0, 0.0, 1.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.TARGET, b"BET");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 0.0;
    save.TOL = 0.0;

    save.RPT = false;
    save.BAIL = false;

    spicelib::GFFOVE(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    // We'll start out with cases using an ellipsoidal target and
    // geometric states. We'll use the instrument defined
    // in nat.ti:
    //
    //    ALPHA_ELLIPSE_NONE
    //
    // This instrument track's body Alpha so body Beta's FOV entry and
    // exit times match the start and stop times of Beta's transit
    // across Alpha.
    //
    // Initialize inputs for the search.
    //
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    //
    // Use a 3-day span that covers 3 events.
    //
    spicelib::WNINSD(
        -(0.5 * spicelib::SPD()),
        (2.5 * spicelib::SPD()),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 300.0;
    save.TOL = CNVTOL;
    fstr::assign(&mut save.ABCORR, b"NONE");

    save.RPT = false;
    save.BAIL = false;

    //
    // --- Case: ------------------------------------------------------
    //
    //

    //
    // Set up the TCASE call.
    //
    fstr::assign(
        &mut save.TITLE,
        b"Target #; target shape #; Inst. #; #; target frame #",
    );

    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TSHAPE, &mut save.TITLE);
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.INST, &mut save.TITLE);
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TFRAME, &mut save.TITLE);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    spicelib::GFFOVE(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    //
    // We're expecting 3 events.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 3, 0, OK, ctx)?;

    if (save.N > 0) {
        for I in 1..=save.N {
            let [arg2, arg3] = save
                .ENDPT
                .get_disjoint_mut([1, 2])
                .expect("mutable array elements passed to function must have disjoint indexes");
            spicelib::WNFETD(save.RESULT.as_slice(), I, arg2, arg3, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check the interval endpoints. The event should
            // start at noon and stop 10 minutes later.

            fstr::assign(&mut save.QNAME, b"Start time @");
            spicelib::REPMI(&save.QNAME.to_vec(), b"@", I, &mut save.QNAME, ctx);

            save.XTIME = (((I - 1) as f64) * spicelib::SPD());

            testutil::CHCKSD(
                &save.QNAME,
                save.ENDPT[1],
                b"~",
                save.XTIME,
                TIMTOL,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.QNAME, b"Stop time @");
            spicelib::REPMI(&save.QNAME.to_vec(), b"@", I, &mut save.QNAME, ctx);

            save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + 600.0);

            testutil::CHCKSD(
                &save.QNAME,
                save.ENDPT[2],
                b"~",
                save.XTIME,
                TIMTOL,
                OK,
                ctx,
            )?;
        }
        //
        // End of result interval check loop.
        //
    }
    //
    // End of non-empty result window block.
    //

    //
    // The second case uses a ray as the target. The ray will
    // point along the +X axis of the BETA_VIEW_XY frame, which
    // tracks body BETA.
    //
    //
    // Initialize inputs for the search.
    //
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    //
    // Use a 3-day span that covers 3 events.
    //
    spicelib::WNINSD(
        -(0.5 * spicelib::SPD()),
        (2.5 * spicelib::SPD()),
        save.CNFINE.as_slice_mut(),
        ctx,
    )?;

    fstr::assign(&mut save.INST, b"ALPHA_RECTANGLE_NONE");
    fstr::assign(&mut save.TARGET, b"<none>");
    fstr::assign(&mut save.TSHAPE, b"RAY");
    fstr::assign(&mut save.TFRAME, b"BETA_VIEW_XY");

    spicelib::VPACK(1.0, 0.0, 0.0, save.RAYDIR.as_slice_mut());

    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 300.0;
    save.TOL = CNVTOL;
    fstr::assign(&mut save.ABCORR, b"NONE");

    save.RPT = false;
    save.BAIL = false;

    //
    // --- Case: ------------------------------------------------------
    //
    //

    //
    // Set up the TCASE call.
    //
    fstr::assign(
        &mut save.TITLE,
        b"Target #; target shape #; Inst. #; #; target frame #",
    );

    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TARGET, &mut save.TITLE);
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TSHAPE, &mut save.TITLE);
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.INST, &mut save.TITLE);
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
    spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.TFRAME, &mut save.TITLE);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    fstr::assign(&mut save.TITLE, b"Target #; target shape #; Inst. #; #; ");

    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Perform the search.
    //
    spicelib::GFFOVE(
        &save.INST,
        &save.TSHAPE,
        save.RAYDIR.as_slice(),
        &save.TARGET,
        &save.TFRAME,
        &save.ABCORR,
        &save.OBSRVR,
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

    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    //
    // We're expecting 3 events.
    //
    testutil::CHCKSI(b"N", save.N, b"=", 3, 0, OK, ctx)?;

    if (save.N > 0) {
        for I in 1..=save.N {
            let [arg2, arg3] = save
                .ENDPT
                .get_disjoint_mut([1, 2])
                .expect("mutable array elements passed to function must have disjoint indexes");
            spicelib::WNFETD(save.RESULT.as_slice(), I, arg2, arg3, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Check the interval endpoints. The event should start at one
            // minute past noon and stop 8 minutes later.
            //
            fstr::assign(&mut save.QNAME, b"Start time @");
            spicelib::REPMI(&save.QNAME.to_vec(), b"@", I, &mut save.QNAME, ctx);

            save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + 60.0);

            testutil::CHCKSD(
                &save.QNAME,
                save.ENDPT[1],
                b"~",
                save.XTIME,
                TIMTOL,
                OK,
                ctx,
            )?;

            fstr::assign(&mut save.QNAME, b"Stop time @");
            spicelib::REPMI(&save.QNAME.to_vec(), b"@", I, &mut save.QNAME, ctx);

            save.XTIME = ((((I - 1) as f64) * spicelib::SPD()) + 540.0);

            testutil::CHCKSD(
                &save.QNAME,
                save.ENDPT[2],
                b"~",
                save.XTIME,
                TIMTOL,
                OK,
                ctx,
            )?;
        }
        //
        // End of result interval check loop.
        //
    }
    //
    // End of non-empty result window block.
    //

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(NSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
