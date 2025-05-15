//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const WDSIZE: i32 = 32;

//$Procedure F_REPMX ( Family of tests for replace marker routines )
pub fn F_REPMX(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LINE = [b' '; LNSIZE as usize];
    let mut RESULT = [b' '; LNSIZE as usize];
    let mut EXPLIN = [b' '; LNSIZE as usize];
    let mut WORD = [b' '; WDSIZE as usize];
    let mut WEXP = [b' '; WDSIZE as usize];
    let mut SUB = [b' '; WDSIZE as usize];
    let mut I: i32 = 0;
    let mut X: f64 = 0.0;

    //
    // Local Variables
    //

    //
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_REPMX", ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: REPMC
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Character substitution  -- check REPMC", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'ok\'.");
    fstr::assign(&mut SUB, b"ok");

    spicelib::REPMC(&LINE, b"#", &SUB, &mut RESULT);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Character substitution in-place -- check REPMC", ctx)?;

    fstr::assign(&mut RESULT, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'ok\'.");
    fstr::assign(&mut SUB, b"ok");

    spicelib::REPMC(&RESULT.clone(), b"#", &SUB, &mut RESULT);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Character substitution truncated -- check REPMC", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'a very long string indeed\'.");
    fstr::assign(&mut SUB, b"a very long string indeed");

    spicelib::REPMC(&LINE, b"#", &SUB, &mut WORD);

    fstr::assign(&mut WEXP, fstr::substr(&EXPLIN, 1..=WDSIZE));

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &WEXP, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Character substitution in-place truncated -- check REPMC",
        ctx,
    )?;

    fstr::assign(&mut WORD, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'a very long string indeed\'.");
    fstr::assign(&mut SUB, b"a very long string indeed");

    spicelib::REPMC(&WORD.clone(), b"#", &SUB, &mut WORD);

    fstr::assign(&mut WEXP, fstr::substr(&EXPLIN, 1..=WDSIZE));

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &WEXP, OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: REPMI
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Integer substitution -- check REPMI", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'3\'.");
    I = 3;

    spicelib::REPMI(&LINE, b"#", I, &mut RESULT, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Integer substitution in-place -- check REPMI", ctx)?;

    fstr::assign(&mut RESULT, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'3\'.");
    I = 3;

    spicelib::REPMI(&RESULT.clone(), b"#", I, &mut RESULT, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: REPMD
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Double precision substitution (scientific) -- REPMD", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'4.0E+00\'.");
    X = 4.0;

    spicelib::REPMD(&LINE, b"#", X, 2, &mut RESULT, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Double precision substitution in-place (scientific) -- REPMD",
        ctx,
    )?;

    fstr::assign(&mut RESULT, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'4.0E+00\'.");
    X = 4.0;

    spicelib::REPMD(&LINE, b"#", X, 2, &mut RESULT, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: REPMF
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Double precision substitution (float) -- REPMF", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'4.0\'.");
    X = 4.0;

    spicelib::REPMF(&LINE, b"#", X, 2, b"f", &mut RESULT, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Double precision substitution in-place (float) -- REPMF",
        ctx,
    )?;

    fstr::assign(&mut RESULT, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'4.0\'.");
    X = 4.0;

    spicelib::REPMF(&RESULT.clone(), b"#", X, 2, b"g", &mut RESULT, ctx);

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: REPML
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Logical value substitution (empty marker) -- check REPML",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'#\'.");

    spicelib::REPML(&LINE, b" ", true, b"u", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Logical value substitution (non-existent marker) -- check REPML",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'#\'.");

    spicelib::REPML(&LINE, b"abc", true, b"u", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Logical value substitution (first marker, TRUE) -- check REPML",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The values are \'#\' and \'#\'.");
    fstr::assign(&mut EXPLIN, b"The values are \'TRUE\' and \'#\'.");

    spicelib::REPML(&LINE, b"#", true, b"u", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Logical value substitution truncated -- check REPML", ctx)?;

    fstr::assign(&mut LINE, b"The values are long \'#\' and \'#\'.");
    fstr::assign(&mut EXPLIN, b"The values are long \'TRUE\' and \'#\'.");
    fstr::assign(&mut WEXP, fstr::substr(&EXPLIN, 1..=WDSIZE));

    spicelib::REPML(&LINE, b"#", true, b"u", &mut WORD, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &WEXP, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Logical value substitution in-place (TRUE) -- check REPML",
        ctx,
    )?;

    fstr::assign(&mut RESULT, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'TRUE\'.");

    spicelib::REPML(&RESULT.clone(), b"#", true, b"U", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Logical value substitution in-place  truncated -- check REPML",
        ctx,
    )?;

    fstr::assign(&mut WORD, b"The value is \'#\' and \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'TRUE\' and \'#\'.");
    fstr::assign(&mut WEXP, fstr::substr(&EXPLIN, 1..=WDSIZE));

    spicelib::REPML(&WORD.clone(), b"#", true, b"U", &mut WORD, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &WEXP, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Logical value substitution (True) -- check REPML", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'True\'.");

    spicelib::REPML(&LINE, b"#", true, b"c", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Logical value substitution (true) -- check REPML", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'true\'.");

    spicelib::REPML(&LINE, b"#", true, b"l", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Logical value substitution (FALSE) -- check REPML", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'FALSE\'.");

    spicelib::REPML(&LINE, b"#", false, b"u", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Logical value substitution (False) -- check REPML", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'False\'.");

    spicelib::REPML(&LINE, b"#", false, b"c", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Logical value substitution (false) -- check REPML", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'false\'.");

    spicelib::REPML(&LINE, b"#", false, b"l", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: REPMCT
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Cardinal text substitution (empty marker) -- check REPMCT",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'#\'.");
    I = 55;

    spicelib::REPMCT(&LINE, b" ", I, b"u", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Cardinal text substitution (non-existent marker) -- check REPMCT",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'#\'.");
    I = 55;

    spicelib::REPMCT(&LINE, b"abc", I, b"u", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Cardinal text substitution, lowercase -- check REPMCT",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'fifty-five\'.");
    I = 55;

    spicelib::REPMCT(&LINE, b"#", I, b"l", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Cardinal text substitution, uppercase -- check REPMCT",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'FIFTY-FIVE\'.");
    I = 55;

    spicelib::REPMCT(&LINE, b"#", I, b"U", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Cardinal text substitution, capitalized -- check REPMCT",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'Fifty-five\'.");
    I = 55;

    spicelib::REPMCT(&LINE, b"#", I, b"c", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Cardinal text substitution truncated -- check REPMCT", ctx)?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'seven hundred seven\'.");
    fstr::assign(&mut WEXP, fstr::substr(&EXPLIN, 1..=WDSIZE));

    I = 777777777;

    spicelib::REPMCT(&LINE, b"#", I, b"l", &mut WORD, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &WEXP, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Cardinal text substitution in-place -- check REPMCT", ctx)?;

    fstr::assign(&mut RESULT, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'fifty-five\'.");
    I = 55;

    spicelib::REPMCT(&RESULT.clone(), b"#", I, b"L", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Cardinal text substitution in-place truncated -- check REPMCT",
        ctx,
    )?;

    fstr::assign(&mut WORD, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'seven hundred seven\'.");
    fstr::assign(&mut WEXP, fstr::substr(&EXPLIN, 1..=WDSIZE));

    I = 777777777;

    spicelib::REPMCT(&WORD.clone(), b"#", I, b"l", &mut WORD, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &WEXP, OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: REPMOT
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Ordinal text substitution (empty marker) -- check REPMOT",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'#\'.");
    I = 55;

    spicelib::REPMOT(&LINE, b" ", I, b"u", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Ordinal text substitution (non-existent marker) -- check REPMOT",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    fstr::assign(&mut EXPLIN, b"The value is \'#\'.");
    I = 55;

    spicelib::REPMOT(&LINE, b"abc", I, b"u", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Ordinal text substitution, lowercase -- check REPMOT", ctx)?;

    fstr::assign(&mut LINE, b"The \'#\' value is 55.");
    fstr::assign(&mut EXPLIN, b"The \'fifty-fifth\' value is 55.");
    I = 55;

    spicelib::REPMOT(&LINE, b"#", I, b"l", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Ordinal text substitution, uppercase -- check REPMOT", ctx)?;

    fstr::assign(&mut LINE, b"The \'#\' value is 55.");
    fstr::assign(&mut EXPLIN, b"The \'FIFTY-FIFTH\' value is 55.");
    I = 55;

    spicelib::REPMOT(&LINE, b"#", I, b"U", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Ordinal text substitution, capitalized -- check REPMOT",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The \'#\' value is 55.");
    fstr::assign(&mut EXPLIN, b"The \'Fifty-fifth\' value is 55.");
    I = 55;

    spicelib::REPMOT(&LINE, b"#", I, b"c", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Ordinal text substitution truncated -- check REPMOT", ctx)?;

    fstr::assign(&mut LINE, b"The \'#\' value is 777 777 777.");
    fstr::assign(&mut EXPLIN, b"The \'seven hundred seventy-seven million\'.");
    fstr::assign(&mut WEXP, fstr::substr(&EXPLIN, 1..=WDSIZE));

    I = 777777777;

    spicelib::REPMOT(&LINE, b"#", I, b"l", &mut WORD, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &WEXP, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"Ordinal text substitution in-place -- check REPMOT", ctx)?;

    fstr::assign(&mut RESULT, b"The \'#\' value is 77.");
    fstr::assign(&mut EXPLIN, b"The \'seventy-seventh\' value is 77.");
    I = 77;

    spicelib::REPMOT(&RESULT.clone(), b"#", I, b"L", &mut RESULT, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"RESULT", &RESULT, b"=", &EXPLIN, OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Ordinal text substitution in-place truncated -- check REPMOT",
        ctx,
    )?;

    fstr::assign(&mut WORD, b"The \'#\' value is 77.");
    fstr::assign(&mut EXPLIN, b"The \'seventy-seventh\' value is 77.");
    fstr::assign(&mut WEXP, fstr::substr(&EXPLIN, 1..=WDSIZE));

    I = 77;

    spicelib::REPMOT(&WORD.clone(), b"#", I, b"l", &mut WORD, ctx)?;

    testutil::CHCKXC(false, b" ", OK, ctx)?;
    testutil::CHCKSC(b"WORD", &WORD, b"=", &WEXP, OK, ctx)?;

    //
    // *****************************************************************
    //
    // Error cases: REPMCT
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Cardinal text substitution (invalid case flag) -- check REPMCT",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    I = 55;

    spicelib::REPMCT(&LINE, b"#", I, b"n", &mut RESULT, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCASE)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Error cases: REPML
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Logical value substitution (invalid case flag) -- check REPML",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");

    spicelib::REPML(&LINE, b"#", true, b"n", &mut RESULT, ctx)?;
    testutil::CHCKXC(true, b"SPICE(INVALIDCASE)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Error cases: REPMOT
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(
        b"Ordinal text substitution (invalid case flag) -- check REPMOT",
        ctx,
    )?;

    fstr::assign(&mut LINE, b"The value is \'#\'.");
    I = 55;

    spicelib::REPMOT(&LINE, b"#", I, b"n", &mut RESULT, ctx)?;

    testutil::CHCKXC(true, b"SPICE(INVALIDCASE)", OK, ctx)?;

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
