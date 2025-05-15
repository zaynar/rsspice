//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZGFDIQ ( GF, return distance between objects )
pub fn ZZGFDIQ(
    TARGID: i32,
    ET: f64,
    ABCORR: &[u8],
    OBSID: i32,
    DIST: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut LT: f64 = 0.0;
    let mut POS = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFDIQ", ctx)?;

    //
    // Get the position of the target relative to the observer.
    //
    SPKEZP(
        TARGID,
        ET,
        b"J2000",
        ABCORR,
        OBSID,
        POS.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFDIQ", ctx)?;
        return Ok(());
    }

    *DIST = VNORM(POS.as_slice());

    CHKOUT(b"ZZGFDIQ", ctx)?;
    Ok(())
}
