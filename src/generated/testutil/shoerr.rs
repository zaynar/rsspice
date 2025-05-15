//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      SHOERR ( Show Errors )
pub fn SHOERR(ctx: &mut Context) -> f2rust_std::Result<()> {
    spicelib::ERRDEV(b"SET", &mut b"SCREEN".clone(), ctx)?;
    Ok(())
}
