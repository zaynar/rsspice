//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FNMLEN: i32 = 255;

//$Procedure      NEWFIL_1 ( Generate a filename that does not exist )
pub fn NEWFIL_1(PATTRN: &[u8], FILE: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FNAME = [b' '; FNMLEN as usize];
    let mut NAME = [b' '; FNMLEN as usize];
    let mut THIS = [b' '; FNMLEN as usize];
    let mut DONE: bool = false;
    let mut NOMORE: bool = false;

    //
    // Spicelib routines.
    //
    //
    // Local Parameters
    //
    // Length of a filename.
    //
    //
    // Local Variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NEWFIL_1", ctx)?;
    }

    fstr::assign(&mut FNAME, b" ");
    //
    // Get the first filename in the pattern space.
    //
    FSTSTR(PATTRN, &mut FNAME, ctx)?;

    fstr::assign(&mut NAME, &FNAME);
    NOMORE = false;
    DONE = false;
    //
    // Look for a file name that does not already exist.
    //
    while !DONE {
        fstr::assign(&mut THIS, &NAME);
        fstr::assign(&mut NAME, b" ");

        NXTSTR(PATTRN, &THIS, &mut NAME);

        DONE = fstr::eq(&NAME, &FNAME);

        if !DONE {
            if !spicelib::EXISTS(&NAME, ctx)? {
                DONE = true;
            }
        } else {
            NOMORE = true;
        }
    }

    if NOMORE {
        fstr::assign(FILE, b" ");
        spicelib::SETMSG(b"It was not possible to create a file name using \'#\' as the pattern. All of the file names that can be generated from this pattern already exist.", ctx);
        spicelib::ERRCH(b"#", PATTRN, ctx);
        spicelib::SIGERR(b"SPICE(CANNOTMAKEFILE)", ctx)?;
        spicelib::CHKOUT(b"NEWFIL_1", ctx)?;
        return Ok(());
    }

    fstr::assign(FILE, &NAME);

    spicelib::CHKOUT(b"NEWFIL_1", ctx)?;
    Ok(())
}
