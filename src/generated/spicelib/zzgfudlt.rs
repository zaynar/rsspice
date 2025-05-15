//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ZZGET: i32 = -1;
const ZZPUT: i32 = -2;
const ZZRESET: i32 = -3;
const ZZNOP: i32 = 3;
const GEN: i32 = 1;
const GF_REF: i32 = 2;
const GF_TOL: i32 = 3;
const GF_DT: i32 = 4;
const NID: i32 = 4;

//$Procedure ZZGFUDLT ( Private --- GF, scalar function < ref value )
pub fn ZZGFUDLT(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    ISLESS: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut UDVAL: f64 = 0.0;
    let mut REFVAL: f64 = 0.0;
    let mut OK: bool = false;

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

    CHKIN(b"ZZGFUDLT", ctx)?;

    //
    // Default value of false for ISLESS.
    //
    *ISLESS = false;

    //
    // Call the routine, return the scalar value corresponding to ET.
    //
    UDFUNC(ET, &mut UDVAL, ctx)?;

    //
    // Check for an error, return if found.
    //
    if FAILED(ctx) {
        CHKOUT(b"ZZGFUDLT", ctx)?;
        return Ok(());
    }

    //
    // Retrieve the stored reference value. Signal an error if a
    // get call fails.
    //
    ZZHOLDD(ZZGET, GF_REF, &mut OK, &mut REFVAL, ctx)?;

    if !OK {
        SETMSG(b"ZZHOLDD GET failed. This indicates a logic error in the GF code due either to a failure to store the GF reference value or a post store reset of ZZHOLDD.", ctx);
        SIGERR(b"SPICE(ZZHOLDDGETFAILED)", ctx)?;
        CHKOUT(b"ZZGFUDLT", ctx)?;
        return Ok(());
    }

    *ISLESS = (UDVAL < REFVAL);

    CHKOUT(b"ZZGFUDLT", ctx)?;
    Ok(())
}
