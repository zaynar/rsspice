//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MSGLEN: i32 = 240;
const TIGHT: f64 = 0.000000000001;

//$Procedure F_STMP03 ( STMP03 tests )
pub fn F_STMP03(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut C0: f64 = 0.0;
    let mut C1: f64 = 0.0;
    let mut C2: f64 = 0.0;
    let mut C3: f64 = 0.0;
    let mut LBOUND: f64 = 0.0;
    let mut UBOUND: f64 = 0.0;
    let mut X: f64 = 0.0;
    let mut XC0: f64 = 0.0;
    let mut XC1: f64 = 0.0;
    let mut XC2: f64 = 0.0;
    let mut XC3: f64 = 0.0;
    let mut Z: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_STMP03", ctx)?;

    //**********************************************************************
    //
    //     Error cases
    //
    //**********************************************************************

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Argument too large.", ctx)?;

    //
    // LBOUND is an approximate lower bound on the input
    // values for which the Stumpff functions can be computed.
    //
    LBOUND = -f64::powi((f64::ln(2.0) + f64::ln(spicelib::DPMAX())), 2);

    X = ((2 as f64) * LBOUND);

    spicelib::STMP03(X, &mut C0, &mut C1, &mut C2, &mut C3, ctx)?;

    testutil::CHCKXC(true, b"SPICE(VALUEOUTOFRANGE)", OK, ctx)?;

    //**********************************************************************
    //
    //     Normal cases
    //
    //**********************************************************************

    //
    // Test STMP03 for values of X less than -1.
    //
    X = (-1.0 - 0.000000000000001);

    while (X > LBOUND) {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"X = #");
        spicelib::REPMD(&TITLE.clone(), b"#", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&TITLE, ctx)?;

        spicelib::STMP03(X, &mut C0, &mut C1, &mut C2, &mut C3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Generate expected results.
        //
        Z = f64::sqrt(-X);
        XC0 = f64::cosh(Z);
        XC1 = (f64::sinh(Z) / Z);
        XC2 = ((1.0 - XC0) / X);
        XC3 = ((1.0 - XC1) / X);

        testutil::CHCKSD(b"C0", C0, b"~/", XC0, TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"C1", C1, b"~/", XC1, TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"C2", C2, b"~/", XC2, TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"C3", C3, b"~/", XC3, TIGHT, OK, ctx)?;

        X = (X * 1.5);
    }

    //
    // Test STMP03 for values of X between -1 and 0.
    //
    X = -1.0;

    while (X < -0.000000000000001) {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"X = #");
        spicelib::REPMD(&TITLE.clone(), b"#", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&TITLE, ctx)?;

        spicelib::STMP03(X, &mut C0, &mut C1, &mut C2, &mut C3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Generate expected results.
        //
        Z = f64::sqrt(-X);
        XC0 = f64::cosh(Z);
        XC1 = (f64::sinh(Z) / Z);

        testutil::CHCKSD(b"C0", C0, b"~/", XC0, TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"C1", C1, b"~/", XC1, TIGHT, OK, ctx)?;

        if (X < -0.01) {
            XC2 = ((1.0 - XC0) / X);
            XC3 = ((1.0 - XC1) / X);

            testutil::CHCKSD(b"C2", C2, b"~/", XC2, TIGHT, OK, ctx)?;
            testutil::CHCKSD(b"C3", C3, b"~/", XC3, TIGHT, OK, ctx)?;
        } else {
            //
            // Compute the C3 and C2 series directly. Group terms to avoid
            // loss of precision.
            //
            XC3 = (1.0 - (X / (18 * 19) as f64));

            for I in 1..=7 {
                XC3 = (1.0 - ((X / ((18 - (2 * I)) * (19 - (2 * I))) as f64) * XC3));
            }

            XC3 = (XC3 / 6 as f64);

            testutil::CHCKSD(b"C3", C3, b"~/", XC3, TIGHT, OK, ctx)?;

            XC2 = (1.0 - (X / (17 * 18) as f64));

            for I in 1..=7 {
                XC2 = (1.0 - ((X / ((17 - (2 * I)) * (18 - (2 * I))) as f64) * XC2));
            }

            XC2 = (XC2 / 2 as f64);

            testutil::CHCKSD(b"C2", C2, b"~/", XC2, TIGHT, OK, ctx)?;
        }

        X = (X * 0.3);
    }

    //
    // Test STMP03 for values of X greater than 1.
    //
    UBOUND = (spicelib::DPMAX() / 10 as f64);

    X = (1.0 + 0.000000000000001);

    while (X < UBOUND) {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"X = #");
        spicelib::REPMD(&TITLE.clone(), b"#", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&TITLE, ctx)?;

        spicelib::STMP03(X, &mut C0, &mut C1, &mut C2, &mut C3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Generate expected results.
        //
        Z = f64::sqrt(X);

        XC0 = f64::cos(Z);
        XC1 = (f64::sin(Z) / Z);
        XC2 = ((1.0 - XC0) / X);
        XC3 = ((1.0 - XC1) / X);

        testutil::CHCKSD(b"C0", C0, b"~/", XC0, TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"C1", C1, b"~/", XC1, TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"C2", C2, b"~/", XC2, TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"C3", C3, b"~/", XC3, TIGHT, OK, ctx)?;

        X = (X * 7 as f64);
    }

    //
    // Test STMP03 for values of X between 0 and 1.
    //
    UBOUND = (spicelib::DPMAX() / 10 as f64);

    X = 0.000000000000001;

    while (X <= 1.0) {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"X = #");
        spicelib::REPMD(&TITLE.clone(), b"#", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::TCASE(&TITLE, ctx)?;

        spicelib::STMP03(X, &mut C0, &mut C1, &mut C2, &mut C3, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Generate expected results.
        //
        Z = f64::sqrt(X);
        XC0 = f64::cos(Z);
        XC1 = (f64::sin(Z) / Z);

        testutil::CHCKSD(b"C0", C0, b"~/", XC0, TIGHT, OK, ctx)?;
        testutil::CHCKSD(b"C1", C1, b"~/", XC1, TIGHT, OK, ctx)?;

        if (X > 0.01) {
            XC2 = ((1.0 - XC0) / X);
            XC3 = ((1.0 - XC1) / X);

            testutil::CHCKSD(b"C2", C2, b"~/", XC2, TIGHT, OK, ctx)?;
            testutil::CHCKSD(b"C3", C3, b"~/", XC3, TIGHT, OK, ctx)?;
        } else {
            //
            // Compute the C3 and C2 series directly. Group terms to avoid
            // loss of precision.
            //
            XC3 = (1.0 - (X / (18 * 19) as f64));

            for I in 1..=7 {
                XC3 = (1.0 - ((X / ((18 - (2 * I)) * (19 - (2 * I))) as f64) * XC3));
            }

            XC3 = (XC3 / 6 as f64);

            testutil::CHCKSD(b"C3", C3, b"~/", XC3, TIGHT, OK, ctx)?;

            XC2 = (1.0 - (X / (17 * 18) as f64));

            for I in 1..=7 {
                XC2 = (1.0 - ((X / ((17 - (2 * I)) * (18 - (2 * I))) as f64) * XC2));
            }

            XC2 = (XC2 / 2 as f64);

            testutil::CHCKSD(b"C2", C2, b"~/", XC2, TIGHT, OK, ctx)?;
        }

        X = (X * 3 as f64);
    }

    //
    // Test STMP03 for values of X = 0.
    //
    X = 0.0;

    //
    // --- Case: ------------------------------------------------------
    //
    fstr::assign(&mut TITLE, b"X = #");
    spicelib::REPMD(&TITLE.clone(), b"#", X, 14, &mut TITLE, ctx);
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::TCASE(&TITLE, ctx)?;

    spicelib::STMP03(X, &mut C0, &mut C1, &mut C2, &mut C3, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Generate expected results.
    //
    XC0 = 1.0;
    XC1 = 1.0;
    XC2 = 0.5;
    XC3 = (1.0 / 6 as f64);

    testutil::CHCKSD(b"C0", C0, b"~/", XC0, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"C1", C1, b"~/", XC1, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"C2", C2, b"~/", XC2, TIGHT, OK, ctx)?;
    testutil::CHCKSD(b"C3", C3, b"~/", XC3, TIGHT, OK, ctx)?;
    //
    // ---------------------------------------------------------
    //
    //
    //     Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
