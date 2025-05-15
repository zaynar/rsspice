//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZEKTRDP ( EK tree, return data pointer )
pub fn ZZEKTRDP(
    HANDLE: i32,
    TREE: i32,
    KEY: i32,
    PTR: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut IDX: i32 = 0;
    let mut NODE: i32 = 0;
    let mut NOFFST: i32 = 0;
    let mut LEVEL: i32 = 0;

    //
    // Local variables
    //

    //
    // Use discovery check-in in this puppy.
    //
    ZZEKTRLK(
        HANDLE,
        TREE,
        KEY,
        &mut IDX,
        &mut NODE,
        &mut NOFFST,
        &mut LEVEL,
        PTR,
        ctx,
    )?;

    Ok(())
}
