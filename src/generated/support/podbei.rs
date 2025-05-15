//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      PODBEI ( Pod, begin and end, integer )
pub fn PODBEI(
    POD: &[i32],
    BEGIN: &mut i32,
    END: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let POD = DummyArray::new(POD, LBCELL..);
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
        spicelib::CHKIN(b"PODBEI", ctx)?;
    }

    //
    // We'll cheat: why write the same code twice?
    //
    PODONI(POD.as_slice(), &mut OFFSET, &mut NUMBER, ctx)?;

    *BEGIN = (OFFSET + 1);
    *END = (OFFSET + NUMBER);

    spicelib::CHKOUT(b"PODBEI", ctx)?;
    Ok(())
}
