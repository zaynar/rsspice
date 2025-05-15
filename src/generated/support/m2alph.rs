//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    M2ALPH: bool,
    I: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2ALPH: bool = false;
        let mut I: i32 = 0;

        Self { M2ALPH, I }
    }
}

//$Procedure      M2ALPH ( Determine if a word starts with a letter)
pub fn M2ALPH(WORD: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Make sure the string has the right length.
    //
    save.I = spicelib::LTRIM(WORD);
    save.M2ALPH = ((fstr::le(b"A", fstr::substr(WORD, save.I..=save.I))
        && fstr::ge(b"Z", fstr::substr(WORD, save.I..=save.I)))
        || (fstr::le(b"a", fstr::substr(WORD, save.I..=save.I))
            && fstr::ge(b"z", fstr::substr(WORD, save.I..=save.I))));

    save.M2ALPH
}
