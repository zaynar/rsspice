//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NONE: i32 = 0;
const COMBUF: i32 = (NONE + 1);
const KEYBRD: i32 = (COMBUF + 1);
const INPFIL: i32 = (KEYBRD + 1);
const FILSIZ: i32 = 255;
const WDSIZE: i32 = 32;
const MAXCOM: i32 = 20;
const FILEN: i32 = 128;
const COMSIZ: i32 = 1024;
const ERRSIZ: i32 = 1760;
const STYSIZ: i32 = 120;
const NSP: i32 = 2;

struct SaveVars {
    USENAM: Vec<u8>,
    ERRFLG: Vec<u8>,
    DOLOG: bool,
    COMMND: Vec<u8>,
    COM2DO: Vec<u8>,
    ERROR: ActualCharArray,
    PROBLM: bool,
    SSTYLE: Vec<u8>,
    LSTYLE: Vec<u8>,
    VSTYLE: Vec<u8>,
    HSTYLE: Vec<u8>,
    FROM: i32,
    L: i32,
    REST: i32,
    LOG: StackArray<bool, 4>,
    TRAP: bool,
    HIT: bool,
    SPCIAL: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut USENAM = vec![b' '; FILSIZ as usize];
        let mut ERRFLG = vec![b' '; WDSIZE as usize];
        let mut DOLOG: bool = false;
        let mut COMMND = vec![b' '; COMSIZ as usize];
        let mut COM2DO = vec![b' '; COMSIZ as usize];
        let mut ERROR = ActualCharArray::new(ERRSIZ, 1..=2);
        let mut PROBLM: bool = false;
        let mut SSTYLE = vec![b' '; STYSIZ as usize];
        let mut LSTYLE = vec![b' '; STYSIZ as usize];
        let mut VSTYLE = vec![b' '; STYSIZ as usize];
        let mut HSTYLE = vec![b' '; STYSIZ as usize];
        let mut FROM: i32 = 0;
        let mut L: i32 = 0;
        let mut REST: i32 = 0;
        let mut LOG = StackArray::<bool, 4>::new(0..=3);
        let mut TRAP: bool = false;
        let mut HIT: bool = false;
        let mut SPCIAL = ActualCharArray::new(8, 1..=NSP);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b" "), Val::C(b"?")].into_iter();
            SPCIAL
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            USENAM,
            ERRFLG,
            DOLOG,
            COMMND,
            COM2DO,
            ERROR,
            PROBLM,
            SSTYLE,
            LSTYLE,
            VSTYLE,
            HSTYLE,
            FROM,
            L,
            REST,
            LOG,
            TRAP,
            HIT,
            SPCIAL,
        }
    }
}

//$Proceedure CMLOOP ( Command line loop )
//
pub fn CMLOOP(
    DELIM: &[u8],
    PROMPT: &[u8],
    LOGNAM: &[u8],
    VERSN: &[u8],
    GREET: fn(&[u8], &mut Context) -> f2rust_std::Result<()>,
    PREPRC: fn(&[u8], &mut [u8]) -> (),
    ACTION: fn(&mut [u8], CharArrayMut, &mut Context) -> f2rust_std::Result<()>,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    //
    // Language Sensitive Strings
    //

    SETDEL(DELIM, ctx);

    fstr::assign(save.ERROR.get_mut(1), b" ");
    fstr::assign(save.ERROR.get_mut(2), b" ");
    fstr::assign(&mut save.COMMND, b" ");

    save.LOG[NONE] = false;
    save.LOG[COMBUF] = false;
    save.LOG[KEYBRD] = true;
    save.LOG[INPFIL] = true;

    spicelib::ERRACT(b"SET", &mut b"RETURN".clone(), ctx)?;
    spicelib::ERRDEV(b"SET", &mut b"NULL".clone(), ctx)?;

    SETDAP(DELIM, PROMPT, ctx)?;

    TRNLAT(b"ERRFLAG", &mut save.ERRFLG, ctx);

    fstr::assign(
        &mut save.SSTYLE,
        &fstr::concat(b"HARDSPACE ^ NEWLINE /cr VTAB /vt FLAG ", &save.ERRFLG),
    );

    fstr::assign(
        &mut save.LSTYLE,
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(
                                b"HARDSPACE ^ NEWLINE /cr VTAB /vt FLAG ",
                                fstr::substr(DELIM, 1..=1),
                            ),
                            fstr::substr(&save.ERRFLG, 1..=QRTRIM(&save.ERRFLG)),
                        ),
                        b" LEADER ",
                    ),
                    fstr::substr(DELIM, 1..=1),
                ),
                b"-- ",
            ),
            b"LEFT 1 RIGHT 72 ",
        ),
    );
    fstr::assign(&mut save.VSTYLE, b"LEFT 1 RIGHT 78 ");
    fstr::assign(
        &mut save.HSTYLE,
        &fstr::concat(
            &fstr::concat(b"LEFT 1 RIGHT 78 LEADER ", fstr::substr(DELIM, 1..=1)),
            b"-- ",
        ),
    );

    NSPSTY(&save.SSTYLE, &save.LSTYLE, ctx);
    NSPLGS(&save.VSTYLE, &save.HSTYLE, DELIM, ctx);
    NSPSLR(1, 78, ctx);

    LOGCHK(LOGNAM, &mut save.USENAM, &mut save.DOLOG, ctx);

    if save.DOLOG {
        NSPOPL(&save.USENAM, VERSN, ctx)?;
    }

    if HAVE(save.ERROR.as_arg_mut(), ctx)? {
        NSPERR(&save.COMMND, save.ERROR.as_arg_mut(), ctx)?;
    }
    GREET(VERSN, ctx)?;
    CMSTUP(ctx)?;

    save.TRAP = true;
    while save.TRAP {
        GETCOM(&mut save.COM2DO, &mut save.FROM, ctx)?;
        EDTCOM(DELIM, PROMPT, &mut save.COM2DO, &mut save.FROM, ctx)?;

        if (NO(save.ERROR.first_mut(), ctx)? && save.LOG[save.FROM]) {
            NSPLOG(&save.COM2DO, false, ctx)?;
        }

        if NO(save.ERROR.first_mut(), ctx)? {
            RESSYM(&save.COM2DO, &mut save.COMMND, ctx)?;
            ECHO(&save.COM2DO, &mut save.COMMND, ctx)?;
        }

        if NO(save.ERROR.first_mut(), ctx)? {
            CMREDO(&save.COMMND, save.FROM, &mut save.TRAP, ctx)?;
        }

        if HAVE(save.ERROR.as_arg_mut(), ctx)? {
            save.TRAP = false;
        }
    }

    fstr::assign(&mut save.COM2DO, &save.COMMND);

    PREPRC(&save.COM2DO, &mut save.COMMND);

    while CMMORE(&save.COMMND, ctx) {
        if NO(save.ERROR.first_mut(), ctx)? {
            BUILTN(
                &mut save.COMMND,
                &mut save.HIT,
                save.ERROR.as_arg_mut(),
                ctx,
            )?;
        }

        if (NO(save.ERROR.first_mut(), ctx)? && !save.HIT) {
            save.L = spicelib::LTRIM(&save.COMMND);
            save.REST = (QRTRIM(&save.COMMND) + 1);

            if (spicelib::ISRCHC(
                fstr::substr(&save.COMMND, save.L..=save.REST),
                NSP,
                save.SPCIAL.as_arg(),
            ) == 0)
            {
                ACTION(&mut save.COMMND, save.ERROR.as_arg_mut(), ctx)?;
            }
        }

        save.PROBLM = HAVE(save.ERROR.as_arg_mut(), ctx)?;
        NSPERR(&save.COMMND, save.ERROR.as_arg_mut(), ctx)?;
        save.TRAP = true;

        while save.TRAP {
            GETCOM(&mut save.COM2DO, &mut save.FROM, ctx)?;
            EDTCOM(DELIM, PROMPT, &mut save.COM2DO, &mut save.FROM, ctx)?;

            if (NO(save.ERROR.first_mut(), ctx)? && save.LOG[save.FROM]) {
                NSPLOG(&save.COM2DO, false, ctx)?;
            }

            if NO(save.ERROR.first_mut(), ctx)? {
                RESSYM(&save.COM2DO, &mut save.COMMND, ctx)?;
                ECHO(&save.COM2DO, &mut save.COMMND, ctx)?;
            }

            if NO(save.ERROR.first_mut(), ctx)? {
                CMREDO(&save.COMMND, save.FROM, &mut save.TRAP, ctx)?;
            }

            if HAVE(save.ERROR.as_arg_mut(), ctx)? {
                save.TRAP = false;
            }
        }
        fstr::assign(&mut save.COM2DO, &save.COMMND);
        PREPRC(&save.COM2DO, &mut save.COMMND);
    }

    if save.LOG[save.FROM] {
        NSPEND(ctx)?;
    }

    Ok(())
}
