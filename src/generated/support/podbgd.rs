//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const CRDLOC: i32 = 0;
const GRPOFF: i32 = -2;

//$Procedure      PODBGD ( Pod, begin group, double precision )
pub fn PODBGD(POD: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut POD = DummyArrayMut::new(POD, LBCELL..);
    let mut HAVE: i32 = 0;
    let mut NEED: i32 = 0;
    let mut PODCRD: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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
        spicelib::CHKIN(b"PODBGD", ctx)?;
    }

    //
    // There must be at least two spaces at the end of the pod:
    // one for bookkeeping, and one for the first element of
    // the new group.
    //
    HAVE = spicelib::SIZED(POD.as_slice(), ctx)?;
    NEED = (spicelib::CARDD(POD.as_slice(), ctx)? + 2);

    if (HAVE < NEED) {
        spicelib::SIGERR(b"SPICE(TOOMANYPEAS)", ctx)?;
        spicelib::CHKOUT(b"PODBGD", ctx)?;
        return Ok(());
    }

    //
    // Okay: go ahead and create the group. The offset of the active
    // group is stored in the first empty slot of the pod; when the
    // new group is removed, this will be reinstated as the offset of
    // the active group.
    //
    PODCRD = (spicelib::CARDD(POD.as_slice(), ctx)? + 1);
    POD[PODCRD] = POD[GRPOFF];

    //
    // This requires the cardinality of the pod to increase by one.
    //
    spicelib::SCARDD(
        (spicelib::CARDD(POD.as_slice(), ctx)? + 1),
        POD.as_slice_mut(),
        ctx,
    )?;

    //
    // Surprise! The new cardinality is the same as the offset of
    // the new group!
    //
    POD[GRPOFF] = POD[CRDLOC];

    spicelib::CHKOUT(b"PODBGD", ctx)?;
    Ok(())
}
