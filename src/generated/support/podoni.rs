//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const GRPOFF: i32 = -2;

//$Procedure      PODONI ( Pod, offset and number, integer )
pub fn PODONI(
    POD: &[i32],
    OFFSET: &mut i32,
    NUMBER: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let POD = DummyArray::new(POD, LBCELL..);

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
        spicelib::CHKIN(b"PODONI", ctx)?;
    }

    //
    // The offset of the active group can be recovered directly from
    // the control area of the pod. The cardinality of the pod always
    // indicates the end of the active group.
    //
    DCODEI(POD[GRPOFF], OFFSET, ctx)?;
    *NUMBER = (spicelib::CARDI(POD.as_slice(), ctx)? - *OFFSET);

    spicelib::CHKOUT(b"PODONI", ctx)?;
    Ok(())
}
