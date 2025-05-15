//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    M2UNIT: bool,
    START: i32,
    END: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2UNIT: bool = false;
        let mut START: i32 = 0;
        let mut END: i32 = 0;

        Self { M2UNIT, START, END }
    }
}

//$Procedure      M2UNIT ( Determine whether a word is a unit spec )
pub fn M2UNIT(WORD: &[u8], ctx: &mut Context) -> f2rust_std::Result<bool> {
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
    save.END = spicelib::RTRIM(WORD);

    save.M2UNIT = UNITP(fstr::substr(WORD, save.START..=save.END), ctx)?;

    Ok(save.M2UNIT)
}
