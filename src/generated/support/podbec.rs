//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      PODBEC ( Pod, begin and end, character )
pub fn PODBEC(
    POD: CharArray,
    BEGIN: &mut i32,
    END: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let POD = DummyCharArray::new(POD, None, LBCELL..);
    let mut NUMBER: i32 = 0;
    let mut OFFSET: i32 = 0;

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
        spicelib::CHKIN(b"PODBEC", ctx)?;
    }

    //
    // We'll cheat: why write the same code twice?
    //
    PODONC(POD.as_arg(), &mut OFFSET, &mut NUMBER, ctx)?;

    *BEGIN = (OFFSET + 1);
    *END = (OFFSET + NUMBER);

    spicelib::CHKOUT(b"PODBEC", ctx)?;
    Ok(())
}
