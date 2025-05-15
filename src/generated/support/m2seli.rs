//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    ERROR: Vec<u8>,
    B: i32,
    E: i32,
    F: i32,
    L: i32,
    MYINT: i32,
    P: i32,
    PNTER: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ERROR = vec![b' '; 80 as usize];
        let mut B: i32 = 0;
        let mut E: i32 = 0;
        let mut F: i32 = 0;
        let mut L: i32 = 0;
        let mut MYINT: i32 = 0;
        let mut P: i32 = 0;
        let mut PNTER: i32 = 0;

        Self {
            ERROR,
            B,
            E,
            F,
            L,
            MYINT,
            P,
            PNTER,
        }
    }
}

//$Procedure M2SELI ( META/2 --- select a named integer )
pub fn M2SELI(
    NAME: &[u8],
    STRING: &[u8],
    NTH: i32,
    FOUND: &mut bool,
    INT: &mut i32,
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

    if (save.P > 0) {
        if fstr::ne(fstr::substr(STRING, save.P..=save.P), b" ") {
            spicelib::CHKIN(b"M2SELI", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2SELI", ctx)?;
            return Ok(());
        }
    }

    if (save.F < save.L) {
        if fstr::ne(fstr::substr(STRING, save.F..=save.F), b" ") {
            spicelib::CHKIN(b"M2SELI", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2SELI", ctx)?;
            return Ok(());
        }
    }

    if (fstr::eq(fstr::substr(STRING, save.B..=save.B), b" ")
        || fstr::eq(fstr::substr(STRING, save.E..=save.E), b" "))
    {
        spicelib::CHKIN(b"M2SELI", ctx)?;
        spicelib::SETMSG(
            b"The input string has been modified since it passed syntax validation in META/2. ",
            ctx,
        );
        spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
        spicelib::CHKOUT(b"M2SELI", ctx)?;
        return Ok(());
    }

    //
    // This is supposed to be an integer double precision number.
    // Parse it.
    //
    spicelib::NPARSI(
        fstr::substr(STRING, save.B..=save.E),
        &mut save.MYINT,
        &mut save.ERROR,
        &mut save.PNTER,
        ctx,
    );

    if fstr::ne(&save.ERROR, b" ") {
        spicelib::CHKIN(b"M2SELI", ctx)?;
        spicelib::SETMSG(
            b"The item requested could not be parsed as an integer. a number.",
            ctx,
        );
        spicelib::SIGERR(b"META/2(CORRUPTEDINTEGER)", ctx)?;
        spicelib::CHKOUT(b"M2SELI", ctx)?;
        return Ok(());
    }

    //
    // Now do the actual assignment
    //
    *INT = save.MYINT;

    Ok(())
}
