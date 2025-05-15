//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    START: i32,
    I: i32,
    LPAREN: i32,
    RPAREN: i32,
    COLON: i32,
    DIGIT: StackArray<bool, 256>,
    ERROR: Vec<u8>,
    POINTR: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut START: i32 = 0;
        let mut I: i32 = 0;
        let mut LPAREN: i32 = 0;
        let mut RPAREN: i32 = 0;
        let mut COLON: i32 = 0;
        let mut DIGIT = StackArray::<bool, 256>::new(0..=255);
        let mut ERROR = vec![b' '; 80];
        let mut POINTR: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            START,
            I,
            LPAREN,
            RPAREN,
            COLON,
            DIGIT,
            ERROR,
            POINTR,
            FIRST,
        }
    }
}

//$Procedure      M2BEGR ( See if a word begins with a range template )
pub fn M2BEGR(STRING: &[u8], BEG: &mut i32, END: i32, A: &mut i32, B: &mut i32, ctx: &mut Context) {
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
                save.DIGIT[save.I] = false;
                save.I += m3__;
            }
        }

        save.DIGIT[intrinsics::ICHAR(b"0")] = true;
        save.DIGIT[intrinsics::ICHAR(b"1")] = true;
        save.DIGIT[intrinsics::ICHAR(b"2")] = true;
        save.DIGIT[intrinsics::ICHAR(b"3")] = true;
        save.DIGIT[intrinsics::ICHAR(b"4")] = true;
        save.DIGIT[intrinsics::ICHAR(b"5")] = true;
        save.DIGIT[intrinsics::ICHAR(b"6")] = true;
        save.DIGIT[intrinsics::ICHAR(b"7")] = true;
        save.DIGIT[intrinsics::ICHAR(b"8")] = true;
        save.DIGIT[intrinsics::ICHAR(b"9")] = true;

        save.LPAREN = intrinsics::ICHAR(b"(");
        save.RPAREN = intrinsics::ICHAR(b")");
        save.COLON = intrinsics::ICHAR(b":");
    }

    //
    // We need at least (x:) in order to have a range template,  that
    // means at least 4 characters.
    //
    if ((END - *BEG) < 3) {
        return;
    }

    save.I = *BEG;
    //
    // Range templates must begin with '('
    //
    if (intrinsics::ICHAR(fstr::substr(STRING, save.I..=save.I)) != save.LPAREN) {
        return;
    }

    save.I = (save.I + 1);

    //
    // We must have at least 1 digit
    //
    if !save.DIGIT[intrinsics::ICHAR(fstr::substr(STRING, save.I..=save.I))] {
        return;
    } else {
        save.I = (save.I + 1);
    }

    //
    // Now examin characters until we reach a non-digit
    // or run out of characters in the string.
    //
    while ((save.I <= END) && save.DIGIT[intrinsics::ICHAR(fstr::substr(STRING, save.I..=save.I))])
    {
        save.I = (save.I + 1);
    }

    //
    // If the last character encountered was a number or if it was
    // not a colon, we don't have a range template.
    //
    if save.DIGIT[intrinsics::ICHAR(fstr::substr(STRING, save.I..=save.I))] {
        return;
    } else if (intrinsics::ICHAR(fstr::substr(STRING, save.I..=save.I)) != save.COLON) {
        return;
    }

    //
    // Ok. we've got an integer. Parse it and put the result
    // into A.
    //
    spicelib::NPARSI(
        fstr::substr(STRING, (*BEG + 1)..=(save.I - 1)),
        A,
        &mut save.ERROR,
        &mut save.POINTR,
        ctx,
    );

    //
    // Just in case, make sure the number didn't cause an NPARSI error
    // (the only thing can go wrong is the number is too big)
    //
    if (save.POINTR != 0) {
        return;
    }

    //
    // Look at the next letter ( if there is one ) and see if it
    // is a digit.
    //
    save.I = (save.I + 1);
    save.START = save.I;

    if (save.I > END) {
        return;
    }

    //
    // Examine letters until we reach a non-digit or run out of
    // characters to examine.
    //
    while ((save.I < END) && save.DIGIT[intrinsics::ICHAR(fstr::substr(STRING, save.I..=save.I))]) {
        save.I = (save.I + 1);
    }

    //
    // If the last character is a digit (we ran out of letters)
    // or was not
    //
    if save.DIGIT[intrinsics::ICHAR(fstr::substr(STRING, save.I..=save.I))] {
        return;
    } else if (intrinsics::ICHAR(fstr::substr(STRING, save.I..=save.I)) != save.RPAREN) {
        return;
    }

    //
    // If the last character read is beyond the first character
    // after the ':', then we've got an integer.
    //
    if (save.I > save.START) {
        spicelib::NPARSI(
            fstr::substr(STRING, save.START..=(save.I - 1)),
            B,
            &mut save.ERROR,
            &mut save.POINTR,
            ctx,
        );

        //
        // Make sure everythin parsed ok.
        //
        if (save.POINTR != 0) {
            return;
        } else if (*B < *A) {
            return;
        } else {
            *BEG = (save.I + 1);
            return;
        }

    //
    // If the first character after the colon was the right parenthesis
    // put INTMAX into B
    //
    } else {
        *B = spicelib::INTMAX();
        *BEG = (save.I + 1);
        return;
    }
}
