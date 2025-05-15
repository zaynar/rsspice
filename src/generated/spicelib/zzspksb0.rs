//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZSPKSB0 ( S/P Kernel, solar system barycenter )
pub fn ZZSPKSB0(
    TARG: i32,
    ET: f64,
    REF: &[u8],
    STARG: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut STARG = DummyArrayMut::new(STARG, 1..=6);
    let mut LT: f64 = 0.0;
    let mut BARY: i32 = 0;

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
    } else {
        CHKIN(b"ZZSPKSB0", ctx)?;
    }

    BARY = 0;

    ZZSPKGO0(TARG, ET, REF, BARY, STARG.as_slice_mut(), &mut LT, ctx)?;

    CHKOUT(b"ZZSPKSB0", ctx)?;
    Ok(())
}
