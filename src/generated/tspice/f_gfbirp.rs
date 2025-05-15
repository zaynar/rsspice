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
const NPCK: &[u8] = b"nat.tpc";
const NSPK: &[u8] = b"nat.bsp";
const NIK: &[u8] = b"nat.ti";
const TIGHT: f64 = 0.000000000001;
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const LNSIZE: i32 = 80;
const LBCELL: i32 = -5;
const MAXWIN: i32 = 100;
const MXPASS: i32 = 20;
const MXCALL: i32 = 20000;
const MAXMSG: i32 = MXMSG;

struct SaveVars {
    ABCORR: Vec<u8>,
    INST: Vec<u8>,
    MSGLOG: ActualCharArray2D,
    QNAME: Vec<u8>,
    OBSRVR: Vec<u8>,
    TARGET: Vec<u8>,
    TFRAME: Vec<u8>,
    TSHAPE: Vec<u8>,
    CNFINE: StackArray<f64, 206>,
    CNFLOG: ActualArray2D<f64>,
    ET0: f64,
    ET1: f64,
    MEASUR: f64,
    RAYDIR: StackArray<f64, 3>,
    REPLOG: ActualArray2D<f64>,
    RESULT: StackArray<f64, 206>,
    STEP: f64,
    TOL: f64,
    XMESUR: f64,
    HAN2: i32,
    MW: i32,
    NCALLS: i32,
    NIVL: i32,
    NUPDAT: i32,
    SEQLOG: ActualArray<i32>,
    TRMLOG: ActualArray<i32>,
    XSQLOG: ActualArray<i32>,
    BAIL: bool,
    RPT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; CORLEN as usize];
        let mut INST = vec![b' '; BDNMLN as usize];
        let mut MSGLOG = ActualCharArray2D::new(MAXMSG, 1..=2, 1..=MXPASS);
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut TFRAME = vec![b' '; FRNMLN as usize];
        let mut TSHAPE = vec![b' '; SHPLEN as usize];
        let mut CNFINE = StackArray::<f64, 206>::new(LBCELL..=(2 * MAXWIN));
        let mut CNFLOG = ActualArray2D::<f64>::new(LBCELL..=(2 * MAXWIN), 1..=MXPASS);
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut MEASUR: f64 = 0.0;
        let mut RAYDIR = StackArray::<f64, 3>::new(1..=3);
        let mut REPLOG = ActualArray2D::<f64>::new(1..=3, 1..=MXCALL);
        let mut RESULT = StackArray::<f64, 206>::new(LBCELL..=(2 * MAXWIN));
        let mut STEP: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut XMESUR: f64 = 0.0;
        let mut HAN2: i32 = 0;
        let mut MW: i32 = 0;
        let mut NCALLS: i32 = 0;
        let mut NIVL: i32 = 0;
        let mut NUPDAT: i32 = 0;
        let mut SEQLOG = ActualArray::<i32>::new(1..=MXCALL);
        let mut TRMLOG = ActualArray::<i32>::new(1..=MXCALL);
        let mut XSQLOG = ActualArray::<i32>::new(1..=MXCALL);
        let mut BAIL: bool = false;
        let mut RPT: bool = false;

        Self {
            ABCORR,
            INST,
            MSGLOG,
            QNAME,
            OBSRVR,
            TARGET,
            TFRAME,
            TSHAPE,
            CNFINE,
            CNFLOG,
            ET0,
            ET1,
            MEASUR,
            RAYDIR,
            REPLOG,
            RESULT,
            STEP,
            TOL,
            XMESUR,
            HAN2,
            MW,
            NCALLS,
            NIVL,
            NUPDAT,
            SEQLOG,
            TRMLOG,
            XSQLOG,
            BAIL,
            RPT,
        }
    }
}

//$Procedure      F_GFBIRP ( Test GF binary quantity progress reporting )
pub fn F_GFBIRP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Maximum progress report prefix or suffix length.
    // MXMSG is declared in zzgf.inc.
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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFBIRP", ctx)?;

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
    // Create and load Nat's solar system SPK, PCK/FK, and IK
    // files.
    //
    testutil::NATPCK(NPCK, true, true, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATSPK(NSPK, true, &mut save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::NATIK(NIK, NSPK, NPCK, true, false, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //*********************************************************************
    //*
    //*    Binary state search tests
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Run simple occultation search using GFOCCE.", ctx)?;

    //
    // Search for any occultation of ALPHA by BETA, abcorr = NONE.
    //

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the confinement window log.
    //
    for I in 1..=MXPASS {
        spicelib::SSIZED((2 * MAXWIN), save.CNFLOG.subarray_mut([LBCELL, I]), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Note: the stellar aberration correction spec should be ignored.
    //
    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    //
    // Create a confinement window with 5 intervals.
    //
    save.NIVL = 5;
    save.XMESUR = 0.0;

    for I in 1..=save.NIVL {
        save.ET0 = (-900.0 + (((I - 3) as f64) * 3600.0));
        save.ET1 = (save.ET0 + 1800.0);

        spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.XMESUR = ((save.XMESUR + save.ET1) - save.ET0);
    }

    save.BAIL = false;
    save.RPT = true;

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
        b"NONE",
        b"sun",
        save.TOL,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now that the search is done, the interesting work starts. We're
    // going to do a post-mortem on the progress reporting calls that
    // were made during the search.
    //
    // Start out by fetching the information that was passed to T_GFREPI
    // during the search.
    //
    save.MW = (2 * MAXWIN);
    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NCALLS,
        save.SEQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Occultation searches require one pass, so we expect that
    // just one call to T_GFREPI was made.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NCALLS, b"=", 1, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.SEQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefix and suffix.
    //
    testutil::CHCKSC(
        b"Prefix",
        &save.MSGLOG[[1, 1]],
        b"=",
        b"Occultation/transit search",
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, 1]], b"=", b"done.", OK, ctx)?;

    //
    // Retrieve the log of calls made to the update routine.
    //
    testutil::T_GFRPLO(
        MXCALL,
        &mut save.NUPDAT,
        save.SEQLOG.as_slice_mut(),
        save.REPLOG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the sequence numbers of the calls. They should
    // range from 2 to NUPDAT+1 and should be in increasing
    // order.
    //
    for I in 1..=save.NUPDAT {
        save.XSQLOG[I] = (I + 1);
    }

    testutil::CHCKAI(
        b"Update SEQLOG",
        save.SEQLOG.as_slice(),
        b"=",
        save.XSQLOG.as_slice(),
        save.NUPDAT,
        OK,
        ctx,
    )?;
    //
    // Make sure that:
    //
    //   - Each update time lies within the corresponding interval.
    //
    //   - Within each interval, the update times are monotonically
    //     non-decreasing.
    //
    // Also record the measure of the reported progress.
    //
    save.MEASUR = 0.0;

    for I in 1..=save.NUPDAT {
        fstr::assign(&mut save.QNAME, b"(0) Update time no. *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(
            &save.QNAME,
            save.REPLOG[[3, I]],
            b">=",
            save.REPLOG[[1, I]],
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(
            &save.QNAME,
            save.REPLOG[[3, I]],
            b"<=",
            save.REPLOG[[2, I]],
            0.0,
            OK,
            ctx,
        )?;

        if (I > 1) {
            if (save.REPLOG[[1, I]] == save.REPLOG[[1, (I - 1)]]) {
                //
                // The current interval is the same as the previous one.
                //
                fstr::assign(&mut save.QNAME, b"(1) Update time no. *");
                spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.REPLOG[[3, I]],
                    b">=",
                    save.REPLOG[[3, (I - 1)]],
                    0.0,
                    OK,
                    ctx,
                )?;

                save.MEASUR = ((save.MEASUR + save.REPLOG[[3, I]]) - save.REPLOG[[3, (I - 1)]]);
            }
        }
    }

    //
    // Compare the measure of the reported progress to that of the
    // confinement window.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.XMESUR, TIGHT, OK, ctx)?;

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Occultation searches require one pass, so we expect that
    // just one call to T_GFRTRM was made.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 1, 0, OK, ctx)?;

    //
    // Check the sequence number of the first termination call. Take
    // into account that the first call was to the init routine.
    //
    testutil::CHCKSI(
        b"T_GFREPF seq",
        save.TRMLOG[1],
        b"=",
        (save.NUPDAT + 2),
        0,
        OK,
        ctx,
    )?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Run simple FOV search using GFFOVE.", ctx)?;

    //
    // We'll use the instrument defined in nat.ti:
    //
    //    ALPHA_ELLIPSE_NONE
    //
    // This instrument tracks body Alpha so body Beta's FOV entry and
    // exit times match the start and stop times of Beta's transit
    // across Alpha.
    //
    // Initialize inputs for the search.
    //
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the confinement window log.
    //
    for I in 1..=MXPASS {
        spicelib::SSIZED((2 * MAXWIN), save.CNFLOG.subarray_mut([LBCELL, I]), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    spicelib::SSIZED((2 * MAXWIN), save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED((2 * MAXWIN), save.RESULT.as_slice_mut(), ctx)?;

    //
    // Create a confinement window with 3 intervals.
    //
    save.NIVL = 3;
    save.XMESUR = 0.0;

    for I in 1..=save.NIVL {
        save.ET0 = (-800.0 + (((I - 3) as f64) * 3600.0));
        save.ET1 = (save.ET0 + 1600.0);

        spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.XMESUR = ((save.XMESUR + save.ET1) - save.ET0);
    }

    fstr::assign(&mut save.INST, b"ALPHA_ELLIPSE_NONE");
    fstr::assign(&mut save.TARGET, b"BETA");
    fstr::assign(&mut save.TSHAPE, b"ELLIPSOID");
    fstr::assign(&mut save.TFRAME, b"BETAFIXED");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.STEP = 300.0;
    save.TOL = CNVTOL;
    fstr::assign(&mut save.ABCORR, b"NONE");

    spicelib::CLEARD(3, save.RAYDIR.as_slice_mut());

    save.RPT = true;
    save.BAIL = false;

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
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.BAIL,
        spicelib::GFBAIL,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Now that the search is done, the interesting work starts. We're
    // going to do a post-mortem on the progress reporting calls that
    // were made during the search.
    //
    // Start out by fetching the information that was passed to T_GFREPI
    // during the search.
    //
    save.MW = (2 * MAXWIN);
    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NCALLS,
        save.SEQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // FOV searches require one pass.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NCALLS, b"=", 1, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.SEQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefix and suffix.
    //
    testutil::CHCKSC(
        b"Prefix",
        &save.MSGLOG[[1, 1]],
        b"=",
        b"Target visibility search",
        OK,
        ctx,
    )?;

    testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, 1]], b"=", b"done.", OK, ctx)?;

    //
    // Retrieve the log of calls made to the update routine.
    //
    testutil::T_GFRPLO(
        MXCALL,
        &mut save.NUPDAT,
        save.SEQLOG.as_slice_mut(),
        save.REPLOG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the sequence numbers of the calls. They should
    // range from 2 to NUPDAT+1 and should be in increasing
    // order.
    //
    for I in 1..=save.NUPDAT {
        save.XSQLOG[I] = (I + 1);
    }

    testutil::CHCKAI(
        b"Update SEQLOG",
        save.SEQLOG.as_slice(),
        b"=",
        save.XSQLOG.as_slice(),
        save.NUPDAT,
        OK,
        ctx,
    )?;
    //
    // Make sure that:
    //
    //   - Each update time lies within the corresponding interval.
    //
    //   - Within each interval, the update times are monotonically
    //     non-decreasing.
    //
    // Also record the measure of the reported progress.
    //
    save.MEASUR = 0.0;

    for I in 1..=save.NUPDAT {
        fstr::assign(&mut save.QNAME, b"(0) Update time no. *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(
            &save.QNAME,
            save.REPLOG[[3, I]],
            b">=",
            save.REPLOG[[1, I]],
            0.0,
            OK,
            ctx,
        )?;
        testutil::CHCKSD(
            &save.QNAME,
            save.REPLOG[[3, I]],
            b"<=",
            save.REPLOG[[2, I]],
            0.0,
            OK,
            ctx,
        )?;

        if (I > 1) {
            if (save.REPLOG[[1, I]] == save.REPLOG[[1, (I - 1)]]) {
                //
                // The current interval is the same as the previous one.
                //
                fstr::assign(&mut save.QNAME, b"(1) Update time no. *");
                spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(
                    &save.QNAME,
                    save.REPLOG[[3, I]],
                    b">=",
                    save.REPLOG[[3, (I - 1)]],
                    0.0,
                    OK,
                    ctx,
                )?;

                save.MEASUR = ((save.MEASUR + save.REPLOG[[3, I]]) - save.REPLOG[[3, (I - 1)]]);
            }
        }
    }

    //
    // Compare the measure of the reported progress to that of the
    // confinement window.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.XMESUR, TIGHT, OK, ctx)?;

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // FOV searches require one pass, so we expect that
    // just one call to T_GFRTRM was made.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 1, 0, OK, ctx)?;

    //
    // Check the sequence number of the first termination call. Take
    // into account that the first call was to the init routine.
    //
    testutil::CHCKSI(
        b"T_GFREPF seq",
        save.TRMLOG[1],
        b"=",
        (save.NUPDAT + 2),
        0,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(save.HAN2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(NSPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
