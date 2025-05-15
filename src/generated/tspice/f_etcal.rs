//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 48;

//$Procedure      F_ETCAL ( Family of tests for ETCAL )
pub fn F_ETCAL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ET: f64 = 0.0;
    let mut ET2: f64 = 0.0;
    let mut STRING = [b' '; WDSIZE as usize];
    let mut ERROR = [b' '; WDSIZE as usize];
    let mut TITLE = [b' '; WDSIZE as usize];
    let mut J: i32 = 0;

    testutil::TOPEN(b"F_ETCAL", ctx)?;

    testutil::TCASE(b"Times out of range --- too big ", ctx)?;

    ET = 100000000000000000.0;

    spicelib::ETCAL(ET, &mut STRING, ctx);
    testutil::CHCKSC(
        b"STRING",
        fstr::substr(&STRING, 1..=12),
        b"=",
        b"Epoch after ",
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Times out of range --- too small ", ctx)?;

    ET = -100000000000000000.0;

    spicelib::ETCAL(ET, &mut STRING, ctx);
    testutil::CHCKSC(
        b"STRING",
        fstr::substr(&STRING, 1..=12),
        b"=",
        b"Epoch before",
        OK,
        ctx,
    )?;

    testutil::TCASE(b"Zero point of the Julian Date Scale", ctx)?;

    ET = -(spicelib::J2000() * spicelib::SPD());
    spicelib::ETCAL(ET, &mut STRING, ctx);
    testutil::CHCKSC(
        b"STRING",
        &STRING,
        b"=",
        b"4714 B.C. NOV 24 12:00:00.000",
        OK,
        ctx,
    )?;

    testutil::TCASE(b"From a string to ET and back ", ctx)?;

    spicelib::TPARSE(b"1993 JAN 12, 13:12:28.999", &mut ET, &mut ERROR, ctx)?;
    spicelib::ETCAL(ET, &mut STRING, ctx);
    testutil::CHCKSC(
        b"STRING",
        &STRING,
        b"=",
        b"1993 JAN 12 13:12:28.999",
        OK,
        ctx,
    )?;

    testutil::TCASE(b"From a string to ET and back ", ctx)?;

    spicelib::TPARSE(b"893 JAN 12, 13:12:28.999", &mut ET, &mut ERROR, ctx)?;
    spicelib::ETCAL(ET, &mut STRING, ctx);
    testutil::CHCKSC(
        b"STRING",
        &STRING,
        b"=",
        b"893 A.D. JAN 12 13:12:28.999",
        OK,
        ctx,
    )?;

    //
    // In this case we loop over a large collection of ET values and
    // make sure that TPARSE reproduces the ET output from ETCAL.
    //
    ET = -100000000.0;
    for I in 1..=100 {
        fstr::assign(&mut TITLE, b"TPARSE - ETCAL Compatibility #");
        J = I;

        spicelib::REPMI(&TITLE.clone(), b"#", J, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        ET = (ET + 2000000.0);
        fstr::assign(&mut ERROR, b" ");

        spicelib::ETCAL(ET, &mut STRING, ctx);
        spicelib::TPARSE(&STRING, &mut ET2, &mut ERROR, ctx)?;

        testutil::CHCKSC(b"ERROR", &ERROR, b"=", b" ", OK, ctx)?;
        testutil::CHCKSD(b"ET", ET, b"=", ET2, 0.0, OK, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
