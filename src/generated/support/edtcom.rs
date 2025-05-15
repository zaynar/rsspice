//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NONE: i32 = 0;
const COMBUF: i32 = (NONE + 1);
const KEYBRD: i32 = (COMBUF + 1);
const INPFIL: i32 = (KEYBRD + 1);
const LNSIZE: i32 = 132;
const BSIZE: i32 = 80;
const INTLEN: i32 = 3;
const LBCELL: i32 = -5;
const WDSIZE: i32 = 32;

struct SaveVars {
    SPACE: Vec<u8>,
    TAB: Vec<u8>,
    REST: Vec<u8>,
    DSTRNG: Vec<u8>,
    BUFFER: ActualCharArray,
    EDITOR: Vec<u8>,
    ERROR: ActualCharArray,
    ERRSTR: Vec<u8>,
    NAME: Vec<u8>,
    LINE: Vec<u8>,
    PATTRN: Vec<u8>,
    FRSTWD: Vec<u8>,
    SCNDWD: Vec<u8>,
    B1: i32,
    B2: i32,
    COMNUM: i32,
    DEPTH: i32,
    E1: i32,
    E2: i32,
    I: i32,
    IOSTAT: i32,
    PTR: i32,
    R: i32,
    UNIT: i32,
    FIRST: bool,
    LSTAT: StackArray<bool, 3>,
    SSTAT: StackArray<bool, 3>,
    STATUS: StackArray<bool, 3>,
    SVSTAT: StackArray<bool, 3>,
    GOTONE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SPACE = vec![b' '; 1 as usize];
        let mut TAB = vec![b' '; 1 as usize];
        let mut REST = vec![b' '; 1760 as usize];
        let mut DSTRNG = vec![b' '; INTLEN as usize];
        let mut BUFFER = ActualCharArray::new(LNSIZE, LBCELL..=BSIZE);
        let mut EDITOR = vec![b' '; LNSIZE as usize];
        let mut ERROR = ActualCharArray::new(LNSIZE, 1..=2);
        let mut ERRSTR = vec![b' '; LNSIZE as usize];
        let mut NAME = vec![b' '; LNSIZE as usize];
        let mut LINE = vec![b' '; LNSIZE as usize];
        let mut PATTRN = vec![b' '; LNSIZE as usize];
        let mut FRSTWD = vec![b' '; WDSIZE as usize];
        let mut SCNDWD = vec![b' '; WDSIZE as usize];
        let mut B1: i32 = 0;
        let mut B2: i32 = 0;
        let mut COMNUM: i32 = 0;
        let mut DEPTH: i32 = 0;
        let mut E1: i32 = 0;
        let mut E2: i32 = 0;
        let mut I: i32 = 0;
        let mut IOSTAT: i32 = 0;
        let mut PTR: i32 = 0;
        let mut R: i32 = 0;
        let mut UNIT: i32 = 0;
        let mut FIRST: bool = false;
        let mut LSTAT = StackArray::<bool, 3>::new(1..=3);
        let mut SSTAT = StackArray::<bool, 3>::new(1..=3);
        let mut STATUS = StackArray::<bool, 3>::new(1..=3);
        let mut SVSTAT = StackArray::<bool, 3>::new(1..=3);
        let mut GOTONE: bool = false;

        fstr::assign(&mut EDITOR, b"emacs");
        FIRST = true;

        Self {
            SPACE,
            TAB,
            REST,
            DSTRNG,
            BUFFER,
            EDITOR,
            ERROR,
            ERRSTR,
            NAME,
            LINE,
            PATTRN,
            FRSTWD,
            SCNDWD,
            B1,
            B2,
            COMNUM,
            DEPTH,
            E1,
            E2,
            I,
            IOSTAT,
            PTR,
            R,
            UNIT,
            FIRST,
            LSTAT,
            SSTAT,
            STATUS,
            SVSTAT,
            GOTONE,
        }
    }
}

//$Procedure      EDTCOM (Edit a command)
pub fn EDTCOM(
    DELIM: &[u8],
    PROMPT: &[u8],
    COMMND: &mut [u8],
    SOURCE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DELIM = &DELIM[..1 as usize];

    //
    // Spicelib Functions
    //

    //
    // Meta/2 functions
    //

    //
    // Below are the various sources from which
    // commands might come.
    //
    // NONE
    // COMBUF
    // KEYBRD
    // INPFIL
    //

    //
    // Local parameters used for allocating space and controlling loop
    // execution.
    //

    //
    // Local Variables.
    //

    //
    // The only time an EDIT/RECALL/DO command can have any meaning
    // is when it comes from the keyboard.
    //
    if (*SOURCE != KEYBRD) {
        return Ok(());
    }

    //
    // Initialize the syntax for the preprocessing commands
    //
    if save.FIRST {
        save.FIRST = false;

        fstr::assign(&mut save.TAB, &intrinsics::CHAR(9));
        fstr::assign(&mut save.SPACE, b" ");
    }

    //
    // Next we take apart the command and see if it is one of the
    // preprocessing commands.
    //
    spicelib::NEXTWD(COMMND, &mut save.FRSTWD, &mut save.REST);
    spicelib::NEXTWD(&save.REST.to_vec(), &mut save.SCNDWD, &mut save.REST);

    //
    // We probably don't have any of the pathologies below, but they
    // are easy to check so we handle them here.
    //
    if fstr::ne(&save.REST, b" ") {
        return Ok(());
    }

    if fstr::eq(&save.FRSTWD, b" ") {
        return Ok(());
    }

    save.B1 = 1;
    save.B2 = 1;
    save.E1 = spicelib::RTRIM(&save.FRSTWD);
    save.E2 = spicelib::RTRIM(&save.SCNDWD);

    if (fstr::eq(&save.SCNDWD, b" ") && !M2WMCH(&save.FRSTWD, save.B1, save.E1, b"RECALL", ctx)?) {
        return Ok(());
    }

    //
    // We need the beginning and endings of the words we've extracted.
    //
    save.B1 = 1;
    save.B2 = 1;
    save.E1 = spicelib::RTRIM(&save.FRSTWD);
    save.E2 = spicelib::RTRIM(&save.SCNDWD);

    if (M2WMCH(&save.FRSTWD, save.B1, save.E1, b"RECALL", ctx)? && fstr::eq(&save.SCNDWD, b" ")) {
        //
        // We don't want the RECALL command to show up in the
        // output.
        //
        DMPBUF(ctx);

        //
        // We don't write the output of a RECALL command to the
        // log file.
        //
        NSPGST(b"LOG", save.STATUS.as_slice_mut(), ctx)?;
        NSPIOH(b"LOG", ctx)?;
        //
        // Determine the depth of the command line buffer.
        //
        GETBSZ(&mut save.DEPTH, ctx);
        //
        // Fetch each paragraph and display it.
        //
        while (save.DEPTH > 0) {
            spicelib::SSIZEC(BSIZE, save.BUFFER.as_arg_mut(), ctx)?;
            spicelib::INTSTR(save.DEPTH, &mut save.DSTRNG, ctx);
            spicelib::LJUST(&save.DSTRNG.to_vec(), &mut save.DSTRNG);
            GETBUF(save.DEPTH, save.BUFFER.as_arg_mut(), ctx)?;

            fstr::assign(
                &mut save.LINE,
                &fstr::concat(fstr::substr(&save.DSTRNG, 1..=3), save.BUFFER.get(1)),
            );

            NSPWLN(&save.LINE, ctx)?;

            {
                let m1__: i32 = 2;
                let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
                let m3__: i32 = 1;
                save.I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    fstr::assign(
                        &mut save.LINE,
                        &fstr::concat(b"   ", save.BUFFER.get(save.I)),
                    );
                    NSPWLN(&save.LINE, ctx)?;
                    save.I += m3__;
                }
            }

            save.DEPTH = (save.DEPTH - 1);
        }
        //
        // Reset the status of the LOG file back to whatever it
        // was before we started dumping old commands.  Finally
        // set the command to a blank.
        //
        NSPPST(b"LOG", save.STATUS.as_slice(), ctx)?;
        fstr::assign(COMMND, b" ");
        *SOURCE = NONE;

        return Ok(());
    } else if (M2WMCH(&save.FRSTWD, save.B1, save.E1, b"RECALL", ctx)?
        && M2WMCH(&save.SCNDWD, save.B2, save.E2, b"@int(1:20)", ctx)?)
    {
        //
        // We don't write the output of a RECALL command to the
        // log file.
        //
        NSPGST(b"LOG", save.STATUS.as_slice_mut(), ctx)?;
        NSPIOH(b"LOG", ctx)?;
        //
        // Find out the depth of the command to fetch.
        //
        spicelib::NPARSI(
            &save.SCNDWD,
            &mut save.DEPTH,
            &mut save.ERRSTR,
            &mut save.PTR,
            ctx,
        );
        //
        // Get rid of the top command (it's the RECALL command).
        //
        DMPBUF(ctx);
        spicelib::SSIZEC(BSIZE, save.BUFFER.as_arg_mut(), ctx)?;
        GETBUF(save.DEPTH, save.BUFFER.as_arg_mut(), ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(
                    &mut save.LINE,
                    &fstr::concat(b"   ", save.BUFFER.get(save.I)),
                );
                NSPWLN(&save.LINE, ctx)?;
                save.I += m3__;
            }
        }
        //
        // Reset the status of the LOG file back to whatever it
        // was before we started dumping old commands.  Finally
        // set the command to a blank.
        //
        NSPPST(b"LOG", save.STATUS.as_slice(), ctx)?;
        fstr::assign(COMMND, b" ");
        *SOURCE = NONE;
        return Ok(());
    } else if M2WMCH(&save.FRSTWD, save.B1, save.E1, b"RECALL", ctx)? {
        //
        // Find out the depth of the command to fetch.
        //
        GETBSZ(&mut save.DEPTH, ctx);
        spicelib::SUFFIX(b"*", 0, &mut save.SCNDWD);

        save.COMNUM = 2;
        save.GOTONE = false;

        while (save.COMNUM <= save.DEPTH) {
            spicelib::SSIZEC(BSIZE, save.BUFFER.as_arg_mut(), ctx)?;
            GETBUF(save.COMNUM, save.BUFFER.as_arg_mut(), ctx)?;

            if ((spicelib::CARDC(save.BUFFER.as_arg(), ctx)? > 0)
                && MATCH(&save.BUFFER[1], &save.SCNDWD, ctx)?)
            {
                //
                // We don't write the output of a RECALL command to the
                // log file.
                //
                NSPGST(b"LOG", save.STATUS.as_slice_mut(), ctx)?;
                NSPIOH(b"LOG", ctx)?;
                //
                // Dump the top command as it is just the recall command.
                //
                DMPBUF(ctx);
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        fstr::assign(
                            &mut save.LINE,
                            &fstr::concat(b"   ", save.BUFFER.get(save.I)),
                        );
                        NSPWLN(&save.LINE, ctx)?;
                        save.I += m3__;
                    }
                }

                save.COMNUM = save.DEPTH;
                save.GOTONE = true;
                fstr::assign(COMMND, b" ");
                *SOURCE = NONE;

                NSPPST(b"LOG", save.STATUS.as_slice(), ctx)?;
            }

            save.COMNUM = (save.COMNUM + 1);
        }
        //
        // Reset the status of the LOG file back to whatever it
        // was before we started dumping old commands.
        //
        if !save.GOTONE {
            fstr::assign(
                save.ERROR.get_mut(1),
                b"There is no command in the command history list that matches \'#\'. ",
            );

            spicelib::REPMC(
                &save.ERROR[1].to_vec(),
                b"#",
                &save.SCNDWD,
                &mut save.ERROR[1],
            );
            fstr::assign(COMMND, b" ");
            *SOURCE = NONE;
            spicelib::SETMSG(save.ERROR.first(), ctx);
            spicelib::SIGERR(b"EDTCOM(NOMATCH)", ctx)?;
            return Ok(());
        }

        return Ok(());
    } else if (M2WMCH(&save.FRSTWD, save.B1, save.E1, b"EDIT", ctx)?
        && M2WMCH(&save.SCNDWD, save.B2, save.E2, b"@int(1:20)", ctx)?)
    {
        spicelib::NPARSI(
            &save.SCNDWD,
            &mut save.DEPTH,
            &mut save.ERRSTR,
            &mut save.PTR,
            ctx,
        );
        DMPBUF(ctx);
        spicelib::SSIZEC(BSIZE, save.BUFFER.as_arg_mut(), ctx)?;
        GETBUF(save.DEPTH, save.BUFFER.as_arg_mut(), ctx)?;
        //
        // Open the utility port to receive the contents of BUFFER.
        //
        fstr::assign(&mut save.PATTRN, b"edt{0-z}{0-z}{0-z}{0-z}{0-z}.tmp");

        NEWFIL(&save.PATTRN, b"UTILITY", &mut save.NAME, ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::RESET(ctx);
            spicelib::CHKIN(b"EDTCOM", ctx)?;
            spicelib::SETMSG(b"The program was unable to open a file that could be used with the editor. Command editing cannot be performed at this time. ", ctx);
            spicelib::SIGERR(b"CMLOOP(COMMANDEDITFAILED)", ctx)?;
            spicelib::CHKOUT(b"EDTCOM", ctx)?;
            return Ok(());
        }
        //
        // We have at this point succeeded in opening a file
        // into which we can write the last command.  But we
        // don't want to write to the screen, log file or save
        // file if there is one. Inhibit writing to any port
        // but the utility port.
        //
        NSPGST(b"LOG", save.LSTAT.as_slice_mut(), ctx)?;
        NSPGST(b"SCREEN", save.SSTAT.as_slice_mut(), ctx)?;
        NSPGST(b"SAVE", save.SVSTAT.as_slice_mut(), ctx)?;

        NSPIOH(b"LOG", ctx)?;
        NSPIOH(b"SCREEN", ctx)?;
        NSPIOH(b"SAVE", ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                NSPWLN(&save.BUFFER[save.I], ctx)?;
                save.I += m3__;
            }
        }

        NSPIOC(b"UTILITY", ctx)?;
        //
        // Activate the editor
        //
        EDTCMD(
            fstr::substr(&save.EDITOR, 1..=spicelib::RTRIM(&save.EDITOR)),
            fstr::substr(&save.NAME, 1..=spicelib::RTRIM(&save.NAME)),
            ctx,
        )?;

        fstr::assign(save.ERROR.get_mut(1), b" ");
        fstr::assign(save.ERROR.get_mut(2), b" ");

        if HAVE(save.ERROR.as_arg_mut(), ctx)? {
            NSPPST(b"LOG", save.LSTAT.as_slice(), ctx)?;
            NSPPST(b"SCREEN", save.SSTAT.as_slice(), ctx)?;
            NSPPST(b"SAVE", save.SVSTAT.as_slice(), ctx)?;
            fstr::assign(COMMND, b" ");
            *SOURCE = NONE;
            spicelib::SETMSG(save.ERROR.first(), ctx);
            spicelib::SIGERR(b"SPICE(FILEREADERROR)", ctx)?;
            return Ok(());
        }
        //
        // Read the first command from the edited file.
        //

        PRSTRT(&save.NAME, save.ERROR.first_mut(), ctx)?;

        if HAVE(save.ERROR.as_arg_mut(), ctx)? {
            NSPPST(b"LOG", save.LSTAT.as_slice(), ctx)?;
            NSPPST(b"SCREEN", save.SSTAT.as_slice(), ctx)?;
            NSPPST(b"SAVE", save.SVSTAT.as_slice(), ctx)?;
            fstr::assign(COMMND, b" ");
            *SOURCE = NONE;
            PREXIT(ctx)?;
            spicelib::SETMSG(save.ERROR.first(), ctx);
            spicelib::SIGERR(b"SPICE(FILEREADERROR)", ctx)?;
            return Ok(());
        }

        PRREAD(DELIM, COMMND, ctx)?;
        PUTCOM(COMMND, KEYBRD, ctx)?;
        PREXIT(ctx)?;

        //
        // Finally, delete the file we used with the editor.
        //
        spicelib::TXTOPR(&save.NAME, &mut save.UNIT, ctx)?;
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.UNIT),
                status: Some(b"DELETE"),
                ..Default::default()
            };
            save.IOSTAT = io::capture_iostat(|| ctx.close(specs))?;
        }

        spicelib::SSIZEC(BSIZE, save.BUFFER.as_arg_mut(), ctx)?;
        GETBUF(1, save.BUFFER.as_arg_mut(), ctx)?;

        NSPIOA(b"SCREEN", ctx)?;

        save.R = (spicelib::RTRIM(PROMPT) + 2);

        {
            let m1__: i32 = 1;
            let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if (save.I == 1) {
                    fstr::assign(&mut save.LINE, PROMPT);
                    spicelib::SUFFIX(&save.BUFFER[save.I], 1, &mut save.LINE);
                } else {
                    fstr::assign(&mut save.LINE, b" ");
                    fstr::assign(
                        fstr::substr_mut(&mut save.LINE, save.R..),
                        save.BUFFER.get(save.I),
                    );
                }

                NSPWLN(
                    fstr::substr(&save.LINE, 1..=spicelib::RTRIM(&save.LINE)),
                    ctx,
                )?;
                save.I += m3__;
            }
        }

        //
        // Reset the writing to all other ports.
        //
        NSPPST(b"LOG", save.LSTAT.as_slice(), ctx)?;
        NSPPST(b"SCREEN", save.SSTAT.as_slice(), ctx)?;
        NSPPST(b"SAVE", save.SVSTAT.as_slice(), ctx)?;

        fstr::assign(COMMND, b" ");
        *SOURCE = NONE;
    } else if (M2WMCH(&save.FRSTWD, save.B1, save.E1, b"EDIT", ctx)?
        && fstr::ne(&save.SCNDWD, b" "))
    {
        save.GOTONE = false;
        save.COMNUM = 2;
        GETBSZ(&mut save.DEPTH, ctx);
        spicelib::SUFFIX(b"*", 0, &mut save.SCNDWD);

        while (save.COMNUM <= save.DEPTH) {
            spicelib::SSIZEC(BSIZE, save.BUFFER.as_arg_mut(), ctx)?;
            GETBUF(save.COMNUM, save.BUFFER.as_arg_mut(), ctx)?;

            if ((spicelib::CARDC(save.BUFFER.as_arg(), ctx)? > 0)
                && MATCH(&save.BUFFER[1], &save.SCNDWD, ctx)?)
            {
                save.GOTONE = true;
                DMPBUF(ctx);

                //
                // Open the utility port to receive the contents of BUFFER.
                //
                fstr::assign(&mut save.PATTRN, b"edt{0-z}{0-z}{0-z}{0-z}{0-z}.tmp");

                NEWFIL(&save.PATTRN, b"UTILITY", &mut save.NAME, ctx)?;

                if spicelib::FAILED(ctx) {
                    spicelib::RESET(ctx);
                    spicelib::CHKIN(b"EDTCOM", ctx)?;
                    spicelib::SETMSG(b"The program was unable to open a file that could be used with the editor. Command editing cannot be performed at this time. ", ctx);

                    spicelib::SIGERR(b"CMLOOP(COMMANDEDITFAILED)", ctx)?;
                    spicelib::CHKOUT(b"EDTCOM", ctx)?;
                    return Ok(());
                }

                //
                // We have at this point succeeded in opening a file
                // into which we can write the last command.  But we
                // don't want to write to the screen, log file or save
                // file if there is one. Inhibit writing to any port
                // but the utility port.
                //
                NSPGST(b"LOG", save.LSTAT.as_slice_mut(), ctx)?;
                NSPGST(b"SCREEN", save.SSTAT.as_slice_mut(), ctx)?;
                NSPGST(b"SAVE", save.SVSTAT.as_slice_mut(), ctx)?;

                NSPIOH(b"LOG", ctx)?;
                NSPIOH(b"SCREEN", ctx)?;
                NSPIOH(b"SAVE", ctx)?;

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        NSPWLN(&save.BUFFER[save.I], ctx)?;
                        save.I += m3__;
                    }
                }

                NSPIOC(b"UTILITY", ctx)?;

                //
                // Activate the editor
                //
                EDTCMD(
                    fstr::substr(&save.EDITOR, 1..=spicelib::RTRIM(&save.EDITOR)),
                    fstr::substr(&save.NAME, 1..=spicelib::RTRIM(&save.NAME)),
                    ctx,
                )?;

                fstr::assign(save.ERROR.get_mut(1), b" ");
                fstr::assign(save.ERROR.get_mut(2), b" ");

                if HAVE(save.ERROR.as_arg_mut(), ctx)? {
                    NSPPST(b"LOG", save.LSTAT.as_slice(), ctx)?;
                    NSPPST(b"SCREEN", save.SSTAT.as_slice(), ctx)?;
                    NSPPST(b"SAVE", save.SVSTAT.as_slice(), ctx)?;
                    fstr::assign(COMMND, b" ");
                    *SOURCE = NONE;
                    spicelib::SETMSG(save.ERROR.first(), ctx);
                    spicelib::SIGERR(b"SPICE(FILEREADERROR)", ctx)?;
                    return Ok(());
                }

                //
                // Read the first command from the edited file.
                //

                PRSTRT(&save.NAME, save.ERROR.first_mut(), ctx)?;

                if HAVE(save.ERROR.as_arg_mut(), ctx)? {
                    NSPPST(b"LOG", save.LSTAT.as_slice(), ctx)?;
                    NSPPST(b"SCREEN", save.SSTAT.as_slice(), ctx)?;
                    NSPPST(b"SAVE", save.SVSTAT.as_slice(), ctx)?;
                    fstr::assign(COMMND, b" ");
                    *SOURCE = NONE;
                    PREXIT(ctx)?;
                    spicelib::SETMSG(save.ERROR.first(), ctx);
                    spicelib::SIGERR(b"SPICE(FILEREADERROR)", ctx)?;
                    return Ok(());
                }

                PRREAD(DELIM, COMMND, ctx)?;
                PUTCOM(COMMND, KEYBRD, ctx)?;
                PREXIT(ctx)?;

                //
                // Finally, delete the file we used with the editor.
                //
                spicelib::TXTOPR(&save.NAME, &mut save.UNIT, ctx)?;
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(save.UNIT),
                        status: Some(b"DELETE"),
                        ..Default::default()
                    };
                    save.IOSTAT = io::capture_iostat(|| ctx.close(specs))?;
                }

                spicelib::SSIZEC(BSIZE, save.BUFFER.as_arg_mut(), ctx)?;
                GETBUF(1, save.BUFFER.as_arg_mut(), ctx)?;

                NSPIOA(b"SCREEN", ctx)?;

                save.R = (spicelib::RTRIM(PROMPT) + 2);

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        if (save.I == 1) {
                            fstr::assign(&mut save.LINE, PROMPT);
                            spicelib::SUFFIX(&save.BUFFER[save.I], 1, &mut save.LINE);
                        } else {
                            fstr::assign(&mut save.LINE, b" ");
                            fstr::assign(
                                fstr::substr_mut(&mut save.LINE, save.R..),
                                save.BUFFER.get(save.I),
                            );
                        }

                        NSPWLN(
                            fstr::substr(&save.LINE, 1..=spicelib::RTRIM(&save.LINE)),
                            ctx,
                        )?;
                        save.I += m3__;
                    }
                }

                //
                // Reset the writing to all other ports.
                //
                NSPPST(b"LOG", save.LSTAT.as_slice(), ctx)?;
                NSPPST(b"SCREEN", save.SSTAT.as_slice(), ctx)?;
                NSPPST(b"SAVE", save.SVSTAT.as_slice(), ctx)?;

                fstr::assign(COMMND, b" ");
                *SOURCE = NONE;

                save.COMNUM = save.DEPTH;
            }

            save.COMNUM = (save.COMNUM + 1);
        }

        if !save.GOTONE {
            fstr::assign(
                save.ERROR.get_mut(1),
                b"There is no command in the command history list that matches \'#\'. ",
            );

            spicelib::REPMC(
                &save.ERROR[1].to_vec(),
                b"#",
                &save.SCNDWD,
                &mut save.ERROR[1],
            );
            fstr::assign(COMMND, b" ");
            *SOURCE = NONE;
            spicelib::SETMSG(save.ERROR.first(), ctx);
            spicelib::SIGERR(b"EDTCOM(NOMATCH)", ctx)?;
            return Ok(());
        }
    } else if (M2WMCH(&save.FRSTWD, save.B1, save.E1, b"DO", ctx)?
        && M2WMCH(&save.SCNDWD, save.B2, save.E2, b"@int(1:20)", ctx)?)
    {
        spicelib::NPARSI(
            &save.SCNDWD,
            &mut save.DEPTH,
            &mut save.ERRSTR,
            &mut save.PTR,
            ctx,
        );
        DMPBUF(ctx);
        spicelib::SSIZEC(BSIZE, save.BUFFER.as_arg_mut(), ctx)?;
        GETBUF(save.DEPTH, save.BUFFER.as_arg_mut(), ctx)?;

        NSPGST(b"SCREEN", save.SSTAT.as_slice_mut(), ctx)?;
        NSPIOA(b"SCREEN", ctx)?;

        save.R = (spicelib::RTRIM(PROMPT) + 2);
        //
        // Reset the paragraph buffer so it can receive another
        // paragraph. (This is where we buffer commands and we
        // need to buffer this one.)
        //
        RSTBUF(ctx);

        {
            let m1__: i32 = 1;
            let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                PUTBUF(&save.BUFFER[save.I], ctx);

                if (save.I == 1) {
                    fstr::assign(&mut save.LINE, PROMPT);
                    spicelib::SUFFIX(&save.BUFFER[save.I], 1, &mut save.LINE);
                } else {
                    fstr::assign(&mut save.LINE, b" ");
                    fstr::assign(
                        fstr::substr_mut(&mut save.LINE, save.R..),
                        save.BUFFER.get(save.I),
                    );
                }

                NSPWLN(
                    fstr::substr(&save.LINE, 1..=spicelib::RTRIM(&save.LINE)),
                    ctx,
                )?;
                save.I += m3__;
            }
        }

        NSPPST(b"SCREEN", save.SSTAT.as_slice(), ctx)?;

        fstr::assign(COMMND, b" ");

        fstr::assign(COMMND, save.BUFFER.get(1));

        {
            let m1__: i32 = 2;
            let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                spicelib::SUFFIX(&save.BUFFER[save.I], 1, COMMND);
                save.I += m3__;
            }
        }

        save.I = intrinsics::INDEX(COMMND, DELIM);

        if (save.I > 0) {
            PUTCOM(fstr::substr(COMMND, 1..=(save.I - 1)), KEYBRD, ctx)?;
        } else {
            PUTCOM(COMMND, COMBUF, ctx)?;
        }

        fstr::assign(COMMND, b" ");
        *SOURCE = NONE;
    } else if (M2WMCH(&save.FRSTWD, save.B1, save.E1, b"DO", ctx)? && fstr::ne(&save.SCNDWD, b" "))
    {
        //
        // This is basically the same as the last case, but
        // we look for a pattern match before doing anything.
        //
        save.GOTONE = false;
        GETBSZ(&mut save.DEPTH, ctx);
        spicelib::SUFFIX(b"*", 0, &mut save.SCNDWD);
        save.COMNUM = 2;

        while (save.COMNUM <= save.DEPTH) {
            spicelib::SSIZEC(BSIZE, save.BUFFER.as_arg_mut(), ctx)?;
            GETBUF(save.COMNUM, save.BUFFER.as_arg_mut(), ctx)?;

            if ((spicelib::CARDC(save.BUFFER.as_arg(), ctx)? > 0)
                && MATCH(&save.BUFFER[1], &save.SCNDWD, ctx)?)
            {
                save.GOTONE = true;
                DMPBUF(ctx);

                NSPGST(b"SCREEN", save.SSTAT.as_slice_mut(), ctx)?;
                NSPIOA(b"SCREEN", ctx)?;

                save.R = (spicelib::RTRIM(PROMPT) + 2);
                //
                // Reset the paragraph buffer so it can receive another
                // paragraph. (This is where we buffer commands and we
                // need to buffer this one.)
                //
                RSTBUF(ctx);

                {
                    let m1__: i32 = 1;
                    let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        PUTBUF(&save.BUFFER[save.I], ctx);
                        if (save.I == 1) {
                            fstr::assign(&mut save.LINE, PROMPT);
                            spicelib::SUFFIX(&save.BUFFER[save.I], 1, &mut save.LINE);
                        } else {
                            fstr::assign(&mut save.LINE, b" ");
                            fstr::assign(
                                fstr::substr_mut(&mut save.LINE, save.R..),
                                save.BUFFER.get(save.I),
                            );
                        }

                        NSPWLN(
                            fstr::substr(&save.LINE, 1..=spicelib::RTRIM(&save.LINE)),
                            ctx,
                        )?;
                        save.I += m3__;
                    }
                }

                NSPPST(b"SCREEN", save.SSTAT.as_slice(), ctx)?;

                fstr::assign(COMMND, b" ");

                fstr::assign(COMMND, save.BUFFER.get(1));

                {
                    let m1__: i32 = 2;
                    let m2__: i32 = spicelib::CARDC(save.BUFFER.as_arg(), ctx)?;
                    let m3__: i32 = 1;
                    save.I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        spicelib::SUFFIX(&save.BUFFER[save.I], 1, COMMND);
                        save.I += m3__;
                    }
                }

                save.I = intrinsics::INDEX(COMMND, DELIM);

                if (save.I > 0) {
                    PUTCOM(fstr::substr(COMMND, 1..=(save.I - 1)), KEYBRD, ctx)?;
                } else {
                    PUTCOM(COMMND, COMBUF, ctx)?;
                }

                fstr::assign(COMMND, b" ");
                *SOURCE = NONE;

                save.COMNUM = save.DEPTH;
            }

            save.COMNUM = (save.COMNUM + 1);
        }

        if !save.GOTONE {
            fstr::assign(
                save.ERROR.get_mut(1),
                b"There is no command in the command history list that matches \'#\'. ",
            );

            spicelib::REPMC(
                &save.ERROR[1].to_vec(),
                b"#",
                &save.SCNDWD,
                &mut save.ERROR[1],
            );
            fstr::assign(COMMND, b" ");
            *SOURCE = NONE;
            spicelib::SETMSG(save.ERROR.first(), ctx);
            spicelib::SIGERR(b"EDTCOM(NOMATCH)", ctx)?;
            return Ok(());
        }
    }

    Ok(())
}

pub fn SETEDT(COMMND: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.EDITOR, COMMND);
}

pub fn GETEDT(COMMND: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(COMMND, &save.EDITOR);
}
