//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    B: i32,
    E: i32,
    F: i32,
    L: i32,
    P: i32,
    W: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut B: i32 = 0;
        let mut E: i32 = 0;
        let mut F: i32 = 0;
        let mut L: i32 = 0;
        let mut P: i32 = 0;
        let mut W: i32 = 0;

        Self { B, E, F, L, P, W }
    }
}

//$Procedure M2SELC ( META/2 --- select a named word )
pub fn M2SELC(
    NAME: &[u8],
    STRING: &[u8],
    NTH: i32,
    FOUND: &mut bool,
    WORD: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // Local variables
    //

    //
    // First look up the beginning and endings of the requested word.
    //
    M2VGET(NAME, NTH, FOUND, &mut save.B, &mut save.E, ctx)?;

    if !*FOUND {
        return Ok(());
    }

    //
    // First make sure there is nothing pathological about the string
    // we are dealing with.
    //
    save.P = (save.B - 1);
    save.F = (save.E + 1);
    save.L = intrinsics::LEN(STRING);
    save.W = intrinsics::LEN(WORD);

    if (save.P > 0) {
        if fstr::ne(fstr::substr(STRING, save.P..=save.P), b" ") {
            spicelib::CHKIN(b"M2SELC", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2SELC", ctx)?;
            return Ok(());
        }
    }

    if (save.F < save.L) {
        if fstr::ne(fstr::substr(STRING, save.F..=save.F), b" ") {
            spicelib::CHKIN(b"M2SELC", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2SELC", ctx)?;
            return Ok(());
        }
    }

    if (fstr::eq(fstr::substr(STRING, save.B..=save.B), b" ")
        || fstr::eq(fstr::substr(STRING, save.E..=save.E), b" "))
    {
        spicelib::CHKIN(b"M2SELC", ctx)?;
        spicelib::SETMSG(
            b"The input string has been modified since it passed syntax validation in META/2. ",
            ctx,
        );
        spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
        spicelib::CHKOUT(b"M2SELC", ctx)?;
        return Ok(());
    }

    if (save.W < ((save.E - save.B) + 1)) {
        spicelib::CHKIN(b"M2SELC", ctx)?;
        spicelib::SETMSG(
            b"There is not sufficient space in the output string to hold the requested word. ",
            ctx,
        );
        spicelib::SIGERR(b"META/2(INSUFFICIENTSPACE)", ctx)?;
        spicelib::CHKOUT(b"M2SELC", ctx)?;
        return Ok(());
    }

    //
    // Now do the actual assignment
    //
    fstr::assign(WORD, fstr::substr(STRING, save.B..=save.E));

    Ok(())
}
