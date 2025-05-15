//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NONE: i32 = 0;
const COMBUF: i32 = (NONE + 1);
const KEYBRD: i32 = (COMBUF + 1);
const INPFIL: i32 = (KEYBRD + 1);
const LNSIZE: i32 = 255;

//$Procedure      CMSTUP ( Command Loop Startup )
pub fn CMSTUP(ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut COMMND = [b' '; LNSIZE as usize];
    let mut COMLIN = [b' '; LNSIZE as usize];
    let mut FILE = [b' '; LNSIZE as usize];
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut START: i32 = 0;
    let mut HAVGO: bool = false;
    let mut DOBTCH: bool = false;
    let mut HAVFIL: bool = false;

    //
    // Command loop fucntions
    //

    //
    // Below are the various sources from which
    // commands might come.
    //
    // NONE
    // COMBUF
    // KEYBRD
    // INPFIL
    //

    GETCML(&mut COMLIN, ctx);

    START = 1;
    HAVGO = false;
    DOBTCH = false;
    HAVFIL = false;

    spicelib::FNDNWD(&COMLIN, START, &mut B, &mut E);

    while (B > 0) {
        if fstr::eq(fstr::substr(&COMLIN, B..=E), b"-b") {
            DOBTCH = true;
        } else if fstr::eq(fstr::substr(&COMLIN, B..=E), b"-start") {
            HAVGO = true;
        } else if (HAVGO && !HAVFIL) {
            fstr::assign(&mut FILE, fstr::substr(&COMLIN, B..=E));
            HAVFIL = true;
        }

        START = (E + 1);
        spicelib::FNDNWD(&COMLIN, START, &mut B, &mut E);
    }

    //
    // If we have a batch flag, notify NXTCOM
    //
    if DOBTCH {
        DOBTCH = SETBAT(ctx);
    }

    if (HAVGO && HAVFIL) {
        TRNLAT(b"START", &mut COMMND, ctx);
        spicelib::SUFFIX(&FILE, 1, &mut COMMND);
        PUTCOM(&COMMND, COMBUF, ctx)?;
    }

    Ok(())
}
