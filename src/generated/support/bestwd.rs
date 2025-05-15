//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const BSIZE: i32 = 10;

//$Procedure      BESTWD ( Perform a spell match against a set of words )
pub fn BESTWD(
    WORD: &[u8],
    KNOWN: CharArray,
    CUTOFF: i32,
    BEST: &mut [i32],
    SCORES: &mut [i32],
    MSSG: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let KNOWN = DummyCharArray::new(KNOWN, None, LBCELL..);
    let mut BEST = DummyArrayMut::new(BEST, LBCELL..);
    let mut SCORES = DummyArrayMut::new(SCORES, LBCELL..);
    let mut BSCORE = StackArray::<i32, 10>::new(1..=BSIZE);
    let mut ITEM = StackArray::<i32, 10>::new(1..=BSIZE);
    let mut HELP = StackArray::<i32, 10>::new(1..=BSIZE);
    let mut USIZE: i32 = 0;
    let mut NBEST: i32 = 0;
    let mut NTH = [b' '; 80];
    let mut NKNOWN: i32 = 0;
    let mut CSCORE: i32 = 0;
    let mut OSCORE: i32 = 0;
    let mut MAXSC: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut L: i32 = 0;
    let mut LOC: i32 = 0;
    let mut TRANS = [b' '; 16];
    let mut HIT: bool = false;
    let mut HITS: i32 = 0;
    let mut CASE = [b' '; 32];
    let mut MYWD = [b' '; 32 as usize];
    let mut TRIES: i32 = 0;
    let mut LENGTH: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // First determine how many words we have to compare with
    // and the amount of room for reporting indices of "good"
    // matches.
    //
    NKNOWN = spicelib::CARDC(KNOWN.as_arg(), ctx)?;
    NBEST = spicelib::SIZEI(BEST.as_slice(), ctx)?;

    //
    // This routine only works on words of 32 or fewer characters
    //
    fstr::assign(&mut MYWD, b" ");
    fstr::assign(&mut MYWD, WORD);

    //
    // USIZE refers to the amount of space we will actually
    // use in the buffers that store the best MATCHC scores and
    // the associated KNOWN word.
    //
    USIZE = intrinsics::MIN0(&[BSIZE, NKNOWN, NBEST]);

    {
        let m1__: i32 = 1;
        let m2__: i32 = USIZE;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            BSCORE[I] = 0;
            ITEM[I] = 0;
            HELP[I] = 0;
            SCORES[I] = 0;
            I += m3__;
        }
    }

    //
    // First apply MATCHC against each of the KNOWNs and keep the
    // top USIZE words that match.
    //

    {
        let m1__: i32 = 1;
        let m2__: i32 = NKNOWN;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Just in case, see if we have an exact match.
            //
            if spicelib::EQSTR(&MYWD, &KNOWN[I]) {
                spicelib::SCARDI(1, BEST.as_slice_mut(), ctx)?;
                spicelib::SCARDI(1, SCORES.as_slice_mut(), ctx)?;

                BEST[1] = I;
                SCORES[1] = 1000;

                spicelib::INTORD(I, &mut NTH, ctx);
                spicelib::LCASE(&NTH.clone(), &mut NTH, ctx);

                fstr::assign(MSSG, &MYWD);

                spicelib::SUFFIX(b"is equal to the ", 1, MSSG);
                spicelib::SUFFIX(&NTH, 1, MSSG);
                spicelib::SUFFIX(b" known word.", 1, MSSG);

                return Ok(());
            }

            CSCORE = MATCHC(&MYWD, &KNOWN[I], ctx);
            J = spicelib::LSTLTI(CSCORE, USIZE, BSCORE.as_slice());

            for K in 1..=(J - 1) {
                BSCORE[K] = BSCORE[(K + 1)];
                ITEM[K] = ITEM[(K + 1)];
            }

            if (J > 0) {
                BSCORE[J] = CSCORE;
                ITEM[J] = I;
            }

            I += m3__;
        }
    }

    //
    // Now for the top USIZE matches, perform a MATCHO comparison.
    // If we get a match of CUTOFF or higher.  Run MATCHE against it
    // to see if we can guess at what went wrong.
    //
    // So far our best score is 0 and we haven't HIT any good matches.
    //
    MAXSC = 0;
    HITS = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = USIZE;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Only examine items that have legitimate indices.
            //
            if (ITEM[I] != 0) {
                BSCORE[I] = MATCHO(&MYWD, &KNOWN[ITEM[I]], ctx);
                CSCORE = MATCHC(&MYWD, &KNOWN[ITEM[I]], ctx);
                MAXSC = intrinsics::MAX0(&[BSCORE[I], MAXSC]);

                if ((BSCORE[I] >= CUTOFF) && (CSCORE >= CUTOFF)) {
                    //
                    // We've HIT a good match.
                    //
                    HITS = (HITS + 1);

                    //
                    // See if the problem with this word can be diagnosed
                    // with MATCHE.
                    //
                    MATCHE(&MYWD, &KNOWN[ITEM[I]], &mut TRANS, &mut LOC, ctx)?;

                    //
                    // If a diagnosis can be performed on this item, we
                    // say that HELP is available at level 2.  Otherwise
                    // since we have a good match anyway we say HELP is
                    // available at level 1.
                    //

                    if fstr::ne(&TRANS, b"NONE") {
                        HELP[I] = 2;
                    } else {
                        HELP[I] = 1;
                    }
                }
            }

            I += m3__;
        }
    }

    //
    // If none of the words had a sufficiently high score, just
    // report those that had the maximum score.
    //
    if (HITS == 0) {
        //
        // Just report the item(s) that had the biggest score.
        //
        // First see how many had the maximum score.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = USIZE;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (ITEM[I] != 0) {
                    if (BSCORE[I] == MAXSC) {
                        HITS = (HITS + 1);
                    }
                }
                I += m3__;
            }
        }

        //
        // If there were no KNOWN words that had letters in common
        // with MYWD, all of the elements of the array ITEM will be
        // zero and we will not have made any HITS against MAXSC.
        // There is nothing at all we can do in this case.
        //
        if (HITS == 0) {
            fstr::assign(MSSG, b"The word");

            spicelib::SUFFIX(&MYWD, 1, MSSG);
            spicelib::SUFFIX(b"has nothing in common with any of", 1, MSSG);
            spicelib::SUFFIX(b"the words I can recognize.  If ", 1, MSSG);
            spicelib::SUFFIX(b"this word was typed interactively,", 1, MSSG);
            spicelib::SUFFIX(b"you may want to see if your ", 1, MSSG);
            spicelib::SUFFIX(b"fingers are over the correct keys.", 1, MSSG);

            spicelib::SCARDI(0, BEST.as_slice_mut(), ctx)?;
            spicelib::SCARDI(0, SCORES.as_slice_mut(), ctx)?;

            return Ok(());
        }

        //
        // Still here.  Then we have at least some item that has
        // something in common with MYWD.  Set up a closing string so
        // that grammar will be correct.
        //
        if (HITS > 1) {
            fstr::assign(&mut CASE, b"my closest matches are: ");
        } else {
            fstr::assign(&mut CASE, b"my closest match is: ");
        }

        fstr::assign(MSSG, b"The word \'");

        spicelib::SUFFIX(&MYWD, 1, MSSG);
        spicelib::SUFFIX(b"\' did not match up well with any of", 1, MSSG);
        spicelib::SUFFIX(b"the words I was comparing against.", 1, MSSG);
        spicelib::SUFFIX(b"However,", 1, MSSG);
        spicelib::SUFFIX(&CASE, 1, MSSG);

        //
        // Now append the list of KNOWN words that matched MYWD with
        // the highest score.
        //
        HIT = false;
        J = 0;

        {
            let m1__: i32 = 1;
            let m2__: i32 = USIZE;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (ITEM[I] == 0) {

                    //
                    // don't do anything
                    //
                } else if (BSCORE[I] == MAXSC) {
                    J = (J + 1);

                    BEST[J] = ITEM[I];
                    L = QRTRIM(&KNOWN[ITEM[I]]);

                    if HIT {
                        spicelib::SUFFIX(b",  \'", 0, MSSG);
                        spicelib::SUFFIX(fstr::substr(&KNOWN[ITEM[I]], 1..=L), 0, MSSG);
                        spicelib::SUFFIX(b"\'", 0, MSSG);
                    } else {
                        HIT = true;

                        spicelib::SUFFIX(b"\'", 1, MSSG);
                        spicelib::SUFFIX(fstr::substr(&KNOWN[ITEM[I]], 1..=L), 0, MSSG);
                        spicelib::SUFFIX(b"\'", 0, MSSG);
                    }

                    spicelib::SUFFIX(b".", 0, MSSG);
                }

                I += m3__;
            }
        }

        //
        // Set the cardinality of the window of BEST indices.
        //
        spicelib::SCARDI(J, BEST.as_slice_mut(), ctx)?;
    } else if (HITS == 1) {
        //
        // There was just one KNOWN word for which there was a good
        // match.  Call MSPELD to produce a diagnosis of the problem
        // and record the index of the item.
        //
        I = 1;

        while (HELP[I] == 0) {
            I = (I + 1);
        }

        MSPELD(&MYWD, &KNOWN[ITEM[I]], MSSG, ctx)?;

        BEST[1] = ITEM[I];
        spicelib::SCARDI(1, BEST.as_slice_mut(), ctx)?;
    } else {
        //
        // There were at least two "good" words.  If any of them
        // could be diagnosed, then report them.  Otherwise
        // report only those that had a maximum MATCHO score.
        //
        TRIES = 0;

        {
            let m1__: i32 = 1;
            let m2__: i32 = 5;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (HELP[I] == 2) {
                    TRIES = (TRIES + 1);
                }

                I += m3__;
            }
        }

        if (TRIES == 0) {
            //
            // None of the KNOWN words had diagnostics available.
            //
            fstr::assign(MSSG, b"Although a the spelling error can\'t be described in a simple way,  I have found the following words that may be what you were trying to say.  ");
            J = 0;

            {
                let m1__: i32 = 1;
                let m2__: i32 = USIZE;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (HELP[I] != 0) {
                        spicelib::SUFFIX(b"\'", 2, MSSG);
                        spicelib::SUFFIX(&KNOWN[ITEM[I]], 0, MSSG);
                        spicelib::SUFFIX(b"\',", 0, MSSG);

                        J = (J + 1);
                        BEST[J] = ITEM[I];
                    }

                    I += m3__;
                }
            }

            spicelib::SCARDI(J, BEST.as_slice_mut(), ctx)?;

            fstr::assign(fstr::substr_mut(MSSG, QRTRIM(MSSG)..=QRTRIM(MSSG)), b" ");
        } else if (TRIES == 1) {
            //
            // Only one of the KNOWN words had diagnostics available.
            //
            {
                let m1__: i32 = 1;
                let m2__: i32 = 5;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (HELP[I] == 2) {
                        MSPELD(&MYWD, &KNOWN[ITEM[I]], MSSG, ctx)?;
                        BEST[1] = ITEM[I];
                    }
                    I += m3__;
                }
            }

            spicelib::SCARDI(1, BEST.as_slice_mut(), ctx)?;
        } else {
            //
            // At least two of the KNOWN words had diagnostics available.
            // Report all of them.
            //
            fstr::assign(
                MSSG,
                b"The following common spelling mistakes may be the reason I did not recognize ",
            );

            spicelib::SUFFIX(&MYWD, 1, MSSG);
            spicelib::SUFFIX(b".", 1, MSSG);

            LENGTH = intrinsics::LEN(MSSG);
            J = 0;

            {
                let m1__: i32 = 1;
                let m2__: i32 = USIZE;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (HELP[I] == 2) {
                        if (QRTRIM(MSSG) < (LENGTH - 3)) {
                            MSPELD(
                                &MYWD,
                                &KNOWN[ITEM[I]],
                                fstr::substr_mut(MSSG, (QRTRIM(MSSG) + 3)..),
                                ctx,
                            )?;

                            J = (J + 1);
                            BEST[J] = ITEM[I];
                        }
                    }

                    I += m3__;
                }
            }

            spicelib::SCARDI(J, BEST.as_slice_mut(), ctx)?;
        }
    }

    //
    // As for the scores, we will report the average of the MATCHO and
    // MATCHC scores for the best matches.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDI(BEST.as_slice(), ctx)?;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            OSCORE = MATCHO(&MYWD, &KNOWN[BEST[I]], ctx);
            CSCORE = MATCHC(&MYWD, &KNOWN[BEST[I]], ctx);

            SCORES[I] = ((OSCORE + CSCORE) / 2);

            if ((OSCORE < CUTOFF) || (CSCORE < CUTOFF)) {
                SCORES[I] = intrinsics::MIN0(&[SCORES[I], (CUTOFF - 1)]);
            }

            I += m3__;
        }
    }

    spicelib::SCARDI(
        spicelib::CARDI(BEST.as_slice(), ctx)?,
        SCORES.as_slice_mut(),
        ctx,
    )?;

    Ok(())
}
