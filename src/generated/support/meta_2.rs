//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const NAMLEN: i32 = 6;
const NUMKEY: i32 = 10;
const KEYLEN: i32 = 32;

struct SaveVars {
    I: i32,
    J: i32,
    BSCORE: i32,
    THNWDS: ActualCharArray,
    SBEG: i32,
    REASON: bool,
    CUTOFF: i32,
    PSSTHN: bool,
    M2CODE: i32,
    SCORE: i32,
    MODE: Vec<u8>,
    INTRCT: bool,
    TRYIT: Vec<u8>,
    KWORDS: ActualCharArray,
    KEYNAM: ActualCharArray,
    B: i32,
    E: i32,
    PICK: Vec<u8>,
    FIXIT: bool,
    MARGNS: Vec<u8>,
    STYLE: Vec<u8>,
    QUESTN: Vec<u8>,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut BSCORE: i32 = 0;
        let mut THNWDS = ActualCharArray::new(32, LBCELL..=1);
        let mut SBEG: i32 = 0;
        let mut REASON: bool = false;
        let mut CUTOFF: i32 = 0;
        let mut PSSTHN: bool = false;
        let mut M2CODE: i32 = 0;
        let mut SCORE: i32 = 0;
        let mut MODE = vec![b' '; 16];
        let mut INTRCT: bool = false;
        let mut TRYIT = vec![b' '; 600];
        let mut KWORDS = ActualCharArray::new(KEYLEN, LBCELL..=NUMKEY);
        let mut KEYNAM = ActualCharArray::new(NAMLEN, 1..=NUMKEY);
        let mut B: i32 = 0;
        let mut E: i32 = 0;
        let mut PICK = vec![b' '; KEYLEN as usize];
        let mut FIXIT: bool = false;
        let mut MARGNS = vec![b' '; 128 as usize];
        let mut STYLE = vec![b' '; 128 as usize];
        let mut QUESTN = vec![b' '; 80 as usize];
        let mut PASS1: bool = false;

        PASS1 = true;
        fstr::assign(&mut MARGNS, b"LEFT 1 RIGHT 75 ");
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"1"),
                Val::C(b"2"),
                Val::C(b"3"),
                Val::C(b"4"),
                Val::C(b"5"),
                Val::C(b"6"),
                Val::C(b"7"),
                Val::C(b"8"),
                Val::C(b"9"),
                Val::C(b"10"),
            ]
            .into_iter();
            for I in intrinsics::range(1, 10, 1) {
                fstr::assign(KEYNAM.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            I,
            J,
            BSCORE,
            THNWDS,
            SBEG,
            REASON,
            CUTOFF,
            PSSTHN,
            M2CODE,
            SCORE,
            MODE,
            INTRCT,
            TRYIT,
            KWORDS,
            KEYNAM,
            B,
            E,
            PICK,
            FIXIT,
            MARGNS,
            STYLE,
            QUESTN,
            PASS1,
        }
    }
}

//$Procedure      META_2 ( Percy's interface to META_0 )
pub fn META_2(
    COMMAND: &mut [u8],
    TEMPS: CharArray,
    NTEMPS: i32,
    TEMP: &mut [u8],
    BTEMP: &mut i32,
    ERROR: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let TEMPS = DummyCharArray::new(TEMPS, None, 1..);
    let mut ERROR = DummyCharArrayMut::new(ERROR, None, 1..=2);

    //
    // Spice Functions
    //
    //
    // Local variables.
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //%&END_DECLARATIONS

    //
    // Take care of first pass initializations.
    //
    if save.PASS1 {
        save.PASS1 = false;

        spicelib::SSIZEC(1, save.THNWDS.as_arg_mut(), ctx)?;
        spicelib::SCARDC(0, save.THNWDS.as_arg_mut(), ctx)?;

        spicelib::SSIZEC(NUMKEY, save.KWORDS.as_arg_mut(), ctx)?;
        spicelib::SCARDC(0, save.KWORDS.as_arg_mut(), ctx)?;

        //
        // Determine if were in batch or interactive mode.
        //
        if BATCH(ctx) {
            fstr::assign(&mut save.MODE, b"BATCH");
        } else {
            fstr::assign(&mut save.MODE, b"INTERACTIVE");
        }
    }

    save.INTRCT = fstr::ne(&save.MODE, b"BATCH");
    fstr::assign(&mut save.STYLE, &save.MARGNS);
    spicelib::SUFFIX(b"NEWLINE /cr VTAB /vt HARDSPACE , ", 1, &mut save.STYLE);

    save.I = 0;
    save.BSCORE = -1;
    save.M2CODE = -1;
    save.CUTOFF = 72;
    save.REASON = true;

    //
    // Look through the templates until we get a match or we
    // run out of templates to try.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NTEMPS;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.SCORE = 0;
            fstr::assign(TEMP, TEMPS.get(save.I));
            save.SBEG = 1;
            save.M2CODE = 0;

            M2GMCH(
                TEMP,
                save.THNWDS.as_arg(),
                COMMAND,
                &mut save.SBEG,
                save.REASON,
                save.CUTOFF,
                &mut save.PSSTHN,
                &mut save.M2CODE,
                &mut save.SCORE,
                ERROR.as_arg_mut(),
                ctx,
            )?;
            //
            // If M2CODE comes back zero, we are done with the work
            // of this routine.
            //
            if (save.M2CODE == 0) {
                *BTEMP = save.I;
                return Ok(());
            }

            if (save.SCORE > save.BSCORE) {
                save.BSCORE = save.SCORE;
                *BTEMP = save.I;
            }

            save.I += m3__;
        }
    }

    //
    // If we get here, we know we didn't have a match.  Examine the
    // highest scoring template to get available diagnostics
    // about the mismatch.
    //

    fstr::assign(TEMP, TEMPS.get(*BTEMP));
    save.SBEG = 1;
    save.FIXIT = true;
    save.M2CODE = 0;

    M2GMCH(
        TEMP,
        save.THNWDS.as_arg(),
        COMMAND,
        &mut save.SBEG,
        true,
        save.CUTOFF,
        &mut save.PSSTHN,
        &mut save.M2CODE,
        &mut save.SCORE,
        ERROR.as_arg_mut(),
        ctx,
    )?;

    //
    // If we are in interactiive mode and we have a spelling error, we
    // can attempt to fix it.  Note this occurs only if the M2CODE
    // is less than 100 mod 10000.
    //
    while (((intrinsics::MOD(save.M2CODE, 10000) < 100) && save.INTRCT) && save.FIXIT) {
        //
        // Construct a friendly message; display it; and
        // get the user's response as to whether or not the
        // command should be modified.
        //

        fstr::assign(&mut save.TRYIT, ERROR.get(1));

        spicelib::PREFIX(b"Hmmmm.,,,", 1, &mut save.TRYIT);
        spicelib::SUFFIX(b"/cr/cr I can repair this if you like.", 0, &mut save.TRYIT);

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(6)?, None)?;
            writer.start()?;
            writer.finish()?;
        }
        NICEIO_3(&save.TRYIT, 6, &save.STYLE, ctx)?;
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(6)?, None)?;
            writer.start()?;
            writer.finish()?;
        }
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(6)?, None)?;
            writer.start()?;
            writer.finish()?;
        }
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(6)?, None)?;
            writer.start()?;
            writer.finish()?;
        }
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(6)?, None)?;
            writer.start()?;
            writer.finish()?;
        }

        M2RCVR(&mut save.B, &mut save.E, save.KWORDS.as_arg_mut(), ctx)?;

        if (spicelib::CARDC(save.KWORDS.as_arg(), ctx)? == 1) {
            fstr::assign(
                &mut save.QUESTN,
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(
                                b"Should I change \"",
                                fstr::substr(COMMAND, save.B..=save.E),
                            ),
                            b"\" to \"",
                        ),
                        fstr::substr(save.KWORDS.get(1), 1..=spicelib::RTRIM(&save.KWORDS[1])),
                    ),
                    b"\" ?",
                ),
            );
            CNFIRM_1(
                fstr::substr(&save.QUESTN, 1..=spicelib::RTRIM(&save.QUESTN)),
                &mut save.FIXIT,
                ctx,
            )?;
        } else {
            CNFIRM_1(b"Should I fix it?", &mut save.FIXIT, ctx)?;
        }

        //
        // If the user has elected to have us fix the command
        // we have a few things to do...
        //
        if save.FIXIT {
            //
            // Look up the suggested fixes.  If there is more than
            // one possibility, see which one the user thinks is
            // best.  Otherwise, no more questions for now.
            //
            M2RCVR(&mut save.B, &mut save.E, save.KWORDS.as_arg_mut(), ctx)?;

            if (spicelib::CARDC(save.KWORDS.as_arg(), ctx)? > 1) {
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = (spicelib::CARDC(save.KWORDS.as_arg(), ctx)? - 4);
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        {
                            use f2rust_std::{
                                data::Val,
                                io::{self, Writer},
                            };

                            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(6)?, None)?;
                            writer.start()?;
                            writer.finish()?;
                        }
                        save.I += m3__;
                    }
                }

                GETOPT_1(
                    b"Which word did you mean?",
                    spicelib::CARDC(save.KWORDS.as_arg(), ctx)?,
                    save.KEYNAM.subarray(1),
                    NAMLEN,
                    save.KWORDS.subarray(1),
                    KEYLEN,
                    save.KWORDS.subarray(1),
                    &mut save.PICK,
                    ctx,
                )?;
            } else {
                fstr::assign(&mut save.PICK, save.KWORDS.get(1));
            }
            //
            // Make the requested repairs on the command, and
            // redisplay the command.
            //
            spicelib::REPSUB(&COMMAND.to_vec(), save.B, save.E, &save.PICK, COMMAND, ctx)?;
            spicelib::CMPRSS(b" ", 1, &COMMAND.to_vec(), COMMAND);
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.io_unit(6)?, None)?;
                writer.start()?;
                writer.write_str(b" ")?;
                writer.finish()?;
            }
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.io_unit(6)?, None)?;
                writer.start()?;
                writer.write_str(b" ")?;
                writer.finish()?;
            }
            NICEIO_3(COMMAND, 6, &save.STYLE, ctx)?;
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.io_unit(6)?, None)?;
                writer.start()?;
                writer.finish()?;
            }

            //
            // Look through the templates again until we get a match or we
            // run out of templates to try.  Note however, that this time
            // we will start in a different spot.  We already have a best
            // matching template.  We'll start our search for a match
            // there and simulate a circular list of templates so that
            // we can examine all of them if necessary.
            //
            fstr::assign(ERROR.get_mut(1), b" ");
            fstr::assign(ERROR.get_mut(2), b" ");
            save.BSCORE = -1;
            save.M2CODE = -1;
            save.CUTOFF = 72;
            save.REASON = true;
            save.J = (*BTEMP - 1);

            {
                let m1__: i32 = 1;
                let m2__: i32 = NTEMPS;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    //
                    // Get the index of the next template to examine.
                    //
                    save.J = (save.J + 1);
                    while (save.J > NTEMPS) {
                        save.J = (save.J - NTEMPS);
                    }
                    //
                    // Set the template, score for this template, spot to
                    // begin examining it and the M2CODE so far.
                    //
                    fstr::assign(TEMP, TEMPS.get(save.J));
                    save.SBEG = 1;
                    save.SCORE = 0;
                    save.M2CODE = 0;

                    M2GMCH(
                        TEMP,
                        save.THNWDS.as_arg(),
                        COMMAND,
                        &mut save.SBEG,
                        save.REASON,
                        save.CUTOFF,
                        &mut save.PSSTHN,
                        &mut save.M2CODE,
                        &mut save.SCORE,
                        ERROR.as_arg_mut(),
                        ctx,
                    )?;
                    //
                    // If we get back a zero M2CODE we've got a match
                    // This routine's work is done.
                    //
                    if (save.M2CODE == 0) {
                        *BTEMP = save.I;
                        return Ok(());
                    }

                    //
                    // Hmmph.  No match.  See if we've got a better
                    // matching score so far and then go on to the next
                    // template if any are left.
                    //
                    if (save.SCORE > save.BSCORE) {
                        save.BSCORE = save.SCORE;
                        *BTEMP = save.I;
                    }

                    save.I += m3__;
                }
            }

            //
            // If we made it to this point the command doesn't properly
            // match any of the templates.  Get the best match and
            // determine the diagnostics for this template.
            //
            fstr::assign(TEMP, TEMPS.get(*BTEMP));
            save.SBEG = 1;
            save.M2CODE = 0;
            save.SCORE = 0;

            M2GMCH(
                TEMP,
                save.THNWDS.as_arg(),
                COMMAND,
                &mut save.SBEG,
                save.REASON,
                save.CUTOFF,
                &mut save.PSSTHN,
                &mut save.M2CODE,
                &mut save.SCORE,
                ERROR.as_arg_mut(),
                ctx,
            )?;
        }
    }

    //
    // If you get to this point. We didn't have a match set up
    // the second level of mismatch diagnostics using the best
    // matching template.  (BTEMP already points to it.)
    //
    fstr::assign(TEMP, TEMPS.get(*BTEMP));

    spicelib::CMPRSS(b" ", 1, &TEMP.to_vec(), TEMP);
    PREPSN(TEMP, ctx);
    PREPSN(&mut ERROR[2], ctx);
    spicelib::PREFIX(b"/cr/cr(-3:-3) ", 1, &mut ERROR[2]);
    spicelib::PREFIX(TEMP, 1, &mut ERROR[2]);
    spicelib::PREFIX(b"/cr/cr(3:3) ", 1, &mut ERROR[2]);
    spicelib::PREFIX(b"a command with the following syntax:", 3, &mut ERROR[2]);
    spicelib::PREFIX(b"I Believe you were trying to enter", 1, &mut ERROR[2]);
    spicelib::PREFIX(b"META/2:", 1, &mut ERROR[2]);

    Ok(())
}

//
// The following entry point allows user's to adjust the margins
// of the META/2 error messages.
//
pub fn M2MARG(TEMP: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.MARGNS, TEMP);
}
