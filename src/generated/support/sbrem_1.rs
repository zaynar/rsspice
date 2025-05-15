//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBCBUF: i32 = 0;

//$Procedure SBREM ( String buffer, remove )
pub fn SBREM_1(
    NAME: &[u8],
    NAMES: CharArrayMut,
    PTRS: &mut [i32],
    BUFFER: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NAMES = DummyCharArrayMut::new(NAMES, None, LBCELL..);
    let mut PTRS = DummyArrayMut::new(PTRS, LBCELL..);
    let mut BUFFER = DummyCharArrayMut::new(BUFFER, None, LBCBUF..);
    let mut NSTR: i32 = 0;
    let mut POS: i32 = 0;

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
        spicelib::CHKIN(b"SBREM_1", ctx)?;
    }

    //
    // Recover the essential control information.
    //
    NSTR = spicelib::CARDC(NAMES.as_arg(), ctx)?;

    //
    // Which string is to be removed?
    //
    POS = spicelib::BSRCHC(NAME, NSTR, NAMES.subarray(1));

    //
    // If the string is not in the buffer, do nothing.
    //
    if (POS > 0) {
        //
        // Remove the name from the name list, and the string from the
        // line buffer.
        //
        spicelib::REMLAC(1, POS, NAMES.subarray_mut(1), &mut NSTR, ctx)?;
        spicelib::SCARDC(NSTR, NAMES.as_arg_mut(), ctx)?;

        LBREM_1(POS, PTRS.as_slice_mut(), BUFFER.as_arg_mut(), ctx)?;
    }

    spicelib::CHKOUT(b"SBREM_1", ctx)?;
    Ok(())
}
