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
const PCK: &[u8] = b"gfpa.pck";
const LSK: &[u8] = b"gfpa.tls";
const SPK1: &[u8] = b"nat.bsp";
const PCK1: &[u8] = b"nat.pck";
const MEDIUM: f64 = 0.0001;
const LNSIZE: i32 = 80;
const LBCELL: i32 = -5;
const MAXWIN: i32 = 5000;
const NCORR: i32 = 5;

struct SaveVars {
    ABCORR: Vec<u8>,
    CORR: ActualCharArray,
    TARGET: Vec<u8>,
    LUMIN: Vec<u8>,
    OBSRVR: Vec<u8>,
    RELATE: Vec<u8>,
    TITLE: Vec<u8>,
    ADJUST: f64,
    ALPHA: f64,
    CNFINE: ActualArray<f64>,
    BEG: f64,
    END: f64,
    LT: f64,
    LEFT: f64,
    RIGHT: f64,
    REFVAL: f64,
    RESULT: ActualArray<f64>,
    STEP: f64,
    WORK: ActualArray2D<f64>,
    POSA: StackArray<f64, 3>,
    POSB: StackArray<f64, 3>,
    VALUE: f64,
    HAN1: i32,
    COUNT: i32,
    NW: i32,
    NDAYS: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ABCORR = vec![b' '; LNSIZE as usize];
        let mut CORR = ActualCharArray::new(LNSIZE, 1..=NCORR);
        let mut TARGET = vec![b' '; LNSIZE as usize];
        let mut LUMIN = vec![b' '; LNSIZE as usize];
        let mut OBSRVR = vec![b' '; LNSIZE as usize];
        let mut RELATE = vec![b' '; LNSIZE as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut ADJUST: f64 = 0.0;
        let mut ALPHA: f64 = 0.0;
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut BEG: f64 = 0.0;
        let mut END: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut LEFT: f64 = 0.0;
        let mut RIGHT: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut STEP: f64 = 0.0;
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=NWPA);
        let mut POSA = StackArray::<f64, 3>::new(1..=3);
        let mut POSB = StackArray::<f64, 3>::new(1..=3);
        let mut VALUE: f64 = 0.0;
        let mut HAN1: i32 = 0;
        let mut COUNT: i32 = 0;
        let mut NW: i32 = 0;
        let mut NDAYS: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"NONE"),
                Val::C(b"lt"),
                Val::C(b" lt+s"),
                Val::C(b" cn"),
                Val::C(b" cn + s"),
            ]
            .into_iter();
            CORR.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ABCORR,
            CORR,
            TARGET,
            LUMIN,
            OBSRVR,
            RELATE,
            TITLE,
            ADJUST,
            ALPHA,
            CNFINE,
            BEG,
            END,
            LT,
            LEFT,
            RIGHT,
            REFVAL,
            RESULT,
            STEP,
            WORK,
            POSA,
            POSB,
            VALUE,
            HAN1,
            COUNT,
            NW,
            NDAYS,
        }
    }
}

//$Procedure F_GFPA ( GFPA family tests )
pub fn F_GFPA(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    testutil::TOPEN(b"F_GFPA", ctx)?;

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

    save.LEFT = 0.0;
    save.RIGHT = ((360 as f64) * spicelib::SPD());

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(2, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
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
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    // Case 2
    //
    testutil::TCASE(b"Non unique body IDs.", ctx)?;

    save.STEP = 1.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"MOON");
    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    fstr::assign(&mut save.TARGET, b"SUN");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.OBSRVR, b"EARTH");

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.OBSRVR, b"SUN");

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BODIESNOTDISTINCT)", OK, ctx)?;

    //
    // Case 3
    //
    testutil::TCASE(b"Invalid aberration correction specifier", ctx)?;

    save.STEP = 1.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.ABCORR, b"X");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDOPTION)", OK, ctx)?;

    //
    // Case 4
    //
    testutil::TCASE(b"Invalid relations operator", ctx)?;

    save.STEP = 1.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.RELATE, b"==");
    save.REFVAL = 0.0;

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    // Case 5
    //
    testutil::TCASE(b"Invalid body names", ctx)?;

    save.STEP = 1.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"X");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.LUMIN, b"X");
    fstr::assign(&mut save.OBSRVR, b"EARTH");

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.OBSRVR, b"X");

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(IDCODENOTFOUND)", OK, ctx)?;

    //
    // Case 6
    //
    testutil::TCASE(b"Negative adjustment value", ctx)?;

    save.STEP = 1.0;
    save.ADJUST = -1.0;
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Case 7
    //
    testutil::TCASE(b"Ephemeris data unavailable", ctx)?;

    save.STEP = 1.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"DAWN");
    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(SPKINSUFFDATA)", OK, ctx)?;

    //
    // Case 8
    //
    testutil::TCASE(b"Invalid value for MW and NW", ctx)?;

    save.STEP = 1.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"MOON");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"EARTH");
    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        0,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Work window count below limit - NW = 4
    //

    save.NW = 4;

    save.STEP = 1.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"BETA");
    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.0;

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.ABCORR,
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
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
    save.STEP = (0.2 * spicelib::SPD());
    save.ADJUST = 0.0;
    save.REFVAL = 0.0;
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"BETA");

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
        //
        // Observe the phase angle of ALPHA from BETA. Two
        // minimum values per day will occur.
        //
        fstr::assign(&mut save.RELATE, b"LOCMIN");

        fstr::assign(&mut save.ABCORR, save.CORR.get(I));

        spicelib::REPMC(b"# #", b"#", &save.RELATE, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFPA(
            &save.TARGET,
            &save.LUMIN,
            &save.ABCORR,
            &save.OBSRVR,
            &save.RELATE,
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWPA,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        // We expect two events per day over CNFINE.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", (2 * save.NDAYS), 0, OK, ctx)?;

        //
        // Two maximum values per day will occur.
        //
        fstr::assign(&mut save.RELATE, b"LOCMAX");

        spicelib::REPMC(b"# #", b"#", &save.RELATE, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFPA(
            &save.TARGET,
            &save.LUMIN,
            &save.ABCORR,
            &save.OBSRVR,
            &save.RELATE,
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWPA,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Check the number of intervals in the result window.
        // We expect two events per day over CNFINE.
        //
        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;
        testutil::CHCKSI(b"COUNT", save.COUNT, b"=", (2 * save.NDAYS), 0, OK, ctx)?;
    }

    //
    // Check for a specific reference value.
    //
    fstr::assign(&mut save.RELATE, b"=");
    save.REFVAL = 0.1;

    for I in 1..=NCORR {
        fstr::assign(&mut save.ABCORR, save.CORR.get(I));

        spicelib::REPMC(b"# #", b"#", &save.RELATE, &mut save.TITLE);
        spicelib::REPMC(&save.TITLE.to_vec(), b"#", &save.ABCORR, &mut save.TITLE);
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFPA(
            &save.TARGET,
            &save.LUMIN,
            &save.ABCORR,
            &save.OBSRVR,
            &save.RELATE,
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWPA,
            save.WORK.as_slice_mut(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.COUNT = 0;
        save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

        for J in 1..=save.COUNT {
            spicelib::WNFETD(save.RESULT.as_slice(), J, &mut save.BEG, &mut save.END, ctx)?;

            spicelib::ZZGFPAQ(save.BEG, 1000, 10, 2000, &save.ABCORR, &mut save.VALUE, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            testutil::CHCKSD(&save.TITLE, save.REFVAL, b"~", save.VALUE, MEDIUM, OK, ctx)?;
        }
    }

    //
    // Case 10
    //
    fstr::assign(&mut save.TITLE, b"Check phase angle against known value");
    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // We can compute the value of the maximum phase angle based
    // on the ALPHA-BETA geometry, the half-angle of the BETA
    // orbit as seen from ALPHA.
    //
    //    sin(alpha) = beta_orbit_radius     5.246368076245e+05
    //                 ------------------ ~  ------------------
    //                 alpha_orbit_radius    2.098547206045e+06
    //

    spicelib::SPKEZP(
        1000,
        save.LEFT,
        b"J2000",
        &save.CORR[1],
        10,
        save.POSA.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    spicelib::SPKEZP(
        2000,
        save.LEFT,
        b"J2000",
        &save.CORR[1],
        10,
        save.POSB.as_slice_mut(),
        &mut save.LT,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.ALPHA =
        f64::asin((spicelib::VNORM(save.POSB.as_slice()) / spicelib::VNORM(save.POSA.as_slice())));

    save.STEP = (0.2 * spicelib::SPD());
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"ALPHA");
    fstr::assign(&mut save.LUMIN, b"SUN");
    fstr::assign(&mut save.OBSRVR, b"BETA");
    fstr::assign(&mut save.RELATE, b"LOCMAX");

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.CORR[1],
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.COUNT = 0;
    save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    for J in 1..=save.COUNT {
        spicelib::WNFETD(save.RESULT.as_slice(), J, &mut save.BEG, &mut save.END, ctx)?;

        spicelib::ZZGFPAQ(
            save.BEG,
            1000,
            10,
            2000,
            &save.CORR[1],
            &mut save.VALUE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(&save.TITLE, save.ALPHA, b"~", save.VALUE, MEDIUM, OK, ctx)?;
    }

    //
    // Minimum phase angle should have value approximately 0.
    //
    fstr::assign(&mut save.RELATE, b"LOCMIN");

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.CORR[1],
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
        save.WORK.as_slice_mut(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.COUNT = 0;
    save.COUNT = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    for J in 1..=save.COUNT {
        spicelib::WNFETD(save.RESULT.as_slice(), J, &mut save.BEG, &mut save.END, ctx)?;

        spicelib::ZZGFPAQ(
            save.BEG,
            1000,
            10,
            2000,
            &save.CORR[1],
            &mut save.VALUE,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(&save.TITLE, 0.0, b"~", save.VALUE, MEDIUM, OK, ctx)?;
    }

    //
    // Case 11
    //
    fstr::assign(&mut save.TITLE, b"Check the GF call uses the GFSTOL value");
    testutil::TCASE(&save.TITLE, ctx)?;

    spicelib::WNFETD(save.RESULT.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;

    //
    // Re-run a valid search after using GFSTOL.
    //

    spicelib::GFSTOL(0.0001, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;

    spicelib::GFPA(
        &save.TARGET,
        &save.LUMIN,
        &save.CORR[1],
        &save.OBSRVR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWPA,
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

    Ok(())
}
