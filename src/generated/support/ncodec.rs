//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure NCODEC ( Encode integer value into character item )
pub fn NCODEC(VALUE: i32, ITEM: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NCODEC", ctx)?;
    }

    if (VALUE >= 0) {
        spicelib::ENCHAR(VALUE, ITEM, ctx)?;
    } else {
        spicelib::SETMSG(b"Cannot encode #; must be non-negative.", ctx);
        spicelib::ERRINT(b"#", VALUE, ctx);
        spicelib::SIGERR(b"SPICE(OUTOFRANGE)", ctx)?;
    }

    spicelib::CHKOUT(b"NCODEC", ctx)?;
    Ok(())
}

//$Procedure DCODEC ( Decode integer value from character item )
pub fn DCODEC(ITEM: &[u8], VALUE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"DCODEC", ctx)?;
    }

    spicelib::DECHAR(ITEM, VALUE, ctx)?;

    spicelib::CHKOUT(b"DCODEC", ctx)?;
    Ok(())
}
