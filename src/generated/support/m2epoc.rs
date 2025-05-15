//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    M2EPOC: bool,
    TCODE: i32,
    ERROR: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2EPOC: bool = false;
        let mut TCODE: i32 = 0;
        let mut ERROR = vec![b' '; 80];

        Self {
            M2EPOC,
            TCODE,
            ERROR,
        }
    }
}

//$Procedure      M2EPOC ( Determine whether or not a word is an epoch )
pub fn M2EPOC(WORD: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICE functions
    //

    //
    // Local variables
    //

    if M2TIME(WORD, ctx) {
        save.M2EPOC = false;
    } else if M2YEAR(WORD, ctx) {
        save.M2EPOC = true;
    } else if M2MON(WORD, ctx) {
        save.M2EPOC = false;
    } else {
        M2CAL(WORD, &mut save.ERROR, &mut save.TCODE, ctx);
        save.M2EPOC = fstr::eq(&save.ERROR, b" ");
    }

    save.M2EPOC
}
