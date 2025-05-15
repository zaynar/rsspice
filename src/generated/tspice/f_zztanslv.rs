//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const MAXIVL: i32 = 1000;
const MAXWIN: i32 = (2 * MAXIVL);

struct SaveVars {
    CNVTOL: f64,
    FINISH: f64,
    POINTS: ActualArray2D<f64>,
    START: f64,
    STEP: f64,
    TOL: f64,
    RESULT: ActualArray<f64>,
    XRESLT: ActualArray<f64>,
    XPNTS: ActualArray2D<f64>,
    N: i32,
    XN: i32,
    CSTEP: bool,
    ENDFLG: StackArray<bool, 2>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CNVTOL: f64 = 0.0;
        let mut FINISH: f64 = 0.0;
        let mut POINTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXWIN);
        let mut START: f64 = 0.0;
        let mut STEP: f64 = 0.0;
        let mut TOL: f64 = 0.0;
        let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXWIN);
        let mut XRESLT = ActualArray::<f64>::new(1..=MAXWIN);
        let mut XPNTS = ActualArray2D::<f64>::new(1..=3, 1..=MAXWIN);
        let mut N: i32 = 0;
        let mut XN: i32 = 0;
        let mut CSTEP: bool = false;
        let mut ENDFLG = StackArray::<bool, 2>::new(1..=2);

        Self {
            CNVTOL,
            FINISH,
            POINTS,
            START,
            STEP,
            TOL,
            RESULT,
            XRESLT,
            XPNTS,
            N,
            XN,
            CSTEP,
            ENDFLG,
        }
    }
}

//$Procedure F_ZZTANSLV ( ZZTANSLV tests )
pub fn F_ZZTANSLV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // EXTERNAL routines
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved values
    //
    // Save variables in order to avoid stack room problems.
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_ZZTANSLV", ctx)?;

    //**********************************************************************
    //
    //     ZZTANSLV normal cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Find all state transitions on interval [1,10]. Use constant step of 0.5.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 1.0;
    save.FINISH = 10.0;
    save.CNVTOL = 0.000000000001;

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intervals found.
    //
    save.XN = 5;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the interval endpoints.
    //
    for I in 1..=(2 * save.XN) {
        save.XRESLT[I] = (I as f64);
    }

    save.TOL = ((2 as f64) * save.CNVTOL);

    testutil::CHCKAD(
        b"RESULT",
        save.RESULT.subarray(1),
        b"~",
        save.XRESLT.as_slice(),
        (2 * save.XN),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the endpoint flags. The first and last endpoints are
    // not considered to be state transitions.
    //
    testutil::CHCKSL(b"First endpoint flag", save.ENDFLG[1], false, OK, ctx)?;
    testutil::CHCKSL(b"Last endpoint flag", save.ENDFLG[2], false, OK, ctx)?;

    //
    // Check the returned points. The values are defined only for
    // transitions, so the first and last points in the array are
    // not checked.
    //
    for I in intrinsics::range(1, (2 * save.XN), 2) {
        for J in 1..=3 {
            save.XPNTS[[J, I]] = (((I + J) - 1) as f64);
            save.XPNTS[[J, (I + 1)]] = save.XPNTS[[J, I]];
        }
    }

    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.subarray([1, 2]),
        b"~",
        save.XPNTS.subarray([1, 2]),
        (3 * ((2 * save.XN) - 2)),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find all state transitions on interval [1,10]. Use a step of 0.5, but use the GF step callback.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = false;

    spicelib::GFSSTP(0.5, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 0.5;
    save.START = 1.0;
    save.FINISH = 10.0;
    save.CNVTOL = 0.000000000001;

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intervals found.
    //
    save.XN = 5;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the interval endpoints.
    //
    for I in 1..=(2 * save.XN) {
        save.XRESLT[I] = (I as f64);
    }

    save.TOL = ((2 as f64) * save.CNVTOL);

    testutil::CHCKAD(
        b"RESULT",
        save.RESULT.subarray(1),
        b"~",
        save.XRESLT.as_slice(),
        (2 * save.XN),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the endpoint flags. The first and last endpoints are
    // not considered to be state transitions.
    //
    testutil::CHCKSL(b"First endpoint flag", save.ENDFLG[1], false, OK, ctx)?;
    testutil::CHCKSL(b"Last endpoint flag", save.ENDFLG[2], false, OK, ctx)?;

    //
    // Check the returned points. The values are defined only for
    // transitions, so the first and last points in the array are
    // not checked.
    //
    for I in intrinsics::range(1, (2 * save.XN), 2) {
        for J in 1..=3 {
            save.XPNTS[[J, I]] = (((I + J) - 1) as f64);
            save.XPNTS[[J, (I + 1)]] = save.XPNTS[[J, I]];
        }
    }

    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.subarray([1, 2]),
        b"~",
        save.XPNTS.subarray([1, 2]),
        (3 * ((2 * save.XN) - 2)),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Find all state transitions on interval [1,10]. Use constant step of 0.99.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.99;
    save.START = 1.0;
    save.FINISH = 10.0;
    save.CNVTOL = 0.000000000001;

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intervals found.
    //
    save.XN = 5;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the interval endpoints.
    //
    for I in 1..=(2 * save.XN) {
        save.XRESLT[I] = (I as f64);
    }

    save.TOL = ((2 as f64) * save.CNVTOL);

    testutil::CHCKAD(
        b"RESULT",
        save.RESULT.subarray(1),
        b"~",
        save.XRESLT.as_slice(),
        (2 * save.XN),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the endpoint flags. The first and last endpoints are
    // not considered to be state transitions.
    //
    testutil::CHCKSL(b"First endpoint flag", save.ENDFLG[1], false, OK, ctx)?;
    testutil::CHCKSL(b"Last endpoint flag", save.ENDFLG[2], false, OK, ctx)?;

    //
    // Check the returned points. The values are defined only for
    // transitions, so the first and last points in the array are
    // not checked.
    //
    for I in intrinsics::range(1, (2 * save.XN), 2) {
        for J in 1..=3 {
            save.XPNTS[[J, I]] = (((I + J) - 1) as f64);
            save.XPNTS[[J, (I + 1)]] = save.XPNTS[[J, I]];
        }
    }

    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.subarray([1, 2]),
        b"~",
        save.XPNTS.subarray([1, 2]),
        (3 * ((2 * save.XN) - 2)),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Find all state transitions on interval [0.5,10.5]. Use constant step of 0.5.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 0.5;
    save.FINISH = 10.5;
    save.CNVTOL = 0.000000000001;

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intervals found.
    //
    save.XN = 5;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the interval endpoints.
    //
    for I in 1..=(2 * save.XN) {
        save.XRESLT[I] = (I as f64);
    }

    save.TOL = ((2 as f64) * save.CNVTOL);

    testutil::CHCKAD(
        b"RESULT",
        save.RESULT.subarray(1),
        b"~",
        save.XRESLT.as_slice(),
        (2 * save.XN),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the endpoint flags. The first and last endpoints ARE
    // not considered to be state transitions.
    //
    testutil::CHCKSL(b"First endpoint flag", save.ENDFLG[1], true, OK, ctx)?;
    testutil::CHCKSL(b"Last endpoint flag", save.ENDFLG[2], true, OK, ctx)?;

    //
    // Check the returned points. All of the points should be valid.
    //
    for I in intrinsics::range(1, (2 * save.XN), 2) {
        for J in 1..=3 {
            save.XPNTS[[J, I]] = (((I + J) - 1) as f64);
            save.XPNTS[[J, (I + 1)]] = save.XPNTS[[J, I]];
        }
    }

    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.as_slice(),
        b"~",
        save.XPNTS.as_slice(),
        (6 * save.XN),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Find all state transitions on interval [0.5,10.5]. Use a step of 0.5, but use the GF step callback.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = false;

    spicelib::GFSSTP(0.5, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.STEP = 0.5;
    save.START = 0.5;
    save.FINISH = 10.5;
    save.CNVTOL = 0.000000000001;

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intervals found.
    //
    save.XN = 5;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the interval endpoints.
    //
    for I in 1..=(2 * save.XN) {
        save.XRESLT[I] = (I as f64);
    }

    save.TOL = ((2 as f64) * save.CNVTOL);

    testutil::CHCKAD(
        b"RESULT",
        save.RESULT.subarray(1),
        b"~",
        save.XRESLT.as_slice(),
        (2 * save.XN),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the endpoint flags. The first and last endpoints ARE
    // not considered to be state transitions.
    //
    testutil::CHCKSL(b"First endpoint flag", save.ENDFLG[1], true, OK, ctx)?;
    testutil::CHCKSL(b"Last endpoint flag", save.ENDFLG[2], true, OK, ctx)?;

    //
    // Check the returned points. All of the points should be valid.
    //
    for I in intrinsics::range(1, (2 * save.XN), 2) {
        for J in 1..=3 {
            save.XPNTS[[J, I]] = (((I + J) - 1) as f64);
            save.XPNTS[[J, (I + 1)]] = save.XPNTS[[J, I]];
        }
    }

    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.as_slice(),
        b"~",
        save.XPNTS.as_slice(),
        (6 * save.XN),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(
        b"Find all state transitions on interval [0.5,10.5]. Use constant step of 0.99.",
        ctx,
    )?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.99;
    save.START = 0.5;
    save.FINISH = 10.5;
    save.CNVTOL = 0.000000000001;

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intervals found.
    //
    save.XN = 5;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    //
    // Check the interval endpoints.
    //
    for I in 1..=(2 * save.XN) {
        save.XRESLT[I] = (I as f64);
    }

    save.TOL = ((2 as f64) * save.CNVTOL);

    testutil::CHCKAD(
        b"RESULT",
        save.RESULT.subarray(1),
        b"~",
        save.XRESLT.as_slice(),
        (2 * save.XN),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // Check the endpoint flags. The first and last endpoints ARE
    // not considered to be state transitions.
    //
    testutil::CHCKSL(b"First endpoint flag", save.ENDFLG[1], true, OK, ctx)?;
    testutil::CHCKSL(b"Last endpoint flag", save.ENDFLG[2], true, OK, ctx)?;

    //
    // Check the returned points. All of the points should be valid.
    //
    for I in intrinsics::range(1, (2 * save.XN), 2) {
        for J in 1..=3 {
            save.XPNTS[[J, I]] = (((I + J) - 1) as f64);
            save.XPNTS[[J, (I + 1)]] = save.XPNTS[[J, I]];
        }
    }

    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.as_slice(),
        b"~",
        save.XPNTS.as_slice(),
        (6 * save.XN),
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look for state transition on interval [2.0,2.5]. Use constant step of 0.5. A transition should be found. The first interval endpoint should be considered a non-transition endpoint, while the second one should be considered a transition.", ctx)?;

    //
    // In this case the state function is .TRUE. at X = 1.0 and
    // nowhere else.
    //

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 2.0;
    save.FINISH = 2.5;
    save.CNVTOL = 0.000000000001;

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intervals found. We should have a transition
    // at the right endpoint of the search interval.
    //
    save.XN = 1;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    testutil::CHCKSL(b"First endpoint flag", save.ENDFLG[1], false, OK, ctx)?;

    //
    // The last endpoint flag should indicate a transition.
    //
    testutil::CHCKSL(b"Last endpoint flag", save.ENDFLG[2], true, OK, ctx)?;

    //
    // Check the result window. The second endpoint is determined
    // by the convergence algorithm. We expect the actual result
    // only to be close to the expected value.
    //
    save.XRESLT[1] = save.START;
    save.XRESLT[2] = (save.START + (save.CNVTOL / 2 as f64));

    save.TOL = ((2 as f64) * save.CNVTOL);

    testutil::CHCKAD(
        b"RESULT",
        save.RESULT.subarray(1),
        b"~",
        save.XRESLT.as_slice(),
        2,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The point value at X = 2 is the same as at X = 1.
    //
    spicelib::VPACK(1.0, 2.0, 3.0, save.XPNTS.as_slice_mut());

    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.subarray([1, 2]),
        b"~~",
        save.XPNTS.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Look for state transition on interval [2.5,3.0]. Use constant step of 0.5. A transition should be found. The first interval endpoint should be considered a transition endpoint, while the second one should NOT be considered a transition.", ctx)?;

    //
    // In this case the state function is .TRUE. at X = 1.0 and
    // nowhere else.
    //

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 2.5;
    save.FINISH = 3.0;
    save.CNVTOL = 0.000000000001;

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check the number of intervals found. We should have a transition
    // at the right endpoint of the search interval.
    //
    save.XN = 1;
    save.N = spicelib::WNCARD(save.RESULT.as_slice(), ctx)?;

    testutil::CHCKSI(b"N", save.N, b"=", save.XN, 0, OK, ctx)?;

    testutil::CHCKSL(b"First endpoint flag", save.ENDFLG[1], true, OK, ctx)?;

    //
    // The last endpoint flag should indicate a transition.
    //
    testutil::CHCKSL(b"Last endpoint flag", save.ENDFLG[2], false, OK, ctx)?;

    //
    // Check the result window. The second endpoint is determined
    // by the convergence algorithm. We expect the actual result
    // only to be close to the expected value.
    //
    save.XRESLT[1] = (save.FINISH - (save.CNVTOL / 2 as f64));
    save.XRESLT[2] = save.FINISH;

    save.TOL = ((2 as f64) * save.CNVTOL);

    testutil::CHCKAD(
        b"RESULT",
        save.RESULT.subarray(1),
        b"~",
        save.XRESLT.as_slice(),
        2,
        save.TOL,
        OK,
        ctx,
    )?;

    //
    // The point value should correspond to the left endpoint.
    //
    spicelib::VPACK(3.0, 4.0, 5.0, save.XPNTS.as_slice_mut());

    testutil::CHCKAD(
        b"POINTS",
        save.POINTS.as_slice(),
        b"~~",
        save.XPNTS.as_slice(),
        3,
        save.TOL,
        OK,
        ctx,
    )?;

    //**********************************************************************
    //
    //     ZZTANSLV Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive tolerance.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 2.5;
    save.FINISH = 3.0;
    save.CNVTOL = 0.0;

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDTOLERANCE)", OK, ctx)?;

    save.CNVTOL = -0.000000000001;

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDTOLERANCE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Tolerance too small compared to START.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 2.5;
    save.FINISH = 3.0;

    save.CNVTOL = (0.00000000000000000001 * save.START);

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDTOLERANCE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Tolerance too small compared to FINISH.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 2.5;
    save.FINISH = 3.0;

    save.CNVTOL = (0.00000000000000000001 * save.FINISH);

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDTOLERANCE)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-positive constant step.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.0;
    save.START = 2.5;
    save.FINISH = 3.0;

    save.CNVTOL = (0.001 * save.FINISH);

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCONSTSTEP)", OK, ctx)?;

    save.STEP = -1.0;

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCONSTSTEP)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Constant step is positive but too small.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.START = 2.5;
    save.FINISH = 3.0;

    save.STEP = (0.0000000000000000001 * save.FINISH);

    save.CNVTOL = (0.001 * save.FINISH);

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCONSTSTEP)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"START > FINISH.", ctx)?;

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 2.5;
    save.FINISH = (save.START - 1.0);

    save.CNVTOL = 0.000000000001;

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BOUNDSOUTOFORDER)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"START = FINISH.", ctx)?;
    //
    // This is a non-error exception.
    //

    spicelib::SSIZED(MAXWIN, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 2.5;
    save.FINISH = save.START;

    save.CNVTOL = 0.000000000001;

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Points array too small.", ctx)?;

    spicelib::SSIZED(8, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 0.5;
    save.FINISH = 10.5;
    save.CNVTOL = 0.000000000001;

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ARRAYTOOSMALL)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Result window too small.", ctx)?;

    spicelib::SSIZED(9, save.RESULT.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    save.CSTEP = true;
    save.STEP = 0.5;
    save.START = 0.5;
    save.FINISH = 10.5;
    save.CNVTOL = 0.000000000001;

    spicelib::CLEARD((3 * MAXWIN), save.POINTS.as_slice_mut());

    spicelib::ZZTANSLV(
        T_TANSTA,
        spicelib::GFSTEP,
        spicelib::GFREFN,
        save.CSTEP,
        save.STEP,
        save.START,
        save.FINISH,
        save.CNVTOL,
        save.RESULT.as_slice_mut(),
        save.POINTS.as_slice_mut(),
        save.ENDFLG.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(WINDOWEXCESS)", OK, ctx)?;

    //
    // End of error cases.
    //

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
