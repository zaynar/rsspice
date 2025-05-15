//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;

//$Procedure ZZBODVCD ( Get d.p. kernel variable for body, with bypass )
pub fn ZZBODVCD(
    BODYID: i32,
    ITEM: &[u8],
    MAXN: i32,
    VARCTR: &mut [i32],
    N: &mut i32,
    VALUES: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut VARCTR = DummyArrayMut::new(VARCTR, 1..=CTRSIZ);
    let mut VALUES = DummyArrayMut::new(VALUES, 1..);
    let mut UPDATE: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZBODVCD", ctx)?;

    //
    // See whether the kernel pool state has changed since the
    // user counter was set. Update the user counter if so.
    //
    ZZPCTRCK(VARCTR.as_slice_mut(), &mut UPDATE, ctx);

    //
    // If the pool was updated, or if we're looking at a new variable,
    // update the kernel variable values, size, and found flag.
    // Otherwise do nothing.
    //
    if UPDATE {
        BODVCD(BODYID, ITEM, MAXN, N, VALUES.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            *N = 0;
        }
    }

    CHKOUT(b"ZZBODVCD", ctx)?;
    Ok(())
}
