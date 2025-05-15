//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const ENVLEN: i32 = 32;
pub const VALLEN: i32 = 255;

//$Procedure      ZZGETENV ( Get environment variable value. )
pub fn ZZGETENV(ENVVAR: &[u8], VALUE: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut MYVALU = [b' '; VALLEN as usize];

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //
    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"ZZGETENV", ctx)?;
    }
    //
    // We do three things:
    //
    //    1) Check to see if the input is blank.
    //    2) Attempt to get the value.
    //    3) If we got a nonblank value, see if it will fit in the
    //       space provided.
    //
    if fstr::eq(ENVVAR, b" ") {
        fstr::assign(&mut MYVALU, b" ");
    } else {
        ctx.getenv(ENVVAR, &mut MYVALU);

        if fstr::ne(&MYVALU, b" ") {
            if (spicelib::RTRIM(&MYVALU) > intrinsics::LEN(VALUE)) {
                fstr::assign(&mut MYVALU, b" ");
            }
        }
    }

    fstr::assign(VALUE, &MYVALU);

    spicelib::CHKOUT(b"ZZGETENV", ctx)?;
    Ok(())
}
