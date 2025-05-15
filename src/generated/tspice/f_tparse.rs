//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure F_TPARSE ( Family of tests for TPARSE )
pub fn F_TPARSE(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TSTRNG = ActualCharArray::new(LNSIZE, 1..=23);
    let mut ERROR = [b' '; LNSIZE as usize];
    let mut EXPDET = StackArray::<f64, 23>::new(1..=23);
    let mut SP2000: f64 = 0.0;

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TPARSE", ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: TPARSE
    //
    // *****************************************************************
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Check to make a standard list of strings map to the expected ET\'s ",
        ctx,
    )?;

    spicelib::TPARCH(b"NO", ctx);

    fstr::assign(TSTRNG.get_mut(1), b"1/9/1986 3:12:59.2");
    EXPDET[1] = -441103620.8;

    fstr::assign(TSTRNG.get_mut(2), b"9 JAN 1986 03:12:59.2");
    EXPDET[2] = -441103620.8;

    fstr::assign(TSTRNG.get_mut(3), b"1 9 1986 3:12:59.2");
    EXPDET[3] = -441103620.8;

    fstr::assign(TSTRNG.get_mut(4), b"9 JAN 1986 03:12:59.2");
    EXPDET[4] = -441103620.8;

    fstr::assign(TSTRNG.get_mut(5), b"2 jan 1991 3:00:12.2");
    EXPDET[5] = -283942787.8;

    fstr::assign(TSTRNG.get_mut(6), b"2 JAN 1991 03:00:12.2");
    EXPDET[6] = -283942787.8;

    fstr::assign(TSTRNG.get_mut(7), b"1991 MAR 10 12:00:00");
    EXPDET[7] = -278121600.0;

    fstr::assign(TSTRNG.get_mut(8), b"10 MAR 1991 12:00:00");
    EXPDET[8] = -278121600.0;

    fstr::assign(TSTRNG.get_mut(9), b"29 February 1975 3:00");
    EXPDET[9] = -783853200.0;

    fstr::assign(TSTRNG.get_mut(10), b"1 MAR 1975 03:00:00");
    EXPDET[10] = -783853200.0;

    fstr::assign(TSTRNG.get_mut(11), b"2010 October 29 3:58");
    EXPDET[11] = 341596680.0;

    fstr::assign(TSTRNG.get_mut(12), b"29 OCT 2010 03:58:00");
    EXPDET[12] = 341596680.0;

    fstr::assign(TSTRNG.get_mut(13), b"dec 31 86 12");
    EXPDET[13] = -410313600.0;

    fstr::assign(TSTRNG.get_mut(14), b"31 DEC 1986 12:00:00");
    EXPDET[14] = -410313600.0;

    fstr::assign(TSTRNG.get_mut(15), b"86-365 // 12:00");
    EXPDET[15] = -410313600.0;

    fstr::assign(TSTRNG.get_mut(16), b"31 DEC 1986 12:00:00");
    EXPDET[16] = -410313600.0;

    fstr::assign(TSTRNG.get_mut(17), b"JD 2451545.");
    EXPDET[17] = 0.0;

    fstr::assign(TSTRNG.get_mut(18), b"1 JAN 2000 12:00:00");
    EXPDET[18] = 0.0;

    fstr::assign(TSTRNG.get_mut(19), b"jd 2451545.");
    EXPDET[19] = 0.0;

    fstr::assign(TSTRNG.get_mut(20), b"1 JAN 2000 12:00:00");
    EXPDET[20] = 0.0;

    fstr::assign(TSTRNG.get_mut(21), b"JD2451545.");
    EXPDET[21] = 0.0;

    fstr::assign(TSTRNG.get_mut(22), b"1 JAN 2000 12:00:00");
    EXPDET[22] = 0.0;

    fstr::assign(TSTRNG.get_mut(23), b"321 B.C. MAR 15 12:00:00");
    EXPDET[23] = -(86400.0 * (847362.0 - 74.0));

    fstr::assign(&mut ERROR, b" ");

    for I in 1..=23 {
        testutil::TSTMSG(b"#", b"Test subcase #.", ctx);
        testutil::TSTMSI(I, ctx);

        spicelib::TPARSE(&TSTRNG[I], &mut SP2000, &mut ERROR, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;
        testutil::CHCKSD(b"SP2000", SP2000, b"=", EXPDET[I], 0.0, OK, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
