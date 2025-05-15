//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    VERBOS: bool,
    VERBON: bool,
    VERBOFF: bool,
    DISPLY: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut VERBOS: bool = false;
        let mut VERBON: bool = false;
        let mut VERBOFF: bool = false;
        let mut DISPLY: bool = false;

        DISPLY = false;

        Self {
            VERBOS,
            VERBON,
            VERBOFF,
            DISPLY,
        }
    }
}

//$Procedure      VERBOS ( Display detailed information to the screen)
//
pub fn VERBOS(ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // Entry points
    //
    //
    // Local Variables.
    //

    save.VERBOS = save.DISPLY;
    save.VERBOS
}

pub fn VERBON(ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.DISPLY = true;
    save.VERBON = true;
    save.VERBON
}

pub fn VERBOFF(ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.DISPLY = false;
    save.VERBOFF = false;
    save.VERBOFF
}
