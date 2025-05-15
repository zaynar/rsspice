//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LNSIZE: i32 = 80;

//$Procedure      F_TTRANS ( Family of tests for TTRANS )
pub fn F_TTRANS(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut INSYS = ActualCharArray::new(WDSIZE, 1..=21);
    let mut OUTSYS = ActualCharArray::new(WDSIZE, 1..=21);
    let mut NTVEC = ActualCharArray::new(WDSIZE, 1..=7);
    let mut STRING = [b' '; WDSIZE as usize];
    let mut ESTRNG = [b' '; WDSIZE as usize];
    let mut BADKER = ActualCharArray::new(LNSIZE, 1..=23);
    let mut GOODKR = ActualCharArray::new(LNSIZE, 1..=23);
    let mut ETVEC = StackArray::<f64, 7>::new(1..=7);
    let mut TVEC = StackArray::<f64, 7>::new(1..=7);
    let mut TVECS = StackArray2D::<f64, 147>::new(1..=7, 1..=21);
    let mut TDT1: f64 = 0.0;
    let mut TDT2: f64 = 0.0;
    let mut DIFF: f64 = 0.0;
    let mut JDUTC1: f64 = 0.0;
    let mut JDUTC2: f64 = 0.0;
    let mut SECNDS: f64 = 0.0;
    let mut JD: f64 = 0.0;
    let mut K: i32 = 0;
    let mut NLINES: i32 = 0;
    let mut SIZE = StackArray::<i32, 21>::new(1..=21);

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //
    //
    // Local Variables
    //

    NLINES = 23;

    testutil::BEGDAT(&mut BADKER[1]);
    fstr::assign(BADKER.get_mut(2), b" ");
    fstr::assign(BADKER.get_mut(3), b"DELTET/DELTA_T_A       =   32.184");
    fstr::assign(BADKER.get_mut(4), b"DELTET/K               =    1.657D-3");
    fstr::assign(BADKER.get_mut(5), b"DELTET/EB              =    1.671D-2");
    fstr::assign(
        BADKER.get_mut(6),
        b"DELTET/M               = (  6.239996D0   1.99096871D-7 )",
    );
    fstr::assign(BADKER.get_mut(7), b" ");
    fstr::assign(
        BADKER.get_mut(8),
        b"DELTET/DELTA_AT        = ( 10,   @1972-JAN-1",
    );
    fstr::assign(
        BADKER.get_mut(9),
        b"                           11,   @1972-JUL-1",
    );
    fstr::assign(
        BADKER.get_mut(10),
        b"                           12,   @1973-JAN-1",
    );
    fstr::assign(
        BADKER.get_mut(11),
        b"                           13,   @1974-JAN-1",
    );
    fstr::assign(
        BADKER.get_mut(12),
        b"                           14,   @1975-JAN-1",
    );
    fstr::assign(
        BADKER.get_mut(13),
        b"                           15,   @1976-JAN-1",
    );
    fstr::assign(
        BADKER.get_mut(14),
        b"                           16,   @1977-JAN-1",
    );
    fstr::assign(
        BADKER.get_mut(15),
        b"                           17,   @1978-JAN-1",
    );
    fstr::assign(
        BADKER.get_mut(16),
        b"                           18,   @1979-JAN-1",
    );
    fstr::assign(
        BADKER.get_mut(17),
        b"                           20,   @1981-JUL-1",
    );
    fstr::assign(
        BADKER.get_mut(18),
        b"                           19,   @1980-JAN-1",
    );
    fstr::assign(
        BADKER.get_mut(19),
        b"                           21,   @1982-JUL-1",
    );
    fstr::assign(
        BADKER.get_mut(20),
        b"                           22,   @1983-JUL-1",
    );
    fstr::assign(
        BADKER.get_mut(21),
        b"                           23,   @1985-JUL-1",
    );
    fstr::assign(
        BADKER.get_mut(22),
        b"                           24,   @1988-JAN-1",
    );
    fstr::assign(
        BADKER.get_mut(23),
        b"                           25,   @1990-JAN-1 )",
    );

    testutil::BEGDAT(&mut GOODKR[1]);
    fstr::assign(GOODKR.get_mut(2), b" ");
    fstr::assign(GOODKR.get_mut(3), b"DELTET/DELTA_T_A       =   32.184");
    fstr::assign(GOODKR.get_mut(4), b"DELTET/K               =    1.657D-3");
    fstr::assign(GOODKR.get_mut(5), b"DELTET/EB              =    1.671D-2");
    fstr::assign(
        GOODKR.get_mut(6),
        b"DELTET/M               = (  6.239996D0   1.99096871D-7 )",
    );
    fstr::assign(GOODKR.get_mut(7), b" ");
    fstr::assign(
        GOODKR.get_mut(8),
        b"DELTET/DELTA_AT        = ( 10,   @1972-JAN-1",
    );
    fstr::assign(
        GOODKR.get_mut(9),
        b"                           11,   @1972-JUL-1",
    );
    fstr::assign(
        GOODKR.get_mut(10),
        b"                           12,   @1973-JAN-1",
    );
    fstr::assign(
        GOODKR.get_mut(11),
        b"                           13,   @1974-JAN-1",
    );
    fstr::assign(
        GOODKR.get_mut(12),
        b"                           14,   @1975-JAN-1",
    );
    fstr::assign(
        GOODKR.get_mut(13),
        b"                           15,   @1976-JAN-1",
    );
    fstr::assign(
        GOODKR.get_mut(14),
        b"                           16,   @1977-JAN-1",
    );
    fstr::assign(
        GOODKR.get_mut(15),
        b"                           17,   @1978-JAN-1",
    );
    fstr::assign(
        GOODKR.get_mut(16),
        b"                           18,   @1979-JAN-1",
    );
    fstr::assign(
        GOODKR.get_mut(17),
        b"                           19,   @1980-JAN-1",
    );
    fstr::assign(
        GOODKR.get_mut(18),
        b"                           20,   @1981-JUL-1",
    );
    fstr::assign(
        GOODKR.get_mut(19),
        b"                           21,   @1982-JUL-1",
    );
    fstr::assign(
        GOODKR.get_mut(20),
        b"                           22,   @1983-JUL-1",
    );
    fstr::assign(
        GOODKR.get_mut(21),
        b"                           23,   @1985-JUL-1",
    );
    fstr::assign(
        GOODKR.get_mut(22),
        b"                           24,   @1988-JAN-1",
    );
    fstr::assign(
        GOODKR.get_mut(23),
        b"                           25,   @1990-JAN-1 )",
    );

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TTRANS", ctx)?;

    TVEC[1] = 1995.0;
    TVEC[2] = 2.0;
    TVEC[3] = 1.0;
    TVEC[4] = 0.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;
    TVEC[7] = 0.0;

    //
    // Try the "no LSK" test case twice to make sure we get
    // the same error diagnosis both times.
    //
    testutil::TCASE(b"Check the \'NOLEAPSECONDS\' exception. ", ctx)?;

    spicelib::CLPOOL(ctx)?;
    spicelib::TTRANS(b"ET", b"YMD", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOLEAPSECONDS)", OK, ctx)?;

    spicelib::TTRANS(b"ET", b"YMD", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOLEAPSECONDS)", OK, ctx)?;

    //
    // Try the "bad LSK" test case twice to make sure we get
    // the same error diagnosis both times.
    //
    testutil::TCASE(
        b"Check to make sure a leapsecond out of order error is properly diagnosed. ",
        ctx,
    )?;

    testutil::TSTTXT(b"testleap.ker", BADKER.as_arg(), NLINES, true, false, ctx)?;
    spicelib::TTRANS(b"ET", b"YMD", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADLEAPSECONDS)", OK, ctx)?;

    spicelib::TTRANS(b"ET", b"YMD", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADLEAPSECONDS)", OK, ctx)?;

    testutil::TCASE(b"Make sure that an unknown time system is properly diagnosed when the first system is unknown. ", ctx)?;

    spicelib::CLPOOL(ctx)?;
    testutil::TSTTXT(b"testleap.ker", GOODKR.as_arg(), NLINES, true, false, ctx)?;

    spicelib::TTRANS(b"ETF", b"YMD", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(UNKNONWNTIMESYSTEM)", OK, ctx)?;

    testutil::TCASE(b"Make sure that an unknown time system is properly diagnosed when the second system is unknown. ", ctx)?;

    spicelib::TTRANS(b"YMD", b"DMY", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(true, b"SPICE(UNKNONWNTIMESYSTEM)", OK, ctx)?;

    testutil::TCASE(b"Without checking the validity of TTRANS, make sure that all of the advertised time systems are recognized and that TVEC changes from it\'s input value to a different output value. ", ctx)?;

    fstr::assign(INSYS.get_mut(1), b"YMD");
    fstr::assign(INSYS.get_mut(2), b"YMDF");
    fstr::assign(INSYS.get_mut(3), b"YD");
    fstr::assign(INSYS.get_mut(4), b"YDF");
    fstr::assign(INSYS.get_mut(5), b"YD.D");
    fstr::assign(INSYS.get_mut(6), b"YD.DF");
    fstr::assign(INSYS.get_mut(7), b"DAYSEC");
    fstr::assign(INSYS.get_mut(8), b"DP2000");
    fstr::assign(INSYS.get_mut(9), b"JDUTC");
    fstr::assign(INSYS.get_mut(10), b"FORMAL");
    fstr::assign(INSYS.get_mut(11), b"YWD");
    fstr::assign(INSYS.get_mut(12), b"YWDF");
    fstr::assign(INSYS.get_mut(13), b"YMWD");
    fstr::assign(INSYS.get_mut(14), b"YMWDF");
    fstr::assign(INSYS.get_mut(15), b"TAI");
    fstr::assign(INSYS.get_mut(16), b"TDT");
    fstr::assign(INSYS.get_mut(17), b"TDB");
    fstr::assign(INSYS.get_mut(18), b"JED");
    fstr::assign(INSYS.get_mut(19), b"ET");
    fstr::assign(INSYS.get_mut(20), b"JDTDB");
    fstr::assign(INSYS.get_mut(21), b"JDTDT");

    for I in 1..=21 {
        fstr::assign(OUTSYS.get_mut(I), INSYS.get(I));
    }

    for J in 1..=21 {
        for I in 1..=7 {
            TVECS[[I, J]] = 0.0;
        }
    }
    //
    // Set up the test values for YMD
    //
    TVECS[[1, 1]] = 1995.0;
    TVECS[[2, 1]] = 1.0;
    TVECS[[3, 1]] = 2.0;
    SIZE[1] = 6;
    //
    // ... for YMDF
    //
    TVECS[[1, 2]] = 1995.0;
    TVECS[[2, 2]] = 1.0;
    TVECS[[3, 2]] = 2.0;
    SIZE[2] = 6;
    //
    // ... for YD
    //
    TVECS[[1, 3]] = 1995.0;
    TVECS[[2, 3]] = 2.0;
    SIZE[3] = 5;
    //
    // ... for YDF
    //
    TVECS[[1, 4]] = 1995.0;
    TVECS[[2, 4]] = 2.0;
    SIZE[4] = 5;
    //
    // ... for YD.D
    //
    TVECS[[1, 5]] = 1995.0;
    TVECS[[2, 5]] = 2.0;
    SIZE[5] = 2;
    //
    // ... for YD.DF
    //
    TVECS[[1, 6]] = 1995.0;
    TVECS[[2, 6]] = 2.0;
    SIZE[6] = 2;
    //
    // ... for DAYSEC
    //
    TVECS[[1, 7]] = 729039.0;
    TVECS[[2, 7]] = 20.0;
    SIZE[7] = 2;
    //
    // ... for DP2000
    //
    TVECS[[1, 8]] = -1850.0;
    TVECS[[2, 8]] = 20.0;
    SIZE[8] = 2;
    //
    // ... for JDUTC and FORMAL
    //
    TVECS[[1, 9]] = 2451525.0;
    TVECS[[1, 10]] = -31000000.0;
    SIZE[9] = 1;
    SIZE[10] = 1;
    //
    // ... for YWD
    //
    TVECS[[1, 11]] = 1995.0;
    TVECS[[2, 11]] = 2.0;
    TVECS[[3, 11]] = 2.0;
    SIZE[11] = 6;
    //
    // ... for YWDF
    //
    TVECS[[1, 12]] = 1995.0;
    TVECS[[2, 12]] = 2.0;
    TVECS[[3, 12]] = 2.0;
    SIZE[12] = 6;
    //
    // ... for YMWD
    //
    TVECS[[1, 13]] = 1995.0;
    TVECS[[2, 13]] = 2.0;
    TVECS[[3, 13]] = 2.0;
    TVECS[[4, 13]] = 1.0;
    SIZE[13] = 7;
    //
    // ... for YMWDF
    //
    TVECS[[1, 14]] = 1995.0;
    TVECS[[2, 14]] = 2.0;
    TVECS[[3, 14]] = 2.0;
    TVECS[[4, 14]] = 2.0;
    SIZE[14] = 7;
    //
    // ... for TAI, TDT, TDB, JED, ET, JDTDB, JDTDT
    //
    TVECS[[1, 15]] = -31000000.0;
    TVECS[[1, 16]] = -31000000.0;
    TVECS[[1, 17]] = -31000000.0;
    TVECS[[1, 18]] = 2451545.0;
    TVECS[[1, 19]] = -31000000.0;
    TVECS[[1, 20]] = 2451545.0;
    TVECS[[1, 21]] = 2451545.0;

    SIZE[15] = 1;
    SIZE[16] = 1;
    SIZE[17] = 1;
    SIZE[18] = 1;
    SIZE[19] = 1;
    SIZE[20] = 1;
    SIZE[21] = 1;

    for I in 1..=21 {
        for J in 1..=21 {
            {
                let m1__: i32 = 1;
                let m2__: i32 = 7;
                let m3__: i32 = 1;
                K = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    TVEC[K] = TVECS[[K, I]];
                    K += m3__;
                }
            }

            spicelib::TTRANS(&INSYS[I], &OUTSYS[J], TVEC.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
        }
    }

    testutil::TCASE(b"This test checks for internal consistency. Each pair is of possible input and output systems are converted \"there and back again\". ", ctx)?;

    fstr::assign(NTVEC.get_mut(1), b"TVEC(1)");
    fstr::assign(NTVEC.get_mut(2), b"TVEC(2)");
    fstr::assign(NTVEC.get_mut(3), b"TVEC(3)");
    fstr::assign(NTVEC.get_mut(4), b"TVEC(4)");
    fstr::assign(NTVEC.get_mut(5), b"TVEC(5)");
    fstr::assign(NTVEC.get_mut(6), b"TVEC(6)");
    fstr::assign(NTVEC.get_mut(7), b"TVEC(7)");

    for I in 1..=21 {
        for J in 1..=21 {
            if fstr::ne(fstr::substr(OUTSYS.get(J), 1..=1), b"J") {
                testutil::TSTMSG(b"#", b"Input system #, Output System #", ctx);
                testutil::TSTMSC(&INSYS[I], ctx);
                testutil::TSTMSC(&OUTSYS[J], ctx);

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = 7;
                    let m3__: i32 = 1;
                    K = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        TVEC[K] = TVECS[[K, I]];
                        K += m3__;
                    }
                }

                spicelib::TTRANS(&INSYS[I], &OUTSYS[J], TVEC.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;
                spicelib::TTRANS(&OUTSYS[J], &INSYS[I], TVEC.as_slice_mut(), ctx)?;
                testutil::CHCKXC(false, b" ", OK, ctx)?;

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = (SIZE[I] - 1);
                    let m3__: i32 = 1;
                    K = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        testutil::CHCKSD(&NTVEC[K], TVEC[K], b"=", TVECS[[K, I]], 0.0, OK, ctx)?;

                        K += m3__;
                    }
                }

                K = SIZE[I];
                testutil::CHCKSD(&NTVEC[K], TVEC[K], b"~", TVECS[[K, I]], 0.0000001, OK, ctx)?;
            }
        }
    }

    testutil::TCASE(b"Check to make sure that when we cross a leapsecond boundary in UTC components that the corresponding TDT change by the right amount.", ctx)?;

    TVEC[1] = 1989.0;
    TVEC[2] = 12.0;
    TVEC[3] = 31.0;
    TVEC[4] = 23.0;
    TVEC[5] = 59.0;
    TVEC[6] = 59.0;

    spicelib::TTRANS(b"YMD", b"TDT", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    TDT1 = TVEC[1];

    TVEC[1] = 1990.0;
    TVEC[2] = 1.0;
    TVEC[3] = 1.0;
    TVEC[4] = 0.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    spicelib::TTRANS(b"YMD", b"TDT", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    TDT2 = TVEC[1];
    DIFF = (TDT2 - TDT1);

    testutil::CHCKSD(b"DIFF", DIFF, b"=", 2.0, 0.0, OK, ctx)?;

    testutil::TCASE(b"Continuing the last case, make sure that TDT epochs that occur during leapseconds are properly transformed when returning to YMD UTC format. ", ctx)?;

    TVEC[1] = (TDT2 - 0.5);

    spicelib::TTRANS(b"TDT", b"YMD", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    ETVEC[1] = 1989.0;
    ETVEC[2] = 12.0;
    ETVEC[3] = 31.0;
    ETVEC[4] = 23.0;
    ETVEC[5] = 59.0;
    ETVEC[6] = 60.5;

    testutil::CHCKAD(
        b"TVEC",
        TVEC.as_slice(),
        b"=",
        ETVEC.as_slice(),
        6,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"Make sure that we can get the day of the week and week of the month correct. ",
        ctx,
    )?;

    TVEC[1] = 1996.0;
    TVEC[2] = 5.0;
    TVEC[3] = 23.0;
    TVEC[4] = 12.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    ETVEC[1] = 1996.0;
    ETVEC[2] = 5.0;
    ETVEC[3] = 4.0;
    ETVEC[4] = 5.0;
    ETVEC[5] = 12.0;
    ETVEC[6] = 0.0;
    ETVEC[7] = 0.0;

    spicelib::TTRANS(b"YMD", b"YMWD", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKAD(
        b"TVEC",
        TVEC.as_slice(),
        b"=",
        ETVEC.as_slice(),
        7,
        0.0,
        OK,
        ctx,
    )?;

    testutil::TCASE(
        b"The value of JDUTC should not change during a leapsecond, make sure that it doesn\'t. ",
        ctx,
    )?;

    TVEC[1] = 1989.0;
    TVEC[2] = 12.0;
    TVEC[3] = 31.0;
    TVEC[4] = 23.0;
    TVEC[5] = 59.0;
    TVEC[6] = 60.0;

    spicelib::TTRANS(b"YMD", b"JDUTC", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    JDUTC1 = TVEC[1];

    TVEC[1] = 1989.0;
    TVEC[2] = 12.0;
    TVEC[3] = 31.0;
    TVEC[4] = 23.0;
    TVEC[5] = 59.0;
    TVEC[6] = 60.5;

    spicelib::TTRANS(b"YMD", b"JDUTC", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    JDUTC2 = TVEC[1];

    testutil::CHCKSD(b"JDUTC", JDUTC2, b"=", JDUTC1, 0.0, OK, ctx)?;

    TVEC[1] = 1990.0;
    TVEC[2] = 1.0;
    TVEC[3] = 1.0;
    TVEC[4] = 0.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    spicelib::TTRANS(b"YMD", b"JDUTC", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    JDUTC2 = TVEC[1];

    testutil::CHCKSD(b"JDUTC", JDUTC2, b"=", JDUTC1, 0.0, OK, ctx)?;

    testutil::TCASE(b"Using a format time system we should be able to convert to FORMAL and then predict the string that will be returned by ETCAL. ", ctx)?;

    TVEC[1] = 1996.0;
    TVEC[2] = 5.0;
    TVEC[3] = 24.0;
    TVEC[4] = 3.0;
    TVEC[5] = 11.0;
    TVEC[6] = 12.0;

    fstr::assign(&mut ESTRNG, b"1996 MAY 24 03:11:12.000");

    spicelib::TTRANS(b"YMDF", b"FORMAL", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ETCAL(TVEC[1], &mut STRING, ctx);
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"We can also predict the TDT time string associated with an epoch and use ETCAL to check this prediction. ", ctx)?;

    TVEC[1] = 1996.0;
    TVEC[2] = 5.0;
    TVEC[3] = 24.0;
    TVEC[4] = 3.0;
    TVEC[5] = 11.0;
    TVEC[6] = 12.0;

    fstr::assign(&mut ESTRNG, b"1996 MAY 24 03:12:09.184");

    spicelib::TTRANS(b"YMD", b"TDT", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ETCAL(TVEC[1], &mut STRING, ctx);
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"There were two leapseconds in 1972, make sure that we can compute the actual number of seconds in 1972. ", ctx)?;

    TVEC[1] = 1972.0;
    TVEC[2] = 1.0;
    TVEC[3] = 1.0;
    TVEC[4] = 0.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    spicelib::TTRANS(b"YMD", b"TDT", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    TDT1 = TVEC[1];

    TVEC[1] = 1973.0;
    TVEC[2] = 1.0;
    TVEC[3] = 1.0;
    TVEC[4] = 0.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    spicelib::TTRANS(b"YMD", b"TDT", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    TDT2 = TVEC[1];
    DIFF = (TDT2 - TDT1);

    SECNDS = ((366.0 * 86400.0) + 2.0);

    testutil::CHCKSD(b"DIFF", DIFF, b"=", SECNDS, 0.0, OK, ctx)?;

    testutil::TCASE(
        b"Make sure that the FORMAL time associated with 2000 Jan 1 12:00:00 is zero. ",
        ctx,
    )?;

    TVEC[1] = 2000.0;
    TVEC[2] = 1.0;
    TVEC[3] = 1.0;
    TVEC[4] = 12.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    spicelib::TTRANS(b"YMDF", b"FORMAL", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"TVEC(1)", TVEC[1], b"=", 0.0, 0.0, OK, ctx)?;

    testutil::TCASE(
        b"Make sure that the TDT time associated with 2000 Jan 1 12:00:00 is zero. 57.184",
        ctx,
    )?;

    TVEC[1] = 2000.0;
    TVEC[2] = 1.0;
    TVEC[3] = 1.0;
    TVEC[4] = 12.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    spicelib::TTRANS(b"YMD", b"TDT", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"TVEC(1)", TVEC[1], b"=", 57.184, 0.0, OK, ctx)?;

    testutil::TCASE(
        b"Make sure that we get the correct julian date for 0.0D0 ET. ",
        ctx,
    )?;

    TVEC[1] = 0.0;

    spicelib::TTRANS(b"TDT", b"JDTDT", TVEC.subarray_mut(1), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    JD = spicelib::J2000();

    testutil::CHCKSD(b"JD", JD, b"=", TVEC[1], 0.0, OK, ctx)?;

    testutil::TCASE(b"Make sure that ancient epochs are handled according to specification.  We use ETCAL to assist with this. ", ctx)?;

    TVEC[1] = -333.0;
    TVEC[2] = 5.0;
    TVEC[3] = 24.0;
    TVEC[4] = 3.0;
    TVEC[5] = 11.0;
    TVEC[6] = 12.0;

    fstr::assign(&mut ESTRNG, b"334 B.C. MAY 24 03:11:12.000");

    spicelib::TTRANS(b"YMD", b"FORMAL", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::ETCAL(TVEC[1], &mut STRING, ctx);
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
