//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXFNM: i32 = 80;
const LNSIZE: i32 = 160;
const MAXTST: i32 = 1000;
const LNGSTR: i32 = 1200;
const STYSIZ: i32 = 120;

struct SaveVars {
    OLDNAM: Vec<u8>,
    MESSGE: Vec<u8>,
    TNAMES: ActualCharArray,
    CTRACE: Vec<u8>,
    OTRACE: Vec<u8>,
    NFAIL: i32,
    NCASE: i32,
    NTEST: i32,
    R: i32,
    TFAMC: i32,
    FIRST: bool,
    OK: bool,
    DUMMY: bool,
    GOOD: Vec<u8>,
    BAD: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut OLDNAM = vec![b' '; MAXFNM as usize];
        let mut MESSGE = vec![b' '; LNSIZE as usize];
        let mut TNAMES = ActualCharArray::new(MAXFNM, 1..=MAXTST);
        let mut CTRACE = vec![b' '; LNGSTR as usize];
        let mut OTRACE = vec![b' '; LNGSTR as usize];
        let mut NFAIL: i32 = 0;
        let mut NCASE: i32 = 0;
        let mut NTEST: i32 = 0;
        let mut R: i32 = 0;
        let mut TFAMC: i32 = 0;
        let mut FIRST: bool = false;
        let mut OK: bool = false;
        let mut DUMMY: bool = false;
        let mut GOOD = vec![b' '; STYSIZ as usize];
        let mut BAD = vec![b' '; STYSIZ as usize];

        TFAMC = 1;
        NTEST = 0;
        fstr::assign(&mut OLDNAM, b" ");
        FIRST = true;

        Self {
            OLDNAM,
            MESSGE,
            TNAMES,
            CTRACE,
            OTRACE,
            NFAIL,
            NCASE,
            NTEST,
            R,
            TFAMC,
            FIRST,
            OK,
            DUMMY,
            GOOD,
            BAD,
        }
    }
}

//$Procedure      TOPEN (Open a family of tests)
pub fn TOPEN(NAME: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Spicelib Functions
    //

    //
    // Local Variables and Parameters
    //

    if save.FIRST {
        save.FIRST = false;

        for I in 1..=MAXTST {
            fstr::assign(save.TNAMES.get_mut(I), b" ");
        }
    } else {
        //
        // See if exceptions were examined in the last test case.
        //
        if ISOFF(b"CHCKXC", ctx) {
            CHCKXC(false, b" ", &mut save.OK, ctx)?;
            save.DUMMY = SETOFF(b"CHCKXC", ctx);
        }
    }
    //
    // Turn off any messages that may have been left lying around from
    // the last call to TSTMSG.
    //
    TSTMOF(ctx);

    //
    // Append this name to the list of names we maintain for
    // summary purposes.
    //
    if (save.NTEST < MAXTST) {
        save.NTEST = (save.NTEST + 1);
        fstr::assign(save.TNAMES.get_mut(save.NTEST), NAME);
    }

    //
    // Get the current value of the trace.
    //
    fstr::assign(&mut save.CTRACE, b" ");
    spicelib::QCKTRC(&mut save.CTRACE, ctx);
    //
    // Get the last stored value of the trace.
    //
    T_TRACE(b"GET", &mut save.OTRACE, ctx);
    //
    // Reset the error handling
    //
    spicelib::RESET(ctx);
    spicelib::ERRDEV(b"SET", &mut b"NULL".clone(), ctx)?;
    //
    // Reset the kernel managers to a "clean" state.
    //
    CSTART(ctx)?;

    //
    // Log the last test case if it passed.
    //
    if (save.NTEST > 1) {
        //
        // First we check the current trace and make sure it is the
        // same as the old trace.
        //
        CHCKSC(
            b"Current Trace",
            &save.CTRACE,
            b"=",
            &save.OTRACE,
            &mut save.OK,
            ctx,
        )?;

        TSTLIP(ctx)?;
        TSTRUL(ctx)?;
        TSTRUL(ctx)?;
        TSTCBL(ctx)?;
    }
    //
    // Set the "old" trace value.
    //
    T_TRACE(b"SET", &mut save.CTRACE, ctx);

    T_NAME(&mut save.OLDNAM, ctx);

    if fstr::ne(&save.OLDNAM, b" ") {
        //
        // We need to close out the old case
        //
        T_FCOUNT(&mut save.NFAIL, ctx);
        T_CNUM(&mut save.NCASE, ctx);
        T_SUCCESS(&mut save.OK, ctx);

        TSTSTY(&mut save.GOOD, &mut save.BAD, ctx);

        if save.OK {
            fstr::assign(&mut save.MESSGE, b"Passed  --- Test Family: ");
            spicelib::SUFFIX(&save.OLDNAM, 1, &mut save.MESSGE);

            TSTLOG(&save.MESSGE, false, ctx)?;

            TSTLGS(
                b"LEFT 9 RIGHT 78 FLAG --- NEWLINE /cr",
                b"LEFT 9 RIGHT 78 FLAG --- NEWLINE /cr",
                ctx,
            );

            fstr::assign(
                &mut save.MESSGE,
                b"Successful tests for # of # test cases. ",
            );
            spicelib::REPMCT(
                &save.MESSGE.to_vec(),
                b"#",
                save.NCASE,
                b"L",
                &mut save.MESSGE,
                ctx,
            )?;
            spicelib::REPMCT(
                &save.MESSGE.to_vec(),
                b"#",
                save.NCASE,
                b"L",
                &mut save.MESSGE,
                ctx,
            )?;
            TSTLOG(&save.MESSGE, false, ctx)?;
        } else {
            fstr::assign(&mut save.MESSGE, b"FAILURE for Test Family: ");
            spicelib::SUFFIX(&save.OLDNAM, 1, &mut save.MESSGE);

            TSTLOG(&save.MESSGE, false, ctx)?;

            TSTLGS(
                b"LEFT 24 RIGHT 78 FLAG : NEWLINE /cr",
                b"LEFT 24 RIGHT 78 FLAG : NEWLINE /cr",
                ctx,
            );
            fstr::assign(&mut save.MESSGE, b"# of # test cases failed. ");
            spicelib::REPMCT(
                &save.MESSGE.to_vec(),
                b"#",
                save.NFAIL,
                b"C",
                &mut save.MESSGE,
                ctx,
            )?;
            spicelib::REPMCT(
                &save.MESSGE.to_vec(),
                b"#",
                save.NCASE,
                b"L",
                &mut save.MESSGE,
                ctx,
            )?;
            TSTLOG(&save.MESSGE, false, ctx)?;
        }

        TSTLGS(&save.GOOD, &save.BAD, ctx);
        TSTLOG(b" ", false, ctx)?;
    }

    T_BEGIN(NAME, ctx);

    fstr::assign(
        &mut save.MESSGE,
        &fstr::concat(b"Testing --- Test Family: ", NAME),
    );
    save.R = spicelib::RTRIM(&save.MESSGE);

    TSTLOG(b" ", false, ctx)?;
    TSTLOG(fstr::substr(&save.MESSGE, 1..=save.R), false, ctx)?;
    TSTLOG(b" ", false, ctx)?;

    Ok(())
}

//
// The following two entry points are provided so that a program
// can fetch in order all tests that were begun with a call to TOPEN.
//
// The entry TBEGF sets the test family counter to 1.  The entry
// TFNAME fetches the next name from the list of test families and
// increments the test family counter.  When the family counter
// passes the number of calls to TOPEN, the name returned is set to
// a blank.
//
pub fn TBEGF(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.TFAMC = 1;
}

pub fn TFNAME(NAME: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if (save.TFAMC <= save.NTEST) {
        fstr::assign(NAME, save.TNAMES.get(save.TFAMC));
        save.TFAMC = (save.TFAMC + 1);
    } else {
        fstr::assign(NAME, b" ");
    }
}
