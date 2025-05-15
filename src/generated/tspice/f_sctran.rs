//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NAMLEN: i32 = 80;

//$Procedure      F_SCTRAN ( Test SCLK name/ID translation )
pub fn F_SCTRAN(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NAME = [b' '; NAMLEN as usize];
    let mut ID: i32 = 0;
    let mut FOUND: bool = false;

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
    testutil::TOPEN(b"F_SCTRAN", ctx)?;

    testutil::TCASE(b"Map ID -77 to clock string.", ctx)?;

    spicelib::SCID2N(-77, &mut NAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSC(b"NAME", &NAME, b"=", b"GALILEO ORBITER SCLK", OK, ctx)?;

    testutil::TCASE(
        b"Map ID -77777 to clock string.  No string should be found.",
        ctx,
    )?;

    spicelib::SCID2N(-77777, &mut NAME, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    testutil::TCASE(
        b"Map clock string \'GALILEO ORBITER SCLK\' to ID -77 .",
        ctx,
    )?;

    spicelib::SCN2ID(b"GALILEO ORBITER SCLK", &mut ID, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"ID", ID, b"=", -77, 0, OK, ctx)?;

    testutil::TCASE(
        b"Map clock string \'galileo orbiter sclk\' to ID -77 .",
        ctx,
    )?;

    spicelib::SCN2ID(b"galileo orbiter sclk", &mut ID, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"ID", ID, b"=", -77, 0, OK, ctx)?;

    testutil::TCASE(
        b"Map clock string \'  gAlileo orbIter  sclk\' to ID -77 .",
        ctx,
    )?;

    spicelib::SCN2ID(b"  gAlileo orbIter  sclk", &mut ID, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"ID", ID, b"=", -77, 0, OK, ctx)?;

    testutil::TCASE(b"Map clock string \'  CAS sclk\' to ID -82 .", ctx)?;

    spicelib::SCN2ID(b"  CAS sclk", &mut ID, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;
    testutil::CHCKSI(b"ID", ID, b"=", -82, 0, OK, ctx)?;

    testutil::TCASE(b"Map \'XYXYX\' to clock ID.  No ID should be found.", ctx)?;

    spicelib::SCN2ID(b"XYXYX", &mut ID, &mut FOUND, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
