//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 2000;
const STYSIZ: i32 = 120;

struct SaveVars {
    MESSGE: Vec<u8>,
    NUMBER: i32,
    LOGIFP: bool,
    PASSED: bool,
    GOOD: Vec<u8>,
    BAD: Vec<u8>,
    PASS: Vec<u8>,
    FAIL: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MESSGE = vec![b' '; LNSIZE as usize];
        let mut NUMBER: i32 = 0;
        let mut LOGIFP: bool = false;
        let mut PASSED: bool = false;
        let mut GOOD = vec![b' '; STYSIZ as usize];
        let mut BAD = vec![b' '; STYSIZ as usize];
        let mut PASS = vec![b' '; STYSIZ as usize];
        let mut FAIL = vec![b' '; STYSIZ as usize];

        LOGIFP = false;

        Self {
            MESSGE,
            NUMBER,
            LOGIFP,
            PASSED,
            GOOD,
            BAD,
            PASS,
            FAIL,
        }
    }
}

//$Procedure      TSTLIP ( Test log if passed )
//
pub fn TSTLIP(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // Test Utility Functions
    //
    //
    // Local Variables.
    //

    fstr::assign(
        &mut save.PASS,
        b"LEFT 1 RIGHT 78 HARDSPACE $ FLAG Case$Passed:",
    );
    fstr::assign(
        &mut save.FAIL,
        b"LEFT 1 RIGHT 78 HARDSPACE $ FLAG CASE$FAILED!",
    );

    if (save.LOGIFP || VERBOS(ctx)) {
        fstr::assign(&mut save.MESSGE, b" ");
        //
        // Copy the current logging style.
        //
        TSTSTY(&mut save.GOOD, &mut save.BAD, ctx);
        //
        // See whether or not this case passed and get its number.
        //
        T_CPASS(&mut save.PASSED, ctx);
        T_CNUM(&mut save.NUMBER, ctx);

        if ((save.NUMBER > 0) && save.PASSED) {
            //
            // Print a blank line before the output message only
            // if we are in verbose mode.
            //
            if VERBOS(ctx) {
                TSTLOG(b" ", false, ctx)?;
            }
            //
            // Set the logging style to PASS and print the case name.
            //
            TSTLGS(&save.PASS, &save.PASS, ctx);
            T_CNAME(&mut save.MESSGE, ctx);
            TSTLOG(&save.MESSGE, false, ctx)?;
        } else if (save.NUMBER > 0) {
            //
            // Always print a blank line before a case that failed.
            //
            TSTLOG(b" ", false, ctx)?;
            //
            // Set the logging style to FAIL and print the case name.
            //
            TSTLGS(&save.FAIL, &save.FAIL, ctx);
            T_CNAME(&mut save.MESSGE, ctx);
            TSTLOG(&save.MESSGE, false, ctx)?;
        }

        TSTLGS(&save.GOOD, &save.BAD, ctx);
    }
    Ok(())
}

//
// Write a blank line if we are in case logging mode or verbose
// mode.
//
pub fn TSTCBL(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if (save.LOGIFP || VERBOS(ctx)) {
        TSTLOG(b" ", false, ctx)?;
    }
    Ok(())
}

//
// To turn on the logging of individual test cases call TSTLCY
//
pub fn TSTLCY(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.LOGIFP = true;
}

//
// To turn on the logging of individual test cases call TSTLCY
//
pub fn TSTLCN(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.LOGIFP = false;
}
