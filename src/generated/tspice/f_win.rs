//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const MAXVAL: i32 = 20000;

//$Procedure F_WIN ( Window family tests )
pub fn F_WIN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut MSG = [b' '; 320 as usize];
    let mut RESULT = ActualArray::<f64>::new(LBCELL..=MAXVAL);
    let mut M: i32 = 0;
    let mut MEAS: f64 = 0.0;
    let mut AVG: f64 = 0.0;
    let mut STDDEV: f64 = 0.0;
    let mut SHORT: i32 = 0;
    let mut LONG: i32 = 0;

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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_WIN", ctx)?;

    //
    // Case 1
    //
    //
    testutil::TCASE(b"Check ZZWNINSD error cases", ctx)?;

    spicelib::SSIZED(MAXVAL, RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    //
    // Try to add an invalid interval to the window.
    //
    spicelib::ZZWNINSD(0.0, -1.0, b"TEST_STRING_1", RESULT.as_slice_mut(), ctx)?;

    spicelib::GETMSG(b"LONG", &mut MSG, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADENDPOINTS)", OK, ctx)?;

    M = spicelib::POS(&MSG, b"TEST_STRING_1", 1);
    testutil::CHCKSI(b"TEST_STRING_1", M, b"=", 115, 0, OK, ctx)?;

    //
    // Empty the window, then reduce size to zero.
    //
    spicelib::SSIZED(0, RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(0, RESULT.as_slice_mut(), ctx)?;

    //
    // Try to add an interval to the window.
    //
    spicelib::ZZWNINSD(0.0, 1.0, b"TEST_STRING_2", RESULT.as_slice_mut(), ctx)?;

    spicelib::GETMSG(b"LONG", &mut MSG, ctx)?;
    testutil::CHCKXC(true, b"SPICE(WINDOWEXCESS)", OK, ctx)?;

    M = spicelib::POS(&MSG, b"TEST_STRING_2", 1);
    testutil::CHCKSI(b"TEST_STRING_2", M, b"=", 90, 0, OK, ctx)?;

    //
    // Case 2
    //
    //
    testutil::TCASE(b"Check WNSUMD error case", ctx)?;

    //
    // An odd cardinality for a WNSUMD input window should
    // signal an error.
    //
    spicelib::SSIZED(MAXVAL, RESULT.as_slice_mut(), ctx)?;
    spicelib::SCARDD(13, RESULT.as_slice_mut(), ctx)?;

    spicelib::WNSUMD(
        RESULT.as_slice(),
        &mut MEAS,
        &mut AVG,
        &mut STDDEV,
        &mut SHORT,
        &mut LONG,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCARDINALITY)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);

    Ok(())
}
