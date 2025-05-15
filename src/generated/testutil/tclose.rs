//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const WDSIZE: i32 = 32;
const MSGSIZ: i32 = 256;
const FILSIZ: i32 = 255;
const STYSIZ: i32 = 120;

//$Procedure      TCLOSE (Close testing.)
pub fn TCLOSE(ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ENV = [b' '; LNSIZE as usize];
    let mut VERSN = [b' '; LNSIZE as usize];
    let mut TIME = [b' '; LNSIZE as usize];
    let mut TIME2 = [b' '; LNSIZE as usize];
    let mut NAME = [b' '; WDSIZE as usize];
    let mut MESSGE = [b' '; MSGSIZ as usize];
    let mut FILE = [b' '; FILSIZ as usize];
    let mut LOGFIL = [b' '; FILSIZ as usize];
    let mut NFAIL: i32 = 0;
    let mut NCASE: i32 = 0;
    let mut FAILUR: bool = false;
    let mut DUMMY: bool = false;
    let mut OK: bool = false;
    let mut GOOD = [b' '; STYSIZ as usize];
    let mut BAD = [b' '; STYSIZ as usize];

    //
    // Test Utility Functions
    //

    //
    // See if exceptions were examined in the last test case.
    //
    if ISOFF(b"CHCKXC", ctx) {
        CHCKXC(false, b" ", &mut OK, ctx)?;
        DUMMY = SETOFF(b"CHCKXC", ctx);
    }

    spicelib::RESET(ctx);
    spicelib::ERRDEV(b"SET", &mut b"NULL".clone(), ctx)?;

    TSTSTY(&mut GOOD, &mut BAD, ctx);

    //
    // Log the last test case if it passed.
    //
    TSTLIP(ctx)?;
    //
    // Draw any rules that might be needed by verbose output.
    //
    TSTRUL(ctx)?;
    TSTRUL(ctx)?;
    //
    // Put a blank line between the cases and our output if we are in
    // case reporting or verbose mode.
    //
    TSTCBL(ctx)?;

    //
    // We need to close out the old case
    //
    T_NAME(&mut NAME, ctx);
    T_FCOUNT(&mut NFAIL, ctx);
    T_CNUM(&mut NCASE, ctx);
    T_SUCCESS(&mut OK, ctx);

    if OK {
        fstr::assign(&mut MESSGE, b"Passed  --- Test Family: ");
        spicelib::SUFFIX(&NAME, 1, &mut MESSGE);

        TSTLOG(&MESSGE, false, ctx)?;

        TSTLGS(
            b"LEFT 9 RIGHT 78 FLAG --- NEWLINE /cr",
            b"LEFT 9 RIGHT 78 FLAG --- NEWLINE /cr",
            ctx,
        );

        fstr::assign(&mut MESSGE, b"Successful tests for # of # test cases. ");
        spicelib::REPMCT(&MESSGE.clone(), b"#", NCASE, b"L", &mut MESSGE, ctx)?;
        spicelib::REPMCT(&MESSGE.clone(), b"#", NCASE, b"L", &mut MESSGE, ctx)?;
        TSTLOG(&MESSGE, false, ctx)?;
    } else {
        fstr::assign(&mut MESSGE, b"FAILURE for Test Family: ");
        spicelib::SUFFIX(&NAME, 1, &mut MESSGE);

        TSTLOG(&MESSGE, false, ctx)?;

        TSTLGS(
            b"LEFT 24 RIGHT 78 FLAG : NEWLINE /cr",
            b"LEFT 24 RIGHT 78 FLAG : NEWLINE /cr",
            ctx,
        );
        fstr::assign(&mut MESSGE, b"# of # test cases failed. ");
        spicelib::REPMCT(&MESSGE.clone(), b"#", NFAIL, b"C", &mut MESSGE, ctx)?;
        spicelib::REPMCT(&MESSGE.clone(), b"#", NCASE, b"L", &mut MESSGE, ctx)?;
        TSTLOG(&MESSGE, false, ctx)?;
    }

    //
    // Get current time. We will print it at the very end.
    //
    support::CURTIM(&mut TIME2, ctx)?;

    TSTLGS(&GOOD, &BAD, ctx);
    //
    // Now we check on the overall success of the testing.
    //
    T_ANYBAD(&mut FAILUR, ctx);

    if !FAILUR {
        TSTGLF(&mut LOGFIL, ctx);
        TSTFIL(b"pass{0-9}{0-9}{0-9}{0-9}.log", b"SAVE", &mut FILE, ctx)?;
        TSTSLF(&LOGFIL, ctx);

        TSTGET(&mut ENV, &mut VERSN, &mut TIME, ctx);
        //
        // We temporarily turn off writing to the log file and screen
        // so that we can put the test stamp in the pass file.
        //
        TSTIOA(b"SAVE", ctx)?;
        TSTIOH(b"LOG", ctx)?;
        TSTIOH(b"SCREEN", ctx)?;

        TSTWLN(&ENV, ctx)?;
        TSTWLN(&VERSN, ctx)?;
        TSTWLN(&TIME, ctx)?;

        //
        // Now reactivate the screen and log file.
        //
        TSTIOA(b"LOG", ctx)?;
        TSTIOA(b"SCREEN", ctx)?;
        //
        // Write out the testing summary.
        //
        TSTWLN(b" ", ctx)?;
        TSTWLN(b"All tests passed.  ", ctx)?;

        TSTIOH(b"SCREEN", ctx)?;
        TSTWLN(b"Tests performed were:", ctx)?;
        TSTWLN(b" ", ctx)?;

        TBEGF(ctx);
        TFNAME(&mut NAME, ctx);

        while fstr::ne(&NAME, b" ") {
            TSTWLN(&NAME, ctx)?;
            TFNAME(&mut NAME, ctx);
        }
        //
        // Turn off writing to the log file,  the next message doesn't
        // belong in it.
        //
        TSTGLF(&mut MESSGE, ctx);
        spicelib::PREFIX(b"For details, see the test log:", 1, &mut MESSGE);

        TSTIOH(b"LOG", ctx)?;
        TSTWLN(b" ", ctx)?;
        TSTLOG(&MESSGE, false, ctx)?;
        TSTWLN(b" ", ctx)?;

        TSTLOG(&TIME2, false, ctx)?;
        TSTWLN(b" ", ctx)?;

        //
        // Close the pass file.
        //
        TSTIOC(b"SAVE", ctx)?;
        //
        // Reactivate writing to the log file.
        //
        TSTIOA(b"LOG", ctx)?;
        TSTIOA(b"SCREEN", ctx)?;
    }

    //
    // Clean up any files that might be lying around from the
    // last test family.
    //
    CSTART(ctx)?;

    //
    // Finish up everything now.
    TSTLOG(b" ", false, ctx)?;
    TSTLOG(&TIME2, false, ctx)?;
    TSTLOG(b" ", false, ctx)?;

    FINISH(ctx)?;

    Ok(())
}
