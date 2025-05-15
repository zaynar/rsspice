//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBPOOL: i32 = -5;

//$Procedure ZZGAPOOL ( Private: get agent set for watched variable )
pub fn ZZGAPOOL(
    VARNAM: &[u8],
    WTVARS: CharArray,
    WTPTRS: &[i32],
    WTPOOL: &[i32],
    WTAGNT: CharArray,
    AGTSET: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let WTVARS = DummyCharArray::new(WTVARS, None, LBCELL..);
    let WTPTRS = DummyArray::new(WTPTRS, 1..);
    let WTPOOL = DummyArray2D::new(WTPOOL, 1..=2, LBPOOL..);
    let WTAGNT = DummyCharArray::new(WTAGNT, None, 1..);
    let mut AGTSET = DummyCharArrayMut::new(AGTSET, None, LBCELL..);
    let mut LOC: i32 = 0;
    let mut NFETCH: i32 = 0;
    let mut NODE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGAPOOL", ctx)?;

    //
    // The output agent set is empty until we find any
    // agents.
    //
    SCARDC(0, AGTSET.as_arg_mut(), ctx)?;

    //
    // Find the location of VARNAM in the set of watched
    // variables.
    //
    LOC = BSRCHC(VARNAM, CARDC(WTVARS.as_arg(), ctx)?, WTVARS.subarray(1));

    if (LOC == 0) {
        //
        // This variable is not watched. The agent set is
        // empty.
        //
        CHKOUT(b"ZZGAPOOL", ctx)?;
        return Ok(());
    }

    //
    // Set NODE to the head node of the agent list for VARNAM.
    // Traverse the agent list for VARNAM. Collect the agents
    // as an unordered list, then turn the list into a set.
    //
    NODE = WTPTRS[LOC];
    NFETCH = 0;

    while (NODE > 0) {
        NFETCH = (NFETCH + 1);
        fstr::assign(AGTSET.get_mut(NFETCH), WTAGNT.get(NODE));

        NODE = LNKNXT(NODE, WTPOOL.as_slice(), ctx)?;
    }

    VALIDC(
        SIZEC(AGTSET.as_arg(), ctx)?,
        NFETCH,
        AGTSET.as_arg_mut(),
        ctx,
    )?;

    CHKOUT(b"ZZGAPOOL", ctx)?;
    Ok(())
}
