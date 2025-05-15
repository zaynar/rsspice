//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      TXTOPS ( Text file, open scratch )
pub fn TXTOPS(UNIT: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;

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
        spicelib::CHKIN(b"TXTOPS", ctx)?;
    }

    spicelib::GETLUN(UNIT, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(*UNIT),
            form: Some(b"FORMATTED"),
            access: Some(b"SEQUENTIAL"),
            status: Some(b"SCRATCH"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    if (IOSTAT != 0) {
        spicelib::SETMSG(b"Could not scratch file. IOSTAT was #. ", ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
        spicelib::CHKOUT(b"TXTOPS", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"TXTOPS", ctx)?;
    Ok(())
}
