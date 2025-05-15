//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ERA: i32 = 1;
const WDAY: i32 = (ERA + 1);
const ZONE: i32 = (WDAY + 1);
const AMPM: i32 = (ZONE + 1);
const SYSTEM: i32 = (AMPM + 1);
const NMODS: i32 = SYSTEM;

//$Procedure      F_TCHECK ( Test the routine TCHECK )
pub fn F_TCHECK(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut YESNO = [b' '; 3 as usize];
    let mut MIN: i32 = 0;
    let mut SEC: i32 = 0;
    let mut TVEC = StackArray::<f64, 6>::new(1..=6);
    let mut TYPE = [b' '; 3 as usize];
    let mut MODS: bool = false;
    let mut MODIFY = ActualCharArray::new(5, 1..=NMODS);
    let mut PASS: bool = false;
    let mut ERROR = [b' '; 240 as usize];
    let mut MESSGE = [b' '; 240 as usize];

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
    // Explicitly set the MODIFY array before use.
    //
    for I in 1..=NMODS {
        fstr::assign(MODIFY.get_mut(I), b" ");
    }

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TCHECK", ctx)?;

    testutil::TCASE(b"Make sure we can set and get the checking status. ", ctx)?;

    spicelib::TPARCH(b"NO", ctx);
    spicelib::TCHCKD(&mut YESNO, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"YESNO", &YESNO, b"=", b"NO", OK, ctx)?;

    spicelib::TPARCH(b"yes", ctx);
    spicelib::TCHCKD(&mut YESNO, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"YESNO", &YESNO, b"=", b"YES", OK, ctx)?;

    spicelib::TPARCH(b"NO", ctx);
    spicelib::TCHCKD(&mut YESNO, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"YESNO", &YESNO, b"=", b"NO", OK, ctx)?;

    testutil::TCASE(
        b"With checking turned off make sure that no checks are performed. ",
        ctx,
    )?;

    TVEC[1] = 1993000000000.0;
    TVEC[2] = 23.0;
    TVEC[3] = 1024.0;
    TVEC[4] = -2.0;
    TVEC[5] = 127.0;
    TVEC[6] = -12.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  JAN -1, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 1.0;
    TVEC[3] = -1.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  JAN 32, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 1.0;
    TVEC[3] = 32.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Feb 30, 1996 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1996.0;
    TVEC[2] = 2.0;
    TVEC[3] = 30.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Feb 29, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 2.0;
    TVEC[3] = 29.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Mar 32, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 3.0;
    TVEC[3] = 32.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Apr 31, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 4.0;
    TVEC[3] = 31.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  May 32, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 5.0;
    TVEC[3] = 32.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Jun 31, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 6.0;
    TVEC[3] = 31.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Jul 32, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 7.0;
    TVEC[3] = 32.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Aug 32, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 8.0;
    TVEC[3] = 32.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Sep 31, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 9.0;
    TVEC[3] = 31.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Oct 32, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 10.0;
    TVEC[3] = 32.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Nov 31, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 11.0;
    TVEC[3] = 32.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Dec 32, 1995 12:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 12.0;
    TVEC[3] = 32.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Jul 31, 1995 24:13:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 7.0;
    TVEC[3] = 31.0;
    TVEC[4] = 24.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Jul 31, 1995 12:60:29", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 7.0;
    TVEC[3] = 31.0;
    TVEC[4] = 12.0;
    TVEC[5] = 60.0;
    TVEC[6] = 29.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Jul 31, 1995 12:13:60", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 7.0;
    TVEC[3] = 31.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 60.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Jul 31, 1995 13:13:29 A.M.", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 7.0;
    TVEC[3] = 31.0;
    TVEC[4] = 13.0;
    TVEC[5] = 13.0;
    TVEC[6] = 29.0;

    MODS = true;
    fstr::assign(MODIFY.get_mut(AMPM), b"A.M.");

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  \"Jul.3\" 1, 1995 00:00:00", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 7.3;
    TVEC[3] = 1.0;
    TVEC[4] = 0.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Jul 31.3, 1995 12:13:59", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 7.0;
    TVEC[3] = 31.3;
    TVEC[4] = 12.0;
    TVEC[5] = 13.0;
    TVEC[6] = 59.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Jul 31, 1995 12.3:13:59", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 7.0;
    TVEC[3] = 31.0;
    TVEC[4] = 12.3;
    TVEC[5] = 13.0;
    TVEC[6] = 59.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Jul 31, 1995 12:13.3:59", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 7.0;
    TVEC[3] = 31.0;
    TVEC[4] = 12.0;
    TVEC[5] = 13.3;
    TVEC[6] = 59.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  Jan  1, 1995.2 00:00:00", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.2;
    TVEC[2] = 1.0;
    TVEC[3] = 1.0;
    TVEC[4] = 0.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(b"Checking on:  1995-366// 00:00:00", ctx)?;

    spicelib::TPARCH(b"YES", ctx);

    TVEC[1] = 1995.0;
    TVEC[2] = 366.0;
    TVEC[3] = 0.0;
    TVEC[4] = 0.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    MODS = false;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, false, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"!=", b" ", OK, ctx)?;

    testutil::TCASE(
        b"With checking turned on, make sure that normal items are not rejected. (YMD) ",
        ctx,
    )?;

    spicelib::TPARCH(b"YES", ctx);

    MODS = false;
    fstr::assign(&mut TYPE, b"YMD");

    SEC = 0;
    MIN = 0;
    //
    // Here's the deal for every hour of every day in 1995 and
    // 1996 we generate a time vector that should be legal
    // and then check it.
    //
    for YEAR in 1995..=1996 {
        TVEC[1] = (YEAR as f64);

        for MON in 1..=12 {
            TVEC[2] = (MON as f64);

            for DAY in 1..=31 {
                if (((MON == 2) && (YEAR == 1996)) && (DAY > 29)) {
                    TVEC[3] = 29.0;
                } else if ((MON == 2) && (DAY > 28)) {
                    TVEC[3] = 28.0;
                } else if ((DAY == 31)
                    && ((((MON == 4) || (MON == 6)) || (MON == 9)) || (MON == 11)))
                {
                    TVEC[3] = 30.0;
                } else {
                    TVEC[3] = (DAY as f64);
                }

                for HR in 0..=23 {
                    MIN = (MIN + 23);
                    if (MIN >= 60) {
                        MIN = (MIN - 60);
                    }

                    SEC = (SEC + 17);

                    if (SEC >= 60) {
                        SEC = (SEC - 60);
                    }

                    TVEC[4] = (HR as f64);
                    TVEC[5] = (MIN as f64);
                    TVEC[6] = (SEC as f64);

                    spicelib::TCHECK(
                        TVEC.as_slice(),
                        &TYPE,
                        MODS,
                        MODIFY.as_arg(),
                        &mut PASS,
                        &mut ERROR,
                        ctx,
                    );
                    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;

                    if !*OK {
                        fstr::assign(
                            &mut MESSGE,
                            b"Year #, Month #, Day #, Hour #, Minute #, Second #. ",
                        );

                        spicelib::REPMI(&MESSGE.clone(), b"#", YEAR, &mut MESSGE, ctx);
                        spicelib::REPMI(&MESSGE.clone(), b"#", MON, &mut MESSGE, ctx);
                        spicelib::REPMI(&MESSGE.clone(), b"#", DAY, &mut MESSGE, ctx);
                        spicelib::REPMI(&MESSGE.clone(), b"#", HR, &mut MESSGE, ctx);
                        spicelib::REPMI(&MESSGE.clone(), b"#", MIN, &mut MESSGE, ctx);
                        spicelib::REPMI(&MESSGE.clone(), b"#", SEC, &mut MESSGE, ctx);

                        testutil::TSTLOG(&MESSGE, true, ctx)?;
                        testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;
                    }
                }
            }
        }
    }

    testutil::TCASE(
        b"With checking turned on, make sure that normal items are not rejected. (YD) ",
        ctx,
    )?;

    spicelib::TPARCH(b"YES", ctx);

    MODS = false;
    fstr::assign(&mut TYPE, b"YD");

    for YEAR in 1995..=1996 {
        TVEC[1] = (YEAR as f64);

        for DAY in 1..=366 {
            if ((DAY == 366) && (YEAR == 1995)) {
                TVEC[2] = 365.0;
            } else {
                TVEC[2] = (DAY as f64);
            }

            for HR in 0..=23 {
                MIN = (MIN + 23);
                if (MIN >= 60) {
                    MIN = (MIN - 60);
                }

                SEC = (SEC + 17);

                if (SEC >= 60) {
                    SEC = (SEC - 60);
                }

                TVEC[3] = (HR as f64);
                TVEC[4] = (MIN as f64);
                TVEC[5] = (SEC as f64);

                spicelib::TCHECK(
                    TVEC.as_slice(),
                    &TYPE,
                    MODS,
                    MODIFY.as_arg(),
                    &mut PASS,
                    &mut ERROR,
                    ctx,
                );
                testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;

                if !*OK {
                    fstr::assign(&mut MESSGE, b"Year #, Day #, Hour #, Minute #, Second #. ");

                    spicelib::REPMI(&MESSGE.clone(), b"#", YEAR, &mut MESSGE, ctx);
                    spicelib::REPMI(&MESSGE.clone(), b"#", DAY, &mut MESSGE, ctx);
                    spicelib::REPMI(&MESSGE.clone(), b"#", HR, &mut MESSGE, ctx);
                    spicelib::REPMI(&MESSGE.clone(), b"#", MIN, &mut MESSGE, ctx);
                    spicelib::REPMI(&MESSGE.clone(), b"#", SEC, &mut MESSGE, ctx);

                    testutil::TSTLOG(&MESSGE, true, ctx)?;
                    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;
                }
            }
        }
    }

    testutil::TCASE(
        b"Make sure that an epoch during a potential leapsecond is not regarded as erroneous. ",
        ctx,
    )?;

    TVEC[1] = 1995.0;
    TVEC[2] = 12.0;
    TVEC[3] = 31.0;
    TVEC[4] = 23.0;
    TVEC[5] = 59.0;
    TVEC[6] = 60.1;

    spicelib::TPARCH(b"YES", ctx);

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1995 DEC 31 23:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1995.0;
    TVEC[2] = 6.0;
    TVEC[3] = 30.0;
    TVEC[4] = 23.0;
    TVEC[5] = 59.0;
    TVEC[6] = 60.1;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1995 JUN 30 23:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1996.0;
    TVEC[2] = 12.0;
    TVEC[3] = 31.0;
    TVEC[4] = 23.0;
    TVEC[5] = 59.0;
    TVEC[6] = 60.1;

    spicelib::TPARCH(b"YES", ctx);

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1996 DEC 31 23:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1996.0;
    TVEC[2] = 6.0;
    TVEC[3] = 30.0;
    TVEC[4] = 23.0;
    TVEC[5] = 59.0;
    TVEC[6] = 60.1;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1996 JUN 30 23:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1995.0;
    TVEC[2] = 365.0;
    TVEC[3] = 23.0;
    TVEC[4] = 59.0;
    TVEC[5] = 60.1;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1995-365 // 23:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1996.0;
    TVEC[2] = 366.0;
    TVEC[3] = 23.0;
    TVEC[4] = 59.0;
    TVEC[5] = 60.1;

    spicelib::TPARCH(b"YES", ctx);

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1996-366 // 23:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1995.0;
    TVEC[2] = 181.0;
    TVEC[3] = 23.0;
    TVEC[4] = 59.0;
    TVEC[5] = 60.1;

    spicelib::TPARCH(b"YES", ctx);

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1995-182 // 23:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1996.0;
    TVEC[2] = 182.0;
    TVEC[3] = 23.0;
    TVEC[4] = 59.0;
    TVEC[5] = 60.1;

    spicelib::TPARCH(b"YES", ctx);

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1995-183 // 23:59:60.1", true, ctx)?;
    }

    testutil::TCASE(b"Make sure that an epoch during a potential leapsecond is not regarded as erroneous. ( 11:00 P.M. subcase.", ctx)?;

    TVEC[1] = 1995.0;
    TVEC[2] = 12.0;
    TVEC[3] = 31.0;
    TVEC[4] = 11.0;
    TVEC[5] = 59.0;
    TVEC[6] = 60.1;

    MODS = true;
    fstr::assign(MODIFY.get_mut(AMPM), b"P.M.");

    spicelib::TPARCH(b"YES", ctx);

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1995 DEC 31 P.M. 11:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1995.0;
    TVEC[2] = 6.0;
    TVEC[3] = 30.0;
    TVEC[4] = 11.0;
    TVEC[5] = 59.0;
    TVEC[6] = 60.1;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1995 JUN 30 P.M. 11:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1996.0;
    TVEC[2] = 12.0;
    TVEC[3] = 31.0;
    TVEC[4] = 11.0;
    TVEC[5] = 59.0;
    TVEC[6] = 60.1;

    spicelib::TPARCH(b"YES", ctx);

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1996 DEC 31 P.M. 11:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1996.0;
    TVEC[2] = 6.0;
    TVEC[3] = 30.0;
    TVEC[4] = 11.0;
    TVEC[5] = 59.0;
    TVEC[6] = 60.1;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1996 JUN 30 P.M. 11:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1995.0;
    TVEC[2] = 365.0;
    TVEC[3] = 11.0;
    TVEC[4] = 59.0;
    TVEC[5] = 60.1;

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1995-365 // P.M. 11:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1996.0;
    TVEC[2] = 366.0;
    TVEC[3] = 11.0;
    TVEC[4] = 59.0;
    TVEC[5] = 60.1;

    spicelib::TPARCH(b"YES", ctx);

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1996-366 // P.M. 11:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1995.0;
    TVEC[2] = 181.0;
    TVEC[3] = 11.0;
    TVEC[4] = 59.0;
    TVEC[5] = 60.1;

    spicelib::TPARCH(b"YES", ctx);

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1995-182 // P.M. 11:59:60.1", true, ctx)?;
    }

    TVEC[1] = 1996.0;
    TVEC[2] = 182.0;
    TVEC[3] = 11.0;
    TVEC[4] = 59.0;
    TVEC[5] = 60.1;

    spicelib::TPARCH(b"YES", ctx);

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"1995-183 // P.M. 11:59:60.1", true, ctx)?;
    }

    spicelib::TPARCH(b"NO", ctx);

    testutil::TCASE(b"Make sure that B.C. leap years are not rejected.", ctx)?;

    //
    // Set the year to 1 B.C.
    //
    TVEC[1] = 1.0;
    TVEC[2] = 2.0;
    TVEC[3] = 29.0;
    TVEC[4] = 0.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    MODS = true;

    for I in 1..=NMODS {
        fstr::assign(MODIFY.get_mut(I), b" ");
    }

    spicelib::TPARCH(b"YES", ctx);

    fstr::assign(MODIFY.get_mut(ERA), b"B.C.");

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"B.C. 1 FEB 29", true, ctx)?;
    }

    //
    // Set the year to 109 B.C.
    //
    TVEC[1] = 109.0;
    TVEC[2] = 2.0;
    TVEC[3] = 29.0;
    TVEC[4] = 0.0;
    TVEC[5] = 0.0;
    TVEC[6] = 0.0;

    MODS = true;

    for I in 1..=NMODS {
        fstr::assign(MODIFY.get_mut(I), b" ");
    }

    spicelib::TPARCH(b"YES", ctx);

    fstr::assign(MODIFY.get_mut(ERA), b"B.C.");

    spicelib::TCHECK(
        TVEC.as_slice(),
        b"YMD",
        MODS,
        MODIFY.as_arg(),
        &mut PASS,
        &mut ERROR,
        ctx,
    );
    testutil::CHCKSL(b"PASS", PASS, true, OK, ctx)?;
    testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;

    if !*OK {
        testutil::TSTLOG(b"B.C. 109 FEB 29", true, ctx)?;
    }

    spicelib::TPARCH(b"NO", ctx);

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
