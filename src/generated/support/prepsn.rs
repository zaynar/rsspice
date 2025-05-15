//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MAXLEN: i32 = 2000;
const WDSIZE: i32 = 63;

//$Procedure      PREPSN (Pretty print syntax definition)
pub fn PREPSN(STRING: &mut [u8], ctx: &mut Context) {
    let mut LONG = [b' '; MAXLEN as usize];
    let mut START: i32 = 0;
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut END: i32 = 0;
    let mut INDNBY: i32 = 0;
    let mut R: i32 = 0;
    let mut BEGIN: bool = false;
    let mut INDENT: bool = false;
    let mut CRLAST: bool = false;
    let mut WORD = [b' '; WDSIZE as usize];
    let mut OUTDNT = [b' '; WDSIZE as usize];

    //
    // Set the initial states.
    //
    //    START  we start looking at the string at the first character
    //    E      end of the first word (we have to start somewhere)
    //    END    is the end of the local buffer LONG.
    //    INDBY  is the amount we've indented things.
    //    LONG   is our local string for creating the pretty print string
    //    OUTDNT is the string for controlling out-denting
    //    BEGIN  we have not begun processing a switch
    //    INDENT we have not indented
    //    CRLAST we did not put a '/cr' in the last word we processed.
    //

    START = 1;
    E = 1;
    END = 1;
    INDNBY = 0;
    fstr::assign(&mut LONG, b" ");
    fstr::assign(&mut OUTDNT, b" ");
    BEGIN = false;
    INDENT = false;
    CRLAST = false;
    //
    // Process the string a word at a time  until we've seen it all.
    //
    while (E != 0) {
        spicelib::FNDNWD(STRING, START, &mut B, &mut E);

        if (E > 0) {
            if fstr::eq(fstr::substr(STRING, E..=E), b"{") {
                //
                // There was a word left in the string.  The beginning
                // of a switch ends with '{'
                //
                BEGIN = true;
                INDENT = false;

                if CRLAST {
                    CRLAST = false;
                    fstr::assign(&mut WORD, &fstr::concat(b" ", fstr::substr(STRING, B..=E)));
                } else {
                    fstr::assign(fstr::substr_mut(&mut WORD, 1..=8), b"/cr(:1) ");
                    fstr::assign(
                        fstr::substr_mut(&mut WORD, 9..),
                        fstr::substr(STRING, B..=E),
                    );
                }
                //
                // We shall indent (if we do at all) by the number
                // of characters that precede the left bracket '{'
                //
                INDNBY = (E - B);
            } else if fstr::eq(fstr::substr(STRING, B..=E), b"|") {
                //
                // Switch separators appear all by themselves a words.
                //
                if BEGIN {
                    //
                    // This is the first separator of this switch, we
                    // are probably going to indent.  And we are no
                    // longer in the beginning simple template of the
                    // switch.
                    //
                    BEGIN = false;
                    INDENT = true;

                    if (INDNBY > 0) {
                        //
                        // Create the indent and outdent strings.
                        //
                        fstr::assign(&mut WORD, b"/cr(#:)|");
                        fstr::assign(&mut OUTDNT, b"/cr(-#:)");
                        spicelib::REPMI(&WORD.clone(), b"#", INDNBY, &mut WORD, ctx);
                        spicelib::REPMI(&OUTDNT.clone(), b"#", INDNBY, &mut OUTDNT, ctx);
                    } else {
                        fstr::assign(&mut WORD, b"/cr|");
                        fstr::assign(&mut OUTDNT, b"/cr(0:0)");
                    }
                } else {
                    //
                    // We are not at the beginning so there is no
                    // need to indent.
                    //
                    fstr::assign(&mut WORD, b"/cr|");
                }
            } else if fstr::eq(fstr::substr(STRING, B..=B), b"}") {
                //
                // We are at the end of a switch (there might be some
                // other stuff such as user punctuation in the string
                // so we don't require STRING(B:E) .EQ. '}'
                //
                BEGIN = false;

                if INDENT {
                    INDENT = false;
                    fstr::assign(
                        &mut WORD,
                        &fstr::concat(fstr::substr(STRING, B..=E), &OUTDNT),
                    );
                } else {
                    fstr::assign(
                        &mut WORD,
                        &fstr::concat(fstr::substr(STRING, B..=E), b"/cr(0:0)"),
                    );
                }
                //
                // We just put in a carriage return at the end of a switch.
                // Set our logical flag that says we did this.
                //
                CRLAST = true;
            } else {
                //
                // This word is to be treated as an ordinary word.
                //
                fstr::assign(&mut WORD, fstr::substr(STRING, B..=E));
                CRLAST = false;
            }

            R = spicelib::RTRIM(&WORD);
            fstr::assign(fstr::substr_mut(&mut LONG, END..=(END + R)), &WORD);
            END = ((END + R) + 1);
        }

        START = (E + 1);
    }
    //
    // That's all folks.  Move our long string into STRING and
    // return.
    //
    fstr::assign(STRING, fstr::substr(&LONG, 1..=END));
}
