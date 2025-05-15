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
const SPK: &[u8] = b"gfuds.bsp";
const SPK1: &[u8] = b"nat.bsp";
const PCK1: &[u8] = b"nat.pck";
const BDNMLN: i32 = 36;
const LNSIZE: i32 = 80;
const LBCELL: i32 = -5;
const MAXWIN: i32 = 20000;
const NLOOPS: i32 = 7;
const VTIGHT: f64 = 0.0000000001;

struct SaveVars {
    RELATE: Vec<u8>,
    RELATS: ActualCharArray,
    TARGET: Vec<u8>,
    OBSRVR: Vec<u8>,
    ABCORR: Vec<u8>,
    TITLE: Vec<u8>,
    BEG: f64,
    END: f64,
    STRTR: f64,
    FNISHR: f64,
    STRTU: f64,
    FNISHU: f64,
    ADJUST: f64,
    LEFT: f64,
    RIGHT: f64,
    REFVAL: f64,
    STEP: f64,
    LT: f64,
    DRDTU: f64,
    DRDTR: f64,
    POSU: StackArray<f64, 6>,
    POSR: StackArray<f64, 6>,
    RESLUD: ActualArray<f64>,
    RESLRR: ActualArray<f64>,
    CNFINE: ActualArray<f64>,
    WORK: ActualArray2D<f64>,
    TOL: f64,
    COUNT: i32,
    MW: i32,
    NW: i32,
    HANDL1: i32,
    HANDL2: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RELATE = vec![b' '; BDNMLN as usize];
        let mut RELATS = ActualCharArray::new(BDNMLN, 1..=NLOOPS);
        let mut TARGET = vec![b' '; BDNMLN as usize];
        let mut OBSRVR = vec![b' '; BDNMLN as usize];
        let mut ABCORR = vec![b' '; BDNMLN as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];
        let mut BEG: f64 = 0.0;
        let mut END: f64 = 0.0;
        let mut STRTR: f64 = 0.0;
        let mut FNISHR: f64 = 0.0;
        let mut STRTU: f64 = 0.0;
        let mut FNISHU: f64 = 0.0;
        let mut ADJUST: f64 = 0.0;
        let mut LEFT: f64 = 0.0;
        let mut RIGHT: f64 = 0.0;
        let mut REFVAL: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut LT: f64 = 0.0;
        let mut DRDTU: f64 = 0.0;
        let mut DRDTR: f64 = 0.0;
        let mut POSU = StackArray::<f64, 6>::new(1..=6);
        let mut POSR = StackArray::<f64, 6>::new(1..=6);
        let mut RESLUD = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut RESLRR = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut CNFINE = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut WORK = ActualArray2D::<f64>::new(LBCELL..=MAXWIN, 1..=NWUDS);
        let mut TOL: f64 = 0.0;
        let mut COUNT: i32 = 0;
        let mut MW: i32 = 0;
        let mut NW: i32 = 0;
        let mut HANDL1: i32 = 0;
        let mut HANDL2: i32 = 0;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"="),
                Val::C(b"<"),
                Val::C(b">"),
                Val::C(b"LOCMIN"),
                Val::C(b"ABSMIN"),
                Val::C(b"LOCMAX"),
                Val::C(b"ABSMAX"),
            ]
            .into_iter();
            RELATS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            RELATE,
            RELATS,
            TARGET,
            OBSRVR,
            ABCORR,
            TITLE,
            BEG,
            END,
            STRTR,
            FNISHR,
            STRTU,
            FNISHU,
            ADJUST,
            LEFT,
            RIGHT,
            REFVAL,
            STEP,
            LT,
            DRDTU,
            DRDTR,
            POSU,
            POSR,
            RESLUD,
            RESLRR,
            CNFINE,
            WORK,
            TOL,
            COUNT,
            MW,
            NW,
            HANDL1,
            HANDL2,
        }
    }
}

//$Procedure F_GFUDS ( GFUDS family tests )
pub fn F_GFUDS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFUDS", ctx)?;

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

    spicelib::SSIZED(MAXWIN, save.RESLUD.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXWIN, save.RESLRR.as_slice_mut(), ctx)?;
    spicelib::SSIZED(2, save.CNFINE.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    //
    // Create a confinement window.
    //
    spicelib::STR2ET(b"2007 JAN 1", &mut save.LEFT, ctx)?;
    spicelib::STR2ET(b"2007 MAY 1", &mut save.RIGHT, ctx)?;
    spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Case 1
    //

    testutil::TCASE(b"Invalid result window size", ctx)?;

    spicelib::SSIZED(1, save.RESLUD.as_slice_mut(), ctx)?;

    save.STEP = spicelib::SPD();
    save.REFVAL = 0.3365;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"=");

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWUDS,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    spicelib::SSIZED(0, save.RESLUD.as_slice_mut(), ctx)?;

    save.STEP = spicelib::SPD();
    save.REFVAL = 0.3365;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"=");

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWUDS,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Case 2
    //

    testutil::TCASE(b"Non-positive step size", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESLUD.as_slice_mut(), ctx)?;

    save.STEP = -1.0;
    save.REFVAL = 0.3365;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"=");

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWUDS,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    save.STEP = 0.0;

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWUDS,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    // Case 3
    //

    testutil::TCASE(b"Invalid relations operator", ctx)?;

    save.STEP = spicelib::SPD();
    save.REFVAL = 0.3365;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"!=");

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWUDS,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(NOTRECOGNIZED)", OK, ctx)?;

    //
    // Case 4
    //

    testutil::TCASE(b"Negative adjustment value", ctx)?;

    save.STEP = spicelib::SPD();
    save.REFVAL = 0.3365;
    save.ADJUST = -1.0;
    fstr::assign(&mut save.RELATE, b"=");

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWUDS,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Case 5
    //

    testutil::TCASE(b"Invalid value for MW, NW", ctx)?;

    save.STEP = spicelib::SPD();
    save.REFVAL = 0.3365;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"=");
    save.MW = 0;
    save.NW = NWUDS;

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Usable size of WORK windows is positive but below limit.
    //
    save.MW = 1;
    save.NW = NWUDS;

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Usable size of WORK windows is positive but is too small
    // to hold all intervals found across CNFINE. CNFINE spans
    // 120 days, the constraint occurs approximately every
    // 15 days.
    //
    //
    save.STEP = spicelib::SPD();
    save.REFVAL = 0.3365;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"=");
    save.MW = 6;
    save.NW = NWUDS;

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(WINDOWEXCESS)", OK, ctx)?;

    //
    // Work window count below limit - NW = 4
    //

    save.STEP = spicelib::SPD();
    save.REFVAL = 0.3365;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.RELATE, b"=");
    save.MW = MAXWIN;
    save.NW = 4;

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        save.MW,
        save.NW,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;

    //
    // Case 6
    //
    save.STEP = spicelib::SPD();
    save.REFVAL = 0.3365;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"301");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"10");

    for I in 1..=NLOOPS {
        fstr::assign(&mut save.RELATE, save.RELATS.get(I));

        spicelib::REPMC(
            b"GFRR vs GFUDS numerical: #",
            b"#",
            &save.RELATE,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

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
            MAXWIN,
            NWRR,
            save.WORK.as_slice_mut(),
            save.RESLRR.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;
        spicelib::WNINSD(save.LEFT, save.RIGHT, save.CNFINE.as_slice_mut(), ctx)?;

        //
        // GFDECR - wholly numerical derivative
        //
        spicelib::GFUDS(
            GFQ,
            GFDECR,
            &save.RELATE,
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWUDS,
            save.WORK.as_slice_mut(),
            save.RESLUD.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // The cardinality of RESULD and RESLRR better match...
        //
        testutil::CHCKSL(
            b"COUNT != 0",
            ((spicelib::WNCARD(save.RESLUD.as_slice(), ctx)? as f64) != 0.0),
            true,
            OK,
            ctx,
        )?;

        testutil::CHCKSI(
            b"COUNT",
            spicelib::WNCARD(save.RESLUD.as_slice(), ctx)?,
            b"=",
            spicelib::WNCARD(save.RESLRR.as_slice(), ctx)?,
            0,
            OK,
            ctx,
        )?;

        if *OK {
            for J in 1..=spicelib::WNCARD(save.RESLUD.as_slice(), ctx)? {
                //
                // Fetch the endpoints of the Jth interval
                // of the result windows.
                //
                spicelib::WNFETD(
                    save.RESLRR.as_slice(),
                    J,
                    &mut save.STRTR,
                    &mut save.FNISHR,
                    ctx,
                )?;
                spicelib::WNFETD(
                    save.RESLUD.as_slice(),
                    J,
                    &mut save.STRTU,
                    &mut save.FNISHU,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKEZR(
                    b"301",
                    save.STRTU,
                    b"J2000",
                    b"NONE",
                    b"10",
                    save.POSU.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                spicelib::SPKEZR(
                    b"301",
                    save.STRTR,
                    b"J2000",
                    b"NONE",
                    b"10",
                    save.POSR.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.DRDTU = spicelib::DVNORM(save.POSU.as_slice());
                save.DRDTR = spicelib::DVNORM(save.POSR.as_slice());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(b"Start", save.DRDTU, b"~", save.DRDTR, VTIGHT, OK, ctx)?;

                spicelib::SPKEZR(
                    b"301",
                    save.FNISHU,
                    b"J2000",
                    b"NONE",
                    b"10",
                    save.POSU.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                spicelib::SPKEZR(
                    b"301",
                    save.FNISHR,
                    b"J2000",
                    b"NONE",
                    b"10",
                    save.POSR.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                save.DRDTU = spicelib::DVNORM(save.POSU.as_slice());
                save.DRDTR = spicelib::DVNORM(save.POSR.as_slice());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(b"Finish", save.DRDTU, b"~", save.DRDTR, VTIGHT, OK, ctx)?;
            }
        }

        //
        // GFRRDC - analytic/numerical derivative
        //
        spicelib::REPMC(
            b"GFRR vs GFUDS analytic: #",
            b"#",
            &save.RELATE,
            &mut save.TITLE,
        );
        testutil::TCASE(&save.TITLE, ctx)?;

        spicelib::GFUDS(
            GFQ,
            GFRRDC,
            &save.RELATE,
            save.REFVAL,
            save.ADJUST,
            save.STEP,
            save.CNFINE.as_slice(),
            MAXWIN,
            NWUDS,
            save.WORK.as_slice_mut(),
            save.RESLUD.as_slice_mut(),
            ctx,
        )?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // The cardinality of RESULD and RESLRR better match...
        //
        testutil::CHCKSL(
            b"COUNT != 0",
            ((spicelib::WNCARD(save.RESLUD.as_slice(), ctx)? as f64) != 0.0),
            true,
            OK,
            ctx,
        )?;

        testutil::CHCKSI(
            b"COUNT",
            spicelib::WNCARD(save.RESLUD.as_slice(), ctx)?,
            b"=",
            spicelib::WNCARD(save.RESLRR.as_slice(), ctx)?,
            0,
            OK,
            ctx,
        )?;

        if *OK {
            for J in 1..=spicelib::WNCARD(save.RESLUD.as_slice(), ctx)? {
                //
                // Fetch the endpoints of the Jth interval
                // of the result windows. The values of the range rate
                // should equal to within VTIGHT.
                //
                spicelib::WNFETD(
                    save.RESLRR.as_slice(),
                    J,
                    &mut save.STRTR,
                    &mut save.FNISHR,
                    ctx,
                )?;
                spicelib::WNFETD(
                    save.RESLUD.as_slice(),
                    J,
                    &mut save.STRTU,
                    &mut save.FNISHU,
                    ctx,
                )?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKEZR(
                    b"301",
                    save.STRTU,
                    b"J2000",
                    b"NONE",
                    b"10",
                    save.POSU.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                save.DRDTU = spicelib::DVNORM(save.POSU.as_slice());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKEZR(
                    b"301",
                    save.STRTR,
                    b"J2000",
                    b"NONE",
                    b"10",
                    save.POSR.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                save.DRDTR = spicelib::DVNORM(save.POSR.as_slice());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(b"Start", save.DRDTU, b"~", save.DRDTR, VTIGHT, OK, ctx)?;

                spicelib::SPKEZR(
                    b"301",
                    save.FNISHU,
                    b"J2000",
                    b"NONE",
                    b"10",
                    save.POSU.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                save.DRDTU = spicelib::DVNORM(save.POSU.as_slice());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                spicelib::SPKEZR(
                    b"301",
                    save.FNISHR,
                    b"J2000",
                    b"NONE",
                    b"10",
                    save.POSR.as_slice_mut(),
                    &mut save.LT,
                    ctx,
                )?;
                save.DRDTR = spicelib::DVNORM(save.POSR.as_slice());
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                testutil::CHCKSD(b"Finish", save.DRDTU, b"~", save.DRDTR, VTIGHT, OK, ctx)?;
            }
        }
    }

    //
    // Case 7
    //
    fstr::assign(&mut save.TITLE, b"Check the GF call uses the GFSTOL value");
    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Re-run a valid search after using GFSTOL to set the convergence
    // tolerance to a value that should cause a numerical error signal.
    //
    save.STEP = spicelib::SPD();
    save.REFVAL = 0.0;
    save.ADJUST = 0.0;
    fstr::assign(&mut save.TARGET, b"301");
    fstr::assign(&mut save.ABCORR, b"NONE");
    fstr::assign(&mut save.OBSRVR, b"10");
    fstr::assign(&mut save.RELATE, b"ABSMIN");

    spicelib::SCARDD(0, save.RESLUD.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWUDS,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.COUNT = 0;
    save.COUNT = spicelib::WNCARD(save.RESLUD.as_slice(), ctx)?;
    testutil::CHCKSI(b"COUNT", save.COUNT, b"!=", 0, 0, OK, ctx)?;

    spicelib::WNFETD(save.RESLUD.as_slice(), 1, &mut save.BEG, &mut save.END, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::SCARDD(0, save.RESLUD.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Reset tol.
    //

    save.TOL = 0.0001;
    spicelib::GFSTOL(save.TOL, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFUDS(
        GFQ,
        GFDECR,
        &save.RELATE,
        save.REFVAL,
        save.ADJUST,
        save.STEP,
        save.CNFINE.as_slice(),
        MAXWIN,
        NWUDS,
        save.WORK.as_slice_mut(),
        save.RESLUD.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.COUNT = 0;
    save.COUNT = spicelib::WNCARD(save.RESLUD.as_slice(), ctx)?;
    testutil::CHCKSI(b"COUNT", save.COUNT, b"!=", 0, 0, OK, ctx)?;

    spicelib::WNFETD(
        save.RESLUD.as_slice(),
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
    save.TOL = CNVTOL;
    spicelib::GFSTOL(save.TOL, ctx)?;

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
