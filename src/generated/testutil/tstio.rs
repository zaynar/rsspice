//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NPORTS: i32 = 4;
const SCREEN: i32 = 6;
const FILSIZ: i32 = 127;
const LOG: i32 = 1;
const STDOUT: i32 = 2;
const SPOOL: i32 = 3;
const WDSIZE: i32 = 32;
const MSGSIZ: i32 = 400;

struct SaveVars {
    ID: i32,
    IOSTAT: i32,
    R: i32,
    TO: i32,
    MESSGE: Vec<u8>,
    PORTS: ActualCharArray,
    FILES: ActualCharArray,
    UNITS: StackArray<i32, 4>,
    ACTIVE: StackArray<bool, 4>,
    OPEN: StackArray<bool, 4>,
    SUSPND: StackArray<bool, 4>,
    LKPORT: StackArray<bool, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ID: i32 = 0;
        let mut IOSTAT: i32 = 0;
        let mut R: i32 = 0;
        let mut TO: i32 = 0;
        let mut MESSGE = vec![b' '; MSGSIZ as usize];
        let mut PORTS = ActualCharArray::new(WDSIZE, 1..=NPORTS);
        let mut FILES = ActualCharArray::new(FILSIZ, 1..=NPORTS);
        let mut UNITS = StackArray::<i32, 4>::new(1..=NPORTS);
        let mut ACTIVE = StackArray::<bool, 4>::new(1..=NPORTS);
        let mut OPEN = StackArray::<bool, 4>::new(1..=NPORTS);
        let mut SUSPND = StackArray::<bool, 4>::new(1..=NPORTS);
        let mut LKPORT = StackArray::<bool, 4>::new(1..=NPORTS);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"LOG"),
                Val::C(b"SCREEN"),
                Val::C(b"SAVE"),
                Val::C(b"UTILITY"),
            ]
            .into_iter();
            PORTS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b" "), Val::C(b" "), Val::C(b" "), Val::C(b" ")].into_iter();
            FILES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0), Val::I(SCREEN), Val::I(0), Val::I(0)].into_iter();
            UNITS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(false), Val::L(true), Val::L(false), Val::L(false)].into_iter();
            ACTIVE
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(false), Val::L(true), Val::L(false), Val::L(false)].into_iter();
            OPEN.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist =
                [Val::L(false), Val::L(false), Val::L(false), Val::L(false)].into_iter();
            SUSPND
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist =
                [Val::L(false), Val::L(false), Val::L(false), Val::L(false)].into_iter();
            LKPORT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ID,
            IOSTAT,
            R,
            TO,
            MESSGE,
            PORTS,
            FILES,
            UNITS,
            ACTIVE,
            OPEN,
            SUSPND,
            LKPORT,
        }
    }
}

//$Procedure TSTIO ( Test utilities IO Manager )
pub fn TSTIO(LINE: &[u8], NAME: &[u8], PORT: &[u8], OK: bool, STATUS: &[bool]) {

    //
    // Below are the various types of output files that
    // might be open.
    //

    //
    // Define tags for the PORTS array (defined in tstlgr.f)
    // indices corresponding to the log file, LOG ('LOG'),
    // the screen or standard out, STDOUT ('SCREEN'),
    // and the error log file SPOOL ('SAVE'). Note the 'SAVE'
    // port as defined in PORTS use the SPOOL tag because SAVE
    // is a FORTRAN intrinsic function.
    //

    //
    // SPICELIB functions
    //
}

//$Procedure TSTOPN ( Open a new port )
pub fn TSTOPN(PORT: &[u8], NAME: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.ID = spicelib::ISRCHC(PORT, NPORTS, save.PORTS.as_arg());

    if (save.ID == 0) {
        spicelib::CHKIN(b"TSTOPN", ctx)?;
        spicelib::SETMSG(b"Unrecognized port: #", ctx);
        spicelib::ERRCH(b"#", PORT, ctx);
        spicelib::SIGERR(b"TESTUTIL(BUG)", ctx)?;
        spicelib::CHKOUT(b"TSTOPN", ctx)?;
        return Ok(());
    }

    //
    // Check for an ID lockout.
    //
    if save.LKPORT[save.ID] {
        return Ok(());
    }

    save.R = spicelib::RTRIM(NAME);

    spicelib::TXTOPN(
        fstr::substr(NAME, 1..=save.R),
        &mut save.UNITS[save.ID],
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        save.OPEN[save.ID] = false;
        save.ACTIVE[save.ID] = false;
        save.SUSPND[save.ID] = false;
        fstr::assign(save.FILES.get_mut(save.ID), b" ");
        return Ok(());
    }

    save.OPEN[save.ID] = true;
    save.ACTIVE[save.ID] = true;
    save.SUSPND[save.ID] = false;
    fstr::assign(save.FILES.get_mut(save.ID), NAME);

    Ok(())
}

//$Procedure TSTIOH ( Inhibit output to a port )
pub fn TSTIOH(PORT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.ID = spicelib::ISRCHC(PORT, NPORTS, save.PORTS.as_arg());

    if (save.ID == 0) {
        spicelib::CHKIN(b"TSTIOH", ctx)?;
        spicelib::SETMSG(b"Unrecognized port: #", ctx);
        spicelib::ERRCH(b"#", PORT, ctx);
        spicelib::SIGERR(b"TESTUTIL(BUG)", ctx)?;
        spicelib::CHKOUT(b"TSTIOH", ctx)?;
        return Ok(());
    }

    save.ACTIVE[save.ID] = false;
    Ok(())
}

//$Procedure TSTIOA ( Activate a port )
pub fn TSTIOA(PORT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.ID = spicelib::ISRCHC(PORT, NPORTS, save.PORTS.as_arg());

    if (save.ID == 0) {
        spicelib::CHKIN(b"TSTIOA", ctx)?;
        spicelib::SETMSG(b"Unrecognized port: #", ctx);
        spicelib::ERRCH(b"#", PORT, ctx);
        spicelib::SIGERR(b"TESTUTIL(BUG)", ctx)?;
        spicelib::CHKOUT(b"TSTIOA", ctx)?;
        return Ok(());
    }

    //
    // Check for an ID lockout.
    //
    if save.LKPORT[save.ID] {
        return Ok(());
    }

    save.ACTIVE[save.ID] = true;

    Ok(())
}

//$Procedure TSTGST ( Get the current status of a port )
pub fn TSTGST(PORT: &[u8], STATUS: &mut [bool], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STATUS = DummyArrayMut::new(STATUS, 1..=3);

    save.ID = spicelib::ISRCHC(PORT, NPORTS, save.PORTS.as_arg());

    if (save.ID == 0) {
        spicelib::CHKIN(b"TSTGST", ctx)?;
        spicelib::SETMSG(b"Unrecognized port: #", ctx);
        spicelib::ERRCH(b"#", PORT, ctx);
        spicelib::SIGERR(b"TESTUTIL(BUG)", ctx)?;
        spicelib::CHKOUT(b"TSTGST", ctx)?;
        return Ok(());
    }

    STATUS[1] = save.ACTIVE[save.ID];
    STATUS[2] = save.OPEN[save.ID];
    STATUS[3] = save.SUSPND[save.ID];

    Ok(())
}

//$Procedure TSTPST ( Put the status of a port )
pub fn TSTPST(PORT: &[u8], STATUS: &[bool], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let STATUS = DummyArray::new(STATUS, 1..=3);

    save.ID = spicelib::ISRCHC(PORT, NPORTS, save.PORTS.as_arg());

    if (save.ID == 0) {
        spicelib::CHKIN(b"TSTPST", ctx)?;
        spicelib::SETMSG(b"Unrecognized port: #", ctx);
        spicelib::ERRCH(b"#", PORT, ctx);
        spicelib::SIGERR(b"TESTUTIL(BUG)", ctx)?;
        spicelib::CHKOUT(b"TSTPST", ctx)?;
        return Ok(());
    }

    //
    // Check for an ID lockout.
    //
    if save.LKPORT[save.ID] {
        return Ok(());
    }

    save.ACTIVE[save.ID] = STATUS[1];
    save.OPEN[save.ID] = STATUS[2];
    save.SUSPND[save.ID] = STATUS[3];

    Ok(())
}

//$Procedure TSTIOC ( Close a port )
pub fn TSTIOC(PORT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.ID = spicelib::ISRCHC(PORT, NPORTS, save.PORTS.as_arg());

    if (save.ID == 0) {
        spicelib::CHKIN(b"TSTIOC", ctx)?;
        spicelib::SETMSG(b"Unrecognized port: #", ctx);
        spicelib::ERRCH(b"#", PORT, ctx);
        spicelib::SIGERR(b"TESTUTIL(BUG)", ctx)?;
        spicelib::CHKOUT(b"TSTIOC", ctx)?;
        return Ok(());
    }

    //
    // Check for an ID lockout.
    //
    if save.LKPORT[save.ID] {
        return Ok(());
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(save.UNITS[save.ID]),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    save.ACTIVE[save.ID] = false;
    save.OPEN[save.ID] = false;
    fstr::assign(save.FILES.get_mut(save.ID), b" ");
    save.SUSPND[save.ID] = false;
    Ok(())
}

//$Procedure TSTIOS ( Suspend a port )
pub fn TSTIOS(PORT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.ID = spicelib::ISRCHC(PORT, NPORTS, save.PORTS.as_arg());

    if (save.ID == 0) {
        spicelib::CHKIN(b"TSTIOS", ctx)?;
        spicelib::SETMSG(b"Unrecognized port: #", ctx);
        spicelib::ERRCH(b"#", PORT, ctx);
        spicelib::SIGERR(b"TESTUTIL(BUG)", ctx)?;
        spicelib::CHKOUT(b"TSTIOS", ctx)?;
        return Ok(());
    }

    //
    // Check for an ID lockout.
    //
    if save.LKPORT[save.ID] {
        return Ok(());
    }

    //
    // close ( units(id) )
    //
    save.OPEN[save.ID] = false;
    save.SUSPND[save.ID] = true;

    Ok(())
}

//$Procedure TSTIOR ( Reopen a suspended port )
pub fn TSTIOR(PORT: &[u8], OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.ID = spicelib::ISRCHC(PORT, NPORTS, save.PORTS.as_arg());

    if (save.ID == 0) {
        spicelib::CHKIN(b"TSTIOR", ctx)?;
        spicelib::SETMSG(b"Unrecognized port: #", ctx);
        spicelib::ERRCH(b"#", PORT, ctx);
        spicelib::SIGERR(b"TESTUTIL(BUG)", ctx)?;
        spicelib::CHKOUT(b"TSTIOR", ctx)?;
        return Ok(());
    }

    //
    // Check for an ID lockout.
    //
    if save.LKPORT[save.ID] {
        return Ok(());
    }

    if !save.SUSPND[save.ID] {
        *OK = false;
        return Ok(());
    }

    save.OPEN[save.ID] = true;
    save.SUSPND[save.ID] = false;
    *OK = true;

    Ok(())
}

//$Procedure TSTWLN ( Write a line to all open and active ports )
pub fn TSTWLN(LINE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    {
        let m1__: i32 = 1;
        let m2__: i32 = NPORTS;
        let m3__: i32 = 1;
        save.ID = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (save.ACTIVE[save.ID] && save.OPEN[save.ID]) {
                save.TO = save.UNITS[save.ID];
                spicelib::WRITLN(fstr::substr(LINE, 1..=spicelib::RTRIM(LINE)), save.TO, ctx)?;

                if ((save.ID != STDOUT) && spicelib::FAILED(ctx)) {
                    save.R = spicelib::RTRIM(&save.FILES[save.ID]);

                    fstr::assign(&mut save.MESSGE, b"I was unable to write to the file #.  The value of IOSTAT returned was #. ");

                    spicelib::REPMC(
                        &save.MESSGE.to_vec(),
                        b"#",
                        fstr::substr(&save.FILES[save.ID], 1..=save.R),
                        &mut save.MESSGE,
                    );
                    spicelib::REPMI(
                        &save.MESSGE.to_vec(),
                        b"#",
                        save.IOSTAT,
                        &mut save.MESSGE,
                        ctx,
                    );

                    spicelib::TOSTDO(&save.MESSGE, ctx)?;

                    {
                        use f2rust_std::io;

                        let specs = io::CloseSpecs {
                            unit: Some(save.UNITS[save.ID]),
                            ..Default::default()
                        };
                        ctx.close(specs)?;
                    }

                    save.ACTIVE[save.ID] = false;
                    save.OPEN[save.ID] = false;
                    save.SUSPND[save.ID] = false;
                    fstr::assign(save.FILES.get_mut(save.ID), b" ");
                }
            }

            save.ID += m3__;
        }
    }

    Ok(())
}

//$Procedure FINISH ( Close files and inform user of their location )
pub fn FINISH(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Obtain the port ID for the SCREEN.
    //
    save.ID = spicelib::ISRCHC(b"SCREEN", NPORTS, save.PORTS.as_arg());

    if save.OPEN[LOG] {
        // If a log file exists and a screen lockout is
        // not active, output the log file information message
        // to standard-out.
        //
        if (save.ACTIVE[save.ID] && save.OPEN[save.ID]) {
            fstr::assign(&mut save.MESSGE, b"The log file was written to: ");
            spicelib::TOSTDO(
                fstr::substr(&save.MESSGE, 1..=spicelib::RTRIM(&save.MESSGE)),
                ctx,
            )?;
            spicelib::TOSTDO(
                fstr::substr(&save.FILES[LOG], 1..=spicelib::RTRIM(&save.FILES[LOG])),
                ctx,
            )?;
            spicelib::TOSTDO(b" ", ctx)?;
        }

        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.UNITS[LOG]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    }

    if save.OPEN[SPOOL] {
        //
        // If a log file exists and a screen lockout is
        // not active, output the error log file information message
        // to standard-out.
        //
        // Recall SPOOL indicates the error log file identified
        // in the tstlgr.f PORTS array as 'SAVE'.
        //
        if (save.ACTIVE[save.ID] && save.OPEN[save.ID]) {
            fstr::assign(
                &mut save.MESSGE,
                b"The list of test failures was written to: ",
            );
            spicelib::TOSTDO(
                fstr::substr(&save.MESSGE, 1..=spicelib::RTRIM(&save.MESSGE)),
                ctx,
            )?;
            spicelib::TOSTDO(
                fstr::substr(&save.FILES[SPOOL], 1..=spicelib::RTRIM(&save.FILES[SPOOL])),
                ctx,
            )?;
            spicelib::TOSTDO(b" ", ctx)?;
        }

        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.UNITS[SPOOL]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    }

    //
    // Reset the OPNERR boolean flag in tstlgr.f to TRUE. This action
    // is required for those situations where the test system does
    // not run in a single run environment (e.g. IDL, MATLAB).
    //
    ZZTSTRE(ctx);

    Ok(())
}

//$Procedure LCKOUT ( Lock out a port from use )
pub fn LCKOUT(PORT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Get the port ID.
    //
    save.ID = spicelib::ISRCHC(PORT, NPORTS, save.PORTS.as_arg());

    if (save.ID == 0) {
        spicelib::CHKIN(b"LCKOUT", ctx)?;
        spicelib::SETMSG(b"Unrecognized port: #", ctx);
        spicelib::ERRCH(b"#", PORT, ctx);
        spicelib::SIGERR(b"TESTUTIL(BUG)", ctx)?;
        spicelib::CHKOUT(b"LCKOUT", ctx)?;
        return Ok(());
    }

    //
    // Close everything.
    //
    save.ACTIVE[save.ID] = false;
    save.OPEN[save.ID] = false;
    save.SUSPND[save.ID] = false;
    save.LKPORT[save.ID] = true;

    Ok(())
}
