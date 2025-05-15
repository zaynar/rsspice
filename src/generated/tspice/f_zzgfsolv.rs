//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const MAXWIN: i32 = 100;

//$Procedure F_ZZGFSOLV ( ZZGFSOLV family tests )
pub fn F_ZZGFSOLV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut STEP: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut FINISH: f64 = 0.0;
    let mut TBEG: f64 = 0.0;
    let mut TEND: f64 = 0.0;
    let mut TOLRNC: f64 = 0.0;
    let mut CRIT: f64 = 0.0;
    let mut RESULT = StackArray::<f64, 106>::new(LBCELL..=MAXWIN);
    let mut BAIL: bool = false;
    let mut CSTEP: bool = false;
    let mut RPT: bool = false;
    let mut N: i32 = 0;
    let mut NRPT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // External routines
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZGFSOLV", ctx)?;

    //
    // Set the progress report test hit counter to zero.
    //
    ZZRPTI(ctx);

    //
    // Case 1
    //
    testutil::TCASE(b"TOLRNC = 0", ctx)?;

    BAIL = false;
    CSTEP = true;
    RPT = false;
    STEP = 1.0;
    START = 0.0;
    FINISH = 10.0;
    TOLRNC = 0.0;

    spicelib::ZZGFSOLV(
        T_UDQLT,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        BAIL,
        spicelib::GFBAIL,
        CSTEP,
        STEP,
        START,
        FINISH,
        TOLRNC,
        RPT,
        spicelib::GFREPU,
        RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDTOLERANCE)", OK, ctx)?;

    //
    // Case 2
    //
    testutil::TCASE(b"START > FINISH", ctx)?;

    BAIL = false;
    CSTEP = true;
    RPT = false;
    STEP = 1.0;
    START = 10.0;
    FINISH = 0.0;
    TOLRNC = 1.0;

    spicelib::ZZGFSOLV(
        T_UDQLT,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        BAIL,
        spicelib::GFBAIL,
        CSTEP,
        STEP,
        START,
        FINISH,
        TOLRNC,
        RPT,
        spicelib::GFREPU,
        RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADTIMECASE)", OK, ctx)?;

    //
    // Case 3
    //
    testutil::TCASE(b"Step function, one interval, one critical value.", ctx)?;

    //
    // Set a search to key on the critical value defined in
    // T_ZZGFSOLV. With a STEP size of one, and a critical value
    // a double precisiion representation of an integer, the
    // RESULT window returns [CRIT + TOLRNC/2, FINISH].
    //
    //                        ------------
    //                        |
    //       -----------------|
    //

    spicelib::SSIZED(MAXWIN, RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    BAIL = false;
    CSTEP = true;
    RPT = true;
    STEP = 1.0;
    START = 0.0;
    FINISH = 100.0;

    //
    // Set a value strictly between START and FINISH.
    //
    ZZCRITS(93.0, ctx);

    //
    // Using bisection for refinement; set TOLRNC as a power of 2.
    //
    // TOLRNC = 2**-N (N=3)
    //
    TOLRNC = 0.125;

    spicelib::ZZGFSOLV(
        T_UDQLT,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        BAIL,
        spicelib::GFBAIL,
        CSTEP,
        STEP,
        START,
        FINISH,
        TOLRNC,
        RPT,
        T_REPU,
        RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect one root.
    //
    N = spicelib::WNCARD(RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    spicelib::WNFETD(RESULT.as_slice(), N, &mut TBEG, &mut TEND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We know the exact values for the expected interval endpoints.
    //
    ZZCRITG(&mut CRIT, ctx);

    testutil::CHCKSD(b"TBEG", TBEG, b"=", (CRIT + (TOLRNC / 2.0)), 0.0, OK, ctx)?;
    testutil::CHCKSD(b"TBEG", TEND, b"=", FINISH, 0.0, OK, ctx)?;

    //
    // We know the exact value for the number of report hits.
    //
    ZZNRPTG(&mut NRPT, ctx);

    testutil::CHCKSI(b"NRPT", NRPT, b"=", 101, 0, OK, ctx)?;

    //
    // Case 4
    //
    testutil::TCASE(b"Step function, one interval, two critical values.", ctx)?;

    //
    //                  -----------------
    //                  |               |
    // -----------------|               |--
    //                 CRIT           2*CRIT
    //
    spicelib::SCARDD(0, RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    BAIL = false;
    CSTEP = true;
    RPT = true;
    STEP = 1.0;
    START = 0.0;
    FINISH = 200.0;

    //
    // Set a value strictly between START and FINISH with
    // 2*CRIT + TOLRNC/2.D0 < FINISH.
    //
    ZZCRITS(93.0, ctx);

    //
    // Using bisection for refinement; set TOLRNC as a power of 2.
    //
    // TOLRNC = 2**-N (N=3)
    //
    TOLRNC = 0.125;

    spicelib::ZZGFSOLV(
        T_UDQLT,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        BAIL,
        spicelib::GFBAIL,
        CSTEP,
        STEP,
        START,
        FINISH,
        TOLRNC,
        RPT,
        T_REPU,
        RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We expect one root.
    //
    N = spicelib::WNCARD(RESULT.as_slice(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"N", N, b"=", 1, 0, OK, ctx)?;

    spicelib::WNFETD(RESULT.as_slice(), N, &mut TBEG, &mut TEND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // We know the exact values for the expected interval endpoints.
    //
    ZZCRITG(&mut CRIT, ctx);

    testutil::CHCKSD(b"TBEG", TBEG, b"=", (CRIT + (TOLRNC / 2.0)), 0.0, OK, ctx)?;
    testutil::CHCKSD(
        b"TBEG",
        TEND,
        b"=",
        ((2.0 * CRIT) + (TOLRNC / 2.0)),
        0.0,
        OK,
        ctx,
    )?;

    //
    // We know the exact value for the number of report hits.
    //
    ZZNRPTG(&mut NRPT, ctx);
    testutil::CHCKSI(b"NRPT", NRPT, b"=", 200, 0, OK, ctx)?;

    //
    // Case 5
    //
    testutil::TCASE(b"Check for loop convergence error signal.", ctx)?;

    spicelib::SSIZED(MAXWIN, RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    BAIL = false;
    CSTEP = true;
    RPT = true;
    STEP = 1.0;
    START = 0.0;
    FINISH = 100.0;

    //
    // Set a value strictly between START and FINISH.
    //
    ZZCRITS(93.0, ctx);

    //
    // Set TOLRNC to something we know will work for bisection.
    //
    TOLRNC = 0.125;

    //
    // Use a test version of GFREFN to ensure a slower than
    // bisection convergence rate.
    //
    spicelib::ZZGFSOLV(
        T_UDQLT,
        spicelib::GFSTEP,
        T_REFN,
        BAIL,
        spicelib::GFBAIL,
        CSTEP,
        STEP,
        START,
        FINISH,
        TOLRNC,
        RPT,
        T_REPU,
        RESULT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(NOCONVERG)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
