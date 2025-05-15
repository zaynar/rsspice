//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MAXLUN: i32 = 200;

//$Procedure T_TSTRLN (Test RESLUN)
pub fn T_TSTRLN(LUN: i32, RESRVD: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LUNARY = StackArray::<i32, 200>::new(1..=MAXLUN);
    let mut IOSTAT: i32 = 0;
    let mut I: i32 = 0;
    let mut DONE: bool = false;
    let mut OPENED: bool = false;
    let mut LUNFND: bool = false;
    let mut FNAME = [b' '; 128 as usize];

    //
    // SPICELIB Functions
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
        spicelib::CHKIN(b"T_TSTRLN", ctx)?;
    }

    //
    // Assume LUN is not reserved until proven otherwise.
    //
    *RESRVD = false;

    //
    // INQUIRE on LUN to see if it is attached to a file.
    //
    {
        use f2rust_std::io;

        let specs = io::InquireSpecs {
            unit: Some(LUN),
            opened: Some(&mut OPENED),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.inquire(specs))?;
    }

    //
    // Check IOSTAT for troubles.
    //
    if (IOSTAT != 0) {
        spicelib::SETMSG(
            b"Attempt to perform INQUIRE on logical unit, \'#\', failed.  IOSTAT = #.",
            ctx,
        );
        spicelib::ERRINT(b"#", LUN, ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
        spicelib::CHKOUT(b"T_TSTRLN", ctx)?;
        return Ok(());
    }

    //
    // Now check to see if LUN is attached to a file.
    //
    if OPENED {
        spicelib::SETMSG(b"The logical unit, \'#\', is currently attached to a file.  This routine can only verify the reserved state of units not currently attached to files.", ctx);
        spicelib::ERRINT(b"#", LUN, ctx);
        spicelib::SIGERR(b"SPICE(LUNINUSE)", ctx)?;
        spicelib::CHKOUT(b"T_TSTRLN", ctx)?;
        return Ok(());
    }

    //
    // This is a total kludge, but it will work.  We are going to
    // call FNDLUN until we run out of space for storing units or
    // we have exhausted all possible logical units.
    //
    LUNFND = false;
    DONE = false;
    I = 0;

    while !DONE {
        //
        // Increment I.
        //
        I = (I + 1);

        //
        // Fetch the next unit from FNDLUN.
        //
        spicelib::FNDLUN(&mut LUNARY[I], ctx)?;

        //
        // Check to see if the INQUIRE buried in FNDLUN failed.
        //
        if (LUNARY[I] < 0) {
            spicelib::SETMSG(b"INQUIRE failed. IOSTAT = #.", ctx);
            spicelib::ERRINT(b"#", -LUNARY[I], ctx);
            spicelib::SIGERR(b"SPICE(INQUIREFAILED)", ctx)?;
            spicelib::CHKOUT(b"T_TSTRLN", ctx)?;
            return Ok(());
        }

        //
        // Now see if LUN is LUNARY(I).  If it is stop, because
        // clearly LUN isn't reserved.
        //
        if (LUN == LUNARY[I]) {
            DONE = true;
            LUNFND = true;

        //
        // Then check to see if LUNARY(I) is 0, indicating that
        // FNDLUN was unable to locate a new logical unit, in
        // which case, we are finished.
        //
        } else if (LUNARY[I] == 0) {
            DONE = true;

        //
        // Otherwise open a scratch file on LUNARY(I) so FNDLUN
        // won't return it again.
        //
        } else {
            //
            // Create a temporary, unused file, at logical unit
            // LUNARY(I).
            //
            fstr::assign(&mut FNAME, b"tmp.#");
            spicelib::REPMI(&FNAME.clone(), b"#", I, &mut FNAME, ctx);

            {
                use f2rust_std::io;

                let specs = io::OpenSpecs {
                    unit: Some(LUNARY[I]),
                    file: Some(&FNAME),
                    form: Some(b"FORMATTED"),
                    access: Some(b"SEQUENTIAL"),
                    status: Some(b"NEW"),
                    ..Default::default()
                };
                IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
            }

            if (IOSTAT != 0) {
                spicelib::SETMSG(b"Could not open file #. IOSTAT was #. ", ctx);
                spicelib::ERRCH(b"#", &FNAME, ctx);
                spicelib::ERRINT(b"#", IOSTAT, ctx);
                spicelib::SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
                spicelib::CHKOUT(b"T_TSTRLN", ctx)?;
                return Ok(());
            }
        }

        //
        // Now see if entering the loop again will push I past
        // MAXLUN.  This should never happen since MAXLUN is
        // greater than the number of units for any platform.
        //
        if ((I == MAXLUN) || (LUNARY[I] == 0)) {
            DONE = true;
        }
    }

    //
    // Wrap up our analysis. Clean up the open units and delete the
    // corresponding files.
    //
    for J in 1..=(I - 1) {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(LUNARY[J]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        fstr::assign(&mut FNAME, b"tmp.#");
        spicelib::REPMI(&FNAME.clone(), b"#", J, &mut FNAME, ctx);
        spicelib::DELFIL(&FNAME, ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"T_TSTRLN", ctx)?;
            return Ok(());
        }
    }

    //
    // If we didn't find LUN in the list, then see if we exhausted
    // all possible logical units.
    //
    if !LUNFND {
        //
        // First check to see if we exhausted the available space to
        // store units.  If we did, signal an error to warn the
        // caller that the test results are inconclusive.
        //
        if ((I == MAXLUN) && (LUNARY[I] > 0)) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(LUNARY[I]),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            spicelib::SETMSG(b"FNDLUN returned too many logical units for the test to be conclusive.  Increase the parameter MAXLUN and recompile this module.", ctx);
            spicelib::SIGERR(b"SPICE(PARAMTOOSMALL)", ctx)?;
            spicelib::CHKOUT(b"T_TSTRLN", ctx)?;
            return Ok(());
        }

        //
        // If we make it this far, we did not use all of the available
        // space to store units.  Just for safety, check to see that
        // LUNARY(I) is 0.
        //
        if (LUNARY[I] != 0) {
            {
                use f2rust_std::io;

                let specs = io::CloseSpecs {
                    unit: Some(LUNARY[I]),
                    ..Default::default()
                };
                ctx.close(specs)?;
            }
            //
            // This should never happen, signal SPICE(BUG) and return.
            //
            spicelib::SETMSG(b"All of the unit storage space was not exhausted, and the last logical unit returned by FNDLUN was not zero.  This should never happen.", ctx);
            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
            spicelib::CHKOUT(b"T_TSTRLN", ctx)?;
            return Ok(());
        }

        //
        // All that remains is to set RESRVD to TRUE, since we have
        // exhausted all of the units in FNDLUN and LUN did not turn
        // up.
        //
        *RESRVD = true;
    }

    spicelib::CHKOUT(b"T_TSTRLN", ctx)?;
    Ok(())
}
