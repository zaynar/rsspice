//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const WDSIZE: i32 = 32;
const NUPPER: i32 = 5;
const NSYN: i32 = NUPPER;
const SYNLEN: i32 = 80;
const NSP: i32 = 2;
const LNSIZE: i32 = 512;
const MAXLNS: i32 = 3;

struct SaveVars {
    SYNKEY: ActualCharArray,
    SYNPTR: StackArray<i32, 11>,
    SYNVAL: ActualCharArray,
    SPCIAL: ActualCharArray,
    MYERR: ActualCharArray,
    TEMPLT: Vec<u8>,
    VALUES: ActualCharArray,
    NAMES: ActualCharArray,
    E: i32,
    I: i32,
    L: i32,
    NITEMS: i32,
    REST: i32,
    FOUND: bool,
    FIRST: bool,
    STATUS: StackArray<bool, 3>,
    DOSAV: bool,
    DODISC: bool,
    DOEDIT: bool,
    DOSYM: bool,
    DOENV: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SYNKEY = ActualCharArray::new(WDSIZE, LBCELL..=NSYN);
        let mut SYNPTR = StackArray::<i32, 11>::new(LBCELL..=NSYN);
        let mut SYNVAL = ActualCharArray::new(SYNLEN, LBCELL..=NSYN);
        let mut SPCIAL = ActualCharArray::new(8, 1..=NSP);
        let mut MYERR = ActualCharArray::new(LNSIZE, 1..=2);
        let mut TEMPLT = vec![b' '; 80 as usize];
        let mut VALUES = ActualCharArray::new(LNSIZE, 1..=MAXLNS);
        let mut NAMES = ActualCharArray::new(WDSIZE, 1..=MAXLNS);
        let mut E: i32 = 0;
        let mut I: i32 = 0;
        let mut L: i32 = 0;
        let mut NITEMS: i32 = 0;
        let mut REST: i32 = 0;
        let mut FOUND: bool = false;
        let mut FIRST: bool = false;
        let mut STATUS = StackArray::<bool, 3>::new(1..=3);
        let mut DOSAV: bool = false;
        let mut DODISC: bool = false;
        let mut DOEDIT: bool = false;
        let mut DOSYM: bool = false;
        let mut DOENV: bool = false;

        FIRST = true;
        DOSAV = true;
        DODISC = true;
        DOEDIT = true;
        DOSYM = true;
        DOENV = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"SET[set]   EDITOR[editor] (1:)@word[rest]"),
                Val::C(b"SHOW[show] SYMBOL[symbol] @word[def]"),
                Val::C(b"SHOW[show] ENVIRONMENT[env]"),
                Val::C(b"SAVE[save] TO  @word[rest]"),
                Val::C(b"DISCARD[discard]"),
            ]
            .into_iter();
            for I in intrinsics::range(1, NUPPER, 1) {
                fstr::assign(SYNVAL.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b" "), Val::C(b"?")].into_iter();
            SPCIAL
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            SYNKEY,
            SYNPTR,
            SYNVAL,
            SPCIAL,
            MYERR,
            TEMPLT,
            VALUES,
            NAMES,
            E,
            I,
            L,
            NITEMS,
            REST,
            FOUND,
            FIRST,
            STATUS,
            DOSAV,
            DODISC,
            DOEDIT,
            DOSYM,
            DOENV,
        }
    }
}

//$Procedure      BUILTN ( Built in Commands )
pub fn BUILTN(
    COMMND: &mut [u8],
    HIT: &mut bool,
    ERROR: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ERROR = DummyCharArrayMut::new(ERROR, None, 1..=2);

    //
    // Spicelib functions
    //
    //
    // Error handling interface routines.
    //
    //
    // META/2 Functions
    //

    //
    // Inspekt External Routines
    //

    //
    // Variables needed for syntax declarations.
    //

    //
    // The following are for special commands that will not be
    // processed by BUILTN.
    //

    //
    // Other Local Variables
    //

    //
    // Save everything
    //

    spicelib::CHKIN(b"BUILTN", ctx)?;

    if save.FIRST {
        save.FIRST = false;
        save.I = 0;
        save.I = spicelib::TOUCHI(save.I);
        M2INTS(
            NSYN,
            save.SYNKEY.as_arg_mut(),
            save.SYNPTR.as_slice_mut(),
            save.SYNVAL.as_arg_mut(),
            ctx,
        )?;
    }

    save.L = spicelib::LTRIM(COMMND);
    save.REST = (spicelib::RTRIM(COMMND) + 1);

    if (spicelib::ISRCHC(
        fstr::substr(COMMND, save.L..=save.REST),
        NSP,
        save.SPCIAL.as_arg(),
    ) > 0)
    {
        spicelib::CHKOUT(b"BUILTN", ctx)?;
        return Ok(());
    }
    //
    // There are no errors yet.
    //
    fstr::assign(ERROR.get_mut(1), b" ");
    fstr::assign(ERROR.get_mut(2), b" ");
    *HIT = false;
    //
    // Check the input command to see if it is recognizable
    //
    M2CHCK(
        COMMND,
        save.SYNKEY.as_arg(),
        save.SYNPTR.as_slice(),
        save.SYNVAL.as_arg_mut(),
        save.MYERR.as_arg_mut(),
        ctx,
    )?;

    if fstr::ne(save.MYERR.get(1), b" ") {
        spicelib::CHKOUT(b"BUILTN", ctx)?;
        return Ok(());
    }

    if (M2XIST(b"set", ctx)? && save.DOEDIT) {
        M2VGET(
            b"rest",
            1,
            &mut save.FOUND,
            &mut save.REST,
            &mut save.E,
            ctx,
        )?;
        SETEDT(fstr::substr(COMMND, save.REST..), ctx);
        *HIT = true;
    } else if (M2XIST(b"symbol", ctx)? && save.DOSYM) {
        M2GETC(b"def", COMMND, &mut save.FOUND, &mut save.TEMPLT, ctx)?;
        SHOSYM(&save.TEMPLT, ctx)?;
        *HIT = true;
    } else if (M2XIST(b"env", ctx)? && save.DOENV) {
        save.NITEMS = 3;
        fstr::assign(save.NAMES.get_mut(1), b"Editor");
        fstr::assign(save.NAMES.get_mut(2), b"Echoing Commands");
        fstr::assign(save.NAMES.get_mut(3), b"Screen Output File");

        GETEDT(&mut save.VALUES[1], ctx);
        GTECHO(&mut save.VALUES[2], ctx);

        NSPGST(b"SAVE", save.STATUS.as_slice_mut(), ctx)?;

        if ((save.STATUS[1] && save.STATUS[2]) && !save.STATUS[3]) {
            NSPPFL(b"SAVE", &mut save.VALUES[3], ctx)?;
        } else {
            fstr::assign(save.VALUES.get_mut(3), b"No Current Screen Save File");
        }

        NSPWLN(b" ", ctx)?;
        NSPWLN(b"Current Environment", ctx)?;
        NSPWLN(b" ", ctx)?;
        FLGRPT(
            save.NITEMS,
            save.NAMES.as_arg(),
            save.VALUES.as_arg(),
            NSPWLN,
            ctx,
        )?;
        NSPWLN(b" ", ctx)?;
        *HIT = true;
    } else if (M2XIST(b"save", ctx)? && save.DOSAV) {
        M2VGET(
            b"rest",
            1,
            &mut save.FOUND,
            &mut save.REST,
            &mut save.E,
            ctx,
        )?;
        NSPSAV(fstr::substr(COMMND, save.REST..), ERROR.as_arg_mut(), ctx)?;
        *HIT = true;
    } else if (M2XIST(b"discard", ctx)? && save.DODISC) {
        NSPIOC(b"SAVE", ctx)?;
        *HIT = true;
    }

    save.FOUND = HAVE(ERROR.as_arg_mut(), ctx)?;
    spicelib::CHKOUT(b"BUILTN", ctx)?;
    Ok(())
}

//$Procedure      BUILTO ( Built in commands off )
pub fn BUILTO(COMMND: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // We just look at command to see which of the built in
    // command should be disabled.
    //
    save.DOSAV = (intrinsics::INDEX(COMMND, b"SAVE") == 0);
    save.DOENV = (intrinsics::INDEX(COMMND, b"ENVIRONMENT") == 0);
    save.DOEDIT = (intrinsics::INDEX(COMMND, b"EDITOR") == 0);
    save.DOSYM = (intrinsics::INDEX(COMMND, b"SYMBOL") == 0);
    save.DODISC = (intrinsics::INDEX(COMMND, b"DISCARD") == 0);
}
