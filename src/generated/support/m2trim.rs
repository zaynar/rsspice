//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    E: i32,
    B: i32,
    LBRACE: i32,
    RBRACE: i32,
    BLANK: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut E: i32 = 0;
        let mut B: i32 = 0;
        let mut LBRACE: i32 = 0;
        let mut RBRACE: i32 = 0;
        let mut BLANK: i32 = 0;

        Self {
            E,
            B,
            LBRACE,
            RBRACE,
            BLANK,
        }
    }
}

//$Procedure      M2TRIM ( META/2 trim the name portion from a word )
pub fn M2TRIM(WORD: &[u8], ROOT: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    fstr::assign(ROOT, WORD);
    save.LBRACE = intrinsics::ICHAR(b"[");
    save.RBRACE = intrinsics::ICHAR(b"]");
    save.BLANK = intrinsics::ICHAR(b" ");
    save.E = intrinsics::LEN(WORD);
    //
    // This loop is the same as RTRIM only faster.
    //
    save.E = QRTRIM(WORD);

    //
    // If the length is not at least 4 or the last character is not
    // a right brace, there is no name associated with this word.
    //
    if ((intrinsics::ICHAR(fstr::substr(WORD, save.E..=save.E)) == save.RBRACE) && (save.E >= 4)) {
        //
        // Ok. We have a chance at getting a name.  Look for
        // a left brace and if found blank out the end portion of
        // ROOT.
        //
        save.B = 2;

        while (save.B < (save.E - 1)) {
            if (intrinsics::ICHAR(fstr::substr(WORD, save.B..=save.B)) == save.LBRACE) {
                //
                // We've found the beginning of the name portion
                // of the word.  Record the end of the meta-2
                // word and then reset L so that we exit this loop.
                //
                fstr::assign(fstr::substr_mut(ROOT, save.B..), b" ");
                save.B = save.E;
            }

            save.B = (save.B + 1);
        }
    }
}
