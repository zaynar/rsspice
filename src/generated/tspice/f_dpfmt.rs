//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 300;
const WDSIZE: i32 = 32;

//$Procedure      F_DPFMT ( Family of tests for DPFMT)
pub fn F_DPFMT(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut STRING = [b' '; LNSIZE as usize];
    let mut ESTRNG = [b' '; LNSIZE as usize];
    let mut FMT = [b' '; LNSIZE as usize];
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
    testutil::TOPEN(b"F_DPFMT", ctx)?;

    X = 123400.0;

    testutil::TCASE(b"Exception PICTUR = \' \' ", ctx)?;

    fstr::assign(&mut FMT, b" ");
    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;
    testutil::CHCKXC(true, b"SPICE(NOPICTURE)", OK, ctx)?;

    testutil::TCASE(b"Exception PICTUR = \'+\' ", ctx)?;

    fstr::assign(&mut FMT, b"+");
    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADPICTURE)", OK, ctx)?;

    testutil::TCASE(b"Exception PICTUR = \'-\' ", ctx)?;

    fstr::assign(&mut FMT, b"-");
    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADPICTURE)", OK, ctx)?;

    testutil::TCASE(b"Exception PICTUR = \'.\' ", ctx)?;

    fstr::assign(&mut FMT, b".");
    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADPICTURE)", OK, ctx)?;

    testutil::TCASE(b"Exception PICTUR = \'+.\' ", ctx)?;

    fstr::assign(&mut FMT, b"+.");
    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADPICTURE)", OK, ctx)?;

    testutil::TCASE(b"Exception PICTUR = \'-.\' ", ctx)?;

    fstr::assign(&mut FMT, b"-.");
    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;
    testutil::CHCKXC(true, b"SPICE(BADPICTURE)", OK, ctx)?;

    testutil::TCASE(b"Exception Long picture/short output string", ctx)?;

    fstr::assign(&mut FMT, b"xxxx.xxxxxxxxx");
    spicelib::DPFMT(X, &FMT, fstr::substr_mut(&mut STRING, 1..=8), ctx)?;
    testutil::CHCKXC(true, b"SPICE(OUTPUTTOOSHORT)", OK, ctx)?;

    testutil::TCASE(
        b"X = 1.23456789D-37, fmt = \'xxxx.xxxxxxxxxxxxxxxxxxxxxxxxxxxxx\' ",
        ctx,
    )?;

    X = 0.000000000000000000000000000000000000123456789;
    fstr::assign(&mut FMT, b"xxxx.xxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    fstr::assign(&mut ESTRNG, b"   0.00000000000000000000000000000");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 1.23456789D+32, fmt = xxxx.xxx", ctx)?;

    X = 123456789000000000000000000000000.0;
    fstr::assign(&mut FMT, b"xxxx.xxx");
    fstr::assign(&mut ESTRNG, b"1.23E+32");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 1.28289392D+7 , FMT = +xxxxxxxx.xxx ", ctx)?;

    X = 12828939.2;
    fstr::assign(&mut FMT, b"+xxxxxxxxx.xxx");
    fstr::assign(&mut ESTRNG, b"+ 12828939.200");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 182.938  fmt = xxx", ctx)?;

    X = 182.938;
    fstr::assign(&mut FMT, b"xxx");
    fstr::assign(&mut ESTRNG, b"183");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 182.938  fmt = xx", ctx)?;

    X = 182.938;
    fstr::assign(&mut FMT, b"xx");
    fstr::assign(&mut ESTRNG, b"**");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = -182.938,  fmt = xx.xxxxxx", ctx)?;

    X = -182.938;
    fstr::assign(&mut FMT, b"xx.xxxxxx");
    fstr::assign(&mut ESTRNG, b"-1.83E+02");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 8.50D+0, fmt = 0x", ctx)?;

    X = 8.5;
    fstr::assign(&mut FMT, b"0x");
    fstr::assign(&mut ESTRNG, b"09");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b" X = -80.5D+0, fmt = xx ", ctx)?;

    X = -80.5;
    fstr::assign(&mut FMT, b"xx");
    fstr::assign(&mut ESTRNG, b"**");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b" X = 80.5D+0, fmt = xx ", ctx)?;

    X = 80.5;
    fstr::assign(&mut FMT, b"xx");
    fstr::assign(&mut ESTRNG, b"81");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b" X = -80.5D+0, fmt = xxx ", ctx)?;

    X = -80.5;
    fstr::assign(&mut FMT, b"xxx");
    fstr::assign(&mut ESTRNG, b"-81");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b" X =  80.5D+0, fmt = -xx ", ctx)?;

    X = 80.5;
    fstr::assign(&mut FMT, b"xxx");
    fstr::assign(&mut ESTRNG, b" 81");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b" X = 9.99,   fmt = x.x", ctx)?;

    X = 9.99;
    fstr::assign(&mut FMT, b"x.x");
    fstr::assign(&mut ESTRNG, b"***");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b" X = 9.99,   fmt = x.xx", ctx)?;

    X = 9.99;
    fstr::assign(&mut FMT, b"x.xx");
    fstr::assign(&mut ESTRNG, b"9.99");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 99.9999999  fmt = xx.xxxxxx", ctx)?;

    X = 99.9999999;
    fstr::assign(&mut FMT, b"xx.xxxxxx");
    fstr::assign(&mut ESTRNG, b"1.000E+02");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 99.9999999  fmt = +xx.xxxxxx", ctx)?;

    X = 99.9999999;
    fstr::assign(&mut FMT, b"+xx.xxxxxx");
    fstr::assign(&mut ESTRNG, b"+1.000E+02");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 99.9999999  fmt = -xx.xxxxxx", ctx)?;

    X = 99.9999999;
    fstr::assign(&mut FMT, b"-xx.xxxxxx");
    fstr::assign(&mut ESTRNG, b" 1.000E+02");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 0.28910, fmt = +.xxxx", ctx)?;

    X = 0.2891;
    fstr::assign(&mut FMT, b"+.xxxx");
    fstr::assign(&mut ESTRNG, b"+.2891");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = -0.28910, fmt = x.xxx ", ctx)?;

    X = -0.2891;
    fstr::assign(&mut FMT, b"x.xxxx");
    fstr::assign(&mut ESTRNG, b"-.2891");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 0.0D0, fmt = +x.xxxx ", ctx)?;

    X = 0.0;
    fstr::assign(&mut FMT, b"+x.xxxx");
    fstr::assign(&mut ESTRNG, b" 0.0000");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 0.0D0, fmt = .xxxx ", ctx)?;

    X = 0.0;
    fstr::assign(&mut FMT, b".xxxx");
    fstr::assign(&mut ESTRNG, b".0000");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b" X = -1.000D0, fmt = xxxx.xxx ", ctx)?;

    X = -1.0;
    fstr::assign(&mut FMT, b"xxxx.xxx");
    fstr::assign(&mut ESTRNG, b"  -1.000");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b" X = -1.000D0, fmt = -xxxx.xxx ", ctx)?;

    X = -1.0;
    fstr::assign(&mut FMT, b"-xxxx.xxx");
    fstr::assign(&mut ESTRNG, b"-   1.000");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = 123.456  fmt = xxxxxx.xxx", ctx)?;

    X = 123.456;
    fstr::assign(&mut FMT, b"xxxxxx.xxx");
    fstr::assign(&mut ESTRNG, b"   123.456");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = -123.456  fmt = xxxxxx.xxx", ctx)?;

    X = -123.456;
    fstr::assign(&mut FMT, b"xxxxxx.xxx");
    fstr::assign(&mut ESTRNG, b"  -123.456");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X = -123.456  fmt = 0xxxxx.xxx", ctx)?;

    X = -123.456;
    fstr::assign(&mut FMT, b"0xxxxx.xxx");
    fstr::assign(&mut ESTRNG, b"-00123.456");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X =  123.456  fmt = 0xxxxx.xxx", ctx)?;

    X = 123.456;
    fstr::assign(&mut FMT, b"0xxxxx.xxx");
    fstr::assign(&mut ESTRNG, b"000123.456");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X =  123.456  fmt = +0xxxx.xxx", ctx)?;

    X = 123.456;
    fstr::assign(&mut FMT, b"+0xxxx.xxx");
    fstr::assign(&mut ESTRNG, b"+00123.456");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X =  123.456  fmt = -0xxxx.xxx", ctx)?;

    X = 123.456;
    fstr::assign(&mut FMT, b"-0xxxx.xxx");
    fstr::assign(&mut ESTRNG, b" 00123.456");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X =  123.456  fmt = -xxxxx.xxx", ctx)?;

    X = 123.456;
    fstr::assign(&mut FMT, b"-xxxxx.xxx");
    fstr::assign(&mut ESTRNG, b"   123.456");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::TCASE(b"X =  123.456  fmt = +xxxxx.xxx", ctx)?;

    X = 123.456;
    fstr::assign(&mut FMT, b"+xxxxx.xxx");
    fstr::assign(&mut ESTRNG, b"+  123.456");

    spicelib::DPFMT(X, &FMT, &mut STRING, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"STRING", &STRING, b"=", &ESTRNG, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
