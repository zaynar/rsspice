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
const SPK: &[u8] = b"gfrr.bsp";
const SPK1: &[u8] = b"nat.bsp";
const PCK1: &[u8] = b"nat.pck";
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
    RELATE: Vec<u8>,
    TARGET: Vec<u8>,
    OBSRVR: Vec<u8>,
    ABCORR: Vec<u8>,
    TITLE: Vec<u8>,
    CORR: ActualCharArray,
    ADJUST: f64,
    BEG: f64,
    END: f64,
    LEFT: f64,
    REFVAL: f64,
    RIGHT: f64,
    STEP: f64,
    RESULT: ActualArray<f64>,
    CNFINE: ActualArray<f64>,
    WORK: ActualArray2D<f64>,
    NDAYS: i32,
    COUNT: i32,
    MW: i32,
    NW: i32,
    HANDL1: i32,
    HANDL2: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RELATE = vec![b' '; BDNMLN as usize];
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut ADJUST: f64 = 0.0;
        let mut BEG: f64 = 0.0;
        let mut END: f64 = 0.0;
        let mut LEFT: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut RIGHT: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=NWRR);
        let mut NDAYS: i32 = 0;
        let mut COUNT: i32 = 0;
        let mut MW: i32 = 0;
        let mut NW: i32 = 0;
        let mut HANDL1: i32 = 0;
        let mut HANDL2: i32 = 0;

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
            RELATE,
            TARGET,
            OBSRVR,
            ABCORR,
            TITLE,
            CORR,
            ADJUST,
            BEG,
            END,
            LEFT,
            REFVAL,
            RIGHT,
            STEP,
            RESULT,
            CNFINE,
            WORK,
            NDAYS,
            COUNT,
            MW,
            NW,
            HANDL1,
            HANDL2,
        }
    }
}

//$Procedure F_GFRR ( GFRR family tests )
pub fn F_GFRR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_GFRR", ctx)?;

    testutil::TCASE(b"Setup: create and load SPK and LSK files.", ctx)?;

    //
    // Leapseconds:  Note that the LSK is deleted after loading, so we
    // don't have to clean it up later.
    //
    testutil::TSTLSK(ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Load an SPK file as well.
    //
    testutil::TSTSPK(SPK, false, &mut save.HANDL1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK, ctx)?;
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
    testutil::NATSPK(SPK1, false, &mut save.HANDL2, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::FURNSH(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.LEFT = 0.0;
    save.RIGHT = ((360 as f64) * spicelib::SPD());

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(2, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Case 1
    //
    testutil::TCASE(b"Invalid result window size", ctx)?;

    spicelib::SSIZED(1, save.RESULT.as_slice_mut(), ctx)?;

    save.STEP = 10.0;
    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.RELATE, b">");
    fstr::assign(&mut save.ABCORR, b"NONE");

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWRR,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    spicelib::SSIZED(0, save.RESULT.as_slice_mut(), ctx)?;

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWRR,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Case 2
    //
    testutil::TCASE(b"Non-positive step size", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(2, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 0.0;
    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.RELATE, b">");
    fstr::assign(&mut save.ABCORR, b"NONE");

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWRR,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    save.STEP = -1.0;
    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWRR,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    // Case 3
    //

    testutil::TCASE(b"Invalid aberration correction specifier", ctx)?;

    save.STEP = ((7 as f64) * spicelib::SPD());
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    save.REFVAL = 0.0;
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    save.MW = MAXWIN;
    save.NW = NWRR;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut save.ABCORR, b"X");

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Case 4
    //

    testutil::TCASE(b"Invalid relations operator", ctx)?;

    save.STEP = ((7 as f64) * spicelib::SPD());
    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    fstr::assign(&mut save.ABCORR, b"NONE");
    save.MW = MAXWIN;
    save.NW = NWRR;
    fstr::assign(&mut save.ABCORR, b"NONE");

    fstr::assign(&mut save.RELATE, b"!=");

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    // Case 5
    //

    testutil::TCASE(b"Invalid body names", ctx)?;

    save.MW = MAXWIN;
    save.NW = NWRR;
    fstr::assign(&mut save.RELATE, b"LOCMAX");

    fstr::assign(&mut save.TARGET, b"X");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    fstr::assign(&mut save.TARGET, b"EARTH");
    fstr::assign(&mut save.OBSRVR, b"X");

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // Case 6
    //

    testutil::TCASE(b"Negative adjustment value", ctx)?;

    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    save.REFVAL = 0.0;
    save.STEP = ((7 as f64) * spicelib::SPD());
    save.MW = MAXWIN;
    save.NW = NWRR;

    save.ADJUST = -1.0;

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Case 7
    //

    testutil::TCASE(b"Ephemeris data unavailable", ctx)?;

    fstr::assign(&mut save.TARGET, b"DAWN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.STEP = ((7 as f64) * spicelib::SPD());
    save.MW = MAXWIN;
    save.NW = NWRR;

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // Case 8
    //
    testutil::TCASE(b"Invalid value for MW, NW", ctx)?;

    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"SUN");
    fstr::assign(&mut save.RELATE, b"LOCMAX");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.STEP = ((7 as f64) * spicelib::SPD());

    //
    // Usable size of WORK windows is zero.
    //
    save.MW = 0;
    save.NW = NWRR;

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Usable size of WORK windows is positive but below limit.
    //
    save.MW = 1;
    save.NW = NWRR;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Usable size of WORK windows is positive but is too small
    // to hold all intervals found across CNFINE. CNFINE spans
    // 360 days, the local maximums occur approximately every
    // 28 days.
    //
    //

    save.MW = 6;
    save.NW = NWRR;

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WINDOWEXCESS)", OK, ctx)?;

    //
    // Work window count below limit - NW = 4
    //

    save.MW = MAXWIN;
    save.NW = 4;
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Case 9
    //
    // Loops over aberration corrections for a configuration with
    // a known geometry and behavior.
    //
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.OBSRVR, b"BETA");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.STEP = (0.4 * spicelib::SPD());
    save.MW = MAXWIN;
    save.NW = NWRR;

    //
    // Confine for ninety days.
    //
    save.NDAYS = 90;
    save.LEFT = 0.0;
    save.RIGHT = ((save.NDAYS as f64) * spicelib::SPD());

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 1..=NCORR {
        fstr::assign(&mut save.ABCORR, save.CORR.get(I));

        fstr::assign(&mut save.RELATE, b"LOCMAX");
        spicelib::REPMC(b"# #", b"#", &save.RELATE, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFRR(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            &save.RELATE,
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            save.MW,
            save.NW,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        // We expect one event per day.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", save.NDAYS, 0, OK, ctx)?;

        fstr::assign(&mut save.RELATE, b"LOCMIN");
        spicelib::REPMC(b"# #", b"#", &save.RELATE, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFRR(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            &save.RELATE,
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            save.MW,
            save.NW,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        // We expect one event per day.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", save.NDAYS, 0, OK, ctx)?;

        //
        // Each orbit of BETA includes a local minimum and local
        // maximum range rate event with respect to ALPHA. As the
        // orbit is periodic and closed, two events per orbit must
        // exist where the sign of the range rate changes., i.e.
        // the range rate equals zero.
        //
        fstr::assign(&mut save.RELATE, b"=");
        save.REFVAL = 0.0;

        spicelib::REPMC(b"# #", b"#", &save.RELATE, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFRR(
            &save.TARGET,
            &save.ABCORR,
            &save.OBSRVR,
            &save.RELATE,
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            save.MW,
            save.NW,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        // We expect two events per day.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", (2 * save.NDAYS), 0, OK, ctx)?;
    }

    //
    // Case 10
    //
    fstr::assign(&mut save.TITLE, b"Check the GF call uses the GFSTOL value");
    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Re-run a valid search after using GFSTOL to set the convergence
    // tolerance to a value that should cause a numerical error signal.
    //

    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.OBSRVR, b"BETA");
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    save.STEP = (0.4 * spicelib::SPD());
    fstr::assign(&mut save.RELATE, b"LOCMIN");
    fstr::assign(&mut save.ABCORR, b"NONE");

    save.LEFT = 0.0;
    save.RIGHT = (2.0 * spicelib::SPD());

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWRR,
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

    spicelib::GFRR(
        &save.TARGET,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWRR,
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
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(SPK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::DELFIL(PCK1, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
