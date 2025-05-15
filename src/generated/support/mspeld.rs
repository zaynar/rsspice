//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      MSPELD ( Misspelling diagnosis )
pub fn MSPELD(
    WORD: &[u8],
    GUESS: &[u8],
    CAUSE: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut LOC: i32 = 0;
    let mut DIAGNS = [b' '; 12];
    let mut FIRST = [b' '; 16];
    let mut LAST = [b' '; 16];

    //
    // Local variables
    //

    MATCHE(WORD, GUESS, &mut DIAGNS, &mut LOC, ctx)?;

    if fstr::eq(&DIAGNS, b"IDENTITY") {
        fstr::assign(CAUSE, b" ");
    } else if fstr::eq(&DIAGNS, b"TRANSPOSE") {
        spicelib::INTORD(LOC, &mut FIRST, ctx);
        spicelib::INTORD((LOC + 1), &mut LAST, ctx);

        spicelib::LCASE(&FIRST.clone(), &mut FIRST, ctx);
        spicelib::LCASE(&LAST.clone(), &mut LAST, ctx);

        fstr::assign(CAUSE, b"It appears that you have transposed the ");

        spicelib::SUFFIX(&FIRST, 1, CAUSE);
        spicelib::SUFFIX(b"and", 1, CAUSE);
        spicelib::SUFFIX(&LAST, 1, CAUSE);
        spicelib::SUFFIX(b"letters of", 1, CAUSE);
        spicelib::SUFFIX(GUESS, 1, CAUSE);
        spicelib::SUFFIX(b"(the letters", 1, CAUSE);
        spicelib::SUFFIX(fstr::substr(GUESS, LOC..=LOC), 1, CAUSE);
        spicelib::SUFFIX(b"and", 1, CAUSE);
        spicelib::SUFFIX(fstr::substr(GUESS, (LOC + 1)..=(LOC + 1)), 1, CAUSE);
        spicelib::SUFFIX(b").", 0, CAUSE);
    } else if fstr::eq(&DIAGNS, b"INSERT") {
        spicelib::INTORD(LOC, &mut FIRST, ctx);
        spicelib::LCASE(&FIRST.clone(), &mut FIRST, ctx);

        fstr::assign(CAUSE, b"It appears that you have left out the ");

        spicelib::SUFFIX(&FIRST, 1, CAUSE);
        spicelib::SUFFIX(b"letter of ", 1, CAUSE);
        spicelib::SUFFIX(GUESS, 1, CAUSE);
        spicelib::SUFFIX(b". (The ", 0, CAUSE);
        spicelib::SUFFIX(&FIRST, 1, CAUSE);
        spicelib::SUFFIX(b"letter should be ", 1, CAUSE);
        spicelib::SUFFIX(fstr::substr(GUESS, LOC..=LOC), 1, CAUSE);
        spicelib::SUFFIX(b".)", 0, CAUSE);
    } else if fstr::eq(&DIAGNS, b"REPLACE") {
        spicelib::INTORD(LOC, &mut FIRST, ctx);
        spicelib::LCASE(&FIRST.clone(), &mut FIRST, ctx);

        fstr::assign(CAUSE, b"It appears that you have mistyped the ");

        spicelib::SUFFIX(&FIRST, 1, CAUSE);
        spicelib::SUFFIX(b"letter of ", 1, CAUSE);
        spicelib::SUFFIX(GUESS, 1, CAUSE);
        spicelib::SUFFIX(b". (The ", 0, CAUSE);
        spicelib::SUFFIX(&FIRST, 1, CAUSE);
        spicelib::SUFFIX(b"letter should be ", 1, CAUSE);
        spicelib::SUFFIX(fstr::substr(GUESS, LOC..=LOC), 1, CAUSE);
        spicelib::SUFFIX(b". You have ", 0, CAUSE);
        spicelib::SUFFIX(fstr::substr(WORD, LOC..=LOC), 1, CAUSE);
        spicelib::SUFFIX(b"instead.)", 1, CAUSE);
    } else if fstr::eq(&DIAGNS, b"REMOVE") {
        spicelib::INTORD(LOC, &mut FIRST, ctx);
        spicelib::LCASE(&FIRST.clone(), &mut FIRST, ctx);

        fstr::assign(CAUSE, b"It appears that you have an extra letter at the ");

        spicelib::SUFFIX(&FIRST, 1, CAUSE);
        spicelib::SUFFIX(b"letter of ", 1, CAUSE);
        spicelib::SUFFIX(WORD, 1, CAUSE);
        spicelib::SUFFIX(b". (The ", 0, CAUSE);
        spicelib::SUFFIX(&FIRST, 1, CAUSE);
        spicelib::SUFFIX(b"letter ", 1, CAUSE);
        spicelib::SUFFIX(fstr::substr(WORD, LOC..=LOC), 1, CAUSE);
        spicelib::SUFFIX(b"should be removed.)", 1, CAUSE);
    } else {
        fstr::assign(CAUSE, b"I believe you meant ");

        spicelib::SUFFIX(GUESS, 1, CAUSE);
        spicelib::SUFFIX(b".  However, the actual spelling ", 1, CAUSE);
        spicelib::SUFFIX(b"error is not a simple one.      ", 1, CAUSE);
    }

    Ok(())
}
