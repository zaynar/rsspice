//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const STYSIZ: i32 = 128;
const LNSIZE: i32 = 80;
const MAXTNM: i32 = 80;
const FILSIZ: i32 = 255;
const MAXCNM: i32 = 1000;

struct SaveVars {
    MYMARK: Vec<u8>,
    SEEN: Vec<u8>,
    HIDE: Vec<u8>,
    NAME: Vec<u8>,
    CNAME: Vec<u8>,
    EXTRA: Vec<u8>,
    ENV: Vec<u8>,
    TIME: Vec<u8>,
    VERSN: Vec<u8>,
    ERRFIL: Vec<u8>,
    PNUM: i32,
    FNUM: i32,
    R: i32,
    OPNERR: bool,
    SHOW: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MYMARK = vec![b' '; 1 as usize];
        let mut SEEN = vec![b' '; STYSIZ as usize];
        let mut HIDE = vec![b' '; STYSIZ as usize];
        let mut NAME = vec![b' '; MAXTNM as usize];
        let mut CNAME = vec![b' '; MAXCNM as usize];
        let mut EXTRA = vec![b' '; MAXCNM as usize];
        let mut ENV = vec![b' '; LNSIZE as usize];
        let mut TIME = vec![b' '; LNSIZE as usize];
        let mut VERSN = vec![b' '; LNSIZE as usize];
        let mut ERRFIL = vec![b' '; FILSIZ as usize];
        let mut PNUM: i32 = 0;
        let mut FNUM: i32 = 0;
        let mut R: i32 = 0;
        let mut OPNERR: bool = false;
        let mut SHOW: bool = false;

        OPNERR = true;
        SHOW = false;
        fstr::assign(&mut EXTRA, b" ");
        fstr::assign(&mut SEEN, b"LEFT 1 RIGHT 78");
        fstr::assign(&mut HIDE, b"LEFT 1 RIGHT 78");
        fstr::assign(&mut MYMARK, b"#");

        Self {
            MYMARK,
            SEEN,
            HIDE,
            NAME,
            CNAME,
            EXTRA,
            ENV,
            TIME,
            VERSN,
            ERRFIL,
            PNUM,
            FNUM,
            R,
            OPNERR,
            SHOW,
        }
    }
}

//$Procedure TSTLGR (Test utilities log files manager)
pub fn TSTLGR(
    MESSGE: &[u8],
    ERRLOG: bool,
    GSTYLE: &[u8],
    FSTYLE: &[u8],
    MARKER: &[u8],
    INT: i32,
    DP: f64,
) {

    //
    // Test Utility Functions.
    //
    //
    // SPICELIB Functions
    //
    //
    // Local Variables
    //
}

//
// This entry point handles the logging of commands.
//
pub fn TSTLOG(MESSGE: &[u8], ERRLOG: bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    //     Version
    //
    //-     Test Utilities Version 1.0.0, 3-NOV-1994 (WLT)
    //

    if ERRLOG {
        //
        // We need to open a log file and write some
        // special additional information if this is the first
        // message for this test case.
        //

        T_NAME(&mut save.NAME, ctx);
        T_CNAME(&mut save.CNAME, ctx);
        T_FCOUNT(&mut save.PNUM, ctx);
        T_CFAIL(ctx);
        T_FCOUNT(&mut save.FNUM, ctx);

        if save.OPNERR {
            save.OPNERR = false;

            TSTFIL(
                b"ERR{0-9}{0-9}{0-9}{0-9}.LOG",
                b"SAVE",
                &mut save.ERRFIL,
                ctx,
            )?;

            TSTGET(&mut save.ENV, &mut save.VERSN, &mut save.TIME, ctx);
            TSTIOA(b"SAVE", ctx)?;
            TSTIOH(b"SCREEN", ctx)?;
            TSTIOH(b"LOG", ctx)?;

            TSTWLN(&save.ENV, ctx)?;
            TSTWLN(&save.VERSN, ctx)?;
            TSTWLN(&save.TIME, ctx)?;
        }
        //
        // Activate the port for reporting errors, inhibit writing to the
        // screen or log file.
        //
        TSTIOA(b"SAVE", ctx)?;
        TSTIOA(b"SCREEN", ctx)?;
        TSTIOA(b"LOG", ctx)?;
        //
        // If there had been no previous errors reported for this
        // family of errors, we create a special message concerning
        // this family and case.
        //
        if (save.PNUM == 0) {
            TSTWLN(b" ", ctx)?;
            TSTWLN(b"A test failure occurred in the test family:", ctx)?;
            TSTWLN(b" ", ctx)?;
            TSTWLN(&fstr::concat(b"Family : ", &save.NAME), ctx)?;
            support::NICEPR_1(
                &save.CNAME,
                b"LEFT 1 RIGHT 78 HARDSPACE ^ FLAG CASE^^^:",
                TSTWLN,
                ctx,
            )?;
            TSTWLN(b" ", ctx)?;
        //
        // If this is the first error message to be reported for this
        // test case we note the failure for this test case.
        //
        } else if (save.PNUM != save.FNUM) {
            TSTWLN(b" ", ctx)?;
            TSTWLN(b"Test Case FAILURE. ", ctx)?;
            support::NICEPR_1(
                &save.CNAME,
                b"LEFT 1 RIGHT 78 HARDSPACE ^ FLAG CASE^^^:",
                TSTWLN,
                ctx,
            )?;
            TSTWLN(b" ", ctx)?;
        }
        //
        // Reactivate the screen and log file.
        //
        TSTIOA(b"SCREEN", ctx)?;
        TSTIOA(b"LOG", ctx)?;
    }

    if ERRLOG {
        if fstr::eq(MESSGE, b" ") {
            TSTWLN(b" ", ctx)?;
        } else {
            if save.SHOW {
                support::NICEPR_1(&save.EXTRA, &save.HIDE, TSTWLN, ctx)?;
                save.SHOW = false;
                TSTWLN(b" ", ctx)?;
            }

            support::NICEPR_1(MESSGE, &save.HIDE, TSTWLN, ctx)?;
        }
    } else {
        if fstr::eq(MESSGE, b" ") {
            TSTWLN(b" ", ctx)?;
        } else {
            support::NICEPR_1(MESSGE, &save.SEEN, TSTWLN, ctx)?;
        }
    }
    //
    // Inhibit writing to the error log.
    //

    TSTIOH(b"SAVE", ctx)?;

    Ok(())
}

//
// This entry point allows users to set the style used for
// logging failure and non-failure and visible commands.
//
pub fn TSTLGS(GSTYLE: &[u8], FSTYLE: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //  Version
    //
    //-     Test Utilities Version 1.0.0, 3-NOV-1994 (WLT)
    //
    //

    fstr::assign(&mut save.SEEN, GSTYLE);
    fstr::assign(&mut save.HIDE, FSTYLE);
}

//
// This entry point allows users to get the style used for
// logging failure and non-failure and visible commands.
//
pub fn TSTSTY(GSTYLE: &mut [u8], FSTYLE: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //  Version
    //
    //-     Test Utilities Version 1.0.0, 3-NOV-1994 (WLT)
    //
    //
    fstr::assign(GSTYLE, &save.SEEN);
    fstr::assign(FSTYLE, &save.HIDE);
}

//
// This entry point handles the logging of commands when we want
// only verbose logging.
//
pub fn VRBLOG(MESSGE: &[u8], ERRLOG: bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    //  Version
    //
    //-     Test Utilities Version 1.0.0, 3-NOV-1994 (WLT)
    //

    if ERRLOG {
        //
        // We need to possibly open a log file and write some
        // special additional information if this is the first
        // message for this test case.
        //

        T_NAME(&mut save.NAME, ctx);
        T_CNAME(&mut save.CNAME, ctx);
        T_FCOUNT(&mut save.PNUM, ctx);
        T_CFAIL(ctx);
        T_FCOUNT(&mut save.FNUM, ctx);

        if save.OPNERR {
            save.OPNERR = false;

            TSTFIL(
                b"ERR{0-9}{0-9}{0-9}{0-9}.LOG",
                b"SAVE",
                &mut save.ERRFIL,
                ctx,
            )?;

            TSTGET(&mut save.ENV, &mut save.VERSN, &mut save.TIME, ctx);
            TSTIOA(b"SAVE", ctx)?;
            TSTIOH(b"SCREEN", ctx)?;
            TSTIOH(b"LOG", ctx)?;

            TSTWLN(&save.ENV, ctx)?;
            TSTWLN(&save.VERSN, ctx)?;
            TSTWLN(&save.TIME, ctx)?;
        }
        //
        // Activate the port for reporting errors, inhibit writing to the
        // screen or log file.
        //
        TSTIOA(b"SAVE", ctx)?;
        TSTIOA(b"SCREEN", ctx)?;
        TSTIOA(b"LOG", ctx)?;
        //
        // If there had been no previous errors reported for this
        // family of errors, we create a special message concerning
        // this family and case.
        //
        if (save.PNUM == 0) {
            TSTWLN(b" ", ctx)?;
            TSTWLN(b"A test failure occurred in the test family:", ctx)?;
            TSTWLN(b" ", ctx)?;
            TSTWLN(&fstr::concat(b"Family : ", &save.NAME), ctx)?;
            support::NICEPR_1(
                &save.CNAME,
                b"LEFT 1 RIGHT 78 HARDSPACE ^ FLAG CASE^^^:",
                TSTWLN,
                ctx,
            )?;
            TSTWLN(b" ", ctx)?;
        //
        // If this is the first error message to be reported for this
        // test case we note the failure for this test case.
        //
        } else if (save.PNUM != save.FNUM) {
            TSTWLN(b" ", ctx)?;
            TSTWLN(b"Test Case FAILURE. ", ctx)?;
            support::NICEPR_1(
                &save.CNAME,
                b"LEFT 1 RIGHT 78 HARDSPACE ^ FLAG CASE^^^:",
                TSTWLN,
                ctx,
            )?;
            TSTWLN(b" ", ctx)?;
        }
        //
        // Reactivate the screen and log file.
        //
        TSTIOA(b"SCREEN", ctx)?;
        TSTIOA(b"LOG", ctx)?;
    }
    //
    // If there is an error, write to every place.
    //
    if ERRLOG {
        if fstr::eq(MESSGE, b" ") {
            TSTWLN(b" ", ctx)?;
        } else {
            if save.SHOW {
                support::NICEPR_1(&save.EXTRA, &save.HIDE, TSTWLN, ctx)?;
                TSTWLN(b" ", ctx)?;
                save.SHOW = false;
            }

            support::NICEPR_1(MESSGE, &save.HIDE, TSTWLN, ctx)?;
        }
    //
    // Otherwise we write out the message only if in verbose mode.
    //
    } else if VERBOS(ctx) {
        if fstr::eq(MESSGE, b" ") {
            TSTWLN(b" ", ctx)?;
        } else {
            support::NICEPR_1(MESSGE, &save.SEEN, TSTWLN, ctx)?;
        }
    }
    //
    // Inhibit writing to the error log.
    //

    TSTIOH(b"SAVE", ctx)?;

    Ok(())
}

//
// Establish a special message to be printed if an error is
// detected.
//
pub fn TSTMSG(MARKER: &[u8], MESSGE: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let MARKER = &MARKER[..1 as usize];

    fstr::assign(&mut save.MYMARK, MARKER);
    fstr::assign(&mut save.EXTRA, MESSGE);
    save.SHOW = true;
}

//
// Fill in the next marker with the string representing INT.
//
pub fn TSTMSI(INT: i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::REPMI(
        &save.EXTRA.to_vec(),
        &save.MYMARK,
        INT,
        &mut save.EXTRA,
        ctx,
    );
}

//
// Fill in the next marker with the string representing DP
//
pub fn TSTMSD(DP: f64, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::REPMD(
        &save.EXTRA.to_vec(),
        &save.MYMARK,
        DP,
        14,
        &mut save.EXTRA,
        ctx,
    );
}

pub fn TSTMSF(DP: f64, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    spicelib::REPMF(
        &save.EXTRA.to_vec(),
        &save.MYMARK,
        DP,
        14,
        b"F",
        &mut save.EXTRA,
        ctx,
    );
}

//
// Fill in the next marker with the character string MESSGE.
//
pub fn TSTMSC(MESSGE: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.R = spicelib::RTRIM(MESSGE);

    spicelib::REPMC(
        &save.EXTRA.to_vec(),
        &save.MYMARK,
        fstr::substr(MESSGE, 1..=save.R),
        &mut save.EXTRA,
    );
}

//$Procedure TSTMSO ( Replace marker with ordinal )
pub fn TSTMSO(INT: i32, MARKER: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let MARKER = &MARKER[..1 as usize];

    if (((((fstr::ne(MARKER, b"C") && fstr::ne(MARKER, b"c")) && fstr::ne(MARKER, b"l"))
        && fstr::ne(MARKER, b"u"))
        && fstr::ne(MARKER, b"L"))
        && fstr::ne(MARKER, b"U"))
    {
        spicelib::REPMOT(
            &save.EXTRA.to_vec(),
            &save.MYMARK,
            INT,
            b"l",
            &mut save.EXTRA,
            ctx,
        )?;
    } else {
        spicelib::REPMOT(
            &save.EXTRA.to_vec(),
            &save.MYMARK,
            INT,
            MARKER,
            &mut save.EXTRA,
            ctx,
        )?;
    }
    Ok(())
}

pub fn TSTMST(INT: i32, MARKER: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let MARKER = &MARKER[..1 as usize];

    if (((((fstr::ne(MARKER, b"C") && fstr::ne(MARKER, b"c")) && fstr::ne(MARKER, b"l"))
        && fstr::ne(MARKER, b"u"))
        && fstr::ne(MARKER, b"L"))
        && fstr::ne(MARKER, b"U"))
    {
        spicelib::REPMCT(
            &save.EXTRA.to_vec(),
            &save.MYMARK,
            INT,
            b"l",
            &mut save.EXTRA,
            ctx,
        )?;
    } else {
        spicelib::REPMCT(
            &save.EXTRA.to_vec(),
            &save.MYMARK,
            INT,
            MARKER,
            &mut save.EXTRA,
            ctx,
        )?;
    }
    Ok(())
}

//
// Turn off the test message so that it will not be shown.
//
pub fn TSTMOF(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.SHOW = false;
}

//$Procedure ZZTSTRE (reset the error log file flag)
pub fn ZZTSTRE(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.OPNERR = true;
}
