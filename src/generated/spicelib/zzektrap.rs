//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZEKTRAP ( EK tree, append item )
pub fn ZZEKTRAP(
    HANDLE: i32,
    TREE: i32,
    VALUE: i32,
    KEY: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // Non-SPICELIB functions
    //

    *KEY = (ZZEKTRSZ(HANDLE, TREE, ctx)? + 1);

    ZZEKTRIN(HANDLE, TREE, *KEY, VALUE, ctx)?;

    Ok(())
}
