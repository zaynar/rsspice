//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure CPUTIM ( CPU Time )
pub fn CPUTIM(TVEC: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut TVEC = DummyArrayMut::new(TVEC, 1..=6);

    //
    // SPICELIB functions
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"CPUTIM", ctx)?;
    }

    //
    // Get the date and time.
    //
    spicelib::ZZCPUTIM(TVEC.as_slice_mut(), ctx)?;

    //
    // That's it.
    //
    spicelib::CHKOUT(b"CPUTIM", ctx)?;
    Ok(())
}
