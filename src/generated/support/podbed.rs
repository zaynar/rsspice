//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      PODBED ( Pod, begin and end, double precision )
pub fn PODBED(
    POD: &[f64],
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
        spicelib::CHKIN(b"PODBED", ctx)?;
    }

    //
    // We'll cheat: why write the same code twice?
    //
    PODOND(POD.as_slice(), &mut OFFSET, &mut NUMBER, ctx)?;

    *BEGIN = (OFFSET + 1);
    *END = (OFFSET + NUMBER);

    spicelib::CHKOUT(b"PODBED", ctx)?;
    Ok(())
}
