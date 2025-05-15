//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const EFILE: i32 = 5;
const LOG: i32 = 2;
const NPORTS: i32 = 8;
const SCREEN: i32 = 6;
const SIZFIL: i32 = 255;
const SIZMSG: i32 = 400;
const SIZWRD: i32 = 32;
const SPOOL: i32 = 4;
const STDOUT: i32 = 1;

struct SaveVars {
    FILES: ActualCharArray,
    MESSGE: Vec<u8>,
    PORTS: ActualCharArray,
    ID: i32,
    R: i32,
    TO: i32,
    UNITS: StackArray<i32, 8>,
    ACTIVE: StackArray<bool, 8>,
    ERROPF: bool,
    OPEN: StackArray<bool, 8>,
    OPENOK: bool,
    SUSPND: StackArray<bool, 8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FILES = ActualCharArray::new(SIZFIL, 1..=NPORTS);
        let mut MESSGE = vec![b' '; SIZMSG as usize];
        let mut PORTS = ActualCharArray::new(SIZWRD, 1..=NPORTS);
        let mut ID: i32 = 0;
        let mut R: i32 = 0;
        let mut TO: i32 = 0;
        let mut UNITS = StackArray::<i32, 8>::new(1..=NPORTS);
        let mut ACTIVE = StackArray::<bool, 8>::new(1..=NPORTS);
        let mut ERROPF: bool = false;
        let mut OPEN = StackArray::<bool, 8>::new(1..=NPORTS);
        let mut OPENOK: bool = false;
        let mut SUSPND = StackArray::<bool, 8>::new(1..=NPORTS);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"SCREEN"),
                Val::C(b"LOG"),
                Val::C(b"SAVE"),
                Val::C(b"UTILITY"),
                Val::C(b"ERROR"),
                Val::C(b"AUX1"),
                Val::C(b"AUX2"),
                Val::C(b"AUX3"),
            ]
            .into_iter();
            PORTS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b" "),
                Val::C(b" "),
                Val::C(b" "),
                Val::C(b" "),
                Val::C(b" "),
                Val::C(b" "),
                Val::C(b" "),
                Val::C(b" "),
            ]
            .into_iter();
            FILES
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(SCREEN),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
            ]
            .into_iter();
            UNITS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
            ]
            .into_iter();
            ACTIVE
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::L(true),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
            ]
            .into_iter();
            OPEN.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
                Val::L(false),
            ]
            .into_iter();
            SUSPND
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        ERROPF = false;

        Self {
            FILES,
            MESSGE,
            PORTS,
            ID,
            R,
            TO,
            UNITS,
            ACTIVE,
            ERROPF,
            OPEN,
            OPENOK,
            SUSPND,
        }
    }
}

//$Procedure NSPIO (Inspekt I/O Manager)
pub fn NSPIO(
    LINE: &[u8],
    PORT: &[u8],
    NAME: &[u8],
    STATUS: &[bool],
    OK: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB Functions
    //

    //
    // Other Functions
    //

    //
    // Local Parameters
    //
    //
    // Error File Port Integer Code.
    //
    //
    // Log File Port Integer Code.
    //

    //
    // The number of total ports supported by this version of NSPIO.
    //

    //
    // The logical unit that is associated with STDOUT.
    //

    //
    // The maximum filename string length.
    //

    //
    // The maximum length of a message.
    //

    //
    // The maximum length of a word.
    //

    //
    // Spool Port Integer Code.
    //

    //
    // Screen Port Integer Code.
    //

    //
    // Local Variables
    //

    //
    // Save all local variables.
    //

    //
    // Initialize the PORT configuration arrays.
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPIO", ctx)?;
        spicelib::SIGERR(b"NSPIO(BOGUSENTRY)", ctx)?;
        spicelib::CHKOUT(b"NSPIO", ctx)?;
    }

    Ok(())
}

//$Procedure NSPOPN ( Inspekt I/O Manager -- Open Port )
pub fn NSPOPN(PORT: &[u8], NAME: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPOPN", ctx)?;
    }

    //
    // Find the integer associated with PORT.
    //
    save.ID = ZZNSPPOK(PORT, NPORTS, save.PORTS.as_arg(), ctx)?;

    //
    // See if an error has been signaled. If so, do nothing
    // further and return.
    //
    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"NSPOPN", ctx)?;
        return Ok(());
    }

    //
    // First check to see whether we are dealing with the SCREEN
    // port.  If we are return, do nothing and return.
    //
    if (save.ID == STDOUT) {
        spicelib::CHKOUT(b"NSPOPN", ctx)?;
        return Ok(());
    }

    //
    // Now at this point we have a request to open a file based
    // port.  Check first to see if it is already open.
    //
    if save.OPEN[save.ID] {
        //
        // If the file attached to PORT is already open, close it
        // before attaching this new file to it.
        //
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(save.UNITS[save.ID]),
                ..Default::default()
            };
            ctx.close(specs)?;
        }

        //
        // Now reset PORT's status.
        //
        save.ACTIVE[save.ID] = false;
        save.OPEN[save.ID] = false;
        save.SUSPND[save.ID] = false;
        fstr::assign(save.FILES.get_mut(save.ID), b" ");
    }

    //
    // Check to see if we are opening the ERROR port.  We treat
    // this port differently from the other file based ports, since
    // if an error occurs opening the file, no error is signaled.
    // The port is simply not opened.
    //
    if (save.ID == EFILE) {
        //
        // Assume there is will be no error opening the file.
        //
        save.ERROPF = false;

        //
        // Attempt to open the file.
        //
        save.R = spicelib::RTRIM(NAME);
        ZZTXTOPN(
            fstr::substr(NAME, 1..=save.R),
            &mut save.UNITS[save.ID],
            &mut save.OPENOK,
            ctx,
        )?;

        //
        // If the OPEN process failed, then clear the status of the
        // port and return.
        //
        if !save.OPENOK {
            save.ACTIVE[save.ID] = false;
            save.OPEN[save.ID] = false;
            save.SUSPND[save.ID] = false;

            //
            // Leave FILES(ID) set, so that the name of the file can
            // be reported.
            //

            fstr::assign(save.FILES.get_mut(save.ID), NAME);

            //
            // Before returning, set ERROPF to .TRUE., since
            // this will facilitate the creation of the warning
            // message when NSPEND is invoked.
            //
            save.ERROPF = true;

            spicelib::CHKOUT(b"NSPOPN", ctx)?;
            return Ok(());
        }

    //
    // Consider all other file based ports.  For these ports we will
    // signal errors if TXTOPN is incapable of opening the file.
    //
    } else {
        //
        // Open the new file.
        //
        save.R = spicelib::RTRIM(NAME);
        spicelib::TXTOPN(
            fstr::substr(NAME, 1..=save.R),
            &mut save.UNITS[save.ID],
            ctx,
        )?;

        //
        // Check FAILED(). If an error has occurred, clear PORT status,
        // check out and return.
        //
        if spicelib::FAILED(ctx) {
            save.ACTIVE[save.ID] = false;
            save.OPEN[save.ID] = false;
            save.SUSPND[save.ID] = false;
            fstr::assign(save.FILES.get_mut(save.ID), b" ");
            spicelib::CHKOUT(b"NSPOPN", ctx)?;
            return Ok(());
        }
    }

    //
    // If we made it this far, then the file was opened successfully.
    // Set PORT status to reflect successful open.
    //
    save.ACTIVE[save.ID] = true;
    save.OPEN[save.ID] = true;
    save.SUSPND[save.ID] = false;
    fstr::assign(save.FILES.get_mut(save.ID), NAME);

    spicelib::CHKOUT(b"NSPOPN", ctx)?;
    Ok(())
}

//$Procedure NSPIOH ( Inspekt I/O Manager -- Inhibit Port )
pub fn NSPIOH(PORT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPIOH", ctx)?;
    }

    //
    // Find the integer associated with PORT.
    //
    save.ID = ZZNSPPOK(PORT, NPORTS, save.PORTS.as_arg(), ctx)?;

    //
    // Inhibit I/O to the port, if no error was signaled. Note - if
    // the port is already inhibited, then this does not change it's
    // state.
    //
    if !spicelib::FAILED(ctx) {
        save.ACTIVE[save.ID] = false;
    }

    spicelib::CHKOUT(b"NSPIOH", ctx)?;
    Ok(())
}

//$Procedure NSPIOA ( Inspekt I/O Manager -- Activate Port )
pub fn NSPIOA(PORT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPIOA", ctx)?;
    }

    //
    // Find the integer associated with PORT.
    //
    save.ID = ZZNSPPOK(PORT, NPORTS, save.PORTS.as_arg(), ctx)?;

    //
    // Activate the port, if no error was signaled. Note - if PORT was
    // already activated, then it will remain activated.
    //
    if !spicelib::FAILED(ctx) {
        save.ACTIVE[save.ID] = true;
    }

    spicelib::CHKOUT(b"NSPIOA", ctx)?;
    Ok(())
}

//$Procedure NSPGST ( Inspekt I/O Manager -- Get Port Status )
pub fn NSPGST(PORT: &[u8], STATUS: &mut [bool], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut STATUS = DummyArrayMut::new(STATUS, 1..=3);

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPGST", ctx)?;
    }

    //
    // Find the integer associated with PORT.
    //
    save.ID = ZZNSPPOK(PORT, NPORTS, save.PORTS.as_arg(), ctx)?;

    //
    // Return the status of the port if no error was signaled.
    //
    if !spicelib::FAILED(ctx) {
        STATUS[1] = save.ACTIVE[save.ID];
        STATUS[2] = save.OPEN[save.ID];
        STATUS[3] = save.SUSPND[save.ID];
    }

    spicelib::CHKOUT(b"NSPGST", ctx)?;
    Ok(())
}

//$Procedure NSPPST ( Inspekt I/O Manager -- Put Port Status )
pub fn NSPPST(PORT: &[u8], STATUS: &[bool], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let STATUS = DummyArray::new(STATUS, 1..=3);

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPPST", ctx)?;
    }

    //
    // Find the integer associated with PORT.
    //
    save.ID = ZZNSPPOK(PORT, NPORTS, save.PORTS.as_arg(), ctx)?;

    //
    // Set the status of the port if no error was signaled.
    //
    if !spicelib::FAILED(ctx) {
        save.ACTIVE[save.ID] = STATUS[1];
        save.OPEN[save.ID] = STATUS[2];
        save.SUSPND[save.ID] = STATUS[3];
    }

    spicelib::CHKOUT(b"NSPPST", ctx)?;
    Ok(())
}

//$Procedure NSPIOC ( Inspekt I/O Manager -- Close Port )
pub fn NSPIOC(PORT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPIOC", ctx)?;
    }

    //
    // Find the integer associated with PORT.
    //
    save.ID = ZZNSPPOK(PORT, NPORTS, save.PORTS.as_arg(), ctx)?;

    //
    // Check FAILED() to see if an error was signaled, or if
    // ID refers to the SCREEN port.  In either case, return without
    // doing anything.
    //
    if (spicelib::FAILED(ctx) || (save.ID == STDOUT)) {
        spicelib::CHKOUT(b"NSPIOC", ctx)?;
        return Ok(());
    }

    //
    // Now check to see if the port is currently closed or if the
    // requested port to close is the SCREEN port.
    //
    if (!save.OPEN[save.ID] || (save.ID == STDOUT)) {
        spicelib::CHKOUT(b"NSPIOC", ctx)?;
        return Ok(());
    }

    //
    // If we make it this far, then we were given an open file
    // based port.  Close the port and reset its status.
    //
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

    //
    // If we have closed the error file, then clear ERROPF.
    //
    if (save.ID == EFILE) {
        save.ERROPF = false;
    }

    spicelib::CHKOUT(b"NSPIOC", ctx)?;
    Ok(())
}

//$Procedure NSPIOS ( Inspekt I/O Manager -- Suspend Port )
pub fn NSPIOS(PORT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPIOS", ctx)?;
    }

    //
    // Find the integer associated with PORT.
    //
    save.ID = ZZNSPPOK(PORT, NPORTS, save.PORTS.as_arg(), ctx)?;

    //
    // Suspend I/O on the port, if no error was signaled. Note - if
    // PORT was already suspended, then it will remain suspended.
    //
    if !spicelib::FAILED(ctx) {
        //
        // Suspend I/O on PORT.
        //
        save.SUSPND[save.ID] = true;
    }

    spicelib::CHKOUT(b"NSPIOS", ctx)?;
    Ok(())
}

//$Procedure NSPIOR ( Inspekt I/O Manager -- Reopen Port )
pub fn NSPIOR(PORT: &[u8], OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPIOR", ctx)?;
    }

    //
    // Find the integer associated with PORT.
    //
    save.ID = ZZNSPPOK(PORT, NPORTS, save.PORTS.as_arg(), ctx)?;

    //
    // See if an error has been signaled. If so, do nothing
    // further and return.
    //
    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"NSPIOR", ctx)?;
        return Ok(());
    }

    //
    // Check to see if PORT is currently suspended.
    //
    if !save.SUSPND[save.ID] {
        //
        // If it's not, then set OK to .FALSE. and return
        //
        *OK = false;
        spicelib::CHKOUT(b"NSPIOR", ctx)?;
        return Ok(());
    }

    //
    // Suspend I/O to this port.
    //
    save.SUSPND[save.ID] = false;

    spicelib::CHKOUT(b"NSPIOR", ctx)?;
    Ok(())
}

//$Procedure NSPWLN ( Inspekt I/O Manager -- Write Line )
pub fn NSPWLN(LINE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPWLN", ctx)?;
    }

    //
    // Write to all the open, active, and non-suspended ports.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NPORTS;
        let m3__: i32 = 1;
        save.ID = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if ((!save.SUSPND[save.ID] && save.ACTIVE[save.ID]) && save.OPEN[save.ID]) {
                //
                // Write the line to this port.
                //
                save.TO = save.UNITS[save.ID];
                spicelib::WRITLN(LINE, save.TO, ctx)?;

                //
                // Check for and process any errors.
                //
                if ((save.ID != STDOUT) && spicelib::FAILED(ctx)) {
                    //
                    // If we have encountered an error then close the
                    // file and reset the port status.  Note we do not
                    // need to reset error status to continue, since
                    // WRITLN does not check RETURN().
                    //
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

    spicelib::CHKOUT(b"NSPWLN", ctx)?;
    Ok(())
}

//$Procedure NSPEND ( Inspekt I/O Manager -- Finished with I/O )
pub fn NSPEND(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPEND", ctx)?;
    }

    //
    // If the LOG port is open, then notify the user about it's
    // location, and close it.
    //
    if save.OPEN[LOG] {
        TRNLAT(b"LOGFILWRITTENTO", &mut save.MESSGE, ctx);

        if ((!save.SUSPND[STDOUT] && save.ACTIVE[STDOUT]) && save.OPEN[STDOUT]) {
            //
            // Write the message.
            //
            spicelib::WRITLN(b" ", SCREEN, ctx)?;

            save.R = spicelib::RTRIM(&save.MESSGE);
            spicelib::WRITLN(fstr::substr(&save.MESSGE, 1..=save.R), SCREEN, ctx)?;

            save.R = spicelib::RTRIM(&save.FILES[LOG]);
            spicelib::WRITLN(fstr::substr(&save.FILES[LOG], 1..=save.R), SCREEN, ctx)?;
        }
    }

    //
    // If the SAVE port is open, then notify the user about it's
    // location, and close it.
    //
    if save.OPEN[SPOOL] {
        TRNLAT(b"SAVFILWRITTENTO", &mut save.MESSGE, ctx);

        if ((!save.SUSPND[STDOUT] && save.ACTIVE[STDOUT]) && save.OPEN[STDOUT]) {
            //
            // Write the message.
            //
            spicelib::WRITLN(b" ", SCREEN, ctx)?;

            save.R = spicelib::RTRIM(&save.MESSGE);
            spicelib::WRITLN(fstr::substr(&save.MESSGE, 1..=save.R), SCREEN, ctx)?;

            save.R = spicelib::RTRIM(&save.FILES[SPOOL]);
            spicelib::WRITLN(fstr::substr(&save.FILES[SPOOL], 1..=save.R), SCREEN, ctx)?;
        }
    }

    //
    // If the ERROR port is open, then notify the user about it's
    // location, and close it.
    //
    if save.OPEN[EFILE] {
        TRNLAT(b"ERRFILWRITTENTO", &mut save.MESSGE, ctx);

        if ((!save.SUSPND[STDOUT] && save.ACTIVE[STDOUT]) && save.OPEN[STDOUT]) {
            //
            // Write the message.
            //
            spicelib::WRITLN(b" ", SCREEN, ctx)?;

            save.R = spicelib::RTRIM(&save.MESSGE);
            spicelib::WRITLN(fstr::substr(&save.MESSGE, 1..=save.R), SCREEN, ctx)?;

            save.R = spicelib::RTRIM(&save.FILES[EFILE]);
            spicelib::WRITLN(fstr::substr(&save.FILES[EFILE], 1..=save.R), SCREEN, ctx)?;
        }
    } else if save.ERROPF {
        TRNLAT(b"ERRFILWRITEFAIL", &mut save.MESSGE, ctx);

        if ((!save.SUSPND[STDOUT] && save.ACTIVE[STDOUT]) && save.OPEN[STDOUT]) {
            //
            // Write the message.
            //
            spicelib::WRITLN(b" ", SCREEN, ctx)?;

            save.R = spicelib::RTRIM(&save.MESSGE);
            spicelib::WRITLN(fstr::substr(&save.MESSGE, 1..=save.R), SCREEN, ctx)?;

            save.R = spicelib::RTRIM(&save.FILES[EFILE]);
            spicelib::WRITLN(fstr::substr(&save.FILES[EFILE], 1..=save.R), SCREEN, ctx)?;
        }
    }

    //
    // Close all ports and restore NSPIO status to it's uninitialized
    // state.  First handle the screen port, since it's an exception.
    //
    save.ACTIVE[STDOUT] = true;
    save.OPEN[STDOUT] = true;
    save.SUSPND[STDOUT] = false;

    //
    // Now reset the file based ports.
    //
    {
        let m1__: i32 = 2;
        let m2__: i32 = NPORTS;
        let m3__: i32 = 1;
        save.ID = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Close the file associated with the port if it's open.
            //
            if save.OPEN[save.ID] {
                {
                    use f2rust_std::io;

                    let specs = io::CloseSpecs {
                        unit: Some(save.UNITS[save.ID]),
                        ..Default::default()
                    };
                    ctx.close(specs)?;
                }
            }
            //
            // Restore original port status.
            //
            save.UNITS[save.ID] = 0;
            fstr::assign(save.FILES.get_mut(save.ID), b" ");
            save.ACTIVE[save.ID] = false;
            save.OPEN[save.ID] = false;
            save.SUSPND[save.ID] = false;

            save.ID += m3__;
        }
    }

    spicelib::CHKOUT(b"NSPEND", ctx)?;
    Ok(())
}

//$Procedure NSPPFL ( Inspekt I/O Manager -- Fetch file name )
pub fn NSPPFL(PORT: &[u8], NAME: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NSPPFL", ctx)?;
    }

    //
    // Find the integer associated with PORT.
    //
    save.ID = ZZNSPPOK(PORT, NPORTS, save.PORTS.as_arg(), ctx)?;

    //
    // See if an error has been signaled. If so, clear NAME
    // and return.
    //
    if spicelib::FAILED(ctx) {
        fstr::assign(NAME, b" ");
        spicelib::CHKOUT(b"NSPPFL", ctx)?;
        return Ok(());

    //
    // If the ID refers to an active, open, non-suspended port, then
    // set NAME to the name of the file.  Note: in the case when PORT
    // is 'SCREEN', the corresponding entry in the FILES array is ' '.
    //
    } else if ((!save.SUSPND[save.ID] && save.ACTIVE[save.ID]) && save.OPEN[save.ID]) {
        fstr::assign(NAME, save.FILES.get(save.ID));

    //
    // If PORT is inactive, suspended or closed, set NAME to ' '.
    //
    } else {
        fstr::assign(NAME, b" ");
    }

    spicelib::CHKOUT(b"NSPPFL", ctx)?;
    Ok(())
}
