//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LNSIZE: i32 = 80;
const TMSIZE: i32 = 60;

//$Procedure      F_ET2UTC ( Family of tests for ET2UTC)
pub fn F_ET2UTC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NLINES: i32 = 0;
    let mut KERNEL = ActualCharArray::new(LNSIZE, 1..=23);
    let mut ET: f64 = 0.0;
    let mut ESTR = ActualCharArray::new(TMSIZE, 1..=6);
    let mut UTCSTR = ActualCharArray::new(TMSIZE, 1..=6);
    let mut PICTUR = [b' '; TMSIZE as usize];
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut PASS: bool = false;
    let mut FMT = ActualCharArray::new(4, 1..=6);
    let mut PREC = StackArray::<i32, 6>::new(1..=6);

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //
    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ET2UTC", ctx)?;
    //
    // Set up the leapseconds kernel we'll be needing.
    //
    testutil::BEGDAT(&mut KERNEL[1]);
    fstr::assign(KERNEL.get_mut(2), b" ");
    fstr::assign(KERNEL.get_mut(3), b"DELTET/DELTA_T_A       =   32.184");
    fstr::assign(KERNEL.get_mut(4), b"DELTET/K               =    1.657D-3");
    fstr::assign(KERNEL.get_mut(5), b"DELTET/EB              =    1.671D-2");
    fstr::assign(
        KERNEL.get_mut(6),
        b"DELTET/M               = (  6.239996D0   1.99096871D-7 )",
    );
    fstr::assign(KERNEL.get_mut(7), b" ");
    fstr::assign(
        KERNEL.get_mut(8),
        b"DELTET/DELTA_AT        = ( 10,   @1972-JAN-1",
    );
    fstr::assign(
        KERNEL.get_mut(9),
        b"                           11,   @1972-JUL-1",
    );
    fstr::assign(
        KERNEL.get_mut(10),
        b"                           12,   @1973-JAN-1",
    );
    fstr::assign(
        KERNEL.get_mut(11),
        b"                           13,   @1974-JAN-1",
    );
    fstr::assign(
        KERNEL.get_mut(12),
        b"                           14,   @1975-JAN-1",
    );
    fstr::assign(
        KERNEL.get_mut(13),
        b"                           15,   @1976-JAN-1",
    );
    fstr::assign(
        KERNEL.get_mut(14),
        b"                           16,   @1977-JAN-1",
    );
    fstr::assign(
        KERNEL.get_mut(15),
        b"                           17,   @1978-JAN-1",
    );
    fstr::assign(
        KERNEL.get_mut(16),
        b"                           18,   @1979-JAN-1",
    );
    fstr::assign(
        KERNEL.get_mut(17),
        b"                           19,   @1980-JAN-1",
    );
    fstr::assign(
        KERNEL.get_mut(18),
        b"                           20,   @1981-JUL-1",
    );
    fstr::assign(
        KERNEL.get_mut(19),
        b"                           21,   @1982-JUL-1",
    );
    fstr::assign(
        KERNEL.get_mut(20),
        b"                           22,   @1983-JUL-1",
    );
    fstr::assign(
        KERNEL.get_mut(21),
        b"                           23,   @1985-JUL-1",
    );
    fstr::assign(
        KERNEL.get_mut(22),
        b"                           24,   @1988-JAN-1",
    );
    fstr::assign(
        KERNEL.get_mut(23),
        b"                           25,   @1990-JAN-1 )",
    );

    NLINES = 23;

    spicelib::CLPOOL(ctx)?;
    testutil::TSTTXT(b"testleap.ker", KERNEL.as_arg(), NLINES, true, false, ctx)?;

    testutil::TCASE(
        b"Test that the advertised conversion in the header of ET2UTC behaves as predicted. ",
        ctx,
    )?;

    ET = -527644192.5403653;

    fstr::assign(ESTR.get_mut(1), b"1983 APR 13 12:09:14");
    fstr::assign(ESTR.get_mut(2), b"1983 APR 13 12:09:14.274");
    fstr::assign(ESTR.get_mut(3), b"1983-103 // 12:09:14.27400");
    fstr::assign(ESTR.get_mut(4), b"JD 2445438.0064152");
    fstr::assign(ESTR.get_mut(5), b"1983-103T12:09:14.274");
    fstr::assign(ESTR.get_mut(6), b"1983-04-13T12:09:14.274");

    spicelib::ET2UTC(ET, b"C", 0, &mut UTCSTR[1], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ET2UTC(ET, b"C", 3, &mut UTCSTR[2], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ET2UTC(ET, b"D", 5, &mut UTCSTR[3], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ET2UTC(ET, b"J", 7, &mut UTCSTR[4], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ET2UTC(ET, b"ISOD", 3, &mut UTCSTR[5], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ET2UTC(ET, b"ISOC", 3, &mut UTCSTR[6], ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSC(b"UTCSTR(1)", &UTCSTR[1], b"=", &ESTR[1], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(2)", &UTCSTR[2], b"=", &ESTR[2], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(3)", &UTCSTR[3], b"=", &ESTR[3], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(4)", &UTCSTR[4], b"=", &ESTR[4], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(5)", &UTCSTR[5], b"=", &ESTR[5], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(6)", &UTCSTR[6], b"=", &ESTR[6], OK, ctx)?;

    testutil::TCASE(b"Use the time formatting routine TIMOUT together with the utility TPICTR to see that we get consistent strings. ", ctx)?;

    for I in 1..=6 {
        testutil::TSTMSG(b"#", b"Test Case 2, Subcase #.", ctx);
        testutil::TSTMSI(I, ctx);

        spicelib::TPICTR(&ESTR[I], &mut PICTUR, &mut PASS, &mut ERROR, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;

        spicelib::TIMOUT(ET, &PICTUR, &mut UTCSTR[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::CHCKSC(b"UTCSTR(1)", &UTCSTR[1], b"=", &ESTR[1], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(2)", &UTCSTR[2], b"=", &ESTR[2], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(3)", &UTCSTR[3], b"=", &ESTR[3], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(4)", &UTCSTR[4], b"=", &ESTR[4], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(5)", &UTCSTR[5], b"=", &ESTR[5], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(6)", &UTCSTR[6], b"=", &ESTR[6], OK, ctx)?;

    testutil::TCASE(b"Test the invertability of the call UTC2ET.", ctx)?;

    fstr::assign(ESTR.get_mut(1), b"JD 2451712.2829282      ");
    fstr::assign(ESTR.get_mut(2), b"1987 MAR 12 19:28:28.28729  ");
    fstr::assign(ESTR.get_mut(3), b"1989 DEC 31 23:59:60.18291  ");
    fstr::assign(ESTR.get_mut(4), b"1990-001 // 00:00:00.1728    ");
    fstr::assign(ESTR.get_mut(5), b"1987-03-18T17:28:28.182        ");
    fstr::assign(ESTR.get_mut(6), b"1986-239T12:29:28.287        ");

    fstr::assign(FMT.get_mut(1), b"J");
    fstr::assign(FMT.get_mut(2), b"C");
    fstr::assign(FMT.get_mut(3), b"C");
    fstr::assign(FMT.get_mut(4), b"D");
    fstr::assign(FMT.get_mut(5), b"ISOC");
    fstr::assign(FMT.get_mut(6), b"ISOD");

    PREC[1] = 7;
    PREC[2] = 5;
    PREC[3] = 5;
    PREC[4] = 4;
    PREC[5] = 3;
    PREC[6] = 3;

    for I in 1..=6 {
        testutil::TSTMSG(b"#", b"Test Case 3, Subcase #.", ctx);
        testutil::TSTMSI(I, ctx);

        spicelib::UTC2ET(&ESTR[I], &mut ET, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::ET2UTC(ET, &FMT[I], PREC[I], &mut UTCSTR[I], ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
    }

    testutil::CHCKSC(b"UTCSTR(1)", &UTCSTR[1], b"=", &ESTR[1], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(2)", &UTCSTR[2], b"=", &ESTR[2], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(3)", &UTCSTR[3], b"=", &ESTR[3], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(4)", &UTCSTR[4], b"=", &ESTR[4], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(5)", &UTCSTR[5], b"=", &ESTR[5], OK, ctx)?;
    testutil::CHCKSC(b"UTCSTR(6)", &UTCSTR[6], b"=", &ESTR[6], OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
