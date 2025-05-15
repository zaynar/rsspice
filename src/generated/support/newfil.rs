//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXCOM: i32 = 20;
const FILEN: i32 = 128;
const COMSIZ: i32 = 1024;
const ERRSIZ: i32 = 1760;
const STYSIZ: i32 = 120;
const MAXTRY: i32 = 20;

//$Procedure      NEWFIL ( Open a new file on the specified port )
pub fn NEWFIL(
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

    spicelib::CHKIN(b"NEWFIL", ctx)?;

    fstr::assign(&mut FNAME, b" ");
    fstr::assign(&mut NAME, b" ");
    fstr::assign(&mut THIS, b" ");

    FSTSTR(PATTRN, &mut FNAME, ctx)?;
    fstr::assign(&mut NAME, &FNAME);
    MORE = true;
    BADOPN = 0;

    while (BADOPN < MAXTRY) {
        while (spicelib::EXISTS(&NAME, ctx)? && MORE) {
            fstr::assign(&mut THIS, &NAME);
            fstr::assign(&mut NAME, b" ");
            NXTSTR(PATTRN, &THIS, &mut NAME);
            MORE = fstr::ne(&NAME, &FNAME);
        }

        if !MORE {
            fstr::assign(FILE, b" ");
            spicelib::SETMSG(b"It was not possible to create a # file as specified. All appropriately named files already exist.", ctx);
            spicelib::ERRCH(b"#", PORT, ctx);
            spicelib::SIGERR(b"CMLOOP(CANNOTMAKEFILE)", ctx)?;
            spicelib::CHKOUT(b"NEWFIL", ctx)?;
            return Ok(());
        } else {
            fstr::assign(FILE, &NAME);
        }

        NSPOPN(PORT, FILE, ctx)?;

        if spicelib::FAILED(ctx) {
            BADOPN = (BADOPN + 1);

            if (BADOPN < MAXTRY) {
                spicelib::RESET(ctx);
            }
        } else {
            spicelib::CHKOUT(b"NEWFIL", ctx)?;
            return Ok(());
        }
    }
    fstr::assign(FILE, b" ");
    spicelib::CHKOUT(b"NEWFIL", ctx)?;

    Ok(())
}
