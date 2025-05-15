//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WRDLEN: i32 = 32;

struct SaveVars {
    M2ENGL: bool,
    I: i32,
    LENGTH: i32,
    START: i32,
    END: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2ENGL: bool = false;
        let mut I: i32 = 0;
        let mut LENGTH: i32 = 0;
        let mut START: i32 = 0;
        let mut END: i32 = 0;

        Self {
            M2ENGL,
            I,
            LENGTH,
            START,
            END,
        }
    }
}

//$Procedure      M2ENGL ( Determine if a word contains all letters)
pub fn M2ENGL(WORD: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // WRDLEN is the parameter that gives the maximum allowed length
    // of a name.
    //

    //
    // Make sure the string has the right length.
    //
    save.START = spicelib::LTRIM(WORD);
    save.END = QRTRIM(WORD);
    save.LENGTH = ((save.END - save.START) + 1);

    save.M2ENGL = ((save.LENGTH <= WRDLEN) && (save.LENGTH >= 1));

    save.I = save.START;

    while (save.M2ENGL && (save.I <= save.END)) {
        save.M2ENGL = (((intrinsics::ICHAR(b"A")
            <= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))
            && (intrinsics::ICHAR(b"Z")
                >= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I))))
            || ((intrinsics::ICHAR(b"a")
                <= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))
                && (intrinsics::ICHAR(b"z")
                    >= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))));

        save.I = (save.I + 1);
    }

    save.M2ENGL
}
