//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 100;
const LNGSTR: i32 = 1200;
const STYSIZ: i32 = 120;

struct SaveVars {
    MESSGE: Vec<u8>,
    CTRACE: Vec<u8>,
    OTRACE: Vec<u8>,
    GOOD: Vec<u8>,
    BAD: Vec<u8>,
    DOING: Vec<u8>,
    OK: bool,
    DUMMY: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MESSGE = vec![b' '; LNSIZE as usize];
        let mut CTRACE = vec![b' '; LNGSTR as usize];
        let mut OTRACE = vec![b' '; LNGSTR as usize];
        let mut GOOD = vec![b' '; STYSIZ as usize];
        let mut BAD = vec![b' '; STYSIZ as usize];
        let mut DOING = vec![b' '; STYSIZ as usize];
        let mut OK: bool = false;
        let mut DUMMY: bool = false;

        Self {
            MESSGE,
            CTRACE,
            OTRACE,
            GOOD,
            BAD,
            DOING,
            OK,
            DUMMY,
        }
    }
}

//$Procedure      TCASE (Test Case)
pub fn TCASE(TITLE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Take care of resetting the error handling and logging the
    // last test case if it passed.
    //
    //
    // Test Utility Functions
    //

    //
    // Local Variables
    //

    //
    // See if exceptions were examined in the last test case.
    //
    if ISOFF(b"CHCKXC", ctx) {
        CHCKXC(false, b" ", &mut save.OK, ctx)?;
        save.DUMMY = SETOFF(b"CHCKXC", ctx);
    }
    //
    // Any message created with TSTMSG is no longer relevent.  Disable
    // any such message.
    //
    TSTMOF(ctx);

    //
    // Check the current traceback to make sure things that
    // checked in checked out in the last test case.
    //
    spicelib::QCKTRC(&mut save.CTRACE, ctx);
    T_TRACE(b"GET", &mut save.OTRACE, ctx);

    CHCKSC(
        b"Current Trace",
        &save.CTRACE,
        b"=",
        &save.OTRACE,
        &mut save.OK,
        ctx,
    )?;

    TSTLIP(ctx)?;
    T_CASE(TITLE, ctx);
    TSTRUL(ctx)?;
    TSTRUL(ctx)?;

    if VERBOS(ctx) {
        //
        // Copy the current logging style.
        //
        TSTSTY(&mut save.GOOD, &mut save.BAD, ctx);

        fstr::assign(
            &mut save.DOING,
            b"LEFT 1 RIGHT 78 HARDSPACE $ FLAG Performing$:",
        );
        fstr::assign(&mut save.MESSGE, b"  ");

        TSTLGS(&save.DOING, &save.DOING, ctx);
        T_CNAME(&mut save.MESSGE, ctx);
        TSTLOG(&save.MESSGE, false, ctx)?;

        //
        // Reset the logging style back to what it was.
        //
        TSTLGS(&save.GOOD, &save.BAD, ctx);
    }

    T_TRACE(b"SET", &mut save.CTRACE, ctx);
    spicelib::RESET(ctx);
    spicelib::ERRDEV(b"SET", &mut b"NULL".clone(), ctx)?;

    Ok(())
}
