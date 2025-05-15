//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    M2YEAR: bool,
    LENGTH: i32,
    I1: i32,
    I2: i32,
    I3: i32,
    I4: i32,
    VALUE: i32,
    VALUES: StackArray<i32, 256>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2YEAR: bool = false;
        let mut LENGTH: i32 = 0;
        let mut I1: i32 = 0;
        let mut I2: i32 = 0;
        let mut I3: i32 = 0;
        let mut I4: i32 = 0;
        let mut VALUE: i32 = 0;
        let mut VALUES = StackArray::<i32, 256>::new(0..=255);
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            M2YEAR,
            LENGTH,
            I1,
            I2,
            I3,
            I4,
            VALUE,
            VALUES,
            FIRST,
        }
    }
}

//$Procedure      M2YEAR ( Determine whether or not a word is a year )
pub fn M2YEAR(WORD: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions
    //
    //
    // Local variables
    //

    if save.FIRST {
        save.FIRST = false;
        //
        // We will construct a value for the string by taking
        // the non-blank portion and computing the value assuming
        // that the first non-blank is a digit with the appropriate
        // power of 10 attached.  Since all non-digit characters
        // will have values of 1000, we will get a value greater
        // than 1000 if any non-digit characters are present.
        //
        for I in 0..=255 {
            save.VALUES[I] = 10000;
        }

        save.VALUES[intrinsics::ICHAR(b"0")] = 0;
        save.VALUES[intrinsics::ICHAR(b"1")] = 1;
        save.VALUES[intrinsics::ICHAR(b"2")] = 2;
        save.VALUES[intrinsics::ICHAR(b"3")] = 3;
        save.VALUES[intrinsics::ICHAR(b"4")] = 4;
        save.VALUES[intrinsics::ICHAR(b"5")] = 5;
        save.VALUES[intrinsics::ICHAR(b"6")] = 6;
        save.VALUES[intrinsics::ICHAR(b"7")] = 7;
        save.VALUES[intrinsics::ICHAR(b"8")] = 8;
        save.VALUES[intrinsics::ICHAR(b"9")] = 9;
    }

    //
    // Make sure the string has the right length.
    //
    save.I1 = spicelib::LTRIM(WORD);
    save.I4 = QRTRIM(WORD);
    save.LENGTH = ((save.I4 - save.I1) + 1);

    //
    // Rule out the goofy cases that NPARSD will allow.
    //
    if (save.LENGTH != 4) {
        save.VALUE = 10000;
    } else {
        save.I2 = (save.I1 + 1);
        save.I3 = (save.I2 + 1);
        save.VALUE = ((((1000
            * save.VALUES[intrinsics::ICHAR(fstr::substr(WORD, save.I1..=save.I1))])
            + (100 * save.VALUES[intrinsics::ICHAR(fstr::substr(WORD, save.I2..=save.I2))]))
            + (10 * save.VALUES[intrinsics::ICHAR(fstr::substr(WORD, save.I3..=save.I3))]))
            + save.VALUES[intrinsics::ICHAR(fstr::substr(WORD, save.I4..=save.I4))]);
    }
    //
    // That's all just make sure that the value is within the
    // bound required of a year.
    //
    save.M2YEAR = ((save.VALUE >= 1000) && (save.VALUE <= 3000));

    save.M2YEAR
}
