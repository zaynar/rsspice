//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure F_TEXPYR ( Family of tests for TEXPYR )
pub fn F_TEXPYR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut YEAR: i32 = 0;
    let mut EXPYR: i32 = 0;

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_TEXPYR", ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: TEXPYR and TSETYR
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure that in the default case two digit years range are mapped to the interval from 1969 to 2068.", ctx)?;

    for I in 0..=99 {
        YEAR = I;

        spicelib::TEXPYR(&mut YEAR, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (I <= 68) {
            EXPYR = (2000 + I);
        } else {
            EXPYR = (1900 + I);
        }

        testutil::CHCKSI(b"YEAR", YEAR, b"=", EXPYR, 0, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure that when we set the lower bound for the century that all two digit years are expanded appropriately. ", ctx)?;

    spicelib::TSETYR(2025, ctx);

    for I in 0..=99 {
        YEAR = I;

        spicelib::TEXPYR(&mut YEAR, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (I <= 24) {
            EXPYR = (2100 + I);
        } else {
            EXPYR = (2000 + I);
        }

        testutil::CHCKSI(b"YEAR", YEAR, b"=", EXPYR, 0, OK, ctx)?;
    }

    //
    // Reset the lower bound and re-do the first set of tests.
    //
    spicelib::TSETYR(1969, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    for I in 0..=99 {
        YEAR = I;

        spicelib::TEXPYR(&mut YEAR, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (I <= 68) {
            EXPYR = (2000 + I);
        } else {
            EXPYR = (1900 + I);
        }

        testutil::CHCKSI(b"YEAR", YEAR, b"=", EXPYR, 0, OK, ctx)?;
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure that non-positive years provided to TSETYR do not change the lower bound for the year expansion.", ctx)?;

    for J in -1..=0 {
        spicelib::TSETYR(J, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        for I in 0..=99 {
            YEAR = I;

            spicelib::TEXPYR(&mut YEAR, ctx);
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Lower bound should still be 1969.
            //
            if (I <= 68) {
                EXPYR = (2000 + I);
            } else {
                EXPYR = (1900 + I);
            }

            testutil::CHCKSI(b"YEAR", YEAR, b"=", EXPYR, 0, OK, ctx)?;
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure that years outside the inclusive range from 0 to 99 are not altered by TEXPYR. ", ctx)?;

    YEAR = 1928;
    EXPYR = 1928;

    spicelib::TEXPYR(&mut YEAR, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"YEAR", YEAR, b"=", EXPYR, 0, OK, ctx)?;

    YEAR = 100;
    EXPYR = 100;

    spicelib::TEXPYR(&mut YEAR, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"YEAR", YEAR, b"=", EXPYR, 0, OK, ctx)?;

    YEAR = -1;
    EXPYR = -1;

    spicelib::TEXPYR(&mut YEAR, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSI(b"YEAR", YEAR, b"=", EXPYR, 0, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
