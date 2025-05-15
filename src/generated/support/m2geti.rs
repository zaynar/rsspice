//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure M2GETI ( META/2 --- get a named integer)
pub fn M2GETI(
    NAME: &[u8],
    STRING: &[u8],
    FOUND: &mut bool,
    INT: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ERROR = [b' '; 80 as usize];
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut F: i32 = 0;
    let mut L: i32 = 0;
    let mut MYINT: i32 = 0;
    let mut P: i32 = 0;
    let mut PNTER: i32 = 0;

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

    if (P > 0) {
        if fstr::ne(fstr::substr(STRING, P..=P), b" ") {
            spicelib::CHKIN(b"M2GETI", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2GETI", ctx)?;
            return Ok(());
        }
    }

    if (F < L) {
        if fstr::ne(fstr::substr(STRING, F..=F), b" ") {
            spicelib::CHKIN(b"M2GETI", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2GETI", ctx)?;
            return Ok(());
        }
    }

    if (fstr::eq(fstr::substr(STRING, B..=B), b" ") || fstr::eq(fstr::substr(STRING, E..=E), b" "))
    {
        spicelib::CHKIN(b"M2GETI", ctx)?;
        spicelib::SETMSG(
            b"The input string has been modified since it passed syntax validation in META/2. ",
            ctx,
        );
        spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
        spicelib::CHKOUT(b"M2GETI", ctx)?;
        return Ok(());
    }

    //
    // This is supposed to be an integer double precision number.
    // Parse it.
    //
    spicelib::NPARSI(
        fstr::substr(STRING, B..=E),
        &mut MYINT,
        &mut ERROR,
        &mut PNTER,
        ctx,
    );

    if fstr::ne(&ERROR, b" ") {
        spicelib::CHKIN(b"M2GETI", ctx)?;
        spicelib::SETMSG(
            b"The item requested could not be parsed as an integer.",
            ctx,
        );
        spicelib::SIGERR(b"META/2(CORRUPTEDINTEGER)", ctx)?;
        spicelib::CHKOUT(b"M2GETI", ctx)?;
        return Ok(());
    }

    //
    // Now do the actual assignment
    //
    *INT = MYINT;

    Ok(())
}
