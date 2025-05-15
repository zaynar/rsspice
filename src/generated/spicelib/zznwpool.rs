//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBPOOL: i32 = -5;

//$Procedure ZZNWPOOL ( Private: notify watchers of update )
pub fn ZZNWPOOL(
    VARNAM: &[u8],
    WTVARS: CharArray,
    WTPTRS: &[i32],
    WTPOOL: &[i32],
    WTAGNT: CharArray,
    AGTWRK: CharArrayMut,
    NOTIFY: CharArrayMut,
    AGENTS: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let WTVARS = DummyCharArray::new(WTVARS, None, LBCELL..);
    let WTPTRS = DummyArray::new(WTPTRS, 1..);
    let WTPOOL = DummyArray2D::new(WTPOOL, 1..=2, LBPOOL..);
    let WTAGNT = DummyCharArray::new(WTAGNT, None, 1..);
    let mut AGTWRK = DummyCharArrayMut::new(AGTWRK, None, LBCELL..);
    let mut NOTIFY = DummyCharArrayMut::new(NOTIFY, None, LBCELL..);
    let mut AGENTS = DummyCharArrayMut::new(AGENTS, None, LBCELL..);

    //
    // SPICELIB functions
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZNWPOOL", ctx)?;

    //
    // Fetch the agents watching VARNAM into the set NOTIFY.
    //
    ZZGAPOOL(
        VARNAM,
        WTVARS.as_arg(),
        WTPTRS.as_slice(),
        WTPOOL.as_slice(),
        WTAGNT.as_arg(),
        NOTIFY.as_arg_mut(),
        ctx,
    )?;

    //
    // Compute the union of NOTIFY and the agent list AGENTS.
    // Place the result in the workspace set AGTWRK; then copy
    // the result to AGENTS.
    //
    UNIONC(NOTIFY.as_arg(), AGENTS.as_arg(), AGTWRK.as_arg_mut(), ctx)?;
    COPYC(AGTWRK.as_arg(), AGENTS.as_arg_mut(), ctx)?;

    CHKOUT(b"ZZNWPOOL", ctx)?;
    Ok(())
}
