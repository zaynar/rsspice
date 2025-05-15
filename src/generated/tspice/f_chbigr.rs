//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VTIGHT: f64 = 0.00000000000001;
const TIGHT: f64 = 0.000000000001;
const MEDTOL: f64 = 0.0000000001;
const MAXCOF: i32 = 100;
const MSGLEN: i32 = 320;

//$Procedure F_CHBIGR ( CHBIGR tests )
pub fn F_CHBIGR(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TITLE = [b' '; MSGLEN as usize];
    let mut CP = StackArray::<f64, 100>::new(1..=MAXCOF);
    let mut DELTA: f64 = 0.0;
    let mut ITGRLP: f64 = 0.0;
    let mut P: f64 = 0.0;
    let mut WORK = StackArray::<f64, 202>::new(1..=(2 * (MAXCOF + 1)));
    let mut X: f64 = 0.0;
    let mut X2S = StackArray::<f64, 2>::new(1..=2);
    let mut XITGRL: f64 = 0.0;
    let mut XP: f64 = 0.0;
    let mut DEGP: i32 = 0;
    let mut NSAMP: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_CHBIGR", ctx)?;

    //
    // *****************************************************************
    //
    // Error cases
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Exception: expansion degree is negative.", ctx)?;

    spicelib::CLEARD(MAXCOF, CP.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    DEGP = -1;
    CP[1] = 0.0;
    X = 0.0;
    X2S[1] = 0.0;
    X2S[2] = 1.0;
    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDDEGREE)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Exception: interval radius is non-positive.", ctx)?;

    spicelib::CLEARD(MAXCOF, CP.as_slice_mut());
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    DEGP = 0;
    CP[1] = 0.0;
    X = 0.0;
    X2S[1] = 0.0;
    X2S[2] = 0.0;
    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDRADIUS)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases
    //
    // *****************************************************************
    //
    // The following cases use input expansions having results that
    // can be verified by inspection.
    //
    // Initialize the coefficient array with numbers that will cause
    // problems if they are incorrectly accessed.
    //
    spicelib::FILLD(spicelib::DPMAX(), MAXCOF, CP.as_slice_mut());

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial is f(x) = 6x**5; interval is [10, 30]; X = 30.",
        ctx,
    )?;

    //
    // Let our domain be the interval [10, 30].
    //
    X2S[1] = 20.0;
    X2S[2] = 10.0;

    DEGP = 5;

    CP[1] = 0.0;
    CP[2] = 3.75;
    CP[3] = 0.0;
    CP[4] = 1.875;
    CP[5] = 0.0;
    CP[6] = 0.375;

    //
    // Evaluate the function and its integral at X = 30.
    //
    X = 30.0;

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The indefinite integral is F(x) = x**6, defined on
    // the interval [-1, 1]. The result, scaled to account
    // for the change of variables, should be 10.
    //
    XITGRL = 10.0;
    XP = 6.0;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial is f(x) = 6x**5; interval is [10, 30]; X = 20.",
        ctx,
    )?;

    //
    // Let our domain be the interval [10, 30].
    //
    X2S[1] = 20.0;
    X2S[2] = 10.0;

    DEGP = 5;

    CP[1] = 0.0;
    CP[2] = 3.75;
    CP[3] = 0.0;
    CP[4] = 1.875;
    CP[5] = 0.0;
    CP[6] = 0.375;

    //
    // Evaluate the function and its integral at X = 20.
    //
    X = 20.0;

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // The indefinite integral is F(x) = x**6, defined on
    // the interval [-1, 1]. The result should be 0.
    //
    XITGRL = 0.0;
    XP = 0.0;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // The following cases use T_CHBIGR to produce expected results.
    //
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 0; radius is 10000, X = midpoint + radius/2.",
        ctx,
    )?;

    DEGP = 0;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    CP[1] = 3.0;

    X = (X2S[1] + (X2S[2] / 2 as f64));

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Continue previous degree 0 case. Sample along domain interval.",
        ctx,
    )?;

    NSAMP = 101;
    DELTA = (((2 as f64) * X2S[2]) / (NSAMP - 1) as f64);

    for J in 1..=NSAMP {
        X = ((X2S[1] - X2S[2]) + (((J - 1) as f64) * DELTA));

        spicelib::CHBIGR(
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            &mut P,
            &mut ITGRLP,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_CHBIGR(
            1,
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            WORK.as_slice_mut(),
            std::slice::from_mut(&mut XP),
            std::slice::from_mut(&mut XITGRL),
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut TITLE, b"P at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, P, b"~", XP, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, P, b"~/", XP, TIGHT, OK, ctx)?;
        }

        fstr::assign(&mut TITLE, b"ITGRLP at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, ITGRLP, b"~", XITGRL, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, ITGRLP, b"~/", XITGRL, TIGHT, OK, ctx)?;
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 1; radius is 10000, X = midpoint + radius/2.",
        ctx,
    )?;

    DEGP = 1;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    CP[1] = 3.0;
    CP[2] = 5.0;

    X = (X2S[1] + (X2S[2] / 2 as f64));

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Continue previous degree 1 case. Sample along domain interval.",
        ctx,
    )?;

    NSAMP = 101;
    DELTA = (((2 as f64) * X2S[2]) / (NSAMP - 1) as f64);

    for J in 1..=NSAMP {
        X = ((X2S[1] - X2S[2]) + (((J - 1) as f64) * DELTA));

        spicelib::CHBIGR(
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            &mut P,
            &mut ITGRLP,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_CHBIGR(
            1,
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            WORK.as_slice_mut(),
            std::slice::from_mut(&mut XP),
            std::slice::from_mut(&mut XITGRL),
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut TITLE, b"P at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Note that in this case, the function passes through 0
        // at X = 94000.
        //
        if (J == 21) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, P, b"~", XP, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, P, b"~/", XP, TIGHT, OK, ctx)?;
        }

        fstr::assign(&mut TITLE, b"ITGRLP at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, ITGRLP, b"~", XITGRL, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, ITGRLP, b"~/", XITGRL, TIGHT, OK, ctx)?;
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 2; radius is 10000, X = midpoint + radius/2.",
        ctx,
    )?;

    DEGP = 2;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    CP[1] = 3.0;
    CP[2] = 5.0;
    CP[3] = 7.0;

    X = (X2S[1] + (X2S[2] / 2 as f64));

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Continue previous degree 2 case. Sample along domain interval.",
        ctx,
    )?;

    NSAMP = 101;
    DELTA = (((2 as f64) * X2S[2]) / (NSAMP - 1) as f64);

    for J in 1..=NSAMP {
        X = ((X2S[1] - X2S[2]) + (((J - 1) as f64) * DELTA));

        spicelib::CHBIGR(
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            &mut P,
            &mut ITGRLP,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_CHBIGR(
            1,
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            WORK.as_slice_mut(),
            std::slice::from_mut(&mut XP),
            std::slice::from_mut(&mut XITGRL),
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut TITLE, b"P at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, P, b"~", XP, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, P, b"~/", XP, MEDTOL, OK, ctx)?;
        }

        fstr::assign(&mut TITLE, b"ITGRLP at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, ITGRLP, b"~", XITGRL, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, ITGRLP, b"~/", XITGRL, MEDTOL, OK, ctx)?;
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 3; radius is 10000, X = midpoint + radius/2.",
        ctx,
    )?;

    DEGP = 3;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    CP[1] = 3.0;
    CP[2] = 5.0;
    CP[3] = 7.0;
    CP[4] = 9.0;

    X = (X2S[1] + (X2S[2] / 2 as f64));

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Continue previous degree 3 case. Sample along domain interval.",
        ctx,
    )?;

    NSAMP = 101;
    DELTA = (((2 as f64) * X2S[2]) / (NSAMP - 1) as f64);

    for J in 1..=NSAMP {
        X = ((X2S[1] - X2S[2]) + (((J - 1) as f64) * DELTA));

        spicelib::CHBIGR(
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            &mut P,
            &mut ITGRLP,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_CHBIGR(
            1,
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            WORK.as_slice_mut(),
            std::slice::from_mut(&mut XP),
            std::slice::from_mut(&mut XITGRL),
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut TITLE, b"P at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, P, b"~", XP, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, P, b"~/", XP, TIGHT, OK, ctx)?;
        }

        fstr::assign(&mut TITLE, b"ITGRLP at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, ITGRLP, b"~", XITGRL, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, ITGRLP, b"~/", XITGRL, MEDTOL, OK, ctx)?;
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 4; radius is 10000, X = midpoint - radius/2.",
        ctx,
    )?;

    DEGP = 4;

    //
    // Use a negative value for the midpoint this time.
    //
    X2S[1] = -100000.0;
    X2S[2] = 10000.0;

    CP[1] = 3.0;
    CP[2] = 5.0;
    CP[3] = 7.0;
    CP[4] = 9.0;
    CP[5] = 11.0;

    X = (X2S[1] - (X2S[2] / 2 as f64));

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Continue previous degree 4 case. Sample along domain interval.",
        ctx,
    )?;

    NSAMP = 101;
    DELTA = (((2 as f64) * X2S[2]) / (NSAMP - 1) as f64);

    for J in 1..=NSAMP {
        X = ((X2S[1] - X2S[2]) + (((J - 1) as f64) * DELTA));

        spicelib::CHBIGR(
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            &mut P,
            &mut ITGRLP,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_CHBIGR(
            1,
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            WORK.as_slice_mut(),
            std::slice::from_mut(&mut XP),
            std::slice::from_mut(&mut XITGRL),
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut TITLE, b"P at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, P, b"~", XP, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, P, b"~/", XP, MEDTOL, OK, ctx)?;
        }

        fstr::assign(&mut TITLE, b"ITGRLP at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, ITGRLP, b"~", XITGRL, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, ITGRLP, b"~/", XITGRL, MEDTOL, OK, ctx)?;
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 5; radius is 10000, X = midpoint - radius/2.",
        ctx,
    )?;

    DEGP = 5;

    //
    // Use a negative value for the midpoint this time.
    //
    X2S[1] = -100000.0;
    X2S[2] = 10000.0;

    CP[1] = 3.0;
    CP[2] = 5.0;
    CP[3] = 7.0;
    CP[4] = 9.0;
    CP[5] = 11.0;
    CP[5] = -13.0;

    X = (X2S[1] - (X2S[2] / 2 as f64));

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Continue previous degree 5 case. Sample along domain interval.",
        ctx,
    )?;

    NSAMP = 101;
    DELTA = (((2 as f64) * X2S[2]) / (NSAMP - 1) as f64);

    for J in 1..=NSAMP {
        X = ((X2S[1] - X2S[2]) + (((J - 1) as f64) * DELTA));

        spicelib::CHBIGR(
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            &mut P,
            &mut ITGRLP,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_CHBIGR(
            1,
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            WORK.as_slice_mut(),
            std::slice::from_mut(&mut XP),
            std::slice::from_mut(&mut XITGRL),
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut TITLE, b"P at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, P, b"~", XP, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, P, b"~/", XP, MEDTOL, OK, ctx)?;
        }

        fstr::assign(&mut TITLE, b"ITGRLP at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, ITGRLP, b"~", XITGRL, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, ITGRLP, b"~/", XITGRL, MEDTOL, OK, ctx)?;
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 15; radius is 10000, X = midpoint + radius/2.",
        ctx,
    )?;

    DEGP = 15;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    X = (X2S[1] + (X2S[2] / 2 as f64));

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 15; radius is 10000, X = midpoint.",
        ctx,
    )?;

    DEGP = 15;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    X = X2S[1];

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use absolute error tests since results should be close to 0.
    //
    testutil::CHCKSD(b"P", P, b"~", XP, MEDTOL, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~", XITGRL, MEDTOL, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 30; radius is 10000, X = midpoint + radius/2.",
        ctx,
    )?;

    DEGP = 30;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    X = (X2S[1] + (X2S[2] / 2 as f64));

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 60; radius is 10000, X = midpoint + radius/2.",
        ctx,
    )?;

    DEGP = 60;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    X = (X2S[1] + (X2S[2] / 2 as f64));

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 98; radius is 10000, X = midpoint + radius/2.",
        ctx,
    )?;

    DEGP = 98;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    X = (X2S[1] + (X2S[2] / 2 as f64));

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 98; radius is 10000, X = midpoint + radius.",
        ctx,
    )?;

    DEGP = 98;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    //
    // Note the abscissa is at the right endpoint of the domain.
    //
    X = (X2S[1] + X2S[2]);

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 98; radius is 10000, X = midpoint - radius.",
        ctx,
    )?;

    DEGP = 98;

    X2S[1] = 100000.0;
    X2S[2] = 10000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    //
    // Note the abscissa is at the left endpoint of the domain.
    //
    X = (X2S[1] - X2S[2]);

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 98; radius is 1000000, X = midpoint + radius.",
        ctx,
    )?;

    DEGP = 98;

    X2S[1] = 1000000.0;
    X2S[2] = 1000000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    //
    // Note the abscissa is at the right endpoint of the domain.
    //
    X = (X2S[1] + X2S[2]);

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 98; radius is 1000000, X = midpoint - radius.",
        ctx,
    )?;

    DEGP = 98;

    X2S[1] = 1000000.0;
    X2S[2] = 1000000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    //
    // Note the abscissa is at the left endpoint of the domain.
    //
    X = (X2S[1] - X2S[2]);

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"P", P, b"~/", XP, VTIGHT, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~/", XITGRL, VTIGHT, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 98; radius is 1000000, X = midpoint.",
        ctx,
    )?;

    DEGP = 98;

    X2S[1] = 1000000.0;
    X2S[2] = 1000000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    //
    // Note the abscissa is at the left endpoint of the domain.
    //
    X = X2S[1];

    spicelib::CHBIGR(
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        &mut P,
        &mut ITGRLP,
        ctx,
    )?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::T_CHBIGR(
        1,
        DEGP,
        CP.as_slice(),
        X2S.as_slice(),
        X,
        WORK.as_slice_mut(),
        std::slice::from_mut(&mut XP),
        std::slice::from_mut(&mut XITGRL),
        ctx,
    );
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Use absolute error tests since results should be close to 0.
    //
    testutil::CHCKSD(b"P", P, b"~", XP, MEDTOL, OK, ctx)?;
    testutil::CHCKSD(b"ITGRLP", ITGRLP, b"~", XITGRL, MEDTOL, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 15; radius is 1000000, sample results along the domain.",
        ctx,
    )?;

    DEGP = 15;

    X2S[1] = 1000000.0;
    X2S[2] = 1000000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    NSAMP = 10001;
    DELTA = (((2 as f64) * X2S[2]) / (NSAMP - 1) as f64);

    for J in 1..=NSAMP {
        X = ((X2S[1] - X2S[2]) + (((J - 1) as f64) * DELTA));

        spicelib::CHBIGR(
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            &mut P,
            &mut ITGRLP,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_CHBIGR(
            1,
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            WORK.as_slice_mut(),
            std::slice::from_mut(&mut XP),
            std::slice::from_mut(&mut XITGRL),
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut TITLE, b"P at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, P, b"~", XP, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, P, b"~/", XP, MEDTOL, OK, ctx)?;
        }

        fstr::assign(&mut TITLE, b"ITGRLP at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, ITGRLP, b"~", XITGRL, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, ITGRLP, b"~/", XITGRL, MEDTOL, OK, ctx)?;
        }
    }

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Polynomial degree is 98; radius is 1000000, Sample results along the domain.",
        ctx,
    )?;

    DEGP = 98;

    X2S[1] = 1000000.0;
    X2S[2] = 1000000.0;

    for I in 1..=(DEGP + 1) {
        CP[I] = (((DEGP / 2) - I) as f64);
    }

    NSAMP = 10001;
    DELTA = (((2 as f64) * X2S[2]) / (NSAMP - 1) as f64);

    for J in 1..=NSAMP {
        X = ((X2S[1] - X2S[2]) + (((J - 1) as f64) * DELTA));

        spicelib::CHBIGR(
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            &mut P,
            &mut ITGRLP,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_CHBIGR(
            1,
            DEGP,
            CP.as_slice(),
            X2S.as_slice(),
            X,
            WORK.as_slice_mut(),
            std::slice::from_mut(&mut XP),
            std::slice::from_mut(&mut XITGRL),
            ctx,
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        fstr::assign(&mut TITLE, b"P at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, P, b"~", XP, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, P, b"~/", XP, MEDTOL, OK, ctx)?;
        }

        fstr::assign(&mut TITLE, b"ITGRLP at @");
        spicelib::REPMD(&TITLE.clone(), b"@", X, 14, &mut TITLE, ctx);
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (J == ((NSAMP / 2) + 1)) {
            //
            // Use absolute error tests since results should be close to
            // 0.
            //
            testutil::CHCKSD(&TITLE, ITGRLP, b"~", XITGRL, MEDTOL, OK, ctx)?;
        } else {
            testutil::CHCKSD(&TITLE, ITGRLP, b"~/", XITGRL, MEDTOL, OK, ctx)?;
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
