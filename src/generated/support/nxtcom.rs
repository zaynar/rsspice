//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXCOM: i32 = 20;
const FILEN: i32 = 128;
const COMSIZ: i32 = 1024;
const ERRSIZ: i32 = 1760;
const STYSIZ: i32 = 120;
const NONE: i32 = 0;
const COMBUF: i32 = (NONE + 1);
const KEYBRD: i32 = (COMBUF + 1);
const INPFIL: i32 = (KEYBRD + 1);
const WDSIZE: i32 = 32;

struct SaveVars {
    SAVDLM: Vec<u8>,
    SAVPMT: Vec<u8>,
    WORD: Vec<u8>,
    ERROR: Vec<u8>,
    LNGMSG: Vec<u8>,
    BUFFER: ActualCharArray,
    FILE: Vec<u8>,
    REST: Vec<u8>,
    EXIT: Vec<u8>,
    SHTMSG: Vec<u8>,
    START: Vec<u8>,
    STOP: Vec<u8>,
    BUFFED: i32,
    BUFSRC: StackArray<i32, 20>,
    FIRST: bool,
    NOCOM: bool,
    READNG: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SAVDLM = vec![b' '; 1 as usize];
        let mut SAVPMT = vec![b' '; 80 as usize];
        let mut WORD = vec![b' '; 80 as usize];
        let mut ERROR = vec![b' '; 300 as usize];
        let mut LNGMSG = vec![b' '; 300 as usize];
        let mut BUFFER = ActualCharArray::new(COMSIZ, 1..=MAXCOM);
        let mut FILE = vec![b' '; FILEN as usize];
        let mut REST = vec![b' '; FILEN as usize];
        let mut EXIT = vec![b' '; WDSIZE as usize];
        let mut SHTMSG = vec![b' '; WDSIZE as usize];
        let mut START = vec![b' '; WDSIZE as usize];
        let mut STOP = vec![b' '; WDSIZE as usize];
        let mut BUFFED: i32 = 0;
        let mut BUFSRC = StackArray::<i32, 20>::new(1..=MAXCOM);
        let mut FIRST: bool = false;
        let mut NOCOM: bool = false;
        let mut READNG: bool = false;

        BUFFED = 0;
        FIRST = true;
        READNG = false;
        fstr::assign(&mut SAVDLM, b";");
        fstr::assign(&mut SAVPMT, b" ");

        Self {
            SAVDLM,
            SAVPMT,
            WORD,
            ERROR,
            LNGMSG,
            BUFFER,
            FILE,
            REST,
            EXIT,
            SHTMSG,
            START,
            STOP,
            BUFFED,
            BUFSRC,
            FIRST,
            NOCOM,
            READNG,
        }
    }
}

//$Procedure      NXTCOM ( Next command )
pub fn NXTCOM(
    PROMPT: &[u8],
    DELIM: &[u8],
    COMMND: &[u8],
    SOURCE: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"NXTCOM", ctx)?;
    }

    spicelib::SETMSG(b"NXTCOM: You have called an entry which performs no run-time function. This may indicate a bug. Please check the documentation for the subroutine NXTCOM.", ctx);

    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    spicelib::CHKOUT(b"NXTCOM", ctx)?;
    Ok(())
}

pub fn GETCOM(COMMND: &mut [u8], SOURCE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"GETCOM", ctx)?;
    }

    if save.FIRST {
        TRNLAT(b"STOP", &mut save.STOP, ctx);
        TRNLAT(b"EXIT", &mut save.EXIT, ctx);
        TRNLAT(b"START", &mut save.START, ctx);
        TRNLAT(b"DEFPROMPT", &mut save.SAVPMT, ctx);

        save.FIRST = false;
    }

    //
    // While we don't have a command, try to get one.  We look
    // in the command buffer first.
    //

    //
    // We don't have a command yet.
    //
    save.NOCOM = true;

    while save.NOCOM {
        if (save.BUFFED > 0) {
            fstr::assign(COMMND, save.BUFFER.get(save.BUFFED));
            *SOURCE = save.BUFSRC[save.BUFFED];
            save.BUFFED = (save.BUFFED - 1);
        } else {
            //
            // If we're already reading from a file then just let PRREAD
            // take care of obtaining the command. If PRREAD reaches the
            // end of the current file, the previous file is popped off
            // the stack, and the next command from this file is read
            // instead. (If no files remain to be read, DELIM is returned.)
            // In that case we are no longer reading from files.
            //
            if save.READNG {
                PRREAD(&save.SAVDLM, COMMND, ctx)?;

                *SOURCE = INPFIL;

                if fstr::eq(COMMND, &save.SAVDLM) {
                    save.READNG = false;
                }
            }

            //
            // If we're not reading from a file, get the command from the
            // keyboard. ( If the command was terminated by a blank line,
            // the command is returned as a blank. )
            //
            if !save.READNG {
                if BATCH(ctx) {
                    fstr::assign(COMMND, &save.EXIT);
                } else {
                    RDSTMT(&save.SAVPMT, &save.SAVDLM, COMMND, ctx)?;
                }

                *SOURCE = KEYBRD;
            }
        }

        //
        // We must have a command at this point.
        //
        save.NOCOM = false;

        //
        // We need to check to see if what we have is a control word.
        //
        spicelib::NEXTWD(COMMND, &mut save.WORD, &mut save.REST);
        spicelib::UCASE(&save.WORD.to_vec(), &mut save.WORD, ctx);

        //
        // If the control word is 'START', we know that we will be
        // reading from a file. Let PRSTRT take care of keeping track of
        // the files being read from. If there's a problem in PRSTRT we
        // need to signal an error here due to PRSTRT's error handling.
        // Bail out if there's a problem. If all goes well in PRSTR,
        // we will read the first command in the file the next pass
        // through the DO LOOP.
        //

        if fstr::eq(&save.WORD, &save.START) {
            //
            // We need to log this command commented out so that anyone
            // using the resulting log file, will not have to worry
            // about starting a file twice.
            //
            NSPLOG(COMMND, true, ctx)?;

            fstr::assign(&mut save.FILE, b" ");
            spicelib::NEXTWD(&save.REST.to_vec(), &mut save.FILE, &mut save.REST);

            if fstr::eq(&save.FILE, b" ") {
                *SOURCE = NONE;

                TRNLAT(b"MISSINGFILELONG", &mut save.LNGMSG, ctx);
                TRNLAT(b"MISSINGFILESHORT", &mut save.SHTMSG, ctx);
                spicelib::SETMSG(&save.LNGMSG, ctx);
                spicelib::SIGERR(&save.SHTMSG, ctx)?;
                spicelib::CHKOUT(b"GETCOM", ctx)?;
                return Ok(());
            }

            PRSTRT(&save.FILE, &mut save.ERROR, ctx)?;

            //
            // If an error occurs in PRSTRT we're in trouble. Signal an
            // error and bail. If there's no problem, we're now reading
            // from a file.
            //
            if fstr::ne(&save.ERROR, b" ") {
                *SOURCE = NONE;

                TRNLAT(b"MISSINGFILESHORT", &mut save.SHTMSG, ctx);
                spicelib::SETMSG(&save.ERROR, ctx);
                spicelib::SIGERR(&save.SHTMSG, ctx)?;
                spicelib::CHKOUT(b"GETCOM", ctx)?;
                return Ok(());
            } else {
                save.READNG = true;
                save.NOCOM = true;
            }

        //
        // If the control word is 'STOP', clear the stack of files.
        // If we were reading commands from files, we won't be anymore.
        // If we were reading commands from the keyboard, the command to
        // return is 'STOP'.
        //
        } else if fstr::eq(&save.WORD, &save.STOP) {
            if save.READNG {
                PRCLR(ctx)?;
                NSPLOG(COMMND, true, ctx)?;
                save.READNG = false;
                save.NOCOM = true;
            } else {
                fstr::assign(COMMND, &save.WORD);
            }

        //
        // If the control word is 'EXIT', and we're reading from a file,
        // we need to remove that file from the stack. If we're reading
        // commands from the keyboard, we'll return the command 'EXIT'.
        //
        } else if fstr::eq(&save.WORD, &save.EXIT) {
            if save.READNG {
                PREXIT(ctx)?;
                NSPLOG(COMMND, true, ctx)?;
                save.NOCOM = true;
            } else {
                fstr::assign(COMMND, &save.WORD);
            }
        }
    }

    spicelib::CHKOUT(b"GETCOM", ctx)?;
    Ok(())
}

//$Procedure SETDAP ( Set the delimeter and prompt values )
pub fn SETDAP(DELIM: &[u8], PROMPT: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"SETDAP", ctx)?;
    }

    //
    // Set the values of the delimeter and prompt.
    //
    fstr::assign(&mut save.SAVDLM, DELIM);
    fstr::assign(&mut save.SAVPMT, PROMPT);

    TRNLAT(b"STOP", &mut save.STOP, ctx);
    TRNLAT(b"EXIT", &mut save.EXIT, ctx);
    TRNLAT(b"START", &mut save.START, ctx);

    if fstr::eq(&save.SAVPMT, b" ") {
        TRNLAT(b"DEFPROMPT", &mut save.SAVPMT, ctx);
    }

    save.FIRST = false;

    spicelib::CHKOUT(b"SETDAP", ctx)?;
    Ok(())
}

//
pub fn PUTCOM(COMMND: &[u8], SOURCE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if (save.BUFFED < MAXCOM) {
        save.BUFFED = (save.BUFFED + 1);
        fstr::assign(save.BUFFER.get_mut(save.BUFFED), COMMND);
        save.BUFSRC[save.BUFFED] = spicelib::BRCKTI(SOURCE, NONE, INPFIL);
        return Ok(());
    }
    spicelib::CHKIN(b"PUTCOM", ctx)?;
    TRNLAT(b"COMBUFFULLLNG", &mut save.LNGMSG, ctx);
    TRNLAT(b"COMBUFFULLSHT", &mut save.SHTMSG, ctx);
    spicelib::SETMSG(&save.LNGMSG, ctx);
    spicelib::SIGERR(&save.SHTMSG, ctx)?;
    spicelib::CHKOUT(b"PUTCOM", ctx)?;
    Ok(())
}
