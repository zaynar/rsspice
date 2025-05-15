//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LONGSZ: i32 = 900;

//$Procedure      LOGCHK ( Log file check )
pub fn LOGCHK(DEFALT: &[u8], USENAM: &mut [u8], DOLOG: &mut bool, ctx: &mut Context) {
    let mut LINE = [b' '; LONGSZ as usize];
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut START: i32 = 0;

    //
    // Spicelib Functions.
    //

    //
    // Until we know otherwise, we set the logname to the default
    // value and set action to "use a log file".
    //
    fstr::assign(USENAM, DEFALT);
    *DOLOG = true;
    START = 1;

    GETCML(&mut LINE, ctx);
    spicelib::FNDNWD(&LINE, START, &mut B, &mut E);

    while (B > 0) {
        START = (E + 1);

        if spicelib::EQSTR(fstr::substr(&LINE, B..=E), b"-nolog") {
            fstr::assign(USENAM, b" ");
            *DOLOG = false;
            return;
        } else if spicelib::EQSTR(fstr::substr(&LINE, B..=E), b"-log") {
            spicelib::FNDNWD(&LINE, START, &mut B, &mut E);

            if (E > B) {
                fstr::assign(USENAM, fstr::substr(&LINE, B..=E));
            }
            return;
        }

        spicelib::FNDNWD(&LINE, START, &mut B, &mut E);
    }
}
