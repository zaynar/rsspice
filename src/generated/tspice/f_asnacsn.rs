//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      F_ASNACSN (Family of tests for DASINE and DACOSN)
pub fn F_ASNACSN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TSTARG: f64 = 0.0;
    let mut TOL: f64 = 0.0;
    let mut TSTANG: f64 = 0.0;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB functions
    //

    //
    // Local variables.
    //

    //
    // Begin every test family with an open call.
    //

    testutil::TOPEN(b"F_ASNACSN", ctx)?;

    //
    // Test for tolerance negative error.
    //

    TOL = -0.000000000001;

    testutil::TCASE(b"Tolerance negative, error in DASINE", ctx)?;

    TSTANG = spicelib::DASINE(0.0, TOL, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    testutil::TCASE(b"Tolerance negative, error in DACOSN", ctx)?;

    TSTANG = spicelib::DACOSN(1.0, TOL, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //
    // Test for error when argument is too large
    //

    TOL = 0.000000000001;

    TSTARG = (1.0 + 0.00000000001);

    testutil::TCASE(b"Argument too large in DASINE.", ctx)?;

    TSTANG = spicelib::DASINE(TSTARG, TOL, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INPUTOUTOFBOUNDS)", OK, ctx)?;

    testutil::TCASE(b"Is value = ASIN (1.0) in DASINE?)", ctx)?;

    testutil::CHCKSD(b"TSTANG", TSTANG, b"=", spicelib::HALFPI(ctx), 0.0, OK, ctx)?;

    testutil::TCASE(b"Argument too large in DACOSN.", ctx)?;

    TSTANG = spicelib::DACOSN(TSTARG, TOL, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INPUTOUTOFBOUNDS)", OK, ctx)?;

    testutil::TCASE(b"Is value = ACOS (1.0) in DACOSN?)", ctx)?;

    testutil::CHCKSD(b"TSTANG", TSTANG, b"=", 0.0, 0.0, OK, ctx)?;

    //
    // Test for error when argument is too small
    //

    TOL = 0.000000000001;

    TSTARG = (-1.0 - 0.00000000001);

    testutil::TCASE(b"Argument too small in DASINE.", ctx)?;

    TSTANG = spicelib::DASINE(TSTARG, TOL, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INPUTOUTOFBOUNDS)", OK, ctx)?;

    testutil::TCASE(b"Is value = ASIN (-1.0) in DASINE?)", ctx)?;

    testutil::CHCKSD(
        b"TSTANG",
        TSTANG,
        b"=",
        -spicelib::HALFPI(ctx),
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Argument too small in DACOSN.", ctx)?;

    TSTANG = spicelib::DACOSN(TSTARG, TOL, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INPUTOUTOFBOUNDS)", OK, ctx)?;

    testutil::TCASE(b"Is value = ACOS (-1.0) in DACOSN?)", ctx)?;

    testutil::CHCKSD(b"TSTANG", TSTANG, b"=", spicelib::PI(ctx), 0.0, OK, ctx)?;

    //
    // All done.
    //

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
