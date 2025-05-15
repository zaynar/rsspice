//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const CRDLOC: i32 = 0;
const GRPOFF: i32 = -2;

//$Procedure      PODDGC ( Pod, duplicate group, character )
pub fn PODDGC(POD: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut POD = DummyCharArrayMut::new(POD, None, LBCELL..);
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
        spicelib::CHKIN(b"PODDGC", ctx)?;
    }

    //
    // How many spaces are needed? One for bookkeeping, and one for
    // each of the elements in the active group. (If there are no
    // elements, then one for future use.)
    //
    PODONC(POD.as_arg(), &mut OFFSET, &mut NUMBER, ctx)?;

    HAVE = spicelib::SIZEC(POD.as_arg(), ctx)?;
    NEED = ((spicelib::CARDC(POD.as_arg(), ctx)? + 1) + intrinsics::MAX0(&[1, NUMBER]));

    if (HAVE < NEED) {
        spicelib::SIGERR(b"SPICE(TOOMANYPEAS)", ctx)?;
        spicelib::CHKOUT(b"PODDGC", ctx)?;
        return Ok(());
    }

    //
    // Go ahead and create a new (empty) group.
    //
    PODBGC(POD.as_arg_mut(), ctx)?;

    //
    // Append the old group (still in the same place) to the pod.
    // (Somewhat incestuous, but practical.) Kids, don't try this
    // at home: you aren't supposed to know that existing groups
    // arent't changed by the addition of new ones.
    //
    PODAEC(
        POD.subarray((OFFSET + 1)).to_owned().as_arg(),
        NUMBER,
        POD.as_arg_mut(),
        ctx,
    )?;

    spicelib::CHKOUT(b"PODDGC", ctx)?;
    Ok(())
}
