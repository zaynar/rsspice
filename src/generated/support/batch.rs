//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    INBTCH: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut INBTCH: bool = false;

        INBTCH = false;

        Self { INBTCH }
    }
}

//$Procedure      BATCH (Tell whether or not a program is in batch mode)
pub fn BATCH(ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut BATCH: bool = false;

    BATCH = save.INBTCH;
    BATCH
}

pub fn SETMOD(ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SETMOD: bool = false;

    save.INBTCH = false;
    SETMOD = true;
    SETMOD
}

pub fn SETBAT(ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut SETBAT: bool = false;

    save.INBTCH = true;
    SETBAT = true;
    SETBAT
}
