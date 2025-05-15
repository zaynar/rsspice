//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure M2GETD (META/2 --- select a named word, double precision )
pub fn M2GETD(
    NAME: &[u8],
    STRING: &[u8],
    FOUND: &mut bool,
    DP: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ERROR = [b' '; 80 as usize];
    let mut MYDP: f64 = 0.0;
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut F: i32 = 0;
    let mut L: i32 = 0;
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
            spicelib::CHKIN(b"M2GETD", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2GETD", ctx)?;
            return Ok(());
        }
    }

    if (F < L) {
        if fstr::ne(fstr::substr(STRING, F..=F), b" ") {
            spicelib::CHKIN(b"M2GETD", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2GETD", ctx)?;
            return Ok(());
        }
    }

    if (fstr::eq(fstr::substr(STRING, B..=B), b" ") || fstr::eq(fstr::substr(STRING, E..=E), b" "))
    {
        spicelib::CHKIN(b"M2GETD", ctx)?;
        spicelib::SETMSG(
            b"The input string has been modified since it passed syntax validation in META/2. ",
            ctx,
        );
        spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
        spicelib::CHKOUT(b"M2GETD", ctx)?;
        return Ok(());
    }

    //
    // This is supposed to be an integer double precision number.
    // Parse it.
    //
    spicelib::NPARSD(
        fstr::substr(STRING, B..=E),
        &mut MYDP,
        &mut ERROR,
        &mut PNTER,
        ctx,
    );

    if fstr::ne(&ERROR, b" ") {
        spicelib::CHKIN(b"M2GETD", ctx)?;
        spicelib::SETMSG(
            b"The item requested could not be parsed as an integer. a number.",
            ctx,
        );
        spicelib::SIGERR(b"META/2(CORRUPTEDNUMBER)", ctx)?;
        spicelib::CHKOUT(b"M2GETD", ctx)?;
        return Ok(());
    }

    //
    // Now do the actual assignment
    //
    *DP = MYDP;

    Ok(())
}
