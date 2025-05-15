//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SMWDSZ: i32 = 16;

//$Procedure      F_TIMDEF ( Family of tests for TIMDEF )
pub fn F_TIMDEF(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut VALUE = [b' '; SMWDSZ as usize];
    let mut ZONE = [b' '; SMWDSZ as usize];
    let mut CALNDR = [b' '; SMWDSZ as usize];
    let mut SYSTEM = [b' '; SMWDSZ as usize];
    let mut ZONES = ActualCharArray::new(SMWDSZ, 1..=8);
    let mut EXPECT = ActualCharArray::new(SMWDSZ, 1..=8);

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TIMDEF", ctx)?;

    testutil::TCASE(b"Check that the default values are SYSTEM = \'UTC\', ZONE = \' \' and CALENDAR = \'GREGORIAN\' ", ctx)?;

    spicelib::TIMDEF(b"get", b"system", &mut VALUE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"VALUE", &VALUE, b"=", b"UTC", OK, ctx)?;

    spicelib::TIMDEF(b"get", b"zone", &mut VALUE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"VALUE", &VALUE, b"=", b" ", OK, ctx)?;

    spicelib::TIMDEF(b"get", b"calendar", &mut VALUE, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"VALUE", &VALUE, b"=", b"GREGORIAN", OK, ctx)?;

    testutil::TCASE(b"Make sure we can set and get SYSTEM UTC, TDT, TDB Make sure Calendar is not affected and that ZONE returns a blank. ", ctx)?;

    //
    // Excessive?
    //

    spicelib::TIMDEF(b"set", b"system", &mut b"tdb".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b"TDB", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"GREGORIAN", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b" ", OK, ctx)?;

    spicelib::TIMDEF(b"set", b"system", &mut b"tdt".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b"TDT", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"GREGORIAN", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b" ", OK, ctx)?;

    spicelib::TIMDEF(b"set", b"system", &mut b"tt".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b"TT", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"GREGORIAN", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b" ", OK, ctx)?;

    spicelib::TIMDEF(b"set", b"system", &mut b"utc".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b"UTC", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"GREGORIAN", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b" ", OK, ctx)?;

    testutil::TCASE(b"Make sure we can set and get CALENDAR MIXED, JULIAN, GREGORIAN Make sure that system and zone are not affected. ", ctx)?;

    spicelib::TIMDEF(b"set", b"calendar", &mut b"Mixed".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b"UTC", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"MIXED", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b" ", OK, ctx)?;

    spicelib::TIMDEF(b"set", b"calendar", &mut b"Julian".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b"UTC", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"JULIAN", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b" ", OK, ctx)?;

    spicelib::TIMDEF(b"set", b"calendar", &mut b"Gregorian".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b"UTC", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"GREGORIAN", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b" ", OK, ctx)?;

    testutil::TCASE(b"Make sure we can set and get all of the U.S. time zones. Also check that the CALENDAR is not affected and that SYSTEM returns a blank. ", ctx)?;

    fstr::assign(ZONES.get_mut(1), b"est");
    fstr::assign(ZONES.get_mut(2), b"edt");
    fstr::assign(ZONES.get_mut(3), b"cst");
    fstr::assign(ZONES.get_mut(4), b"cdt");
    fstr::assign(ZONES.get_mut(5), b"mst");
    fstr::assign(ZONES.get_mut(6), b"mdt");
    fstr::assign(ZONES.get_mut(7), b"pst");
    fstr::assign(ZONES.get_mut(8), b"pdt");

    fstr::assign(EXPECT.get_mut(1), b"UTC-5");
    fstr::assign(EXPECT.get_mut(2), b"UTC-4");
    fstr::assign(EXPECT.get_mut(3), b"UTC-6");
    fstr::assign(EXPECT.get_mut(4), b"UTC-5");
    fstr::assign(EXPECT.get_mut(5), b"UTC-7");
    fstr::assign(EXPECT.get_mut(6), b"UTC-6");
    fstr::assign(EXPECT.get_mut(7), b"UTC-8");
    fstr::assign(EXPECT.get_mut(8), b"UTC-7");

    for I in 1..=8 {
        spicelib::TIMDEF(b"set", b"zone", &mut ZONES[I], ctx)?;
        spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
        spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
        spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b" ", OK, ctx)?;
        testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"GREGORIAN", OK, ctx)?;
        testutil::CHCKSC(b"ZONE", &ZONE, b"=", &EXPECT[I], OK, ctx)?;
    }

    testutil::TCASE(
        b"Make sure we can set and get several non-U.S. time zones. ",
        ctx,
    )?;

    spicelib::TIMDEF(b"set", b"zone", &mut b"utc+3:19".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"GREGORIAN", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b"UTC+3:19", OK, ctx)?;

    spicelib::TIMDEF(b"set", b"zone", &mut b"utc-4:27".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"GREGORIAN", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b"UTC-4:27", OK, ctx)?;

    testutil::TCASE(
        b"Make sure that unrecognized ACTIONS trigger an error. ",
        ctx,
    )?;

    spicelib::TIMDEF(b"put", b"zone", &mut b"pdt".clone(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADACTION)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure that unrecognized ITEMS trigger an error. SET case",
        ctx,
    )?;

    spicelib::TIMDEF(b"set", b"zone", &mut b"utc-4:27".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"GREGORIAN", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b"UTC-4:27", OK, ctx)?;

    spicelib::TIMDEF(b"set", b"year", &mut b"1950".clone(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADTIMEITEM)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure that unrecognized ITEMS trigger an error. GET case",
        ctx,
    )?;

    spicelib::TIMDEF(b"set", b"zone", &mut b"utc-4:27".clone(), ctx)?;
    spicelib::TIMDEF(b"get", b"system", &mut SYSTEM, ctx)?;
    spicelib::TIMDEF(b"get", b"zone", &mut ZONE, ctx)?;
    spicelib::TIMDEF(b"get", b"calendar", &mut CALNDR, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"SYSTEM", &SYSTEM, b"=", b" ", OK, ctx)?;
    testutil::CHCKSC(b"CALNDR", &CALNDR, b"=", b"GREGORIAN", OK, ctx)?;
    testutil::CHCKSC(b"ZONE", &ZONE, b"=", b"UTC-4:27", OK, ctx)?;

    spicelib::TIMDEF(b"GET", b"year", &mut VALUE, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADTIMEITEM)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure that unrecongized time zones trigger an error. SET case ",
        ctx,
    )?;

    spicelib::TIMDEF(b"set", b"ZONE", &mut b"GMT".clone(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDEFAULTVALUE)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure that unrecognized calendars trigger and error. ",
        ctx,
    )?;

    spicelib::TIMDEF(b"set", b"CALENDAR", &mut b"MUSLIM".clone(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDEFAULTVALUE)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure that unrecognized systems trigger an error. ",
        ctx,
    )?;

    spicelib::TIMDEF(b"set", b"SYSTEM", &mut b"GMT".clone(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADDEFAULTVALUE)", OK, ctx)?;

    //
    // Reset the defaults
    //
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
