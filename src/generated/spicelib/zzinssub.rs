//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZINSSUB ( Insert a substring )
pub fn ZZINSSUB(IN: &[u8], SUB: &[u8], LOC: i32, OUT: &mut [u8]) {
    let mut CHR = [b' '; 1 as usize];
    let mut FROM: i32 = 0;
    let mut INLEN: i32 = 0;
    let mut MYLOC: i32 = 0;
    let mut NMOVE: i32 = 0;
    let mut OUTLEN: i32 = 0;
    let mut SUBEND: i32 = 0;
    let mut SUBLEN: i32 = 0;
    let mut TO: i32 = 0;
    let mut SAME: bool = false;

    //
    // Local Variables
    //

    //
    // Note to the careful reader:  in order to scrupulously avoid
    // non-standard assignments of characters from a substring of IN to
    // an overlapping substring of OUT, in the case where IN and OUT
    // refer to the same memory, we'll test whether the output and
    // input strings are the same.  If they're the same, we can avoid
    // various assignments that could cause trouble if IN and OUT
    // actually refer to the same memory.  This test has little effect on
    // performance, and allows the author to sleep more soundly at night.
    //
    // Capture the lengths of the input, output, and substitution
    // strings.
    //
    INLEN = intrinsics::LEN(IN);
    OUTLEN = intrinsics::LEN(OUT);
    SUBLEN = intrinsics::LEN(SUB);
    MYLOC = intrinsics::MIN0(&[(INLEN + 1), intrinsics::MAX0(&[1, LOC])]);

    //
    // If the insertion occurs after the end of the output string,
    // just return the original string.  Don't do the assignment if
    // the output and input strings have equal values; the assignment
    // is not needed in this case and could cause a run-time error if
    // OUT and IN refer to the same memory.
    //
    SAME = fstr::eq(OUT, IN);

    if (MYLOC > OUTLEN) {
        if !SAME {
            fstr::assign(OUT, IN);
        }

        return;
    }

    //
    // At this point, we're guaranteed that
    //
    //    MYLOC  <  OUTLEN
    //         -
    //
    //    MYLOC  <  INLEN + 1
    //         -
    //
    //    MYLOC  >  0
    //
    //
    // The first part of the input string is copied without change
    // to the output string, if this first part is non-empty.
    //
    if (MYLOC > 1) {
        //
        // Again, do the assignment only if it's required.
        //
        if !SAME {
            fstr::assign(fstr::substr_mut(OUT, 1..=(MYLOC - 1)), IN);
        }
    }

    //
    // The part following the new substring is shifted into place, if
    // there's both something to move and a place to put it.  Move the
    // rightmost characters first.
    //
    SUBEND = ((MYLOC - 1) + SUBLEN);

    if ((MYLOC <= INLEN) && (SUBEND < OUTLEN)) {
        NMOVE = intrinsics::MIN0(&[(OUTLEN - SUBEND), ((INLEN - MYLOC) + 1)]);

        for I in intrinsics::range(NMOVE, 1, -1) {
            FROM = ((MYLOC + I) - 1);
            TO = (SUBEND + I);
            fstr::assign(&mut CHR, fstr::substr(IN, FROM..=FROM));
            fstr::assign(fstr::substr_mut(OUT, TO..=TO), &CHR);
        }
    }

    //
    // And the new word is dropped into the middle.
    //
    fstr::assign(
        fstr::substr_mut(OUT, MYLOC..=intrinsics::MIN0(&[SUBEND, OUTLEN])),
        SUB,
    );

    //
    // Blank-pad the output string if necessary.
    //
    if (OUTLEN > (INLEN + SUBLEN)) {
        fstr::assign(fstr::substr_mut(OUT, ((INLEN + SUBLEN) + 1)..), b" ");
    }
}
