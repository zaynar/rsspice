//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCBUF: i32 = 0;

//$Procedure SIZECB ( Size of character buffer )
pub fn SIZECB_1(BUFFER: CharArray, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let BUFFER = DummyCharArray::new(BUFFER, None, LBCBUF..);
    let mut SIZECB_1: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Give the function some initial value.  Zero seems as good as
    // anything.
    //
    SIZECB_1 = 0;

    //
    // Standard error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(SIZECB_1);
    } else {
        spicelib::CHKIN(b"SIZECB_1", ctx)?;
    }

    //
    // Size = dimension * length.
    //
    SIZECB_1 = (DIMCB_1(BUFFER.as_arg(), ctx)? * intrinsics::LEN(&BUFFER[1]));

    spicelib::CHKOUT(b"SIZECB_1", ctx)?;
    Ok(SIZECB_1)
}
