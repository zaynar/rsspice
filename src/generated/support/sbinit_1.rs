//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBCBUF: i32 = 0;

//$Procedure SBINIT ( String buffer, initialize )
pub fn SBINIT_1(
    NSIZE: i32,
    PSIZE: i32,
    VDIM: i32,
    NAMES: CharArrayMut,
    PTRS: &mut [i32],
    BUFFER: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NAMES = DummyCharArrayMut::new(NAMES, None, LBCELL..=NSIZE);
    let mut PTRS = DummyArrayMut::new(PTRS, LBCELL..=PSIZE);
    let mut BUFFER = DummyCharArrayMut::new(BUFFER, None, LBCBUF..=VDIM);
    let mut MAXPTR: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"SBINIT_1", ctx)?;
    }

    //
    // Make sure that the line buffer is large enough (but ONLY large
    // enough) to hold the maximum number of strings. The name list
    // should be empty. The LB should be initialized as a unit.
    //
    MAXPTR = (4 * (NSIZE + 1));

    if (PSIZE < MAXPTR) {
        spicelib::SIGERR(b"SPICE(SBINSUFPTRSIZE)", ctx)?;
    } else {
        spicelib::SSIZEC(NSIZE, NAMES.as_arg_mut(), ctx)?;
        LBINIT_1(MAXPTR, VDIM, PTRS.as_slice_mut(), BUFFER.as_arg_mut(), ctx)?;
    }

    spicelib::CHKOUT(b"SBINIT_1", ctx)?;
    Ok(())
}
