//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure M2GETA ( META/2 --- get all of a named word )
pub fn M2GETA(
    NAME: &[u8],
    STRING: &[u8],
    FOUND: &mut bool,
    WORD: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut B = StackArray::<i32, 2>::new(1..=2);
    let mut E = StackArray::<i32, 2>::new(1..=2);
    let mut F: i32 = 0;
    let mut L: i32 = 0;
    let mut LAST: i32 = 0;
    let mut P: i32 = 0;
    let mut W: i32 = 0;

    //

    //
    // META/2 functions
    //

    //
    // Local variables
    //

    //
    // First look up the beginning and endings of the requested
    // substring.
    //
    M2VGET(NAME, 1, FOUND, B.first_mut(), E.first_mut(), ctx)?;

    if !*FOUND {
        return Ok(());
    }

    //
    // First find out how many substrings are associated with this name.
    //
    LAST = M2HAVE(NAME, ctx)?;

    if (LAST == 0) {
        *FOUND = false;
        return Ok(());
    }

    //
    // Now get the beginning and ending of all the stuff associated
    // with this word.
    //
    M2VGET(NAME, 1, FOUND, &mut B[1], &mut E[1], ctx)?;
    M2VGET(NAME, LAST, FOUND, &mut B[2], &mut E[2], ctx)?;

    //
    // First make sure there are no obvious pathologies about the string
    // we are dealing with.
    //
    L = intrinsics::LEN(STRING);

    for I in 1..=2 {
        P = (B[I] - 1);
        F = (E[I] + 1);

        if (P > 0) {
            if fstr::ne(fstr::substr(STRING, P..=P), b" ") {
                spicelib::CHKIN(b"M2GETA", ctx)?;
                spicelib::SETMSG(b"The input string has been modified since it passed syntax validation in META/2. ", ctx);
                spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
                spicelib::CHKOUT(b"M2GETA", ctx)?;
                return Ok(());
            }
        }

        if (F < L) {
            if fstr::ne(fstr::substr(STRING, F..=F), b" ") {
                spicelib::CHKIN(b"M2GETA", ctx)?;
                spicelib::SETMSG(b"The input string has been modified since it passed syntax validation in META/2. ", ctx);
                spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
                spicelib::CHKOUT(b"M2GETA", ctx)?;
                return Ok(());
            }
        }

        if (fstr::eq(fstr::substr(STRING, B[I]..=B[I]), b" ")
            || fstr::eq(fstr::substr(STRING, E[I]..=E[I]), b" "))
        {
            spicelib::CHKIN(b"M2GETA", ctx)?;
            spicelib::SETMSG(
                b"The input string has been modified since it passed syntax validation in META/2. ",
                ctx,
            );
            spicelib::SIGERR(b"META/2(CORRUPTEDINPUTSTRING)", ctx)?;
            spicelib::CHKOUT(b"M2GETA", ctx)?;
            return Ok(());
        }
    }

    //
    // Next make sure there is room to hold everything.
    //
    W = intrinsics::LEN(WORD);

    if (W < ((E[2] - B[1]) + 1)) {
        spicelib::CHKIN(b"M2GETA", ctx)?;
        spicelib::SETMSG(
            b"There is not sufficient space in the output string to hold the requested word. ",
            ctx,
        );
        spicelib::SIGERR(b"META/2(INSUFFICIENTSPACE)", ctx)?;
        spicelib::CHKOUT(b"M2GETA", ctx)?;
        return Ok(());
    }

    //
    // Now do the actual assignment
    //
    fstr::assign(WORD, fstr::substr(STRING, B[1]..=E[2]));

    Ok(())
}
