//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WSTR: &[u8; 1] = &fstr::extend_const::<1>(b"*");
const WCHR: &[u8; 1] = &fstr::extend_const::<1>(b"%");
const NOTCHR: &[u8; 1] = &fstr::extend_const::<1>(b"~");
const ORCHR: &[u8; 1] = &fstr::extend_const::<1>(b"|");

//$Procedure MATCH ( Match string against multiple wildcard templates )
pub fn MATCH(STRING: &[u8], TEMPL: &[u8], ctx: &mut Context) -> f2rust_std::Result<bool> {
    let mut MATCH: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Give the function an intial value of FALSE
    //
    MATCH = false;

    //
    // Standard SPICE error handling
    //
    if spicelib::RETURN(ctx) {
        return Ok(MATCH);
    } else {
        spicelib::CHKIN(b"MATCH", ctx)?;
    }

    MATCH = MATCHM(STRING, TEMPL, WSTR, WCHR, NOTCHR, ORCHR, ctx)?;

    spicelib::CHKOUT(b"MATCH", ctx)?;
    Ok(MATCH)
}
