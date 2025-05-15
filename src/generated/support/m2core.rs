//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const LNSIZE: i32 = 120;
const MAXBST: i32 = 10;
const METASC: i32 = 15;
const KEYSC: i32 = 100;

struct SaveVars {
    TBEGIN: i32,
    SBEGIN: i32,
    BEGOUT: i32,
    SLEN: i32,
    ROOT: Vec<u8>,
    PHRASE: Vec<u8>,
    ARTCLE: Vec<u8>,
    ERROR: bool,
    CALWRD: bool,
    KEYTBE: bool,
    MSPELL: i32,
    TE: i32,
    TB: i32,
    TC: i32,
    ORIGNL: i32,
    LOWER: i32,
    UPPER: i32,
    USEEND: bool,
    USELST: bool,
    USEKEY: bool,
    ENDOK: bool,
    ENDCHK: i32,
    KB: i32,
    KE: i32,
    NKEY: i32,
    ENDIT: bool,
    MCOUNT: i32,
    COUNT: i32,
    DCOUNT: i32,
    SB: i32,
    SE: i32,
    DB: i32,
    DE: i32,
    CB: i32,
    OVERSB: i32,
    LASTSB: i32,
    OVERSE: i32,
    LASTSE: i32,
    TIMEB: i32,
    SUFFSB: i32,
    SUFFSE: i32,
    TCODE: i32,
    KEYWRD: bool,
    CMATCH: bool,
    LOWERC: Vec<u8>,
    UPPERC: Vec<u8>,
    COUNTC: Vec<u8>,
    SCORES: StackArray<i32, 16>,
    BEST: StackArray<i32, 16>,
    BSCORE: i32,
    KNOWN: ActualCharArray,
    PBEG: i32,
    PEND: i32,
    MSSG: Vec<u8>,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TBEGIN: i32 = 0;
        let mut SBEGIN: i32 = 0;
        let mut BEGOUT: i32 = 0;
        let mut SLEN: i32 = 0;
        let mut ROOT = vec![b' '; 32 as usize];
        let mut PHRASE = vec![b' '; LNSIZE as usize];
        let mut ARTCLE = vec![b' '; 2 as usize];
        let mut ERROR: bool = false;
        let mut CALWRD: bool = false;
        let mut KEYTBE: bool = false;
        let mut MSPELL: i32 = 0;
        let mut TE: i32 = 0;
        let mut TB: i32 = 0;
        let mut TC: i32 = 0;
        let mut ORIGNL: i32 = 0;
        let mut LOWER: i32 = 0;
        let mut UPPER: i32 = 0;
        let mut USEEND: bool = false;
        let mut USELST: bool = false;
        let mut USEKEY: bool = false;
        let mut ENDOK: bool = false;
        let mut ENDCHK: i32 = 0;
        let mut KB: i32 = 0;
        let mut KE: i32 = 0;
        let mut NKEY: i32 = 0;
        let mut ENDIT: bool = false;
        let mut MCOUNT: i32 = 0;
        let mut COUNT: i32 = 0;
        let mut DCOUNT: i32 = 0;
        let mut SB: i32 = 0;
        let mut SE: i32 = 0;
        let mut DB: i32 = 0;
        let mut DE: i32 = 0;
        let mut CB: i32 = 0;
        let mut OVERSB: i32 = 0;
        let mut LASTSB: i32 = 0;
        let mut OVERSE: i32 = 0;
        let mut LASTSE: i32 = 0;
        let mut TIMEB: i32 = 0;
        let mut SUFFSB: i32 = 0;
        let mut SUFFSE: i32 = 0;
        let mut TCODE: i32 = 0;
        let mut KEYWRD: bool = false;
        let mut CMATCH: bool = false;
        let mut LOWERC = vec![b' '; 64];
        let mut UPPERC = vec![b' '; 64];
        let mut COUNTC = vec![b' '; 64];
        let mut SCORES = StackArray::<i32, 16>::new(LBCELL..=MAXBST);
        let mut BEST = StackArray::<i32, 16>::new(LBCELL..=MAXBST);
        let mut BSCORE: i32 = 0;
        let mut KNOWN = ActualCharArray::new(32, LBCELL..=MAXBST);
        let mut PBEG: i32 = 0;
        let mut PEND: i32 = 0;
        let mut MSSG = vec![b' '; 420];
        let mut PASS1: bool = false;

        PASS1 = true;

        Self {
            TBEGIN,
            SBEGIN,
            BEGOUT,
            SLEN,
            ROOT,
            PHRASE,
            ARTCLE,
            ERROR,
            CALWRD,
            KEYTBE,
            MSPELL,
            TE,
            TB,
            TC,
            ORIGNL,
            LOWER,
            UPPER,
            USEEND,
            USELST,
            USEKEY,
            ENDOK,
            ENDCHK,
            KB,
            KE,
            NKEY,
            ENDIT,
            MCOUNT,
            COUNT,
            DCOUNT,
            SB,
            SE,
            DB,
            DE,
            CB,
            OVERSB,
            LASTSB,
            OVERSE,
            LASTSE,
            TIMEB,
            SUFFSB,
            SUFFSE,
            TCODE,
            KEYWRD,
            CMATCH,
            LOWERC,
            UPPERC,
            COUNTC,
            SCORES,
            BEST,
            BSCORE,
            KNOWN,
            PBEG,
            PEND,
            MSSG,
            PASS1,
        }
    }
}

//$Procedure      M2CORE ( META/2 core syntax checking routines. )
pub fn M2CORE(
    TEMP: &[u8],
    TBEG: i32,
    KEYWDS: CharArray,
    STRING: &[u8],
    SBEG: i32,
    REASON: bool,
    CUTOFF: i32,
    M2CODE: i32,
    SCORE: i32,
    CAUSE: CharArray,
    SEND: i32,
) {

    //

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // KNOWN, PBEG, and PEND are storage for the recovery entry point.
    //
    // Should a spelling error be detected, the best matching words will
    // be stored in KNOWN and the index of the beginning and ending
    // of the problem word in STRING will be stored in PBEG and PEND
    // respectively.
    //

    //
    // Initial values
    //
}

//$Procedure  M2MTCH ( Match a string with a simple META/2 template )
pub fn M2MTCH(
    TEMP: &[u8],
    TBEG: i32,
    KEYWDS: CharArray,
    STRING: &[u8],
    SBEG: &mut i32,
    REASON: bool,
    CUTOFF: i32,
    M2CODE: &mut i32,
    SCORE: &mut i32,
    CAUSE: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let KEYWDS = DummyCharArray::new(KEYWDS, None, LBCELL..);
    let mut CAUSE = DummyCharArrayMut::new(CAUSE, None, 1..=2);

    //

    //
    // Take care of the SPICE error handling first.
    //
    if save.PASS1 {
        save.PASS1 = false;

        spicelib::SSIZEI(MAXBST, save.BEST.as_slice_mut(), ctx)?;
        spicelib::SSIZEI(MAXBST, save.SCORES.as_slice_mut(), ctx)?;
        spicelib::SSIZEC(MAXBST, save.KNOWN.as_arg_mut(), ctx)?;
        spicelib::SCARDC(1, save.KNOWN.as_arg_mut(), ctx)?;
    }

    save.SLEN = intrinsics::LEN(STRING);

    fstr::assign(CAUSE.get_mut(1), b" ");
    *SCORE = 0;

    save.TBEGIN = TBEG;
    save.SBEGIN = *SBEG;

    save.ERROR = false;

    *M2CODE = 0;
    save.MSPELL = 0;

    save.KB = 0;
    save.KE = 0;

    save.NKEY = spicelib::CARDC(KEYWDS.as_arg(), ctx)?;

    //
    // Locate the next word of the template.
    //
    spicelib::FNDNWD(TEMP, save.TBEGIN, &mut save.TB, &mut save.TE);

    while ((save.TB != 0) && !save.ERROR) {
        //
        // Zero out the keyword pointers.
        //
        save.KB = 0;
        save.KE = 0;
        save.ENDOK = false;

        //
        // Examine the current template word.  Is there a range template
        // attatched?
        //
        save.ORIGNL = save.TB;

        M2BEGR(
            TEMP,
            &mut save.TB,
            save.TE,
            &mut save.LOWER,
            &mut save.UPPER,
            ctx,
        );

        //
        // Locate the boundaries of the root of this template word.
        //
        M2TRIM(fstr::substr(TEMP, save.TB..=save.TE), &mut save.ROOT, ctx);

        save.TC = ((QRTRIM(&save.ROOT) - 1) + save.TB);

        //
        // If TB changed from its original value there is a range template
        // attached to the word TEMP(TB:TE).  The associated values are in
        // LOWER and UPPER.
        //
        save.KEYTBE = M2KEYW(fstr::substr(TEMP, save.TB..=save.TE), ctx);
        save.CALWRD =
            (fstr::eq(fstr::substr(TEMP, save.TB..=save.TC), b"@calendar") && !save.KEYTBE);

        if ((save.ORIGNL < save.TB) || save.CALWRD) {
            //
            // Yes.   There is a range template attatched.  Is it of
            // variable length?
            //
            if save.CALWRD {
                save.LOWER = 1;
                save.UPPER = 40;
                save.TIMEB = save.SBEGIN;
            }

            if (save.LOWER != save.UPPER) {
                //
                // Yes.  The template has a variable length. Determine
                // what delimiters might signal the end of a matching
                // substring of word from string.
                //
                // Possibilities are:  The end of the string     (USEEND)
                //                     One of the listed KEYWDS  (USELST)
                //                     A keyword listed in TEMP. (USEKEY)
                //
                // Right now we don't know which of the three cases to use.
                //

                save.USEEND = false;
                save.USELST = false;
                save.USEKEY = false;
                save.ENDOK = false;

                save.ENDCHK = (save.TE + 1);

                //
                // If the end of the current template word, was not
                // at the end of the template, then there might be
                // a keyword next.  Look for the next word to find out.
                //
                spicelib::FNDNWD(TEMP, save.ENDCHK, &mut save.KB, &mut save.KE);

                if (save.KE > 0) {
                    //
                    // There is a word in the template that follows
                    // our current template word.  See if it is a keyword.
                    //
                    if M2KEYW(fstr::substr(TEMP, save.KB..=save.KE), ctx) {
                        //
                        // If it is a keyword, it will be used as the
                        // delimiter for a sequence of words in STRING.
                        // ( Note we only want to work with the root of this
                        //   template word. )
                        //
                        save.USEKEY = true;
                        M2TRIM(fstr::substr(TEMP, save.KB..=save.KE), &mut save.ROOT, ctx);
                        save.KE = ((QRTRIM(&save.ROOT) + save.KB) - 1);
                    } else {
                        //
                        // Its not a keyword.  Bad, Bad. The user was not
                        // using META/2 properly.
                        //

                        spicelib::CHKIN(b"M2MTCH", ctx)?;
                        spicelib::SETMSG(b"M2MTCH: Any META-KEY that is preceded by a variable length range template in a specification statement must be followed by a keyword. ", ctx);

                        spicelib::SIGERR(b"SPICE(KEYWORDNOTFOUND)", ctx)?;
                        spicelib::CHKOUT(b"M2MTCH", ctx)?;

                        return Ok(());
                    }
                } else if (save.KE <= 0) {
                    //
                    // We got to this point because there was nothing
                    // to look at beyond where we were in TEMP.  So we
                    // either use one of the listed keywords or the end
                    // of the string will be our delimiter.
                    //
                    if (save.NKEY > 0) {
                        save.USELST = true;
                        save.ENDOK =
                            (spicelib::ESRCHC(b"@end", save.NKEY, KEYWDS.subarray(1)) != 0);
                    } else {
                        save.USEEND = true;
                    }
                }

                //
                // Until we have detected one of the keywords
                //    or we have not matched the current class
                //    or we run out of words in the sentence
                //
                //      Grab the next word of the sentence
                //      Check it for keyword .
                //      Check it for class .
                //
                save.ENDIT = false;
                save.KEYWRD = false;

                save.MCOUNT = 0;
                save.SUFFSB = 0;
                save.OVERSB = 0;
                save.OVERSE = 0;
                save.LASTSB = save.SBEGIN;
                save.LASTSE = (spicelib::POS(STRING, b" ", save.SBEGIN) - 1);

                while !save.ENDIT {
                    //
                    // Fetch the next word of the sentence.
                    //
                    spicelib::FNDNWD(STRING, save.SBEGIN, &mut save.SB, &mut save.SE);

                    //
                    // If there WAS a next word SE will not be zero.
                    //
                    if (save.SE == 0) {
                        save.KEYWRD = (save.USEEND || save.ENDOK);
                        save.ENDIT = true;

                        //
                        // BEGOUT will point past the matched portion of the
                        // string.  If no errors occur, it will be used to
                        // set SBEG on output.
                        //
                        save.BEGOUT = (save.SLEN + 1);
                    } else {
                        //
                        // is this a delimiting word for a variable length
                        // list?
                        //
                        if save.USELST {
                            save.KEYWRD = (spicelib::ESRCHC(
                                fstr::substr(STRING, save.SB..=save.SE),
                                save.NKEY,
                                KEYWDS.subarray(1),
                            ) != 0);

                            save.ENDIT = save.KEYWRD;

                            if save.KEYWRD {
                                //
                                // Mark the position just before the beginning
                                // of this word in STRING  so that SBEG will
                                // point to the first word past the end of
                                // the matched portion of STRING.
                                //
                                save.BEGOUT = (save.SB - 1);
                            }
                        } else if save.USEKEY {
                            save.KEYWRD =
                                (M2WMCH(
                                    STRING,
                                    save.SB,
                                    save.SE,
                                    fstr::substr(TEMP, save.KB..=save.KE),
                                    ctx,
                                )? && M2KEYW(fstr::substr(TEMP, save.KB..=save.KE), ctx));
                            save.ENDIT = save.KEYWRD;

                            //
                            // Mark the position of the "next" character
                            // in the string beyond the end of the current
                            // STRING word.
                            //
                            save.BEGOUT = (save.SE + 1);
                        }

                        //
                        // If we didn't bump into a keyword this must
                        // be (or should be) another of the words specified
                        // by the META-KEY TEMP(TB:TE)
                        //
                        if !save.KEYWRD {
                            save.CMATCH = M2WMCH(
                                STRING,
                                save.SB,
                                save.SE,
                                fstr::substr(TEMP, save.TB..=save.TE),
                                ctx,
                            )?;

                            if save.CMATCH {
                                save.MCOUNT = (save.MCOUNT + 1);

                                //
                                // Mark the position of the first character
                                // beyond the end of the current STRING
                                // word.
                                //
                                save.BEGOUT = (save.SE + 1);

                                //
                                // If MCOUNT has gotten too big, record the
                                // begin and end of the "bad" portion of the
                                // substring.
                                //
                                if (save.MCOUNT == (save.LOWER + 1)) {
                                    //
                                    // Mark the location of the beginning
                                    // and end of this word in case we need to
                                    // backtrack to here.
                                    //
                                    save.SUFFSB = save.SB;
                                    save.SUFFSE = save.SE;
                                } else if (save.MCOUNT <= save.UPPER) {
                                    //
                                    // Mark the end of this word in case
                                    // we need it later.
                                    //
                                    save.SUFFSE = save.SE;
                                } else if (save.MCOUNT == (save.UPPER + 1)) {
                                    save.OVERSB = save.SB;
                                    save.OVERSE = save.SE;
                                } else if (save.MCOUNT > save.UPPER) {
                                    save.OVERSE = save.SE;
                                }
                            } else {
                                save.ENDIT = true;
                            }
                        }

                        save.LASTSB = save.SB;
                        save.LASTSE = save.SE;
                    }

                    //
                    // Set the pointer to the input string to the first
                    // character past the end of the current word.
                    //
                    save.SBEGIN = (save.SE + 1);
                }

                //
                // We're now at the end of the loop matching words of STRING
                // with the class of object that had a variable length
                // template.
                //
                // The question now is: 'Did we get out of the loop in
                // a healthy or unhealthy way?'
                //
                //
                // Did we have the required range of items in the class?
                // Did we hit  the keyword?
                //
                // If both questions were answered YES,
                //
                if ((save.KEYWRD && (save.MCOUNT >= save.LOWER)) && (save.MCOUNT <= save.UPPER)) {
                    //
                    // Increment the score by METASC times the number of
                    // words found in the variable length template plus
                    // KEYSC for getting the keyword right.
                    //

                    if !save.CALWRD {
                        *SCORE = ((*SCORE + (METASC * save.MCOUNT)) + KEYSC);

                        if save.USEKEY {
                            //
                            // set the end of the last template word used to
                            // be the end of the keyword that we just hit.
                            //
                            save.TE = save.KE;
                        }
                    } else {
                        fstr::assign(&mut save.MSSG, b" ");
                        M2CAL(
                            fstr::substr(STRING, save.TIMEB..=save.SUFFSE),
                            &mut save.MSSG,
                            &mut save.TCODE,
                            ctx,
                        );

                        if (save.TCODE == 0) {
                            *SCORE = (*SCORE + (2 * KEYSC));
                        } else {
                            *SCORE = (*SCORE + KEYSC);
                            save.ERROR = true;

                            if (*M2CODE == 0) {
                                *M2CODE = (1000 + save.TCODE);
                            }

                            if REASON {
                                fstr::assign(
                                    CAUSE.get_mut(1),
                                    b"I was not able to parse the calendar string \"",
                                );

                                spicelib::SUFFIX(
                                    fstr::substr(STRING, save.TIMEB..=save.SUFFSE),
                                    0,
                                    CAUSE.first_mut(),
                                );
                                spicelib::SUFFIX(b"\". ", 0, CAUSE.first_mut());
                                spicelib::SUFFIX(&save.MSSG, 1, CAUSE.first_mut());

                                M2MARK(STRING, save.TIMEB, save.SUFFSE, CAUSE.first_mut(), ctx);

                                let val = CAUSE.get(1).to_vec();
                                fstr::assign(CAUSE.get_mut(2), &val);
                            }
                        }
                    }

                //
                // If less than the required range but a keyword was found
                // the error was: " not enough values loaded. "
                //
                } else if (save.KEYWRD && (save.MCOUNT < save.LOWER)) {
                    spicelib::INTTXT(save.LOWER, &mut save.LOWERC, ctx);
                    spicelib::INTTXT(save.MCOUNT, &mut save.COUNTC, ctx);

                    spicelib::LCASE(&save.LOWERC.to_vec(), &mut save.LOWERC, ctx);
                    spicelib::LCASE(&save.COUNTC.to_vec(), &mut save.COUNTC, ctx);

                    save.ERROR = true;

                    if (*M2CODE == 0) {
                        *M2CODE = 101;
                    }

                    //
                    // We grant METASC points for every word of the current
                    // class that was found, but we subtract METASC points
                    // for each item we were short.  That is:
                    //
                    // MCOUNT + ( LOWER - MCOUNT ) = 2*MCOUNT - LOWER
                    //
                    *SCORE = (*SCORE
                        + (METASC * intrinsics::MAX0(&[0, ((2 * save.MCOUNT) - save.LOWER)])));

                    //
                    // Add on KEYSC points for getting the correct keyword.
                    //
                    *SCORE = (*SCORE + KEYSC);

                    if REASON {
                        fstr::assign(CAUSE.get_mut(1), b"I was expecting to see at least # # at this point in the command string. I counted #. ");

                        M2CLSS(
                            fstr::substr(TEMP, save.TB..=save.TC),
                            save.LOWER,
                            &mut save.PHRASE,
                            ctx,
                        );
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.LOWERC,
                            CAUSE.first_mut(),
                        );
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.PHRASE,
                            CAUSE.first_mut(),
                        );
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.COUNTC,
                            CAUSE.first_mut(),
                        );

                        //
                        // OK. now we want to tack on the string and keep
                        // track of where the current word STRING(SB:SE)
                        // will get put.
                        //
                        M2MARK(STRING, save.LASTSB, save.LASTSE, CAUSE.first_mut(), ctx);
                        let val = CAUSE.get(1).to_vec();
                        fstr::assign(CAUSE.get_mut(2), &val);
                    }

                //
                // If more than the required range but a keyword was found
                // the error was too many values loaded.
                //
                } else if (save.KEYWRD && (save.MCOUNT > save.UPPER)) {
                    spicelib::INTTXT(save.UPPER, &mut save.UPPERC, ctx);
                    spicelib::INTTXT(save.MCOUNT, &mut save.COUNTC, ctx);

                    spicelib::LCASE(&save.UPPERC.to_vec(), &mut save.UPPERC, ctx);
                    spicelib::LCASE(&save.COUNTC.to_vec(), &mut save.COUNTC, ctx);

                    save.ERROR = true;

                    if (*M2CODE == 0) {
                        *M2CODE = 102;
                    }

                    //
                    // We grant METASC points for every word of the current
                    // class that was found prior to the cutoff limit.
                    // But we subtract METASC points for each extra item.
                    // That is:
                    //
                    // UPPER + ( MCOUNT - UPPER ) = 2*UPPER - MCOUNT
                    //
                    *SCORE = (*SCORE
                        + (METASC * intrinsics::MAX0(&[0, ((2 * save.UPPER) - save.MCOUNT)])));

                    //
                    // Add on KEYSC points for getting the correct keyword.
                    //
                    *SCORE = (*SCORE + KEYSC);

                    if REASON {
                        fstr::assign(CAUSE.get_mut(1), b"I was expecting to see at most # #. I counted #. I\'ve marked the location of the problem for you. ");

                        M2CLSS(
                            fstr::substr(TEMP, save.TB..=save.TC),
                            save.UPPER,
                            &mut save.PHRASE,
                            ctx,
                        );
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.UPPERC,
                            CAUSE.first_mut(),
                        );
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.PHRASE,
                            CAUSE.first_mut(),
                        );
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.COUNTC,
                            CAUSE.first_mut(),
                        );

                        M2MARK(STRING, save.OVERSB, save.OVERSE, CAUSE.first_mut(), ctx);
                        let val = CAUSE.get(1).to_vec();
                        fstr::assign(CAUSE.get_mut(2), &val);
                    }

                //
                // If required range but no keyword, error could be
                // misspelled keyword ( we estimate this ) or keyword
                // was missing.
                //
                } else if ((save.MCOUNT >= save.LOWER) && (save.MCOUNT <= save.UPPER)) {
                    //
                    // Add METASC points to the score for each of the
                    // words encountered.
                    //

                    if (save.SE == 0) {
                        //
                        // We are going to try to see if we had a spelling
                        // error that caused us to run out of string
                        //
                        spicelib::FNDNWD(STRING, save.SUFFSB, &mut save.DB, &mut save.DE);

                        save.ORIGNL = save.SUFFSB;
                        save.COUNT = (save.LOWER + 1);
                        save.BSCORE = 0;
                        save.DCOUNT = 0;

                        while (save.COUNT <= save.MCOUNT) {
                            if save.USEKEY {
                                fstr::assign(
                                    save.KNOWN.get_mut(1),
                                    fstr::substr(TEMP, save.KB..=save.KE),
                                );

                                //
                                // Compare the last word encountered in the
                                // string with the KEYWORD we were expecting.
                                //
                                BESTWD(
                                    fstr::substr(STRING, save.DB..=save.DE),
                                    save.KNOWN.as_arg(),
                                    CUTOFF,
                                    save.BEST.as_slice_mut(),
                                    save.SCORES.as_slice_mut(),
                                    &mut save.MSSG,
                                    ctx,
                                )?;
                            } else if save.USELST {
                                //
                                // Compare the last word that we hit with one
                                // of the keywords from the list of possible
                                // closing keywords.
                                //
                                BESTWD(
                                    fstr::substr(STRING, save.DB..=save.DE),
                                    KEYWDS.as_arg(),
                                    CUTOFF,
                                    save.BEST.as_slice_mut(),
                                    save.SCORES.as_slice_mut(),
                                    &mut save.MSSG,
                                    ctx,
                                )?;
                            }

                            if ((spicelib::CARDI(save.SCORES.as_slice(), ctx)? > 0)
                                && (save.SCORES[1] >= CUTOFF))
                            {
                                //
                                // We are going to treat this as a spelling
                                // error.
                                //
                                if (*M2CODE == 0) {
                                    *M2CODE = 13;
                                }

                                //
                                // Save the beginning and ending of the
                                // problem word for use in the recovery
                                // entry point.
                                //
                                if (save.SCORES[1] > save.BSCORE) {
                                    save.BSCORE = save.SCORES[1];
                                    save.PBEG = save.DB;
                                    save.PEND = save.DE;

                                    //
                                    // Everything up to this is now regarded
                                    // as simply matching the META-KEY.  Store
                                    // this number of META-KEYs for use by
                                    // diagnostics generation.
                                    //
                                    save.DCOUNT = (save.COUNT - 1);
                                }
                            }

                            save.SUFFSB = (save.DE + 1);

                            //
                            // Look at the next word until we have gone
                            // past UPPER even if we already have a
                            // candidate for misspelling, there might be
                            // a better one.
                            //
                            spicelib::FNDNWD(STRING, save.SUFFSB, &mut save.DB, &mut save.DE);

                            save.COUNT = (save.COUNT + 1);
                        }

                        //
                        // Save the misspelling information associated
                        // with the best match (if there was one).
                        //
                        if (save.BSCORE > 0) {
                            if save.USEKEY {
                                fstr::assign(
                                    save.KNOWN.get_mut(1),
                                    fstr::substr(TEMP, save.KB..=save.KE),
                                );

                                //
                                // Compare the last word encountered in the
                                // string with the KEYWORD we were expecting.
                                //
                                BESTWD(
                                    fstr::substr(STRING, save.PBEG..=save.PEND),
                                    save.KNOWN.as_arg(),
                                    CUTOFF,
                                    save.BEST.as_slice_mut(),
                                    save.SCORES.as_slice_mut(),
                                    &mut save.MSSG,
                                    ctx,
                                )?;
                            } else if save.USELST {
                                //
                                // Compare the last word that we hit with one
                                // of the keywords from the list of possible
                                // closing keywords.
                                //
                                BESTWD(
                                    fstr::substr(STRING, save.PBEG..=save.PEND),
                                    KEYWDS.as_arg(),
                                    CUTOFF,
                                    save.BEST.as_slice_mut(),
                                    save.SCORES.as_slice_mut(),
                                    &mut save.MSSG,
                                    ctx,
                                )?;

                                //
                                // Save the best matches for use in the
                                // recovery entry point.
                                //
                                for I in 1..=spicelib::CARDI(save.BEST.as_slice(), ctx)? {
                                    fstr::assign(save.KNOWN.get_mut(I), KEYWDS.get(save.BEST[I]));
                                }

                                spicelib::SCARDC(
                                    spicelib::CARDI(save.BEST.as_slice(), ctx)?,
                                    save.KNOWN.as_arg_mut(),
                                    ctx,
                                )?;
                            }

                            //
                            // This is not regarded as an error worth
                            // stopping for unless our
                            // misspelling total has runs over 100.
                            //
                            save.MSPELL = (save.MSPELL + (100 - save.BSCORE));

                            if (save.MSPELL < 100) {
                                *SCORE = ((*SCORE + (save.DCOUNT * METASC)) + save.BSCORE);

                                //
                                // Back the value of SBEGIN back up to the
                                // point of failure, so that we can continue
                                // processing as if nothing had gone wrong.
                                //
                                save.SBEGIN = (save.PEND + 1);
                            } else {
                                *SCORE = (*SCORE + (save.DCOUNT * METASC));
                                save.ERROR = true;
                            }
                        } else {
                            //
                            // Restore the initial value of SUFFSB
                            //
                            save.SUFFSB = save.ORIGNL;
                            *SCORE = (*SCORE + (save.MCOUNT * METASC));

                            if (*M2CODE == 0) {
                                *M2CODE = 103;
                            }

                            //
                            // This occurs if we ran out of stuff in STRING
                            // and we were looking to find a keyword instead.
                            //
                            save.ERROR = true;
                        }

                        if (save.USEKEY && REASON) {
                            fstr::assign(CAUSE.get_mut(1), b"I was looking for the keyword \"");

                            spicelib::SUFFIX(
                                fstr::substr(TEMP, save.KB..=save.KE),
                                1,
                                CAUSE.first_mut(),
                            );
                            spicelib::SUFFIX(b"\" when I reached the", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(b"end of the input ", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(b"command. ", 1, CAUSE.first_mut());
                        } else if (save.USELST && REASON) {
                            fstr::assign(CAUSE.get_mut(1), b"I was looking for one of the keywords that follow when I reached the end of the input command.  Keywords: {");

                            for I in 1..=save.NKEY {
                                spicelib::SUFFIX(&KEYWDS[I], 2, CAUSE.first_mut());
                                spicelib::SUFFIX(b",", 0, CAUSE.first_mut());
                            }

                            save.CB = QLSTNB(&CAUSE[1]);
                            fstr::assign(fstr::substr_mut(CAUSE.get_mut(1), save.CB..), b" }.");
                        }

                        if (REASON && (save.BSCORE != 0)) {
                            let val = CAUSE.get(1).to_vec();
                            fstr::assign(CAUSE.get_mut(2), &val);
                            fstr::assign(CAUSE.get_mut(1), b" ");

                            spicelib::SUFFIX(&save.MSSG, 1, CAUSE.first_mut());
                            M2MARK(STRING, save.PBEG, save.PEND, CAUSE.first_mut(), ctx);
                            spicelib::SUFFIX(&save.MSSG, 1, &mut CAUSE[2]);
                            M2MARK(STRING, save.PBEG, save.PEND, &mut CAUSE[2], ctx);
                        } else if REASON {
                            M2MARK(STRING, save.LASTSB, save.LASTSE, CAUSE.first_mut(), ctx);
                            let val = CAUSE.get(1).to_vec();
                            fstr::assign(CAUSE.get_mut(2), &val);
                        }

                    //
                    // Recall that we are examining the case when the number
                    // of word matches is within the expected range, but
                    // no keyword was present.  We have already looked at
                    // what to do if we ran out of string prematurely.
                    //
                    } else if (save.SE > 0) {
                        *SCORE = (*SCORE + (save.MCOUNT * METASC));

                        //
                        // We ran into something unexepected.  Possibly
                        // a misspelled keyword.  See if any of the
                        // expected keywords are close to what we got.
                        //
                        if save.USEEND {
                            save.ERROR = true;

                            if (*M2CODE == 0) {
                                *M2CODE = 104;
                            }

                            if REASON {
                                fstr::assign(CAUSE.get_mut(1), b"The input command contains extra characters that are not part of a valid command.  ");

                                M2MARK(STRING, save.SB, save.SE, CAUSE.first_mut(), ctx);

                                let val = CAUSE.get(1).to_vec();
                                fstr::assign(CAUSE.get_mut(2), &val);
                            }
                        } else if (save.USEKEY || save.USELST) {
                            if save.USEKEY {
                                fstr::assign(
                                    save.KNOWN.get_mut(1),
                                    fstr::substr(TEMP, save.KB..=save.KE),
                                );

                                //
                                // Compare the last word encountered in the
                                // string with the KEYWORD we were expecting.
                                //

                                BESTWD(
                                    fstr::substr(STRING, save.SB..=save.SE),
                                    save.KNOWN.as_arg(),
                                    CUTOFF,
                                    save.BEST.as_slice_mut(),
                                    save.SCORES.as_slice_mut(),
                                    &mut save.MSSG,
                                    ctx,
                                )?;
                            } else if save.USELST {
                                //
                                // Compare the last word that we hit with one
                                // of the keywords from the list of possible
                                // closing keywords.
                                //
                                BESTWD(
                                    fstr::substr(STRING, save.SB..=save.SE),
                                    KEYWDS.as_arg(),
                                    CUTOFF,
                                    save.BEST.as_slice_mut(),
                                    save.SCORES.as_slice_mut(),
                                    &mut save.MSSG,
                                    ctx,
                                )?;

                                //
                                // Save the best matches for use in the recovery
                                // entry point.
                                //
                                for I in 1..=spicelib::CARDI(save.BEST.as_slice(), ctx)? {
                                    fstr::assign(save.KNOWN.get_mut(I), KEYWDS.get(save.BEST[I]));
                                }

                                spicelib::SCARDC(
                                    spicelib::CARDI(save.BEST.as_slice(), ctx)?,
                                    save.KNOWN.as_arg_mut(),
                                    ctx,
                                )?;
                            }

                            //
                            // We are still checking out the case in which we
                            // had a correct range of words for a variable
                            // length template, but ran into
                            // something that was not a terminating keyword
                            // that we were expecting.  Possibly we hit a
                            // mispelled keyword.
                            //
                            // Well? Was there anything to the rumor of a
                            // spelling error?
                            //
                            if ((spicelib::CARDI(save.SCORES.as_slice(), ctx)? > 0)
                                && (save.SCORES[1] >= CUTOFF))
                            {
                                if (*M2CODE == 0) {
                                    *M2CODE = 10;

                                    //
                                    // Save the beginning and ending of the
                                    // problem word for use in the recovery
                                    // entry point.
                                    //
                                    save.PBEG = save.SB;
                                    save.PEND = save.SE;
                                }

                                //
                                // This is probably a spelling error.
                                // Point out the error.
                                //
                                save.MSPELL = (save.MSPELL + (100 - save.SCORES[1]));

                                if (save.MSPELL < 100) {
                                    *SCORE = (*SCORE + save.SCORES[1]);
                                } else {
                                    save.ERROR = true;
                                }

                                if REASON {
                                    //
                                    // Construct an error message indicating
                                    // the spelling diagnostic.
                                    //
                                    save.ERROR = true;
                                }
                            } else if ((spicelib::CARDI(save.SCORES.as_slice(), ctx)? == 0)
                                || (save.SCORES[1] < CUTOFF))
                            {
                                //
                                // This is not a misspelling.
                                // Set the error flag
                                //
                                save.ERROR = true;

                                if (*M2CODE == 0) {
                                    *M2CODE = 105;
                                }

                                fstr::assign(&mut save.MSSG, b" ");
                            }

                            if (REASON && save.USEKEY) {
                                fstr::assign(CAUSE.get_mut(1), b"I was looking for the ");

                                spicelib::SUFFIX(b"keyword \"", 1, CAUSE.first_mut());
                                spicelib::SUFFIX(
                                    fstr::substr(TEMP, save.KB..=save.KE),
                                    0,
                                    CAUSE.first_mut(),
                                );
                                spicelib::SUFFIX(b"\" when I ", 0, CAUSE.first_mut());
                                spicelib::SUFFIX(b"encountered ", 1, CAUSE.first_mut());
                                spicelib::SUFFIX(b"the word \"", 1, CAUSE.first_mut());
                                spicelib::SUFFIX(
                                    fstr::substr(STRING, save.SB..=save.SE),
                                    0,
                                    CAUSE.first_mut(),
                                );
                                spicelib::SUFFIX(b"\".   ", 0, CAUSE.first_mut());
                                spicelib::SUFFIX(&save.MSSG, 1, CAUSE.first_mut());
                                let val = CAUSE.get(1).to_vec();
                                fstr::assign(CAUSE.get_mut(2), &val);
                                fstr::assign(CAUSE.get_mut(1), &save.MSSG);

                                M2MARK(STRING, save.SB, save.SE, &mut CAUSE[1], ctx);
                                M2MARK(STRING, save.SB, save.SE, &mut CAUSE[2], ctx);
                            } else if (REASON && save.USELST) {
                                fstr::assign(
                                    CAUSE.get_mut(1),
                                    b"I was looking for one of the keywords in the list: { ",
                                );

                                for I in 1..=save.NKEY {
                                    spicelib::SUFFIX(&KEYWDS[I], 1, CAUSE.first_mut());

                                    if (I != save.NKEY) {
                                        spicelib::SUFFIX(b",", 0, CAUSE.first_mut());
                                    }
                                }

                                spicelib::SUFFIX(b"}  when I ", 1, CAUSE.first_mut());
                                spicelib::SUFFIX(b"encountered ", 1, CAUSE.first_mut());
                                spicelib::SUFFIX(b"the word \"", 1, CAUSE.first_mut());
                                spicelib::SUFFIX(
                                    fstr::substr(STRING, save.SB..=save.SE),
                                    0,
                                    CAUSE.first_mut(),
                                );
                                spicelib::SUFFIX(b"\".   ", 0, CAUSE.first_mut());
                                spicelib::SUFFIX(&save.MSSG, 1, CAUSE.first_mut());

                                M2MARK(STRING, save.SB, save.SE, CAUSE.first_mut(), ctx);
                                let val = CAUSE.get(1).to_vec();
                                fstr::assign(CAUSE.get_mut(2), &val);
                            }
                        }
                    }

                //
                // If out of range and no keyword then we don't have
                // a good guess as to what went wrong.
                //
                } else if (!save.KEYWRD
                    && ((save.MCOUNT < save.LOWER) || (save.MCOUNT > save.UPPER)))
                {
                    if (save.MCOUNT < save.LOWER) {
                        if (*M2CODE == 0) {
                            *M2CODE = 106;
                        }

                        *SCORE = (*SCORE
                            + (METASC * intrinsics::MAX0(&[0, ((2 * save.MCOUNT) - save.LOWER)])));

                        save.ERROR = true;
                    } else if (save.MCOUNT > save.UPPER) {
                        if (save.USEKEY || save.USELST) {
                            //
                            // We are going to try to see if we had a spelling
                            // error that caused us to get too many words.
                            //
                            spicelib::FNDNWD(STRING, save.SUFFSB, &mut save.DB, &mut save.DE);

                            save.COUNT = (save.LOWER + 1);
                            save.BSCORE = 0;
                            save.DCOUNT = 0;

                            while (save.COUNT <= (save.UPPER + 1)) {
                                if save.USEKEY {
                                    fstr::assign(
                                        save.KNOWN.get_mut(1),
                                        fstr::substr(TEMP, save.KB..=save.KE),
                                    );

                                    //
                                    // Compare the last word encountered in the
                                    // string with the KEYWORD we were expecting.
                                    //
                                    BESTWD(
                                        fstr::substr(STRING, save.DB..=save.DE),
                                        save.KNOWN.as_arg(),
                                        CUTOFF,
                                        save.BEST.as_slice_mut(),
                                        save.SCORES.as_slice_mut(),
                                        &mut save.MSSG,
                                        ctx,
                                    )?;
                                } else if save.USELST {
                                    //
                                    // Compare the last word that we hit with one
                                    // of the keywords from the list of possible
                                    // closing keywords.
                                    //
                                    BESTWD(
                                        fstr::substr(STRING, save.DB..=save.DE),
                                        KEYWDS.as_arg(),
                                        CUTOFF,
                                        save.BEST.as_slice_mut(),
                                        save.SCORES.as_slice_mut(),
                                        &mut save.MSSG,
                                        ctx,
                                    )?;
                                }

                                if ((spicelib::CARDI(save.SCORES.as_slice(), ctx)? > 0)
                                    && (save.SCORES[1] >= CUTOFF))
                                {
                                    //
                                    // We are going to treat this as a spelling
                                    // error.
                                    //
                                    if (*M2CODE == 0) {
                                        *M2CODE = 12;
                                    }

                                    //
                                    // Save the beginning and ending of the
                                    // problem word for use in the recovery
                                    // entry point.
                                    //
                                    if (save.SCORES[1] > save.BSCORE) {
                                        save.BSCORE = save.SCORES[1];
                                        save.PBEG = save.DB;
                                        save.PEND = save.DE;

                                        //
                                        // Everything up to this is now regarded
                                        // as simply matching the META-KEY.  Store
                                        // this number of META-KEYs for use by
                                        // diagnostics generation.
                                        //
                                        save.DCOUNT = (save.COUNT - 1);
                                    }
                                }

                                save.SUFFSB = (save.DE + 1);

                                //
                                // Look at the next word until we have gone
                                // past UPPER even if we already have a
                                // candidate for misspelling, there might be
                                // a better one.
                                //
                                spicelib::FNDNWD(STRING, save.SUFFSB, &mut save.DB, &mut save.DE);

                                save.COUNT = (save.COUNT + 1);
                            }

                            //
                            // Save the misspelling information associated
                            // with the best match (if there was one).
                            //
                            if (save.BSCORE > 0) {
                                if save.USEKEY {
                                    fstr::assign(
                                        save.KNOWN.get_mut(1),
                                        fstr::substr(TEMP, save.KB..=save.KE),
                                    );

                                    //
                                    // Compare the last word encountered in the
                                    // string with the KEYWORD we were expecting.
                                    //
                                    BESTWD(
                                        fstr::substr(STRING, save.PBEG..=save.PEND),
                                        save.KNOWN.as_arg(),
                                        CUTOFF,
                                        save.BEST.as_slice_mut(),
                                        save.SCORES.as_slice_mut(),
                                        &mut save.MSSG,
                                        ctx,
                                    )?;
                                } else if save.USELST {
                                    //
                                    // Compare the last word that we hit with one
                                    // of the keywords from the list of possible
                                    // closing keywords.
                                    //
                                    BESTWD(
                                        fstr::substr(STRING, save.PBEG..=save.PEND),
                                        KEYWDS.as_arg(),
                                        CUTOFF,
                                        save.BEST.as_slice_mut(),
                                        save.SCORES.as_slice_mut(),
                                        &mut save.MSSG,
                                        ctx,
                                    )?;

                                    //
                                    // Save the best matches for use in the
                                    // recovery entry point.
                                    //
                                    for I in 1..=spicelib::CARDI(save.BEST.as_slice(), ctx)? {
                                        fstr::assign(
                                            save.KNOWN.get_mut(I),
                                            KEYWDS.get(save.BEST[I]),
                                        );
                                    }

                                    spicelib::SCARDC(
                                        spicelib::CARDI(save.BEST.as_slice(), ctx)?,
                                        save.KNOWN.as_arg_mut(),
                                        ctx,
                                    )?;
                                }

                                //
                                // This is not regarded as an error worth
                                // stopping for unless our
                                // misspelling total has runs over 100.
                                //
                                save.MSPELL = (save.MSPELL + (100 - save.BSCORE));

                                if (save.MSPELL < 100) {
                                    *SCORE = ((*SCORE + (save.DCOUNT * METASC)) + save.BSCORE);

                                    //
                                    // Back the value of SBEGIN back up to the
                                    // point of failure, so that we can continue
                                    // processing as if nothing had gone wrong.
                                    //
                                    save.SBEGIN = (save.PEND + 1);
                                } else {
                                    *SCORE = (*SCORE + (save.DCOUNT * METASC));
                                    save.ERROR = true;
                                }
                            }
                        }

                        //
                        // We might not have had a good candidate for a
                        // misspelling, if not we don't have a good clue
                        // as to what went wrong.
                        //
                        if (*M2CODE == 0) {
                            *M2CODE = 107;
                            *SCORE = (*SCORE
                                + (METASC
                                    * intrinsics::MAX0(&[0, ((2 * save.UPPER) - save.MCOUNT)])));

                            save.ERROR = true;
                        }
                    }

                    //
                    // If there is to be a diagnostic generated, set up
                    // the beginning of it so that everyone else can
                    // share in the same work.
                    //
                    if REASON {
                        save.ERROR = true;

                        fstr::assign(
                            CAUSE.get_mut(1),
                            b"I was expecting to see between # and # # ",
                        );

                        M2CLSS(
                            fstr::substr(TEMP, save.TB..=save.TC),
                            save.UPPER,
                            &mut save.PHRASE,
                            ctx,
                        );
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.LOWERC,
                            CAUSE.first_mut(),
                        );
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.UPPERC,
                            CAUSE.first_mut(),
                        );
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.PHRASE,
                            CAUSE.first_mut(),
                        );

                        if save.USEKEY {
                            spicelib::SUFFIX(b"followed by ", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(b"the keyword, ", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(
                                fstr::substr(TEMP, save.KB..=save.KE),
                                1,
                                CAUSE.first_mut(),
                            );
                            spicelib::SUFFIX(b".", 0, CAUSE.first_mut());
                        } else if save.USELST {
                            spicelib::SUFFIX(b"followed by ", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(b"one of the ", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(b"keywords from the", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(b"list {", 1, CAUSE.first_mut());

                            for I in 1..=save.NKEY {
                                spicelib::SUFFIX(&KEYWDS[I], 1, CAUSE.first_mut());

                                if (I != save.NKEY) {
                                    spicelib::SUFFIX(b",", 1, CAUSE.first_mut());
                                }
                            }

                            spicelib::SUFFIX(b"}.", 1, CAUSE.first_mut());
                        } else if save.USEEND {
                            spicelib::SUFFIX(b"filling out the   ", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(b"end of the string.", 1, CAUSE.first_mut());
                        }

                        //
                        // Use the information stored in M2CODE to determine
                        // how many words we encountered before we figured
                        // out we had an error.
                        //
                        if (*M2CODE >= 100) {
                            spicelib::INTTXT(save.MCOUNT, &mut save.COUNTC, ctx);
                            spicelib::LCASE(&save.COUNTC.to_vec(), &mut save.COUNTC, ctx);
                        } else {
                            spicelib::INTTXT(save.DCOUNT, &mut save.COUNTC, ctx);
                            spicelib::LCASE(&save.COUNTC.to_vec(), &mut save.COUNTC, ctx);
                        }

                        spicelib::SUFFIX(b"I had counted ", 1, CAUSE.first_mut());
                        spicelib::SUFFIX(&save.COUNTC, 1, CAUSE.first_mut());

                        if (save.MCOUNT == 1) {
                            spicelib::SUFFIX(b"such word", 1, CAUSE.first_mut());
                        } else {
                            spicelib::SUFFIX(b"such words", 1, CAUSE.first_mut());
                        }

                        spicelib::SUFFIX(b"when I encountered", 1, CAUSE.first_mut());
                    }

                    //
                    // We are still in the case of a variable length template
                    // for which we did not hit a keyword and did not have
                    // the expected range of items for the current META-KEY.
                    //
                    // OK. Now tailor the end of the message to reflect
                    // what went wrong in particular.
                    //
                    if (REASON && (*M2CODE < 100)) {
                        spicelib::SUFFIX(b"the word \"", 1, CAUSE.first_mut());
                        spicelib::SUFFIX(
                            fstr::substr(STRING, save.PBEG..=save.PEND),
                            0,
                            CAUSE.first_mut(),
                        );
                        spicelib::SUFFIX(b"\" .", 0, CAUSE.first_mut());

                        spicelib::SUFFIX(&save.MSSG, 1, CAUSE.first_mut());

                        M2MARK(STRING, save.PBEG, save.PEND, CAUSE.first_mut(), ctx);
                        let val = CAUSE.get(1).to_vec();
                        fstr::assign(CAUSE.get_mut(2), &val);
                    } else if (REASON && (save.SE == 0)) {
                        spicelib::SUFFIX(b"the end of the input", 1, CAUSE.first_mut());
                        spicelib::SUFFIX(b"string.    ", 1, CAUSE.first_mut());
                        M2MARK(
                            STRING,
                            (QLSTNB(STRING) + 1),
                            (QLSTNB(STRING) + 1),
                            CAUSE.first_mut(),
                            ctx,
                        );
                        let val = CAUSE.get(1).to_vec();
                        fstr::assign(CAUSE.get_mut(2), &val);

                    //
                    // check for a misspell.
                    //
                    } else if (REASON && (save.SE != 0)) {
                        spicelib::SUFFIX(b"the word \"", 1, CAUSE.first_mut());
                        spicelib::SUFFIX(
                            fstr::substr(STRING, save.SB..=save.SE),
                            0,
                            CAUSE.first_mut(),
                        );
                        spicelib::SUFFIX(b"\" .", 0, CAUSE.first_mut());

                        //
                        // If misspell likely mention that too.
                        //
                        if save.USEKEY {
                            fstr::assign(
                                save.KNOWN.get_mut(1),
                                fstr::substr(TEMP, save.KB..=save.KE),
                            );

                            BESTWD(
                                fstr::substr(STRING, save.SB..=save.SE),
                                save.KNOWN.as_arg(),
                                CUTOFF,
                                save.BEST.as_slice_mut(),
                                save.SCORES.as_slice_mut(),
                                &mut save.MSSG,
                                ctx,
                            )?;
                        } else if save.USELST {
                            BESTWD(
                                fstr::substr(STRING, save.SB..=save.SE),
                                KEYWDS.as_arg(),
                                CUTOFF,
                                save.BEST.as_slice_mut(),
                                save.SCORES.as_slice_mut(),
                                &mut save.MSSG,
                                ctx,
                            )?;
                        }

                        if ((spicelib::CARDI(save.SCORES.as_slice(), ctx)? > 0)
                            && (save.SCORES[1] > CUTOFF))
                        {
                            spicelib::SUFFIX(&save.MSSG, 1, CAUSE.first_mut());
                        }

                        M2MARK(STRING, save.SB, save.SE, CAUSE.first_mut(), ctx);
                        let val = CAUSE.get(1).to_vec();
                        fstr::assign(CAUSE.get_mut(2), &val);
                    }
                }
            } else {
                //
                // This "ELSE" is the "NO" response to the question:  "Ok.
                // we have a range template. Is it of variable length?"
                //
                save.ENDIT = (save.LOWER == 0);
                save.MCOUNT = 0;

                while !save.ENDIT {
                    spicelib::FNDNWD(STRING, save.SBEGIN, &mut save.SB, &mut save.SE);

                    if (save.SE == 0) {
                        save.ENDIT = true;
                        save.ERROR = true;

                        if (*M2CODE == 0) {
                            *M2CODE = 108;
                        }

                        if REASON {
                            fstr::assign(CAUSE.get_mut(1), b"I was expecting to see # # when I ran out of words in the command string. ");

                            M2CLSS(
                                fstr::substr(TEMP, save.TB..=save.TC),
                                1,
                                &mut save.PHRASE,
                                ctx,
                            );
                            spicelib::ANA(&save.PHRASE, b"L", &mut save.ARTCLE, ctx);
                            spicelib::REPMC(
                                &CAUSE.first().to_vec(),
                                b"#",
                                &save.ARTCLE,
                                CAUSE.first_mut(),
                            );
                            spicelib::REPMC(
                                &CAUSE.first().to_vec(),
                                b"#",
                                &save.PHRASE,
                                CAUSE.first_mut(),
                            );

                            M2MARK(
                                STRING,
                                (QLSTNB(STRING) + 1),
                                (QLSTNB(STRING) + 1),
                                CAUSE.first_mut(),
                                ctx,
                            );

                            let val = CAUSE.get(1).to_vec();
                            fstr::assign(CAUSE.get_mut(2), &val);
                        }
                    } else if M2WMCH(
                        STRING,
                        save.SB,
                        save.SE,
                        fstr::substr(TEMP, save.TB..=save.TE),
                        ctx,
                    )? {
                        save.MCOUNT = (save.MCOUNT + 1);
                        *SCORE = (*SCORE + METASC);

                        save.SBEGIN = (save.SE + 1);

                        //
                        // Mark the position of the first character beyond the
                        // current STRING word.
                        //
                        save.BEGOUT = save.SBEGIN;
                        save.ENDIT = (save.MCOUNT >= save.LOWER);
                    } else {
                        if (*M2CODE == 0) {
                            *M2CODE = 109;
                        }

                        save.ERROR = true;
                        save.ENDIT = true;
                        if REASON {
                            fstr::assign(CAUSE.get_mut(1), b"I was expecting to see # # when I encounterd the word \"#\" in the command. ");

                            M2CLSS(
                                fstr::substr(TEMP, save.TB..=save.TC),
                                1,
                                &mut save.PHRASE,
                                ctx,
                            );
                            spicelib::ANA(&save.PHRASE, b"L", &mut save.ARTCLE, ctx);
                            spicelib::REPMC(
                                &CAUSE.first().to_vec(),
                                b"#",
                                &save.ARTCLE,
                                CAUSE.first_mut(),
                            );
                            spicelib::REPMC(
                                &CAUSE.first().to_vec(),
                                b"#",
                                &save.PHRASE,
                                CAUSE.first_mut(),
                            );
                            spicelib::REPMC(
                                &CAUSE.first().to_vec(),
                                b"#",
                                fstr::substr(STRING, save.SB..=save.SE),
                                CAUSE.first_mut(),
                            );

                            M2MARK(STRING, save.SB, save.SE, CAUSE.first_mut(), ctx);
                            let val = CAUSE.get(1).to_vec();
                            fstr::assign(CAUSE.get_mut(2), &val);
                        }
                    }
                }
            }
        } else {
            spicelib::FNDNWD(STRING, save.SBEGIN, &mut save.SB, &mut save.SE);

            //
            // This "ELSE" is the "NO" response to the question: "Is a
            // range template present?" that was asked a very long, long
            // time ago.
            //
            save.CMATCH = M2WMCH(
                STRING,
                save.SB,
                save.SE,
                fstr::substr(TEMP, save.TB..=save.TE),
                ctx,
            )?;

            //
            // Set the string pointer to the first character following
            // the current string word.
            //
            save.SBEGIN = (save.SE + 1);

            //
            // Record SBEGIN in case we have run out of teplate and
            // haven't produced any errors.
            //
            save.BEGOUT = save.SBEGIN;

            if save.CMATCH {
                save.KEYWRD = M2KEYW(fstr::substr(TEMP, save.TB..=save.TE), ctx);

                if save.KEYWRD {
                    *SCORE = (*SCORE + KEYSC);
                } else {
                    *SCORE = (*SCORE + METASC);
                }
            } else if !save.CMATCH {
                save.KEYWRD = M2KEYW(fstr::substr(TEMP, save.TB..=save.TE), ctx);

                //
                // See if we were supposed to get a keyword and if
                // so see if this is just some simple spelling error.
                //
                if save.KEYWRD {
                    fstr::assign(save.KNOWN.get_mut(1), fstr::substr(TEMP, save.TB..=save.TC));
                    spicelib::SCARDC(1, save.KNOWN.as_arg_mut(), ctx)?;

                    if (save.SE > 0) {
                        BESTWD(
                            fstr::substr(STRING, save.SB..=save.SE),
                            save.KNOWN.as_arg(),
                            CUTOFF,
                            save.BEST.as_slice_mut(),
                            save.SCORES.as_slice_mut(),
                            &mut save.MSSG,
                            ctx,
                        )?;
                    }

                    if ((spicelib::CARDI(save.SCORES.as_slice(), ctx)? > 0)
                        && (save.SCORES[1] >= CUTOFF))
                    {
                        if (*M2CODE == 0) {
                            *M2CODE = 11;

                            //
                            // Save the beginning and ending of the
                            // problem word for use in the recovery
                            // entry point.
                            //
                            save.PBEG = save.SB;
                            save.PEND = save.SE;
                        }

                        //
                        // We regard this to be a spelling error  of the
                        // keyword. This will be a signal to stop looking at
                        // this keyword if we are asking for diagnostics.
                        //

                        if (save.MSPELL > 100) {
                            save.ERROR = true;
                        } else {
                            *SCORE = (*SCORE + save.SCORES[1]);
                            save.MSPELL = (save.MSPELL + (100 - save.SCORES[1]));
                        }

                        if REASON {
                            save.ERROR = true;

                            fstr::assign(
                                CAUSE.get_mut(1),
                                b"I was expecting to see the keyword \"",
                            );

                            spicelib::SUFFIX(
                                fstr::substr(TEMP, save.TB..=save.TC),
                                0,
                                CAUSE.first_mut(),
                            );
                            spicelib::SUFFIX(b"\" when I encountered", 0, CAUSE.first_mut());
                            spicelib::SUFFIX(b"the word \"", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(
                                fstr::substr(STRING, save.SB..=save.SE),
                                0,
                                CAUSE.first_mut(),
                            );
                            spicelib::SUFFIX(b"\" in the input ", 0, CAUSE.first_mut());
                            spicelib::SUFFIX(b"string.     ", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(&save.MSSG, 1, CAUSE.first_mut());
                            let val = CAUSE.get(1).to_vec();
                            fstr::assign(CAUSE.get_mut(2), &val);
                            fstr::assign(CAUSE.get_mut(1), &save.MSSG);
                            M2MARK(STRING, save.SB, save.SE, &mut CAUSE[1], ctx);
                            M2MARK(STRING, save.SB, save.SE, &mut CAUSE[2], ctx);
                        }
                    } else if ((spicelib::CARDI(save.SCORES.as_slice(), ctx)? == 0)
                        || (save.SCORES[1] < CUTOFF))
                    {
                        save.ERROR = true;

                        if (*M2CODE == 0) {
                            *M2CODE = 110;

                            if (save.SE > 0) {
                                BESTWD(
                                    fstr::substr(STRING, save.SB..=save.SE),
                                    save.KNOWN.as_arg(),
                                    1,
                                    save.BEST.as_slice_mut(),
                                    save.SCORES.as_slice_mut(),
                                    &mut save.MSSG,
                                    ctx,
                                )?;
                            }

                            if ((save.SB != 0)
                                && (spicelib::CARDI(save.SCORES.as_slice(), ctx)? > 0))
                            {
                                *SCORE = (*SCORE + save.SCORES[1]);
                            }
                        }

                        if REASON {
                            fstr::assign(
                                CAUSE.get_mut(1),
                                b"I was expecting to see the keyword \"",
                            );

                            spicelib::SUFFIX(
                                fstr::substr(TEMP, save.TB..=save.TC),
                                0,
                                CAUSE.first_mut(),
                            );
                            spicelib::SUFFIX(b"\" when I ", 0, CAUSE.first_mut());

                            if (save.SB == 0) {
                                spicelib::SUFFIX(b"ran out of ", 1, CAUSE.first_mut());
                                spicelib::SUFFIX(b"characters in the", 1, CAUSE.first_mut());
                                spicelib::SUFFIX(b"input string. ", 1, CAUSE.first_mut());

                                save.SB = (QLSTNB(STRING) + 1);
                                save.SE = save.SB;
                            } else {
                                spicelib::SUFFIX(b"encountered", 1, CAUSE.first_mut());
                                spicelib::SUFFIX(b"the word \"", 1, CAUSE.first_mut());
                                spicelib::SUFFIX(
                                    fstr::substr(STRING, save.SB..=save.SE),
                                    0,
                                    CAUSE.first_mut(),
                                );
                                spicelib::SUFFIX(b"\" in the input ", 0, CAUSE.first_mut());
                                spicelib::SUFFIX(b"string.     ", 1, CAUSE.first_mut());
                            }

                            M2MARK(STRING, save.SB, save.SE, CAUSE.first_mut(), ctx);
                            let val = CAUSE.get(1).to_vec();
                            fstr::assign(CAUSE.get_mut(2), &val);
                        }
                    }
                } else if !M2KEYW(fstr::substr(TEMP, save.TB..=save.TE), ctx) {
                    save.ERROR = true;

                    if (*M2CODE == 0) {
                        *M2CODE = 111;
                    }

                    if REASON {
                        fstr::assign(CAUSE.get_mut(1), b"I was expecting to see # # when I ");

                        M2CLSS(
                            fstr::substr(TEMP, save.TB..=save.TC),
                            1,
                            &mut save.PHRASE,
                            ctx,
                        );
                        spicelib::ANA(&save.PHRASE, b"L", &mut save.ARTCLE, ctx);
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.ARTCLE,
                            CAUSE.first_mut(),
                        );
                        spicelib::REPMC(
                            &CAUSE.first().to_vec(),
                            b"#",
                            &save.PHRASE,
                            CAUSE.first_mut(),
                        );

                        if (save.SB == 0) {
                            spicelib::SUFFIX(b"ran out of characters", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(b"in the input string. ", 1, CAUSE.first_mut());

                            save.SB = (QLSTNB(STRING) + 1);
                            save.SE = save.SB;
                        } else {
                            spicelib::SUFFIX(b"encountered the word \"", 1, CAUSE.first_mut());
                            spicelib::SUFFIX(
                                fstr::substr(STRING, save.SB..=save.SE),
                                0,
                                CAUSE.first_mut(),
                            );
                            spicelib::SUFFIX(b"\" in the input string.", 0, CAUSE.first_mut());
                        }

                        M2MARK(STRING, save.SB, save.SE, CAUSE.first_mut(), ctx);
                        let val = CAUSE.get(1).to_vec();
                        fstr::assign(CAUSE.get_mut(2), &val);
                    }
                }
            }
        }

        save.TBEGIN = (intrinsics::MAX0(&[save.KE, save.TE]) + 1);

        //
        // Locate the next word of the template and continue unless
        // we get a second error detected.
        //
        spicelib::FNDNWD(TEMP, save.TBEGIN, &mut save.TB, &mut save.TE);
    }

    //
    // If we got out of the template without an error, set SBEG to
    // BEGOUT---the first character after the matched portion of the
    // STRING and before the first word of whatever is left.
    //
    if (*M2CODE == 0) {
        *SBEG = save.BEGOUT;
    }

    Ok(())
}

//
//$Prodedure M2RCVR ( Recover from a spelling error )
//
pub fn M2RCVR(
    SBEG: &mut i32,
    SEND: &mut i32,
    KEYWDS: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut KEYWDS = DummyCharArrayMut::new(KEYWDS, None, LBCELL..);

    //
    //

    *SBEG = save.PBEG;
    *SEND = save.PEND;

    spicelib::COPYC(save.KNOWN.as_arg(), KEYWDS.as_arg_mut(), ctx)?;

    Ok(())
}
