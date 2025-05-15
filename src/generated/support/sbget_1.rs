//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const LBCBUF: i32 = 0;

//$Procedure SBGET ( String buffer, get )
pub fn SBGET_1(
    NAME: &[u8],
    NAMES: CharArray,
    PTRS: &[i32],
    BUFFER: CharArray,
    STR: &mut [u8],
    POS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let NAMES = DummyCharArray::new(NAMES, None, LBCELL..);
    let PTRS = DummyArray::new(PTRS, LBCELL..);
    let BUFFER = DummyCharArray::new(BUFFER, None, LBCBUF..);
    let mut FOUND: bool = false;

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
        spicelib::CHKIN(b"SBGET_1", ctx)?;
    }

    //
    // Is this string even in the list?
    //
    *POS = spicelib::BSRCHC(
        NAME,
        spicelib::CARDC(NAMES.as_arg(), ctx)?,
        NAMES.subarray(1),
    );

    //
    // If so, get it.
    //
    if (*POS > 0) {
        LBGET_1(*POS, PTRS.as_slice(), BUFFER.as_arg(), STR, &mut FOUND, ctx)?;
    }

    spicelib::CHKOUT(b"SBGET_1", ctx)?;
    Ok(())
}
