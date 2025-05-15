//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const DM16: f64 = 0.0000000000000001;
const DM11: f64 = 0.00000000001;
const DM8: f64 = 0.00000001;
const MAXN: i32 = 2;

//$Procedure F_QDERIV ( QDERIV tests )
pub fn F_QDERIV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DELTA: f64 = 0.0;
    let mut EPSLON: f64 = 0.0;
    let mut F0 = StackArray::<f64, 2>::new(1..=MAXN);
    let mut F2 = StackArray::<f64, 2>::new(1..=MAXN);
    let mut DFDT = StackArray::<f64, 2>::new(1..=MAXN);
    let mut SCALE: f64 = 0.0;
    let mut X0: f64 = 0.0;
    let mut XD = StackArray::<f64, 2>::new(1..=MAXN);
    let mut N: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Tolerance levels for various tests.
    //

    //
    // Local Variables
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_QDERIV", ctx)?;

    //
    // Compute machine epsilon and the horizontal scale
    // factor required for determining an appropriate value
    // of DELTA.
    //
    EPSLON = f64::powi(2.0, -53);

    SCALE = f64::powf(EPSLON, (1.0 / 3.0));

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    // --- Case 1 ------------------------------------------------------
    //
    testutil::TCASE(b"Estimate the derivative of sin(x) at x = 0.", ctx)?;

    N = 1;
    DELTA = 0.000001;

    F0[1] = f64::sin(-DELTA);
    F2[1] = f64::sin(DELTA);

    spicelib::QDERIV(
        N,
        F0.as_slice(),
        F2.as_slice(),
        DELTA,
        DFDT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XD[1] = 1.0;

    testutil::CHCKSD(b"d(sin(x))/dt at x=0", DFDT[1], b"~", XD[1], DM11, OK, ctx)?;

    //
    // --- Case 2 ------------------------------------------------------
    //
    testutil::TCASE(b"Estimate the derivative of sin(x) at x = pi/4.", ctx)?;

    N = 1;
    X0 = (spicelib::PI(ctx) / 4 as f64);
    DELTA = (X0 * SCALE);

    F0[1] = f64::sin((X0 - DELTA));
    F2[1] = f64::sin((X0 + DELTA));

    spicelib::QDERIV(
        N,
        F0.as_slice(),
        F2.as_slice(),
        DELTA,
        DFDT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XD[1] = f64::cos(X0);

    testutil::CHCKSD(
        b"d(sin(x))/dt at x = pi/4",
        DFDT[1],
        b"~",
        XD[1],
        DM11,
        OK,
        ctx,
    )?;

    //
    // --- Case 3 ------------------------------------------------------
    //
    testutil::TCASE(b"Estimate the derivative of x2 at x = 2.", ctx)?;

    N = 1;
    DELTA = 0.001;

    F0[1] = f64::powf((2.0 - DELTA), 2.0);
    F2[1] = f64::powf((2.0 + DELTA), 2.0);

    spicelib::QDERIV(
        N,
        F0.as_slice(),
        F2.as_slice(),
        DELTA,
        DFDT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XD[1] = 4.0;

    testutil::CHCKSD(b"d(x**2)/dt at x=2", DFDT[1], b"~", XD[1], DM11, OK, ctx)?;

    //
    // --- Case 4 ------------------------------------------------------
    //
    testutil::TCASE(b"Vector case: combine the previous cases.", ctx)?;

    N = 2;
    DELTA = 0.000001;

    F0[1] = f64::sin(-DELTA);
    F2[1] = f64::sin(DELTA);

    F0[2] = f64::powf((2.0 - DELTA), 2.0);
    F2[2] = f64::powf((2.0 + DELTA), 2.0);

    spicelib::QDERIV(
        N,
        F0.as_slice(),
        F2.as_slice(),
        DELTA,
        DFDT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    XD[1] = 1.0;
    XD[2] = 4.0;

    testutil::CHCKAD(
        b"Vector case: combine previous cases",
        DFDT.as_slice(),
        b"~",
        XD.as_slice(),
        2,
        DM8,
        OK,
        ctx,
    )?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    // --- Case 5 ------------------------------------------------------
    //
    testutil::TCASE(b"DELTA == 0", ctx)?;

    spicelib::QDERIV(
        N,
        F0.as_slice(),
        F2.as_slice(),
        0.0,
        DFDT.as_slice_mut(),
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(DIVIDEBYZERO)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
