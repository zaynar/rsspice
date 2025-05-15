//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure NCODEI ( Encode integer value into integer item )
pub fn NCODEI(VALUE: i32, ITEM: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NCODEI", ctx)?;
    }

    if (VALUE >= 0) {
        *ITEM = VALUE;
    } else {
        spicelib::SETMSG(b"Cannot encode #; must be non-negative.", ctx);
        spicelib::ERRINT(b"#", VALUE, ctx);
        spicelib::SIGERR(b"SPICE(OUTOFRANGE)", ctx)?;
    }

    spicelib::CHKOUT(b"NCODEI", ctx)?;
    Ok(())
}

//$Procedure DCODEI ( Decode integer value from integer item )
pub fn DCODEI(ITEM: i32, VALUE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"DCODEI", ctx)?;
    }

    *VALUE = ITEM;

    spicelib::CHKOUT(b"DCODEI", ctx)?;
    Ok(())
}
