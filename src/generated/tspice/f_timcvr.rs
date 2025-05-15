//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const WDSIZE: i32 = 32;

//$Procedure      F_TIMCVR ( TIMOUT tests to exercise branch coverage)
pub fn F_TIMCVR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut PICTUR = [b' '; LNSIZE as usize];
    let mut EXPCTD = ActualCharArray::new(LNSIZE, 1..=48);
    let mut FTIME = [b' '; LNSIZE as usize];
    let mut TIMES = ActualCharArray::new(LNSIZE, 1..=48);
    let mut NAME = [b' '; WDSIZE as usize];
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
    testutil::TOPEN(b"F_TIMCVR", ctx)?;

    fstr::assign(TIMES.get_mut(1), b"12 B.C. JAN 01 10:00:00 ");
    fstr::assign(TIMES.get_mut(2), b"12 B.C. FEB 05 11:00:00 ");
    fstr::assign(TIMES.get_mut(3), b"12 B.C. MAR 10 12:00:00 ");
    fstr::assign(TIMES.get_mut(4), b"12 B.C. APR 15 13:00:00 ");
    fstr::assign(TIMES.get_mut(5), b"12 B.C. MAY 20 14:00:00 ");
    fstr::assign(TIMES.get_mut(6), b"12 B.C. JUN 25 15:00:00 ");
    fstr::assign(TIMES.get_mut(7), b"12 B.C. JUL 01 16:00:00 ");
    fstr::assign(TIMES.get_mut(8), b"12 B.C. AUG 10 17:00:00 ");
    fstr::assign(TIMES.get_mut(9), b"12 B.C. SEP 15 18:00:00 ");
    fstr::assign(TIMES.get_mut(10), b"12 B.C. OCT 20 19:00:00 ");
    fstr::assign(TIMES.get_mut(11), b"12 B.C. NOV 25 20:00:00 ");
    fstr::assign(TIMES.get_mut(12), b"12 B.C. DEC 31 21:00:00 ");

    fstr::assign(TIMES.get_mut(13), b"141 A.D. JAN 01 10:00:00 ");
    fstr::assign(TIMES.get_mut(14), b"141 A.D. FEB 05 11:00:00 ");
    fstr::assign(TIMES.get_mut(15), b"141 A.D. MAR 10 12:00:00 ");
    fstr::assign(TIMES.get_mut(16), b"141 A.D. APR 15 13:00:00 ");
    fstr::assign(TIMES.get_mut(17), b"141 A.D. MAY 20 14:00:00 ");
    fstr::assign(TIMES.get_mut(18), b"141 A.D. JUN 25 15:00:00 ");
    fstr::assign(TIMES.get_mut(19), b"141 A.D. JUL 01 16:00:00 ");
    fstr::assign(TIMES.get_mut(20), b"141 A.D. AUG 10 17:00:00 ");
    fstr::assign(TIMES.get_mut(21), b"141 A.D. SEP 15 18:00:00 ");
    fstr::assign(TIMES.get_mut(22), b"141 A.D. OCT 20 19:00:00 ");
    fstr::assign(TIMES.get_mut(23), b"141 A.D. NOV 25 20:00:00 ");
    fstr::assign(TIMES.get_mut(24), b"141 A.D. DEC 31 21:00:00 ");

    fstr::assign(TIMES.get_mut(25), b"1582 JAN 01  10:00:00 ");
    fstr::assign(TIMES.get_mut(26), b"1582 FEB 05  11:00:00 ");
    fstr::assign(TIMES.get_mut(27), b"1582 MAR 10  12:00:00 ");
    fstr::assign(TIMES.get_mut(28), b"1582 APR 15  13:00:00 ");
    fstr::assign(TIMES.get_mut(29), b"1582 MAY 20  14:00:00 ");
    fstr::assign(TIMES.get_mut(30), b"1582 JUN 25  15:00:00 ");
    fstr::assign(TIMES.get_mut(31), b"1582 JUL 01  16:00:00 ");
    fstr::assign(TIMES.get_mut(32), b"1582 AUG 10  17:00:00 ");
    fstr::assign(TIMES.get_mut(33), b"1582 SEP 15  18:00:00 ");
    fstr::assign(TIMES.get_mut(34), b"1582 OCT 12  19:00:00 ");
    fstr::assign(TIMES.get_mut(35), b"1582 OCT 25  20:00:00 ");
    fstr::assign(TIMES.get_mut(36), b"1582 DEC 31  21:00:00 ");

    fstr::assign(TIMES.get_mut(37), b"1986 JAN 01 10:00:00 ");
    fstr::assign(TIMES.get_mut(38), b"1986 FEB 05 11:00:00 ");
    fstr::assign(TIMES.get_mut(39), b"1986 MAR 10 12:00:00 ");
    fstr::assign(TIMES.get_mut(40), b"1986 APR 15 13:00:00 ");
    fstr::assign(TIMES.get_mut(41), b"1986 MAY 20 14:00:00 ");
    fstr::assign(TIMES.get_mut(42), b"1986 JUN 25 15:00:00 ");
    fstr::assign(TIMES.get_mut(43), b"1986 JUL 01 16:00:00 ");
    fstr::assign(TIMES.get_mut(44), b"1986 AUG 10 17:00:00 ");
    fstr::assign(TIMES.get_mut(45), b"1986 SEP 15 18:00:00 ");
    fstr::assign(TIMES.get_mut(46), b"1986 OCT 20 19:00:00 ");
    fstr::assign(TIMES.get_mut(47), b"1986 NOV 25 20:00:00 ");
    fstr::assign(TIMES.get_mut(48), b"1986 DEC 31 21:00:00 ");
    //
    // Clear the kernel pool and then load a leapseconds kernel.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::TSTLSK(ctx)?;

    fstr::assign(&mut PICTUR, b"YYYY ERA MON DD AP:MN:SC.### ampm PDT ::RND");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b"  12 B.C. JAN 01 10:00:00.000 a.m. PDT");
    fstr::assign(EXPCTD.get_mut(2), b"  12 B.C. FEB 05 11:00:00.000 a.m. PDT");
    fstr::assign(EXPCTD.get_mut(3), b"  12 B.C. MAR 10 12:00:00.000 p.m. PDT");
    fstr::assign(EXPCTD.get_mut(4), b"  12 B.C. APR 15 01:00:00.000 p.m. PDT");
    fstr::assign(EXPCTD.get_mut(5), b"  12 B.C. MAY 20 02:00:00.000 p.m. PDT");
    fstr::assign(EXPCTD.get_mut(6), b"  12 B.C. JUN 25 03:00:00.000 p.m. PDT");
    fstr::assign(EXPCTD.get_mut(7), b"  12 B.C. JUL 01 04:00:00.000 p.m. PDT");
    fstr::assign(EXPCTD.get_mut(8), b"  12 B.C. AUG 10 05:00:00.000 p.m. PDT");
    fstr::assign(EXPCTD.get_mut(9), b"  12 B.C. SEP 15 06:00:00.000 p.m. PDT");
    fstr::assign(
        EXPCTD.get_mut(10),
        b"  12 B.C. OCT 20 07:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(11),
        b"  12 B.C. NOV 25 08:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(12),
        b"  12 B.C. DEC 31 09:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(13),
        b" 141 A.D. JAN 01 10:00:00.000 a.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(14),
        b" 141 A.D. FEB 05 11:00:00.000 a.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(15),
        b" 141 A.D. MAR 10 12:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(16),
        b" 141 A.D. APR 15 01:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(17),
        b" 141 A.D. MAY 20 02:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(18),
        b" 141 A.D. JUN 25 03:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(19),
        b" 141 A.D. JUL 01 04:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(20),
        b" 141 A.D. AUG 10 05:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(21),
        b" 141 A.D. SEP 15 06:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(22),
        b" 141 A.D. OCT 20 07:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(23),
        b" 141 A.D. NOV 25 08:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(24),
        b" 141 A.D. DEC 31 09:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(25),
        b"1582 A.D. JAN 01 10:00:00.000 a.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(26),
        b"1582 A.D. FEB 05 11:00:00.000 a.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(27),
        b"1582 A.D. MAR 10 12:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(28),
        b"1582 A.D. APR 15 01:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(29),
        b"1582 A.D. MAY 20 02:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(30),
        b"1582 A.D. JUN 25 03:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(31),
        b"1582 A.D. JUL 01 04:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(32),
        b"1582 A.D. AUG 10 05:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(33),
        b"1582 A.D. SEP 15 06:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(34),
        b"1582 A.D. OCT 12 07:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(35),
        b"1582 A.D. OCT 25 08:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(36),
        b"1582 A.D. DEC 31 09:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(37),
        b"1986 A.D. JAN 01 10:00:00.000 a.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(38),
        b"1986 A.D. FEB 05 11:00:00.000 a.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(39),
        b"1986 A.D. MAR 10 12:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(40),
        b"1986 A.D. APR 15 01:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(41),
        b"1986 A.D. MAY 20 02:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(42),
        b"1986 A.D. JUN 25 03:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(43),
        b"1986 A.D. JUL 01 04:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(44),
        b"1986 A.D. AUG 10 05:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(45),
        b"1986 A.D. SEP 15 06:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(46),
        b"1986 A.D. OCT 20 07:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(47),
        b"1986 A.D. NOV 25 08:00:00.000 p.m. PDT",
    );
    fstr::assign(
        EXPCTD.get_mut(48),
        b"1986 A.D. DEC 31 09:00:00.000 p.m. PDT",
    );

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY MM DD Weekday HR:MN:SC.## ::TRNC ");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 01 01 Sunday 10:00:41.18");
    fstr::assign(EXPCTD.get_mut(2), b" -11 02 05 Sunday 11:00:41.18");
    fstr::assign(EXPCTD.get_mut(3), b" -11 03 10 Friday 12:00:41.18");
    fstr::assign(EXPCTD.get_mut(4), b" -11 04 15 Saturday 13:00:41.18");
    fstr::assign(EXPCTD.get_mut(5), b" -11 05 20 Saturday 14:00:41.18");
    fstr::assign(EXPCTD.get_mut(6), b" -11 06 25 Sunday 15:00:41.18");
    fstr::assign(EXPCTD.get_mut(7), b" -11 07 01 Saturday 16:00:41.18");
    fstr::assign(EXPCTD.get_mut(8), b" -11 08 10 Thursday 17:00:41.18");
    fstr::assign(EXPCTD.get_mut(9), b" -11 09 15 Friday 18:00:41.18");
    fstr::assign(EXPCTD.get_mut(10), b" -11 10 20 Friday 19:00:41.18");
    fstr::assign(EXPCTD.get_mut(11), b" -11 11 25 Saturday 20:00:41.18");
    fstr::assign(EXPCTD.get_mut(12), b" -11 12 31 Sunday 21:00:41.18");
    fstr::assign(EXPCTD.get_mut(13), b" 141 01 01 Sunday 10:00:41.18");
    fstr::assign(EXPCTD.get_mut(14), b" 141 02 05 Sunday 11:00:41.18");
    fstr::assign(EXPCTD.get_mut(15), b" 141 03 10 Friday 12:00:41.18");
    fstr::assign(EXPCTD.get_mut(16), b" 141 04 15 Saturday 13:00:41.18");
    fstr::assign(EXPCTD.get_mut(17), b" 141 05 20 Saturday 14:00:41.18");
    fstr::assign(EXPCTD.get_mut(18), b" 141 06 25 Sunday 15:00:41.18");
    fstr::assign(EXPCTD.get_mut(19), b" 141 07 01 Saturday 16:00:41.18");
    fstr::assign(EXPCTD.get_mut(20), b" 141 08 10 Thursday 17:00:41.18");
    fstr::assign(EXPCTD.get_mut(21), b" 141 09 15 Friday 18:00:41.18");
    fstr::assign(EXPCTD.get_mut(22), b" 141 10 20 Friday 19:00:41.18");
    fstr::assign(EXPCTD.get_mut(23), b" 141 11 25 Saturday 20:00:41.18");
    fstr::assign(EXPCTD.get_mut(24), b" 141 12 31 Sunday 21:00:41.18");
    fstr::assign(EXPCTD.get_mut(25), b"1582 01 01 Friday 10:00:41.18");
    fstr::assign(EXPCTD.get_mut(26), b"1582 02 05 Friday 11:00:41.18");
    fstr::assign(EXPCTD.get_mut(27), b"1582 03 10 Wednesday 12:00:41.18");
    fstr::assign(EXPCTD.get_mut(28), b"1582 04 15 Thursday 13:00:41.18");
    fstr::assign(EXPCTD.get_mut(29), b"1582 05 20 Thursday 14:00:41.18");
    fstr::assign(EXPCTD.get_mut(30), b"1582 06 25 Friday 15:00:41.18");
    fstr::assign(EXPCTD.get_mut(31), b"1582 07 01 Thursday 16:00:41.18");
    fstr::assign(EXPCTD.get_mut(32), b"1582 08 10 Tuesday 17:00:41.18");
    fstr::assign(EXPCTD.get_mut(33), b"1582 09 15 Wednesday 18:00:41.18");
    fstr::assign(EXPCTD.get_mut(34), b"1582 10 12 Tuesday 19:00:41.18");
    fstr::assign(EXPCTD.get_mut(35), b"1582 10 25 Monday 20:00:41.18");
    fstr::assign(EXPCTD.get_mut(36), b"1582 12 31 Friday 21:00:41.18");
    fstr::assign(EXPCTD.get_mut(37), b"1986 01 01 Wednesday 10:00:55.18");
    fstr::assign(EXPCTD.get_mut(38), b"1986 02 05 Wednesday 11:00:55.18");
    fstr::assign(EXPCTD.get_mut(39), b"1986 03 10 Monday 12:00:55.18");
    fstr::assign(EXPCTD.get_mut(40), b"1986 04 15 Tuesday 13:00:55.18");
    fstr::assign(EXPCTD.get_mut(41), b"1986 05 20 Tuesday 14:00:55.18");
    fstr::assign(EXPCTD.get_mut(42), b"1986 06 25 Wednesday 15:00:55.18");
    fstr::assign(EXPCTD.get_mut(43), b"1986 07 01 Tuesday 16:00:55.18");
    fstr::assign(EXPCTD.get_mut(44), b"1986 08 10 Sunday 17:00:55.18");
    fstr::assign(EXPCTD.get_mut(45), b"1986 09 15 Monday 18:00:55.18");
    fstr::assign(EXPCTD.get_mut(46), b"1986 10 20 Monday 19:00:55.18");
    fstr::assign(EXPCTD.get_mut(47), b"1986 11 25 Tuesday 20:00:55.18");
    fstr::assign(EXPCTD.get_mut(48), b"1986 12 31 Wednesday 21:00:55.18");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &fstr::concat(&PICTUR, b"::TDT"), &mut FTIME, ctx)?;

        fstr::assign(&mut NAME, b"FTIME(#) TDT");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;

        spicelib::TIMOUT(ET, &fstr::concat(&PICTUR, b"::TT"), &mut FTIME, ctx)?;

        fstr::assign(&mut NAME, b"FTIME(#) TT");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY Mon DD HR:MN:SC.## Wkd");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"JULIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"TDB".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 Jan 01 10:00:00.00 Fri");
    fstr::assign(EXPCTD.get_mut(2), b" -11 Feb 05 11:00:00.00 Fri");
    fstr::assign(EXPCTD.get_mut(3), b" -11 Mar 10 12:00:00.00 Wed");
    fstr::assign(EXPCTD.get_mut(4), b" -11 Apr 15 13:00:00.00 Thu");
    fstr::assign(EXPCTD.get_mut(5), b" -11 May 20 14:00:00.00 Thu");
    fstr::assign(EXPCTD.get_mut(6), b" -11 Jun 25 15:00:00.00 Fri");
    fstr::assign(EXPCTD.get_mut(7), b" -11 Jul 01 16:00:00.00 Thu");
    fstr::assign(EXPCTD.get_mut(8), b" -11 Aug 10 17:00:00.00 Tue");
    fstr::assign(EXPCTD.get_mut(9), b" -11 Sep 15 18:00:00.00 Wed");
    fstr::assign(EXPCTD.get_mut(10), b" -11 Oct 20 19:00:00.00 Wed");
    fstr::assign(EXPCTD.get_mut(11), b" -11 Nov 25 20:00:00.00 Thu");
    fstr::assign(EXPCTD.get_mut(12), b" -11 Dec 31 21:00:00.00 Fri");
    fstr::assign(EXPCTD.get_mut(13), b" 141 Jan 01 10:00:00.00 Sat");
    fstr::assign(EXPCTD.get_mut(14), b" 141 Feb 05 11:00:00.00 Sat");
    fstr::assign(EXPCTD.get_mut(15), b" 141 Mar 10 12:00:00.00 Thu");
    fstr::assign(EXPCTD.get_mut(16), b" 141 Apr 15 13:00:00.00 Fri");
    fstr::assign(EXPCTD.get_mut(17), b" 141 May 20 14:00:00.00 Fri");
    fstr::assign(EXPCTD.get_mut(18), b" 141 Jun 25 15:00:00.00 Sat");
    fstr::assign(EXPCTD.get_mut(19), b" 141 Jul 01 16:00:00.00 Fri");
    fstr::assign(EXPCTD.get_mut(20), b" 141 Aug 10 17:00:00.00 Wed");
    fstr::assign(EXPCTD.get_mut(21), b" 141 Sep 15 18:00:00.00 Thu");
    fstr::assign(EXPCTD.get_mut(22), b" 141 Oct 20 19:00:00.00 Thu");
    fstr::assign(EXPCTD.get_mut(23), b" 141 Nov 25 20:00:00.00 Fri");
    fstr::assign(EXPCTD.get_mut(24), b" 141 Dec 31 21:00:00.00 Sat");
    fstr::assign(EXPCTD.get_mut(25), b"1582 Jan 01 10:00:00.00 Mon");
    fstr::assign(EXPCTD.get_mut(26), b"1582 Feb 05 11:00:00.00 Mon");
    fstr::assign(EXPCTD.get_mut(27), b"1582 Mar 10 12:00:00.00 Sat");
    fstr::assign(EXPCTD.get_mut(28), b"1582 Apr 15 13:00:00.00 Sun");
    fstr::assign(EXPCTD.get_mut(29), b"1582 May 20 14:00:00.00 Sun");
    fstr::assign(EXPCTD.get_mut(30), b"1582 Jun 25 15:00:00.00 Mon");
    fstr::assign(EXPCTD.get_mut(31), b"1582 Jul 01 16:00:00.00 Sun");
    fstr::assign(EXPCTD.get_mut(32), b"1582 Aug 10 17:00:00.00 Fri");
    fstr::assign(EXPCTD.get_mut(33), b"1582 Sep 15 18:00:00.00 Sat");
    fstr::assign(EXPCTD.get_mut(34), b"1582 Oct 12 19:00:00.00 Fri");
    fstr::assign(EXPCTD.get_mut(35), b"1582 Oct 25 20:00:00.00 Thu");
    fstr::assign(EXPCTD.get_mut(36), b"1582 Dec 31 21:00:00.00 Mon");
    fstr::assign(EXPCTD.get_mut(37), b"1986 Jan 01 10:00:00.00 Tue");
    fstr::assign(EXPCTD.get_mut(38), b"1986 Feb 05 11:00:00.00 Tue");
    fstr::assign(EXPCTD.get_mut(39), b"1986 Mar 10 12:00:00.00 Sun");
    fstr::assign(EXPCTD.get_mut(40), b"1986 Apr 15 13:00:00.00 Mon");
    fstr::assign(EXPCTD.get_mut(41), b"1986 May 20 14:00:00.00 Mon");
    fstr::assign(EXPCTD.get_mut(42), b"1986 Jun 25 15:00:00.00 Tue");
    fstr::assign(EXPCTD.get_mut(43), b"1986 Jul 01 16:00:00.00 Mon");
    fstr::assign(EXPCTD.get_mut(44), b"1986 Aug 10 17:00:00.00 Sat");
    fstr::assign(EXPCTD.get_mut(45), b"1986 Sep 15 18:00:00.00 Sun");
    fstr::assign(EXPCTD.get_mut(46), b"1986 Oct 20 19:00:00.00 Sun");
    fstr::assign(EXPCTD.get_mut(47), b"1986 Nov 25 20:00:00.00 Mon");
    fstr::assign(EXPCTD.get_mut(48), b"1986 Dec 31 21:00:00.00 Tue");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY Mon DD HR:MN:SC.### Wkd ::TDB");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"MIXED".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"TDB".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 Jan 01 10:00:00.000 Fri");
    fstr::assign(EXPCTD.get_mut(2), b" -11 Feb 05 11:00:00.000 Fri");
    fstr::assign(EXPCTD.get_mut(3), b" -11 Mar 10 12:00:00.000 Wed");
    fstr::assign(EXPCTD.get_mut(4), b" -11 Apr 15 13:00:00.000 Thu");
    fstr::assign(EXPCTD.get_mut(5), b" -11 May 20 14:00:00.000 Thu");
    fstr::assign(EXPCTD.get_mut(6), b" -11 Jun 25 15:00:00.000 Fri");
    fstr::assign(EXPCTD.get_mut(7), b" -11 Jul 01 16:00:00.000 Thu");
    fstr::assign(EXPCTD.get_mut(8), b" -11 Aug 10 17:00:00.000 Tue");
    fstr::assign(EXPCTD.get_mut(9), b" -11 Sep 15 18:00:00.000 Wed");
    fstr::assign(EXPCTD.get_mut(10), b" -11 Oct 20 19:00:00.000 Wed");
    fstr::assign(EXPCTD.get_mut(11), b" -11 Nov 25 20:00:00.000 Thu");
    fstr::assign(EXPCTD.get_mut(12), b" -11 Dec 31 21:00:00.000 Fri");
    fstr::assign(EXPCTD.get_mut(13), b" 141 Jan 01 10:00:00.000 Sat");
    fstr::assign(EXPCTD.get_mut(14), b" 141 Feb 05 11:00:00.000 Sat");
    fstr::assign(EXPCTD.get_mut(15), b" 141 Mar 10 12:00:00.000 Thu");
    fstr::assign(EXPCTD.get_mut(16), b" 141 Apr 15 13:00:00.000 Fri");
    fstr::assign(EXPCTD.get_mut(17), b" 141 May 20 14:00:00.000 Fri");
    fstr::assign(EXPCTD.get_mut(18), b" 141 Jun 25 15:00:00.000 Sat");
    fstr::assign(EXPCTD.get_mut(19), b" 141 Jul 01 16:00:00.000 Fri");
    fstr::assign(EXPCTD.get_mut(20), b" 141 Aug 10 17:00:00.000 Wed");
    fstr::assign(EXPCTD.get_mut(21), b" 141 Sep 15 18:00:00.000 Thu");
    fstr::assign(EXPCTD.get_mut(22), b" 141 Oct 20 19:00:00.000 Thu");
    fstr::assign(EXPCTD.get_mut(23), b" 141 Nov 25 20:00:00.000 Fri");
    fstr::assign(EXPCTD.get_mut(24), b" 141 Dec 31 21:00:00.000 Sat");
    fstr::assign(EXPCTD.get_mut(25), b"1582 Jan 01 10:00:00.000 Mon");
    fstr::assign(EXPCTD.get_mut(26), b"1582 Feb 05 11:00:00.000 Mon");
    fstr::assign(EXPCTD.get_mut(27), b"1582 Mar 10 12:00:00.000 Sat");
    fstr::assign(EXPCTD.get_mut(28), b"1582 Apr 15 13:00:00.000 Sun");
    fstr::assign(EXPCTD.get_mut(29), b"1582 May 20 14:00:00.000 Sun");
    fstr::assign(EXPCTD.get_mut(30), b"1582 Jun 25 15:00:00.000 Mon");
    fstr::assign(EXPCTD.get_mut(31), b"1582 Jul 01 16:00:00.000 Sun");
    fstr::assign(EXPCTD.get_mut(32), b"1582 Aug 10 17:00:00.000 Fri");
    fstr::assign(EXPCTD.get_mut(33), b"1582 Sep 15 18:00:00.000 Sat");
    fstr::assign(EXPCTD.get_mut(34), b"1582 Oct 02 19:00:00.000 Tue");
    fstr::assign(EXPCTD.get_mut(35), b"1582 Oct 25 20:00:00.000 Mon");
    fstr::assign(EXPCTD.get_mut(36), b"1582 Dec 31 21:00:00.000 Fri");
    fstr::assign(EXPCTD.get_mut(37), b"1986 Jan 01 10:00:00.000 Wed");
    fstr::assign(EXPCTD.get_mut(38), b"1986 Feb 05 11:00:00.000 Wed");
    fstr::assign(EXPCTD.get_mut(39), b"1986 Mar 10 12:00:00.000 Mon");
    fstr::assign(EXPCTD.get_mut(40), b"1986 Apr 15 13:00:00.000 Tue");
    fstr::assign(EXPCTD.get_mut(41), b"1986 May 20 14:00:00.000 Tue");
    fstr::assign(EXPCTD.get_mut(42), b"1986 Jun 25 15:00:00.000 Wed");
    fstr::assign(EXPCTD.get_mut(43), b"1986 Jul 01 16:00:00.000 Tue");
    fstr::assign(EXPCTD.get_mut(44), b"1986 Aug 10 17:00:00.000 Sun");
    fstr::assign(EXPCTD.get_mut(45), b"1986 Sep 15 18:00:00.000 Mon");
    fstr::assign(EXPCTD.get_mut(46), b"1986 Oct 20 19:00:00.000 Mon");
    fstr::assign(EXPCTD.get_mut(47), b"1986 Nov 25 20:00:00.000 Tue");
    fstr::assign(EXPCTD.get_mut(48), b"1986 Dec 31 21:00:00.000 Wed");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY mon DD wkd HR:MN:SC.### AMPM");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"TDB".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 jan 01 sun 10:00:00.000 A.M.");
    fstr::assign(EXPCTD.get_mut(2), b" -11 feb 05 sun 11:00:00.000 A.M.");
    fstr::assign(EXPCTD.get_mut(3), b" -11 mar 10 fri 12:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(4), b" -11 apr 15 sat 13:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(5), b" -11 may 20 sat 14:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(6), b" -11 jun 25 sun 15:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(7), b" -11 jul 01 sat 16:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(8), b" -11 aug 10 thu 17:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(9), b" -11 sep 15 fri 18:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(10), b" -11 oct 20 fri 19:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(11), b" -11 nov 25 sat 20:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(12), b" -11 dec 31 sun 21:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(13), b" 141 jan 01 sun 10:00:00.000 A.M.");
    fstr::assign(EXPCTD.get_mut(14), b" 141 feb 05 sun 11:00:00.000 A.M.");
    fstr::assign(EXPCTD.get_mut(15), b" 141 mar 10 fri 12:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(16), b" 141 apr 15 sat 13:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(17), b" 141 may 20 sat 14:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(18), b" 141 jun 25 sun 15:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(19), b" 141 jul 01 sat 16:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(20), b" 141 aug 10 thu 17:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(21), b" 141 sep 15 fri 18:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(22), b" 141 oct 20 fri 19:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(23), b" 141 nov 25 sat 20:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(24), b" 141 dec 31 sun 21:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(25), b"1582 jan 01 fri 10:00:00.000 A.M.");
    fstr::assign(EXPCTD.get_mut(26), b"1582 feb 05 fri 11:00:00.000 A.M.");
    fstr::assign(EXPCTD.get_mut(27), b"1582 mar 10 wed 12:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(28), b"1582 apr 15 thu 13:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(29), b"1582 may 20 thu 14:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(30), b"1582 jun 25 fri 15:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(31), b"1582 jul 01 thu 16:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(32), b"1582 aug 10 tue 17:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(33), b"1582 sep 15 wed 18:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(34), b"1582 oct 12 tue 19:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(35), b"1582 oct 25 mon 20:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(36), b"1582 dec 31 fri 21:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(37), b"1986 jan 01 wed 10:00:00.000 A.M.");
    fstr::assign(EXPCTD.get_mut(38), b"1986 feb 05 wed 11:00:00.000 A.M.");
    fstr::assign(EXPCTD.get_mut(39), b"1986 mar 10 mon 12:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(40), b"1986 apr 15 tue 13:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(41), b"1986 may 20 tue 14:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(42), b"1986 jun 25 wed 15:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(43), b"1986 jul 01 tue 16:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(44), b"1986 aug 10 sun 17:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(45), b"1986 sep 15 mon 18:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(46), b"1986 oct 20 mon 19:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(47), b"1986 nov 25 tue 20:00:00.000 P.M.");
    fstr::assign(EXPCTD.get_mut(48), b"1986 dec 31 wed 21:00:00.000 P.M.");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY Mon DD WKD");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 Jan 01 SUN");
    fstr::assign(EXPCTD.get_mut(2), b" -11 Feb 05 SUN");
    fstr::assign(EXPCTD.get_mut(3), b" -11 Mar 10 FRI");
    fstr::assign(EXPCTD.get_mut(4), b" -11 Apr 15 SAT");
    fstr::assign(EXPCTD.get_mut(5), b" -11 May 20 SAT");
    fstr::assign(EXPCTD.get_mut(6), b" -11 Jun 25 SUN");
    fstr::assign(EXPCTD.get_mut(7), b" -11 Jul 01 SAT");
    fstr::assign(EXPCTD.get_mut(8), b" -11 Aug 10 THU");
    fstr::assign(EXPCTD.get_mut(9), b" -11 Sep 15 FRI");
    fstr::assign(EXPCTD.get_mut(10), b" -11 Oct 20 FRI");
    fstr::assign(EXPCTD.get_mut(11), b" -11 Nov 25 SAT");
    fstr::assign(EXPCTD.get_mut(12), b" -11 Dec 31 SUN");
    fstr::assign(EXPCTD.get_mut(13), b" 141 Jan 01 SUN");
    fstr::assign(EXPCTD.get_mut(14), b" 141 Feb 05 SUN");
    fstr::assign(EXPCTD.get_mut(15), b" 141 Mar 10 FRI");
    fstr::assign(EXPCTD.get_mut(16), b" 141 Apr 15 SAT");
    fstr::assign(EXPCTD.get_mut(17), b" 141 May 20 SAT");
    fstr::assign(EXPCTD.get_mut(18), b" 141 Jun 25 SUN");
    fstr::assign(EXPCTD.get_mut(19), b" 141 Jul 01 SAT");
    fstr::assign(EXPCTD.get_mut(20), b" 141 Aug 10 THU");
    fstr::assign(EXPCTD.get_mut(21), b" 141 Sep 15 FRI");
    fstr::assign(EXPCTD.get_mut(22), b" 141 Oct 20 FRI");
    fstr::assign(EXPCTD.get_mut(23), b" 141 Nov 25 SAT");
    fstr::assign(EXPCTD.get_mut(24), b" 141 Dec 31 SUN");
    fstr::assign(EXPCTD.get_mut(25), b"1582 Jan 01 FRI");
    fstr::assign(EXPCTD.get_mut(26), b"1582 Feb 05 FRI");
    fstr::assign(EXPCTD.get_mut(27), b"1582 Mar 10 WED");
    fstr::assign(EXPCTD.get_mut(28), b"1582 Apr 15 THU");
    fstr::assign(EXPCTD.get_mut(29), b"1582 May 20 THU");
    fstr::assign(EXPCTD.get_mut(30), b"1582 Jun 25 FRI");
    fstr::assign(EXPCTD.get_mut(31), b"1582 Jul 01 THU");
    fstr::assign(EXPCTD.get_mut(32), b"1582 Aug 10 TUE");
    fstr::assign(EXPCTD.get_mut(33), b"1582 Sep 15 WED");
    fstr::assign(EXPCTD.get_mut(34), b"1582 Oct 12 TUE");
    fstr::assign(EXPCTD.get_mut(35), b"1582 Oct 25 MON");
    fstr::assign(EXPCTD.get_mut(36), b"1582 Dec 31 FRI");
    fstr::assign(EXPCTD.get_mut(37), b"1986 Jan 01 WED");
    fstr::assign(EXPCTD.get_mut(38), b"1986 Feb 05 WED");
    fstr::assign(EXPCTD.get_mut(39), b"1986 Mar 10 MON");
    fstr::assign(EXPCTD.get_mut(40), b"1986 Apr 15 TUE");
    fstr::assign(EXPCTD.get_mut(41), b"1986 May 20 TUE");
    fstr::assign(EXPCTD.get_mut(42), b"1986 Jun 25 WED");
    fstr::assign(EXPCTD.get_mut(43), b"1986 Jul 01 TUE");
    fstr::assign(EXPCTD.get_mut(44), b"1986 Aug 10 SUN");
    fstr::assign(EXPCTD.get_mut(45), b"1986 Sep 15 MON");
    fstr::assign(EXPCTD.get_mut(46), b"1986 Oct 20 MON");
    fstr::assign(EXPCTD.get_mut(47), b"1986 Nov 25 TUE");
    fstr::assign(EXPCTD.get_mut(48), b"1986 Dec 31 WED");

    for J in 1..=2 {
        if (J == 1) {
            spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"TDT".clone(), ctx)?;
        } else {
            spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"TT".clone(), ctx)?;
        }

        for I in 1..=48 {
            spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
            spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;

            fstr::assign(&mut NAME, b"FTIME(#)");
            spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
        }
    }

    fstr::assign(&mut PICTUR, b"YYYY month weekday DD HR:MN.###");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"ZONE", &mut b"UTC+8".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 january sunday 01 10:00.000");
    fstr::assign(EXPCTD.get_mut(2), b" -11 february sunday 05 11:00.000");
    fstr::assign(EXPCTD.get_mut(3), b" -11 march friday 10 12:00.000");
    fstr::assign(EXPCTD.get_mut(4), b" -11 april saturday 15 13:00.000");
    fstr::assign(EXPCTD.get_mut(5), b" -11 may saturday 20 14:00.000");
    fstr::assign(EXPCTD.get_mut(6), b" -11 june sunday 25 15:00.000");
    fstr::assign(EXPCTD.get_mut(7), b" -11 july saturday 01 16:00.000");
    fstr::assign(EXPCTD.get_mut(8), b" -11 august thursday 10 17:00.000");
    fstr::assign(EXPCTD.get_mut(9), b" -11 september friday 15 18:00.000");
    fstr::assign(EXPCTD.get_mut(10), b" -11 october friday 20 19:00.000");
    fstr::assign(EXPCTD.get_mut(11), b" -11 november saturday 25 20:00.000");
    fstr::assign(EXPCTD.get_mut(12), b" -11 december sunday 31 21:00.000");
    fstr::assign(EXPCTD.get_mut(13), b" 141 january sunday 01 10:00.000");
    fstr::assign(EXPCTD.get_mut(14), b" 141 february sunday 05 11:00.000");
    fstr::assign(EXPCTD.get_mut(15), b" 141 march friday 10 12:00.000");
    fstr::assign(EXPCTD.get_mut(16), b" 141 april saturday 15 13:00.000");
    fstr::assign(EXPCTD.get_mut(17), b" 141 may saturday 20 14:00.000");
    fstr::assign(EXPCTD.get_mut(18), b" 141 june sunday 25 15:00.000");
    fstr::assign(EXPCTD.get_mut(19), b" 141 july saturday 01 16:00.000");
    fstr::assign(EXPCTD.get_mut(20), b" 141 august thursday 10 17:00.000");
    fstr::assign(EXPCTD.get_mut(21), b" 141 september friday 15 18:00.000");
    fstr::assign(EXPCTD.get_mut(22), b" 141 october friday 20 19:00.000");
    fstr::assign(EXPCTD.get_mut(23), b" 141 november saturday 25 20:00.000");
    fstr::assign(EXPCTD.get_mut(24), b" 141 december sunday 31 21:00.000");
    fstr::assign(EXPCTD.get_mut(25), b"1582 january friday 01 10:00.000");
    fstr::assign(EXPCTD.get_mut(26), b"1582 february friday 05 11:00.000");
    fstr::assign(EXPCTD.get_mut(27), b"1582 march wednesday 10 12:00.000");
    fstr::assign(EXPCTD.get_mut(28), b"1582 april thursday 15 13:00.000");
    fstr::assign(EXPCTD.get_mut(29), b"1582 may thursday 20 14:00.000");
    fstr::assign(EXPCTD.get_mut(30), b"1582 june friday 25 15:00.000");
    fstr::assign(EXPCTD.get_mut(31), b"1582 july thursday 01 16:00.000");
    fstr::assign(EXPCTD.get_mut(32), b"1582 august tuesday 10 17:00.000");
    fstr::assign(EXPCTD.get_mut(33), b"1582 september wednesday 15 18:00.000");
    fstr::assign(EXPCTD.get_mut(34), b"1582 october tuesday 12 19:00.000");
    fstr::assign(EXPCTD.get_mut(35), b"1582 october monday 25 20:00.000");
    fstr::assign(EXPCTD.get_mut(36), b"1582 december friday 31 21:00.000");
    fstr::assign(EXPCTD.get_mut(37), b"1986 january wednesday 01 10:00.000");
    fstr::assign(EXPCTD.get_mut(38), b"1986 february wednesday 05 11:00.000");
    fstr::assign(EXPCTD.get_mut(39), b"1986 march monday 10 12:00.000");
    fstr::assign(EXPCTD.get_mut(40), b"1986 april tuesday 15 13:00.000");
    fstr::assign(EXPCTD.get_mut(41), b"1986 may tuesday 20 14:00.000");
    fstr::assign(EXPCTD.get_mut(42), b"1986 june wednesday 25 15:00.000");
    fstr::assign(EXPCTD.get_mut(43), b"1986 july tuesday 01 16:00.000");
    fstr::assign(EXPCTD.get_mut(44), b"1986 august sunday 10 17:00.000");
    fstr::assign(EXPCTD.get_mut(45), b"1986 september monday 15 18:00.000");
    fstr::assign(EXPCTD.get_mut(46), b"1986 october monday 20 19:00.000");
    fstr::assign(EXPCTD.get_mut(47), b"1986 november tuesday 25 20:00.000");
    fstr::assign(EXPCTD.get_mut(48), b"1986 december wednesday 31 21:00.000");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY MONTH DD::UTC+07:30-HR:MN ::MCAL");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 JANUARY 03-17:30");
    fstr::assign(EXPCTD.get_mut(2), b" -11 FEBRUARY 07-18:30");
    fstr::assign(EXPCTD.get_mut(3), b" -11 MARCH 12-19:30");
    fstr::assign(EXPCTD.get_mut(4), b" -11 APRIL 17-20:30");
    fstr::assign(EXPCTD.get_mut(5), b" -11 MAY 22-21:30");
    fstr::assign(EXPCTD.get_mut(6), b" -11 JUNE 27-22:30");
    fstr::assign(EXPCTD.get_mut(7), b" -11 JULY 03-23:30");
    fstr::assign(EXPCTD.get_mut(8), b" -11 AUGUST 13-00:30");
    fstr::assign(EXPCTD.get_mut(9), b" -11 SEPTEMBER 18-01:30");
    fstr::assign(EXPCTD.get_mut(10), b" -11 OCTOBER 23-02:30");
    fstr::assign(EXPCTD.get_mut(11), b" -11 NOVEMBER 28-03:30");
    fstr::assign(EXPCTD.get_mut(12), b" -10 JANUARY 03-04:30");
    fstr::assign(EXPCTD.get_mut(13), b" 141 JANUARY 02-17:30");
    fstr::assign(EXPCTD.get_mut(14), b" 141 FEBRUARY 06-18:30");
    fstr::assign(EXPCTD.get_mut(15), b" 141 MARCH 11-19:30");
    fstr::assign(EXPCTD.get_mut(16), b" 141 APRIL 16-20:30");
    fstr::assign(EXPCTD.get_mut(17), b" 141 MAY 21-21:30");
    fstr::assign(EXPCTD.get_mut(18), b" 141 JUNE 26-22:30");
    fstr::assign(EXPCTD.get_mut(19), b" 141 JULY 02-23:30");
    fstr::assign(EXPCTD.get_mut(20), b" 141 AUGUST 12-00:30");
    fstr::assign(EXPCTD.get_mut(21), b" 141 SEPTEMBER 17-01:30");
    fstr::assign(EXPCTD.get_mut(22), b" 141 OCTOBER 22-02:30");
    fstr::assign(EXPCTD.get_mut(23), b" 141 NOVEMBER 27-03:30");
    fstr::assign(EXPCTD.get_mut(24), b" 142 JANUARY 02-04:30");
    fstr::assign(EXPCTD.get_mut(25), b"1581 DECEMBER 22-17:30");
    fstr::assign(EXPCTD.get_mut(26), b"1582 JANUARY 26-18:30");
    fstr::assign(EXPCTD.get_mut(27), b"1582 FEBRUARY 28-19:30");
    fstr::assign(EXPCTD.get_mut(28), b"1582 APRIL 05-20:30");
    fstr::assign(EXPCTD.get_mut(29), b"1582 MAY 10-21:30");
    fstr::assign(EXPCTD.get_mut(30), b"1582 JUNE 15-22:30");
    fstr::assign(EXPCTD.get_mut(31), b"1582 JUNE 21-23:30");
    fstr::assign(EXPCTD.get_mut(32), b"1582 AUGUST 01-00:30");
    fstr::assign(EXPCTD.get_mut(33), b"1582 SEPTEMBER 06-01:30");
    fstr::assign(EXPCTD.get_mut(34), b"1582 OCTOBER 03-02:30");
    fstr::assign(EXPCTD.get_mut(35), b"1582 OCTOBER 26-03:30");
    fstr::assign(EXPCTD.get_mut(36), b"1583 JANUARY 01-04:30");
    fstr::assign(EXPCTD.get_mut(37), b"1986 JANUARY 01-17:30");
    fstr::assign(EXPCTD.get_mut(38), b"1986 FEBRUARY 05-18:30");
    fstr::assign(EXPCTD.get_mut(39), b"1986 MARCH 10-19:30");
    fstr::assign(EXPCTD.get_mut(40), b"1986 APRIL 15-20:30");
    fstr::assign(EXPCTD.get_mut(41), b"1986 MAY 20-21:30");
    fstr::assign(EXPCTD.get_mut(42), b"1986 JUNE 25-22:30");
    fstr::assign(EXPCTD.get_mut(43), b"1986 JULY 01-23:30");
    fstr::assign(EXPCTD.get_mut(44), b"1986 AUGUST 11-00:30");
    fstr::assign(EXPCTD.get_mut(45), b"1986 SEPTEMBER 16-01:30");
    fstr::assign(EXPCTD.get_mut(46), b"1986 OCTOBER 21-02:30");
    fstr::assign(EXPCTD.get_mut(47), b"1986 NOVEMBER 26-03:30");
    fstr::assign(EXPCTD.get_mut(48), b"1987 JANUARY 01-04:30");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;

        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY MONTH DD HR.### ::UTC+07:40::MCAL::RND");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 JANUARY 03 17.667");
    fstr::assign(EXPCTD.get_mut(2), b" -11 FEBRUARY 07 18.667");
    fstr::assign(EXPCTD.get_mut(3), b" -11 MARCH 12 19.667");
    fstr::assign(EXPCTD.get_mut(4), b" -11 APRIL 17 20.667");
    fstr::assign(EXPCTD.get_mut(5), b" -11 MAY 22 21.667");
    fstr::assign(EXPCTD.get_mut(6), b" -11 JUNE 27 22.667");
    fstr::assign(EXPCTD.get_mut(7), b" -11 JULY 03 23.667");
    fstr::assign(EXPCTD.get_mut(8), b" -11 AUGUST 13 00.667");
    fstr::assign(EXPCTD.get_mut(9), b" -11 SEPTEMBER 18 01.667");
    fstr::assign(EXPCTD.get_mut(10), b" -11 OCTOBER 23 02.667");
    fstr::assign(EXPCTD.get_mut(11), b" -11 NOVEMBER 28 03.667");
    fstr::assign(EXPCTD.get_mut(12), b" -10 JANUARY 03 04.667");
    fstr::assign(EXPCTD.get_mut(13), b" 141 JANUARY 02 17.667");
    fstr::assign(EXPCTD.get_mut(14), b" 141 FEBRUARY 06 18.667");
    fstr::assign(EXPCTD.get_mut(15), b" 141 MARCH 11 19.667");
    fstr::assign(EXPCTD.get_mut(16), b" 141 APRIL 16 20.667");
    fstr::assign(EXPCTD.get_mut(17), b" 141 MAY 21 21.667");
    fstr::assign(EXPCTD.get_mut(18), b" 141 JUNE 26 22.667");
    fstr::assign(EXPCTD.get_mut(19), b" 141 JULY 02 23.667");
    fstr::assign(EXPCTD.get_mut(20), b" 141 AUGUST 12 00.667");
    fstr::assign(EXPCTD.get_mut(21), b" 141 SEPTEMBER 17 01.667");
    fstr::assign(EXPCTD.get_mut(22), b" 141 OCTOBER 22 02.667");
    fstr::assign(EXPCTD.get_mut(23), b" 141 NOVEMBER 27 03.667");
    fstr::assign(EXPCTD.get_mut(24), b" 142 JANUARY 02 04.667");
    fstr::assign(EXPCTD.get_mut(25), b"1581 DECEMBER 22 17.667");
    fstr::assign(EXPCTD.get_mut(26), b"1582 JANUARY 26 18.667");
    fstr::assign(EXPCTD.get_mut(27), b"1582 FEBRUARY 28 19.667");
    fstr::assign(EXPCTD.get_mut(28), b"1582 APRIL 05 20.667");
    fstr::assign(EXPCTD.get_mut(29), b"1582 MAY 10 21.667");
    fstr::assign(EXPCTD.get_mut(30), b"1582 JUNE 15 22.667");
    fstr::assign(EXPCTD.get_mut(31), b"1582 JUNE 21 23.667");
    fstr::assign(EXPCTD.get_mut(32), b"1582 AUGUST 01 00.667");
    fstr::assign(EXPCTD.get_mut(33), b"1582 SEPTEMBER 06 01.667");
    fstr::assign(EXPCTD.get_mut(34), b"1582 OCTOBER 03 02.667");
    fstr::assign(EXPCTD.get_mut(35), b"1582 OCTOBER 26 03.667");
    fstr::assign(EXPCTD.get_mut(36), b"1583 JANUARY 01 04.667");
    fstr::assign(EXPCTD.get_mut(37), b"1986 JANUARY 01 17.667");
    fstr::assign(EXPCTD.get_mut(38), b"1986 FEBRUARY 05 18.667");
    fstr::assign(EXPCTD.get_mut(39), b"1986 MARCH 10 19.667");
    fstr::assign(EXPCTD.get_mut(40), b"1986 APRIL 15 20.667");
    fstr::assign(EXPCTD.get_mut(41), b"1986 MAY 20 21.667");
    fstr::assign(EXPCTD.get_mut(42), b"1986 JUNE 25 22.667");
    fstr::assign(EXPCTD.get_mut(43), b"1986 JULY 01 23.667");
    fstr::assign(EXPCTD.get_mut(44), b"1986 AUGUST 11 00.667");
    fstr::assign(EXPCTD.get_mut(45), b"1986 SEPTEMBER 16 01.667");
    fstr::assign(EXPCTD.get_mut(46), b"1986 OCTOBER 21 02.667");
    fstr::assign(EXPCTD.get_mut(47), b"1986 NOVEMBER 26 03.667");
    fstr::assign(EXPCTD.get_mut(48), b"1987 JANUARY 01 04.667");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;

        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(
        &mut PICTUR,
        b"YYYY MONTH DD ::UTC-07:20HR.### .# SC. ::JULIAN",
    );

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 JANUARY 01 02.666 .# 00. ::JULIAN");
    fstr::assign(
        EXPCTD.get_mut(2),
        b" -11 FEBRUARY 05 03.666 .# 00. ::JULIAN",
    );
    fstr::assign(EXPCTD.get_mut(3), b" -11 MARCH 10 04.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(4), b" -11 APRIL 15 05.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(5), b" -11 MAY 20 06.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(6), b" -11 JUNE 25 07.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(7), b" -11 JULY 01 08.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(8), b" -11 AUGUST 10 09.666 .# 00. ::JULIAN");
    fstr::assign(
        EXPCTD.get_mut(9),
        b" -11 SEPTEMBER 15 10.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(10),
        b" -11 OCTOBER 20 11.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(11),
        b" -11 NOVEMBER 25 12.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(12),
        b" -11 DECEMBER 31 13.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(13),
        b" 141 JANUARY 01 02.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(14),
        b" 141 FEBRUARY 05 03.666 .# 00. ::JULIAN",
    );
    fstr::assign(EXPCTD.get_mut(15), b" 141 MARCH 10 04.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(16), b" 141 APRIL 15 05.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(17), b" 141 MAY 20 06.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(18), b" 141 JUNE 25 07.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(19), b" 141 JULY 01 08.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(20), b" 141 AUGUST 10 09.666 .# 00. ::JULIAN");
    fstr::assign(
        EXPCTD.get_mut(21),
        b" 141 SEPTEMBER 15 10.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(22),
        b" 141 OCTOBER 20 11.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(23),
        b" 141 NOVEMBER 25 12.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(24),
        b" 141 DECEMBER 31 13.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(25),
        b"1582 JANUARY 01 02.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(26),
        b"1582 FEBRUARY 05 03.666 .# 00. ::JULIAN",
    );
    fstr::assign(EXPCTD.get_mut(27), b"1582 MARCH 10 04.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(28), b"1582 APRIL 15 05.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(29), b"1582 MAY 20 06.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(30), b"1582 JUNE 25 07.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(31), b"1582 JULY 01 08.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(32), b"1582 AUGUST 10 09.666 .# 00. ::JULIAN");
    fstr::assign(
        EXPCTD.get_mut(33),
        b"1582 SEPTEMBER 15 10.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(34),
        b"1582 OCTOBER 12 11.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(35),
        b"1582 OCTOBER 25 12.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(36),
        b"1582 DECEMBER 31 13.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(37),
        b"1986 JANUARY 01 02.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(38),
        b"1986 FEBRUARY 05 03.666 .# 00. ::JULIAN",
    );
    fstr::assign(EXPCTD.get_mut(39), b"1986 MARCH 10 04.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(40), b"1986 APRIL 15 05.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(41), b"1986 MAY 20 06.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(42), b"1986 JUNE 25 07.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(43), b"1986 JULY 01 08.666 .# 00. ::JULIAN");
    fstr::assign(EXPCTD.get_mut(44), b"1986 AUGUST 10 09.666 .# 00. ::JULIAN");
    fstr::assign(
        EXPCTD.get_mut(45),
        b"1986 SEPTEMBER 15 10.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(46),
        b"1986 OCTOBER 20 11.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(47),
        b"1986 NOVEMBER 25 12.666 .# 00. ::JULIAN",
    );
    fstr::assign(
        EXPCTD.get_mut(48),
        b"1986 DECEMBER 31 13.666 .# 00. ::JULIAN",
    );

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;

        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY MONTH DD HR:MN.SC.### #");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 JANUARY 01 10:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(2), b" -11 FEBRUARY 05 11:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(3), b" -11 MARCH 10 12:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(4), b" -11 APRIL 15 13:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(5), b" -11 MAY 20 14:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(6), b" -11 JUNE 25 15:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(7), b" -11 JULY 01 16:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(8), b" -11 AUGUST 10 17:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(9), b" -11 SEPTEMBER 15 18:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(10), b" -11 OCTOBER 20 19:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(11), b" -11 NOVEMBER 25 20:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(12), b" -11 DECEMBER 31 21:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(13), b" 141 JANUARY 01 10:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(14), b" 141 FEBRUARY 05 11:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(15), b" 141 MARCH 10 12:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(16), b" 141 APRIL 15 13:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(17), b" 141 MAY 20 14:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(18), b" 141 JUNE 25 15:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(19), b" 141 JULY 01 16:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(20), b" 141 AUGUST 10 17:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(21), b" 141 SEPTEMBER 15 18:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(22), b" 141 OCTOBER 20 19:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(23), b" 141 NOVEMBER 25 20:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(24), b" 141 DECEMBER 31 21:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(25), b"1582 JANUARY 01 10:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(26), b"1582 FEBRUARY 05 11:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(27), b"1582 MARCH 10 12:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(28), b"1582 APRIL 15 13:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(29), b"1582 MAY 20 14:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(30), b"1582 JUNE 25 15:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(31), b"1582 JULY 01 16:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(32), b"1582 AUGUST 10 17:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(33), b"1582 SEPTEMBER 15 18:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(34), b"1582 OCTOBER 12 19:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(35), b"1582 OCTOBER 25 20:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(36), b"1582 DECEMBER 31 21:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(37), b"1986 JANUARY 01 10:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(38), b"1986 FEBRUARY 05 11:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(39), b"1986 MARCH 10 12:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(40), b"1986 APRIL 15 13:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(41), b"1986 MAY 20 14:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(42), b"1986 JUNE 25 15:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(43), b"1986 JULY 01 16:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(44), b"1986 AUGUST 10 17:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(45), b"1986 SEPTEMBER 15 18:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(46), b"1986 OCTOBER 20 19:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(47), b"1986 NOVEMBER 25 20:00.00.000 #");
    fstr::assign(EXPCTD.get_mut(48), b"1986 DECEMBER 31 21:00.00.000 #");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"SP2000.####  ::TDB");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b"-6.346097996E+10");
    fstr::assign(EXPCTD.get_mut(2), b"-6.345795236E+10");
    fstr::assign(EXPCTD.get_mut(3), b"-6.345509756E+10");
    fstr::assign(EXPCTD.get_mut(4), b"-6.345198356E+10");
    fstr::assign(EXPCTD.get_mut(5), b"-6.344895596E+10");
    fstr::assign(EXPCTD.get_mut(6), b"-6.344584196E+10");
    fstr::assign(EXPCTD.get_mut(7), b"-6.344531996E+10");
    fstr::assign(EXPCTD.get_mut(8), b"-6.344186036E+10");
    fstr::assign(EXPCTD.get_mut(9), b"-6.343874636E+10");
    fstr::assign(EXPCTD.get_mut(10), b"-6.343571876E+10");
    fstr::assign(EXPCTD.get_mut(11), b"-6.343260476E+10");
    fstr::assign(EXPCTD.get_mut(12), b"-6.342949076E+10");
    fstr::assign(EXPCTD.get_mut(13), b"-5.866431116E+10");
    fstr::assign(EXPCTD.get_mut(14), b"-5.866128356E+10");
    fstr::assign(EXPCTD.get_mut(15), b"-5.865842876E+10");
    fstr::assign(EXPCTD.get_mut(16), b"-5.865531476E+10");
    fstr::assign(EXPCTD.get_mut(17), b"-5.865228716E+10");
    fstr::assign(EXPCTD.get_mut(18), b"-5.864917316E+10");
    fstr::assign(EXPCTD.get_mut(19), b"-5.864865116E+10");
    fstr::assign(EXPCTD.get_mut(20), b"-5.864519156E+10");
    fstr::assign(EXPCTD.get_mut(21), b"-5.864207756E+10");
    fstr::assign(EXPCTD.get_mut(22), b"-5.863904996E+10");
    fstr::assign(EXPCTD.get_mut(23), b"-5.863593596E+10");
    fstr::assign(EXPCTD.get_mut(24), b"-5.863282196E+10");
    fstr::assign(EXPCTD.get_mut(25), b"-1.319078156E+10");
    fstr::assign(EXPCTD.get_mut(26), b"-1.318775396E+10");
    fstr::assign(EXPCTD.get_mut(27), b"-1.318489916E+10");
    fstr::assign(EXPCTD.get_mut(28), b"-1.318178516E+10");
    fstr::assign(EXPCTD.get_mut(29), b"-1.317875756E+10");
    fstr::assign(EXPCTD.get_mut(30), b"-1.317564356E+10");
    fstr::assign(EXPCTD.get_mut(31), b"-1.317512156E+10");
    fstr::assign(EXPCTD.get_mut(32), b"-1.317166196E+10");
    fstr::assign(EXPCTD.get_mut(33), b"-1.316854796E+10");
    fstr::assign(EXPCTD.get_mut(34), b"-1.316621156E+10");
    fstr::assign(EXPCTD.get_mut(35), b"-1.316508476E+10");
    fstr::assign(EXPCTD.get_mut(36), b"-1.315929236E+10");
    fstr::assign(EXPCTD.get_mut(37), b" -441770344.8161");
    fstr::assign(EXPCTD.get_mut(38), b" -438742744.8151");
    fstr::assign(EXPCTD.get_mut(39), b" -435887944.8145");
    fstr::assign(EXPCTD.get_mut(40), b" -432773944.8144");
    fstr::assign(EXPCTD.get_mut(41), b" -429746344.8149");
    fstr::assign(EXPCTD.get_mut(42), b" -426632344.8158");
    fstr::assign(EXPCTD.get_mut(43), b" -426110344.8160");
    fstr::assign(EXPCTD.get_mut(44), b" -422650744.8170");
    fstr::assign(EXPCTD.get_mut(45), b" -419536744.8176");
    fstr::assign(EXPCTD.get_mut(46), b" -416509144.8176");
    fstr::assign(EXPCTD.get_mut(47), b" -413395144.8171");
    fstr::assign(EXPCTD.get_mut(48), b" -410281144.8161");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"SP1950.####");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b"-6.188310000E+10");
    fstr::assign(EXPCTD.get_mut(2), b"-6.188007240E+10");
    fstr::assign(EXPCTD.get_mut(3), b"-6.187721760E+10");
    fstr::assign(EXPCTD.get_mut(4), b"-6.187410360E+10");
    fstr::assign(EXPCTD.get_mut(5), b"-6.187107600E+10");
    fstr::assign(EXPCTD.get_mut(6), b"-6.186796200E+10");
    fstr::assign(EXPCTD.get_mut(7), b"-6.186744000E+10");
    fstr::assign(EXPCTD.get_mut(8), b"-6.186398040E+10");
    fstr::assign(EXPCTD.get_mut(9), b"-6.186086640E+10");
    fstr::assign(EXPCTD.get_mut(10), b"-6.185783880E+10");
    fstr::assign(EXPCTD.get_mut(11), b"-6.185472480E+10");
    fstr::assign(EXPCTD.get_mut(12), b"-6.185161080E+10");
    fstr::assign(EXPCTD.get_mut(13), b"-5.708643120E+10");
    fstr::assign(EXPCTD.get_mut(14), b"-5.708340360E+10");
    fstr::assign(EXPCTD.get_mut(15), b"-5.708054880E+10");
    fstr::assign(EXPCTD.get_mut(16), b"-5.707743480E+10");
    fstr::assign(EXPCTD.get_mut(17), b"-5.707440720E+10");
    fstr::assign(EXPCTD.get_mut(18), b"-5.707129320E+10");
    fstr::assign(EXPCTD.get_mut(19), b"-5.707077120E+10");
    fstr::assign(EXPCTD.get_mut(20), b"-5.706731160E+10");
    fstr::assign(EXPCTD.get_mut(21), b"-5.706419760E+10");
    fstr::assign(EXPCTD.get_mut(22), b"-5.706117000E+10");
    fstr::assign(EXPCTD.get_mut(23), b"-5.705805600E+10");
    fstr::assign(EXPCTD.get_mut(24), b"-5.705494200E+10");
    fstr::assign(EXPCTD.get_mut(25), b"-1.161290160E+10");
    fstr::assign(EXPCTD.get_mut(26), b"-1.160987400E+10");
    fstr::assign(EXPCTD.get_mut(27), b"-1.160701920E+10");
    fstr::assign(EXPCTD.get_mut(28), b"-1.160390520E+10");
    fstr::assign(EXPCTD.get_mut(29), b"-1.160087760E+10");
    fstr::assign(EXPCTD.get_mut(30), b"-1.159776360E+10");
    fstr::assign(EXPCTD.get_mut(31), b"-1.159724160E+10");
    fstr::assign(EXPCTD.get_mut(32), b"-1.159378200E+10");
    fstr::assign(EXPCTD.get_mut(33), b"-1.159066800E+10");
    fstr::assign(EXPCTD.get_mut(34), b"-1.158833160E+10");
    fstr::assign(EXPCTD.get_mut(35), b"-1.158720480E+10");
    fstr::assign(EXPCTD.get_mut(36), b"-1.158141240E+10");
    fstr::assign(EXPCTD.get_mut(37), b" 1136109600.0000");
    fstr::assign(EXPCTD.get_mut(38), b" 1139137200.0000");
    fstr::assign(EXPCTD.get_mut(39), b" 1141992000.0000");
    fstr::assign(EXPCTD.get_mut(40), b" 1145106000.0000");
    fstr::assign(EXPCTD.get_mut(41), b" 1148133600.0000");
    fstr::assign(EXPCTD.get_mut(42), b" 1151247600.0000");
    fstr::assign(EXPCTD.get_mut(43), b" 1151769600.0000");
    fstr::assign(EXPCTD.get_mut(44), b" 1155229200.0000");
    fstr::assign(EXPCTD.get_mut(45), b" 1158343200.0000");
    fstr::assign(EXPCTD.get_mut(46), b" 1161370800.0000");
    fstr::assign(EXPCTD.get_mut(47), b" 1164484800.0000");
    fstr::assign(EXPCTD.get_mut(48), b" 1167598800.0000");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY MONTH DD HR:MN:SC ::JCAL");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 JANUARY 03 10:00:00");
    fstr::assign(EXPCTD.get_mut(2), b" -11 FEBRUARY 07 11:00:00");
    fstr::assign(EXPCTD.get_mut(3), b" -11 MARCH 12 12:00:00");
    fstr::assign(EXPCTD.get_mut(4), b" -11 APRIL 17 13:00:00");
    fstr::assign(EXPCTD.get_mut(5), b" -11 MAY 22 14:00:00");
    fstr::assign(EXPCTD.get_mut(6), b" -11 JUNE 27 15:00:00");
    fstr::assign(EXPCTD.get_mut(7), b" -11 JULY 03 16:00:00");
    fstr::assign(EXPCTD.get_mut(8), b" -11 AUGUST 12 17:00:00");
    fstr::assign(EXPCTD.get_mut(9), b" -11 SEPTEMBER 17 18:00:00");
    fstr::assign(EXPCTD.get_mut(10), b" -11 OCTOBER 22 19:00:00");
    fstr::assign(EXPCTD.get_mut(11), b" -11 NOVEMBER 27 20:00:00");
    fstr::assign(EXPCTD.get_mut(12), b" -10 JANUARY 02 21:00:00");
    fstr::assign(EXPCTD.get_mut(13), b" 141 JANUARY 02 10:00:00");
    fstr::assign(EXPCTD.get_mut(14), b" 141 FEBRUARY 06 11:00:00");
    fstr::assign(EXPCTD.get_mut(15), b" 141 MARCH 11 12:00:00");
    fstr::assign(EXPCTD.get_mut(16), b" 141 APRIL 16 13:00:00");
    fstr::assign(EXPCTD.get_mut(17), b" 141 MAY 21 14:00:00");
    fstr::assign(EXPCTD.get_mut(18), b" 141 JUNE 26 15:00:00");
    fstr::assign(EXPCTD.get_mut(19), b" 141 JULY 02 16:00:00");
    fstr::assign(EXPCTD.get_mut(20), b" 141 AUGUST 11 17:00:00");
    fstr::assign(EXPCTD.get_mut(21), b" 141 SEPTEMBER 16 18:00:00");
    fstr::assign(EXPCTD.get_mut(22), b" 141 OCTOBER 21 19:00:00");
    fstr::assign(EXPCTD.get_mut(23), b" 141 NOVEMBER 26 20:00:00");
    fstr::assign(EXPCTD.get_mut(24), b" 142 JANUARY 01 21:00:00");
    fstr::assign(EXPCTD.get_mut(25), b"1581 DECEMBER 22 10:00:00");
    fstr::assign(EXPCTD.get_mut(26), b"1582 JANUARY 26 11:00:00");
    fstr::assign(EXPCTD.get_mut(27), b"1582 FEBRUARY 28 12:00:00");
    fstr::assign(EXPCTD.get_mut(28), b"1582 APRIL 05 13:00:00");
    fstr::assign(EXPCTD.get_mut(29), b"1582 MAY 10 14:00:00");
    fstr::assign(EXPCTD.get_mut(30), b"1582 JUNE 15 15:00:00");
    fstr::assign(EXPCTD.get_mut(31), b"1582 JUNE 21 16:00:00");
    fstr::assign(EXPCTD.get_mut(32), b"1582 JULY 31 17:00:00");
    fstr::assign(EXPCTD.get_mut(33), b"1582 SEPTEMBER 05 18:00:00");
    fstr::assign(EXPCTD.get_mut(34), b"1582 OCTOBER 02 19:00:00");
    fstr::assign(EXPCTD.get_mut(35), b"1582 OCTOBER 15 20:00:00");
    fstr::assign(EXPCTD.get_mut(36), b"1582 DECEMBER 21 21:00:00");
    fstr::assign(EXPCTD.get_mut(37), b"1985 DECEMBER 19 10:00:00");
    fstr::assign(EXPCTD.get_mut(38), b"1986 JANUARY 23 11:00:00");
    fstr::assign(EXPCTD.get_mut(39), b"1986 FEBRUARY 25 12:00:00");
    fstr::assign(EXPCTD.get_mut(40), b"1986 APRIL 02 13:00:00");
    fstr::assign(EXPCTD.get_mut(41), b"1986 MAY 07 14:00:00");
    fstr::assign(EXPCTD.get_mut(42), b"1986 JUNE 12 15:00:00");
    fstr::assign(EXPCTD.get_mut(43), b"1986 JUNE 18 16:00:00");
    fstr::assign(EXPCTD.get_mut(44), b"1986 JULY 28 17:00:00");
    fstr::assign(EXPCTD.get_mut(45), b"1986 SEPTEMBER 02 18:00:00");
    fstr::assign(EXPCTD.get_mut(46), b"1986 OCTOBER 07 19:00:00");
    fstr::assign(EXPCTD.get_mut(47), b"1986 NOVEMBER 12 20:00:00");
    fstr::assign(EXPCTD.get_mut(48), b"1986 DECEMBER 18 21:00:00");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"JULIAND.#####");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b"1717042.91666");
    fstr::assign(EXPCTD.get_mut(2), b"1717077.95833");
    fstr::assign(EXPCTD.get_mut(3), b"1717111.00000");
    fstr::assign(EXPCTD.get_mut(4), b"1717147.04166");
    fstr::assign(EXPCTD.get_mut(5), b"1717182.08333");
    fstr::assign(EXPCTD.get_mut(6), b"1717218.12500");
    fstr::assign(EXPCTD.get_mut(7), b"1717224.16666");
    fstr::assign(EXPCTD.get_mut(8), b"1717264.20833");
    fstr::assign(EXPCTD.get_mut(9), b"1717300.25000");
    fstr::assign(EXPCTD.get_mut(10), b"1717335.29166");
    fstr::assign(EXPCTD.get_mut(11), b"1717371.33333");
    fstr::assign(EXPCTD.get_mut(12), b"1717407.37500");
    fstr::assign(EXPCTD.get_mut(13), b"1772559.91666");
    fstr::assign(EXPCTD.get_mut(14), b"1772594.95833");
    fstr::assign(EXPCTD.get_mut(15), b"1772628.00000");
    fstr::assign(EXPCTD.get_mut(16), b"1772664.04166");
    fstr::assign(EXPCTD.get_mut(17), b"1772699.08333");
    fstr::assign(EXPCTD.get_mut(18), b"1772735.12500");
    fstr::assign(EXPCTD.get_mut(19), b"1772741.16666");
    fstr::assign(EXPCTD.get_mut(20), b"1772781.20833");
    fstr::assign(EXPCTD.get_mut(21), b"1772817.25000");
    fstr::assign(EXPCTD.get_mut(22), b"1772852.29166");
    fstr::assign(EXPCTD.get_mut(23), b"1772888.33333");
    fstr::assign(EXPCTD.get_mut(24), b"1772924.37500");
    fstr::assign(EXPCTD.get_mut(25), b"2298873.91666");
    fstr::assign(EXPCTD.get_mut(26), b"2298908.95833");
    fstr::assign(EXPCTD.get_mut(27), b"2298942.00000");
    fstr::assign(EXPCTD.get_mut(28), b"2298978.04166");
    fstr::assign(EXPCTD.get_mut(29), b"2299013.08333");
    fstr::assign(EXPCTD.get_mut(30), b"2299049.12500");
    fstr::assign(EXPCTD.get_mut(31), b"2299055.16666");
    fstr::assign(EXPCTD.get_mut(32), b"2299095.20833");
    fstr::assign(EXPCTD.get_mut(33), b"2299131.25000");
    fstr::assign(EXPCTD.get_mut(34), b"2299158.29166");
    fstr::assign(EXPCTD.get_mut(35), b"2299171.33333");
    fstr::assign(EXPCTD.get_mut(36), b"2299238.37500");
    fstr::assign(EXPCTD.get_mut(37), b"2446431.91666");
    fstr::assign(EXPCTD.get_mut(38), b"2446466.95833");
    fstr::assign(EXPCTD.get_mut(39), b"2446500.00000");
    fstr::assign(EXPCTD.get_mut(40), b"2446536.04166");
    fstr::assign(EXPCTD.get_mut(41), b"2446571.08333");
    fstr::assign(EXPCTD.get_mut(42), b"2446607.12500");
    fstr::assign(EXPCTD.get_mut(43), b"2446613.16666");
    fstr::assign(EXPCTD.get_mut(44), b"2446653.20833");
    fstr::assign(EXPCTD.get_mut(45), b"2446689.25000");
    fstr::assign(EXPCTD.get_mut(46), b"2446724.29166");
    fstr::assign(EXPCTD.get_mut(47), b"2446760.33333");
    fstr::assign(EXPCTD.get_mut(48), b"2446796.37500");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY Month DD HR:MN:SC Weekday");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b" -11 January 01 10:00:00 Sunday");
    fstr::assign(EXPCTD.get_mut(2), b" -11 February 05 11:00:00 Sunday");
    fstr::assign(EXPCTD.get_mut(3), b" -11 March 10 12:00:00 Friday");
    fstr::assign(EXPCTD.get_mut(4), b" -11 April 15 13:00:00 Saturday");
    fstr::assign(EXPCTD.get_mut(5), b" -11 May 20 14:00:00 Saturday");
    fstr::assign(EXPCTD.get_mut(6), b" -11 June 25 15:00:00 Sunday");
    fstr::assign(EXPCTD.get_mut(7), b" -11 July 01 16:00:00 Saturday");
    fstr::assign(EXPCTD.get_mut(8), b" -11 August 10 17:00:00 Thursday");
    fstr::assign(EXPCTD.get_mut(9), b" -11 September 15 18:00:00 Friday");
    fstr::assign(EXPCTD.get_mut(10), b" -11 October 20 19:00:00 Friday");
    fstr::assign(EXPCTD.get_mut(11), b" -11 November 25 20:00:00 Saturday");
    fstr::assign(EXPCTD.get_mut(12), b" -11 December 31 21:00:00 Sunday");
    fstr::assign(EXPCTD.get_mut(13), b" 141 January 01 10:00:00 Sunday");
    fstr::assign(EXPCTD.get_mut(14), b" 141 February 05 11:00:00 Sunday");
    fstr::assign(EXPCTD.get_mut(15), b" 141 March 10 12:00:00 Friday");
    fstr::assign(EXPCTD.get_mut(16), b" 141 April 15 13:00:00 Saturday");
    fstr::assign(EXPCTD.get_mut(17), b" 141 May 20 14:00:00 Saturday");
    fstr::assign(EXPCTD.get_mut(18), b" 141 June 25 15:00:00 Sunday");
    fstr::assign(EXPCTD.get_mut(19), b" 141 July 01 16:00:00 Saturday");
    fstr::assign(EXPCTD.get_mut(20), b" 141 August 10 17:00:00 Thursday");
    fstr::assign(EXPCTD.get_mut(21), b" 141 September 15 18:00:00 Friday");
    fstr::assign(EXPCTD.get_mut(22), b" 141 October 20 19:00:00 Friday");
    fstr::assign(EXPCTD.get_mut(23), b" 141 November 25 20:00:00 Saturday");
    fstr::assign(EXPCTD.get_mut(24), b" 141 December 31 21:00:00 Sunday");
    fstr::assign(EXPCTD.get_mut(25), b"1582 January 01 10:00:00 Friday");
    fstr::assign(EXPCTD.get_mut(26), b"1582 February 05 11:00:00 Friday");
    fstr::assign(EXPCTD.get_mut(27), b"1582 March 10 12:00:00 Wednesday");
    fstr::assign(EXPCTD.get_mut(28), b"1582 April 15 13:00:00 Thursday");
    fstr::assign(EXPCTD.get_mut(29), b"1582 May 20 14:00:00 Thursday");
    fstr::assign(EXPCTD.get_mut(30), b"1582 June 25 15:00:00 Friday");
    fstr::assign(EXPCTD.get_mut(31), b"1582 July 01 16:00:00 Thursday");
    fstr::assign(EXPCTD.get_mut(32), b"1582 August 10 17:00:00 Tuesday");
    fstr::assign(EXPCTD.get_mut(33), b"1582 September 15 18:00:00 Wednesday");
    fstr::assign(EXPCTD.get_mut(34), b"1582 October 12 19:00:00 Tuesday");
    fstr::assign(EXPCTD.get_mut(35), b"1582 October 25 20:00:00 Monday");
    fstr::assign(EXPCTD.get_mut(36), b"1582 December 31 21:00:00 Friday");
    fstr::assign(EXPCTD.get_mut(37), b"1986 January 01 10:00:00 Wednesday");
    fstr::assign(EXPCTD.get_mut(38), b"1986 February 05 11:00:00 Wednesday");
    fstr::assign(EXPCTD.get_mut(39), b"1986 March 10 12:00:00 Monday");
    fstr::assign(EXPCTD.get_mut(40), b"1986 April 15 13:00:00 Tuesday");
    fstr::assign(EXPCTD.get_mut(41), b"1986 May 20 14:00:00 Tuesday");
    fstr::assign(EXPCTD.get_mut(42), b"1986 June 25 15:00:00 Wednesday");
    fstr::assign(EXPCTD.get_mut(43), b"1986 July 01 16:00:00 Tuesday");
    fstr::assign(EXPCTD.get_mut(44), b"1986 August 10 17:00:00 Sunday");
    fstr::assign(EXPCTD.get_mut(45), b"1986 September 15 18:00:00 Monday");
    fstr::assign(EXPCTD.get_mut(46), b"1986 October 20 19:00:00 Monday");
    fstr::assign(EXPCTD.get_mut(47), b"1986 November 25 20:00:00 Tuesday");
    fstr::assign(EXPCTD.get_mut(48), b"1986 December 31 21:00:00 Wednesday");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY ERA Month DD HR:MN:SC.###");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b"  12 B.C. January 01 10:00:00.000");
    fstr::assign(EXPCTD.get_mut(2), b"  12 B.C. February 05 11:00:00.000");
    fstr::assign(EXPCTD.get_mut(3), b"  12 B.C. March 10 12:00:00.000");
    fstr::assign(EXPCTD.get_mut(4), b"  12 B.C. April 15 13:00:00.000");
    fstr::assign(EXPCTD.get_mut(5), b"  12 B.C. May 20 14:00:00.000");
    fstr::assign(EXPCTD.get_mut(6), b"  12 B.C. June 25 15:00:00.000");
    fstr::assign(EXPCTD.get_mut(7), b"  12 B.C. July 01 16:00:00.000");
    fstr::assign(EXPCTD.get_mut(8), b"  12 B.C. August 10 17:00:00.000");
    fstr::assign(EXPCTD.get_mut(9), b"  12 B.C. September 15 18:00:00.000");
    fstr::assign(EXPCTD.get_mut(10), b"  12 B.C. October 20 19:00:00.000");
    fstr::assign(EXPCTD.get_mut(11), b"  12 B.C. November 25 20:00:00.000");
    fstr::assign(EXPCTD.get_mut(12), b"  12 B.C. December 31 21:00:00.000");
    fstr::assign(EXPCTD.get_mut(13), b" 141 A.D. January 01 10:00:00.000");
    fstr::assign(EXPCTD.get_mut(14), b" 141 A.D. February 05 11:00:00.000");
    fstr::assign(EXPCTD.get_mut(15), b" 141 A.D. March 10 12:00:00.000");
    fstr::assign(EXPCTD.get_mut(16), b" 141 A.D. April 15 13:00:00.000");
    fstr::assign(EXPCTD.get_mut(17), b" 141 A.D. May 20 14:00:00.000");
    fstr::assign(EXPCTD.get_mut(18), b" 141 A.D. June 25 15:00:00.000");
    fstr::assign(EXPCTD.get_mut(19), b" 141 A.D. July 01 16:00:00.000");
    fstr::assign(EXPCTD.get_mut(20), b" 141 A.D. August 10 17:00:00.000");
    fstr::assign(EXPCTD.get_mut(21), b" 141 A.D. September 15 18:00:00.000");
    fstr::assign(EXPCTD.get_mut(22), b" 141 A.D. October 20 19:00:00.000");
    fstr::assign(EXPCTD.get_mut(23), b" 141 A.D. November 25 20:00:00.000");
    fstr::assign(EXPCTD.get_mut(24), b" 141 A.D. December 31 21:00:00.000");
    fstr::assign(EXPCTD.get_mut(25), b"1582 A.D. January 01 10:00:00.000");
    fstr::assign(EXPCTD.get_mut(26), b"1582 A.D. February 05 11:00:00.000");
    fstr::assign(EXPCTD.get_mut(27), b"1582 A.D. March 10 12:00:00.000");
    fstr::assign(EXPCTD.get_mut(28), b"1582 A.D. April 15 13:00:00.000");
    fstr::assign(EXPCTD.get_mut(29), b"1582 A.D. May 20 14:00:00.000");
    fstr::assign(EXPCTD.get_mut(30), b"1582 A.D. June 25 15:00:00.000");
    fstr::assign(EXPCTD.get_mut(31), b"1582 A.D. July 01 16:00:00.000");
    fstr::assign(EXPCTD.get_mut(32), b"1582 A.D. August 10 17:00:00.000");
    fstr::assign(EXPCTD.get_mut(33), b"1582 A.D. September 15 18:00:00.000");
    fstr::assign(EXPCTD.get_mut(34), b"1582 A.D. October 12 19:00:00.000");
    fstr::assign(EXPCTD.get_mut(35), b"1582 A.D. October 25 20:00:00.000");
    fstr::assign(EXPCTD.get_mut(36), b"1582 A.D. December 31 21:00:00.000");
    fstr::assign(EXPCTD.get_mut(37), b"1986 A.D. January 01 10:00:00.000");
    fstr::assign(EXPCTD.get_mut(38), b"1986 A.D. February 05 11:00:00.000");
    fstr::assign(EXPCTD.get_mut(39), b"1986 A.D. March 10 12:00:00.000");
    fstr::assign(EXPCTD.get_mut(40), b"1986 A.D. April 15 13:00:00.000");
    fstr::assign(EXPCTD.get_mut(41), b"1986 A.D. May 20 14:00:00.000");
    fstr::assign(EXPCTD.get_mut(42), b"1986 A.D. June 25 15:00:00.000");
    fstr::assign(EXPCTD.get_mut(43), b"1986 A.D. July 01 16:00:00.000");
    fstr::assign(EXPCTD.get_mut(44), b"1986 A.D. August 10 17:00:00.000");
    fstr::assign(EXPCTD.get_mut(45), b"1986 A.D. September 15 18:00:00.000");
    fstr::assign(EXPCTD.get_mut(46), b"1986 A.D. October 20 19:00:00.000");
    fstr::assign(EXPCTD.get_mut(47), b"1986 A.D. November 25 20:00:00.000");
    fstr::assign(EXPCTD.get_mut(48), b"1986 A.D. December 31 21:00:00.000");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY ERA MM/DD HR:MN ::TRNC");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b"  12 B.C. 01/01 10:00");
    fstr::assign(EXPCTD.get_mut(2), b"  12 B.C. 02/05 11:00");
    fstr::assign(EXPCTD.get_mut(3), b"  12 B.C. 03/10 12:00");
    fstr::assign(EXPCTD.get_mut(4), b"  12 B.C. 04/15 13:00");
    fstr::assign(EXPCTD.get_mut(5), b"  12 B.C. 05/20 14:00");
    fstr::assign(EXPCTD.get_mut(6), b"  12 B.C. 06/25 15:00");
    fstr::assign(EXPCTD.get_mut(7), b"  12 B.C. 07/01 16:00");
    fstr::assign(EXPCTD.get_mut(8), b"  12 B.C. 08/10 17:00");
    fstr::assign(EXPCTD.get_mut(9), b"  12 B.C. 09/15 18:00");
    fstr::assign(EXPCTD.get_mut(10), b"  12 B.C. 10/20 19:00");
    fstr::assign(EXPCTD.get_mut(11), b"  12 B.C. 11/25 20:00");
    fstr::assign(EXPCTD.get_mut(12), b"  12 B.C. 12/31 21:00");
    fstr::assign(EXPCTD.get_mut(13), b" 141 A.D. 01/01 10:00");
    fstr::assign(EXPCTD.get_mut(14), b" 141 A.D. 02/05 11:00");
    fstr::assign(EXPCTD.get_mut(15), b" 141 A.D. 03/10 12:00");
    fstr::assign(EXPCTD.get_mut(16), b" 141 A.D. 04/15 13:00");
    fstr::assign(EXPCTD.get_mut(17), b" 141 A.D. 05/20 14:00");
    fstr::assign(EXPCTD.get_mut(18), b" 141 A.D. 06/25 15:00");
    fstr::assign(EXPCTD.get_mut(19), b" 141 A.D. 07/01 16:00");
    fstr::assign(EXPCTD.get_mut(20), b" 141 A.D. 08/10 17:00");
    fstr::assign(EXPCTD.get_mut(21), b" 141 A.D. 09/15 18:00");
    fstr::assign(EXPCTD.get_mut(22), b" 141 A.D. 10/20 19:00");
    fstr::assign(EXPCTD.get_mut(23), b" 141 A.D. 11/25 20:00");
    fstr::assign(EXPCTD.get_mut(24), b" 141 A.D. 12/31 21:00");
    fstr::assign(EXPCTD.get_mut(25), b"1582 A.D. 01/01 10:00");
    fstr::assign(EXPCTD.get_mut(26), b"1582 A.D. 02/05 11:00");
    fstr::assign(EXPCTD.get_mut(27), b"1582 A.D. 03/10 12:00");
    fstr::assign(EXPCTD.get_mut(28), b"1582 A.D. 04/15 13:00");
    fstr::assign(EXPCTD.get_mut(29), b"1582 A.D. 05/20 14:00");
    fstr::assign(EXPCTD.get_mut(30), b"1582 A.D. 06/25 15:00");
    fstr::assign(EXPCTD.get_mut(31), b"1582 A.D. 07/01 16:00");
    fstr::assign(EXPCTD.get_mut(32), b"1582 A.D. 08/10 17:00");
    fstr::assign(EXPCTD.get_mut(33), b"1582 A.D. 09/15 18:00");
    fstr::assign(EXPCTD.get_mut(34), b"1582 A.D. 10/12 19:00");
    fstr::assign(EXPCTD.get_mut(35), b"1582 A.D. 10/25 20:00");
    fstr::assign(EXPCTD.get_mut(36), b"1582 A.D. 12/31 21:00");
    fstr::assign(EXPCTD.get_mut(37), b"1986 A.D. 01/01 10:00");
    fstr::assign(EXPCTD.get_mut(38), b"1986 A.D. 02/05 11:00");
    fstr::assign(EXPCTD.get_mut(39), b"1986 A.D. 03/10 12:00");
    fstr::assign(EXPCTD.get_mut(40), b"1986 A.D. 04/15 13:00");
    fstr::assign(EXPCTD.get_mut(41), b"1986 A.D. 05/20 14:00");
    fstr::assign(EXPCTD.get_mut(42), b"1986 A.D. 06/25 15:00");
    fstr::assign(EXPCTD.get_mut(43), b"1986 A.D. 07/01 16:00");
    fstr::assign(EXPCTD.get_mut(44), b"1986 A.D. 08/10 17:00");
    fstr::assign(EXPCTD.get_mut(45), b"1986 A.D. 09/15 18:00");
    fstr::assign(EXPCTD.get_mut(46), b"1986 A.D. 10/20 19:00");
    fstr::assign(EXPCTD.get_mut(47), b"1986 A.D. 11/25 20:00");
    fstr::assign(EXPCTD.get_mut(48), b"1986 A.D. 12/31 21:00");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY ERA MONTH DD WEEKDAY");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b"  12 B.C. JANUARY 01 SUNDAY");
    fstr::assign(EXPCTD.get_mut(2), b"  12 B.C. FEBRUARY 05 SUNDAY");
    fstr::assign(EXPCTD.get_mut(3), b"  12 B.C. MARCH 10 FRIDAY");
    fstr::assign(EXPCTD.get_mut(4), b"  12 B.C. APRIL 15 SATURDAY");
    fstr::assign(EXPCTD.get_mut(5), b"  12 B.C. MAY 20 SATURDAY");
    fstr::assign(EXPCTD.get_mut(6), b"  12 B.C. JUNE 25 SUNDAY");
    fstr::assign(EXPCTD.get_mut(7), b"  12 B.C. JULY 01 SATURDAY");
    fstr::assign(EXPCTD.get_mut(8), b"  12 B.C. AUGUST 10 THURSDAY");
    fstr::assign(EXPCTD.get_mut(9), b"  12 B.C. SEPTEMBER 15 FRIDAY");
    fstr::assign(EXPCTD.get_mut(10), b"  12 B.C. OCTOBER 20 FRIDAY");
    fstr::assign(EXPCTD.get_mut(11), b"  12 B.C. NOVEMBER 25 SATURDAY");
    fstr::assign(EXPCTD.get_mut(12), b"  12 B.C. DECEMBER 31 SUNDAY");
    fstr::assign(EXPCTD.get_mut(13), b" 141 A.D. JANUARY 01 SUNDAY");
    fstr::assign(EXPCTD.get_mut(14), b" 141 A.D. FEBRUARY 05 SUNDAY");
    fstr::assign(EXPCTD.get_mut(15), b" 141 A.D. MARCH 10 FRIDAY");
    fstr::assign(EXPCTD.get_mut(16), b" 141 A.D. APRIL 15 SATURDAY");
    fstr::assign(EXPCTD.get_mut(17), b" 141 A.D. MAY 20 SATURDAY");
    fstr::assign(EXPCTD.get_mut(18), b" 141 A.D. JUNE 25 SUNDAY");
    fstr::assign(EXPCTD.get_mut(19), b" 141 A.D. JULY 01 SATURDAY");
    fstr::assign(EXPCTD.get_mut(20), b" 141 A.D. AUGUST 10 THURSDAY");
    fstr::assign(EXPCTD.get_mut(21), b" 141 A.D. SEPTEMBER 15 FRIDAY");
    fstr::assign(EXPCTD.get_mut(22), b" 141 A.D. OCTOBER 20 FRIDAY");
    fstr::assign(EXPCTD.get_mut(23), b" 141 A.D. NOVEMBER 25 SATURDAY");
    fstr::assign(EXPCTD.get_mut(24), b" 141 A.D. DECEMBER 31 SUNDAY");
    fstr::assign(EXPCTD.get_mut(25), b"1582 A.D. JANUARY 01 FRIDAY");
    fstr::assign(EXPCTD.get_mut(26), b"1582 A.D. FEBRUARY 05 FRIDAY");
    fstr::assign(EXPCTD.get_mut(27), b"1582 A.D. MARCH 10 WEDNESDAY");
    fstr::assign(EXPCTD.get_mut(28), b"1582 A.D. APRIL 15 THURSDAY");
    fstr::assign(EXPCTD.get_mut(29), b"1582 A.D. MAY 20 THURSDAY");
    fstr::assign(EXPCTD.get_mut(30), b"1582 A.D. JUNE 25 FRIDAY");
    fstr::assign(EXPCTD.get_mut(31), b"1582 A.D. JULY 01 THURSDAY");
    fstr::assign(EXPCTD.get_mut(32), b"1582 A.D. AUGUST 10 TUESDAY");
    fstr::assign(EXPCTD.get_mut(33), b"1582 A.D. SEPTEMBER 15 WEDNESDAY");
    fstr::assign(EXPCTD.get_mut(34), b"1582 A.D. OCTOBER 12 TUESDAY");
    fstr::assign(EXPCTD.get_mut(35), b"1582 A.D. OCTOBER 25 MONDAY");
    fstr::assign(EXPCTD.get_mut(36), b"1582 A.D. DECEMBER 31 FRIDAY");
    fstr::assign(EXPCTD.get_mut(37), b"1986 A.D. JANUARY 01 WEDNESDAY");
    fstr::assign(EXPCTD.get_mut(38), b"1986 A.D. FEBRUARY 05 WEDNESDAY");
    fstr::assign(EXPCTD.get_mut(39), b"1986 A.D. MARCH 10 MONDAY");
    fstr::assign(EXPCTD.get_mut(40), b"1986 A.D. APRIL 15 TUESDAY");
    fstr::assign(EXPCTD.get_mut(41), b"1986 A.D. MAY 20 TUESDAY");
    fstr::assign(EXPCTD.get_mut(42), b"1986 A.D. JUNE 25 WEDNESDAY");
    fstr::assign(EXPCTD.get_mut(43), b"1986 A.D. JULY 01 TUESDAY");
    fstr::assign(EXPCTD.get_mut(44), b"1986 A.D. AUGUST 10 SUNDAY");
    fstr::assign(EXPCTD.get_mut(45), b"1986 A.D. SEPTEMBER 15 MONDAY");
    fstr::assign(EXPCTD.get_mut(46), b"1986 A.D. OCTOBER 20 MONDAY");
    fstr::assign(EXPCTD.get_mut(47), b"1986 A.D. NOVEMBER 25 TUESDAY");
    fstr::assign(EXPCTD.get_mut(48), b"1986 A.D. DECEMBER 31 WEDNESDAY");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"YYYY era MON DD");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b"  12 b.c. JAN 01");
    fstr::assign(EXPCTD.get_mut(2), b"  12 b.c. FEB 05");
    fstr::assign(EXPCTD.get_mut(3), b"  12 b.c. MAR 10");
    fstr::assign(EXPCTD.get_mut(4), b"  12 b.c. APR 15");
    fstr::assign(EXPCTD.get_mut(5), b"  12 b.c. MAY 20");
    fstr::assign(EXPCTD.get_mut(6), b"  12 b.c. JUN 25");
    fstr::assign(EXPCTD.get_mut(7), b"  12 b.c. JUL 01");
    fstr::assign(EXPCTD.get_mut(8), b"  12 b.c. AUG 10");
    fstr::assign(EXPCTD.get_mut(9), b"  12 b.c. SEP 15");
    fstr::assign(EXPCTD.get_mut(10), b"  12 b.c. OCT 20");
    fstr::assign(EXPCTD.get_mut(11), b"  12 b.c. NOV 25");
    fstr::assign(EXPCTD.get_mut(12), b"  12 b.c. DEC 31");
    fstr::assign(EXPCTD.get_mut(13), b" 141 a.d. JAN 01");
    fstr::assign(EXPCTD.get_mut(14), b" 141 a.d. FEB 05");
    fstr::assign(EXPCTD.get_mut(15), b" 141 a.d. MAR 10");
    fstr::assign(EXPCTD.get_mut(16), b" 141 a.d. APR 15");
    fstr::assign(EXPCTD.get_mut(17), b" 141 a.d. MAY 20");
    fstr::assign(EXPCTD.get_mut(18), b" 141 a.d. JUN 25");
    fstr::assign(EXPCTD.get_mut(19), b" 141 a.d. JUL 01");
    fstr::assign(EXPCTD.get_mut(20), b" 141 a.d. AUG 10");
    fstr::assign(EXPCTD.get_mut(21), b" 141 a.d. SEP 15");
    fstr::assign(EXPCTD.get_mut(22), b" 141 a.d. OCT 20");
    fstr::assign(EXPCTD.get_mut(23), b" 141 a.d. NOV 25");
    fstr::assign(EXPCTD.get_mut(24), b" 141 a.d. DEC 31");
    fstr::assign(EXPCTD.get_mut(25), b"1582 a.d. JAN 01");
    fstr::assign(EXPCTD.get_mut(26), b"1582 a.d. FEB 05");
    fstr::assign(EXPCTD.get_mut(27), b"1582 a.d. MAR 10");
    fstr::assign(EXPCTD.get_mut(28), b"1582 a.d. APR 15");
    fstr::assign(EXPCTD.get_mut(29), b"1582 a.d. MAY 20");
    fstr::assign(EXPCTD.get_mut(30), b"1582 a.d. JUN 25");
    fstr::assign(EXPCTD.get_mut(31), b"1582 a.d. JUL 01");
    fstr::assign(EXPCTD.get_mut(32), b"1582 a.d. AUG 10");
    fstr::assign(EXPCTD.get_mut(33), b"1582 a.d. SEP 15");
    fstr::assign(EXPCTD.get_mut(34), b"1582 a.d. OCT 12");
    fstr::assign(EXPCTD.get_mut(35), b"1582 a.d. OCT 25");
    fstr::assign(EXPCTD.get_mut(36), b"1582 a.d. DEC 31");
    fstr::assign(EXPCTD.get_mut(37), b"1986 a.d. JAN 01");
    fstr::assign(EXPCTD.get_mut(38), b"1986 a.d. FEB 05");
    fstr::assign(EXPCTD.get_mut(39), b"1986 a.d. MAR 10");
    fstr::assign(EXPCTD.get_mut(40), b"1986 a.d. APR 15");
    fstr::assign(EXPCTD.get_mut(41), b"1986 a.d. MAY 20");
    fstr::assign(EXPCTD.get_mut(42), b"1986 a.d. JUN 25");
    fstr::assign(EXPCTD.get_mut(43), b"1986 a.d. JUL 01");
    fstr::assign(EXPCTD.get_mut(44), b"1986 a.d. AUG 10");
    fstr::assign(EXPCTD.get_mut(45), b"1986 a.d. SEP 15");
    fstr::assign(EXPCTD.get_mut(46), b"1986 a.d. OCT 20");
    fstr::assign(EXPCTD.get_mut(47), b"1986 a.d. NOV 25");
    fstr::assign(EXPCTD.get_mut(48), b"1986 a.d. DEC 31");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"JULIAND.####### ::TDB");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b"1717042.9171433");
    fstr::assign(EXPCTD.get_mut(2), b"1717077.9588100");
    fstr::assign(EXPCTD.get_mut(3), b"1717111.0004766");
    fstr::assign(EXPCTD.get_mut(4), b"1717147.0421433");
    fstr::assign(EXPCTD.get_mut(5), b"1717182.0838100");
    fstr::assign(EXPCTD.get_mut(6), b"1717218.1254766");
    fstr::assign(EXPCTD.get_mut(7), b"1717224.1671433");
    fstr::assign(EXPCTD.get_mut(8), b"1717264.2088099");
    fstr::assign(EXPCTD.get_mut(9), b"1717300.2504766");
    fstr::assign(EXPCTD.get_mut(10), b"1717335.2921433");
    fstr::assign(EXPCTD.get_mut(11), b"1717371.3338099");
    fstr::assign(EXPCTD.get_mut(12), b"1717407.3754766");
    fstr::assign(EXPCTD.get_mut(13), b"1772559.9171433");
    fstr::assign(EXPCTD.get_mut(14), b"1772594.9588100");
    fstr::assign(EXPCTD.get_mut(15), b"1772628.0004766");
    fstr::assign(EXPCTD.get_mut(16), b"1772664.0421433");
    fstr::assign(EXPCTD.get_mut(17), b"1772699.0838100");
    fstr::assign(EXPCTD.get_mut(18), b"1772735.1254766");
    fstr::assign(EXPCTD.get_mut(19), b"1772741.1671433");
    fstr::assign(EXPCTD.get_mut(20), b"1772781.2088099");
    fstr::assign(EXPCTD.get_mut(21), b"1772817.2504766");
    fstr::assign(EXPCTD.get_mut(22), b"1772852.2921433");
    fstr::assign(EXPCTD.get_mut(23), b"1772888.3338099");
    fstr::assign(EXPCTD.get_mut(24), b"1772924.3754766");
    fstr::assign(EXPCTD.get_mut(25), b"2298873.9171433");
    fstr::assign(EXPCTD.get_mut(26), b"2298908.9588100");
    fstr::assign(EXPCTD.get_mut(27), b"2298942.0004766");
    fstr::assign(EXPCTD.get_mut(28), b"2298978.0421433");
    fstr::assign(EXPCTD.get_mut(29), b"2299013.0838100");
    fstr::assign(EXPCTD.get_mut(30), b"2299049.1254766");
    fstr::assign(EXPCTD.get_mut(31), b"2299055.1671433");
    fstr::assign(EXPCTD.get_mut(32), b"2299095.2088099");
    fstr::assign(EXPCTD.get_mut(33), b"2299131.2504766");
    fstr::assign(EXPCTD.get_mut(34), b"2299158.2921433");
    fstr::assign(EXPCTD.get_mut(35), b"2299171.3338099");
    fstr::assign(EXPCTD.get_mut(36), b"2299238.3754766");
    fstr::assign(EXPCTD.get_mut(37), b"2446431.9173053");
    fstr::assign(EXPCTD.get_mut(38), b"2446466.9589720");
    fstr::assign(EXPCTD.get_mut(39), b"2446500.0006387");
    fstr::assign(EXPCTD.get_mut(40), b"2446536.0423053");
    fstr::assign(EXPCTD.get_mut(41), b"2446571.0839720");
    fstr::assign(EXPCTD.get_mut(42), b"2446607.1256387");
    fstr::assign(EXPCTD.get_mut(43), b"2446613.1673053");
    fstr::assign(EXPCTD.get_mut(44), b"2446653.2089720");
    fstr::assign(EXPCTD.get_mut(45), b"2446689.2506386");
    fstr::assign(EXPCTD.get_mut(46), b"2446724.2923053");
    fstr::assign(EXPCTD.get_mut(47), b"2446760.3339720");
    fstr::assign(EXPCTD.get_mut(48), b"2446796.3756387");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &PICTUR, &mut FTIME, ctx)?;
        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    fstr::assign(&mut PICTUR, b"JULIAND.####### ");

    testutil::TCASE(&PICTUR, ctx)?;

    spicelib::TIMDEF(b"SET", b"CALENDAR", &mut b"GREGORIAN".clone(), ctx)?;
    spicelib::TIMDEF(b"SET", b"SYSTEM", &mut b"UTC".clone(), ctx)?;

    fstr::assign(EXPCTD.get_mut(1), b"1717042.9171433");
    fstr::assign(EXPCTD.get_mut(2), b"1717077.9588099");
    fstr::assign(EXPCTD.get_mut(3), b"1717111.0004766");
    fstr::assign(EXPCTD.get_mut(4), b"1717147.0421433");
    fstr::assign(EXPCTD.get_mut(5), b"1717182.0838099");
    fstr::assign(EXPCTD.get_mut(6), b"1717218.1254766");
    fstr::assign(EXPCTD.get_mut(7), b"1717224.1671433");
    fstr::assign(EXPCTD.get_mut(8), b"1717264.2088099");
    fstr::assign(EXPCTD.get_mut(9), b"1717300.2504766");
    fstr::assign(EXPCTD.get_mut(10), b"1717335.2921433");
    fstr::assign(EXPCTD.get_mut(11), b"1717371.3338099");
    fstr::assign(EXPCTD.get_mut(12), b"1717407.3754766");
    fstr::assign(EXPCTD.get_mut(13), b"1772559.9171433");
    fstr::assign(EXPCTD.get_mut(14), b"1772594.9588099");
    fstr::assign(EXPCTD.get_mut(15), b"1772628.0004766");
    fstr::assign(EXPCTD.get_mut(16), b"1772664.0421433");
    fstr::assign(EXPCTD.get_mut(17), b"1772699.0838099");
    fstr::assign(EXPCTD.get_mut(18), b"1772735.1254766");
    fstr::assign(EXPCTD.get_mut(19), b"1772741.1671433");
    fstr::assign(EXPCTD.get_mut(20), b"1772781.2088099");
    fstr::assign(EXPCTD.get_mut(21), b"1772817.2504766");
    fstr::assign(EXPCTD.get_mut(22), b"1772852.2921433");
    fstr::assign(EXPCTD.get_mut(23), b"1772888.3338099");
    fstr::assign(EXPCTD.get_mut(24), b"1772924.3754766");
    fstr::assign(EXPCTD.get_mut(25), b"2298873.9171433");
    fstr::assign(EXPCTD.get_mut(26), b"2298908.9588100");
    fstr::assign(EXPCTD.get_mut(27), b"2298942.0004766");
    fstr::assign(EXPCTD.get_mut(28), b"2298978.0421433");
    fstr::assign(EXPCTD.get_mut(29), b"2299013.0838100");
    fstr::assign(EXPCTD.get_mut(30), b"2299049.1254766");
    fstr::assign(EXPCTD.get_mut(31), b"2299055.1671433");
    fstr::assign(EXPCTD.get_mut(32), b"2299095.2088100");
    fstr::assign(EXPCTD.get_mut(33), b"2299131.2504766");
    fstr::assign(EXPCTD.get_mut(34), b"2299158.2921433");
    fstr::assign(EXPCTD.get_mut(35), b"2299171.3338100");
    fstr::assign(EXPCTD.get_mut(36), b"2299238.3754766");
    fstr::assign(EXPCTD.get_mut(37), b"2446431.9173053");
    fstr::assign(EXPCTD.get_mut(38), b"2446466.9589720");
    fstr::assign(EXPCTD.get_mut(39), b"2446500.0006387");
    fstr::assign(EXPCTD.get_mut(40), b"2446536.0423053");
    fstr::assign(EXPCTD.get_mut(41), b"2446571.0839720");
    fstr::assign(EXPCTD.get_mut(42), b"2446607.1256387");
    fstr::assign(EXPCTD.get_mut(43), b"2446613.1673053");
    fstr::assign(EXPCTD.get_mut(44), b"2446653.2089720");
    fstr::assign(EXPCTD.get_mut(45), b"2446689.2506387");
    fstr::assign(EXPCTD.get_mut(46), b"2446724.2923053");
    fstr::assign(EXPCTD.get_mut(47), b"2446760.3339720");
    fstr::assign(EXPCTD.get_mut(48), b"2446796.3756387");

    for I in 1..=48 {
        spicelib::STR2ET(&TIMES[I], &mut ET, ctx)?;
        spicelib::TIMOUT(ET, &fstr::concat(&PICTUR, b"::TDT"), &mut FTIME, ctx)?;

        fstr::assign(&mut NAME, b"FTIME(#)");
        spicelib::REPMI(&NAME.clone(), b"#", I, &mut NAME, ctx);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;

        spicelib::TIMOUT(ET, &fstr::concat(&PICTUR, b"::TT"), &mut FTIME, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(&NAME, &FTIME, b"=", &EXPCTD[I], OK, ctx)?;
    }

    //
    // All 'DO Loops' have been executed.
    // All 'IF branches' have been executed.
    //
    //
    // Clean up after ourselves.  Clear the kernel pool.
    //
    spicelib::CLPOOL(ctx)?;
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
