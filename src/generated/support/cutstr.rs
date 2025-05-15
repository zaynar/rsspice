//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SPACE: i32 = 1;
const PREF: i32 = (SPACE + 1);
const PUNC: i32 = (PREF + 1);
const QUOT: i32 = (PUNC + 1);
const OTHR: i32 = (QUOT + 1);
const FAIL: i32 = (OTHR + 1);
const FINSH: i32 = (FAIL + 1);
const BSLASH: i32 = 92;

//$Procedure      CUTSTR ( Cut a long string into substrings )
pub fn CUTSTR(
    STRING: &[u8],
    START: i32,
    WIDTH: i32,
    BREAKS: &[u8],
    BEG: &mut i32,
    END: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut A: i32 = 0;
    let mut B: i32 = 0;
    let mut BLANK: i32 = 0;
    let mut P: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut LONG: i32 = 0;
    let mut PUNCT = [b' '; 15];
    let mut QUOTE = [b' '; 15];
    let mut OTHER = [b' '; 15];
    let mut DEF = [b' '; 15];
    let mut QTYPE = [b' '; 15];
    let mut PTYPE = [b' '; 15];
    let mut OTYPE = [b' '; 15];
    let mut DTYPE = [b' '; 15];
    let mut PASS: i32 = 0;
    let mut THIS = [b' '; 1];
    let mut NEXT = [b' '; 1];
    let mut HERE: i32 = 0;
    let mut THERE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Parameters used to simulate an enumerated type for
    // the various passes required to break the string at
    // good places.  Note that the order is important.
    // This forces the routine to try spaces first, user
    // supplied preferences next, etc.    It is also
    // critical that these be defined to be a sequence
    // of consecutive integers.
    //

    //
    // The ASCII character value for the backslash is needed for
    // uniformity of porting this routine (Some platforms treat the
    // backslah as a special character and so we can't just use
    // the character in strings.)
    //

    //
    // Standard error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    //
    // Exceptions first. Is START outside the bounds of the string?
    //
    LENGTH = intrinsics::LEN(STRING);

    if (START > LENGTH) {
        *BEG = 0;
        *END = 0;
        return Ok(());
    } else if (START < 1) {
        spicelib::CHKIN(b"CUTSTR", ctx)?;
        spicelib::SIGERR(b"SPICE(BEFOREBEGSTR)", ctx)?;
        spicelib::CHKOUT(b"CUTSTR", ctx)?;
        return Ok(());
    }

    //
    // Is the width reasonable?
    //
    if (WIDTH < 1) {
        spicelib::CHKIN(b"CUTSTR", ctx)?;
        spicelib::SIGERR(b"SPICE(WIDTHTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"CUTSTR", ctx)?;
        return Ok(());
    }

    //
    // Does the remainder of the string contain anything besides blanks?
    //
    if fstr::eq(fstr::substr(STRING, START..), b" ") {
        *BEG = 0;
        *END = 0;
        return Ok(());
    }

    //
    // Obviously, we should try to get the longest possible substring.
    //
    //
    *BEG = START;
    BLANK = intrinsics::ICHAR(b" ");

    while (intrinsics::ICHAR(fstr::substr(STRING, *BEG..=*BEG)) == BLANK) {
        *BEG = (*BEG + 1);
    }

    LONG = ((*BEG + WIDTH) - 1);

    //
    // The remainder of the substring may fit without a trim.
    // But drop trailing blanks anyway.
    //
    if (LENGTH <= LONG) {
        *END = LENGTH;

        while (intrinsics::ICHAR(fstr::substr(STRING, *END..=*END)) == BLANK) {
            *END = (*END - 1);
        }

        return Ok(());
    }

    //
    // Assign the default break characters. Each character in PUNCT,
    // QUOTE, or OTHER indicates a good place to break. The associated
    // type indicates whether the break should occur at or before the
    // the character:
    //
    //    Type         Break occurs
    //    ----         ------------------------------------------------
    //     A           At the character.
    //     B           Before the character.
    //     P           At an EVEN occurrence, or
    //                 Before an ODD occurrence.
    //
    //
    fstr::assign(&mut PUNCT, b",.;:-)]}>([{<");
    fstr::assign(&mut PTYPE, b"AAAAAAAAABBBB");

    fstr::assign(&mut QUOTE, b"\"\'");
    fstr::assign(&mut QTYPE, b"PP");

    fstr::assign(
        &mut OTHER,
        &fstr::concat(b"?!~$@^=_%*/|&+", &intrinsics::CHAR(BSLASH)),
    );
    fstr::assign(&mut OTYPE, b"AABBBBAAABBBBBB");

    //
    // We will do this in five passes. During the first pass, we will
    // try to break before a space. During the second pass, we will try
    // to break at one of the preferred characters. During the third,
    // fourth, and fifth passes, we will try to break at or before one
    // of the quotation, punctuation, or other default characters.
    //
    PASS = SPACE;
    A = intrinsics::ICHAR(b"A");
    P = intrinsics::ICHAR(b"P");
    B = intrinsics::ICHAR(b"B");

    while (PASS != FAIL) {
        *END = LONG;

        while (*END >= *BEG) {
            fstr::assign(&mut THIS, fstr::substr(STRING, *END..=*END));
            fstr::assign(&mut NEXT, fstr::substr(STRING, (*END + 1)..=(*END + 1)));

            //
            // Always break BEFORE a space.
            //
            if (PASS == SPACE) {
                if (intrinsics::ICHAR(&NEXT) == BLANK) {
                    PASS = FINSH;
                }

            //
            // Always break AT a preferred character.
            //
            } else if (PASS == PREF) {
                if (intrinsics::INDEX(BREAKS, &THIS) > 0) {
                    PASS = FINSH;
                }

            //
            // But with default characters, some break at, some
            // before, and some depend on the parity of strangers.
            //
            } else {
                HERE = intrinsics::INDEX(&DEF, &THIS);
                THERE = intrinsics::INDEX(&DEF, &NEXT);

                if (HERE > 0) {
                    if (intrinsics::ICHAR(fstr::substr(&DTYPE, HERE..=HERE)) == A) {
                        PASS = FINSH;
                    } else if (intrinsics::ICHAR(fstr::substr(&DTYPE, HERE..=HERE)) == P) {
                        if spicelib::EVEN(OCCURS(fstr::substr(STRING, 1..=*END), &THIS)) {
                            PASS = FINSH;
                        }
                    }
                }

                if ((THERE > 0) && (PASS != FINSH)) {
                    if (intrinsics::ICHAR(fstr::substr(&DTYPE, THERE..=THERE)) == B) {
                        PASS = FINSH;
                    } else if (intrinsics::ICHAR(fstr::substr(&DTYPE, THERE..=THERE)) == P) {
                        if spicelib::EVEN(OCCURS(fstr::substr(STRING, 1..=*END), &NEXT)) {
                            PASS = FINSH;
                        }
                    }
                }
            }

            //
            // If we've found a break point, remove any trailing blanks
            // before returning.
            //
            if (PASS == FINSH) {
                while (intrinsics::ICHAR(fstr::substr(STRING, *END..=*END)) == BLANK) {
                    *END = (*END - 1);
                }

                return Ok(());
            } else {
                *END = (*END - 1);
            }
        }

        //
        // We may have to try another pass.
        //
        PASS = (PASS + 1);

        //
        // In the final passes, only the character set changes.
        //
        if (PASS == PUNC) {
            fstr::assign(&mut DEF, &PUNCT);
            fstr::assign(&mut DTYPE, &PTYPE);
        } else if (PASS == QUOT) {
            fstr::assign(&mut DEF, &QUOTE);
            fstr::assign(&mut DTYPE, &QTYPE);
        } else if (PASS == OTHR) {
            fstr::assign(&mut DEF, &OTHER);
            fstr::assign(&mut DTYPE, &OTYPE);
        }
    }

    //
    // Looks like we'll have to do this the hard way.
    //
    *END = LONG;

    Ok(())
}
