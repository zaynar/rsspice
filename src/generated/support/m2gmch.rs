//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

struct SaveVars {
    PASS1: bool,
    TERMS: ActualCharArray,
    INDXES: StackArray<i32, 70>,
    TLEN: i32,
    SIZE: i32,
    I: i32,
    J: i32,
    K: i32,
    TMPJ: i32,
    TBEG: i32,
    TEND: i32,
    MORE: bool,
    GROUP: bool,
    SIMPLE: bool,
    LAST: Vec<u8>,
    POSITN: i32,
    LABEL: Vec<u8>,
    EO: i32,
    BO: i32,
    OPTDIR: bool,
    BW: i32,
    EW: i32,
    FEWEST: i32,
    MOST: i32,
    GMATCH: i32,
    BSCORE: i32,
    BEGOFG: i32,
    ENDOFG: i32,
    AFTERG: i32,
    BS: i32,
    ES: i32,
    A: i32,
    B: i32,
    E: i32,
    BLSTWD: i32,
    CLSTWD: i32,
    ELSTWD: i32,
    KEYWDS: ActualCharArray,
    SWORDS: ActualCharArray,
    T1CODE: i32,
    TSCORE: i32,
    LOC: i32,
    BDIAGS: i32,
    EDIAGS: i32,
    BCODE: i32,
    SUBTMP: Vec<u8>,
    REDIAG: bool,
    VTEMPL: bool,
    CARD: i32,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PASS1: bool = false;
        let mut TERMS = ActualCharArray::new(32, LBCELL..=64);
        let mut INDXES = StackArray::<i32, 70>::new(LBCELL..=64);
        let mut TLEN: i32 = 0;
        let mut SIZE: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut TMPJ: i32 = 0;
        let mut TBEG: i32 = 0;
        let mut TEND: i32 = 0;
        let mut MORE: bool = false;
        let mut GROUP: bool = false;
        let mut SIMPLE: bool = false;
        let mut LAST = vec![b' '; 8];
        let mut POSITN: i32 = 0;
        let mut LABEL = vec![b' '; 32];
        let mut EO: i32 = 0;
        let mut BO: i32 = 0;
        let mut OPTDIR: bool = false;
        let mut BW: i32 = 0;
        let mut EW: i32 = 0;
        let mut FEWEST: i32 = 0;
        let mut MOST: i32 = 0;
        let mut GMATCH: i32 = 0;
        let mut BSCORE: i32 = 0;
        let mut BEGOFG: i32 = 0;
        let mut ENDOFG: i32 = 0;
        let mut AFTERG: i32 = 0;
        let mut BS: i32 = 0;
        let mut ES: i32 = 0;
        let mut A: i32 = 0;
        let mut B: i32 = 0;
        let mut E: i32 = 0;
        let mut BLSTWD: i32 = 0;
        let mut CLSTWD: i32 = 0;
        let mut ELSTWD: i32 = 0;
        let mut KEYWDS = ActualCharArray::new(32, LBCELL..=64);
        let mut SWORDS = ActualCharArray::new(32, LBCELL..=64);
        let mut T1CODE: i32 = 0;
        let mut TSCORE: i32 = 0;
        let mut LOC: i32 = 0;
        let mut BDIAGS: i32 = 0;
        let mut EDIAGS: i32 = 0;
        let mut BCODE: i32 = 0;
        let mut SUBTMP = vec![b' '; 1024];
        let mut REDIAG: bool = false;
        let mut VTEMPL: bool = false;
        let mut CARD: i32 = 0;

        PASS1 = true;

        Self {
            PASS1,
            TERMS,
            INDXES,
            TLEN,
            SIZE,
            I,
            J,
            K,
            TMPJ,
            TBEG,
            TEND,
            MORE,
            GROUP,
            SIMPLE,
            LAST,
            POSITN,
            LABEL,
            EO,
            BO,
            OPTDIR,
            BW,
            EW,
            FEWEST,
            MOST,
            GMATCH,
            BSCORE,
            BEGOFG,
            ENDOFG,
            AFTERG,
            BS,
            ES,
            A,
            B,
            E,
            BLSTWD,
            CLSTWD,
            ELSTWD,
            KEYWDS,
            SWORDS,
            T1CODE,
            TSCORE,
            LOC,
            BDIAGS,
            EDIAGS,
            BCODE,
            SUBTMP,
            REDIAG,
            VTEMPL,
            CARD,
        }
    }
}

//$Procedure      M2GMCH ( Match a META/2 template including groups )
pub fn M2GMCH(
    TEMP: &mut [u8],
    THNWDS: CharArray,
    STRING: &[u8],
    SBEG: &mut i32,
    REASON: bool,
    CUTOFF: i32,
    PSSTHN: &mut bool,
    M2CODE: &mut i32,
    SCORE: &mut i32,
    CAUSE: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let THNWDS = DummyCharArray::new(THNWDS, None, LBCELL..);
    let mut CAUSE = DummyCharArrayMut::new(CAUSE, None, 1..=2);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // If this is the first pass through this routine, set the size of
    // the cells TERMS and INDXES
    //
    if save.PASS1 {
        save.PASS1 = false;

        spicelib::SSIZEC(64, save.TERMS.as_arg_mut(), ctx)?;
        spicelib::SSIZEI(64, save.INDXES.as_slice_mut(), ctx)?;
        spicelib::SSIZEC(64, save.KEYWDS.as_arg_mut(), ctx)?;
        spicelib::SSIZEC(64, save.SWORDS.as_arg_mut(), ctx)?;
    }

    //
    // Clear out the parse table.
    //
    M2PCLR(ctx)?;

    //
    // Collect the list of potential terminating keywords.
    //
    M2TERM(
        TEMP,
        save.TERMS.as_arg_mut(),
        save.INDXES.as_slice_mut(),
        ctx,
    )?;

    //
    // Append all of the '@then(*)'-keywords to the list of terminators.
    //
    if ((spicelib::CARDC(save.TERMS.as_arg(), ctx)? + spicelib::CARDC(THNWDS.as_arg(), ctx)?)
        >= (spicelib::SIZEC(save.TERMS.as_arg(), ctx)? - 2))
    {
        spicelib::CHKIN(b"M2GMCH", ctx)?;
        spicelib::SIGERR(b"SPICE(TOOMANYKEYWORDS)", ctx)?;
        spicelib::CHKOUT(b"M2GMCH", ctx)?;
        return Ok(());
    }

    save.TLEN = (intrinsics::LEN(TEMP) + 1);
    save.J = (spicelib::CARDC(save.TERMS.as_arg(), ctx)? + 1);

    {
        let m1__: i32 = 1;
        let m2__: i32 = spicelib::CARDC(THNWDS.as_arg(), ctx)?;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            fstr::assign(save.TERMS.get_mut(save.J), THNWDS.get(save.I));
            save.INDXES[save.J] = save.TLEN;
            save.J = (save.J + 1);
            save.I += m3__;
        }
    }

    //
    // Append a '@end' and a '}' to the end of the terminators, and
    // adjust the cardinality of the TERMS cell
    //
    fstr::assign(save.TERMS.get_mut(save.J), b"@end");
    save.INDXES[save.J] = save.TLEN;
    save.J = (save.J + 1);

    fstr::assign(save.TERMS.get_mut(save.J), b"}");
    save.INDXES[save.J] = save.TLEN;
    save.SIZE =
        ((spicelib::CARDC(save.TERMS.as_arg(), ctx)? + spicelib::CARDC(THNWDS.as_arg(), ctx)?) + 2);

    spicelib::SCARDC(save.SIZE, save.TERMS.as_arg_mut(), ctx)?;
    spicelib::SCARDI(save.SIZE, save.INDXES.as_slice_mut(), ctx)?;

    //
    // This routine will only use the portion of the template up
    // to a qualified @then.
    //
    M2THNQ(TEMP, &mut save.POSITN, &mut save.LABEL, ctx);

    if (save.POSITN <= intrinsics::LEN(TEMP)) {
        fstr::assign(fstr::substr_mut(TEMP, save.POSITN..), b" ");
    }

    //
    // Now initialize pointers and the loop control variable MORE so
    // that we can start the loop.
    //
    save.TBEG = 1;
    save.TEND = 1;

    save.MORE = true;

    while save.MORE {
        //
        // As long as we are not told to exit
        //
        //    Look at the next word,
        //
        spicelib::FNDNWD(TEMP, save.TBEG, &mut save.BW, &mut save.EW);

        if (save.BW == 0) {
            //
            // There wasn't a next word.  There is nothing left to do.
            // We set MORE to .FALSE. so that we can exit the loop.
            //
            save.MORE = false;
            save.GROUP = false;
            save.SIMPLE = false;
        } else if fstr::eq(fstr::substr(TEMP, save.BW..=save.EW), b"@then") {
            //
            // We have an unqualified @then directive.  This means that
            // we are on the right track as far as determining what
            // command we are working on.  Set the PASSED-A-@then flag
            // (PSSTHN) to .TRUE. and the other candidates to .FALSE.
            //
            *PSSTHN = true;
            save.GROUP = false;
            save.SIMPLE = false;

            save.TBEG = (save.EW + 1);
            save.TEND = save.TBEG;
        } else if MATCH(fstr::substr(TEMP, save.BW..=save.EW), b"(%*:%*){", ctx)? {
            //
            // We are about to enter a group template.  Determine
            // the FEWEST number of simple templates in the group
            // that must match and the MOST that we will check.
            //
            M2BEGR(
                TEMP,
                &mut save.BW,
                save.EW,
                &mut save.FEWEST,
                &mut save.MOST,
                ctx,
            );

            save.GROUP = true;
            save.SIMPLE = false;
            fstr::assign(&mut save.LAST, b"GROUP");

            //
            // Set up the pointers for looking for the simple
            // templates within this group.
            //
            save.TBEG = (save.EW + 1);
            save.TEND = save.TBEG;
        } else {
            //
            // The only possible candidate is a simple template.
            //
            save.GROUP = false;
            save.SIMPLE = true;
            save.TBEG = save.BW;
            fstr::assign(&mut save.LAST, b"SIMPLE");
        }

        if save.GROUP {
            //
            // Set up the initial values for this group.  We need
            //
            // 1)  The number of simple template matches so far for
            //     this group.
            //
            save.GMATCH = 0;

            //
            // 2)  A best score of the simple templates checked so far.
            //
            save.BSCORE = -1;

            //
            // 3)  A temporary place to store the M2CODE returned for
            //     a simple template of this group.
            //
            save.T1CODE = 0;

            //
            // 4)  The position in the full template to jump to when we are
            //     done with this template, the beginning and the end of
            //     the group
            //
            save.BEGOFG = save.TBEG;
            save.ENDOFG = UPTO(TEMP, b" }", save.TBEG);
            save.AFTERG = (save.ENDOFG + 3);

            //
            // Make sure there is a viable simple template within this
            // group.
            //
            save.BS = spicelib::NCPOS(TEMP, b" ", save.BEGOFG);
            save.ES = intrinsics::MIN0(&[
                UPTO(TEMP, b" | ", save.BEGOFG),
                UPTO(TEMP, b" } ", save.BEGOFG),
            ]);

            if ((((save.BS == 0) || (save.BS >= save.ES))
                || fstr::eq(fstr::substr(TEMP, save.BS..=save.BS), b"}"))
                || (save.ES > save.ENDOFG))
            {
                save.GROUP = false;
            }

            //
            // Finally, if FEWEST is 1 or 0,
            // remove the '}' that has index equal to the index
            // of the '}' that is the terminator of this group.
            //
            if (save.FEWEST <= 1) {
                save.LOC = spicelib::BSRCHI(
                    (save.ENDOFG + 2),
                    spicelib::CARDI(save.INDXES.as_slice(), ctx)?,
                    save.INDXES.subarray(1),
                );

                if (save.LOC != 0) {
                    save.CARD = spicelib::CARDI(save.INDXES.as_slice(), ctx)?;
                    spicelib::REMLAI(
                        1,
                        save.LOC,
                        save.INDXES.subarray_mut(1),
                        &mut save.CARD,
                        ctx,
                    )?;
                    save.CARD = spicelib::CARDC(save.TERMS.as_arg(), ctx)?;
                    spicelib::REMLAC(1, save.LOC, save.TERMS.subarray_mut(1), &mut save.CARD, ctx)?;

                    spicelib::SCARDC(
                        (spicelib::CARDC(save.TERMS.as_arg(), ctx)? - 1),
                        save.TERMS.as_arg_mut(),
                        ctx,
                    )?;
                    spicelib::SCARDI(
                        (spicelib::CARDI(save.INDXES.as_slice(), ctx)? - 1),
                        save.INDXES.as_slice_mut(),
                        ctx,
                    )?;
                }
            }

            while save.GROUP {
                //
                // We've got a viable simple template for this group.
                //
                // If it ends with a variable template find out what the
                // possible terminating words are.
                //
                save.A = 0;
                save.B = 0;

                FNDPTK(
                    TEMP,
                    b" ",
                    (save.ES + 1),
                    &mut save.BLSTWD,
                    &mut save.ELSTWD,
                    ctx,
                )?;
                M2BEGR(
                    TEMP,
                    &mut save.BLSTWD,
                    save.ELSTWD,
                    &mut save.A,
                    &mut save.B,
                    ctx,
                );

                save.CLSTWD = intrinsics::MIN0(&[save.ELSTWD, (save.BLSTWD + 8)]);
                save.VTEMPL = ((save.A != save.B)
                    || (!M2KEYW(fstr::substr(TEMP, save.BLSTWD..=save.ELSTWD), ctx)
                        && fstr::eq(b"@calendar", fstr::substr(TEMP, save.BLSTWD..=save.CLSTWD))));

                if save.VTEMPL {
                    //
                    // There is a variable length template, the keywords
                    // that might terminate this template are given
                    // in TERMS up to the first occurance of a '}'.
                    //
                    if (save.GMATCH < (save.MOST - 1)) {
                        save.I = (spicelib::LSTLTI(
                            save.BEGOFG,
                            spicelib::CARDI(save.INDXES.as_slice(), ctx)?,
                            save.INDXES.subarray(1),
                        ) + 1);
                    } else {
                        save.I = (spicelib::LSTLTI(
                            save.AFTERG,
                            spicelib::CARDI(save.INDXES.as_slice(), ctx)?,
                            save.INDXES.subarray(1),
                        ) + 1);
                    }

                    save.J = 0;

                    while fstr::ne(save.TERMS.get(save.I), b"}") {
                        //
                        // Keep only those keywords that are not the initial
                        // keyword of this template.
                        //

                        if (save.INDXES[save.I] != save.BS) {
                            save.J = (save.J + 1);
                            M2TRIM(&save.TERMS[save.I], &mut save.KEYWDS[save.J], ctx);
                        }

                        save.I = (save.I + 1);
                    }

                    spicelib::SCARDC(save.J, save.KEYWDS.as_arg_mut(), ctx)?;
                } else {
                    spicelib::SCARDC(0, save.KEYWDS.as_arg_mut(), ctx)?;
                }

                //
                // Check the current template with M2MTCH.
                //

                if fstr::eq(fstr::substr(TEMP, save.BS..=save.ES), b"@options") {
                    save.T1CODE = -1;
                    save.TSCORE = -1;
                } else {
                    //
                    // Dump the temporary parse table.
                    //
                    M2TCLR(ctx)?;
                    M2MTCH(
                        fstr::substr(TEMP, save.BS..=save.ES),
                        1,
                        save.KEYWDS.as_arg(),
                        STRING,
                        SBEG,
                        false,
                        CUTOFF,
                        &mut save.T1CODE,
                        &mut save.TSCORE,
                        CAUSE.as_arg_mut(),
                        ctx,
                    )?;
                }
                //
                // If the attempt at a match succeeded ...
                //
                if (save.T1CODE == 0) {
                    //
                    // Increment the number of group matches by 1.
                    // Increment the score for this template.
                    // Set the best score obtained thus far to zero
                    // in preparation for the next pass through the
                    // group.
                    //
                    save.GMATCH = (save.GMATCH + 1);
                    *SCORE = (*SCORE + save.TSCORE);
                    save.BSCORE = -1;

                    //
                    // Move the temporary parse table to the keepers
                    // parse table.
                    //
                    M2KEEP(ctx)?;
                    //
                    // The current template should be taken off the viable
                    // list.
                    //
                    if (save.ES < save.ENDOFG) {
                        fstr::assign(fstr::substr_mut(TEMP, save.BS..=(save.ES + 2)), b" ");
                    } else {
                        FNDPTK(TEMP, b" ", save.BS, &mut save.A, &mut save.B, ctx)?;

                        if fstr::eq(fstr::substr(TEMP, save.A..=save.B), b"|") {
                            fstr::assign(fstr::substr_mut(TEMP, save.A..=save.ES), b" ");
                        } else {
                            fstr::assign(fstr::substr_mut(TEMP, save.BS..=save.ES), b" ");
                        }
                    }

                    //
                    // Reset ES to be the one before the beginning of
                    // the group template (BS will be set to ES + 1
                    // at the end of the group loop).
                    //
                    save.ES = (save.BEGOFG - 1);

                    //
                    // Adjust the possible terminating keyword set.
                    // (remove the initial keyword of the simple template
                    // just matched from the collection).
                    //
                    save.LOC = spicelib::BSRCHI(
                        save.BS,
                        spicelib::CARDI(save.INDXES.as_slice(), ctx)?,
                        save.INDXES.subarray(1),
                    );

                    save.CARD = spicelib::CARDI(save.INDXES.as_slice(), ctx)?;
                    spicelib::REMLAI(
                        1,
                        save.LOC,
                        save.INDXES.subarray_mut(1),
                        &mut save.CARD,
                        ctx,
                    )?;
                    save.CARD = spicelib::CARDC(save.TERMS.as_arg(), ctx)?;
                    spicelib::REMLAC(1, save.LOC, save.TERMS.subarray_mut(1), &mut save.CARD, ctx)?;

                    spicelib::SCARDC(
                        (spicelib::CARDC(save.TERMS.as_arg(), ctx)? - 1),
                        save.TERMS.as_arg_mut(),
                        ctx,
                    )?;
                    spicelib::SCARDI(
                        (spicelib::CARDI(save.INDXES.as_slice(), ctx)? - 1),
                        save.INDXES.as_slice_mut(),
                        ctx,
                    )?;

                    //
                    // Finally, if we have now exactly matched FEWEST-1,
                    // remove the '}' that has index equal to the index
                    // of the '}' that is the terminator of this group.
                    //
                    if (save.GMATCH == (save.FEWEST - 1)) {
                        save.LOC = spicelib::BSRCHI(
                            (save.ENDOFG + 2),
                            spicelib::CARDI(save.INDXES.as_slice(), ctx)?,
                            save.INDXES.subarray(1),
                        );

                        if (save.LOC != 0) {
                            save.CARD = spicelib::CARDI(save.INDXES.as_slice(), ctx)?;
                            spicelib::REMLAI(
                                1,
                                save.LOC,
                                save.INDXES.subarray_mut(1),
                                &mut save.CARD,
                                ctx,
                            )?;
                            save.CARD = spicelib::CARDC(save.TERMS.as_arg(), ctx)?;
                            spicelib::REMLAC(
                                1,
                                save.LOC,
                                save.TERMS.subarray_mut(1),
                                &mut save.CARD,
                                ctx,
                            )?;

                            spicelib::SCARDC(
                                (spicelib::CARDC(save.TERMS.as_arg(), ctx)? - 1),
                                save.TERMS.as_arg_mut(),
                                ctx,
                            )?;
                            spicelib::SCARDI(
                                (spicelib::CARDI(save.INDXES.as_slice(), ctx)? - 1),
                                save.INDXES.as_slice_mut(),
                                ctx,
                            )?;
                        }
                    }
                } else {
                    //
                    // Record the score if this is higher than a previous
                    // value.
                    //
                    if (save.TSCORE > save.BSCORE) {
                        save.BSCORE = save.TSCORE;
                        save.BDIAGS = save.BS;
                        save.EDIAGS = save.ES;
                        save.BCODE = save.T1CODE;

                        spicelib::COPYC(save.KEYWDS.as_arg(), save.SWORDS.as_arg_mut(), ctx)?;
                    }
                }

                //
                // Remove all introductory '@options' directives.
                //
                save.OPTDIR = true;

                while save.OPTDIR {
                    save.BO = spicelib::NCPOS(TEMP, b" ", save.BEGOFG);
                    save.EO = intrinsics::MIN0(&[
                        UPTO(TEMP, b" | ", save.BEGOFG),
                        UPTO(TEMP, b" } ", save.BEGOFG),
                    ]);

                    if (save.BO < save.EO) {
                        save.OPTDIR = fstr::eq(fstr::substr(TEMP, save.BO..=save.EO), b"@options");

                        if save.OPTDIR {
                            fstr::assign(fstr::substr_mut(TEMP, save.BO..=save.EO), b" ");
                            save.EO = (save.EO + 2);

                            if fstr::eq(fstr::substr(TEMP, save.EO..=save.EO), b"|") {
                                fstr::assign(fstr::substr_mut(TEMP, save.EO..=save.EO), b" ");
                            }
                        }
                    } else {
                        save.OPTDIR = false;
                    }
                }
                //
                // Should we stay in this group? Only if you can answer yes
                // to  all of the following:
                //
                //     1.) Are more matches allowed for this group.
                //
                //     2.) Is there another template in this group that
                //         hasn't been checked.
                //
                if (save.GMATCH >= save.MOST) {
                    save.GROUP = false;
                } else {
                    //
                    // Make sure there is a viable simple template within
                    // this group.
                    //

                    save.BS = spicelib::NCPOS(TEMP, b" |", (save.ES + 1));
                    save.ES = intrinsics::MIN0(&[
                        UPTO(TEMP, b" | ", save.BS),
                        UPTO(TEMP, b" } ", save.BS),
                    ]);

                    if ((((save.BS == 0) || (save.BS >= save.ES))
                        || fstr::eq(fstr::substr(TEMP, save.BS..=save.BS), b"}"))
                        || (save.ES > save.ENDOFG))
                    {
                        save.GROUP = false;
                    }
                }
            }

            //
            // When we leave the group, see if we had a sufficient number
            // of matches.  If we did, jump past the end of the group.
            // If we didn't, this is an error---head for home.
            //
            save.OPTDIR =
                (intrinsics::INDEX(fstr::substr(TEMP, save.BEGOFG..=save.ENDOFG), b" @options ")
                    != 0);

            if (!save.OPTDIR && (save.GMATCH >= save.FEWEST)) {
                save.TBEG = save.AFTERG;
            } else if (save.OPTDIR && (save.GMATCH >= save.MOST)) {
                if REASON {
                    spicelib::CMPRSS(
                        b" ",
                        1,
                        &fstr::substr(TEMP, save.BEGOFG..=save.ENDOFG).to_vec(),
                        fstr::substr_mut(TEMP, save.BEGOFG..=save.ENDOFG),
                    );

                    save.B = (save.BEGOFG - 1);
                    save.E = (intrinsics::INDEX(
                        fstr::substr(TEMP, save.BEGOFG..=save.ENDOFG),
                        b" @options ",
                    ) + 1);

                    fstr::assign(CAUSE.get_mut(1), b"I had already matched the maximum number of allowed simple templates in a group without matching the  following REQUIRED templates./cr/cr(3:3)");
                    spicelib::SUFFIX(fstr::substr(TEMP, save.B..=save.E), 1, &mut CAUSE[1]);
                    spicelib::SUFFIX(b"} /cr/cr(-3:-3)", 1, &mut CAUSE[1]);

                    *M2CODE = 11000;
                    save.MORE = false;
                }
            } else if (save.OPTDIR && (save.GMATCH >= save.FEWEST)) {
                *SCORE = (*SCORE + save.BSCORE);

                //
                // If diagnostics are requested then see what went wrong
                // with the best fitting simple template.
                //
                if REASON {
                    save.BS = save.BDIAGS;
                    save.ES = save.EDIAGS;

                    M2MTCH(
                        fstr::substr(TEMP, save.BS..=save.ES),
                        1,
                        save.SWORDS.as_arg(),
                        STRING,
                        SBEG,
                        REASON,
                        CUTOFF,
                        &mut save.T1CODE,
                        &mut save.TSCORE,
                        CAUSE.as_arg_mut(),
                        ctx,
                    )?;

                    let val = CAUSE.get(1).to_vec();
                    fstr::assign(CAUSE.get_mut(2), &val);

                    save.B = (save.BEGOFG - 1);
                    save.E = (save.ENDOFG + 2);

                    spicelib::CMPRSS(
                        b" ",
                        1,
                        fstr::substr(TEMP, save.B..=save.E),
                        &mut save.SUBTMP,
                    );

                    if (intrinsics::INDEX(&save.SUBTMP, b" | ")
                        == (intrinsics::INDEX(&save.SUBTMP, b" @options ") - 2))
                    {
                        spicelib::PREFIX(b"/cr/cr(-3:)", 1, &mut CAUSE[2]);
                        spicelib::PREFIX(&save.SUBTMP, 1, &mut CAUSE[2]);
                        spicelib::PREFIX(b"Although I had matched a required number of expressions in the group below, I had not yet matched the explicitely required expression that appears prior to the META/2 \"@options\" directive in the group shown here./cr(3:) ", 1, &mut CAUSE[2]);

                        save.K = spicelib::POS(
                            &save.SUBTMP,
                            b" | ",
                            intrinsics::INDEX(&save.SUBTMP, b" @options "),
                        );

                        if ((save.K > 0) && (spicelib::NCPOS(STRING, b" ", *SBEG) != 0)) {
                            spicelib::SUFFIX(b"/cr/cr Of the remaining simple templates (including the optional ones) the one that comes closest to matching is: /cr/cr(3:) ", 1, &mut CAUSE[2]);
                            spicelib::SUFFIX(
                                fstr::substr(TEMP, save.BDIAGS..=save.EDIAGS),
                                1,
                                &mut CAUSE[2],
                            );
                            spicelib::SUFFIX(b"/cr/cr(-3:)", 0, &mut CAUSE[2]);
                        }
                    } else {
                        spicelib::PREFIX(b"/cr/cr(-3:)", 1, &mut CAUSE[2]);
                        spicelib::PREFIX(&save.SUBTMP, 0, &mut CAUSE[2]);
                        spicelib::PREFIX(b"Although I had matched a required number of expressions in the group below, I had not yet matched the explicitely required expressions that appear prior to the META/2 \"@options\" directive in the group shown here. ./cr/cr(3:) ", 1, &mut CAUSE[2]);

                        if (spicelib::NCPOS(STRING, b" ", *SBEG) != 0) {
                            spicelib::SUFFIX(b"/cr/crOf the remaining simple templates, the one that comes closest to matching is: /cr/cr(3:) ", 1, &mut CAUSE[2]);
                            spicelib::SUFFIX(
                                fstr::substr(TEMP, save.BDIAGS..=save.EDIAGS),
                                1,
                                &mut CAUSE[2],
                            );
                            spicelib::SUFFIX(b"/cr/cr(-3:)", 0, &mut CAUSE[2]);
                        }
                    }
                }

                *M2CODE = save.BCODE;
                save.MORE = false;
            } else if (save.GMATCH < save.FEWEST) {
                *SCORE = (*SCORE + save.BSCORE);

                //
                // If diagnostics are requested then see what went wrong
                // with the best fitting simple template.
                //
                if REASON {
                    save.BS = save.BDIAGS;
                    save.ES = save.EDIAGS;

                    M2MTCH(
                        fstr::substr(TEMP, save.BS..=save.ES),
                        1,
                        save.SWORDS.as_arg(),
                        STRING,
                        SBEG,
                        REASON,
                        CUTOFF,
                        &mut save.T1CODE,
                        &mut save.TSCORE,
                        CAUSE.as_arg_mut(),
                        ctx,
                    )?;

                    let val = CAUSE.get(1).to_vec();
                    fstr::assign(CAUSE.get_mut(2), &val);

                    save.B = (save.BEGOFG - 1);
                    save.E = (save.ENDOFG + 2);

                    spicelib::CMPRSS(
                        b" ",
                        1,
                        fstr::substr(TEMP, save.B..=save.E),
                        &mut save.SUBTMP,
                    );

                    if (intrinsics::INDEX(&save.SUBTMP, b" | ") != 0) {
                        spicelib::PREFIX(b"\'./cr/cr(-3:)", 1, &mut CAUSE[2]);
                        spicelib::PREFIX(
                            fstr::substr(TEMP, save.BDIAGS..=save.EDIAGS),
                            0,
                            &mut CAUSE[2],
                        );
                        spicelib::PREFIX(&fstr::concat(&fstr::concat(&fstr::concat(&fstr::concat(b"I was trying to match part of the input string with one of the expressions listed here:/cr/cr(3:) ", fstr::substr(&save.SUBTMP, 1 ..= spicelib::RTRIM(&save.SUBTMP))), b"./cr/cr(-3:) The expression "), b"that came "), b"closest was: /cr/cr(3:)\'"), 0, &mut CAUSE[2]);
                    } else {
                        spicelib::PREFIX(b"\'./cr/cr(-3:)", 1, &mut CAUSE[2]);
                        spicelib::PREFIX(
                            fstr::substr(TEMP, save.BDIAGS..=save.EDIAGS),
                            0,
                            &mut CAUSE[2],
                        );
                        spicelib::PREFIX(b"I was trying to match part of the input string with the expression: /cr/cr(3:) \'", 0, &mut CAUSE[2]);
                    }
                }

                *M2CODE = save.BCODE;
                save.MORE = false;
            }
        //
        } else if save.SIMPLE {
            save.TEND = (intrinsics::MIN0(&[
                UPTO(TEMP, b" @then", save.TBEG),
                UPTO(TEMP, b"){ ", save.TBEG),
            ]) + 1);

            FNDPTK(
                TEMP,
                b" ",
                save.TEND,
                &mut save.BLSTWD,
                &mut save.ELSTWD,
                ctx,
            )?;

            save.TEND = save.ELSTWD;
            //
            // See if the simple template ends with a variable template.
            // If it does, find out what the possible terminating words
            // are.
            //
            save.A = 0;
            save.B = 0;

            M2BEGR(
                TEMP,
                &mut save.BLSTWD,
                save.ELSTWD,
                &mut save.A,
                &mut save.B,
                ctx,
            );

            save.CLSTWD = intrinsics::MIN0(&[save.ELSTWD, (save.BLSTWD + 8)]);
            save.VTEMPL = ((save.A != save.B)
                || (!M2KEYW(fstr::substr(TEMP, save.BLSTWD..=save.ELSTWD), ctx)
                    && fstr::eq(b"@calendar", fstr::substr(TEMP, save.BLSTWD..=save.CLSTWD))));

            if save.VTEMPL {
                //
                // There is a variable length template, the keywords
                // that might terminate this template are given
                // in TERMS up to the first occurance of a '}'.
                //
                save.I = (spicelib::LSTLTI(
                    save.ELSTWD,
                    spicelib::CARDI(save.INDXES.as_slice(), ctx)?,
                    save.INDXES.subarray(1),
                ) + 1);
                save.J = 0;

                //
                // Just load keywords onto the list until we hit a '}'
                // (We are guarenteed that this will happen, because
                // we put a '}' on the end of the list at the beginning
                // of this routine.)
                //
                while fstr::ne(save.TERMS.get(save.I), b"}") {
                    save.J = (save.J + 1);
                    M2TRIM(&save.TERMS[save.I], &mut save.KEYWDS[save.J], ctx);
                    save.I = (save.I + 1);
                }

                spicelib::SCARDC(save.J, save.KEYWDS.as_arg_mut(), ctx)?;
            } else {
                spicelib::SCARDC(0, save.KEYWDS.as_arg_mut(), ctx)?;
            }

            //
            // Check the current template with M2MTCH.
            //
            M2TCLR(ctx)?;
            M2MTCH(
                fstr::substr(TEMP, save.TBEG..=save.TEND),
                1,
                save.KEYWDS.as_arg(),
                STRING,
                SBEG,
                false,
                CUTOFF,
                &mut save.T1CODE,
                &mut save.TSCORE,
                CAUSE.as_arg_mut(),
                ctx,
            )?;

            //
            // If the attempt at a match succeeded ...
            //
            if (save.T1CODE == 0) {
                *SCORE = (*SCORE + save.TSCORE);
                save.TBEG = (save.TEND + 1);
                M2KEEP(ctx)?;
            } else {
                M2MTCH(
                    fstr::substr(TEMP, save.TBEG..=save.TEND),
                    1,
                    save.KEYWDS.as_arg(),
                    STRING,
                    SBEG,
                    REASON,
                    CUTOFF,
                    &mut save.T1CODE,
                    &mut save.TSCORE,
                    CAUSE.as_arg_mut(),
                    ctx,
                )?;

                *SCORE = (*SCORE + save.TSCORE);
                *M2CODE = save.T1CODE;
                save.MORE = false;
            }
        }
    }

    //
    // If there were no THNWDS and there is stuff left in the string and
    // we haven't already noticed, we've got an error dude.
    //
    if (((spicelib::CARDC(THNWDS.as_arg(), ctx)? == 0) && (*SBEG < QLSTNB(STRING)))
        && (*M2CODE == 0))
    {
        //
        // Until we have evidence to justify looking for probable causes
        // of the current overage of input string, we assume that we
        // are not interested in offering conjectures about what the
        // problem is.  We'll just say there is extra stuff.
        //
        save.REDIAG = false;

        //
        // Now look for justification of fancier diagnostics.
        //
        // Was the last thing we attempted to match part of a group
        // template?
        //
        if (fstr::eq(&save.LAST, b"GROUP") && (save.GMATCH < save.MOST)) {
            //
            // We are going to see if one of the options of an ending group
            // template looks like it was the intention of the user.
            //
            if (save.BCODE < 100) {
                //
                // We had a probable spelling error, set the flag to
                // diagnose the problem.
                //
                save.REDIAG = true;
            } else {
                //
                // Look at what the score could have been for the
                // simple template that was the closest match.
                //
                save.I = 1;
                save.J = save.BDIAGS;
                save.TSCORE = 0;

                spicelib::FNDNWD(TEMP, save.J, &mut save.I, &mut save.TMPJ);
                save.J = save.TMPJ;

                while ((save.I != 0) && (save.I < save.EDIAGS)) {
                    save.A = 1;
                    save.B = 1;

                    M2BEGR(TEMP, &mut save.I, save.J, &mut save.A, &mut save.B, ctx);

                    if M2KEYW(fstr::substr(TEMP, save.I..=save.J), ctx) {
                        save.TSCORE = (save.TSCORE + 100);
                    } else {
                        save.TSCORE = (save.TSCORE + (save.A * 15));
                    }

                    spicelib::FNDNWD(TEMP, save.J, &mut save.I, &mut save.TMPJ);
                    save.J = save.TMPJ;
                }

                //
                // If the score actually recorded made it at least a quarter
                // of the way, we will guess that this may have been the
                // root of the problem.
                //
                save.REDIAG = (save.BSCORE > intrinsics::MAX0(&[CUTOFF, (save.TSCORE / 4)]));
            }
        }

        //
        // If there was sufficient grounds to warrant second guessing,
        // run the best guess template through M2MTCH to get a diagnostic.
        //
        if save.REDIAG {
            if REASON {
                save.BS = save.BDIAGS;
                save.ES = save.EDIAGS;

                M2MTCH(
                    fstr::substr(TEMP, save.BS..=save.ES),
                    1,
                    save.KEYWDS.as_arg(),
                    STRING,
                    SBEG,
                    REASON,
                    CUTOFF,
                    &mut save.T1CODE,
                    &mut save.TSCORE,
                    CAUSE.as_arg_mut(),
                    ctx,
                )?;

                let val = CAUSE.get(1).to_vec();
                fstr::assign(CAUSE.get_mut(2), &val);

                save.B = (save.BEGOFG - 1);
                save.E = (save.ENDOFG + 2);

                spicelib::CMPRSS(
                    b" ",
                    1,
                    fstr::substr(TEMP, save.B..=save.E),
                    &mut save.SUBTMP,
                );

                if (intrinsics::INDEX(&save.SUBTMP, b" | ") != 0) {
                    spicelib::PREFIX(b"\'./cr/cr(-3:)", 1, &mut CAUSE[2]);
                    spicelib::PREFIX(
                        fstr::substr(TEMP, save.BDIAGS..=save.EDIAGS),
                        0,
                        &mut CAUSE[2],
                    );
                    spicelib::PREFIX(&fstr::concat(&fstr::concat(&fstr::concat(&fstr::concat(b"Extra words appear in the input string that are not part of a valid expression. I think you may have been trying to supply one of the optional expressions listed here:/cr/cr(3:) ", fstr::substr(&save.SUBTMP, 1 ..= spicelib::RTRIM(&save.SUBTMP))), b"/cr/cr(-3:). "), b"The expression that came "), b"closest was: /cr/cr(3:) \'"), 0, &mut CAUSE[2]);
                } else {
                    spicelib::PREFIX(b"\'./cr/cr(-3:)", 1, &mut CAUSE[2]);
                    spicelib::PREFIX(
                        fstr::substr(TEMP, save.BDIAGS..=save.EDIAGS),
                        0,
                        &mut CAUSE[2],
                    );
                    spicelib::PREFIX(b"Extra words appear in the input string that are not part of a valid expression. I think you may have been trying to supply the optional expression:/cr/cr(3:)\'", 0, &mut CAUSE[2]);
                }
            }

            //
            // Whatever error code we got back, add 10000 so that this
            // routine will have its stamp on it to indicate we are second
            // level guessing at what went wrong.
            //
            *M2CODE = (10000 + save.T1CODE);
        } else {
            //
            // Sorry, we couldn't guess why there was extra stuff in the
            // command.  Maybe just happy fingers.  Anyway, just say there
            // was extra stuff and hit the road.
            //
            if REASON {
                fstr::assign(CAUSE.get_mut(1), b"The input string contains extra words that are not recognized as part of a valid command.");

                M2MARK(STRING, *SBEG, QLSTNB(STRING), &mut CAUSE[1], ctx);
            }

            *M2CODE = 10200;
        }
    }

    Ok(())
}
