//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const STYSIZ: i32 = 128;
const LNSIZE: i32 = 80;
const WDSIZE: i32 = 32;

struct SaveVars {
    SSTYLE: Vec<u8>,
    LSTYLE: Vec<u8>,
    VSTYLE: Vec<u8>,
    HSTYLE: Vec<u8>,
    WORD: Vec<u8>,
    ON: bool,
    DUMMY: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SSTYLE = vec![b' '; STYSIZ as usize];
        let mut LSTYLE = vec![b' '; STYSIZ as usize];
        let mut VSTYLE = vec![b' '; STYSIZ as usize];
        let mut HSTYLE = vec![b' '; STYSIZ as usize];
        let mut WORD = vec![b' '; WDSIZE as usize];
        let mut ON: bool = false;
        let mut DUMMY: bool = false;

        Self {
            SSTYLE,
            LSTYLE,
            VSTYLE,
            HSTYLE,
            WORD,
            ON,
            DUMMY,
        }
    }
}

//$Procedure TSETUP ( Test utility setup )
pub fn TSETUP(
    CMLINE: &mut [u8],
    LOGNAM: &[u8],
    VERSN: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Test Utility Functions
    //

    //
    // Local Variables
    //

    //
    // The following styles are for reporting errors to the
    // screen and log file respectively.
    //
    fstr::assign(
        &mut save.SSTYLE,
        b"HARDSPACE ^ NEWLINE /cr VTAB /vt FLAG Failure: ",
    );

    fstr::assign(
        &mut save.LSTYLE,
        b"HARDSPACE ^ NEWLINE /cr VTAB /vt FLAG Failure: LEADER -- LEFT 1 RIGHT 72 ",
    );

    //
    // Set the exception handling status to OFF and set the debugging
    // status to OFF.
    //
    save.DUMMY = SETOFF(b"CHCKXC", ctx);
    save.DUMMY = SETOFF(b"DEBUGGING", ctx);

    //
    // The following styles will be used for logging of
    // commands and for commenting them out.
    //
    fstr::assign(&mut save.VSTYLE, b"LEFT 1 RIGHT 78 ");
    fstr::assign(&mut save.HSTYLE, b"LEFT 1 RIGHT 78 LEADER -- FLAG -- ");

    TSTLGS(&save.VSTYLE, &save.HSTYLE, ctx);

    while fstr::ne(CMLINE, b" ") {
        spicelib::NEXTWD(&CMLINE.to_vec(), &mut save.WORD, CMLINE);
        spicelib::UCASE(&save.WORD.to_vec(), &mut save.WORD, ctx);

        if fstr::eq(&save.WORD, b"-V") {
            save.ON = VERBON(ctx);
        } else if fstr::eq(&save.WORD, b"-C") {
            TSTLCY(ctx);
        } else if fstr::eq(&save.WORD, b"-D") {
            //
            // We want to run in debugging mode. This means
            // files are not automatically deleted when a new
            // test family is initiated.
            //
            save.ON = SETON(b"DEBUGGING", ctx);
        }

        //
        // Check for quiet mode; supress all output to the
        // standard IO device.
        //
        if fstr::eq(&save.WORD, b"-Q") {
            LCKOUT(b"SCREEN", ctx)?;
        }
    }

    //
    // Open a log file.
    //
    TSTOPL(LOGNAM, VERSN, ctx)?;

    //
    // Now, set up the SPICELIB error handling.
    //
    spicelib::ERRACT(b"SET", &mut b"RETURN".clone(), ctx)?;
    spicelib::ERRDEV(b"SET", &mut b"NULL".clone(), ctx)?;

    Ok(())
}
