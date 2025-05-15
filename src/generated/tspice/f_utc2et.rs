//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LNSIZE: i32 = 80;

//$Procedure      F_UTC2ET ( A family of tests for UTC2ET )
pub fn F_UTC2ET(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NLINES: i32 = 0;
    let mut TSTRNG = ActualCharArray::new(LNSIZE, 1..=22);
    let mut KERNEL = ActualCharArray::new(LNSIZE, 1..=23);
    let mut EXPDET = StackArray::<f64, 22>::new(1..=22);
    let mut ET: f64 = 0.0;

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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_UTC2ET", ctx)?;

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

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(b"Check that an exception is generated if an A.M. or P.M. is specified in the time string. ", ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"1994 JAN 12 8:43 A.M.");

    spicelib::UTC2ET(&TSTRNG[1], &mut ET, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDTIMESTRING)", OK, ctx)?;

    testutil::TCASE(
        b"Check that an exception is generated if a time zone is included as part of the string. ",
        ctx,
    )?;

    fstr::assign(TSTRNG.get_mut(1), b"1994 JAN 12 8:43 PDT");

    spicelib::UTC2ET(&TSTRNG[1], &mut ET, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDTIMESTRING)", OK, ctx)?;

    testutil::TCASE(
        b"Check that an exception is generated if a time system is included in the time string. ",
        ctx,
    )?;

    fstr::assign(TSTRNG.get_mut(1), b"1994 JAN 12 8:43 TDT");

    spicelib::UTC2ET(&TSTRNG[1], &mut ET, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDTIMESTRING)", OK, ctx)?;

    testutil::TCASE(b"Make sure that when checking is enabled that an exception is generated when a component of the time string is out of range. ", ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"1994 JAN 32 08:43:12");

    spicelib::TPARCH(b"YES", ctx);
    spicelib::UTC2ET(&TSTRNG[1], &mut ET, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDTIMESTRING)", OK, ctx)?;

    testutil::TCASE(b"Check UTC2ET against a collection of times for which the outcome has been computed and hardcoded in advance. ", ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b" 1/9/1986 3:12:59.2");
    EXPDET[1] = -441103565.6158334;

    fstr::assign(TSTRNG.get_mut(2), b"9 JAN 1986 03:12:59.2");
    EXPDET[2] = -441103565.6158334;

    fstr::assign(TSTRNG.get_mut(3), b"1 9 1986 3:12:59.2");
    EXPDET[3] = -441103565.6158334;

    fstr::assign(TSTRNG.get_mut(4), b"9 JAN 1986 03:12:59.2");
    EXPDET[4] = -441103565.6158334;

    fstr::assign(TSTRNG.get_mut(5), b"2 jan 1991 3:00:12.2");
    EXPDET[5] = -283942730.6160449;

    fstr::assign(TSTRNG.get_mut(6), b"2 JAN 1991 03:00:12.2");
    EXPDET[6] = -283942730.6160449;

    fstr::assign(TSTRNG.get_mut(7), b"1991 MAR 10 12:00:00");
    EXPDET[7] = -278121542.81448925;

    fstr::assign(TSTRNG.get_mut(8), b"10 MAR 1991 12:00:00");
    EXPDET[8] = -278121542.81448925;

    fstr::assign(TSTRNG.get_mut(9), b"1 March 1975 3:00");
    EXPDET[9] = -783853153.8146169;

    fstr::assign(TSTRNG.get_mut(10), b"1 MAR 1975 03:00:00");
    EXPDET[10] = -783853153.8146169;

    fstr::assign(TSTRNG.get_mut(11), b"2010 October 29 3:58");
    EXPDET[11] = 341596737.18247914;

    fstr::assign(TSTRNG.get_mut(12), b"29 OCT 2010 03:58:00");
    EXPDET[12] = 341596737.18247914;

    fstr::assign(TSTRNG.get_mut(13), b"dec 31 86 12");
    EXPDET[13] = -410313544.8160908;

    fstr::assign(TSTRNG.get_mut(14), b"31 DEC 1986 12:00:00");
    EXPDET[14] = -410313544.8160908;

    fstr::assign(TSTRNG.get_mut(15), b"86-365 // 12:00");
    EXPDET[15] = -410313544.8160908;

    fstr::assign(TSTRNG.get_mut(16), b"31 DEC 1986 12:00:00");
    EXPDET[16] = -410313544.8160908;

    fstr::assign(TSTRNG.get_mut(17), b"JD 2451545.");
    EXPDET[17] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(18), b"1 JAN 2000 12:00:00");
    EXPDET[18] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(19), b"jd 2451545.");
    EXPDET[19] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(20), b"1 JAN 2000 12:00:00");
    EXPDET[20] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(21), b"JD2451545.");
    EXPDET[21] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(22), b"1 JAN 2000 12:00:00");
    EXPDET[22] = 57.1839272823238;

    for I in 1..=22 {
        testutil::TSTMSG(b"#", b"Test subcase #.", ctx);
        testutil::TSTMSI(I, ctx);

        spicelib::UTC2ET(&TSTRNG[I], &mut ET, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        if (I < 17) {
            testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[I], 0.0000000000001, OK, ctx)?;
        } else {
            testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[I], 0.00000000001, OK, ctx)?;
        }
    }

    testutil::TCASE(
        b"Step accross a leapsecond making sure that ET has the expected behavior. ",
        ctx,
    )?;

    fstr::assign(TSTRNG.get_mut(1), b"31 DEC 1989 23:59:55.01");
    EXPDET[1] = -315575948.8060699;

    fstr::assign(TSTRNG.get_mut(2), b"31 DEC 1989 23:59:56.02");
    EXPDET[2] = -315575947.7960699;

    fstr::assign(TSTRNG.get_mut(3), b"31 DEC 1989 23:59:57.03");
    EXPDET[3] = -315575946.7860699;

    fstr::assign(TSTRNG.get_mut(4), b"31 DEC 1989 23:59:58.04");
    EXPDET[4] = -315575945.7760699;

    fstr::assign(TSTRNG.get_mut(5), b"31 DEC 1989 23:59:59.05");
    EXPDET[5] = -315575944.7660699;

    fstr::assign(TSTRNG.get_mut(6), b"31 DEC 1989 23:59:60.06");
    EXPDET[6] = -315575943.7560699;

    fstr::assign(TSTRNG.get_mut(7), b"1  JAN 1990 00:00:00.07");
    EXPDET[7] = -315575942.7460699;

    fstr::assign(TSTRNG.get_mut(8), b"1  JAN 1990 00:00:01.08");
    EXPDET[8] = -315575941.7360699;

    for I in 1..=8 {
        testutil::TSTMSG(b"#", b"Test subcase #.", ctx);
        testutil::TSTMSI(I, ctx);

        spicelib::UTC2ET(&TSTRNG[I], &mut ET, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[I], 0.0000000000001, OK, ctx)?;
    }

    spicelib::TPARCH(b"NO", ctx);
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
