//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      F_DPSTRF (Family of tests for dpstrf )
pub fn F_DPSTRF(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ESTRNG = [b' '; LNSIZE as usize];
    let mut STRING = [b' '; LNSIZE as usize];
    let mut SIGDIG: i32 = 0;
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
    testutil::TOPEN(b"F_DPSTRF", ctx)?;

    testutil::TCASE(b"Test Rounding X =  1.28372D+8, SIGDIG = 2", ctx)?;

    X = 128372000.0;
    SIGDIG = 2;
    fstr::assign(&mut ESTRNG, b" 130000000.");

    spicelib::DPSTRF(X, SIGDIG, b"F", &mut STRING, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"Test Rounding X = -1.28372D+8, SIGDIG = 2", ctx)?;

    X = -128372000.0;
    SIGDIG = 2;
    fstr::assign(&mut ESTRNG, b"-130000000.");

    spicelib::DPSTRF(X, SIGDIG, b"F", &mut STRING, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"Test Rounding X =  9.9995 , SIGDIG = 4", ctx)?;

    X = 9.9995;
    SIGDIG = 4;
    fstr::assign(&mut ESTRNG, b" 10.00");

    spicelib::DPSTRF(X, SIGDIG, b"F", &mut STRING, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"Test Rounding X =  9.9995 , SIGDIG = 5", ctx)?;

    X = 9.9995;
    SIGDIG = 5;
    fstr::assign(&mut ESTRNG, b" 9.9995");

    spicelib::DPSTRF(X, SIGDIG, b"F", &mut STRING, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"Test small values X = 1.2829D-21, SIGDIG = 5", ctx)?;

    X = 0.0000000000000000000012829;
    SIGDIG = 5;
    fstr::assign(&mut ESTRNG, b" 0.0000000000000000000012829");

    spicelib::DPSTRF(X, SIGDIG, b"F", &mut STRING, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"Test large values X = 9.92829D+35 SIGDIG = 3", ctx)?;

    X = 992829000000000000000000000000000000.0;
    SIGDIG = 3;
    fstr::assign(&mut ESTRNG, b" 993000000000000000000000000000000000.");

    spicelib::DPSTRF(X, SIGDIG, b"F", &mut STRING, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"Test large values X = 9.96829D+35 SIGDIG = 2", ctx)?;

    X = 996829000000000000000000000000000000.0;
    SIGDIG = 2;
    fstr::assign(&mut ESTRNG, b" 1000000000000000000000000000000000000.");

    spicelib::DPSTRF(X, SIGDIG, b"F", &mut STRING, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"Intermediate value of X with non-zero integer and fractional parts.  X = 123.18272 SIGDIG = 14 ", ctx)?;

    X = 123.18272;
    SIGDIG = 14;

    fstr::assign(&mut ESTRNG, b" 123.18272000000");

    spicelib::DPSTRF(X, SIGDIG, b"F", &mut STRING, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"Intermediate value of X with non-zero integer and fractional parts.  X = -123.18272 SIGDIG = 14 ", ctx)?;

    X = -123.18272;
    SIGDIG = 14;

    fstr::assign(&mut ESTRNG, b"-123.18272000000");

    spicelib::DPSTRF(X, SIGDIG, b"F", &mut STRING, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"Zero case X = 0, SIGDIG = 7 ", ctx)?;

    X = 0.0;
    SIGDIG = 7;
    fstr::assign(&mut ESTRNG, b" 0.0000000");

    spicelib::DPSTRF(X, SIGDIG, b"F", &mut STRING, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
