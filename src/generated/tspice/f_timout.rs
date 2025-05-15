//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LNSIZE: i32 = 80;
const LONGSZ: i32 = 240;

//$Procedure      F_TIMOUT ( Family of tests for TIMOUT )
pub fn F_TIMOUT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CASE = [b' '; LNSIZE as usize];
    let mut ERROR = [b' '; LONGSZ as usize];
    let mut GOODKR = ActualCharArray::new(LNSIZE, 1..=23);
    let mut MODIFY = ActualCharArray::new(WDSIZE, 1..=5);
    let mut PICTUR = [b' '; LNSIZE as usize];
    let mut STRING = [b' '; LNSIZE as usize];
    let mut TYPE = [b' '; WDSIZE as usize];
    let mut HOFF: f64 = 0.0;
    let mut MOFF: f64 = 0.0;
    let mut TVEC = StackArray::<f64, 10>::new(1..=10);
    let mut LAST: i32 = 0;
    let mut NLINES: i32 = 0;
    let mut NTVEC: i32 = 0;
    let mut YEAR: i32 = 0;
    let mut MODS: bool = false;
    let mut SUCCES: bool = false;
    let mut YABBRV: bool = false;

    //
    // Test Utility Functions
    //

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TIMOUT", ctx)?;
    //
    // The next sequence of test cases use TPARTV to parse a
    // string and then call TIMOUT to see if we can duplicate
    // the the input to TPARTV.
    //
    // We will need a leapsecond kernel to do this.
    //
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

    NLINES = 23;

    spicelib::CLPOOL(ctx)?;
    testutil::TSTTXT(b"testleap.ker", GOODKR.as_arg(), NLINES, true, false, ctx)?;

    fstr::assign(&mut CASE, b"1996 JAN 12 13:00:15.0 ");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"1996 JAN 12 02:00:15.0 A.M.");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"13 APR 1996 11:12:11.0");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"1996-122/ 13:00:15.0 ");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"1996-03-17T15:12:18.9 ");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"Mon Jun 17 10:45:26 PDT 1996");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;
    spicelib::PREFIX(b"::", 0, &mut MODIFY[3]);

    spicelib::ZZUTCPM(
        &MODIFY[3],
        1,
        &mut HOFF,
        &mut MOFF,
        &mut LAST,
        &mut SUCCES,
        ctx,
    );
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    TVEC[4] = (TVEC[4] - HOFF);
    TVEC[5] = (TVEC[5] - MOFF);

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"Wednesday June 19, 01:12:29.19 P.M. 1996");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    TVEC[4] = (TVEC[4] + 12.0);

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        &mut PICTUR,
        b"Weekday Month DD, AP:MN:SC.## AMPM YYYY ::RND",
    );

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(
        &mut CASE,
        b"Wednesday June 19, 01:12:29.19 1996 (From Gregorian to Julian)",
    );
    testutil::TCASE(&CASE, ctx)?;

    fstr::assign(&mut CASE, b"Wednesday June 19, 01:12:29.19 1996");
    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    fstr::assign(&mut CASE, b"Wednesday June 06, 01:12:29.19 1996 (Julian)");
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(
        &mut PICTUR,
        b"Weekday Month DD, HR:MN:SC.## YYYY (Julian)::RND::JCAL",
    );

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"Mon Jun 17 10:45:26 PDT \'96");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    YEAR = TVEC[1] as i32;
    spicelib::TEXPYR(&mut YEAR, ctx);
    TVEC[1] = YEAR as f64;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;
    spicelib::PREFIX(b"::", 0, &mut MODIFY[3]);

    spicelib::ZZUTCPM(
        &MODIFY[3],
        1,
        &mut HOFF,
        &mut MOFF,
        &mut LAST,
        &mut SUCCES,
        ctx,
    );
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    TVEC[4] = (TVEC[4] - HOFF);
    TVEC[5] = (TVEC[5] - MOFF);

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    testutil::TCASE(
        b"Check to make sure time zones have their leapseconds at the right time. ",
        ctx,
    )?;

    fstr::assign(&mut CASE, b"1989 DEC 31 23:59:60.5 ");

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut PICTUR, b"YYYY MON DD HR:MN:SC.# PDT ::RND::UTC-7");
    fstr::assign(&mut CASE, b"1989 DEC 31 16:59:60.5 PDT");

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"Oct 12, 1492 A.D.");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut PICTUR, b"Mon DD, YYYY ERA ::JCAL ");

    fstr::assign(&mut CASE, b"Oct 03, 1492 A.D.");
    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"1995 MAY 13 12:28:29.281 TDB ");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut PICTUR, b"YYYY MON DD HR:MN:SC.### ::TDB::TRNC");

    spicelib::ETCAL(TVEC[1], &mut CASE, ctx);
    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    testutil::TCASE(b"-10000000.123 seconds past J2000.", ctx)?;

    TVEC[1] = -10000000.123;

    fstr::assign(&mut CASE, b"  -10000000.123");

    fstr::assign(&mut PICTUR, b"SP2000.### ::RND::TDB");

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"  123456789.123 ");
    testutil::TCASE(b"123456789.123 seconds past 1950", ctx)?;

    TVEC[1] = ((spicelib::SPD() * (spicelib::J1950() - spicelib::J2000())) + 123456789.123);

    fstr::assign(&mut PICTUR, b"SP1950.### ::RND::TDB");

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b" 321 B.C. MAR 15 12:00:00");
    testutil::TCASE(b"The ephemeris epoch of 321 B.C. MAR 15 12:00:00", ctx)?;

    TVEC[1] = -(86400.0 * (847362.0 - 74.0));
    fstr::assign(&mut PICTUR, b"YYYY ERA MON DD HR:MN:SC ::TDB");

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b" 321 B.C. MAR 15 12:00:00");
    testutil::TCASE(b"The ephemeris epoch of 321 B.C. MAR 15 12:00:00", ctx)?;

    TVEC[1] = -(86400.0 * (847362.0 - 74.0));
    fstr::assign(&mut PICTUR, b"YYYY ERA MON DD HR:MN:SC ::TDB");

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b" 321 B.C. MAR 15 12:00:00");
    testutil::TCASE(b"The ephemeris epoch of 321 B.C. MAR 15 12:00:00", ctx)?;

    TVEC[1] = -(86400.0 * (847362.0 - 74.0));
    fstr::assign(&mut PICTUR, b"YYYY?ERA?MON DD HR:MN:SC ::TDB");

    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"1998 MAY 13 12:28:29.281 TDB ");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut PICTUR, b"YYYY?ERA?MON DD HR:MN:SC.### ::TDB::TRNC");

    spicelib::ETCAL(TVEC[1], &mut CASE, ctx);
    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    fstr::assign(&mut CASE, b"398 A.D. MAY 13 12:28:29.281 TDB ");
    testutil::TCASE(&CASE, ctx)?;

    spicelib::TPARTV(
        &CASE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        &mut PICTUR,
        &mut ERROR,
        ctx,
    );

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, true, OK, ctx)?;

    spicelib::TTRANS(&TYPE, b"ET", TVEC.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    fstr::assign(&mut PICTUR, b"YYYY?ERA?MON DD HR:MN:SC.### ::TDB::TRNC");

    spicelib::ETCAL(TVEC[1], &mut CASE, ctx);
    spicelib::TIMOUT(TVEC[1], &PICTUR, &mut STRING, ctx)?;
    spicelib::LJUST(&STRING.clone(), &mut STRING);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &CASE, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
