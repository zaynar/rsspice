//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    BEGIN: i32,
    LAST: i32,
    J: i32,
    K: i32,
    ERROR: Vec<u8>,
    MINVAL: f64,
    MAXVAL: f64,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BEGIN: i32 = 0;
        let mut LAST: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut ERROR = vec![b' '; 80];
        let mut MINVAL: f64 = 0.0;
        let mut MAXVAL: f64 = 0.0;

        Self {
            BEGIN,
            LAST,
            J,
            K,
            ERROR,
            MINVAL,
            MAXVAL,
        }
    }
}

//$Procedure      M2NTEM ( Parse the numeric template of a @number )
pub fn M2NTEM(
    STRING: &[u8],
    BASE: &[u8],
    BEG: &mut i32,
    END: i32,
    A: &mut f64,
    B: &mut f64,
    ctx: &mut Context,
) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    save.LAST = (END - 1);
    save.BEGIN = (*BEG + 1);

    //
    // First get the min and max's for this kind of word.
    //
    if fstr::eq(BASE, b"@int") {
        save.MINVAL = (spicelib::INTMIN() as f64);
        save.MAXVAL = (spicelib::INTMAX() as f64);
    } else {
        save.MINVAL = spicelib::DPMIN();
        save.MAXVAL = spicelib::DPMAX();
    }

    //
    // parse the restriction template
    //
    if fstr::eq(fstr::substr(STRING, save.BEGIN..=save.BEGIN), b":") {
        *A = save.MINVAL;
        spicelib::NPARSD(
            fstr::substr(STRING, (save.BEGIN + 1)..=save.LAST),
            B,
            &mut save.ERROR,
            &mut save.J,
            ctx,
        );

        if fstr::ne(&save.ERROR, b" ") {
            *B = save.MAXVAL;
        }
    } else if fstr::eq(fstr::substr(STRING, save.LAST..=save.LAST), b":") {
        spicelib::NPARSD(
            fstr::substr(STRING, save.BEGIN..=(save.LAST - 1)),
            A,
            &mut save.ERROR,
            &mut save.J,
            ctx,
        );

        if fstr::ne(&save.ERROR, b" ") {
            *A = save.MINVAL;
        }

        *B = save.MAXVAL;
    } else {
        save.J = (intrinsics::INDEX(fstr::substr(STRING, save.BEGIN..=save.LAST), b":") + *BEG);

        spicelib::NPARSD(
            fstr::substr(STRING, save.BEGIN..=(save.J - 1)),
            A,
            &mut save.ERROR,
            &mut save.K,
            ctx,
        );

        if fstr::ne(&save.ERROR, b" ") {
            *A = save.MINVAL;
        }

        spicelib::NPARSD(
            fstr::substr(STRING, (save.J + 1)..=save.LAST),
            B,
            &mut save.ERROR,
            &mut save.K,
            ctx,
        );

        if fstr::ne(&save.ERROR, b" ") {
            *B = save.MAXVAL;
        }
    }

    *BEG = (END + 1);
}
