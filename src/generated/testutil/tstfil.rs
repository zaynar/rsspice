//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FILEN: i32 = 127;
const MAXTRY: i32 = 20;

//$Procedure      TSTFIL ( Open a new file on the specified port )
//
pub fn TSTFIL(
    PATTRN: &[u8],
    PORT: &[u8],
    FILE: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut FNAME = [b' '; FILEN as usize];
    let mut NAME = [b' '; FILEN as usize];
    let mut THIS = [b' '; FILEN as usize];
    let mut BADOPN: i32 = 0;
    let mut MORE: bool = false;

    //

    //

    //
    // Spicelib routines.
    //

    //
    // Local Parameters
    //

    spicelib::CHKIN(b"TSTFIL", ctx)?;

    fstr::assign(&mut FNAME, b" ");
    fstr::assign(&mut NAME, b" ");
    fstr::assign(&mut THIS, b" ");

    support::FSTSTR(PATTRN, &mut FNAME, ctx)?;
    fstr::assign(&mut NAME, &FNAME);
    MORE = true;
    BADOPN = 0;

    while (BADOPN < MAXTRY) {
        //
        // Look for a file name that does not already exist.
        //
        while (spicelib::EXISTS(&NAME, ctx)? && MORE) {
            fstr::assign(&mut THIS, &NAME);
            fstr::assign(&mut NAME, b" ");
            support::NXTSTR(PATTRN, &THIS, &mut NAME);
            MORE = fstr::ne(&NAME, &FNAME);
        }

        if !MORE {
            fstr::assign(FILE, b" ");
            spicelib::SETMSG(b"It was not possible to create a # file as specified. All appropriately named files already exist.", ctx);
            spicelib::ERRCH(b"#", PORT, ctx);
            spicelib::SIGERR(b"CMLOOP(CANNOTMAKEFILE)", ctx)?;
            spicelib::CHKOUT(b"TSTFIL", ctx)?;
            return Ok(());
        } else {
            fstr::assign(FILE, &NAME);
        }

        //
        // Ok.  We've got a good candidate, try to attach it to the
        // specified port.
        //
        TSTOPN(PORT, FILE, ctx)?;

        if spicelib::FAILED(ctx) {
            BADOPN = (BADOPN + 1);
            //
            // We will try a few more times on the off chance that
            // some other program used the same name first.  This
            // is not likely, file protection problems or PATTRN
            // specifications are a more probable cause of the trouble,
            // but we try anyway.
            //

            if (BADOPN < MAXTRY) {
                spicelib::RESET(ctx);
            }
        } else {
            //
            // We were successful in opening the port with the
            // specified name.  We can quit now.
            //
            TSTSLF(FILE, ctx);
            spicelib::CHKOUT(b"TSTFIL", ctx)?;
            return Ok(());
        }
    }
    //
    // If you get to this point, a file was not succesfully
    // attached to PORT.  But TSTIO has already diagnosed
    // the problem as much as we're going to.  Just set FILE
    // to a blank and return.
    //
    fstr::assign(FILE, b" ");
    spicelib::CHKOUT(b"TSTFIL", ctx)?;

    Ok(())
}
