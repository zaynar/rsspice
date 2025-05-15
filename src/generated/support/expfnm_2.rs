//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const ENVLEN: i32 = 32;
pub const FNMLEN: i32 = 255;

//$Procedure      EXPFNM_2 ( Expand a filename )
pub fn EXPFNM_2(INSTR: &[u8], OUTFIL: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut MYENV = [b' '; ENVLEN as usize];
    let mut MYFIL = [b' '; FNMLEN as usize];
    let mut MYVAL = [b' '; FNMLEN as usize];
    let mut BLANK: i32 = 0;
    let mut DOLLAR: i32 = 0;
    let mut INLEN: i32 = 0;
    let mut NEED: i32 = 0;
    let mut OUTLEN: i32 = 0;
    let mut VALLEN: i32 = 0;
    let mut SLASH: i32 = 0;
    let mut VARLEN: i32 = 0;

    //

    //
    // SPICELIB functions
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
        spicelib::CHKIN(b"EXPFNM_2", ctx)?;
    }
    //
    // If the input filename is blank, that's an error.
    //
    if fstr::eq(INSTR, b" ") {
        fstr::assign(OUTFIL, b" ");
        spicelib::SETMSG(b"The input filename \'#\' was blank.", ctx);
        spicelib::ERRCH(b"#", INSTR, ctx);
        spicelib::SIGERR(b"SPICE(BADFILENAME)", ctx)?;
        spicelib::CHKOUT(b"EXPFNM_2", ctx)?;
        return Ok(());
    }
    //
    // We know the input was not blank, so left justify it and
    // check for embedded blanks.
    //
    spicelib::LJUST(INSTR, &mut MYFIL);

    BLANK = spicelib::POS(fstr::substr(&MYFIL, 1..=spicelib::RTRIM(&MYFIL)), b" ", 1);

    if (BLANK != 0) {
        fstr::assign(OUTFIL, b" ");
        spicelib::SETMSG(b"The input filename \'#\' contained embedded blanks.", ctx);
        spicelib::ERRCH(b"#", &MYFIL, ctx);
        spicelib::SIGERR(b"SPICE(BADFILENAME)", ctx)?;
        spicelib::CHKOUT(b"EXPFNM_2", ctx)?;
        return Ok(());
    }
    //
    // We have two cases that we need to consider:
    //
    //    1) The input file does not contain a dollar sign. This
    //       indicates that it is a complete filename;
    //
    //    2) The input file has a dollar sign as the first character.
    //       This indicates that the input filename has its full name,
    //       or leading path components, specified by the value of an
    //       environment variable. In this case, we get the environment
    //       variable's value and replace the environment variable in
    //       the input filename.
    //
    // We deal with each of these cases, in order, below.
    //
    DOLLAR = spicelib::POS(&MYFIL, b"$", 1);

    if (DOLLAR == 0) {
        //
        // The input is assumed to be an actual filename, so set the
        // output to be the input.
        //
        fstr::assign(OUTFIL, INSTR);
    } else if (DOLLAR == 1) {
        //
        // The input is assumed to contain the name of an environment
        // variable whose value contains a complete path name to a
        // file or the leading path elements that will create a complete
        // path name to a file. To find out which, we look for a forward
        // slash. If there is one, everything between the dollar sign and
        // the first forward slash, noninclusive, is the name of the
        // environment variable. If there are no slashes, the entire
        // input name is the name of the environment variable.
        //
        SLASH = spicelib::POS(&MYFIL, b"/", 2);

        if (SLASH == 0) {
            VARLEN = spicelib::RTRIM(&MYFIL);
        } else {
            VARLEN = (SLASH - 1);
        }

        if (VARLEN > ENVLEN) {
            fstr::assign(OUTFIL, b" ");
            spicelib::SETMSG(b"The environment variable name \'#\' is too long. The maximum length for an environment variable name is #.", ctx);
            spicelib::ERRCH(b"#", fstr::substr(&MYFIL, 2..=(SLASH - 1)), ctx);
            spicelib::ERRINT(b"#", ENVLEN, ctx);
            spicelib::SIGERR(b"SPICE(STRINGTOOSMALL)", ctx)?;
            spicelib::CHKOUT(b"EXPFNM_2", ctx)?;
            return Ok(());
        }
        //
        // Remember to skip the dollar sign.
        //
        fstr::assign(&mut MYENV, fstr::substr(&MYFIL, 2..=VARLEN));

        //
        // Try to get the value of the environment variable. If the
        // environment variable does not exist, a blank string is
        // returned.
        //
        ZZGETENV(&MYENV, &mut MYVAL, ctx)?;
        //
        // If we got something, use it. We don't make any value
        // judgements.
        //
        if fstr::eq(&MYVAL, b" ") {
            fstr::assign(OUTFIL, b" ");
            spicelib::SETMSG(b"The environment variable \'#\' was not defined.", ctx);
            spicelib::ERRCH(b"#", &MYENV, ctx);
            spicelib::SIGERR(b"SPICE(NOENVVARIABLE)", ctx)?;
            spicelib::CHKOUT(b"EXPFNM_2", ctx)?;
            return Ok(());
        }

        INLEN = spicelib::RTRIM(fstr::substr(&MYFIL, 2..));
        VALLEN = spicelib::RTRIM(&MYVAL);
        OUTLEN = intrinsics::LEN(OUTFIL);
        NEED = ((INLEN - VARLEN) + VALLEN);
        //
        // If the output filename length is not long enough for
        // the substitution, signal an error. Otherwise, substitute
        // in the new value.
        //
        if (NEED > OUTLEN) {
            fstr::assign(OUTFIL, b" ");
            spicelib::SETMSG(b"The expanded filename for the input filename \'#\' exceeded the length of the output filename. The expanded name was # characters too long.", ctx);
            spicelib::ERRCH(b"#", &MYFIL, ctx);
            spicelib::ERRINT(b"#", (NEED - OUTLEN), ctx);
            spicelib::SIGERR(b"SPICE(STRINGTOOSMALL)", ctx)?;
            spicelib::CHKOUT(b"EXPFNM_2", ctx)?;
            return Ok(());
        }

        spicelib::REPSUB(
            &MYFIL,
            1,
            VARLEN,
            fstr::substr(&MYVAL, 1..=VALLEN),
            OUTFIL,
            ctx,
        )?;
    } else {
        //
        // There was a dollar sign in a position other than the first
        // nonblank position of the input filename. We do not allow
        // this. If an input filename contains a dollar sign, it must
        // be in the first nonblank position.
        //
        fstr::assign(OUTFIL, b" ");
        spicelib::SETMSG(b"The input filename \'#\' contained a dollar sign ($) that was not in the first nonblank position; this is not allowed. See the subroutine EXPFNM_2 for details.", ctx);
        spicelib::ERRCH(b"#", &MYFIL, ctx);
        spicelib::SIGERR(b"SPICE(BADFILENAME)", ctx)?;
        spicelib::CHKOUT(b"EXPFNM_2", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"EXPFNM_2", ctx)?;

    Ok(())
}
