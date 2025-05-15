//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      F_KPSOLV (Family of tests for KPSOLV )
pub fn F_KPSOLV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut EVEC = StackArray::<f64, 2>::new(1..=2);
    let mut FX: f64 = 0.0;
    let mut H: f64 = 0.0;
    let mut K: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut THETA: f64 = 0.0;
    let mut X: f64 = 0.0;

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
    testutil::TOPEN(b"F_KPSOLV", ctx)?;

    testutil::TCASE(b"Determine how well the solution fits h,k whose polar coordinates are r = {0, .05, .1, ... .9} and theta = { 0, .1 ... pi*2 } ", ctx)?;

    R = 0.0;

    for I in 1..=20 {
        THETA = 0.0;

        for J in 1..=63 {
            testutil::TSTMSG(b"#", b"Failure for R = # and THETA = #", ctx);
            testutil::TSTMSF(R, ctx);
            testutil::TSTMSF(THETA, ctx);

            H = (R * f64::cos(THETA));
            K = (R * f64::sin(THETA));
            EVEC[1] = H;
            EVEC[2] = K;

            X = spicelib::KPSOLV(EVEC.as_slice(), ctx)?;
            FX = ((H * f64::cos(X)) + (K * f64::sin(X)));

            testutil::CHCKXC(false, b" ", OK, ctx)?;
            testutil::CHCKSD(b"FX", FX, b"~", X, 0.000000000000001, OK, ctx)?;

            THETA = (THETA + 0.1);
        }

        R = (R + 0.05);
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
