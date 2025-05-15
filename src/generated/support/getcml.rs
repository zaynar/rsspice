//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ARGLEN: i32 = 512;

struct SaveVars {
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FIRST: bool = false;

        FIRST = true;

        Self { FIRST }
    }
}

//$Procedure      GETCML ( Get the command line )
pub fn GETCML(LINE: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ARGMNT = [b' '; ARGLEN as usize];
    let mut HOWMNY: i32 = 0;
    let mut I: i32 = 0;

    //
    // Other functions
    //

    //
    // Local Variables
    //

    //
    // Call the FORTRAN library function iargc to determine how many
    // words are on the command line. Then, get the arguments one at a
    // time and construct the output string.
    //
    HOWMNY = ctx.iargc();
    I = 1;
    fstr::assign(LINE, b" ");

    if (HOWMNY == 0) {
        return;
    }

    while (I <= HOWMNY) {
        ctx.getarg(I, &mut ARGMNT);

        if save.FIRST {
            spicelib::SUFFIX(&ARGMNT, 0, LINE);
            save.FIRST = false;
        } else {
            spicelib::SUFFIX(&ARGMNT, 1, LINE);
        }

        I = (I + 1);
    }
}
