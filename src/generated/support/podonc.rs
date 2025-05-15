//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const GRPOFF: i32 = -2;

//$Procedure      PODONC ( Pod, offset and number, character )
pub fn PODONC(
    POD: CharArray,
    OFFSET: &mut i32,
    NUMBER: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let POD = DummyCharArray::new(POD, None, LBCELL..);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"PODONC", ctx)?;
    }

    //
    // The offset of the active group can be recovered directly from
    // the control area of the pod. The cardinality of the pod always
    // indicates the end of the active group.
    //
    DCODEC(&POD[GRPOFF], OFFSET, ctx)?;
    *NUMBER = (spicelib::CARDC(POD.as_arg(), ctx)? - *OFFSET);

    spicelib::CHKOUT(b"PODONC", ctx)?;
    Ok(())
}
