//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure M2GETC ( META/2 --- get a named word---character )
pub fn M2GETC(
    NAME: &[u8],
    STRING: &[u8],
    FOUND: &mut bool,
    WORD: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut F: i32 = 0;
    let mut L: i32 = 0;
    let mut P: i32 = 0;
    let mut W: i32 = 0;

    //

    //
    // Local variables
    //

    //
    // First look up the beginning and endings of the requested word.
    //
    M2VGET(NAME, 1, FOUND, &mut B, &mut E, ctx)?;

    if !*FOUND {
        return Ok(());
    }

    //
    // First make sure there is nothing pathological about the string
    // we are dealing with.
    //
    P = (B - 1);
    F = (E + 1);
    L = intrinsics::LEN(STRING);
    W = intrinsics::LEN(WORD);

    if (P > 0) {
        if fstr::ne(fstr::substr(STRING, P..=P), b" ") {
            spicelib::CHKIN(b"M2GETC", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2GETC", ctx)?;
            return Ok(());
        }
    }

    if (F < L) {
        if fstr::ne(fstr::substr(STRING, F..=F), b" ") {
            spicelib::CHKIN(b"M2GETC", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2GETC", ctx)?;
            return Ok(());
        }
    }

    if (fstr::eq(fstr::substr(STRING, B..=B), b" ") || fstr::eq(fstr::substr(STRING, E..=E), b" "))
    {
        spicelib::CHKIN(b"M2GETC", ctx)?;
        spicelib::SETMSG(
            b"The input string has been modified since it passed syntax validation in META/2. ",
            ctx,
        );
        spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
        spicelib::CHKOUT(b"M2GETC", ctx)?;
        return Ok(());
    }

    if (W < ((E - B) + 1)) {
        spicelib::CHKIN(b"M2GETC", ctx)?;
        spicelib::SETMSG(
            b"There is not sufficient space in the output string to hold the requested word. ",
            ctx,
        );
        spicelib::SIGERR(b"META/2(INSUFFICIENTSPACE)", ctx)?;
        spicelib::CHKOUT(b"M2GETC", ctx)?;
        return Ok(());
    }

    //
    // Now do the actual assignment
    //
    fstr::assign(WORD, fstr::substr(STRING, B..=E));

    Ok(())
}
