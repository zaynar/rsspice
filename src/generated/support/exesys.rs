//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure   EXESYS  ( Execute system command )
pub fn EXESYS(CMD: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut STATUS: i32 = 0;

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
        spicelib::CHKIN(b"EXESYS", ctx)?;
    }

    ctx.system(fstr::substr(CMD, 1..=spicelib::RTRIM(CMD)), &mut STATUS);

    if (STATUS != 0) {
        //
        // Uh, we've got a problem.
        //
        spicelib::SETMSG(
            b"The \"system\" call returned code # in response to command #.",
            ctx,
        );
        spicelib::ERRINT(b"#", STATUS, ctx);
        spicelib::ERRCH(b"#", CMD, ctx);
        spicelib::SIGERR(b"SPICE(SYSTEMCALLFAILED)", ctx)?;
        spicelib::CHKOUT(b"EXESYS", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"EXESYS", ctx)?;
    Ok(())
}
