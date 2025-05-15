//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const CTRSIZ: i32 = 2;

//$Procedure ZZBODS2C ( Body name to ID translation, with bypass )
pub fn ZZBODS2C(
    USRCTR: &mut [i32],
    SAVNAM: &mut [u8],
    SAVCDE: &mut i32,
    SAVFND: &mut bool,
    NAME: &[u8],
    CODE: &mut i32,
    FOUND: &mut bool,
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
    // Check/update ZZBODTRN state counter.
    //
    ZZBCTRCK(USRCTR.as_slice_mut(), &mut UPDATE, ctx)?;

    //
    // Check update flag, saved found flag, and saved name against the
    // input.
    //
    if ((!UPDATE && *SAVFND) && fstr::eq(SAVNAM, NAME)) {
        //
        // No change in body-name mapping state, the saved name was
        // successfully resolved earlier, and input and saved names are
        // the same. Return saved ID and FOUND.
        //
        *CODE = *SAVCDE;
        *FOUND = *SAVFND;
    } else {
        //
        // Check in because BODS2C may fail.
        //
        CHKIN(b"ZZBODS2C", ctx)?;

        //
        // Body-name mapping state changed, or the saved name was never
        // successfully resolved earlier, or input and saved names are
        // different. Call BODS2C to look up ID and FOUND and reset saved
        // values.
        //
        BODS2C(NAME, CODE, FOUND, ctx)?;

        fstr::assign(SAVNAM, NAME);
        *SAVCDE = *CODE;
        *SAVFND = *FOUND;

        //
        // Check out.
        //
        CHKOUT(b"ZZBODS2C", ctx)?;
    }

    Ok(())
}
