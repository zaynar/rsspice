//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure GFDECR ( Derivative numeric )
pub fn GFDECR(
    UDFUNC: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
    ET: &mut f64,
    BOOL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DT: f64 = 0.0;

    DT = 1.0;

    //
    // Determine if GFQ is increasing or decreasing at ET.
    //
    // UDDC - the SPICE function to determine if
    //        the derivative of the user defined
    //        function is negative at ET.
    //
    // UDFUNC - the user defined scalar quantity function.
    //
    spicelib::UDDC(UDFUNC, *ET, DT, BOOL, ctx)?;

    Ok(())
}
