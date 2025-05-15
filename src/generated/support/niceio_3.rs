//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure     NICEIO_3 ( Nicely formatted output -- test version )
pub fn NICEIO_3(
    STRING: &[u8],
    UNIT: i32,
    STYLE: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut LEFT: i32 = 0;
    let mut RIGHT: i32 = 0;
    let mut NLINTK: bool = false;
    let mut VTABTK: bool = false;
    let mut FLAGTK: bool = false;
    let mut HARDSP: bool = false;
    let mut HSPCHR = [b' '; 1];
    let mut BEG: i32 = 0;
    let mut END: i32 = 0;
    let mut B: i32 = 0;
    let mut E: i32 = 0;
    let mut VBEG: i32 = 0;
    let mut VB: i32 = 0;
    let mut VE: i32 = 0;
    let mut FLAGB: i32 = 0;
    let mut FLAGE: i32 = 0;
    let mut PRAMBW: i32 = 0;
    let mut PSTAMB: i32 = 0;
    let mut VTABB: i32 = 0;
    let mut VTABE: i32 = 0;
    let mut VTABW: i32 = 0;
    let mut NLINB: i32 = 0;
    let mut NLINE: i32 = 0;
    let mut NLINEW: i32 = 0;
    let mut K: i32 = 0;
    let mut ERRORL = [b' '; 160];
    let mut ERRORR = [b' '; 160];
    let mut LINE = [b' '; 512];
    let mut BREAKS = [b' '; 1];
    let mut LEFTB: i32 = 0;
    let mut LEFTE: i32 = 0;
    let mut RIGHTB: i32 = 0;
    let mut RIGHTE: i32 = 0;
    let mut NLEFT: i32 = 0;
    let mut NRIGHT: i32 = 0;
    let mut WIDTH: i32 = 0;
    let mut W: i32 = 0;
    let mut START: i32 = 0;
    let mut VTABAT: i32 = 0;
    let mut INDENT: i32 = 0;
    let mut LOOPED: bool = false;
    let mut LRIGHT: i32 = 0;
    let mut LAST: i32 = 0;
    let mut NEWLIN: bool = false;
    let mut MRGCHG: bool = false;
    let mut LEADTK: bool = false;
    let mut TRLTK: bool = false;
    let mut FLAGW: i32 = 0;
    let mut LEADRW: i32 = 0;
    let mut TRAILW: i32 = 0;
    let mut ORIGR: i32 = 0;
    let mut ORIGL: i32 = 0;
    let mut RMARG: i32 = 0;
    let mut TRAILE: i32 = 0;
    let mut TRAILB: i32 = 0;
    let mut LEADRB: i32 = 0;
    let mut LEADRE: i32 = 0;

    //
    // SPICELIB Functions.
    //
    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NICEIO_3", ctx)?;
    }

    //
    // Set the defaults and initial values.
    //
    //
    // Set the defaults and initial values.
    //
    LEFT = 1;
    RIGHT = 80;

    FLAGTK = false;
    LEADTK = false;
    HARDSP = false;
    NLINTK = false;
    TRLTK = false;
    VTABTK = false;

    fstr::assign(&mut HSPCHR, b" ");

    FLAGW = 0;
    LEADRW = 0;
    TRAILW = 0;
    PRAMBW = 0;

    VTABW = 0;
    BEG = 1;
    fstr::assign(&mut ERRORL, b" ");
    fstr::assign(&mut ERRORR, b" ");
    fstr::assign(&mut BREAKS, b" ");

    //
    // Parse the style string.
    //

    spicelib::FNDNWD(STYLE, BEG, &mut B, &mut E);

    while (B != 0) {
        VBEG = (E + 1);

        spicelib::FNDNWD(STYLE, VBEG, &mut VB, &mut VE);

        if (VB != 0) {
            if fstr::eq(fstr::substr(STYLE, B..=E), b"FLAG") {
                FLAGB = VB;
                FLAGE = VE;
                FLAGW = ((VE - VB) + 2);
                FLAGTK = true;
            } else if fstr::eq(fstr::substr(STYLE, B..=E), b"LEADER") {
                LEADRB = VB;
                LEADRE = VE;
                LEADRW = ((VE - VB) + 1);
                LEADTK = true;
            } else if fstr::eq(fstr::substr(STYLE, B..=E), b"TRAILER") {
                TRAILB = VB;
                TRAILE = VE;
                TRAILW = ((VE - VB) + 1);
                TRLTK = true;
            } else if fstr::eq(fstr::substr(STYLE, B..=E), b"VTAB") {
                VTABB = VB;
                VTABE = VE;
                VTABW = ((VE - VB) + 1);
                VTABTK = true;
            } else if fstr::eq(fstr::substr(STYLE, B..=E), b"NEWLINE") {
                NLINB = VB;
                NLINE = VE;
                NLINEW = ((VE - VB) + 1);
                NLINTK = true;
            } else if fstr::eq(fstr::substr(STYLE, B..=E), b"LEFT") {
                spicelib::NPARSI(
                    fstr::substr(STYLE, VB..=VE),
                    &mut LEFT,
                    &mut ERRORL,
                    &mut K,
                    ctx,
                );
                if fstr::ne(&ERRORL, b" ") {
                    spicelib::SETMSG(
                        b"The word following the keyword \'LEFT\' must parse as an integer. # ",
                        ctx,
                    );
                    spicelib::ERRCH(b"#", &ERRORL, ctx);
                    spicelib::SIGERR(b"SPICE(NONNUMERICSTRING)", ctx)?;
                    spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                    return Ok(());
                }
            } else if fstr::eq(fstr::substr(STYLE, B..=E), b"RIGHT") {
                spicelib::NPARSI(
                    fstr::substr(STYLE, VB..=VE),
                    &mut RIGHT,
                    &mut ERRORR,
                    &mut K,
                    ctx,
                );
                if fstr::ne(&ERRORR, b" ") {
                    spicelib::SETMSG(
                        b"The word following the keyword \'RIGHT\' must parse as an integer. #",
                        ctx,
                    );
                    spicelib::ERRCH(b"#", &ERRORL, ctx);
                    spicelib::SIGERR(b"SPICE(NONNUMERICSTRING)", ctx)?;
                    spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                    return Ok(());
                }
            } else if fstr::eq(fstr::substr(STYLE, B..=E), b"HARDSPACE") {
                HARDSP = true;

                if (VB != VE) {
                    spicelib::SETMSG(
                        b"Hardspaces must be a single character.  You have \"#\".",
                        ctx,
                    );
                    spicelib::ERRCH(b"#", fstr::substr(STYLE, VB..=VE), ctx);
                    spicelib::SIGERR(b"SPICE(BADHARDSPACE)", ctx)?;
                    spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                    return Ok(());
                } else {
                    fstr::assign(&mut HSPCHR, fstr::substr(STYLE, VB..=VB));
                }
            } else {
                fstr::assign(&mut LINE, fstr::substr(STYLE, B..=E));
                spicelib::SUFFIX(
                    b"is not a recognized keyword for the SPICELIB routine NICEIO. ",
                    1,
                    &mut LINE,
                );

                spicelib::SETMSG(&LINE, ctx);
                spicelib::SIGERR(b"SPICE(UNKNOWNKEY)", ctx)?;
                spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                return Ok(());
            }

            BEG = (VE + 1);
            spicelib::FNDNWD(STYLE, BEG, &mut B, &mut E);
        } else {
            spicelib::SETMSG(b"# did not have an associated value", ctx);
            spicelib::ERRCH(b"#", fstr::substr(STYLE, B..=E), ctx);
            spicelib::SIGERR(b"SPICE(UNBALANCEDPAIR)", ctx)?;
            spicelib::CHKOUT(b"NICEIO_3", ctx)?;
            return Ok(());
        }
    }

    //
    // So ends the parsing of the style string.  Now do the actual work.
    //
    fstr::assign(&mut LINE, b" ");

    //
    // Determine how much space needs to be allocated for the
    // flag and leaders.
    //
    ORIGR = RIGHT;
    ORIGL = LEFT;
    RMARG = RIGHT;

    PRAMBW = intrinsics::MAX0(&[FLAGW, LEADRW]);
    PSTAMB = ((RIGHT - TRAILW) + 1);
    RIGHT = (RIGHT - TRAILW);

    if (FLAGW > 0) {
        fstr::assign(
            fstr::substr_mut(&mut LINE, ORIGL..),
            fstr::substr(STYLE, FLAGB..=FLAGE),
        );
    } else if (LEADRW > 0) {
        fstr::assign(
            fstr::substr_mut(&mut LINE, ORIGL..),
            fstr::substr(STYLE, LEADRB..=LEADRE),
        );
    }

    if (TRAILW > 0) {
        fstr::assign(
            fstr::substr_mut(&mut LINE, PSTAMB..=ORIGR),
            fstr::substr(STYLE, TRAILB..=TRAILE),
        );
    }

    B = intrinsics::MAX0(&[1, spicelib::FRSTNB(STRING)]);
    LAST = QLSTNB(STRING);
    //
    // If there is a newline token, we have to write out empty lines
    // and modify the margins as we encounter newline tokens and
    // newline tokens with margin modifiers.  Typically the loop
    // in the if block below will never be exercised.
    //
    if NLINTK {
        E = ((B + NLINEW) - 1);

        if (E < LAST) {
            NEWLIN = fstr::eq(
                fstr::substr(STRING, B..=E),
                fstr::substr(STYLE, NLINB..=NLINE),
            );
        } else {
            NEWLIN = false;
        }

        while NEWLIN {
            //
            // See if the new line token is qualified so as to change
            // the margins of the output.
            //
            if ((E + 1) < LAST) {
                MRGCHG = MATCH(fstr::substr(STRING, (E + 1)..), b"(*:*)*", ctx)?;
            } else {
                MRGCHG = false;
            }

            if MRGCHG {
                //
                // Looks like we should change the columns. Locate the
                // tokens of the newline marker.
                //
                FNDNTK(STRING, b"(:", (E + 1), &mut LEFTB, &mut LEFTE);
                FNDNTK(STRING, b":)", LEFTE, &mut RIGHTB, &mut RIGHTE);

                //
                // Parse the strings representing the increments to left
                // and right column positions.
                //
                fstr::assign(&mut ERRORL, b" ");
                fstr::assign(&mut ERRORR, b" ");

                if (LEFTB <= LEFTE) {
                    spicelib::NPARSI(
                        fstr::substr(STRING, LEFTB..=LEFTE),
                        &mut NLEFT,
                        &mut ERRORL,
                        &mut K,
                        ctx,
                    );
                } else {
                    NLEFT = 0;
                }

                if (RIGHTB <= RIGHTE) {
                    spicelib::NPARSI(
                        fstr::substr(STRING, RIGHTB..=RIGHTE),
                        &mut NRIGHT,
                        &mut ERRORR,
                        &mut K,
                        ctx,
                    );
                } else {
                    NRIGHT = 0;
                }

                //
                // Only if no errors were encountered during parsing do we
                // change the columns.
                //
                if (fstr::eq(&ERRORL, b" ") && fstr::eq(&ERRORR, b" ")) {
                    B = (RIGHTE + 2);
                    LEFT = (LEFT + NLEFT);
                    RIGHT = (RIGHT - NRIGHT);

                    RMARG = intrinsics::MAX0(&[ORIGR, RIGHT]);
                } else {
                    B = (B + NLINEW);
                }
            } else {
                B = (B + NLINEW);
            }

            //
            // Check for goofy margins.
            //
            if (LEFT < 1) {
                spicelib::SETMSG(b"The current value for the left column is #. This is less than 1 and thus not a valid value. ", ctx);
                spicelib::ERRINT(b"#", LEFT, ctx);
                spicelib::SIGERR(b"SPICE(INVALIDCOLUMN)", ctx)?;
                spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                return Ok(());
            } else if (LEFT > RIGHT) {
                spicelib::SETMSG(b"The current value for the left column is greater than the value for the right column. The value for the left column is #.  The value for the right column is #. ", ctx);
                spicelib::ERRINT(b"#", LEFT, ctx);
                spicelib::ERRINT(b"#", RIGHT, ctx);
                spicelib::SIGERR(b"SPICE(INVALIDCOLUMN)", ctx)?;
                spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                return Ok(());
            }

            //
            // Output something, but first replace hard spaces by spaces
            //

            if HARDSP {
                spicelib::REPLCH(
                    &fstr::substr(&LINE, 1..=RMARG).to_vec(),
                    &HSPCHR,
                    b" ",
                    fstr::substr_mut(&mut LINE, 1..=RMARG),
                );
            }

            spicelib::WRITLN(fstr::substr(&LINE, 1..=RMARG), UNIT, ctx)?;
            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut LINE, b" ");

            if LEADTK {
                fstr::assign(
                    fstr::substr_mut(&mut LINE, ORIGL..),
                    fstr::substr(STYLE, LEADRB..=LEADRE),
                );
            }

            if TRLTK {
                fstr::assign(
                    fstr::substr_mut(&mut LINE, PSTAMB..),
                    fstr::substr(STYLE, TRAILB..=TRAILE),
                );
            }

            //
            // Adjust the beginning and ending of the next portion
            // of the string to examine.
            //
            B = intrinsics::MAX0(&[B, spicelib::NCPOS(STRING, b" ", B)]);
            E = ((B + NLINEW) - 1);

            if (E < LAST) {
                NEWLIN = fstr::eq(
                    fstr::substr(STRING, B..=E),
                    fstr::substr(STYLE, NLINB..=NLINE),
                );
            } else {
                NEWLIN = false;
            }
        }
        //
        // Find the next portion of the string to examine (it's up to
        // the next new line token or end of string whichever
        // comes first.
        //
        E = UPTO(STRING, fstr::substr(STYLE, NLINB..=NLINE), B);
    } else {
        E = LAST;
    }

    //
    // Now we have are to the point of processing legitimate text.
    // Process the current substring  STRING(B:E).  It contains
    // no newline tokens.
    //
    while (E != 0) {
        WIDTH = (((RIGHT - LEFT) + 1) - PRAMBW);

        if (WIDTH < 1) {
            spicelib::SIGERR(b"SPICE(SPACETOONARROW)", ctx)?;
            spicelib::CHKOUT(b"NICEIO_3", ctx)?;
            return Ok(());
        }

        W = WIDTH;
        START = B;
        INDENT = 0;

        //
        // Grab the biggest piece of the substring that can be output
        // within the allowed space.
        //

        CUTSTR(
            fstr::substr(STRING, 1..=E),
            START,
            W,
            &BREAKS,
            &mut BEG,
            &mut END,
            ctx,
        )?;

        while (BEG != 0) {
            //
            // See if there are any vertical tab marks
            //
            if !VTABTK {
                fstr::assign(
                    fstr::substr_mut(&mut LINE, (LEFT + PRAMBW)..=RIGHT),
                    fstr::substr(STRING, BEG..=END),
                );
            } else {
                VTABAT = spicelib::POS(
                    fstr::substr(STRING, 1..=E),
                    fstr::substr(STYLE, VTABB..=VTABE),
                    START,
                );

                if ((VTABAT > 0) && (VTABAT <= END)) {
                    //
                    // If there is a vertical tab at the beginning of the
                    // string, we don't need to modify LINE.
                    //
                    if (VTABAT > BEG) {
                        END = (VTABAT - 1);
                        fstr::assign(
                            fstr::substr_mut(&mut LINE, ((LEFT + PRAMBW) + INDENT)..=RIGHT),
                            fstr::substr(STRING, BEG..=END),
                        );

                        INDENT = (((INDENT + END) - BEG) + 1);
                        END = (((END + VTABE) - VTABB) + 1);
                    } else if (VTABAT == BEG) {
                        fstr::assign(
                            fstr::substr_mut(&mut LINE, ((LEFT + PRAMBW) + INDENT)..=RIGHT),
                            b" ",
                        );
                        END = ((BEG + VTABE) - VTABB);
                    }
                } else {
                    //
                    // We just fill out the rest of this line.  There will
                    // be no need to indent the next one.
                    //
                    fstr::assign(
                        fstr::substr_mut(&mut LINE, ((LEFT + PRAMBW) + INDENT)..=RIGHT),
                        fstr::substr(STRING, BEG..=END),
                    );
                    INDENT = 0;
                }
            }

            //
            // Handle any hard spaces
            //
            if HARDSP {
                spicelib::REPLCH(
                    &fstr::substr(&LINE, 1..=RMARG).to_vec(),
                    &HSPCHR,
                    b" ",
                    fstr::substr_mut(&mut LINE, 1..=RMARG),
                );
            }

            spicelib::WRITLN(fstr::substr(&LINE, 1..=RMARG), UNIT, ctx)?;
            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut LINE, b" ");

            if LEADTK {
                fstr::assign(
                    fstr::substr_mut(&mut LINE, ORIGL..),
                    fstr::substr(STYLE, LEADRB..=LEADRE),
                );
            }

            if TRLTK {
                fstr::assign(
                    fstr::substr_mut(&mut LINE, PSTAMB..),
                    fstr::substr(STYLE, TRAILB..=TRAILE),
                );
            }

            START = (END + 1);
            W = (WIDTH - INDENT);

            if (W < 3) {
                W = WIDTH;
                INDENT = 0;
            }

            CUTSTR(
                fstr::substr(STRING, 1..=E),
                START,
                W,
                &BREAKS,
                &mut BEG,
                &mut END,
                ctx,
            )?;
        }

        //
        // Check to see if we should be looking for a newline token.
        //
        if NLINTK {
            //
            // Ok.  Get ready to jump through hoops again.  We have to
            // look for newline tokens, for all those in excess of one
            // in a row, we have to output a blank line.
            //
            B = (E + 1);
            E = (E + NLINEW);
            LOOPED = false;

            if (E <= LAST) {
                NEWLIN = fstr::eq(
                    fstr::substr(STRING, B..=E),
                    fstr::substr(STYLE, NLINB..=NLINE),
                );
            } else {
                NEWLIN = false;
            }

            while NEWLIN {
                LRIGHT = RIGHT;
                //
                // See if the new line token is qualified so as to change
                // the margins of the output.
                //
                if (E >= LAST) {
                    //
                    // In this case we can't possibly match as in the case
                    // below
                    //
                    B = (B + NLINEW);
                } else if MATCH(fstr::substr(STRING, (E + 1)..), b"(*:*)*", ctx)? {
                    //
                    // Looks like we should change the columns. Locate the
                    // tokens of the newline marker.
                    //
                    FNDNTK(STRING, b"(:", E, &mut LEFTB, &mut LEFTE);
                    FNDNTK(STRING, b":)", LEFTE, &mut RIGHTB, &mut RIGHTE);

                    //
                    // Parse the strings representing the increments to left
                    // and right column positions.
                    //
                    fstr::assign(&mut ERRORL, b" ");
                    fstr::assign(&mut ERRORR, b" ");

                    if (LEFTB <= LEFTE) {
                        spicelib::NPARSI(
                            fstr::substr(STRING, LEFTB..=LEFTE),
                            &mut NLEFT,
                            &mut ERRORL,
                            &mut K,
                            ctx,
                        );
                    } else {
                        NLEFT = 0;
                    }

                    if (RIGHTB <= RIGHTE) {
                        spicelib::NPARSI(
                            fstr::substr(STRING, RIGHTB..=RIGHTE),
                            &mut NRIGHT,
                            &mut ERRORR,
                            &mut K,
                            ctx,
                        );
                    } else {
                        NRIGHT = 0;
                    }

                    //
                    // Only if no errors were encountered during parsing
                    // do we change the columns.
                    //
                    if (fstr::eq(&ERRORL, b" ") && fstr::eq(&ERRORR, b" ")) {
                        B = (RIGHTE + 2);
                        LEFT = (LEFT + NLEFT);
                        RIGHT = (RIGHT - NRIGHT);
                        RMARG = intrinsics::MAX0(&[ORIGR, RIGHT]);
                    } else {
                        B = (B + NLINEW);
                    }
                } else {
                    B = (B + NLINEW);
                }

                //
                // Take care of the case when outdenting or indenting has
                // forced us into absurd margins.
                //
                if (LEFT < 1) {
                    spicelib::SETMSG(b"The current value for the left column is #. This is less than 1 and thus not a valid value. ", ctx);
                    spicelib::ERRINT(b"#", LEFT, ctx);
                    spicelib::SIGERR(b"SPICE(INVALIDCOLUMN)", ctx)?;
                    spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                    return Ok(());
                } else if (LEFT > RIGHT) {
                    spicelib::SETMSG(b"The current value for the left column is greater than the value for the right column. The value for the left column is #.  The value for the right column is #. ", ctx);
                    spicelib::ERRINT(b"#", LEFT, ctx);
                    spicelib::ERRINT(b"#", RIGHT, ctx);
                    spicelib::SIGERR(b"SPICE(INVALIDCOLUMN)", ctx)?;
                    spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                    return Ok(());
                }

                //
                // Output something if this is not the first pass through
                // the loop.
                //
                if !LOOPED {
                    LOOPED = true;
                    fstr::assign(&mut LINE, b" ");

                    if LEADTK {
                        fstr::assign(
                            fstr::substr_mut(&mut LINE, ORIGL..),
                            fstr::substr(STYLE, LEADRB..=LEADRE),
                        );
                    }

                    if TRLTK {
                        fstr::assign(
                            fstr::substr_mut(&mut LINE, PSTAMB..),
                            fstr::substr(STYLE, TRAILB..=TRAILE),
                        );
                    }
                } else {
                    //
                    // Handle any hard spaces
                    //
                    if HARDSP {
                        spicelib::REPLCH(
                            &fstr::substr(&LINE, 1..=RMARG).to_vec(),
                            &HSPCHR,
                            b" ",
                            fstr::substr_mut(&mut LINE, 1..=RMARG),
                        );
                    }

                    spicelib::WRITLN(fstr::substr(&LINE, 1..=RMARG), UNIT, ctx)?;
                    if spicelib::FAILED(ctx) {
                        spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                        return Ok(());
                    }
                }

                B = intrinsics::MAX0(&[B, spicelib::NCPOS(STRING, b" ", B)]);
                E = ((B + NLINEW) - 1);

                if (E <= LAST) {
                    NEWLIN = fstr::eq(
                        fstr::substr(STRING, B..=E),
                        fstr::substr(STYLE, NLINB..=NLINE),
                    );
                } else {
                    NEWLIN = false;
                }
            }

            E = UPTO(STRING, fstr::substr(STYLE, NLINB..=NLINE), B);

            //
            // Just in case we went through the loop, and didn't
            // output a line, and we've reached the end of the
            // string.  We check and write a blank line if necessary
            //
            if (LOOPED && (E == 0)) {
                //
                // Handle any hard spaces
                //
                if HARDSP {
                    spicelib::REPLCH(
                        &fstr::substr(&LINE, 1..=RMARG).to_vec(),
                        &HSPCHR,
                        b" ",
                        fstr::substr_mut(&mut LINE, 1..=RMARG),
                    );
                }

                spicelib::WRITLN(fstr::substr(&LINE, 1..=RMARG), UNIT, ctx)?;
                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"NICEIO_3", ctx)?;
                    return Ok(());
                }
            }
        } else {
            E = 0;
        }
    }

    spicelib::CHKOUT(b"NICEIO_3", ctx)?;
    Ok(())
}
