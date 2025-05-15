//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 80;
const WDSIZE: i32 = 32;

struct SaveVars {
    STYLE: Vec<u8>,
    HIDE: Vec<u8>,
    SEEN: Vec<u8>,
    HSTYLE: Vec<u8>,
    DONT: Vec<u8>,
    REPEAT: Vec<u8>,
    FRSTWD: Vec<u8>,
    SCNDWD: Vec<u8>,
    THRDWD: Vec<u8>,
    DELIM: Vec<u8>,
    CDELIM: Vec<u8>,
    FLAG: Vec<u8>,
    LEAD: Vec<u8>,
    LOC: i32,
    STAT: StackArray<bool, 3>,
    DOIT: bool,
    FIRST: bool,
    WIPE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STYLE = vec![b' '; LNSIZE as usize];
        let mut HIDE = vec![b' '; LNSIZE as usize];
        let mut SEEN = vec![b' '; LNSIZE as usize];
        let mut HSTYLE = vec![b' '; LNSIZE as usize];
        let mut DONT = vec![b' '; WDSIZE as usize];
        let mut REPEAT = vec![b' '; WDSIZE as usize];
        let mut FRSTWD = vec![b' '; WDSIZE as usize];
        let mut SCNDWD = vec![b' '; WDSIZE as usize];
        let mut THRDWD = vec![b' '; WDSIZE as usize];
        let mut DELIM = vec![b' '; 1 as usize];
        let mut CDELIM = vec![b' '; 1 as usize];
        let mut FLAG = vec![b' '; 3 as usize];
        let mut LEAD = vec![b' '; 3 as usize];
        let mut LOC: i32 = 0;
        let mut STAT = StackArray::<bool, 3>::new(1..=3);
        let mut DOIT: bool = false;
        let mut FIRST: bool = false;
        let mut WIPE: bool = false;

        DOIT = false;
        FIRST = true;

        Self {
            STYLE,
            HIDE,
            SEEN,
            HSTYLE,
            DONT,
            REPEAT,
            FRSTWD,
            SCNDWD,
            THRDWD,
            DELIM,
            CDELIM,
            FLAG,
            LEAD,
            LOC,
            STAT,
            DOIT,
            FIRST,
            WIPE,
        }
    }
}

//$Procedure      ECHO ( Echo the translation of a string )
pub fn ECHO(STRING: &[u8], TRANSL: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.FIRST {
        //
        // Find out what the words for NO and ECHO are
        // in the current language.
        //
        save.FIRST = false;

        TRNLAT(b"DONT", &mut save.DONT, ctx);
        TRNLAT(b"ECHO", &mut save.REPEAT, ctx);
    }

    spicelib::NTHWD(TRANSL, 1, &mut save.FRSTWD, &mut save.LOC);
    spicelib::NTHWD(TRANSL, 2, &mut save.SCNDWD, &mut save.LOC);
    spicelib::NTHWD(TRANSL, 3, &mut save.THRDWD, &mut save.LOC);

    spicelib::UCASE(&save.FRSTWD.to_vec(), &mut save.FRSTWD, ctx);
    spicelib::UCASE(&save.SCNDWD.to_vec(), &mut save.SCNDWD, ctx);
    spicelib::UCASE(&save.THRDWD.to_vec(), &mut save.THRDWD, ctx);

    if (fstr::eq(&save.FRSTWD, &save.REPEAT) && fstr::eq(&save.SCNDWD, b" ")) {
        save.WIPE = true;
        save.DOIT = true;
    } else if ((fstr::eq(&save.FRSTWD, &save.DONT) && fstr::eq(&save.SCNDWD, &save.REPEAT))
        && fstr::eq(&save.THRDWD, b" "))
    {
        save.WIPE = true;
        save.DOIT = false;
    } else {
        save.WIPE = false;
    }

    if save.DOIT {
        if fstr::ne(STRING, TRANSL) {
            //
            // Get the current margins and the delimiter.
            //
            NSPMRG(&mut save.STYLE, ctx);
            GETDEL(&mut save.DELIM, ctx);
            //
            // Create the NICEIO style string it will be of the form
            //
            //    LEFT 1 RIGHT margin FLAG ;;; LEADER  ;
            //
            // (provided of course that ';' is the command
            //
            fstr::assign(
                &mut save.FLAG,
                &fstr::concat(&fstr::concat(&save.DELIM, &save.DELIM), &save.DELIM),
            );
            fstr::assign(&mut save.LEAD, &fstr::concat(&save.DELIM, b"++"));

            spicelib::PREFIX(&save.LEAD, 1, &mut save.STYLE);
            spicelib::PREFIX(b"LEADER ", 1, &mut save.STYLE);
            spicelib::PREFIX(&save.FLAG, 1, &mut save.STYLE);
            spicelib::PREFIX(b"FLAG", 1, &mut save.STYLE);
            //
            // Get the current status of the "log" port and
            // for the moment inhibit writing to that port.
            //
            NSPGST(b"LOG", save.STAT.as_slice_mut(), ctx)?;
            NSPIOH(b"LOG", ctx)?;
            //
            // Display the translated string.
            //
            NICEPR_1(TRANSL, &save.STYLE, NSPWLN, ctx)?;
            //
            // Now re-establish the status of the log port.
            //
            NSPPST(b"LOG", save.STAT.as_slice(), ctx)?;
            //
            // Send the translated string to the log file and
            // do it so that it is a comment in the log file.
            // Note that we use a special logging style for
            // echoing the symbol translation.
            //
            fstr::assign(&mut save.HSTYLE, b"LEFT 1 RIGHT 78 ");

            spicelib::PREFIX(&save.LEAD, 1, &mut save.HSTYLE);
            spicelib::PREFIX(b"LEADER ", 1, &mut save.HSTYLE);
            spicelib::PREFIX(&save.FLAG, 1, &mut save.HSTYLE);
            spicelib::PREFIX(b"FLAG", 1, &mut save.HSTYLE);

            NSPGLS(&mut save.SEEN, &mut save.HIDE, &mut save.CDELIM, ctx);
            NSPLGS(&save.SEEN, &save.HSTYLE, &save.CDELIM, ctx);
            NSPLOG(TRANSL, true, ctx)?;
            NSPLGS(&save.SEEN, &save.HIDE, &save.CDELIM, ctx);
        }
    }

    if save.WIPE {
        fstr::assign(TRANSL, b" ");
    }

    Ok(())
}

//
// The following entry points allow you to
//
//    1) Enable echoing of translations
//    2) Disable echoing of translations
//    3) Find out the current status of echoing.
//
// Since the code in each case is trivial, we aren't
// going to set up those big old nasty NAIF headers.
// (What a rebel!)
//
pub fn DOECHO(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.DOIT = true;
}

pub fn NOECHO(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.DOIT = false;
}

pub fn GTECHO(STRING: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.DOIT {
        fstr::assign(STRING, b"ENABLED");
    } else {
        fstr::assign(STRING, b"DISABLED");
    }
}
