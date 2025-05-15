//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 15;
const TIGHT: f64 = 0.00000000000001;
const MEDIUM: f64 = 0.000000000001;

fn DF(X: f64) -> f64 {
    (((3.0 * f64::powi(X, 2)) + (4.0 * X)) - 4.0)
}

//$Procedure F_LAGRNG ( Lagrange interpolation )
pub fn F_LAGRNG(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut EVAL: f64 = 0.0;
    let mut FIRST: f64 = 0.0;
    let mut P: f64 = 0.0;
    let mut DP: f64 = 0.0;
    let mut STEP: f64 = 0.0;
    let mut WORK = StackArray2D::<f64, 30>::new(1..=MAXDEG, 1..=2);
    let mut X: f64 = 0.0;
    let mut XVALS = StackArray::<f64, 15>::new(1..=MAXDEG);
    let mut YVALS = StackArray::<f64, 15>::new(1..=MAXDEG);
    let mut N: i32 = 0;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Statement function for (a particular) polynomial derivative:
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_LAGRNG", ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: LGRESP
    //
    // *****************************************************************
    //
    N = 4;
    FIRST = -1.0;
    STEP = 2.0;

    YVALS[1] = -2.0;
    YVALS[2] = -8.0;
    YVALS[3] = 26.0;
    YVALS[4] = 148.0;

    //
    // --- Case: -------------------------------------------------------
    //
    //    The unique cubic polynomial that fits these points is
    //
    //               3      2
    //       f(x) = x  + 2*x  - 4*x - 7
    //
    testutil::TCASE(
        b"LGRESP normal case #1: f(x) = x^3 + 2*x^2 -4*x - 7.  Evaluate at input x=2.",
        ctx,
    )?;

    X = 2.0;
    EVAL = spicelib::LGRESP(
        N,
        FIRST,
        STEP,
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        X,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"EVAL", EVAL, b"~/", 1.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    //    The unique cubic polynomial that fits these points is
    //
    //               3      2
    //       f(x) = x  + 2*x  - 4*x - 7
    //
    testutil::TCASE(b"LGRESP normal case #2: f(x) = x^3 + 2*x^2 -4*x - 7.  Evaluate at FIRST + N * STEP values.", ctx)?;

    for I in 1..=N {
        X = (FIRST + (((I - 1) as f64) * STEP));

        EVAL = spicelib::LGRESP(
            N,
            FIRST,
            STEP,
            YVALS.as_slice(),
            WORK.as_slice_mut(),
            X,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(b"EVAL", EVAL, b"~/", YVALS[I], TIGHT, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRESP normal case #3: linear polynomial.", ctx)?;

    N = 2;

    FIRST = -1.0;
    STEP = 1.0;

    YVALS[1] = -2.0;
    YVALS[2] = -7.0;

    EVAL = spicelib::LGRESP(
        N,
        FIRST,
        STEP,
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        2.0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"Y(2)", EVAL, b"~/", -17.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRESP normal case #4: constant polynomial.", ctx)?;

    N = 1;

    FIRST = -1.0;
    STEP = 3.0;

    YVALS[1] = -2.0;

    EVAL = spicelib::LGRESP(
        N,
        FIRST,
        STEP,
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        2.0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"Y(2)", EVAL, b"~/", -2.0, TIGHT, OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: LGRIND
    //
    // *****************************************************************
    //
    N = 4;

    XVALS[1] = -1 as f64;
    XVALS[2] = 0 as f64;
    XVALS[3] = 1 as f64;
    XVALS[4] = 3 as f64;

    YVALS[1] = -2 as f64;
    YVALS[2] = -7 as f64;
    YVALS[3] = -8 as f64;
    YVALS[4] = 26 as f64;

    //
    // --- Case: -------------------------------------------------------
    //
    //
    //    The unique cubic polynomial that fits these points is
    //
    //               3      2
    //       f(x) = x  + 2*x  - 4*x - 7
    //
    //
    //    The derivative of f(x) is
    //
    //        '         2
    //       f (x) = 3*x  + 4*x - 4
    //
    testutil::TCASE(
        b"LGRIND normal case #1: f(x) = x^3 + 2*x^2 -4*x - 7.  Evaluate at input x values.",
        ctx,
    )?;

    for I in 1..=N {
        X = XVALS[I];

        spicelib::LGRIND(
            N,
            XVALS.as_slice(),
            YVALS.as_slice(),
            WORK.as_slice_mut(),
            X,
            &mut P,
            &mut DP,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(b"y(xvals(i))", P, b"~/", YVALS[I], TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"y\'(xvals(i))", DP, b"~", DF(X), TIGHT, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"LGRIND normal case #2: f(x) = x^3 + 2*x^2 -4*x - 7.  Evaluate at x = 2.",
        ctx,
    )?;

    spicelib::LGRIND(
        N,
        XVALS.as_slice(),
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        2.0,
        &mut P,
        &mut DP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The returned value of P should be 1.D0.
    //
    // The returned value of DP should be 1.6D1.
    //
    testutil::CHCKSD(b"y(2)", P, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(2)", DP, b"~", 16.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRIND normal case #3: linear polynomial.", ctx)?;

    N = 2;

    XVALS[1] = -1 as f64;
    XVALS[2] = 0 as f64;

    YVALS[1] = -2 as f64;
    YVALS[2] = -7 as f64;

    spicelib::LGRIND(
        N,
        XVALS.as_slice(),
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        2.0,
        &mut P,
        &mut DP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"y(2)", P, b"~/", -17.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(2)", DP, b"~", -5.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRIND normal case #4: constant polynomial.", ctx)?;

    N = 1;

    XVALS[1] = -1 as f64;

    YVALS[1] = -2 as f64;

    spicelib::LGRIND(
        N,
        XVALS.as_slice(),
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        2.0,
        &mut P,
        &mut DP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"y(2)", P, b"~/", -2.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(2)", DP, b"~", 0.0, TIGHT, OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: LGRINT
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    //
    //    The unique cubic polynomial that fits these points is
    //
    //               3      2
    //       f(x) = x  + 2*x  - 4*x - 7
    //
    testutil::TCASE(
        b"LGRINT normal case #1: f(x) = x^3 + 2*x^2 -4*x - 7. Evaluate at input x values.",
        ctx,
    )?;

    N = 4;

    XVALS[1] = -1 as f64;
    XVALS[2] = 0 as f64;
    XVALS[3] = 1 as f64;
    XVALS[4] = 3 as f64;

    YVALS[1] = -2 as f64;
    YVALS[2] = -7 as f64;
    YVALS[3] = -8 as f64;
    YVALS[4] = 26 as f64;

    for I in 1..=N {
        X = XVALS[I];

        EVAL = spicelib::LGRINT(
            N,
            XVALS.as_slice(),
            YVALS.as_slice(),
            WORK.as_slice_mut(),
            X,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(b"y(xvals(i))", EVAL, b"~/", YVALS[I], TIGHT, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"LGRINT normal case #2: f(x) = x^3 + 2*x^2 -4*x - 7. Evaluate at x = 2.",
        ctx,
    )?;

    EVAL = spicelib::LGRINT(
        N,
        XVALS.as_slice(),
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        2.0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The returned value of P should be 1.D0.
    //
    testutil::CHCKSD(b"y(2)", EVAL, b"~/", 1.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRINT normal case #3: linear polynomial.", ctx)?;

    N = 2;

    XVALS[1] = -1 as f64;
    XVALS[2] = 0 as f64;

    YVALS[1] = -2 as f64;
    YVALS[2] = -7 as f64;

    EVAL = spicelib::LGRINT(
        N,
        XVALS.as_slice(),
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        2.0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"y(2)", EVAL, b"~/", -17.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRINT normal case #4: constant polynomial.", ctx)?;

    N = 1;

    XVALS[1] = -1 as f64;

    YVALS[1] = -2 as f64;

    EVAL = spicelib::LGRINT(
        N,
        XVALS.as_slice(),
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        2.0,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"y(2)", EVAL, b"~/", -2.0, TIGHT, OK, ctx)?;

    // *****************************************************************
    //
    // Error handling tests follow.
    //
    // *****************************************************************
    //
    // LGRESP error cases.
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRESP error case:  array size non-positive.", ctx)?;

    EVAL = spicelib::LGRESP(
        0,
        FIRST,
        STEP,
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        X,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;
    testutil::CHCKSD(b"EVAL", EVAL, b"=", 0.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRESP error case:  step zero.", ctx)?;

    EVAL = spicelib::LGRESP(N, FIRST, 0.0, YVALS.as_slice(), WORK.as_slice_mut(), X, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSTEPSIZE)", OK, ctx)?;
    testutil::CHCKSD(b"EVAL", EVAL, b"=", 0.0, TIGHT, OK, ctx)?;

    //
    // LGRIND error cases.
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRIND error case:  equal abscissa values.", ctx)?;

    XVALS[2] = -1.0;

    spicelib::LGRIND(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        3.0,
        &mut P,
        &mut DP,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DIVIDEBYZERO)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRIND error case:  array size non-positive.", ctx)?;

    spicelib::LGRIND(
        0,
        XVALS.as_slice(),
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        3.0,
        &mut P,
        &mut DP,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    //
    // LGRINT error cases.
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRINT error case:  equal abscissa values.", ctx)?;

    XVALS[2] = -1.0;

    EVAL = spicelib::LGRINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        X,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DIVIDEBYZERO)", OK, ctx)?;
    testutil::CHCKSD(b"EVAL", EVAL, b"=", 0.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"LGRINT error case:  array size non-positive.", ctx)?;

    EVAL = spicelib::LGRINT(
        0,
        XVALS.as_slice(),
        YVALS.as_slice(),
        WORK.as_slice_mut(),
        X,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;
    testutil::CHCKSD(b"EVAL", EVAL, b"=", 0.0, TIGHT, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
