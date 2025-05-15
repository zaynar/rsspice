//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;
const WDSIZE: i32 = 32;

//$Procedure ZZPLTCHK ( Private --- Platform Check )
pub fn ZZPLTCHK(OK: bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut STRBFF = [b' '; WDSIZE as usize];
    let mut VALUE = [b' '; WDSIZE as usize];
    let mut RTEBFF = [b' '; WDSIZE as usize];
    let mut BFF: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZPLTCHK", ctx)?;
    }

    //
    // Verify that the runtime environment's binary file format agrees
    // with the value listed in ZZPLATFM.
    //
    ZZPLATFM(b"FILE_FORMAT", &mut STRBFF, ctx);

    //
    // Determine what the runtime environment binary file format appears
    // to be.
    //
    ZZGETBFF(&mut BFF, ctx);
    ZZDDHGSD(b"BFF", BFF, &mut RTEBFF, ctx);

    //
    // Check results, signal SPICE(BUG) if a discrepancy appears.
    //
    if fstr::ne(&STRBFF, &RTEBFF) {
        SETMSG(b"This version of SPICELIB was originally packaged by NAIF for # hardware using # with the # compiler.  This environment has a binary file format of #; however the software is running on an environment that has a binary file format of #.  This is a severe problem and may be because the software package was intended for use on a different computer system.  It also may be the result of an improper port; please contact NAIF.", ctx);

        ZZPLATFM(b"SYSTEM", &mut VALUE, ctx);
        ERRCH(b"#", &VALUE, ctx);

        ZZPLATFM(b"O/S", &mut VALUE, ctx);
        ERRCH(b"#", &VALUE, ctx);

        ZZPLATFM(b"COMPILER", &mut VALUE, ctx);
        ERRCH(b"#", &VALUE, ctx);

        ERRCH(b"#", &STRBFF, ctx);

        if fstr::eq(&RTEBFF, b" ") {
            ERRCH(b"#", b"UNKNOWN", ctx);
        } else {
            ERRCH(b"#", &RTEBFF, ctx);
        }

        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZPLTCHK", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZPLTCHK", ctx)?;
    Ok(())
}
