//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;

//$Procedure      F_VSTRNG ( Tests for the virtual string routine )
pub fn F_VSTRNG(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut X: f64 = 0.0;
    let mut EXP: i32 = 0;
    let mut EXPECT: i32 = 0;
    let mut LETTER = [b' '; 1 as usize];
    let mut EWORD = [b' '; WDSIZE as usize];
    let mut WORD = [b' '; WDSIZE as usize];
    let mut RND: bool = false;
    let mut DID: bool = false;

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
    testutil::TOPEN(b"F_VSTRNG", ctx)?;

    RND = false;

    testutil::TCASE(
        b"Make sure that exponents are returned with the correct value. ",
        ctx,
    )?;

    X = 128.2792;
    spicelib::ZZVSTSTR(X, b" ", &mut EXP, ctx);

    testutil::CHCKSI(b"EXP", EXP, b"=", 2, 0, OK, ctx)?;

    X = 0.01282792;
    spicelib::ZZVSTSTR(X, b" ", &mut EXP, ctx);

    testutil::CHCKSI(b"EXP", EXP, b"=", -2, 0, OK, ctx)?;

    X = 1282792000000000000000000000.0;
    spicelib::ZZVSTSTR(X, b" ", &mut EXP, ctx);

    testutil::CHCKSI(b"EXP", EXP, b"=", 27, 0, OK, ctx)?;

    X = 0.0000000000000000000000000000000001282792;
    spicelib::ZZVSTSTR(X, b" ", &mut EXP, ctx);

    testutil::CHCKSI(b"EXP", EXP, b"=", -34, 0, OK, ctx)?;

    X = 128279200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    spicelib::ZZVSTSTR(X, b" ", &mut EXP, ctx);

    testutil::CHCKSI(b"EXP", EXP, b"=", 257, 0, OK, ctx)?;

    X = 0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001282792;
    spicelib::ZZVSTSTR(X, b" ", &mut EXP, ctx);

    testutil::CHCKSI(b"EXP", EXP, b"=", -103, 0, OK, ctx)?;

    testutil::TCASE(
        b"Perform an exhaustive check on the exponent properties of ZZVSTSTR. ",
        ctx,
    )?;

    X = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012;
    EXPECT = -257;

    for I in 1..=500 {
        spicelib::ZZVSTSTR(X, b" ", &mut EXP, ctx);
        testutil::CHCKSI(b"EXP", EXP, b"=", EXPECT, 0, OK, ctx)?;

        X = (X * 10.0);
        EXPECT = (EXPECT + 1);
    }

    testutil::TCASE(
        b"Make sure we get the correct virtual characters over a wide range of characters. ",
        ctx,
    )?;

    X = 123.281928291;

    spicelib::ZZVSTSTR(X, b"*", &mut EXP, ctx);

    testutil::CHCKSI(b"EXP", EXP, b"=", 2, 0, OK, ctx)?;

    for I in -10..=-4 {
        spicelib::ZZVSBSTR(I, I, RND, &mut LETTER, &mut DID, ctx);
        testutil::CHCKSC(b"LETTER", &LETTER, b"=", b"*", OK, ctx)?;
    }

    spicelib::ZZVSBSTR(-3, -3, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_-3", &LETTER, b"=", b"1", OK, ctx)?;

    spicelib::ZZVSBSTR(-2, -2, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_-2", &LETTER, b"=", b"2", OK, ctx)?;

    spicelib::ZZVSBSTR(-1, -1, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_-1", &LETTER, b"=", b"3", OK, ctx)?;

    spicelib::ZZVSBSTR(0, 0, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_0", &LETTER, b"=", b".", OK, ctx)?;

    spicelib::ZZVSBSTR(1, 1, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_1", &LETTER, b"=", b"2", OK, ctx)?;

    spicelib::ZZVSBSTR(2, 2, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_2", &LETTER, b"=", b"8", OK, ctx)?;

    spicelib::ZZVSBSTR(3, 3, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_3", &LETTER, b"=", b"1", OK, ctx)?;

    spicelib::ZZVSBSTR(4, 4, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_4", &LETTER, b"=", b"9", OK, ctx)?;

    spicelib::ZZVSBSTR(5, 5, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_5", &LETTER, b"=", b"2", OK, ctx)?;

    spicelib::ZZVSBSTR(6, 6, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_6", &LETTER, b"=", b"8", OK, ctx)?;

    spicelib::ZZVSBSTR(7, 7, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_7", &LETTER, b"=", b"2", OK, ctx)?;

    spicelib::ZZVSBSTR(8, 8, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_8", &LETTER, b"=", b"9", OK, ctx)?;

    spicelib::ZZVSBSTR(9, 9, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_9", &LETTER, b"=", b"1", OK, ctx)?;

    spicelib::ZZVSBSTR(10, 10, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_10", &LETTER, b"=", b"0", OK, ctx)?;

    spicelib::ZZVSBSTR(15, 15, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_15", &LETTER, b"=", b"0", OK, ctx)?;

    testutil::TCASE(
        b"Make sure we get the correct virtual characters over a wide range of characters. ",
        ctx,
    )?;

    X = 0.000123281928291;

    spicelib::ZZVSTSTR(X, b"&", &mut EXP, ctx);

    testutil::CHCKSI(b"EXP", EXP, b"=", -4, 0, OK, ctx)?;

    for I in -10..=-2 {
        spicelib::ZZVSBSTR(I, I, RND, &mut LETTER, &mut DID, ctx);
        testutil::CHCKSC(b"LETTER", &LETTER, b"=", b"&", OK, ctx)?;
    }

    spicelib::ZZVSBSTR(-1, -1, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_-1", &LETTER, b"=", b"0", OK, ctx)?;

    spicelib::ZZVSBSTR(0, 0, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_0", &LETTER, b"=", b".", OK, ctx)?;

    spicelib::ZZVSBSTR(1, 1, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_1", &LETTER, b"=", b"0", OK, ctx)?;

    spicelib::ZZVSBSTR(2, 2, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_2", &LETTER, b"=", b"0", OK, ctx)?;

    spicelib::ZZVSBSTR(3, 3, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_3", &LETTER, b"=", b"0", OK, ctx)?;

    spicelib::ZZVSBSTR(4, 4, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_4", &LETTER, b"=", b"1", OK, ctx)?;

    spicelib::ZZVSBSTR(5, 5, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_5", &LETTER, b"=", b"2", OK, ctx)?;

    spicelib::ZZVSBSTR(6, 6, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_6", &LETTER, b"=", b"3", OK, ctx)?;

    spicelib::ZZVSBSTR(7, 7, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_7", &LETTER, b"=", b"2", OK, ctx)?;

    spicelib::ZZVSBSTR(8, 8, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_8", &LETTER, b"=", b"8", OK, ctx)?;

    spicelib::ZZVSBSTR(9, 9, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_9", &LETTER, b"=", b"1", OK, ctx)?;

    spicelib::ZZVSBSTR(10, 10, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_10", &LETTER, b"=", b"9", OK, ctx)?;

    spicelib::ZZVSBSTR(25, 25, RND, &mut LETTER, &mut DID, ctx);
    testutil::CHCKSC(b"LETTER_25", &LETTER, b"=", b"0", OK, ctx)?;

    testutil::TCASE(b"Retrieve a substring of the virtual string ", ctx)?;

    X = (spicelib::PI(ctx) * 100.0);

    spicelib::ZZVSTSTR(X, b"&", &mut EXP, ctx);

    testutil::CHCKSI(b"EXP", EXP, b"=", 2, 0, OK, ctx)?;

    spicelib::ZZVSBSTR(-8, 3, RND, &mut WORD, &mut DID, ctx);

    fstr::assign(&mut EWORD, b"&&&&&314.159");

    testutil::CHCKSC(b"WORD", &WORD, b"=", &EWORD, OK, ctx)?;

    testutil::TCASE(b"Retrieve a substring of the virtual string ", ctx)?;

    X = (spicelib::PI(ctx) * 0.01);

    spicelib::ZZVSTSTR(X, b"&", &mut EXP, ctx);

    testutil::CHCKSI(b"EXP", EXP, b"=", -2, 0, OK, ctx)?;

    spicelib::ZZVSBSTR(-8, 12, RND, &mut WORD, &mut DID, ctx);

    fstr::assign(&mut EWORD, b"&&&&&&&0.031415926535");

    testutil::CHCKSC(b"WORD", &WORD, b"=", &EWORD, OK, ctx)?;

    testutil::TCASE(b"Check to see if virtual string round works.", ctx)?;

    X = 4.999995;

    spicelib::ZZVSTSTR(X, b" ", &mut EXP, ctx);
    testutil::CHCKSI(b"EXP", EXP, b"=", 0, 0, OK, ctx)?;

    spicelib::ZZVSBSTR(-1, 5, true, &mut WORD, &mut DID, ctx);

    fstr::assign(&mut EWORD, b"5.00000");

    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &EWORD, OK, ctx)?;

    X = 9.999995;

    spicelib::ZZVSTSTR(X, b" ", &mut EXP, ctx);
    testutil::CHCKSI(b"EXP", EXP, b"=", 0, 0, OK, ctx)?;

    spicelib::ZZVSBSTR(-1, 5, true, &mut WORD, &mut DID, ctx);

    fstr::assign(&mut EWORD, b"0.00000");

    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &EWORD, OK, ctx)?;

    X = 9.5;

    spicelib::ZZVSTSTR(X, b" ", &mut EXP, ctx);
    testutil::CHCKSI(b"EXP", EXP, b"=", 0, 0, OK, ctx)?;

    spicelib::ZZVSBSTR(-1, 0, true, &mut WORD, &mut DID, ctx);

    fstr::assign(&mut EWORD, b"0.");

    testutil::CHCKSL(b"DID", DID, true, OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &EWORD, OK, ctx)?;

    spicelib::ZZVSBSTR(-3, 0, true, &mut WORD, &mut DID, ctx);

    fstr::assign(&mut EWORD, b" 10.");

    testutil::CHCKSL(b"DID", DID, false, OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &EWORD, OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
