//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure NCODED ( Encode integer value into DP item )
pub fn NCODED(VALUE: i32, ITEM: &mut f64, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NCODED", ctx)?;
    }

    if (VALUE >= 0) {
        *ITEM = (VALUE as f64);
    } else {
        spicelib::SETMSG(b"Cannot encode #; must be non-negative.", ctx);
        spicelib::ERRINT(b"#", VALUE, ctx);
        spicelib::SIGERR(b"SPICE(OUTOFRANGE)", ctx)?;
    }

    spicelib::CHKOUT(b"NCODED", ctx)?;
    Ok(())
}

//$Procedure DCODED ( Decode integer value from DP item )
pub fn DCODED(ITEM: f64, VALUE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"DCODED", ctx)?;
    }

    *VALUE = (ITEM as i32);

    spicelib::CHKOUT(b"DCODED", ctx)?;
    Ok(())
}
