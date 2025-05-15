//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LNSIZE: i32 = 80;
const MSGSIZ: i32 = 300;

//$Procedure F_STR2ET ( Family of tests for STR2ET )
pub fn F_STR2ET(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NLINES: i32 = 0;
    let mut N: i32 = 0;
    let mut TSTRNG = ActualCharArray::new(LNSIZE, 1..=29);
    let mut ISO = ActualCharArray::new(LNSIZE, 1..=13);
    let mut KERNEL = ActualCharArray::new(LNSIZE, 1..=23);
    let mut ERROR = [b' '; MSGSIZ as usize];
    let mut EXPDET = StackArray::<f64, 29>::new(1..=29);
    let mut ET: f64 = 0.0;
    let mut ETISO: f64 = 0.0;
    let mut TT: f64 = 0.0;
    let mut TDT: f64 = 0.0;
    let mut TDB: f64 = 0.0;
    let mut TAI: f64 = 0.0;
    let mut GPS: f64 = 0.0;
    let mut EB: f64 = 0.0;
    let mut K: f64 = 0.0;
    let mut M = StackArray::<f64, 2>::new(0..=1);
    let mut FOUND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_STR2ET", ctx)?;

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

    //
    // STR2ET critically depends on UNITIM. Confirm expected
    // basic behavior.
    //
    testutil::TCASE(b"Check UNITIM.", ctx)?;

    TT = spicelib::UNITIM(0.0, b"TDB", b"TT", ctx)?;
    TDT = spicelib::UNITIM(0.0, b"TDB", b"TDT", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"TDB -> TT TDT", TT, b"=", TDT, 0.0, OK, ctx)?;

    //
    // Confirm the expected offset from TT of GPS and TAI.
    //
    GPS = spicelib::UNITIM(0.0, b"TDT", b"GPS", ctx)?;
    TAI = spicelib::UNITIM(0.0, b"TDT", b"TAI", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(
        b"TDT -> GPS",
        GPS,
        b"~/",
        -51.184,
        0.000000000000001,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(
        b"TDT -> TAI",
        TAI,
        b"~/",
        -32.184,
        0.000000000000001,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(
        b"TAI-GPS",
        (TAI - GPS),
        b"~/",
        19.0,
        0.000000000000001,
        OK,
        ctx,
    )?;

    //
    // Reverse order.
    //

    spicelib::GDPOOL(
        b"DELTET/EB",
        1,
        1,
        &mut N,
        std::slice::from_mut(&mut EB),
        &mut FOUND,
        ctx,
    )?;
    spicelib::GDPOOL(
        b"DELTET/K",
        1,
        1,
        &mut N,
        std::slice::from_mut(&mut K),
        &mut FOUND,
        ctx,
    )?;
    spicelib::GDPOOL(b"DELTET/M", 1, 2, &mut N, M.as_slice_mut(), &mut FOUND, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    TT = spicelib::UNITIM(0.0, b"TT", b"TDB", ctx)?;
    TDT = spicelib::UNITIM(0.0, b"TDT", b"TDB", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Explicitly calculate the TDB value for TDT 0.
    //
    TDB = (K * f64::sin((M[0] + (EB * f64::sin(M[0])))));

    //
    // Idiot check.
    //
    testutil::CHCKSD(b"TDB !=0", TDB, b"!=", 0.0, 0.0, OK, ctx)?;

    //
    // Confirm the subsystem recognizes 'TT' and 'TDT' as the same
    // time system.
    //
    testutil::CHCKSD(b"TT TDT -> TDB", TT, b"=", TDT, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"TDB = f(0)", TT, b"=", TDB, 0.0, OK, ctx)?;

    GPS = spicelib::UNITIM(0.0, b"GPS", b"TDT", ctx)?;
    TAI = spicelib::UNITIM(0.0, b"TAI", b"TDT", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Confirm the expected offset from TT of GPS and TAI.
    //
    testutil::CHCKSD(
        b"GPS -> TDT",
        GPS,
        b"~/",
        51.184,
        0.000000000000001,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(
        b"TAI -> TDT",
        TAI,
        b"~/",
        32.184,
        0.000000000000001,
        OK,
        ctx,
    )?;

    testutil::CHCKSD(
        b"GPS-TAI",
        (GPS - TAI),
        b"~/",
        19.0,
        0.000000000000001,
        OK,
        ctx,
    )?;

    //
    // Confirm error response.
    //
    TT = spicelib::UNITIM(0.0, b"NULL", b"TDT", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADTIMETYPE)", OK, ctx)?;

    TT = spicelib::UNITIM(0.0, b"TDT", b"NULL", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADTIMETYPE)", OK, ctx)?;

    testutil::TCASE(b"Make sure that an exception is generated when a component of the time string is out of range. ", ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"1994 JAN 32 08:43:12");

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADTIMESTRING)", OK, ctx)?;

    testutil::TCASE(b"Check STR2ET against a collection of times for which the outcome has been computed and hard coded in advance. ", ctx)?;

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

    fstr::assign(TSTRNG.get_mut(13), b"dec 31 86 12:00");
    EXPDET[13] = -410313544.8160908;

    fstr::assign(TSTRNG.get_mut(14), b"31 DEC 1986 12:00:00");
    EXPDET[14] = -410313544.8160908;

    fstr::assign(TSTRNG.get_mut(15), b"86-365 // 12:00");
    EXPDET[15] = -410313544.8160908;

    fstr::assign(TSTRNG.get_mut(16), b"31 DEC 1986 12:00:00");
    EXPDET[16] = -410313544.8160908;

    fstr::assign(TSTRNG.get_mut(17), b"1991 MAR 10 12:00.25");
    EXPDET[17] = -278121527.81448925;

    fstr::assign(TSTRNG.get_mut(18), b"1991 MAR 10 12.5");
    EXPDET[18] = -278119742.81448925;

    fstr::assign(TSTRNG.get_mut(19), b"069-1991 // 12:00.25");
    EXPDET[19] = -278121527.81448925;

    fstr::assign(TSTRNG.get_mut(20), b"JD 2451545.");
    EXPDET[20] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(21), b"1 JAN 2000 12:00:00");
    EXPDET[21] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(22), b"jd 2451545.");
    EXPDET[22] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(23), b"1 JAN 2000 12:00:00");
    EXPDET[23] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(24), b"JD2451545.");
    EXPDET[24] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(25), b"1 JAN 2000 12:00:00");
    EXPDET[25] = 57.1839272823238;

    fstr::assign(TSTRNG.get_mut(26), b"JDTDB 2451546.0");
    EXPDET[26] = 86400.0;

    fstr::assign(TSTRNG.get_mut(27), b"JDTDB 2451545.0");
    EXPDET[27] = 0.0;

    fstr::assign(TSTRNG.get_mut(28), b"TDBJD 2451546.0");
    EXPDET[28] = 86400.0;

    fstr::assign(TSTRNG.get_mut(29), b"2451545.0 TDBJD");
    EXPDET[29] = 0.0;

    for I in 1..=29 {
        testutil::TSTMSG(b"#", b"Test subcase #. #", ctx);
        testutil::TSTMSI(I, ctx);

        spicelib::STR2ET(&TSTRNG[I], &mut ET, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (I < 20) {
            testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[I], 0.0000000000001, OK, ctx)?;
        } else {
            testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[I], 0.00000000001, OK, ctx)?;
        }
    }

    testutil::TCASE(b"Check ISO formats with and without Z marker.", ctx)?;

    fstr::assign(ISO.get_mut(1), b"1986-01-18T12:19:52.18");
    fstr::assign(ISO.get_mut(2), b"1996-12-18T12:28:28");
    fstr::assign(ISO.get_mut(3), b"1986-01-18T12:19");
    fstr::assign(ISO.get_mut(4), b"1986-01-18T12");
    fstr::assign(ISO.get_mut(5), b"1986-01-18T");
    fstr::assign(ISO.get_mut(6), b"1986-01-18T12.5");
    fstr::assign(ISO.get_mut(7), b"1986-01-18T12:19.5");
    fstr::assign(ISO.get_mut(8), b"1995-08T18:28:12.53");
    fstr::assign(ISO.get_mut(9), b"1995-08T18:28:12");
    fstr::assign(ISO.get_mut(10), b"1995-08T18:28");
    fstr::assign(ISO.get_mut(11), b"1995-18T");
    fstr::assign(ISO.get_mut(12), b"1986-209T12:19.18");
    fstr::assign(ISO.get_mut(13), b"1986-209T12.18");

    for I in 1..=13 {
        testutil::TCASE(&ISO[I], ctx)?;

        spicelib::STR2ET(&ISO[I], &mut ET, ctx)?;

        spicelib::STR2ET(&fstr::concat(ISO.get(I), b"Z"), &mut ETISO, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"ETISO", ET, b"=", ETISO, 0.0, OK, ctx)?;
    }

    testutil::TCASE(b"Check ISO formats two digit year", ctx)?;

    fstr::assign(ISO.get_mut(1), b"00-01-01T");

    fstr::assign(TSTRNG.get_mut(1), b"Jan 1, 2000");

    testutil::TCASE(&ISO[1], ctx)?;

    spicelib::STR2ET(&ISO[1], &mut ETISO, ctx)?;

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ETISO", ET, b"=", ETISO, 0.0, OK, ctx)?;

    fstr::assign(ISO.get_mut(1), b"11-001T00:00:01.000");

    fstr::assign(TSTRNG.get_mut(1), b"2011-01-01 00:00:01.000");

    testutil::TCASE(&ISO[1], ctx)?;

    spicelib::STR2ET(&ISO[1], &mut ETISO, ctx)?;

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ETISO", ET, b"=", ETISO, 0.0, OK, ctx)?;

    testutil::TCASE(b"Check ISO formats 100 AD. to 9999 AD", ctx)?;

    //
    // Loop over AD years 100 to 9999.
    //
    for J in 100..=9999 {
        fstr::assign(ISO.get_mut(1), b"#-01-01T");
        fstr::assign(TSTRNG.get_mut(1), b"Jan 1, # A.D.");

        spicelib::REPMI(&ISO[1].to_vec(), b"#", J, &mut ISO[1], ctx);
        spicelib::REPMI(&TSTRNG[1].to_vec(), b"#", J, &mut TSTRNG[1], ctx);

        testutil::TCASE(&ISO[1], ctx)?;

        spicelib::STR2ET(&ISO[1], &mut ETISO, ctx)?;

        spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"ETISO", ET, b"=", ETISO, 0.0, OK, ctx)?;
    }

    testutil::TCASE(
        b"Step across a leapsecond making sure that ET has the expected behavior. ",
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

        spicelib::STR2ET(&TSTRNG[I], &mut ET, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[I], 0.0000000000001, OK, ctx)?;
    }

    testutil::TCASE(
        b"Check to see that we can successfully compute TDB based times. SubCase 1.",
        ctx,
    )?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 11:02:05 1996 TDB");

    spicelib::TPARSE(b"Sep 27 11:02:05 1996", &mut EXPDET[1], &mut ERROR, ctx)?;
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    testutil::TCASE(
        b"Check to see that we can successfully compute TDB based times. SubCase 2.",
        ctx,
    )?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri, Sep 27 11:02:05 1996 TDB");

    spicelib::TPARSE(b"Sep 27 11:02:05 1996", &mut EXPDET[1], &mut ERROR, ctx)?;
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    testutil::TCASE(
        b"Check to see that we can successfully compute times for various time zones. ",
        ctx,
    )?;

    spicelib::UTC2ET(b"Sep 27 18:08:31 1996", &mut EXPDET[1], ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 11:08:31 PDT 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 10:08:31 PST 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri, Sep 27 10:08:31 PST 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 11:08:31 MST 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri, Sep 27 11:08:31 MST 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 12:08:31 MDT 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 12:08:31 CST 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 13:08:31 CDT 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 13:08:31 EST 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 14:08:31 EDT 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    testutil::TCASE(b"Check to see that we can successfully compute times for time zones that are offset from UTC by a fractional number of hours. ", ctx)?;

    spicelib::UTC2ET(b"Sep 27 18:08:31 1996", &mut EXPDET[1], ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 13:00:31 1996  UTC-5:08");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"=", EXPDET[1], 0.0000000000001, OK, ctx)?;

    testutil::TCASE(
        b"Make sure that TDT labeled strings are recognized and properly processed. ",
        ctx,
    )?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 11:02:05 1996 TDT");

    spicelib::TPARSE(b"Sep 27 11:02:05 1996", &mut EXPDET[1], &mut ERROR, ctx)?;

    EXPDET[1] = spicelib::UNITIM(EXPDET[1], b"TDT", b"TDB", ctx)?;

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~", EXPDET[1], 0.0000001, OK, ctx)?;

    testutil::TCASE(b"Make sure that an exception is triggered if we try to put a leapsecond in the wrong place on the UTC scale. ", ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 23:59:60.1 1996");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADTIMESTRING)", OK, ctx)?;

    testutil::TCASE(b"Make sure that an exception is triggered if we try to put a leapsecond in the wrong place on a non-UTC scale. ", ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"DEC 31 23:59:60.1 1996 PDT");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADTIMESTRING)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure that we get a good value for a real leapsecond on a non-UTC Scale. ",
        ctx,
    )?;
    //
    // Note we just copied the expected value of ET from a
    // previous case.
    //
    fstr::assign(TSTRNG.get_mut(1), b"31 DEC 1989 15:59:60.06 PST");
    EXPDET[1] = -315575943.7560699;

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"31 DEC 1989 13:39:60.06 UTC-10:20");
    EXPDET[1] = -315575943.7560699;

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[1], 0.0000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"1 JAN 1990 10:19:60.06 UTC+10:20");
    EXPDET[1] = -315575943.7560699;

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[1], 0.0000000000001, OK, ctx)?;

    testutil::TCASE(
        b"Make sure that leap seconds are not allowed in the TDB system. ",
        ctx,
    )?;

    fstr::assign(TSTRNG.get_mut(1), b"31 DEC 1989 23:59:60.06 TDB");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADTIMESTRING)", OK, ctx)?;

    testutil::TCASE(
        b"Make sure that leap seconds are not allowed in the TDT system. ",
        ctx,
    )?;

    fstr::assign(TSTRNG.get_mut(1), b"31 DEC 1989 23:59:60.06 TDT");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADTIMESTRING)", OK, ctx)?;

    testutil::TCASE(b"Change the default time system and see that the change is properly reflected in the behavior of STR2ET. DEFSYS = TDB ", ctx)?;

    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"TDB".clone(), ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"31 DEC 1989 15:59:00 ");
    spicelib::TPARSE(b"31 DEC 1989 15:59:00 ", &mut EXPDET[1], &mut ERROR, ctx)?;
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[1], 0.0000000000001, OK, ctx)?;

    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    testutil::TCASE(b"Change the default time system and see that the change is properly reflected in the behavior of STR2ET. DEFSYS = TDT ", ctx)?;

    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"TDT".clone(), ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"31 DEC 1989 15:59:00 ");
    spicelib::TPARSE(b"31 DEC 1989 15:59:00 ", &mut ET, &mut ERROR, ctx)?;
    EXPDET[1] = spicelib::UNITIM(ET, b"TDT", b"TDB", ctx)?;

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[1], 0.00000000000001, OK, ctx)?;

    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    testutil::TCASE(b"Change the default time zone and make sure the change is properly reflected in the action of STR2ET. ZONE = \'PST\' ", ctx)?;

    spicelib::TIMDEF(b"SET", b"ZONE", &mut b"PST".clone(), ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 12:42:55 1996");
    spicelib::UTC2ET(b"Fri Sep 27 20:42:55 1996", &mut EXPDET[1], ctx)?;

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~/", EXPDET[1], 0.00000000000001, OK, ctx)?;

    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    testutil::TCASE(b"Change the default calendar to JULIAN and make sure that we get the appropriate behavior from STR2ET. ", ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"JULIAN".clone(), ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Sep 3 12:42:55 1752");

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    spicelib::UTC2ET(b"Sep 14 12:42:55 1752", &mut EXPDET[1], ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~", EXPDET[1], 0.00000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"1752-200//12:42:55");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    spicelib::UTC2ET(b"1752-211//12:42:55", &mut EXPDET[1], ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~", EXPDET[1], 0.00000000000001, OK, ctx)?;

    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;

    testutil::TCASE(b"Change the default calendar to MIXED and make sure that we get the appropriate behavior from STR2ET. ", ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"MIXED".clone(), ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"Fri Sep 27 13:08:46 1996");

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    spicelib::UTC2ET(b"Fri Sep 27 13:08:46 1996", &mut EXPDET[1], ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~", EXPDET[1], 0.00000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"1582 OCT 3, 12:42:55");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    spicelib::UTC2ET(b"1582 OCT 13, 12:42:55", &mut EXPDET[1], ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~", EXPDET[1], 0.00000000000001, OK, ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"1582 OCT 13, 12:42:55");
    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    spicelib::UTC2ET(b"1582 OCT 13, 12:42:55", &mut EXPDET[1], ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~", EXPDET[1], 0.00000000000001, OK, ctx)?;

    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;

    testutil::TCASE(
        b"Make sure that we can handle leap seconds on the Julian calendar. ",
        ctx,
    )?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"JULIAN".clone(), ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"December 18, 1996 23:59:60.6");

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    spicelib::UTC2ET(b"December 31, 1996 23:59:60.6", &mut EXPDET[1], ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~", EXPDET[1], 0.00000000000001, OK, ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;

    testutil::TCASE(
        b"Make sure that we can handle leap seconds on the Julian calendar in non-UTC time zones. ",
        ctx,
    )?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"JULIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"ZONE", &mut b"PST".clone(), ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"December 18, 1996 15:59:60.6");

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    spicelib::UTC2ET(b"December 31, 1996 23:59:60.6", &mut EXPDET[1], ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSD(b"ET", ET, b"~", EXPDET[1], 0.00000000000001, OK, ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;

    testutil::TCASE(
        b"Make sure that seconds out of range are properly diagnosed on the Julian Calendar. ",
        ctx,
    )?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"JULIAN".clone(), ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"December 17, 1996 23:59:60.6");

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADTIMESTRING)", OK, ctx)?;

    testutil::TCASE(b"Make sure that out of range seconds are properly diagnosed on the Julian Calendar when using a non-UTC time zone. ", ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"JULIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"ZONE", &mut b"PST".clone(), ctx)?;

    fstr::assign(TSTRNG.get_mut(1), b"December 17, 1996 15:59:60.6");

    spicelib::STR2ET(&TSTRNG[1], &mut ET, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADTIMESTRING)", OK, ctx)?;

    spicelib::TPARCH(b"NO", ctx);

    //
    // Leave time system in default state.
    //
    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The documentation of TIMDEF states that the above
    // call sets the time zone to blank ( ' ' ), so
    // no call to set the time zone is required.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
