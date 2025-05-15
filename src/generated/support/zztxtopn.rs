//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZTXTOPN ( Private Routine -- Text file, open new )
pub fn ZZTXTOPN(
    FNAME: &[u8],
    UNIT: &mut i32,
    SUCCSS: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
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
        spicelib::CHKIN(b"ZZTXTOPN", ctx)?;
    }

    *SUCCSS = true;

    if fstr::eq(FNAME, b" ") {
        *SUCCSS = false;
        spicelib::SETMSG(b"A blank string is unacceptable as a file name", ctx);
        spicelib::SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
        spicelib::CHKOUT(b"ZZTXTOPN", ctx)?;
        return Ok(());
    }

    spicelib::GETLUN(UNIT, ctx)?;

    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(*UNIT),
            file: Some(FNAME),
            form: Some(b"FORMATTED"),
            access: Some(b"SEQUENTIAL"),
            status: Some(b"NEW"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    if (IOSTAT != 0) {
        *SUCCSS = false;
        spicelib::CHKOUT(b"ZZTXTOPN", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"ZZTXTOPN", ctx)?;
    Ok(())
}
