//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const CRDLOC: i32 = 0;
const GRPOFF: i32 = -2;

//$Procedure      PODDGD ( Pod, duplicate group, double precision )
pub fn PODDGD(POD: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut POD = DummyArrayMut::new(POD, LBCELL..);
    let mut HAVE: i32 = 0;
    let mut NEED: i32 = 0;
    let mut NUMBER: i32 = 0;
    let mut OFFSET: i32 = 0;

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
        spicelib::CHKIN(b"PODDGD", ctx)?;
    }

    //
    // How many spaces are needed? One for bookkeeping, and one for
    // each of the elements in the active group. (If there are no
    // elements, then one for future use.)
    //
    PODOND(POD.as_slice(), &mut OFFSET, &mut NUMBER, ctx)?;

    HAVE = spicelib::SIZED(POD.as_slice(), ctx)?;
    NEED = ((spicelib::CARDD(POD.as_slice(), ctx)? + 1) + intrinsics::MAX0(&[1, NUMBER]));

    if (HAVE < NEED) {
        spicelib::SIGERR(b"SPICE(TOOMANYPEAS)", ctx)?;
        spicelib::CHKOUT(b"PODDGD", ctx)?;
        return Ok(());
    }

    //
    // Go ahead and create a new (empty) group.
    //
    PODBGD(POD.as_slice_mut(), ctx)?;

    //
    // Append the old group (still in the same place) to the pod.
    // (Somewhat incestuous, but practical.) Kids, don't try this
    // at home: you aren't supposed to know that existing groups
    // arent't changed by the addition of new ones.
    //
    PODAED(
        &POD.subarray((OFFSET + 1)).to_vec(),
        NUMBER,
        POD.as_slice_mut(),
        ctx,
    )?;

    spicelib::CHKOUT(b"PODDGD", ctx)?;
    Ok(())
}
