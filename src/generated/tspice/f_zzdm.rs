//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure F_ZZDM ( Test ZZDIV, ZZMULT )
pub fn F_ZZDM(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut A: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut DENOM: f64 = 0.0;
    let mut DIV: f64 = 0.0;
    let mut LOGVAL: f64 = 0.0;
    let mut MULT: f64 = 0.0;
    let mut MULTX: f64 = 0.0;
    let mut NUMR: f64 = 0.0;
    let mut EXPNT: f64 = 0.0;
    let mut SEED: i32 = 0;
    let mut CONT: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_ZZDM", ctx)?;

    //
    // Initialize the random number seed.
    //
    SEED = -6273618;

    //
    // Initialize the exponent value.
    //
    EXPNT = (((f64::log10(spicelib::DPMAX()) as i32) as f64) - 1.0);

    //
    // Case 1
    //
    testutil::TCASE(b"Safe division check.", ctx)?;

    //
    // Standard, safe evaluation.
    //
    NUMR = 1.0;
    DENOM = 10.0;

    DIV = spicelib::ZZDIV(NUMR, DENOM, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ZZDIV 1/10", DIV, b"~", 0.1, 0.000000000001, OK, ctx)?;

    //
    // Case 2
    //
    testutil::TCASE(b"Underflow division check.", ctx)?;

    //
    // A numeric underflow event as defined in ZZDIV. No error,
    // return zero.
    //
    NUMR = 0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;
    DENOM = 100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

    DIV = spicelib::ZZDIV(NUMR, DENOM, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ZZDIV 1D-400", DIV, b"=", 0.0, 0.0, OK, ctx)?;

    //
    // Case 3
    //
    testutil::TCASE(b"Overflow division check.", ctx)?;

    //
    // A numeric overflow event as defined in ZZDIV.
    //
    NUMR = spicelib::DPMAX();
    DENOM = 1.0;

    DIV = spicelib::ZZDIV(NUMR, DENOM, ctx)?;
    testutil::CHCKXC(true, b"SPICE(NUMERICOVERFLOW)", OK, ctx)?;

    testutil::CHCKSD(b"ZZDIV DPMAX/1", DIV, b"=", 0.0, 0.0, OK, ctx)?;

    //
    // Case 4
    //
    testutil::TCASE(b"1/0 division check.", ctx)?;

    //
    // A divide by zero event.
    //
    NUMR = 1.0;
    DENOM = 0.0;

    DIV = spicelib::ZZDIV(NUMR, DENOM, ctx)?;
    testutil::CHCKXC(true, b"SPICE(DIVIDEBYZERO)", OK, ctx)?;

    testutil::CHCKSD(b"ZZDIV 1/0", DIV, b"=", 0.0, 0.0, OK, ctx)?;

    //
    // Case 5
    //
    testutil::TCASE(b"0/0 division check.", ctx)?;

    //
    // A 0/0 event. ZZDIV treats this as a divide by zero
    // event.
    //
    NUMR = 0.0;
    DENOM = 0.0;

    DIV = spicelib::ZZDIV(NUMR, DENOM, ctx)?;
    testutil::CHCKXC(true, b"SPICE(DIVIDEBYZERO)", OK, ctx)?;

    testutil::CHCKSD(b"ZZDIV 0/0", DIV, b"=", 0.0, 0.0, OK, ctx)?;

    //
    // Case 6
    //
    testutil::TCASE(b"Safe multiplication check.", ctx)?;

    //
    // Standard, safe evaluation.
    //
    A = 2.0;
    B = 10.0;

    MULT = spicelib::ZZMULT(A, B, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ZZMULT 2*10", MULT, b"~", 20.0, 0.000000000001, OK, ctx)?;

    //
    // Case 7
    //
    testutil::TCASE(b"A numeric overflow event (1).", ctx)?;

    A = 1.0;
    B = spicelib::DPMAX();

    MULT = spicelib::ZZMULT(A, B, ctx)?;
    testutil::CHCKXC(true, b"SPICE(NUMERICOVERFLOW)", OK, ctx)?;

    testutil::CHCKSD(
        b"ZZMULT overflow (1)",
        MULT,
        b"=",
        0.0,
        0.000000000001,
        OK,
        ctx,
    )?;

    //
    // Case 8
    //
    testutil::TCASE(b"A numeric overflow event (2).", ctx)?;

    A = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    B = 10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

    MULT = spicelib::ZZMULT(A, B, ctx)?;
    testutil::CHCKXC(true, b"SPICE(NUMERICOVERFLOW)", OK, ctx)?;

    testutil::CHCKSD(
        b"ZZMULT overflow (2)",
        MULT,
        b"=",
        0.0,
        0.000000000001,
        OK,
        ctx,
    )?;

    //
    // Case 9
    //
    testutil::TCASE(b"Zero multiplication check (1).", ctx)?;

    A = 0.0;
    B = 10.0;

    MULT = spicelib::ZZMULT(A, B, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ZZMULT zero (1)", MULT, b"=", 0.0, 0.000000000001, OK, ctx)?;

    //
    // Case 10
    //
    testutil::TCASE(b"Zero multiplication check (2).", ctx)?;

    A = 10.0;
    B = 0.0;

    MULT = spicelib::ZZMULT(A, B, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ZZMULT zero (2)", MULT, b"=", 0.0, 0.000000000001, OK, ctx)?;

    //
    // Case 11
    //
    testutil::TCASE(b"Zero multiplication check (3).", ctx)?;

    A = 0.0;
    B = 0.0;

    MULT = spicelib::ZZMULT(A, B, ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    testutil::CHCKSD(b"ZZMULT zero (3)", MULT, b"=", 0.0, 0.000000000001, OK, ctx)?;

    //
    // Note, the use of 5000 iterations is an arbitrary choice.
    //

    //
    // Case 12
    //
    testutil::TCASE(b"Random multiplication check.", ctx)?;

    for I in 1..=5000 {
        A = f64::powf(10.0, testutil::T_RANDD(-EXPNT, EXPNT, &mut SEED, ctx)?);
        B = f64::powf(10.0, testutil::T_RANDD(-EXPNT, EXPNT, &mut SEED, ctx)?);

        CONT = false;
        LOGVAL = (f64::log10(f64::abs(A)) + f64::log10(f64::abs(B)));

        //
        // Pass over the calculation if the log10 of the calculated is not
        // within (-EXPNT, EXPNT).
        //
        CONT = ((LOGVAL < EXPNT) && (LOGVAL > -EXPNT));

        if CONT {
            MULT = spicelib::ZZMULT(A, B, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            MULTX = (A * B);

            testutil::CHCKSD(
                b"Random multiplication",
                MULT,
                b"=",
                MULTX,
                0.000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // Case 13
    //
    testutil::TCASE(b"Random division check.", ctx)?;

    for I in 1..=5000 {
        NUMR = f64::powf(10.0, testutil::T_RANDD(-EXPNT, EXPNT, &mut SEED, ctx)?);
        DENOM = f64::powf(10.0, testutil::T_RANDD(-EXPNT, EXPNT, &mut SEED, ctx)?);

        CONT = false;
        LOGVAL = (f64::log10(f64::abs(NUMR)) - f64::log10(f64::abs(DENOM)));

        //
        // Pass over the calculation if the log10 of the calculated is not
        // within (-EXPNT, EXPNT) or if the denominator is zero.
        //
        CONT = (((LOGVAL < EXPNT) && (LOGVAL > -EXPNT)) && (DENOM != 0.0));

        if CONT {
            DIV = spicelib::ZZDIV(NUMR, DENOM, ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            MULTX = (NUMR / DENOM);

            testutil::CHCKSD(
                b"Random division",
                DIV,
                b"=",
                MULTX,
                0.000000000001,
                OK,
                ctx,
            )?;
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
