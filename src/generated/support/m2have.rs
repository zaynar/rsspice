//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    M2HAVE: i32,
    SIZE: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2HAVE: i32 = 0;
        let mut SIZE: i32 = 0;

        Self { M2HAVE, SIZE }
    }
}

//$Procedure      M2HAVE ( META/2 --- How many matches do we have )
pub fn M2HAVE(NAME: &[u8], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // Local variables
    //

    //
    // Find out how many endpoints were matched, and put the answer into
    // M2HAVE.
    //
    M2VSIZ(NAME, &mut save.SIZE, ctx)?;

    save.M2HAVE = save.SIZE;

    Ok(save.M2HAVE)
}
