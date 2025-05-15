//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    M2NUMB: bool,
    LENGTH: i32,
    START: i32,
    END: i32,
    X: f64,
    ERROR: Vec<u8>,
    POINTR: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2NUMB: bool = false;
        let mut LENGTH: i32 = 0;
        let mut START: i32 = 0;
        let mut END: i32 = 0;
        let mut X: f64 = 0.0;
        let mut ERROR = vec![b' '; 80];
        let mut POINTR: i32 = 0;

        Self {
            M2NUMB,
            LENGTH,
            START,
            END,
            X,
            ERROR,
            POINTR,
        }
    }
}

//$Procedure      M2NUMB ( Determine whether or not a word is a number )
pub fn M2NUMB(WORD: &[u8], ctx: &mut Context) -> bool {
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
    save.START = spicelib::LTRIM(WORD);
    save.END = QRTRIM(WORD);
    save.LENGTH = ((save.END - save.START) + 1);

    //
    // Rule out the goofy cases that NPARSD will allow.
    //
    if (save.LENGTH == 1) {
        save.M2NUMB = (fstr::le(b"0", WORD) && fstr::ge(b"9", WORD));
        return save.M2NUMB;
    }

    if (save.LENGTH >= 2) {
        if (((fstr::eq(fstr::substr(WORD, save.START..=save.START), b"E")
            || fstr::eq(fstr::substr(WORD, save.START..=save.START), b"e"))
            || fstr::eq(fstr::substr(WORD, save.START..=save.START), b"D"))
            || fstr::eq(fstr::substr(WORD, save.START..=save.START), b"d"))
        {
            save.M2NUMB = false;
            return save.M2NUMB;
        }

        if (((((((((((fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b"+E")
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b"-E"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b"+D"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b"-D"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b"-e"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b"+e"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b"-d"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b"+d"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b".E"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b".D"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b".e"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 1)), b".d"))
        {
            save.M2NUMB = false;
            return save.M2NUMB;
        }
    }

    if (save.LENGTH >= 3) {
        if (((fstr::eq(fstr::substr(WORD, save.START..=(save.START + 2)), b"+.E")
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 2)), b"-.E"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 2)), b"+.D"))
            || fstr::eq(fstr::substr(WORD, save.START..=(save.START + 2)), b"-.D"))
        {
            save.M2NUMB = false;
            return save.M2NUMB;
        }
    }

    //
    // Ok.  Now just hit the word with NPARSD.
    //
    fstr::assign(&mut save.ERROR, b" ");

    spicelib::NPARSD(WORD, &mut save.X, &mut save.ERROR, &mut save.POINTR, ctx);

    //
    // Any errors indicate we don't have a number.
    //
    if fstr::ne(&save.ERROR, b" ") {
        save.M2NUMB = false;
    } else {
        save.M2NUMB = true;
    }

    save.M2NUMB
}
