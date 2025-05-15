//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WRDLEN: i32 = 32;

struct SaveVars {
    M2NAME: bool,
    I: i32,
    LENGTH: i32,
    START: i32,
    END: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2NAME: bool = false;
        let mut I: i32 = 0;
        let mut LENGTH: i32 = 0;
        let mut START: i32 = 0;
        let mut END: i32 = 0;

        Self {
            M2NAME,
            I,
            LENGTH,
            START,
            END,
        }
    }
}

//$Procedure      M2NAME ( Determine whether or not a word is a name )
pub fn M2NAME(WORD: &[u8], ctx: &mut Context) -> bool {
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

    save.M2NAME = ((save.LENGTH <= WRDLEN) && (save.LENGTH >= 1));

    if save.M2NAME {
        save.I = save.START;
        save.M2NAME = (((intrinsics::ICHAR(b"A")
            <= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))
            && (intrinsics::ICHAR(b"Z")
                >= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I))))
            || ((intrinsics::ICHAR(b"a")
                <= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))
                && (intrinsics::ICHAR(b"z")
                    >= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))));

        save.I = (save.I + 1);
    }

    while (save.M2NAME && (save.I <= save.END)) {
        save.M2NAME = ((((((intrinsics::ICHAR(b"A")
            <= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))
            && (intrinsics::ICHAR(b"Z")
                >= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I))))
            || ((intrinsics::ICHAR(b"a")
                <= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))
                && (intrinsics::ICHAR(b"z")
                    >= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))))
            || ((intrinsics::ICHAR(b"0")
                <= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))
                && (intrinsics::ICHAR(b"9")
                    >= intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)))))
            || (intrinsics::ICHAR(b"_")
                == intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I))))
            || (intrinsics::ICHAR(b"-") == intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I))));

        save.I = (save.I + 1);
    }

    save.M2NAME
}
