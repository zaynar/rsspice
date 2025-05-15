//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LLNSIZ: i32 = 2048;

struct SaveVars {
    HKEY: Vec<u8>,
    HLINE: Vec<u8>,
    HLNGWD: Vec<u8>,
    LNGWD: Vec<u8>,
    ULINE: Vec<u8>,
    BEGPOS: i32,
    CLIDX: i32,
    ENDPOS: i32,
    PCLIDX: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut HKEY = vec![b' '; LLNSIZ as usize];
        let mut HLINE = vec![b' '; LLNSIZ as usize];
        let mut HLNGWD = vec![b' '; LLNSIZ as usize];
        let mut LNGWD = vec![b' '; LLNSIZ as usize];
        let mut ULINE = vec![b' '; LLNSIZ as usize];
        let mut BEGPOS: i32 = 0;
        let mut CLIDX: i32 = 0;
        let mut ENDPOS: i32 = 0;
        let mut PCLIDX: i32 = 0;

        Self {
            HKEY,
            HLINE,
            HLNGWD,
            LNGWD,
            ULINE,
            BEGPOS,
            CLIDX,
            ENDPOS,
            PCLIDX,
        }
    }
}

//$Procedure      PARCML ( Parse command line )
pub fn PARCML(
    LINE: &mut [u8],
    NKEYS: i32,
    CLKEYS: CharArray,
    CLFLAG: &mut [bool],
    CLVALS: CharArrayMut,
    FOUND: &mut bool,
    UNPRSD: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CLKEYS = DummyCharArray::new(CLKEYS, None, 1..);
    let mut CLFLAG = DummyArrayMut::new(CLFLAG, 1..);
    let mut CLVALS = DummyCharArrayMut::new(CLVALS, None, 1..);

    //
    // Local variables.
    //

    //
    // Save everything to prevent potential memory problems in f2c'ed
    // version.
    //

    //
    // SPICELIB functions.
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"PARCML", ctx)?;
    }

    //
    // Set initial values of keys to blanks and flags to .FALSE.
    //
    for I in 1..=NKEYS {
        CLFLAG[I] = false;
        fstr::assign(CLVALS.get_mut(I), b" ");
    }

    *FOUND = false;

    //
    // Parsing loop. We will set the sub-string buffer HLINE to as many
    // characters from the input line as it will fit, starting with the
    // initial part of the line on the first iteration and resetting to
    // sub-strings starting at the first character of each value after
    // the previous key-value pair was processed, and will pick at HLINE
    // word by word looking for recognized keys. The loop will
    // continue until we reach the end of the string -- all key-value
    // pairs were processed and the sub-string buffer HLINE was set to
    // blank.
    //
    fstr::assign(&mut save.HLINE, LINE);

    save.PCLIDX = 0;
    save.CLIDX = 0;

    fstr::assign(UNPRSD, LINE);

    while fstr::ne(&save.HLINE, b" ") {
        //
        // Get next word; uppercase it; look for it in the input keys
        // array.
        //
        spicelib::NEXTWD(&save.HLINE.to_vec(), &mut save.LNGWD, &mut save.HLINE);
        spicelib::UCASE(&save.LNGWD, &mut save.HLNGWD, ctx);
        save.CLIDX = spicelib::ISRCHC(&save.HLNGWD, NKEYS, CLKEYS.as_arg());

        //
        // Is the token that we found a recognized key?
        //
        if (save.CLIDX != 0) {
            //
            // Yes, it is. Is it the first key that we have found?
            //
            if (save.PCLIDX != 0) {
                //
                // No it is not. We need to save the value of the previous
                // key.
                //
                // Compute the begin and end positions of the sub-string
                // that contains the previous value by looking for the
                // previous and current keys in the upper-cased remainder of
                // the input line.
                //
                // The begin position is the position of the previous key
                // plus its length. The end position is the position of the
                // front-n-back blank-padded current key.
                //
                spicelib::UCASE(LINE, &mut save.ULINE, ctx);
                save.BEGPOS = (spicelib::POS(
                    &save.ULINE,
                    fstr::substr(
                        &CLKEYS[save.PCLIDX],
                        1..=spicelib::RTRIM(&CLKEYS[save.PCLIDX]),
                    ),
                    1,
                ) + spicelib::RTRIM(&CLKEYS[save.PCLIDX]));

                fstr::assign(
                    &mut save.HKEY,
                    &fstr::concat(
                        b" ",
                        fstr::substr(
                            CLKEYS.get(save.CLIDX),
                            1..=spicelib::RTRIM(&CLKEYS[save.CLIDX]),
                        ),
                    ),
                );
                save.ENDPOS = spicelib::POS(
                    &fstr::concat(&save.ULINE, b" "),
                    fstr::substr(&save.HKEY, 1..=(spicelib::RTRIM(&save.HKEY) + 1)),
                    save.BEGPOS,
                );

                //
                // Extract the value, left-justify it, and RTRIM it. Set
                // "value found" flag to .TRUE.
                //
                fstr::assign(
                    CLVALS.get_mut(save.PCLIDX),
                    fstr::substr(LINE, save.BEGPOS..=save.ENDPOS),
                );
                spicelib::LJUST(&CLVALS[save.PCLIDX].to_vec(), &mut CLVALS[save.PCLIDX]);
                let val = fstr::substr(
                    CLVALS.get(save.PCLIDX),
                    1..=spicelib::RTRIM(&CLVALS[save.PCLIDX]),
                )
                .to_vec();
                fstr::assign(CLVALS.get_mut(save.PCLIDX), &val);

                CLFLAG[save.PCLIDX] = true;

                //
                // Check whether we already parsed the whole line. It will
                // be so if the remainder of the buffer holding the
                // sub-string that we examine word-by-word is a blank
                // string.
                //
                if fstr::ne(&save.HLINE, b" ") {
                    //
                    // No, we did not parse the whole line yet. There is
                    // more stuff to parse and we reset the temporary
                    // sub-string buffer to hold the part of the input string
                    // starting with the first character after the current
                    // key -- the end position plus the length of the
                    // current key.
                    //
                    //
                    fstr::assign(
                        &mut save.HLINE,
                        fstr::substr(
                            LINE,
                            ((save.ENDPOS + 1) + spicelib::RTRIM(&CLKEYS[save.CLIDX]))..,
                        ),
                    );
                }

                //
                // Now reset the line to its portion starting with the
                // first character of the current key.
                //
                let val = fstr::substr(LINE, (save.ENDPOS + 1)..).to_vec();
                fstr::assign(LINE, &val);
            } else {
                //
                // This is the first key that we have found. Set UNPRSD
                // to the part of the line from the start to this key.
                //
                spicelib::UCASE(LINE, &mut save.ULINE, ctx);

                fstr::assign(
                    &mut save.HKEY,
                    &fstr::concat(
                        b" ",
                        fstr::substr(
                            CLKEYS.get(save.CLIDX),
                            1..=spicelib::RTRIM(&CLKEYS[save.CLIDX]),
                        ),
                    ),
                );
                save.BEGPOS = spicelib::POS(
                    &fstr::concat(b" ", &save.ULINE),
                    fstr::substr(&save.HKEY, 1..=(spicelib::RTRIM(&save.HKEY) + 1)),
                    1,
                );
                if (save.BEGPOS <= 1) {
                    fstr::assign(UNPRSD, b" ");
                } else {
                    fstr::assign(UNPRSD, fstr::substr(LINE, 1..=(save.BEGPOS - 1)));
                }
            }

            //
            // Save the current key index in as previous.
            //
            save.PCLIDX = save.CLIDX;
        }
    }

    //
    // If we found at least one recognized key, we need to save the last
    // value.
    //
    if (save.PCLIDX != 0) {
        //
        // Set "found any" output flag and "found previous key" flags to
        // .TRUE.
        //
        *FOUND = true;
        CLFLAG[save.PCLIDX] = true;

        //
        // Check if there was any value following the last key (there was
        // if the non-blank length of what's left in the line starting
        // with the last key if greater than the non-blank length of the
        // last key).
        //
        if (spicelib::RTRIM(LINE) > spicelib::RTRIM(&CLKEYS[save.PCLIDX])) {
            //
            // Compute begin position of, extract, left justify and
            // RTRIM the last value.
            //
            spicelib::UCASE(LINE, &mut save.ULINE, ctx);
            save.BEGPOS = (spicelib::POS(
                &save.ULINE,
                fstr::substr(
                    &CLKEYS[save.PCLIDX],
                    1..=spicelib::RTRIM(&CLKEYS[save.PCLIDX]),
                ),
                1,
            ) + spicelib::RTRIM(&CLKEYS[save.PCLIDX]));

            fstr::assign(
                CLVALS.get_mut(save.PCLIDX),
                fstr::substr(LINE, save.BEGPOS..),
            );
            spicelib::LJUST(&CLVALS[save.PCLIDX].to_vec(), &mut CLVALS[save.PCLIDX]);
            let val = fstr::substr(
                CLVALS.get(save.PCLIDX),
                1..=spicelib::RTRIM(&CLVALS[save.PCLIDX]),
            )
            .to_vec();
            fstr::assign(CLVALS.get_mut(save.PCLIDX), &val);
        } else {
            //
            // The key was the last thing on the line. So, it's value is
            // blank.
            //
            fstr::assign(CLVALS.get_mut(save.PCLIDX), b" ");
        }
    }

    spicelib::CHKOUT(b"PARCML", ctx)?;
    Ok(())
}
