//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const NLINES: i32 = 6;

//$Procedure F_BADKPV (Family of tests for BADKPV )
pub fn F_BADKPV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LINES = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut LNUM: bool = false;
    let mut LSTR: bool = false;

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
    testutil::TOPEN(b"F_BADKPV", ctx)?;

    testutil::BEGTXT(&mut LINES[1]);
    fstr::assign(
        LINES.get_mut(2),
        b"This text kernel is for test purposes only.  It serves no",
    );
    fstr::assign(LINES.get_mut(3), b"other purpose");
    testutil::BEGDAT(&mut LINES[4]);
    fstr::assign(LINES.get_mut(5), b"NUM = ( 1, 2, 3, 4, 5 )");
    fstr::assign(
        LINES.get_mut(6),
        b"STR = ( \'This\', \'is\', \'a\', \'string\', \'of\', \'words.\' )",
    );

    testutil::KILFIL(b"sample.txt", ctx)?;
    testutil::TSTTXT(b"sample.txt", LINES.as_arg(), 6, true, true, ctx)?;

    LNUM = false;
    LSTR = false;

    //
    // *****************************************************************
    //
    // Error cases: BADKPV
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure an error is signaled when a character variable has integer values. ",
        ctx,
    )?;

    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b"=", 5, 1, b"C", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLETYPE)", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, true, OK, ctx)?;
    LNUM = false;
    LSTR = false;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure an error is signaled when an numeric variable has character values ",
        ctx,
    )?;

    LSTR = spicelib::BADKPV(b"TEST", b"STR", b"=", 6, 1, b"N", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLETYPE)", OK, ctx)?;
    testutil::CHCKSL(b"LSTR", LSTR, true, OK, ctx)?;
    LNUM = false;
    LSTR = false;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure an error is signaled when a variable has a dimension that is not the exact expected value ", ctx)?;

    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b"=", 6, 1, b"N", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, true, OK, ctx)?;
    LNUM = false;
    LSTR = false;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure an error is signaled when a variable has a dimension that is not more than an expected value. ", ctx)?;

    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b">", 5, 1, b"N", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, true, OK, ctx)?;
    LNUM = false;
    LSTR = false;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure an error is signaled when a variable has a dimension that is not at least an expected value. ", ctx)?;

    LSTR = spicelib::BADKPV(b"TEST", b"STR", b"=>", 7, 1, b"C", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;
    testutil::CHCKSL(b"LSTR", LSTR, true, OK, ctx)?;
    LNUM = false;
    LSTR = false;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure an error is signaled when a variable has a dimension that is not at most an expected value. ", ctx)?;

    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b"<=", 4, 1, b"N", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, true, OK, ctx)?;
    LNUM = false;
    LSTR = false;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure an error is signaled when a variable has a dimension that is not less than some expected value. ", ctx)?;

    LSTR = spicelib::BADKPV(b"TEST", b"STR", b"<", 6, 1, b"C", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;
    testutil::CHCKSL(b"LSTR", LSTR, true, OK, ctx)?;
    LNUM = false;
    LSTR = false;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure an error is signaled when the dimension of a variable is not divisible by the prescribed value. ", ctx)?;

    LSTR = spicelib::BADKPV(b"TEST", b"STR", b"=", 6, 4, b"C", ctx)?;

    testutil::CHCKXC(true, b"SPICE(BADVARIABLESIZE)", OK, ctx)?;
    testutil::CHCKSL(b"LSTR", LSTR, true, OK, ctx)?;
    LNUM = false;
    LSTR = false;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure an error is signaled when a variable is not found in the kernel pool. ",
        ctx,
    )?;

    LNUM = spicelib::BADKPV(b"TEST", b"SPK", b"=", 1, 1, b"N", ctx)?;

    testutil::CHCKXC(true, b"SPICE(VARIABLENOTFOUND)", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, true, OK, ctx)?;

    LNUM = false;
    LSTR = false;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Make sure an error is signaled when the comparison operator is not recognized. ",
        ctx,
    )?;

    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b"?", 5, 1, b"N", ctx)?;

    testutil::CHCKXC(true, b"SPICE(UNKNOWNCOMPARE)", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, true, OK, ctx)?;
    LNUM = false;
    LSTR = false;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure an error is signaled when the expected kernel variable type is not recognized. ", ctx)?;

    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b"=", 5, 1, b"?", ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDTYPE)", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, true, OK, ctx)?;
    LNUM = false;
    LSTR = false;

    //
    // *****************************************************************
    //
    // Normal cases: BADKPV
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure the \'=\' operator behaves as expected ", ctx)?;

    LSTR = spicelib::BADKPV(b"TEST", b"STR", b"=", 6, 2, b"C", ctx)?;
    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b"=", 5, 5, b"N", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, false, OK, ctx)?;
    testutil::CHCKSL(b"LSTR", LSTR, false, OK, ctx)?;
    LNUM = true;
    LSTR = true;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure the \'>\' operator behaves as expected ", ctx)?;

    LSTR = spicelib::BADKPV(b"TEST", b"STR", b">", 4, 2, b"C", ctx)?;
    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b">", 4, 5, b"N", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, false, OK, ctx)?;
    testutil::CHCKSL(b"LSTR", LSTR, false, OK, ctx)?;
    LNUM = true;
    LSTR = true;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure the \'=>\' operator behaves as expected ", ctx)?;

    LSTR = spicelib::BADKPV(b"TEST", b"STR", b"=>", 6, 2, b"C", ctx)?;
    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b"=>", 5, 5, b"N", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, false, OK, ctx)?;
    testutil::CHCKSL(b"LSTR", LSTR, false, OK, ctx)?;
    LNUM = true;
    LSTR = true;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure the \'<\' operator behaves as expected ", ctx)?;

    LSTR = spicelib::BADKPV(b"TEST", b"STR", b"<", 7, 2, b"C", ctx)?;
    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b"<", 6, 5, b"N", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, false, OK, ctx)?;
    testutil::CHCKSL(b"LSTR", LSTR, false, OK, ctx)?;
    LNUM = true;
    LSTR = true;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Make sure the \'<=\' operator behaves as expected ", ctx)?;

    LSTR = spicelib::BADKPV(b"TEST", b"STR", b"<=", 6, 2, b"C", ctx)?;
    LNUM = spicelib::BADKPV(b"TEST", b"NUM", b"<=", 5, 5, b"N", ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSL(b"LNUM", LNUM, false, OK, ctx)?;
    testutil::CHCKSL(b"LSTR", LSTR, false, OK, ctx)?;
    LNUM = true;
    LSTR = true;

    testutil::KILFIL(b"sample.txt", ctx)?;
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
