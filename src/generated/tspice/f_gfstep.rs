//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      F_GFSTEP ( Test GFSTEP )
pub fn F_GFSTEP(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ET: f64 = 0.0;
    let mut STEP: f64 = 0.0;
    let mut XSTEP: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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
    testutil::TOPEN(b"F_GFSTEP", ctx)?;

    //*********************************************************************
    //*
    //*    Error cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //

    // CALL TCASE ( 'Call GFSTEP while step is uninitialized.' )

    // CALL GFSTEP ( 0.D0, STEP )

    // CALL CHCKXC ( .TRUE., 'SPICE(NOTINITIALIZED)', OK )

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Set zero step.", ctx)?;

    //
    // Give ET an initial value.
    //
    ET = 1000000000.0;

    //
    // Set initial, valid step value.
    //
    XSTEP = 300.0;
    spicelib::GFSSTP(XSTEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFSSTP(0.0, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    // Retrieve step.
    //
    spicelib::GFSTEP(&mut ET, &mut STEP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"STEP", STEP, b"=", XSTEP, 0.0, OK, ctx)?;

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(b"Set negative step.", ctx)?;

    //
    // Set initial, valid step value.
    //
    XSTEP = 300.0;
    spicelib::GFSSTP(XSTEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFSSTP(-1.0, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDSTEP)", OK, ctx)?;

    //
    // Retrieve step.
    //
    spicelib::GFSTEP(&mut ET, &mut STEP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"STEP", STEP, b"=", XSTEP, 0.0, OK, ctx)?;

    //*********************************************************************
    //*
    //*    Normal cases
    //*
    //*********************************************************************

    //
    //---- Case -------------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure positive step sizes are accepted and saved.",
        ctx,
    )?;

    //
    // ET should be ignored. Use an unusual value of ET.
    //
    ET = spicelib::DPMAX();

    XSTEP = 1.0;

    spicelib::GFSSTP(XSTEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFSTEP(&mut ET, &mut STEP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"STEP (1)", STEP, b"=", XSTEP, 0.0, OK, ctx)?;

    ET = -(spicelib::DPMAX() / 2 as f64);
    XSTEP = spicelib::DPMAX();

    spicelib::GFSSTP(XSTEP, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::GFSTEP(&mut ET, &mut STEP, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"STEP (DPMAX())", STEP, b"=", XSTEP, 0.0, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
