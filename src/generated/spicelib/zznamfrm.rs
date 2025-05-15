//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;

//$Procedure ZZNAMFRM ( Frame name to ID translation, with bypass )
pub fn ZZNAMFRM(
    USRCTR: &mut [i32],
    SAVNAM: &mut [u8],
    SAVCDE: &mut i32,
    FRNAME: &[u8],
    FRCODE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut USRCTR = DummyArrayMut::new(USRCTR, 1..=CTRSIZ);
    let mut UPDATE: bool = false;

    //
    // SPICE functions.
    //

    //
    // Local variables.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Check/update POOL state counter.
    //
    ZZPCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx);

    //
    // Check update flag, saved ID, and saved name against the input.
    //
    if ((!UPDATE && (*SAVCDE != 0)) && fstr::eq(SAVNAM, FRNAME)) {
        //
        // No change in the POOL state, the saved name was successfully
        // resolved earlier, and input and saved names are the same.
        // Return saved ID.
        //
        *FRCODE = *SAVCDE;
    } else {
        //
        // Check in because NAMFRM may fail.
        //
        CHKIN(b"ZZNAMFRM", ctx)?;

        //
        // POOL state changed, or the saved name was never successfully
        // resolved earlier, or input and saved names are different. Call
        // NAMFRM to look up ID and reset saved values.
        //
        NAMFRM(FRNAME, FRCODE, ctx)?;

        fstr::assign(SAVNAM, FRNAME);
        *SAVCDE = *FRCODE;

        //
        // Check out.
        //
        CHKOUT(b"ZZNAMFRM", ctx)?;
    }

    Ok(())
}
