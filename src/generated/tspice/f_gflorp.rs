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
const PCK: &[u8] = b"gflorp.tpc";
const SPK: &[u8] = b"gflorp.bsp";
const TIGHT: f64 = 0.000001;
const LNSIZE: i32 = 80;
const LBCELL: i32 = -5;
const MAXWIN: i32 = 200;
const MXPASS: i32 = 20;
const MXCALL: i32 = 20000;
const MAXMSG: i32 = MXMSG;
const MAXPAR: i32 = 20;

struct SaveVars {
    GQUANT: Vec<u8>,
    MSGLOG: ActualCharArray2D,
    OP: Vec<u8>,
    QNAME: Vec<u8>,
    QCPARS: ActualCharArray,
    QPNAMS: ActualCharArray,
    XPREFX: Vec<u8>,
    ADJUST: f64,
    CENTRL: f64,
    CNFINE: StackArray<f64, 206>,
    CNFLOG: ActualArray2D<f64>,
    CNFMES: f64,
    ET0: f64,
    ET1: f64,
    MEASUR: f64,
    QDPARS: StackArray<f64, 20>,
    REFVAL: f64,
    REPLOG: ActualArray2D<f64>,
    RESULT: StackArray<f64, 206>,
    STEP: f64,
    TOL: f64,
    WORK: ActualArray2D<f64>,
    HANDLE: i32,
    ISQLOG: StackArray<i32, 20>,
    MW: i32,
    NCALLS: i32,
    NIVL: i32,
    NPASS: i32,
    NUPDAT: i32,
    NW: i32,
    QIPARS: StackArray<i32, 20>,
    QNPARS: i32,
    SEQLOG: ActualArray<i32>,
    TOTCAL: i32,
    TRMLOG: ActualArray<i32>,
    XSQLOG: ActualArray<i32>,
    BAIL: bool,
    QLPARS: StackArray<bool, 20>,
    RPT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut GQUANT = vec![b' '; LNSIZE as usize];
        let mut MSGLOG = ActualCharArray2D::new(MAXMSG, 1..=2, 1..=MXPASS);
        let mut OP = vec![b' '; LNSIZE as usize];
        let mut QNAME = vec![b' '; LNSIZE as usize];
        let mut QCPARS = ActualCharArray::new(LNSIZE, 1..=MAXPAR);
        let mut QPNAMS = ActualCharArray::new(LNSIZE, 1..=MAXPAR);
        let mut XPREFX = vec![b' '; MAXMSG as usize];
        let mut ADJUST: f64 = 0.0;
        let mut CENTRL: f64 = 0.0;
        let mut CNFINE = StackArray::<f64, 206>::new(LBCELL..=MAXWIN);
        let mut CNFLOG = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=MXPASS);
        let mut CNFMES: f64 = 0.0;
        let mut ET0: f64 = 0.0;
        let mut ET1: f64 = 0.0;
        let mut MEASUR: f64 = 0.0;
        let mut QDPARS = StackArray::<f64, 20>::new(1..=MAXPAR);
        let mut REFVAL: f64 = 0.0;
        let mut REPLOG = ActualArray2D::<f64>::new(1..=3, 1..=MXCALL);
        let mut RESULT = StackArray::<f64, 206>::new(LBCELL..=MAXWIN);
        let mut STEP: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=NWMAX);
        let mut HANDLE: i32 = 0;
        let mut ISQLOG = StackArray::<i32, 20>::new(1..=MXPASS);
        let mut MW: i32 = 0;
        let mut NCALLS: i32 = 0;
        let mut NIVL: i32 = 0;
        let mut NPASS: i32 = 0;
        let mut NUPDAT: i32 = 0;
        let mut NW: i32 = 0;
        let mut QIPARS = StackArray::<i32, 20>::new(1..=MAXPAR);
        let mut QNPARS: i32 = 0;
        let mut SEQLOG = ActualArray::<i32>::new(1..=MXCALL);
        let mut TOTCAL: i32 = 0;
        let mut TRMLOG = ActualArray::<i32>::new(1..=MXCALL);
        let mut XSQLOG = ActualArray::<i32>::new(1..=MXCALL);
        let mut BAIL: bool = false;
        let mut QLPARS = StackArray::<bool, 20>::new(1..=MAXPAR);
        let mut RPT: bool = false;

        Self {
            GQUANT,
            MSGLOG,
            OP,
            QNAME,
            QCPARS,
            QPNAMS,
            XPREFX,
            ADJUST,
            CENTRL,
            CNFINE,
            CNFLOG,
            CNFMES,
            ET0,
            ET1,
            MEASUR,
            QDPARS,
            REFVAL,
            REPLOG,
            RESULT,
            STEP,
            TOL,
            WORK,
            HANDLE,
            ISQLOG,
            MW,
            NCALLS,
            NIVL,
            NPASS,
            NUPDAT,
            NW,
            QIPARS,
            QNPARS,
            SEQLOG,
            TOTCAL,
            TRMLOG,
            XSQLOG,
            BAIL,
            QLPARS,
            RPT,
        }
    }
}

//$Procedure F_GFLORP ( Test GF longitude progress reporting )
pub fn F_GFLORP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_GFLORP", ctx)?;

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

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Run a longitude LOCMIN search using GFEVNT.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Initialize the confinement window log.
    //
    for I in 1..=MXPASS {
        spicelib::SSIZED(MAXWIN, save.CNFLOG.subarray_mut([LBCELL, I]), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // when a local minimum occurs.
    //
    //
    // Note: the epoch below was derived using SPK and
    // PCK files created by the test utilities called above.
    //
    spicelib::STR2ET(b"2000 JUN 05 04:09:54.717 TDB", &mut save.CENTRL, ctx)?;

    save.NIVL = 3;
    save.CNFMES = 0.0;

    for I in 1..=save.NIVL {
        save.ET0 = ((-spicelib::SPD() + (((4 * (I - 2)) as f64) * spicelib::SPD())) + save.CENTRL);
        save.ET1 = (save.ET0 + ((2 as f64) * spicelib::SPD()));

        spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CNFMES = ((save.CNFMES + save.ET1) - save.ET0);
    }

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(2), b"MOON");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"LATITUDINAL");
    fstr::assign(save.QCPARS.get_mut(5), b"LONGITUDE");
    fstr::assign(save.QCPARS.get_mut(6), b"IAU_MOON");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"LOCMIN");
    save.REFVAL = 0.0;

    save.ADJUST = 0.0;

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 3000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scalar quantity local extrema searches (excluding longitude, RA,
    // etc.) require one pass.
    //
    // Look up the log of the progress report initialization calls. We
    // expect that there were 6 progress reporting passes.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 6, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 6");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude local extrema searches require 6 passes,
    // so we expect 6 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 6, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Run a longitude LOCMAX search using GFEVNT.", ctx)?;

    //
    // We'll use all inputs from the previous case except for the
    // relational operator and the confinement window.
    //
    fstr::assign(&mut save.OP, b"LOCMAX");

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // when a local maximum occurs.
    //
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Note: the epoch below was derived using SPK and
    // PCK files created by the test utilities called above.
    //
    spicelib::STR2ET(b"2000 JUN 16 04:37:06.960 TDB", &mut save.CENTRL, ctx)?;

    save.NIVL = 3;
    save.CNFMES = 0.0;

    for I in 1..=save.NIVL {
        save.ET0 = ((-spicelib::SPD() + (((4 * (I - 2)) as f64) * spicelib::SPD())) + save.CENTRL);
        save.ET1 = (save.ET0 + ((2 as f64) * spicelib::SPD()));

        spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.CNFMES = ((save.CNFMES + save.ET1) - save.ET0);
    }

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Scalar quantity local extrema searches (excluding longitude, RA,
    // etc.) require one pass.
    //
    // Look up the log of the progress report initialization calls. We
    // expect that there were 6 progress reporting passes.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 6, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 6");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude local extrema searches require 6 passes,
    // so we expect 6 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 6, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run a longitude ABSMAX search using GFEVNT. Max is in 2nd quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    spicelib::STR2ET(b"1999 SEP 25", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((150 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((150 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"LATITUDINAL");
    fstr::assign(save.QCPARS.get_mut(5), b"LONGITUDE");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMAX");
    save.REFVAL = 0.0;

    save.ADJUST = 0.0;

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA unadjusted absolute extrema searches require
    // 7 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 7, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude absolute extrema searches require 7 passes,
    // so we expect 7 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 7, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run a longitude ABSMAX search using GFEVNT. Max is in right half circle.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    spicelib::STR2ET(b"1999 SEP 25", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((90 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((90 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"LATITUDINAL");
    fstr::assign(save.QCPARS.get_mut(5), b"LONGITUDE");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMAX");
    save.REFVAL = 0.0;

    save.ADJUST = 0.0;

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA unadjusted absolute extrema searches require
    // 7 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 7, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude absolute extrema searches require 7 passes,
    // so we expect 7 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 7, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run a longitude ABSMAX search using GFEVNT. Max is in 4th quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    spicelib::STR2ET(b"1999 SEP 25", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"LATITUDINAL");
    fstr::assign(save.QCPARS.get_mut(5), b"LONGITUDE");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMAX");
    save.REFVAL = 0.0;

    save.ADJUST = 0.0;

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA unadjusted absolute extrema searches require
    // 7 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 7, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude absolute extrema searches require 7 passes,
    // so we expect 7 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 7, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run a longitude ABSMIN search using GFEVNT. Min is in 2nd quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    spicelib::STR2ET(b"2000 JUL 15", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"LATITUDINAL");
    fstr::assign(save.QCPARS.get_mut(5), b"LONGITUDE");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMIN");
    save.REFVAL = 0.0;

    save.ADJUST = 0.0;

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA unadjusted absolute extrema searches require
    // 7 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 7, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude absolute extrema searches require 7 passes,
    // so we expect 7 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 7, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run a longitude ABSMIN search using GFEVNT. Min is in right half.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    spicelib::STR2ET(b"2000 FEB 15", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"LATITUDINAL");
    fstr::assign(save.QCPARS.get_mut(5), b"LONGITUDE");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMIN");
    save.REFVAL = 0.0;

    save.ADJUST = 0.0;

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA unadjusted absolute extrema searches require
    // 7 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 7, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude absolute extrema searches require 7 passes,
    // so we expect 7 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 7, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run a longitude ABSMIN search using GFEVNT. Min is in 3rd quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    spicelib::STR2ET(b"2000 OCT 1", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"LATITUDINAL");
    fstr::assign(save.QCPARS.get_mut(5), b"LONGITUDE");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMIN");
    save.REFVAL = 0.0;

    save.ADJUST = 0.0;

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA unadjusted absolute extrema searches require
    // 7 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 7, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude absolute extrema searches require 7 passes,
    // so we expect 7 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 7, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run an RA adjusted ABSMAX search using GFEVNT. Max is in 4th quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    //
    spicelib::STR2ET(b"2000 FEB 1", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"RA/DEC");
    fstr::assign(save.QCPARS.get_mut(5), b"RIGHT ASCENSION");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMAX");
    save.REFVAL = 0.0;

    //
    // Note non-zero adjustment value:
    //
    save.ADJUST = (2.0 * spicelib::RPD(ctx));

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA adjusted absolute extrema searches require
    // 9 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 9, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7-9");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude adjusted absolute extrema searches require 7-9 passes;
    // in this case we expect 9 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 9, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run an RA adjusted ABSMAX search using GFEVNT. Max is in left half circle.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    //
    spicelib::STR2ET(b"2000 SEP 1", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"RA/DEC");
    fstr::assign(save.QCPARS.get_mut(5), b"RIGHT ASCENSION");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMAX");
    save.REFVAL = 0.0;

    //
    // Note non-zero adjustment value:
    //
    save.ADJUST = (2.0 * spicelib::RPD(ctx));

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA adjusted absolute extrema searches require
    // 9 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 9, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7-9");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude adjusted absolute extrema searches require 7-9 passes;
    // in this case we expect 9 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 9, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run an RA adjusted ABSMAX search using GFEVNT. Max is in 1st quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    //
    spicelib::STR2ET(b"2000 APR 1", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"RA/DEC");
    fstr::assign(save.QCPARS.get_mut(5), b"RIGHT ASCENSION");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMAX");
    save.REFVAL = 0.0;

    //
    // Note non-zero adjustment value:
    //
    save.ADJUST = (2.0 * spicelib::RPD(ctx));

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA adjusted absolute extrema searches require
    // 9 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 9, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7-9");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude adjusted absolute extrema searches require 7-9 passes;
    // in this case we expect 9 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 9, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run an RA adjusted ABSMIN search using GFEVNT. Min is in 4th quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    //
    spicelib::STR2ET(b"2000 FEB 1", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"RA/DEC");
    fstr::assign(save.QCPARS.get_mut(5), b"RIGHT ASCENSION");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMIN");
    save.REFVAL = 0.0;

    //
    // Note non-zero adjustment value:
    //
    save.ADJUST = (2.0 * spicelib::RPD(ctx));

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA adjusted absolute extrema searches require
    // 9 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 9, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7-9");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude adjusted absolute extrema searches require 7-9 passes;
    // in this case we expect 9 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 9, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run an RA adjusted ABSMIN search using GFEVNT. Min is in left half circle.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    //
    spicelib::STR2ET(b"2000 SEP 1", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"RA/DEC");
    fstr::assign(save.QCPARS.get_mut(5), b"RIGHT ASCENSION");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMIN");
    save.REFVAL = 0.0;

    //
    // Note non-zero adjustment value:
    //
    save.ADJUST = (2.0 * spicelib::RPD(ctx));

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA adjusted absolute extrema searches require
    // 9 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 9, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7-9");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude adjusted absolute extrema searches require 7-9 passes;
    // in this case we expect 9 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 9, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run an RA adjusted ABSMIN search using GFEVNT. Min is in 1st quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    //
    spicelib::STR2ET(b"2000 APR 1", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"RA/DEC");
    fstr::assign(save.QCPARS.get_mut(5), b"RIGHT ASCENSION");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"ABSMIN");
    save.REFVAL = 0.0;

    //
    // Note non-zero adjustment value:
    //
    save.ADJUST = (2.0 * spicelib::RPD(ctx));

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA adjusted absolute extrema searches require
    // 9 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 9, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 7-9");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude adjusted absolute extrema searches require 7-9 passes;
    // in this case we expect 9 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 9, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run an RA < (inequality) search using GFEVNT. Reference value is in 1st quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    //
    spicelib::STR2ET(b"2000 APR 1", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"RA/DEC");
    fstr::assign(save.QCPARS.get_mut(5), b"RIGHT ASCENSION");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"<");
    save.REFVAL = (spicelib::PI(ctx) / 4 as f64);

    save.ADJUST = 0.0;

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA inequality searches require 8 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 8, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 8");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude inequality searches require 8 passes,
    // so we expect 8 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 8, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run a longitude > (inequality) search using GFEVNT. Reference value is in 1st quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    //
    spicelib::STR2ET(b"2000 APR 1", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"LATITUDINAL");
    fstr::assign(save.QCPARS.get_mut(5), b"LONGITUDE");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b">");
    save.REFVAL = (spicelib::PI(ctx) / 4 as f64);

    save.ADJUST = 0.0;

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA inequality searches require 8 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 8, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 8");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude inequality searches require 8 passes,
    // so we expect 8 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 8, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Run a longitude equality search using GFEVNT. Reference value is in 1st quadrant.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;

    //
    // Initialize the progress reporting test utility package.
    //
    testutil::T_GFUINI(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // For absolute extrema, the Sun and Earth provide a good test
    // case, since the Earth-Sun position vector traverses the full
    // range of possible longitudes.
    //
    // Create a confinement window with 3 intervals. We pick
    // as the central epoch of the middle interval a time
    // near the epoch of the absolute minimum.
    //
    //
    spicelib::STR2ET(b"2000 APR 1", &mut save.ET0, ctx)?;

    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ET0 = (save.ET1 + spicelib::SPD());
    save.ET1 = (save.ET0 + ((10 as f64) * spicelib::SPD()));

    spicelib::WNINSD(save.ET0, save.ET1, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.GQUANT, b"COORDINATE");

    save.QNPARS = 10;
    fstr::assign(save.QPNAMS.get_mut(1), b"TARGET");
    fstr::assign(save.QPNAMS.get_mut(2), b"OBSERVER");
    fstr::assign(save.QPNAMS.get_mut(3), b"ABCORR");
    fstr::assign(save.QPNAMS.get_mut(4), b"COORDINATE SYSTEM");
    fstr::assign(save.QPNAMS.get_mut(5), b"COORDINATE");
    fstr::assign(save.QPNAMS.get_mut(6), b"REFERENCE FRAME");
    fstr::assign(save.QPNAMS.get_mut(7), b"VECTOR DEFINITION");
    fstr::assign(save.QPNAMS.get_mut(8), b"METHOD");
    fstr::assign(save.QPNAMS.get_mut(9), b"DREF");
    fstr::assign(save.QPNAMS.get_mut(10), b"DVEC");

    fstr::assign(save.QCPARS.get_mut(1), b"SUN");
    fstr::assign(save.QCPARS.get_mut(2), b"EARTH");
    fstr::assign(save.QCPARS.get_mut(3), b"NONE");
    fstr::assign(save.QCPARS.get_mut(4), b"LATITUDINAL");
    fstr::assign(save.QCPARS.get_mut(5), b"LONGITUDE");
    fstr::assign(save.QCPARS.get_mut(6), b"J2000");
    fstr::assign(save.QCPARS.get_mut(7), b"POSITION");
    fstr::assign(save.QCPARS.get_mut(8), b" ");
    fstr::assign(save.QCPARS.get_mut(9), b" ");

    spicelib::CLEARD(3, save.QDPARS.as_slice_mut());

    save.MW = MAXWIN;
    save.NW = NWMAX;

    fstr::assign(&mut save.OP, b"=");
    save.REFVAL = (spicelib::PI(ctx) / 4 as f64);

    save.ADJUST = 0.0;

    save.TOL = CNVTOL;

    save.RPT = true;
    save.BAIL = false;

    save.STEP = 100000.0;

    spicelib::GFSSTP(save.STEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFEVNT(
        spicelib::GFSTEP,
        spicelib::GFREFN,
        &save.GQUANT,
        save.QNPARS,
        save.QPNAMS.as_arg(),
        save.QCPARS.as_arg(),
        save.QDPARS.as_slice(),
        save.QIPARS.as_slice(),
        save.QLPARS.as_slice(),
        &save.OP,
        save.REFVAL,
        save.TOL,
        save.ADJUST,
        save.CNFINE.as_slice(),
        save.RPT,
        testutil::T_GFREPI,
        testutil::T_GFREPU,
        testutil::T_GFREPF,
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.BAIL,
        spicelib::GFBAIL,
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.MW = MAXWIN;

    testutil::T_GFRINI(
        MXPASS,
        save.MW,
        &mut save.NPASS,
        save.ISQLOG.as_slice_mut(),
        save.CNFLOG.as_slice_mut(),
        save.MSGLOG.as_arg_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude/RA equality searches require 2 passes.
    //
    // Look up the log of the progress report initialization calls.
    //
    testutil::CHCKSI(b"No. of T_GFREPI calls", save.NPASS, b"=", 2, 0, OK, ctx)?;

    //
    // Check the sequence number of the first call.
    //
    testutil::CHCKSI(b"SEQLOG(1)", save.ISQLOG[1], b"=", 1, 0, OK, ctx)?;

    //
    // Check the progress report message prefixes and suffixes.
    //
    for I in 1..=save.NPASS {
        fstr::assign(&mut save.QNAME, b"Prefix *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut save.XPREFX, b"Coordinate pass * of 2");
        spicelib::REPMI(&save.XPREFX.to_vec(), b"*", I, &mut save.XPREFX, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSC(
            &save.QNAME,
            &save.MSGLOG[[1, I]],
            b"=",
            &save.XPREFX,
            OK,
            ctx,
        )?;

        testutil::CHCKSC(b"Suffix", &save.MSGLOG[[2, I]], b"=", b"done.", OK, ctx)?;
    }

    //
    // Get the log of the report termination calls.
    //
    testutil::T_GFRTRM(MXPASS, &mut save.NCALLS, save.TRMLOG.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Longitude equality searches require 2 passes,
    // so we expect 2 report termination calls.
    //
    testutil::CHCKSI(b"No. of T_GFRTRM calls", save.NCALLS, b"=", 2, 0, OK, ctx)?;

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
    // Check the updates for each pass.
    //
    save.TOTCAL = 0;

    for I in 1..=save.NPASS {
        //
        // Check the sequence numbers of the calls. They should
        // range from the sequence number of the last init
        // call +1 to the sequence number of the Ith termination
        // call -1. They should be in increasing
        // order.
        //

        save.NCALLS = ((save.TRMLOG[I] - save.ISQLOG[I]) - 1);

        for J in 1..=save.NCALLS {
            save.XSQLOG[J] = (save.ISQLOG[I] + J);
        }

        fstr::assign(&mut save.QNAME, b"Update SEQLOG, pass *");
        spicelib::REPMI(&save.QNAME.to_vec(), b"*", I, &mut save.QNAME, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKAI(
            &save.QNAME,
            save.SEQLOG.subarray((save.TOTCAL + 1)),
            b"=",
            save.XSQLOG.as_slice(),
            save.NCALLS,
            OK,
            ctx,
        )?;

        save.TOTCAL = (save.TOTCAL + save.NCALLS);
    }

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
    // Find the sum of the measures of the confinement windows
    // for each pass.
    //
    save.CNFMES = 0.0;

    for I in 1..=save.NPASS {
        for J in intrinsics::range(
            2,
            spicelib::CARDD(save.CNFLOG.subarray([LBCELL, I]), ctx)?,
            2,
        ) {
            save.CNFMES = ((save.CNFMES + save.CNFLOG[[J, I]]) - save.CNFLOG[[(J - 1), I]]);
        }
    }

    //
    // Compare the measure of the reported progress to the sum of
    // the measures of the confinement windows used for each
    // search.
    //
    testutil::CHCKSD(b"MEASUR", save.MEASUR, b"~", save.CNFMES, TIGHT, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Clean up:  delete kernels.", ctx)?;

    spicelib::SPKUEF(save.HANDLE, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // To be added, if necessary.
    //

    //
    //---- Case -------------------------------------------------------------
    //

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
