//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXLEN: i32 = 255;

//$Procedure      EXPFNM_1 ( Expand a filename )
pub fn EXPFNM_1(INFIL: &[u8], OUTFIL: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BLANK: i32 = 0;
    let mut SLASH: i32 = 0;
    let mut WORD = [b' '; MAXLEN as usize];
    let mut DIR = [b' '; MAXLEN as usize];
    let mut INLEN: i32 = 0;
    let mut WRDLEN: i32 = 0;
    let mut DIRLEN: i32 = 0;
    let mut OUTLEN: i32 = 0;
    let mut KEEP: i32 = 0;
    let mut NEED: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"EXPFNM_1", ctx)?;
    }

    //
    // If the input filename is blank, that's an error.
    //
    if fstr::eq(INFIL, b" ") {
        spicelib::SETMSG(b"The input filename \'#\' was blank.", ctx);
        spicelib::ERRCH(b"#", INFIL, ctx);
        spicelib::SIGERR(b"SPICE(BADFILENAME)", ctx)?;
        spicelib::CHKOUT(b"EXPFNM_1", ctx)?;
        return Ok(());
    }

    //
    // If there are blanks anywhere in the filename, SPICELIB
    // considers the filename to be invalid.
    //
    BLANK = spicelib::POS(fstr::substr(INFIL, 1..=spicelib::RTRIM(INFIL)), b" ", 1);

    if (BLANK != 0) {
        spicelib::SETMSG(b"The input filename \'#\' had blank characters in it.", ctx);
        spicelib::ERRCH(b"#", INFIL, ctx);
        spicelib::SIGERR(b"SPICE(BADFILENAME)", ctx)?;
        spicelib::CHKOUT(b"EXPFNM_1", ctx)?;
        return Ok(());
    }

    //
    // Look for a slash in the filename.
    //
    SLASH = spicelib::POS(INFIL, b"/", 1);

    //
    // If we found a slash in a position other than the first
    // character position, we want to examine the word that
    // comes before it just in case it is an environment
    // variable.
    //
    if (SLASH > 1) {
        fstr::assign(&mut WORD, fstr::substr(INFIL, 1..=(SLASH - 1)));

        ctx.getenv(&WORD, &mut DIR);

        //
        // If the word was an environment variable, then construct
        // the expanded filename. If it wasn't, just return the original
        // input filename.
        //
        if fstr::ne(&DIR, b" ") {
            fstr::assign(OUTFIL, INFIL);

            INLEN = spicelib::RTRIM(INFIL);
            WRDLEN = spicelib::RTRIM(&WORD);
            DIRLEN = spicelib::RTRIM(&DIR);
            OUTLEN = intrinsics::LEN(OUTFIL);
            KEEP = (INLEN - WRDLEN);
            NEED = (KEEP + DIRLEN);

            //
            // If the output filename length is not long enough for
            // the substitution, signal an error. Otherwise, substitute
            // in the new value.
            //
            if (NEED > OUTLEN) {
                spicelib::SETMSG(b"The expanded filename for the input filename \'#\' exceeded the length of the output filename. The expanded name was # characters too long.", ctx);
                spicelib::ERRCH(b"#", INFIL, ctx);
                spicelib::ERRINT(b"#", (NEED - OUTLEN), ctx);
                spicelib::SIGERR(b"SPICE(STRINGTOOSMALL)", ctx)?;
                spicelib::CHKOUT(b"EXPFNM_1", ctx)?;
                return Ok(());
            } else {
                spicelib::REPSUB(
                    INFIL,
                    1,
                    (SLASH - 1),
                    fstr::substr(&DIR, 1..=spicelib::RTRIM(&DIR)),
                    OUTFIL,
                    ctx,
                )?;
            }
        } else {
            fstr::assign(OUTFIL, INFIL);
        }
    } else {
        //
        // No slashes are in the filename, so it's just an easy case.
        //
        // It's possible that the entire filename is an environment
        // variable. If it's not, then just return the input filename.
        //

        ctx.getenv(INFIL, OUTFIL);

        if fstr::eq(OUTFIL, b" ") {
            fstr::assign(OUTFIL, INFIL);
        }
    }

    spicelib::CHKOUT(b"EXPFNM_1", ctx)?;
    Ok(())
}
