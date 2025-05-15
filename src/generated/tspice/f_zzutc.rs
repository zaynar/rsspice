//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;

//$Procedure      F_ZZUTC ( Family of tests for ZZUTCPM )
pub fn F_ZZUTC(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut STRING = [b' '; LNSIZE as usize];
    let mut EXPHOF: f64 = 0.0;
    let mut EXPMOF: f64 = 0.0;
    let mut HOFF: f64 = 0.0;
    let mut MOFF: f64 = 0.0;
    let mut ELAST: i32 = 0;
    let mut LAST: i32 = 0;
    let mut START: i32 = 0;
    let mut ESUCC: bool = false;
    let mut SUCCES: bool = false;

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
    testutil::TOPEN(b"F_ZZUTC.", ctx)?;

    testutil::TCASE(b"Positive Hours only.", ctx)?;

    fstr::assign(&mut STRING, b"Some ::UTC+10 hours");
    START = 6;
    EXPHOF = 10.0;
    EXPMOF = 0.0;
    ELAST = 13;
    ESUCC = true;

    spicelib::ZZUTCPM(
        &STRING,
        START,
        &mut HOFF,
        &mut MOFF,
        &mut LAST,
        &mut SUCCES,
        ctx,
    );

    testutil::CHCKSI(b"LAST", LAST, b"=", ELAST, 0, OK, ctx)?;
    testutil::CHCKSI(b"START", START, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKSD(b"MOFF", MOFF, b"=", EXPMOF, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"HOFF", HOFF, b"=", EXPHOF, 0.0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCC, OK, ctx)?;

    testutil::TCASE(b"Negative Hours only.", ctx)?;

    fstr::assign(&mut STRING, b"Some ::UTC-8 hours");
    START = 6;
    EXPHOF = -8.0;
    EXPMOF = 0.0;
    ELAST = 12;
    ESUCC = true;

    spicelib::ZZUTCPM(
        &STRING,
        START,
        &mut HOFF,
        &mut MOFF,
        &mut LAST,
        &mut SUCCES,
        ctx,
    );

    testutil::CHCKSI(b"LAST", LAST, b"=", ELAST, 0, OK, ctx)?;
    testutil::CHCKSI(b"START", START, b"=", 6, 0, OK, ctx)?;
    testutil::CHCKSD(b"MOFF", MOFF, b"=", EXPMOF, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"HOFF", HOFF, b"=", EXPHOF, 0.0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCC, OK, ctx)?;

    testutil::TCASE(b"Postive Hours and Minutes.", ctx)?;

    fstr::assign(&mut STRING, b"An offset of ::UTC+11:17 hours and minutes");

    START = 14;
    EXPHOF = 11.0;
    EXPMOF = 17.0;
    ELAST = 24;
    ESUCC = true;

    spicelib::ZZUTCPM(
        &STRING,
        START,
        &mut HOFF,
        &mut MOFF,
        &mut LAST,
        &mut SUCCES,
        ctx,
    );

    testutil::CHCKSI(b"LAST", LAST, b"=", ELAST, 0, OK, ctx)?;
    testutil::CHCKSI(b"START", START, b"=", 14, 0, OK, ctx)?;
    testutil::CHCKSD(b"MOFF", MOFF, b"=", EXPMOF, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"HOFF", HOFF, b"=", EXPHOF, 0.0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCC, OK, ctx)?;

    testutil::TCASE(b"Negative Hours and Minutes.", ctx)?;

    fstr::assign(&mut STRING, b"An offset of ::UTC-05:33 hours and minutes");

    START = 14;
    EXPHOF = -5.0;
    EXPMOF = -33.0;
    ELAST = 24;
    ESUCC = true;

    spicelib::ZZUTCPM(
        &STRING,
        START,
        &mut HOFF,
        &mut MOFF,
        &mut LAST,
        &mut SUCCES,
        ctx,
    );

    testutil::CHCKSI(b"LAST", LAST, b"=", ELAST, 0, OK, ctx)?;
    testutil::CHCKSI(b"START", START, b"=", 14, 0, OK, ctx)?;
    testutil::CHCKSD(b"MOFF", MOFF, b"=", EXPMOF, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"HOFF", HOFF, b"=", EXPHOF, 0.0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCC, OK, ctx)?;

    testutil::TCASE(b"Out of range Hours.", ctx)?;

    fstr::assign(&mut STRING, b"An offset of ::UTC-15:33 hours and minutes");

    START = 14;
    EXPHOF = 0.0;
    EXPMOF = 0.0;
    ELAST = 13;
    ESUCC = false;

    spicelib::ZZUTCPM(
        &STRING,
        START,
        &mut HOFF,
        &mut MOFF,
        &mut LAST,
        &mut SUCCES,
        ctx,
    );

    testutil::CHCKSI(b"LAST", LAST, b"=", ELAST, 0, OK, ctx)?;
    testutil::CHCKSI(b"START", START, b"=", 14, 0, OK, ctx)?;
    testutil::CHCKSD(b"MOFF", MOFF, b"=", EXPMOF, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"HOFF", HOFF, b"=", EXPHOF, 0.0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCC, OK, ctx)?;

    testutil::TCASE(b"Out of range Minutes.", ctx)?;

    fstr::assign(&mut STRING, b"An offset of ::UTC+05:63 hours and minutes");

    START = 14;
    EXPHOF = 5.0;
    EXPMOF = 0.0;
    ELAST = 21;
    ESUCC = true;

    spicelib::ZZUTCPM(
        &STRING,
        START,
        &mut HOFF,
        &mut MOFF,
        &mut LAST,
        &mut SUCCES,
        ctx,
    );

    testutil::CHCKSI(b"LAST", LAST, b"=", ELAST, 0, OK, ctx)?;
    testutil::CHCKSI(b"START", START, b"=", 14, 0, OK, ctx)?;
    testutil::CHCKSD(b"MOFF", MOFF, b"=", EXPMOF, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"HOFF", HOFF, b"=", EXPHOF, 0.0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCC, OK, ctx)?;

    testutil::TCASE(b"Unparsable Hours.", ctx)?;

    fstr::assign(&mut STRING, b"An offset of ::UTC+ONE:33 hours and minutes");

    START = 14;
    EXPHOF = 0.0;
    EXPMOF = 0.0;
    ELAST = 13;
    ESUCC = false;

    spicelib::ZZUTCPM(
        &STRING,
        START,
        &mut HOFF,
        &mut MOFF,
        &mut LAST,
        &mut SUCCES,
        ctx,
    );

    testutil::CHCKSI(b"LAST", LAST, b"=", ELAST, 0, OK, ctx)?;
    testutil::CHCKSI(b"START", START, b"=", 14, 0, OK, ctx)?;
    testutil::CHCKSD(b"MOFF", MOFF, b"=", EXPMOF, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"HOFF", HOFF, b"=", EXPHOF, 0.0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCC, OK, ctx)?;

    testutil::TCASE(b"Unparsable Minutes.", ctx)?;

    fstr::assign(
        &mut STRING,
        b"An offset of ::UTC+01:TWELVE hours and minutes",
    );

    START = 14;
    EXPHOF = 1.0;
    EXPMOF = 0.0;
    ELAST = 21;
    ESUCC = true;

    spicelib::ZZUTCPM(
        &STRING,
        START,
        &mut HOFF,
        &mut MOFF,
        &mut LAST,
        &mut SUCCES,
        ctx,
    );

    testutil::CHCKSI(b"LAST", LAST, b"=", ELAST, 0, OK, ctx)?;
    testutil::CHCKSI(b"START", START, b"=", 14, 0, OK, ctx)?;
    testutil::CHCKSD(b"MOFF", MOFF, b"=", EXPMOF, 0.0, OK, ctx)?;
    testutil::CHCKSD(b"HOFF", HOFF, b"=", EXPHOF, 0.0, OK, ctx)?;
    testutil::CHCKSL(b"SUCCES", SUCCES, ESUCC, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
