//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      F_JUL2GR ( Family of tests for Julian and Gregorian
pub fn F_JUL2GR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LEAPS: i32 = 0;
    let mut YEAR: i32 = 0;
    let mut MONTH: i32 = 0;
    let mut DAY: i32 = 0;
    let mut DAYS: i32 = 0;
    let mut DOY: i32 = 0;
    let mut THISYR: i32 = 0;
    let mut THISMO: i32 = 0;
    let mut THISDY: i32 = 0;

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
    testutil::TOPEN(b"F_JUL2GR", ctx)?;

    testutil::TCASE(
        b"The Julian Date corresponding to October 15, 1582 should be October 5, 1582 ",
        ctx,
    )?;

    YEAR = 1582;
    MONTH = 10;
    DAY = 15;

    spicelib::GR2JUL(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"YEAR", YEAR, b"=", 1582, 0, OK, ctx)?;
    testutil::CHCKSI(b"MONTH", MONTH, b"=", 10, 0, OK, ctx)?;
    testutil::CHCKSI(b"DAY", DAY, b"=", 5, 0, OK, ctx)?;

    testutil::TCASE(b"The Gregorian date corresponding to the Julian date October 5, 1582 should be October 15, 1582 ", ctx)?;

    YEAR = 1582;
    MONTH = 10;
    DAY = 5;

    spicelib::JUL2GR(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"YEAR", YEAR, b"=", 1582, 0, OK, ctx)?;
    testutil::CHCKSI(b"MONTH", MONTH, b"=", 10, 0, OK, ctx)?;
    testutil::CHCKSI(b"DAY", DAY, b"=", 15, 0, OK, ctx)?;

    testutil::TCASE(
        b"September 3, 1752 Julian, should be September 14, 1752 Gregorian ",
        ctx,
    )?;

    YEAR = 1752;
    MONTH = 9;
    DAY = 3;

    spicelib::JUL2GR(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSI(b"YEAR", YEAR, b"=", 1752, 0, OK, ctx)?;
    testutil::CHCKSI(b"MONTH", MONTH, b"=", 9, 0, OK, ctx)?;
    testutil::CHCKSI(b"DAY", DAY, b"=", 14, 0, OK, ctx)?;

    testutil::TCASE(b"Make sure that on centuries, the number of days in the year is 366 for each Julian Year. ", ctx)?;

    for I in intrinsics::range(0, 2000, 100) {
        YEAR = I;
        MONTH = 12;
        DAY = 31;

        spicelib::JUL2GR(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;
        spicelib::GR2JUL(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"YEAR", YEAR, b"=", I, 0, OK, ctx)?;
        testutil::CHCKSI(b"MONTH", MONTH, b"=", 12, 0, OK, ctx)?;
        testutil::CHCKSI(b"DAY", DAY, b"=", 31, 0, OK, ctx)?;
        testutil::CHCKSI(b"DOY", DOY, b"=", 366, 0, OK, ctx)?;
    }

    testutil::TCASE(b"Make sure that the number of days on centuries is 366 only on those centuries divisible by 400. ", ctx)?;

    for I in intrinsics::range(0, 2000, 100) {
        YEAR = I;
        DAYS = (365 + intrinsics::MAX0(&[((1 + ((YEAR / 400) * 400)) - YEAR), 0]));

        MONTH = 12;
        DAY = 31;

        spicelib::GR2JUL(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;
        spicelib::JUL2GR(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(b"YEAR", YEAR, b"=", I, 0, OK, ctx)?;
        testutil::CHCKSI(b"MONTH", MONTH, b"=", 12, 0, OK, ctx)?;
        testutil::CHCKSI(b"DAY", DAY, b"=", 31, 0, OK, ctx)?;
        testutil::CHCKSI(b"DOY", DOY, b"=", DAYS, 0, OK, ctx)?;
    }

    testutil::TCASE(
        b"Perform a large set of cases from Julian to Gregorian and back. ",
        ctx,
    )?;

    for I in 1904..=1904 {
        YEAR = I;
        LEAPS = intrinsics::MAX0(&[((((YEAR / 4) * 4) - YEAR) + 1), 0]);

        for J in 1..=12 {
            MONTH = J;

            for K in 1..=31 {
                if (MONTH == 2) {
                    DAY = intrinsics::MIN0(&[(28 + LEAPS), K]);
                } else if (MONTH == 4) {
                    DAY = intrinsics::MIN0(&[30, K]);
                } else if (MONTH == 6) {
                    DAY = intrinsics::MIN0(&[30, K]);
                } else if (MONTH == 9) {
                    DAY = intrinsics::MIN0(&[30, K]);
                } else if (MONTH == 11) {
                    DAY = intrinsics::MIN0(&[30, K]);
                } else {
                    DAY = K;
                }

                THISYR = YEAR;
                THISMO = MONTH;
                THISDY = DAY;

                spicelib::JUL2GR(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;
                spicelib::GR2JUL(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSI(b"YEAR", YEAR, b"=", THISYR, 0, OK, ctx)?;
                testutil::CHCKSI(b"MONTH", MONTH, b"=", THISMO, 0, OK, ctx)?;
                testutil::CHCKSI(b"DAY", DAY, b"=", THISDY, 0, OK, ctx)?;
            }
        }
    }

    testutil::TCASE(
        b"Perform a large set of cases from Gregorian to Julian and back. ",
        ctx,
    )?;

    for I in 1604..=1604 {
        YEAR = I;
        LEAPS = intrinsics::MAX0(&[((((YEAR / 4) * 4) - YEAR) + 1), 0]);

        for J in 1..=12 {
            MONTH = J;

            for K in 1..=31 {
                if (MONTH == 2) {
                    DAY = intrinsics::MIN0(&[(28 + LEAPS), K]);
                } else if (MONTH == 4) {
                    DAY = intrinsics::MIN0(&[30, K]);
                } else if (MONTH == 6) {
                    DAY = intrinsics::MIN0(&[30, K]);
                } else if (MONTH == 9) {
                    DAY = intrinsics::MIN0(&[30, K]);
                } else if (MONTH == 11) {
                    DAY = intrinsics::MIN0(&[30, K]);
                } else {
                    DAY = K;
                }

                THISYR = YEAR;
                THISMO = MONTH;
                THISDY = DAY;

                spicelib::GR2JUL(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;
                spicelib::JUL2GR(&mut YEAR, &mut MONTH, &mut DAY, &mut DOY, ctx)?;

                testutil::CHCKXC(false, b" ", OK, ctx)?;
                testutil::CHCKSI(b"YEAR", YEAR, b"=", THISYR, 0, OK, ctx)?;
                testutil::CHCKSI(b"MONTH", MONTH, b"=", THISMO, 0, OK, ctx)?;
                testutil::CHCKSI(b"DAY", DAY, b"=", THISDY, 0, OK, ctx)?;
            }
        }
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
