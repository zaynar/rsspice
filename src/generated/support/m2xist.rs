//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    SIZE: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SIZE: i32 = 0;

        Self { SIZE }
    }
}

//$Procedure      M2XIST ( META/2 --- does a named template word exist )
pub fn M2XIST(NAME: &[u8], ctx: &mut Context) -> f2rust_std::Result<bool> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut M2XIST: bool = false;

    //

    //
    // Local variables
    //

    //
    // Find out how many endpoints were matched.  The NAME is there
    // if SIZE is greater than 0.
    //
    M2VSIZ(NAME, &mut save.SIZE, ctx)?;

    M2XIST = (save.SIZE > 0);

    Ok(M2XIST)
}
