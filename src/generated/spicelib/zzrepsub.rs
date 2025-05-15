//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZREPSUB ( Replace one substring with another )
pub fn ZZREPSUB(IN: &[u8], LEFT: i32, RIGHT: i32, STRING: &[u8], OUT: &mut [u8]) {
    let mut INLEN: i32 = 0;
    let mut STRLEN: i32 = 0;
    let mut OUTLEN: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut USE = StackArray::<i32, 3>::new(1..=3);
    let mut END: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut MYLEFT: i32 = 0;
    let mut MYRGHT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Get the lengths of all the strings involved in this transaction.
    //
    INLEN = intrinsics::LEN(IN);
    STRLEN = intrinsics::LEN(STRING);
    OUTLEN = intrinsics::LEN(OUT);
    MYLEFT = intrinsics::MIN0(&[(INLEN + 1), intrinsics::MAX0(&[1, LEFT])]);
    MYRGHT = intrinsics::MIN0(&[INLEN, intrinsics::MAX0(&[0, RIGHT])]);
    //
    // Reject bad inputs.
    //
    if (MYLEFT < 1) {
        MYLEFT = 1;
    } else if (MYRGHT > INLEN) {
        MYRGHT = INLEN;
    } else if (RIGHT < (LEFT - 1)) {
        return;
    }

    //
    // Consider three separate sections:
    //
    //    1) The front of the original string.
    //
    //    2) The replacement string.
    //
    //    3) The end of the original string.
    //
    // Determine how much of each section to use in the output string.
    // REMAIN is the number of characters that will fit in the output
    // string.
    //
    REMAIN = OUTLEN;
    USE[1] = intrinsics::MIN0(&[REMAIN, (MYLEFT - 1)]);

    REMAIN = (REMAIN - USE[1]);
    USE[2] = intrinsics::MIN0(&[REMAIN, STRLEN]);

    REMAIN = (REMAIN - USE[2]);
    USE[3] = intrinsics::MIN0(&[REMAIN, (INLEN - RIGHT)]);

    //
    // Move the third section first. It gets moved back to front
    // or front to back, depending on whether the replacement string
    // is longer than the original substring. The main thing is to
    // avoid overwriting characters that have yet to be moved.
    //
    END = SUMAI(USE.as_slice(), 3);

    if ((MYLEFT + STRLEN) > RIGHT) {
        NEXT = END;

        for I in intrinsics::range(USE[3], 1, -1) {
            fstr::assign(
                fstr::substr_mut(OUT, NEXT..=NEXT),
                fstr::substr(IN, (RIGHT + I)..=(RIGHT + I)),
            );
            NEXT = (NEXT - 1);
        }
    } else {
        NEXT = (MYLEFT + STRLEN);

        for I in 1..=USE[3] {
            fstr::assign(
                fstr::substr_mut(OUT, NEXT..=NEXT),
                fstr::substr(IN, (RIGHT + I)..=(RIGHT + I)),
            );
            NEXT = (NEXT + 1);
        }
    }

    //
    // The first two sections can be moved directly to the front of
    // the output string.
    //
    NEXT = 1;

    for I in 1..=USE[1] {
        fstr::assign(fstr::substr_mut(OUT, NEXT..=NEXT), fstr::substr(IN, I..=I));
        NEXT = (NEXT + 1);
    }

    for I in 1..=USE[2] {
        fstr::assign(
            fstr::substr_mut(OUT, NEXT..=NEXT),
            fstr::substr(STRING, I..=I),
        );
        NEXT = (NEXT + 1);
    }

    //
    // Pad with blanks, if the output string was not filled.
    //
    if (END < OUTLEN) {
        fstr::assign(fstr::substr_mut(OUT, (END + 1)..), b" ");
    }
}
