//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCBUF: i32 = 0;

//$Procedure DIMCB ( Dimension of character buffer )
pub fn DIMCB_1(BUFFER: CharArray, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let BUFFER = DummyCharArray::new(BUFFER, None, LBCBUF..);
    let mut DIMCB_1: i32 = 0;
    let mut DIM: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //
    //
    // Give the function an initial value even if it is bogus in the
    // event that we are in RETURN mode.
    //
    DIMCB_1 = 0;

    //
    // Standard error handling.
    //

    if spicelib::RETURN(ctx) {
        return Ok(DIMCB_1);
    } else {
        spicelib::CHKIN(b"DIMCB_1", ctx)?;
    }

    //
    // Only the first eight bytes are used.
    //
    spicelib::DECHAR(fstr::substr(&BUFFER[0], 1..=8), &mut DIM, ctx)?;
    DIMCB_1 = DIM;

    spicelib::CHKOUT(b"DIMCB_1", ctx)?;
    Ok(DIMCB_1)
}
