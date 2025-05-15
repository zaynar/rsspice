//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZGFRRQ ( Private - GF, range rate between objects )
pub fn ZZGFRRQ(
    ET: f64,
    TARG: i32,
    OBS: i32,
    ABCORR: &[u8],
    VALUE: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut LT: f64 = 0.0;
    let mut STATE = StackArray::<f64, 6>::new(1..=6);
    let mut REF = [b' '; 5 as usize];

    //
    // SPICELIB functions.
    //

    //
    // Local Variables.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZGFRRQ", ctx)?;
    }

    //
    // We just want the range rate of TARG relative to OBS.
    // This calculation is invariant with respect to reference
    // frame; we use 'J2000'.
    //
    fstr::assign(&mut REF, b"J2000");
    SPKEZ(
        TARG,
        ET,
        &REF,
        ABCORR,
        OBS,
        STATE.as_slice_mut(),
        &mut LT,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZGFRRQ", ctx)?;
        return Ok(());
    }

    //
    // Calculate the derivative from the STATE vector.
    //
    *VALUE = DVNORM(STATE.as_slice());

    //
    // All done.
    //
    CHKOUT(b"ZZGFRRQ", ctx)?;
    Ok(())
}
