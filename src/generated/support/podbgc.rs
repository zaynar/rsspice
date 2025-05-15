//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const CRDLOC: i32 = 0;
const GRPOFF: i32 = -2;

//$Procedure      PODBGC ( Pod, begin group, character )
pub fn PODBGC(POD: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut POD = DummyCharArrayMut::new(POD, None, LBCELL..);
    let mut HAVE: i32 = 0;
    let mut NEED: i32 = 0;

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
        spicelib::CHKIN(b"PODBGC", ctx)?;
    }

    //
    // There must be at least two spaces at the end of the pod:
    // one for bookkeeping, and one for the first element of
    // the new group.
    //
    HAVE = spicelib::SIZEC(POD.as_arg(), ctx)?;
    NEED = (spicelib::CARDC(POD.as_arg(), ctx)? + 2);

    if (HAVE < NEED) {
        spicelib::SIGERR(b"SPICE(TOOMANYPEAS)", ctx)?;
        spicelib::CHKOUT(b"PODBGC", ctx)?;
        return Ok(());
    }

    //
    // Okay: go ahead and create the group. The offset of the active
    // group is stored in the first empty slot of the pod; when the
    // new group is removed, this will be reinstated as the offset of
    // the active group.
    //
    let val = POD.get(GRPOFF).to_vec();
    fstr::assign(POD.get_mut((spicelib::CARDC(POD.as_arg(), ctx)? + 1)), &val);

    //
    // This requires the cardinality of the pod to increase by one.
    //
    spicelib::SCARDC(
        (spicelib::CARDC(POD.as_arg(), ctx)? + 1),
        POD.as_arg_mut(),
        ctx,
    )?;

    //
    // Surprise! The new cardinality is the same as the offset of
    // the new group!
    //
    let val = POD.get(CRDLOC).to_vec();
    fstr::assign(POD.get_mut(GRPOFF), &val);

    spicelib::CHKOUT(b"PODBGC", ctx)?;
    Ok(())
}
