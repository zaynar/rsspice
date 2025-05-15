//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

fn ISQ(L: i32, SQUOTE: i32, DQUOTE: i32, SPCIAL: i32) -> bool {
    (((L == SQUOTE) || (L == DQUOTE)) || (L == SPCIAL))
}

fn BLANK(
    L: i32,
    ODDDQ: bool,
    ODDSQ: bool,
    ODDSP: bool,
    SQUOTE: i32,
    DQUOTE: i32,
    SPCIAL: i32,
) -> bool {
    (((((L == intrinsics::ICHAR(b" ")) || ODDDQ) || ODDSQ) || ODDSP)
        || ISQ(L, SQUOTE, DQUOTE, SPCIAL))
}

//$Procedure      NTHUQW ( N'th unquoted word)
pub fn NTHUQW(STRING: &[u8], N: i32, EQUOTE: &[u8], WORD: &mut [u8], LOC: &mut i32) {
    let EQUOTE = &EQUOTE[..1 as usize];
    let mut B: i32 = 0;
    let mut DQUOTE: i32 = 0;
    let mut E: i32 = 0;
    let mut L: i32 = 0;
    let mut LAST: i32 = 0;
    let mut SQUOTE: i32 = 0;
    let mut WCOUNT: i32 = 0;
    let mut SPCIAL: i32 = 0;
    let mut ODDDQ: bool = false;
    let mut ODDSQ: bool = false;
    let mut ODDSP: bool = false;
    let mut INWORD: bool = false;

    //
    // Spice Functions
    //
    //
    // Local Variables
    //

    //
    // An integer
    //

    //
    // Take care of the dumb cases first.
    //
    if (N <= 0) {
        fstr::assign(WORD, b" ");
        *LOC = 0;
        return;
    }

    SQUOTE = intrinsics::ICHAR(b"\'");
    DQUOTE = intrinsics::ICHAR(b"\"");

    SPCIAL = intrinsics::ICHAR(fstr::substr(EQUOTE, 1..=1));

    if (SPCIAL == intrinsics::ICHAR(b" ")) {
        SPCIAL = SQUOTE;
    }

    LAST = spicelib::RTRIM(STRING);
    WCOUNT = 0;
    ODDDQ = false;
    ODDSQ = false;
    ODDSP = false;
    INWORD = false;

    for I in 1..=LAST {
        //
        // Get the integer value of the I'th character of string.
        //
        L = intrinsics::ICHAR(fstr::substr(STRING, I..=I));

        //
        // If this is a quote character, then flip the ODDQ logical
        //
        if (L == SPCIAL) {
            ODDSP = !ODDSP;
        }

        if (L == SQUOTE) {
            ODDSQ = !ODDSQ;
        }

        if (L == DQUOTE) {
            ODDDQ = !ODDDQ;
        }
        //
        // If this is a blank ...
        //
        if BLANK(L, ODDDQ, ODDSQ, ODDSP, SQUOTE, DQUOTE, SPCIAL) {
            //
            // if we are in the middle of a word, we are about to
            // end it.  If the word counter WCOUNT has the same
            // value of N then we've found the N'th unquoted word.
            // Set the various outputs and return.
            //
            if (INWORD && (WCOUNT == N)) {
                fstr::assign(WORD, fstr::substr(STRING, B..=E));
                *LOC = B;
                return;
            }
            //
            // If we get to here, we just point out that we are
            // not in a word.
            //
            INWORD = false;
        } else {
            //
            // If this is not a "blank"  then ODDDQ, ODDSQ and ODDSP are
            // false so we are not inside a quoted string.  We are either
            // already in a word, or we are just starting one.
            //
            if INWORD {
                //
                // We are in a word, just bump the end of this one.
                //
                E = I;
            } else {
                //
                // We are beginning a word. Up the word counter,
                // set the end and beginning of the word.
                //
                INWORD = true;
                WCOUNT = (WCOUNT + 1);
                B = I;
                E = I;
            }
        }
        //
        // Examine the next character.
        //
    }

    if (INWORD && (WCOUNT == N)) {
        *LOC = B;
        fstr::assign(WORD, fstr::substr(STRING, B..));
    } else {
        *LOC = 0;
        fstr::assign(WORD, b" ");
    }
}
