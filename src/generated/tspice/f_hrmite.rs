//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 15;
const TIGHT: f64 = 0.00000000000001;
const MEDIUM: f64 = 0.000000000001;
const LOOSE: f64 = 0.00000000001;

//$Procedure F_HRMITE ( Hermite interpolation )
pub fn F_HRMITE(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut F: f64 = 0.0;
    let mut DF: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut STEP: f64 = 0.0;
    let mut WORK = StackArray2D::<f64, 60>::new(1..=(2 * MAXDEG), 1..=2);
    let mut XVALS = StackArray::<f64, 15>::new(1..=MAXDEG);
    let mut YVALS = StackArray::<f64, 30>::new(1..=(2 * MAXDEG));

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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_HRMITE", ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: HRMINT
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMINT test:  interpolate the constant function y == 1.  The function satisfies y(0) = 1, y\'(0) = 0, y(1) = 1, y\'(1) = 0.  Evaluate the function at x = -5, x = -1, x = 0, x = 1, x = 5.", ctx)?;

    XVALS[1] = 0.0;
    XVALS[2] = 1.0;

    YVALS[1] = 1.0;
    YVALS[2] = 0.0;
    YVALS[3] = 1.0;
    YVALS[4] = 0.0;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-5)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-5)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-1)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-1)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        0.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(0)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(0)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(1)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(5)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(5)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMINT test:  interpolate the linear function y == 1.  The function satisfies y(0) = 0, y\'(0) = 1, y(1) = 1, y\'(1) = 1.  Evaluate the function at x = -5, x = -1, x = 0, x = 1, x = 5.", ctx)?;

    XVALS[1] = 0.0;
    XVALS[2] = 1.0;

    YVALS[1] = 0.0;
    YVALS[2] = 1.0;
    YVALS[3] = 1.0;
    YVALS[4] = 1.0;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-5)", F, b"~/", -5.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-5)", DF, b"~/", 1.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-1)", F, b"~/", -1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-1)", DF, b"~/", 1.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        0.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(0)", F, b"~", 0.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(0)", DF, b"~/", 1.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(1)", DF, b"~/", 1.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(5)", F, b"~/", 5.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(5)", DF, b"~/", 1.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMINT test:  interpolate the 3rd degree polynomial y == -x**3 + 12*x**2 -7*x -6. The function y and its derivative take the values y(-6) = 684, y\'(-6) = -259, values y(-1) = 14,  y\'(-1) = -34.", ctx)?;

    XVALS[1] = -6.0;
    XVALS[2] = -1.0;

    YVALS[1] = 684.0;
    YVALS[2] = -259.0;
    YVALS[3] = 14.0;
    YVALS[4] = -34.0;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -6.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-6)", F, b"~/", YVALS[1], TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-6)", DF, b"~/", YVALS[2], TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-1)", F, b"~/", YVALS[3], TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-1)", DF, b"~/", YVALS[4], TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        2,
        XVALS.as_slice(),
        YVALS.as_slice(),
        6.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(6)", F, b"~/", 168.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(6)", DF, b"~/", 29.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMINT test:  interpolate the 7th degree polynomial y == x**7 + 2*x**2 +5.  The function y(-1) = 6, y\'(-1) = 3, y(0) = 5, y\'(0) = 0, y(3) = 2210, y\'(3) = 5115, y(5) = 78180, y\'(5) = 109395.", ctx)?;

    XVALS[1] = -1.0;
    XVALS[2] = 0.0;
    XVALS[3] = 3.0;
    XVALS[4] = 5.0;

    YVALS[1] = 6.0;
    YVALS[2] = 3.0;
    YVALS[3] = 5.0;
    YVALS[4] = 0.0;
    YVALS[5] = 2210.0;
    YVALS[6] = 5115.0;
    YVALS[7] = 78180.0;
    YVALS[8] = 109395.0;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-1)", F, b"~/", 6.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-1)", DF, b"~/", 3.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        0.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(0)", F, b"~/", 5.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(0)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        2.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(2)", F, b"~/", 141.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(2)", DF, b"~/", 456.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        3.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(3)", F, b"~/", 2210.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(3)", DF, b"~/", 5115.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(5)", F, b"~/", 78180.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(5)", DF, b"~/", 109395.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMINT test:  interpolate the 7th degree polynomial y == x**7 + 2*x**2 +5 at a variety of abscissa values.", ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -15.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-15)", F, b"~/", -170858920.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-15)", DF, b"~/", 79734315.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        100.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(100)", F, b"~/", 100000000020005.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(100)", DF, b"~/", 7000000000400.0, TIGHT, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -1000.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"y(-1000)",
        F,
        b"~/",
        -999999999999998000000.0,
        TIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"y\'(-1000)",
        DF,
        b"~/",
        6999999999999996000.0,
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        2000.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"y(2000)",
        F,
        b"~/",
        128000000000000000000000.0,
        TIGHT,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"y\'(2000)",
        DF,
        b"~/",
        448000000000000000000.0,
        TIGHT,
        OK,
        ctx,
    )?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        0.001,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1.D-3)", F, b"~/", 5.000002, MEDIUM, OK, ctx)?;
    testutil::CHCKSD(
        b"y\'(1.D-3)",
        DF,
        b"~/",
        0.004000000000000007,
        MEDIUM,
        OK,
        ctx,
    )?;

    //
    // *****************************************************************
    //
    // HRMINT error cases.
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMINT error case:  equal abscissa values.", ctx)?;

    XVALS[2] = -1.0;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        3.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(DIVIDEBYZERO)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMINT error case:  number of points = 0.", ctx)?;

    XVALS[2] = 0.0;

    spicelib::HRMINT(
        0,
        XVALS.as_slice(),
        YVALS.as_slice(),
        3.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    //
    // Further HRMINT non-error tests:
    //
    // Using the same polynomial as above
    //
    //         7      2
    //    y = x  + 2*x  + 5
    //
    // define the polynomial on a set of abscissa values having
    // spacing of widely varying magnitudes.
    //
    testutil::TCASE(b"HRMINT test:  interpolate the 7th degree polynomial y = x**7 + 2*x**2 +5.  Use  abscissa points -1000, 0, 1, 1.000001.", ctx)?;

    XVALS[1] = -100.0;
    XVALS[2] = 0.0;
    XVALS[3] = 1.0;
    XVALS[4] = 1.1;

    YVALS[1] = -99999999979995.0;
    YVALS[2] = 6999999999600.0;
    YVALS[3] = 5.0;
    YVALS[4] = 0.0;
    YVALS[5] = 8.0;
    YVALS[6] = 11.0;
    YVALS[7] = 9.368717100000001;
    YVALS[8] = 16.80092700000001;

    //
    // Make sure we can recover the interpolated values.
    //
    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        XVALS[1],
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-100)", F, b"~/", YVALS[1], LOOSE, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-100)", DF, b"~/", YVALS[2], LOOSE, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        XVALS[2],
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(0)", F, b"~/", YVALS[3], LOOSE, OK, ctx)?;
    testutil::CHCKSD(b"y\'(0)", DF, b"~", YVALS[4], LOOSE, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        XVALS[3],
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1)", F, b"~/", YVALS[5], LOOSE, OK, ctx)?;
    testutil::CHCKSD(b"y\'(1)", DF, b"~/", YVALS[6], LOOSE, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        XVALS[4],
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1.1)", F, b"~/", YVALS[7], LOOSE, OK, ctx)?;
    testutil::CHCKSD(b"y\'(1.1)", DF, b"~/", YVALS[8], LOOSE, OK, ctx)?;

    //
    // Try a variety of abscissa values.
    //
    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -15.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-15)", F, b"~/", -170858920.0, LOOSE, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-15)", DF, b"~/", 79734315.0, LOOSE, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        100.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(100)", F, b"~/", 100000000020005.0, LOOSE, OK, ctx)?;
    testutil::CHCKSD(b"y\'(100)", DF, b"~/", 7000000000400.0, LOOSE, OK, ctx)?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        -1000.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"y(-1000)",
        F,
        b"~/",
        -999999999999998000000.0,
        LOOSE,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"y\'(-1000)",
        DF,
        b"~/",
        6999999999999996000.0,
        LOOSE,
        OK,
        ctx,
    )?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        2000.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(
        b"y(2000)",
        F,
        b"~/",
        128000000000000000000000.0,
        LOOSE,
        OK,
        ctx,
    )?;
    testutil::CHCKSD(
        b"y\'(2000)",
        DF,
        b"~/",
        448000000000000000000.0,
        LOOSE,
        OK,
        ctx,
    )?;

    spicelib::HRMINT(
        4,
        XVALS.as_slice(),
        YVALS.as_slice(),
        0.001,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1.D-3)", F, b"~/", 5.000002, LOOSE, OK, ctx)?;
    testutil::CHCKSD(
        b"y\'(1.D-3)",
        DF,
        b"~/",
        0.004000000000000007,
        LOOSE,
        OK,
        ctx,
    )?;

    //
    // *****************************************************************
    //
    // Normal cases: HRMESP
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMESP test:  interpolate the constant function y == 1.  The function satisfies y(0) = 1, y\'(0) = 0, y(1) = 1, y\'(1) = 0.  Evaluate the function at x = -5, x = -1, x = 0, x = 1, x = 5.", ctx)?;

    START = 0.0;
    STEP = 1.0;

    YVALS[1] = 1.0;
    YVALS[2] = 0.0;
    YVALS[3] = 1.0;
    YVALS[4] = 0.0;

    spicelib::HRMESP(
        2,
        START,
        STEP,
        YVALS.as_slice(),
        -5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-5)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-5)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        2,
        START,
        STEP,
        YVALS.as_slice(),
        -1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1-)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-1)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        2,
        START,
        STEP,
        YVALS.as_slice(),
        0.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(0)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(0)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        2,
        START,
        STEP,
        YVALS.as_slice(),
        1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(1)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        2,
        START,
        STEP,
        YVALS.as_slice(),
        5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(5)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(5)", DF, b"~", 0.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMESP test:  interpolate the linear function y == 1.  The function satisfies y(0) = 0, y\'(0) = 1, y(1) = 1, y\'(1) = 1.  Evaluate the function at x = -5, x = -1, x = 0, x = 1, x = 5.", ctx)?;

    START = 0.0;
    STEP = 1.0;

    YVALS[1] = 0.0;
    YVALS[2] = 1.0;
    YVALS[3] = 1.0;
    YVALS[4] = 1.0;

    spicelib::HRMESP(
        2,
        START,
        STEP,
        YVALS.as_slice(),
        -5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-5)", F, b"~/", -5.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-5)", DF, b"~/", 1.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        2,
        START,
        STEP,
        YVALS.as_slice(),
        -1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-1)", F, b"~/", -1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-1)", DF, b"~/", 1.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        2,
        START,
        STEP,
        YVALS.as_slice(),
        0.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(0)", F, b"~", 0.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(0)", DF, b"~/", 1.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        2,
        START,
        STEP,
        YVALS.as_slice(),
        1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1)", F, b"~/", 1.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(1)", DF, b"~/", 1.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        2,
        START,
        STEP,
        YVALS.as_slice(),
        5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(5)", F, b"~/", 5.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(5)", DF, b"~/", 1.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMESP test:  interpolate the 7th degree polynomial y == x**7 + 2*x**2 +5.  The function y(-1) = 6, y\'(-1) = 3, y(1) = 8, y\'(1) = 11, y(3) = 2210, y\'(3) = 5115, y(5) = 78180, y\'(5) = 109395.", ctx)?;

    START = -1.0;
    STEP = 2.0;

    YVALS[1] = 6.0;
    YVALS[2] = 3.0;
    YVALS[3] = 8.0;
    YVALS[4] = 11.0;
    YVALS[5] = 2210.0;
    YVALS[6] = 5115.0;
    YVALS[7] = 78180.0;
    YVALS[8] = 109395.0;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        -1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-1)", F, b"~/", 6.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-1)", DF, b"~/", 3.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1)", F, b"~/", 8.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(1)", DF, b"~", 11.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        2.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(2)", F, b"~/", 141.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(2)", DF, b"~/", 456.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        3.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(3)", F, b"~/", 2210.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(3)", DF, b"~/", 5115.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(5)", F, b"~/", 78180.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(5)", DF, b"~/", 109395.0, TIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMESP test:  interpolate the 7th degree polynomial y == x**7 + 2*x**2 +5.  The function y(-1) = 6, y\'(-1) = 3, y(1) = 8, y\'(1) = 11, y(3) = 2210, y\'(3) = 5115, y(5) = 78180, y\'(5) = 109395.  This time use a NEGATIVE step.", ctx)?;

    START = 5.0;
    STEP = -2.0;

    YVALS[7] = 6.0;
    YVALS[8] = 3.0;
    YVALS[5] = 8.0;
    YVALS[6] = 11.0;
    YVALS[3] = 2210.0;
    YVALS[4] = 5115.0;
    YVALS[1] = 78180.0;
    YVALS[2] = 109395.0;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        -1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(-1)", F, b"~/", 6.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(-1)", DF, b"~/", 3.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        1.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(1)", F, b"~/", 8.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(1)", DF, b"~", 11.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        2.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(2)", F, b"~/", 141.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(2)", DF, b"~/", 456.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        3.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(3)", F, b"~/", 2210.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(3)", DF, b"~/", 5115.0, TIGHT, OK, ctx)?;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"y(5)", F, b"~/", 78180.0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"y\'(5)", DF, b"~/", 109395.0, TIGHT, OK, ctx)?;

    //
    // *****************************************************************
    //
    // HRMESP error cases.
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMESP error case:  step size zero.", ctx)?;

    START = -1.0;
    STEP = 0.0;

    spicelib::HRMESP(
        4,
        START,
        STEP,
        YVALS.as_slice(),
        5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSTEPSIZE)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"HRMESP error case:  number of points = 0.", ctx)?;

    START = -1.0;
    STEP = 2.0;

    spicelib::HRMESP(
        0,
        START,
        STEP,
        YVALS.as_slice(),
        5.0,
        WORK.as_slice_mut(),
        &mut F,
        &mut DF,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSIZE)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
