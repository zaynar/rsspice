//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      F_LTIME ( Family of light time tests )
pub fn F_LTIME(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut ELAPS: f64 = 0.0;
    let mut ET: f64 = 0.0;
    let mut ET2: f64 = 0.0;
    let mut EXPET: f64 = 0.0;
    let mut EXPET2: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut HANDLE: i32 = 0;

    //
    // Test Utility Functions
    //
    // None.
    //
    //
    // SPICELIB Functions
    //
    //
    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_LTIME", ctx)?;

    testutil::TSTSPK(b"test.bsp", true, &mut HANDLE, ctx)?;

    testutil::TCASE(b"Test the advertised exception", ctx)?;

    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;
    spicelib::LTIME(ET, 399, b"TO", 499, &mut ET2, &mut ELAPS, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADDIRECTION)", OK, ctx)?;

    testutil::TCASE(b"Check to make sure downlink light time matches the light time returned by SPKEZ when the option \'CN\' is used in SPKEZ. ", ctx)?;

    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;
    spicelib::LTIME(ET, 399, b"<-", 599, &mut ET2, &mut ELAPS, ctx)?;
    spicelib::SPKEZ(
        599,
        ET,
        b"J2000",
        b"CN",
        399,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    EXPET2 = (ET - LT);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ELAPS", ELAPS, b"~", LT, 0.000000000001, OK, ctx)?;
    testutil::CHCKSD(b"ET2", ET2, b"~/", EXPET2, 0.0000000000001, OK, ctx)?;

    testutil::TCASE(
        b"Check to make sure that the uplink time is compatible with the down link time. ",
        ctx,
    )?;

    spicelib::TPARSE(b"1 JAN 1995", &mut ET, &mut ERROR, ctx)?;
    spicelib::LTIME(ET, 399, b"->", 599, &mut ET2, &mut LT, ctx)?;
    spicelib::LTIME(ET2, 599, b"<-", 399, &mut EXPET, &mut ELAPS, ctx)?;

    EXPET2 = (ET + LT);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET2", ET2, b"~/", EXPET2, 0.0000000000001, OK, ctx)?;
    testutil::CHCKSD(b"ELAPS", ELAPS, b"~/", LT, 0.0000000000001, OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~/", EXPET, 0.0000000000001, OK, ctx)?;

    //
    // That's all folks.
    //
    spicelib::SPKUEF(HANDLE, ctx)?;
    testutil::KILFIL(b"test.bsp", ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
