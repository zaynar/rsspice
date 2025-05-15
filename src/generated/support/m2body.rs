//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;

//$Procedure      M2BODY ( Determine whether or not a word is a body )
pub fn M2BODY(WORD: &[u8], ctx: &mut Context) -> f2rust_std::Result<bool> {
    let mut M2BODY: bool = false;
    let mut COPY = [b' '; WDSIZE as usize];
    let mut IDCODE: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Library functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    if M2INT(WORD, ctx) {
        M2BODY = true;
        return Ok(M2BODY);
    }

    spicelib::UCASE(WORD, &mut COPY, ctx);

    M2BODN2C(&COPY, &mut IDCODE, &mut FOUND, ctx)?;

    M2BODY = FOUND;

    Ok(M2BODY)
}
