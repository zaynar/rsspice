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
const LBCELL: i32 = -5;
const MAXWIN: i32 = 50;
const LNSIZE: i32 = 80;

struct SaveVars {
    CNFINE: StackArray<f64, 8>,
    RESULT: StackArray<f64, 56>,
    LHS: StackArray<f64, 3>,
    RHS: StackArray<f64, 3>,
    LEFT: f64,
    RIGHT: f64,
    STEP: f64,
    BEG: f64,
    END: f64,
    BADSIZ: StackArray<i32, 3>,
    COUNT: i32,
    MSG1: Vec<u8>,
    MSG2: Vec<u8>,
    TITLE: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CNFINE = StackArray::<f64, 8>::new(LBCELL..=2);
        let mut RESULT = StackArray::<f64, 56>::new(LBCELL..=MAXWIN);
        let mut LHS = StackArray::<f64, 3>::new(1..=3);
        let mut RHS = StackArray::<f64, 3>::new(1..=3);
        let mut LEFT: f64 = 0.0;
        let mut RIGHT: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut BEG: f64 = 0.0;
        let mut END: f64 = 0.0;
        let mut BADSIZ = StackArray::<i32, 3>::new(1..=3);
        let mut COUNT: i32 = 0;
        let mut MSG1 = vec![b' '; LNSIZE as usize];
        let mut MSG2 = vec![b' '; LNSIZE as usize];
        let mut TITLE = vec![b' '; LNSIZE as usize];

        Self {
            CNFINE,
            RESULT,
            LHS,
            RHS,
            LEFT,
            RIGHT,
            STEP,
            BEG,
            END,
            BADSIZ,
            COUNT,
            MSG1,
            MSG2,
            TITLE,
        }
    }
}

//$Procedure F_GFUDB ( GFUDB family tests )
pub fn F_GFUDB(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_GFUDB", ctx)?;

    save.BADSIZ[1] = 0;
    save.BADSIZ[2] = 1;
    save.BADSIZ[3] = 3;

    //
    // The event boundaries as defined in GFB.
    //
    save.LHS[1] = 20.0;
    save.LHS[2] = 55.0;
    save.LHS[3] = 92.0;

    save.RHS[1] = 30.0;
    save.RHS[2] = 57.0;
    save.RHS[3] = 100.0;

    //
    // Error cases
    //

    //
    // Case 1:
    //
    // Confirm bad window sizes cause the expected error.
    //

    save.STEP = spicelib::SPD();

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, save.CNFINE.as_slice_mut(), ctx)?;

    fstr::assign(&mut save.MSG1, b"Invalid result window size #");

    for I in 1..=3 {
        spicelib::REPMI(
            &save.MSG1.to_vec(),
            b"#",
            save.BADSIZ[I],
            &mut save.MSG1,
            ctx,
        );

        testutil::TCASE(&save.MSG1, ctx)?;

        spicelib::SSIZED(save.BADSIZ[I], save.RESULT.as_slice_mut(), ctx)?;

        spicelib::GFUDB(
            spicelib::UDF,
            GFB,
            save.STEP,
            save.CNFINE.as_slice(),
            save.RESULT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(true, b"SPICE(INVALIDDIMENSION)", OK, ctx)?;
    }

    //
    // Setp appropriate sizes for the windows.
    //
    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    spicelib::SSIZED(2, save.CNFINE.as_slice_mut(), ctx)?;

    //
    // Case 2:
    //
    // Confirm non-positive step causes the expected error.
    //
    testutil::TCASE(b"Non-positive step size.", ctx)?;

    save.STEP = -1.0;
    spicelib::GFUDB(
        spicelib::UDF,
        GFB,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    save.STEP = 0.0;
    spicelib::GFUDB(
        spicelib::UDF,
        GFB,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    // Standard cases.
    //

    //
    // Case 3:
    //
    // Test output from use of GFB
    //

    //
    // Store the time bounds of our search interval in
    // the confinement window.
    //
    spicelib::WNINSD(-5.0, 105.0, save.CNFINE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 1.9;

    spicelib::GFUDB(
        spicelib::UDF,
        GFB,
        save.STEP,
        save.CNFINE.as_slice(),
        save.RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect 3 intervals in RESULT.
    //
    testutil::CHCKSI(
        b"WNCARD( RESULT )",
        spicelib::WNCARD(save.RESULT.as_slice(), ctx)?,
        b"=",
        3,
        0,
        OK,
        ctx,
    )?;

    if *OK {
        for J in 1..=spicelib::WNCARD(save.RESULT.as_slice(), ctx)? {
            //
            // Fetch the endpoints of the Jth interval of the result
            // window. The window interval values should equal the
            // expected values (LHS, RHS) to within CNVTOL.
            //
            spicelib::WNFETD(
                save.RESULT.as_slice(),
                J,
                &mut save.LEFT,
                &mut save.RIGHT,
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            fstr::assign(&mut save.MSG1, b"GFB use output, LHS #");
            spicelib::REPMI(&save.MSG1.to_vec(), b"#", J, &mut save.MSG1, ctx);

            testutil::TCASE(&save.MSG1, ctx)?;
            testutil::CHCKSD(&save.MSG1, save.LEFT, b"~", save.LHS[J], CNVTOL, OK, ctx)?;

            fstr::assign(&mut save.MSG2, b"GFB use output, RHS #");
            spicelib::REPMI(&save.MSG2.to_vec(), b"#", J, &mut save.MSG2, ctx);

            testutil::TCASE(&save.MSG2, ctx)?;
            testutil::CHCKSD(&save.MSG2, save.RIGHT, b"~", save.RHS[J], CNVTOL, OK, ctx)?;
        }
    }

    //
    // Case 4
    //
    fstr::assign(&mut save.TITLE, b"Check the GF call uses the GFSTOL value");
    testutil::TCASE(&save.TITLE, ctx)?;

    //
    // Re-run a valid search after using GFSTOL to set the convergence
    // tolerance to a value that should cause a numerical error signal.
    //

    save.STEP = 1.0;

    spicelib::SCARDD(0, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFUDB(
        spicelib::UDF,
        GFB,
        save.STEP,
        save.CNFINE.as_slice(),
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

    spicelib::GFUDB(
        spicelib::UDF,
        GFB,
        save.STEP,
        save.CNFINE.as_slice(),
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

    Ok(())
}
