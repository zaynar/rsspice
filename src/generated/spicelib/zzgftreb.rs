//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZGFTREB ( Geometry finder: return body axes )
pub fn ZZGFTREB(BODY: i32, AXES: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut AXES = DummyArrayMut::new(AXES, 1..=3);
    let mut N: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFTREB", ctx)?;

    //
    // Look up BODY radii in the kernel pool.
    //
    BODVCD(BODY, b"RADII", 3, &mut N, AXES.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFTREB", ctx)?;
        return Ok(());
    }

    if (N != 3) {
        SETMSG(
            b"Only # axes were found  for ID #. Three axes expected.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        ERRINT(b"#", BODY, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZGFTREB", ctx)?;
        return Ok(());
    } else {
        for I in 1..=3 {
            if (AXES[I] <= 0.0) {
                SETMSG(b"Degenerate case. The # axis of body # is negative or zero.  Please check the text PCK file. You should fix the # component of the kernel pool variable  BODY#_RADII. ", ctx);

                ERRINT(b"#", I, ctx);
                ERRINT(b"#", BODY, ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", BODY, ctx);
                SIGERR(b"SPICE(BADAXISLENGTH)", ctx)?;
                CHKOUT(b"ZZGFTREB", ctx)?;
                return Ok(());
            }
        }
    }

    CHKOUT(b"ZZGFTREB", ctx)?;
    Ok(())
}
