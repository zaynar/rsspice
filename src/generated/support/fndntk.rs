//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      FNDNTK ( Find the next token in a string )
pub fn FNDNTK(STRING: &[u8], DELIMS: &[u8], START: i32, BEG: &mut i32, END: &mut i32) {
    let mut SPACE: bool = false;
    let mut LAST: i32 = 0;
    let mut EOL: i32 = 0;
    let mut B: i32 = 0;
    let mut BLANK: bool = false;
    let mut NBR: i32 = 0;
    let mut NBL: i32 = 0;
    let mut DELIMR: bool = false;
    let mut DELIML: bool = false;
    let mut NODELM: bool = false;

    //
    // Local variables
    //

    //%&END_DECLARATIONS

    //
    // First we gather some data regarding the input string and
    // delimiters
    //
    SPACE = (intrinsics::INDEX(DELIMS, b" ") != 0);
    LAST = intrinsics::LEN(STRING);
    EOL = (LAST + 1);
    B = intrinsics::MAX0(&[1, START]);

    //
    // We don't have to do anything if we are starting past the end of
    // the string.
    //
    if (B > EOL) {
        *BEG = 0;
        *END = 0;
        return;
    }

    //
    // Find the first non-blank character at or to the right of where
    // we are starting.
    //
    BLANK = true;
    NBR = B;

    while BLANK {
        if (NBR >= EOL) {
            BLANK = false;
        } else if fstr::ne(fstr::substr(STRING, NBR..=NBR), b" ") {
            BLANK = false;
        } else {
            NBR = (NBR + 1);
        }
    }

    //
    // Find the first non-blank character and first non-blank delimiter
    // to the left of the starting point.
    //
    BLANK = true;
    NBL = (B - 1);

    while BLANK {
        if (NBL <= 0) {
            BLANK = false;
        } else if fstr::ne(fstr::substr(STRING, NBL..=NBL), b" ") {
            BLANK = false;
        } else {
            NBL = (NBL - 1);
        }
    }

    //
    // If both the preceeding non-blank character and the following
    // non-blank character are delimiters, we have a null item.
    //
    if (NBR >= EOL) {
        DELIMR = true;
    } else {
        DELIMR = (intrinsics::INDEX(DELIMS, fstr::substr(STRING, NBR..=NBR)) != 0);
    }

    if (NBL <= 0) {
        DELIML = true;
    } else {
        DELIML = (intrinsics::INDEX(DELIMS, fstr::substr(STRING, NBL..=NBL)) != 0);
    }

    if (DELIMR && DELIML) {
        *BEG = NBR;
        *END = (*BEG - 1);

        return;
    }

    //
    // Still here? See if we were past the last delimiter.
    //
    if ((NBR >= EOL) && !DELIML) {
        *BEG = 0;
        *END = 0;
        return;
    }

    //
    // If the left most non-blank is a delimiter OR a blank is a
    // delimiter and the non-blank character to the left is at least
    // two characters away from the right non-blank character, then
    // we have a token beginning at the right non-blank. We just need
    // to find the right boundary.
    //
    if (DELIML || ((((NBR - NBL) >= 2) && SPACE) && !DELIMR)) {
        *BEG = NBR;
        *END = *BEG;

        //
        // Note: DELIMR is already .FALSE. or else we couldn't get to
        // this point.
        //
        while !DELIMR {
            if ((*END + 1) >= EOL) {
                DELIMR = true;
            } else if (intrinsics::INDEX(DELIMS, fstr::substr(STRING, (*END + 1)..=(*END + 1)))
                != 0)
            {
                DELIMR = true;
            } else {
                *END = (*END + 1);
            }
        }

        //
        // Back up END to the first non-blank that precedes it.
        //
        while fstr::eq(fstr::substr(STRING, *END..=*END), b" ") {
            *END = (*END - 1);
        }

        return;
    }

    //
    // Still here? In that case we were in the middle of something
    // to start with.  Move the pointer forward until we reach a
    // delimiter.
    //
    // Keep in mind that DELIMR still has the information as to whether
    // or not NBR points to a non-blank delimiter. We are going to use
    // this information to determine whether to look for a delimiter
    // first or not.
    //
    if !DELIMR {
        NODELM = true;
        B = NBR;

        while NODELM {
            NBR = (NBR + 1);

            if (NBR >= EOL) {
                NODELM = false;
            } else {
                NODELM = (intrinsics::INDEX(DELIMS, fstr::substr(STRING, NBR..=NBR)) == 0);
            }
        }

        //
        // If a space is a delimiter and we happen to have landed on one,
        // we want to continue until we hit a non-blank delimiter or just
        // before a non-blank character.
        //
        if (SPACE && (NBR < EOL)) {
            NODELM = fstr::eq(fstr::substr(STRING, NBR..=NBR), b" ");

            while NODELM {
                NBR = (NBR + 1);

                if (NBR == EOL) {
                    NODELM = false;
                } else if (intrinsics::INDEX(DELIMS, fstr::substr(STRING, NBR..=NBR)) != 0) {
                    NODELM = fstr::eq(fstr::substr(STRING, NBR..=NBR), b" ");
                } else if fstr::ne(fstr::substr(STRING, NBR..=NBR), b" ") {
                    NODELM = false;
                    //
                    // Back up one, to just before the non-blank character
                    //
                    NBR = (NBR - 1);
                }
            }
        }
        //
        // Since we did not start on a delimiter if we reached the end of
        // the string before hitting one, then there is no token to find
        // here.
        //
        if (NBR >= EOL) {
            *BEG = 0;
            *END = 0;

            return;
        }
    }

    //
    // Still here?  Then starting at the first character to the right of
    // the delimiter, find the next non-blank character, and the next
    // right delimiter after that.
    //
    NBL = NBR;
    BLANK = true;

    while BLANK {
        NBL = (NBL + 1);

        if (NBL >= EOL) {
            BLANK = false;
        } else {
            BLANK = fstr::eq(fstr::substr(STRING, NBL..=NBL), b" ");
        }
    }

    //
    // Now locate the next delimiter.
    //
    NBR = (NBL - 1);
    DELIMR = false;

    while !DELIMR {
        NBR = (NBR + 1);

        if (NBR >= EOL) {
            DELIMR = true;
        } else {
            DELIMR = (intrinsics::INDEX(DELIMS, fstr::substr(STRING, NBR..=NBR)) != 0);
        }
    }

    *BEG = NBL;
    *END = (NBR - 1);

    if (*END > *BEG) {
        //
        // Backup until we are at a non-space.
        //
        while (fstr::eq(fstr::substr(STRING, *END..=*END), b" ") && (*END > *BEG)) {
            *END = (*END - 1);
        }
    }
}
