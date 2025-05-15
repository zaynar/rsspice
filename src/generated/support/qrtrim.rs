//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TUNE: i32 = 16;

//$Procedure      QRTRIM (Quick right trim )
pub fn QRTRIM(STRING: &[u8]) -> i32 {
    let mut QRTRIM: i32 = 0;
    let mut B: i32 = 0;
    let mut L: i32 = 0;
    let mut M: i32 = 0;
    let mut NL: i32 = 0;
    let mut BLANK: i32 = 0;

    BLANK = intrinsics::ICHAR(b" ");
    L = intrinsics::LEN(STRING);
    //
    // If this is a short string there is no particular advantage
    // to be gained by making use of the binary search idea.
    // The speed up just doesn't buy much when compared with
    // the loop overhead.
    //
    if (L <= 32) {
        for I in intrinsics::range(L, 1, -1) {
            if (intrinsics::ICHAR(fstr::substr(STRING, I..=I)) != BLANK) {
                QRTRIM = I;
                return QRTRIM;
            }
        }

        QRTRIM = 1;
        return QRTRIM;
    }

    B = 1;
    NL = (L - 1);

    //
    // We want M to be ( B + NL ) / 2   but right now that's L/2
    //
    M = (L / 2);

    while ((L - B) > TUNE) {
        //
        // What is true right now?  The string from L+1 on out
        // is blank.  L > B; L-1 = NL >= B;  M = (B + NL) / 2;
        // and M >= B,  B is at least one and if greater than 1
        // there must be a non-blank character between B and the
        // end of the string.
        //
        if (intrinsics::ICHAR(fstr::substr(STRING, L..=L)) != BLANK) {
            QRTRIM = L;
            return QRTRIM;
        } else if (intrinsics::ICHAR(fstr::substr(STRING, M..=M)) != BLANK) {
            L = NL;
            B = M;
        } else if fstr::eq(fstr::substr(STRING, (M + 1)..=NL), b" ") {
            //
            // If you got here, the STRING(L:L) is a blank.
            // The string from L+1 on out is blank.
            // The string from M to NL (=L-1) is blank.  Thus the
            // string from M out is blank.
            //
            // M is greater than or equal to B
            // If M  is less than B + 2, then L will become
            // B or less and there will not be a
            // next pass through the loop.  That means that
            // we will never get to this point again and don't
            // have to worry about the reference STRING(M:NL)
            // giving us an access violation.
            //
            L = (M - 1);
        //
        // With the new value of L, we now know that STRING(L+1:)
        // is blank.
        //
        } else {
            //
            // If you get to this point all of the string from
            // L out is blank and L is greater than M.
            // There is a non-blank character between M+1 and NL.
            // If L should become equal to B below, then the loop
            // will not be executed again.  That means again that
            // we don't have to worry about STRING(M:NL) being
            // an ill formed string.
            //
            L = NL;
            B = (M + 1);
            //
            // With the new value of L, we now know that STRING(L+1:)
            // is blank.
            //
        }

        NL = (L - 1);
        M = ((B + NL) / 2);

        //
        // What's true now?  The string from L+1 on out is blank.
        // Somewhere between B and L is a non-blank character.
        //
    }
    //
    // Either B never changed from 1 or B was set to a value such that
    // there was a non-blank character between B and the end of
    // the string,  And the string from L+1 out to the end is
    // blank.  Since we want this to mimick RTRIM, we are done.
    //
    for I in intrinsics::range(L, 1, -1) {
        if (intrinsics::ICHAR(fstr::substr(STRING, I..=I)) != BLANK) {
            QRTRIM = I;
            return QRTRIM;
        }
    }

    QRTRIM = 1;
    QRTRIM
}
