//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;

//$Procedure ZZCVPOOL ( Private---Check variable update, with counter )
pub fn ZZCVPOOL(
    AGENT: &[u8],
    USRCTR: &mut [i32],
    UPDATE: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut USRCTR = DummyArrayMut::new(USRCTR, 1..=CTRSIZ);

    //
    // SPICELIB functions.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Check/update counter.
    //
    ZZPCTRCK(USRCTR.as_slice_mut(), UPDATE, ctx);

    //
    // If counter was updated, check in and call CVPOOL.
    //
    if *UPDATE {
        CHKIN(b"ZZCVPOOL", ctx)?;

        CVPOOL(AGENT, UPDATE, ctx)?;

        CHKOUT(b"ZZCVPOOL", ctx)?;
    }

    Ok(())
}
