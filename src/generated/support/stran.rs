//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LBCELL: i32 = -5;
const LBCBUF: i32 = 0;
const LNSIZE: i32 = 1024;
const WDSIZE: i32 = 32;
const MAXN: i32 = 200;
const MAXL: i32 = 256;
const AVGL: i32 = 64;
const PTRSIZ: i32 = ((MAXN * 4) + 4);
const MAXCHR: i32 = (MAXN * AVGL);
const NLINES: i32 = ((MAXCHR / MAXL) + 1);
const NRES: i32 = 12;
const BIGWRD: i32 = 33;

struct SaveVars {
    NAMES: ActualCharArray,
    PTRS: ActualArray<i32>,
    BUFFER: ActualCharArray,
    DELIM: Vec<u8>,
    RESVRD: ActualCharArray,
    SYMBL: Vec<u8>,
    SYMBOL: Vec<u8>,
    KEY: Vec<u8>,
    PATTRN: Vec<u8>,
    MYPRMT: Vec<u8>,
    ALPHAB: Vec<u8>,
    DEF: Vec<u8>,
    EQUOTE: Vec<u8>,
    I: i32,
    J: i32,
    L: i32,
    LDEF: i32,
    LENO: i32,
    LOC: i32,
    LOUT: i32,
    LSTTRY: i32,
    LSYM: i32,
    N: i32,
    NNAME: i32,
    NXTCHR: i32,
    PLACE: i32,
    PSIZE: i32,
    SLOT: i32,
    VDIM: i32,
    CHECK: StackArray<bool, 200>,
    CHECKD: StackArray<bool, 200>,
    FIRST: bool,
    GOTONE: bool,
    NEW: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NAMES = ActualCharArray::new(WDSIZE, LBCELL..=MAXN);
        let mut PTRS = ActualArray::<i32>::new(LBCELL..=PTRSIZ);
        let mut BUFFER = ActualCharArray::new(MAXL, LBCBUF..=NLINES);
        let mut DELIM = vec![b' '; 1 as usize];
        let mut RESVRD = ActualCharArray::new(WDSIZE, 1..=NRES);
        let mut SYMBL = vec![b' '; BIGWRD as usize];
        let mut SYMBOL = vec![b' '; BIGWRD as usize];
        let mut KEY = vec![b' '; WDSIZE as usize];
        let mut PATTRN = vec![b' '; 80 as usize];
        let mut MYPRMT = vec![b' '; 80 as usize];
        let mut ALPHAB = vec![b' '; WDSIZE as usize];
        let mut DEF = vec![b' '; LNSIZE as usize];
        let mut EQUOTE = vec![b' '; 1 as usize];
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut L: i32 = 0;
        let mut LDEF: i32 = 0;
        let mut LENO: i32 = 0;
        let mut LOC: i32 = 0;
        let mut LOUT: i32 = 0;
        let mut LSTTRY: i32 = 0;
        let mut LSYM: i32 = 0;
        let mut N: i32 = 0;
        let mut NNAME: i32 = 0;
        let mut NXTCHR: i32 = 0;
        let mut PLACE: i32 = 0;
        let mut PSIZE: i32 = 0;
        let mut SLOT: i32 = 0;
        let mut VDIM: i32 = 0;
        let mut CHECK = StackArray::<bool, 200>::new(1..=MAXN);
        let mut CHECKD = StackArray::<bool, 200>::new(1..=MAXN);
        let mut FIRST: bool = false;
        let mut GOTONE: bool = false;
        let mut NEW: bool = false;

        FIRST = true;

        Self {
            NAMES,
            PTRS,
            BUFFER,
            DELIM,
            RESVRD,
            SYMBL,
            SYMBOL,
            KEY,
            PATTRN,
            MYPRMT,
            ALPHAB,
            DEF,
            EQUOTE,
            I,
            J,
            L,
            LDEF,
            LENO,
            LOC,
            LOUT,
            LSTTRY,
            LSYM,
            N,
            NNAME,
            NXTCHR,
            PLACE,
            PSIZE,
            SLOT,
            VDIM,
            CHECK,
            CHECKD,
            FIRST,
            GOTONE,
            NEW,
        }
    }
}

//$Procedure     STRAN
pub fn STRAN(
    INPUT: &[u8],
    OUTPUT: &mut [u8],
    TRAN: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    // SPICELIB Functions
    //
    //
    // Other supporting functions
    //

    //
    // The following parameters are used to define our table
    // of symbol translations.
    //

    //
    // Longest allowed symbol name is given by WDSIZE
    //
    //
    // Maximum number of allowed symbols is MAXN
    //
    //
    // The longest we expect any symbol to be is MAXL characters
    //
    //
    // The average number of characters per symbol is AVGL
    //

    //
    // Finally, here are the arrays used to hold the symbol translations.
    //

    //
    // Here's the storage we need for the reserved words.
    //

    //
    // Set up all of the data structures and special strings in
    // the first pass through the routine.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"STRAN", ctx)?;

    if save.FIRST {
        save.FIRST = false;

        save.VDIM = NLINES;
        save.PSIZE = PTRSIZ;
        save.NNAME = MAXN;

        SBINIT_1(
            save.NNAME,
            save.PSIZE,
            save.VDIM,
            save.NAMES.as_arg_mut(),
            save.PTRS.as_slice_mut(),
            save.BUFFER.as_arg_mut(),
            ctx,
        )?;

        fstr::assign(save.RESVRD.get_mut(1), b"START");
        fstr::assign(save.RESVRD.get_mut(2), b"STOP");
        fstr::assign(save.RESVRD.get_mut(3), b"EXIT");
        fstr::assign(save.RESVRD.get_mut(4), b"INQUIRE");
        fstr::assign(save.RESVRD.get_mut(5), b"SHOW");
        fstr::assign(save.RESVRD.get_mut(6), b"DEFINE");
        fstr::assign(save.RESVRD.get_mut(7), b"SHOW");
        fstr::assign(save.RESVRD.get_mut(8), b"UNDEFINE");
        fstr::assign(save.RESVRD.get_mut(9), b"HELP");
        fstr::assign(save.RESVRD.get_mut(10), b"RECALL");
        fstr::assign(save.RESVRD.get_mut(11), b"DO");
        fstr::assign(save.RESVRD.get_mut(12), b"EDIT");

        fstr::assign(&mut save.ALPHAB, b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    //
    // Find out what the special marker character is for suppressing
    // symbol evaluation.
    //
    GETEQ(&mut save.EQUOTE, ctx);

    //
    // Is this a definition statement? The presence of DEFINE, INQUIRE or
    // UNDEFINE at the beginning of the string will confirm this.
    //
    spicelib::NTHWD(INPUT, 1, &mut save.KEY, &mut save.LOC);
    spicelib::UCASE(&save.KEY.to_vec(), &mut save.KEY, ctx);

    //
    // The keyword must be followed by a valid symbol name.
    //
    if ((fstr::eq(&save.KEY, b"DEFINE") || fstr::eq(&save.KEY, b"INQUIRE"))
        || fstr::eq(&save.KEY, b"UNDEFINE"))
    {
        spicelib::NTHWD(INPUT, 2, &mut save.SYMBL, &mut save.LOC);
        spicelib::UCASE(&save.SYMBL, &mut save.SYMBOL, ctx);
        save.L = spicelib::RTRIM(&save.SYMBOL);

        if fstr::eq(&save.SYMBOL, b" ") {
            fstr::assign(OUTPUT, b" ");
            *TRAN = false;
            spicelib::SETMSG(b"The \"#\" command must be followed by the name of the symbol that you want to #. ", ctx);
            spicelib::ERRCH(b"#", &save.KEY, ctx);
            spicelib::LCASE(&save.KEY.to_vec(), &mut save.KEY, ctx);
            spicelib::ERRCH(b"#", &save.KEY, ctx);
            spicelib::SIGERR(b"BAD_SYMBOL_SPEC", ctx)?;
            spicelib::CHKOUT(b"STRAN", ctx)?;
            return Ok(());
        } else if (intrinsics::INDEX(&save.ALPHAB, fstr::substr(&save.SYMBOL, 1..=1)) == 0) {
            fstr::assign(OUTPUT, b" ");
            *TRAN = false;
            spicelib::LCASE(&save.KEY.to_vec(), &mut save.KEY, ctx);

            spicelib::SETMSG(
                b"You cannot # \"#\".  Symbols must begin with a letter (A-Z) ",
                ctx,
            );
            spicelib::ERRCH(b"#", &save.KEY, ctx);
            spicelib::ERRCH(b"#", &save.SYMBOL, ctx);
            spicelib::SIGERR(b"BAD_SYMBOL_SPEC", ctx)?;
            spicelib::CHKOUT(b"STRAN", ctx)?;
            return Ok(());
        } else if (save.L > 32) {
            fstr::assign(OUTPUT, b" ");
            *TRAN = false;
            spicelib::LCASE(&save.KEY.to_vec(), &mut save.KEY, ctx);
            spicelib::SETMSG(
                b"You cannot # \"#...\".  Symbols may not be longer than 32 characters in length.",
                ctx,
            );
            spicelib::ERRCH(b"#", &save.KEY, ctx);
            spicelib::ERRCH(b"#", &save.SYMBOL, ctx);
            spicelib::SIGERR(b"BAD_SYMBOL_SPEC", ctx)?;
            spicelib::CHKOUT(b"STRAN", ctx)?;
            return Ok(());
        } else if fstr::eq(fstr::substr(&save.SYMBOL, save.L..=save.L), b"?") {
            fstr::assign(OUTPUT, b" ");
            *TRAN = false;
            spicelib::LCASE(&save.KEY.to_vec(), &mut save.KEY, ctx);

            spicelib::SETMSG(
                b"You cannot # \"#\".  Symbols may not end with a question mark \'?\'. ",
                ctx,
            );
            spicelib::ERRCH(b"#", &save.KEY, ctx);
            spicelib::ERRCH(b"#", &save.SYMBOL, ctx);
            spicelib::SIGERR(b"BAD_SYMBOL_SPEC", ctx)?;
            spicelib::CHKOUT(b"STRAN", ctx)?;
            return Ok(());
        } else if ((fstr::eq(&save.KEY, b"DEFINE") || fstr::eq(&save.KEY, b"INQUIRE"))
            && (spicelib::ISRCHC(&save.SYMBOL, NRES, save.RESVRD.as_arg()) > 0))
        {
            fstr::assign(OUTPUT, b" ");
            *TRAN = false;

            spicelib::SETMSG(
                b"The word \'#\' is a reserved word. You may not redefine it. ",
                ctx,
            );
            spicelib::ERRCH(b"#", &save.SYMBOL, ctx);
            spicelib::SIGERR(b"BAD_SYMBOL_SPEC", ctx)?;
            spicelib::CHKOUT(b"STRAN", ctx)?;
            return Ok(());
        }
    }

    if fstr::eq(&save.KEY, b"INQUIRE") {
        //
        // First of all we, can only INQUIRE for symbol definitions
        // if the program is not running in "batch" mode.
        //
        if BATCH(ctx) {
            spicelib::SETMSG(b"You\'ve attempted to INQUIRE for the value of a symbol while the program is running in \"batch\" mode. You can INQUIRE for a symbol value only if you are running in INTERACTIVE mode. ", ctx);
            spicelib::SIGERR(b"WRONG_MODE", ctx)?;
            spicelib::CHKOUT(b"STRAN", ctx)?;
            return Ok(());
        }
        //
        // See if there is anything following the symbol that is
        // to be defined.  This will be used as our prompt value.
        //
        save.NXTCHR = intrinsics::MAX0(&[
            (save.LOC + save.L),
            spicelib::NCPOS(INPUT, b" ", (save.LOC + save.L)),
        ]);

        if fstr::ne(fstr::substr(INPUT, save.NXTCHR..), b" ") {
            fstr::assign(&mut save.MYPRMT, fstr::substr(INPUT, save.NXTCHR..));
        } else {
            fstr::assign(&mut save.MYPRMT, b"Enter definition for");
            spicelib::SUFFIX(&save.SYMBOL, 1, &mut save.MYPRMT);
            spicelib::SUFFIX(b">", 1, &mut save.MYPRMT);
        }

        GETDEL(&mut save.DELIM, ctx);
        RDSTMN(&save.MYPRMT, &save.DELIM, &mut save.DEF, ctx)?;

        SBSET_1(
            &save.SYMBOL,
            &save.DEF,
            save.NAMES.as_arg_mut(),
            save.PTRS.as_slice_mut(),
            save.BUFFER.as_arg_mut(),
            ctx,
        )?;
    }
    //
    // If this is a definition, and the symbol already exists in the
    // symbol table, simply replace the existing definition with the
    // string following the symbol name. If this is a new symbol,
    // find the first symbol in the list that should follow the new
    // one. Move the rest of the symbols back, and insert the new one
    // at this point.
    //
    if fstr::eq(&save.KEY, b"DEFINE") {
        save.NXTCHR = intrinsics::MAX0(&[
            (save.LOC + save.L),
            spicelib::NCPOS(INPUT, b" ", (save.LOC + save.L)),
        ]);

        SBSET_1(
            &save.SYMBOL,
            fstr::substr(INPUT, save.NXTCHR..),
            save.NAMES.as_arg_mut(),
            save.PTRS.as_slice_mut(),
            save.BUFFER.as_arg_mut(),
            ctx,
        )?;
    }

    if (fstr::eq(&save.KEY, b"DEFINE") || fstr::eq(&save.KEY, b"INQUIRE")) {
        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"STRAN", ctx)?;
            return Ok(());
        }
        //
        // Now check for a recursive definition.  To do this we have
        // two parallel arrays to the NAMES array of the string
        // buffer.  The first array CHECK is used to indicate that
        // in the course of the definition resolution of the
        // new symbol, another symbol shows up.  The second array
        // called CHECKD indicats whether or not we have examined this
        // existing symbol to see if contains the newly created
        // symbol as part of its definition.
        //
        // So far we have nothing to check and haven't checked anything.
        //
        save.N = spicelib::CARDC(save.NAMES.as_arg(), ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = save.N;
            let m3__: i32 = 1;
            save.J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.CHECK[save.J] = false;
                save.CHECKD[save.J] = false;
                save.J += m3__;
            }
        }
        //
        // Find the location of our new symbol in the NAMES cell.
        //

        save.PLACE = spicelib::ISRCHC(&save.SYMBOL, save.N, save.NAMES.subarray(1));

        save.NEW = true;

        while save.NEW {
            //
            // Look up the definition currently associated with
            // the symbol we are checking.
            //
            SBGET_1(
                &save.SYMBOL,
                save.NAMES.as_arg(),
                save.PTRS.as_slice(),
                save.BUFFER.as_arg(),
                &mut save.DEF,
                &mut save.I,
                ctx,
            )?;

            save.J = 1;
            NTHUQW(
                &save.DEF,
                save.J,
                &save.EQUOTE,
                &mut save.SYMBOL,
                &mut save.LOC,
            );

            while (save.LOC > 0) {
                spicelib::UCASE(&save.SYMBOL.to_vec(), &mut save.SYMBOL, ctx);
                save.SLOT = spicelib::ISRCHC(&save.SYMBOL, save.N, save.NAMES.subarray(1));
                //
                // If the word is located in the same place as the
                // symbol we've just defined, we've introduced
                // a recursive symbol definition.  Remove this
                // symbol and diagnose the error.
                //
                if (save.SLOT == save.PLACE) {
                    fstr::assign(OUTPUT, b" ");
                    *TRAN = false;
                    fstr::assign(&mut save.SYMBOL, save.NAMES.get(save.PLACE));
                    SBREM_1(
                        &save.SYMBOL,
                        save.NAMES.as_arg_mut(),
                        save.PTRS.as_slice_mut(),
                        save.BUFFER.as_arg_mut(),
                        ctx,
                    )?;

                    spicelib::SETMSG(b"The definition of \'#\' is recursive.  Recursively defined symbol definitions are not allowed. ", ctx);

                    spicelib::ERRCH(b"#", &save.SYMBOL, ctx);
                    spicelib::SIGERR(b"RECURSIVE_SYMBOL", ctx)?;
                    spicelib::CHKOUT(b"STRAN", ctx)?;
                    return Ok(());
                } else if (save.SLOT > 0) {
                    //
                    // Otherwise if this word is in the names list
                    // we may need to check this symbol to see if
                    // it lists the just defined symbol in its definition.
                    //
                    if save.CHECKD[save.SLOT] {
                        save.CHECK[save.SLOT] = false;
                    } else {
                        save.CHECK[save.SLOT] = true;
                    }
                }
                //
                // Locate the next unquoted word in the definition.
                //
                save.J = (save.J + 1);
                NTHUQW(
                    &save.DEF,
                    save.J,
                    &save.EQUOTE,
                    &mut save.SYMBOL,
                    &mut save.LOC,
                );
            }
            //
            // See if there are any new items to check.  If there
            // are create a new value for symbol, and mark the
            // new item as being checked.
            //
            save.NEW = false;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.N;
                let m3__: i32 = 1;
                save.J = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (save.CHECK[save.J] && !save.NEW) {
                        fstr::assign(&mut save.SYMBOL, save.NAMES.get(save.J));
                        save.CHECK[save.J] = false;
                        save.CHECKD[save.J] = true;
                        save.NEW = true;
                    }

                    save.J += m3__;
                }
            }
        }
        //
        // If we get to this point, we have a new non-recursively
        // defined symbol.
        //
        fstr::assign(OUTPUT, b" ");
        *TRAN = false;
        spicelib::CHKOUT(b"STRAN", ctx)?;
        return Ok(());
    }

    //
    // If this is a deletion, and the symbol already exists in the
    // symbol table, simply move the symbols that follow toward the
    // front of the table.
    //
    if fstr::eq(&save.KEY, b"UNDEFINE") {
        SBREM_1(
            &save.SYMBOL,
            save.NAMES.as_arg_mut(),
            save.PTRS.as_slice_mut(),
            save.BUFFER.as_arg_mut(),
            ctx,
        )?;
        fstr::assign(OUTPUT, b" ");
        *TRAN = false;
        spicelib::CHKOUT(b"STRAN", ctx)?;
        return Ok(());
    }

    //
    // This is not a definition statement. Look for potential symbols.
    // Try to resolve the first symbol in the string by substituting the
    // corresponding definition for the existing symbol.
    //
    fstr::assign(OUTPUT, INPUT);
    *TRAN = false;
    save.J = 1;

    NTHUQW(
        OUTPUT,
        save.J,
        &save.EQUOTE,
        &mut save.SYMBOL,
        &mut save.LOC,
    );

    while (!*TRAN && fstr::ne(&save.SYMBOL, b" ")) {
        spicelib::UCASE(&save.SYMBOL.to_vec(), &mut save.SYMBOL, ctx);
        SBGET_1(
            &save.SYMBOL,
            save.NAMES.as_arg(),
            save.PTRS.as_slice(),
            save.BUFFER.as_arg(),
            &mut save.DEF,
            &mut save.I,
            ctx,
        )?;

        if (save.I > 0) {
            save.LSYM = spicelib::LASTNB(&save.SYMBOL);
            save.LDEF = (spicelib::LASTNB(&save.DEF) + 1);
            save.LOUT = spicelib::LASTNB(OUTPUT);
            save.LENO = intrinsics::LEN(OUTPUT);

            if (((save.LOUT - save.LSYM) + save.LDEF) > save.LENO) {
                *TRAN = false;

                spicelib::SETMSG(b"As a result of attempting to resolve the symbols in the input command, the command has overflowed the allocated memory. This is may be due to unintentionally using symbols that you had not intended to use.  You may protect portions of your string from symbol evaluation by enclosing that portion of your string between the character # as in \'DO #THIS PART WITHOUT SYMBOLS#\' . ", ctx);

                spicelib::ERRCH(b"#", &save.EQUOTE, ctx);
                spicelib::ERRCH(b"#", &save.EQUOTE, ctx);
                spicelib::ERRCH(b"#", &save.EQUOTE, ctx);
                spicelib::SIGERR(b"SYMBOL_OVERFLOW", ctx)?;
                spicelib::CHKOUT(b"STRAN", ctx)?;
                return Ok(());
            }

            spicelib::REPSUB(
                &OUTPUT.to_vec(),
                save.LOC,
                ((save.LOC + save.LSYM) - 1),
                fstr::substr(&save.DEF, 1..=save.LDEF),
                OUTPUT,
                ctx,
            )?;

            *TRAN = true;
        } else {
            save.J = (save.J + 1);
        }

        NTHUQW(
            OUTPUT,
            save.J,
            &save.EQUOTE,
            &mut save.SYMBOL,
            &mut save.LOC,
        );
    }

    spicelib::CHKOUT(b"STRAN", ctx)?;
    Ok(())
}

//
// The following entry point allows us to set up a search
// of defined symbols that match a wild-card pattern.  It must
// be called prior to getting any symbol definitions.
//
pub fn SYMPAT(INPUT: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.LSTTRY = 0;
    fstr::assign(&mut save.PATTRN, INPUT);
}

//
// The following entry point fetches the next symbol and its
// definition for the next SYMBOL whose name
// matches a previously supplied template via the entry point
// above --- SYMPAT.
//
// If there is no matching symbol, we get back blanks.  Note
// that no translation of the definition is performed.
//
pub fn SYMGET(INPUT: &mut [u8], OUTPUT: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(INPUT, b" ");
    fstr::assign(OUTPUT, b" ");
    save.N = spicelib::CARDC(save.NAMES.as_arg(), ctx)?;
    while (save.LSTTRY < save.N) {
        save.LSTTRY = (save.LSTTRY + 1);
        save.GOTONE = MATCHM(
            &save.NAMES[save.LSTTRY],
            &save.PATTRN,
            b"*",
            b"%",
            b"~",
            b"|",
            ctx,
        )?;

        if save.GOTONE {
            fstr::assign(&mut save.SYMBOL, save.NAMES.get(save.LSTTRY));
            fstr::assign(INPUT, save.NAMES.get(save.LSTTRY));

            SBGET_1(
                &save.SYMBOL,
                save.NAMES.as_arg(),
                save.PTRS.as_slice(),
                save.BUFFER.as_arg(),
                OUTPUT,
                &mut save.I,
                ctx,
            )?;
            return Ok(());
        }
    }
    Ok(())
}
