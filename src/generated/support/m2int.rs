//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    M2INT: bool,
    END: i32,
    FACTOR: i32,
    I: i32,
    LENGTH: i32,
    MINUS: i32,
    PLUS: i32,
    START: i32,
    SUBSEQ: i32,
    VALUE: i32,
    ZERO: i32,
    BAD: StackArray<bool, 256>,
    FIRST: bool,
    USEMIN: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut M2INT: bool = false;
        let mut END: i32 = 0;
        let mut FACTOR: i32 = 0;
        let mut I: i32 = 0;
        let mut LENGTH: i32 = 0;
        let mut MINUS: i32 = 0;
        let mut PLUS: i32 = 0;
        let mut START: i32 = 0;
        let mut SUBSEQ: i32 = 0;
        let mut VALUE: i32 = 0;
        let mut ZERO: i32 = 0;
        let mut BAD = StackArray::<bool, 256>::new(0..=255);
        let mut FIRST: bool = false;
        let mut USEMIN: bool = false;

        FIRST = true;

        Self {
            M2INT,
            END,
            FACTOR,
            I,
            LENGTH,
            MINUS,
            PLUS,
            START,
            SUBSEQ,
            VALUE,
            ZERO,
            BAD,
            FIRST,
            USEMIN,
        }
    }
}

//$Procedure      M2INT ( Determine whether or not a word is an integer )
pub fn M2INT(WORD: &[u8], ctx: &mut Context) -> bool {
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

        {
            let m1__: i32 = 0;
            let m2__: i32 = 255;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.BAD[save.I] = true;
                save.I += m3__;
            }
        }

        save.MINUS = intrinsics::ICHAR(b"-");
        save.PLUS = intrinsics::ICHAR(b"+");
        save.ZERO = intrinsics::ICHAR(b"0");

        save.BAD[intrinsics::ICHAR(b"0")] = false;
        save.BAD[intrinsics::ICHAR(b"1")] = false;
        save.BAD[intrinsics::ICHAR(b"2")] = false;
        save.BAD[intrinsics::ICHAR(b"3")] = false;
        save.BAD[intrinsics::ICHAR(b"4")] = false;
        save.BAD[intrinsics::ICHAR(b"5")] = false;
        save.BAD[intrinsics::ICHAR(b"6")] = false;
        save.BAD[intrinsics::ICHAR(b"7")] = false;
        save.BAD[intrinsics::ICHAR(b"8")] = false;
        save.BAD[intrinsics::ICHAR(b"9")] = false;
    }

    save.START = spicelib::LTRIM(WORD);
    save.END = QRTRIM(WORD);
    save.LENGTH = ((save.END - save.START) + 1);
    save.SUBSEQ = (save.START + 1);

    if (save.LENGTH == 1) {
        save.BAD[save.MINUS] = true;
        save.BAD[save.PLUS] = true;

        save.M2INT = !save.BAD[intrinsics::ICHAR(fstr::substr(WORD, save.START..=save.START))];
        return save.M2INT;
    } else if (save.LENGTH > 10) {
        save.M2INT = false;
    } else {
        save.BAD[save.MINUS] = false;
        save.BAD[save.PLUS] = false;
    }

    if save.BAD[intrinsics::ICHAR(fstr::substr(WORD, save.START..=save.START))] {
        save.M2INT = false;
        return save.M2INT;
    }

    save.BAD[save.MINUS] = true;
    save.BAD[save.PLUS] = true;

    {
        let m1__: i32 = save.SUBSEQ;
        let m2__: i32 = save.END;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if save.BAD[intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I))] {
                save.M2INT = false;
                return save.M2INT;
            }
            save.I += m3__;
        }
    }
    //
    // We allow 10 digit numbers only if the first character
    // is a '+' or '-'  So if we have 10 digits the first must
    // now be a "bad" character.
    //
    save.USEMIN = (intrinsics::ICHAR(fstr::substr(WORD, save.START..=save.START)) == save.MINUS);

    if save.BAD[intrinsics::ICHAR(fstr::substr(WORD, save.START..=save.START))] {
        if (save.LENGTH < 11) {
            save.M2INT = true;
            return save.M2INT;
        }

        save.START = save.SUBSEQ;
    } else if (save.LENGTH == 11) {
        save.M2INT = false;
        return save.M2INT;
    } else if (save.LENGTH < 10) {
        save.M2INT = true;
        return save.M2INT;
    }

    if save.USEMIN {
        save.VALUE = spicelib::INTMIN();
        save.FACTOR = 1;

        {
            let m1__: i32 = save.END;
            let m2__: i32 = (save.START + 1);
            let m3__: i32 = -1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.VALUE = (save.VALUE
                    + ((intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)) - save.ZERO)
                        * save.FACTOR));
                save.FACTOR = (save.FACTOR * 10);
                save.I += m3__;
            }
        }

        if (intrinsics::ICHAR(fstr::substr(WORD, save.START..=save.START))
            > intrinsics::ICHAR(b"2"))
        {
            save.M2INT = false;
        } else {
            save.I = save.START;
            save.VALUE = (save.VALUE
                + ((intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)) - save.ZERO)
                    * save.FACTOR));
            save.M2INT = (save.VALUE <= 0);
        }
    } else {
        save.VALUE = spicelib::INTMAX();
        save.FACTOR = 1;
        {
            let m1__: i32 = save.END;
            let m2__: i32 = (save.START + 1);
            let m3__: i32 = -1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.VALUE = (save.VALUE
                    - ((intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)) - save.ZERO)
                        * save.FACTOR));
                save.FACTOR = (save.FACTOR * 10);
                save.I += m3__;
            }
        }

        if (intrinsics::ICHAR(fstr::substr(WORD, save.START..=save.START))
            > intrinsics::ICHAR(b"2"))
        {
            save.M2INT = false;
        } else {
            save.I = save.START;
            save.VALUE = (save.VALUE
                - ((intrinsics::ICHAR(fstr::substr(WORD, save.I..=save.I)) - save.ZERO)
                    * save.FACTOR));
            save.M2INT = (save.VALUE >= 0);
        }
    }

    save.M2INT
}
