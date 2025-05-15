//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MXWDTH: i32 = 255;
pub const ROOMH: i32 = 15;
pub const ROOMT: i32 = 10;
pub const ROOMF: i32 = 10;
const WDSIZE: i32 = 32;
const T: i32 = 1;
const H: i32 = 2;
const F: i32 = 3;
const B: i32 = 4;
const P: i32 = 5;
const NSECN: i32 = P;

struct SaveVars {
    TITLE: ActualCharArray,
    HEADER: ActualCharArray,
    FOOTER: ActualCharArray,
    MYLINE: Vec<u8>,
    RESPNS: Vec<u8>,
    QUESTN: Vec<u8>,
    SECTN: Vec<u8>,
    PAGMRK: Vec<u8>,
    DOMARK: bool,
    PAGESZ: i32,
    PAGEWD: i32,
    PAGMLN: i32,
    QLENTH: i32,
    WFACTR: i32,
    FREQ: StackArray<i32, 5>,
    NEED: StackArray<i32, 5>,
    SIZE: StackArray<i32, 5>,
    VISIBL: StackArray<bool, 5>,
    KEEPSP: StackArray<bool, 5>,
    ROW: i32,
    PAGENO: i32,
    BODY: bool,
    DOPRMT: bool,
    DIDPMT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = ActualCharArray::new(MXWDTH, 1..=ROOMT);
        let mut HEADER = ActualCharArray::new(MXWDTH, 1..=ROOMH);
        let mut FOOTER = ActualCharArray::new(MXWDTH, 1..=ROOMF);
        let mut MYLINE = vec![b' '; MXWDTH as usize];
        let mut RESPNS = vec![b' '; MXWDTH as usize];
        let mut QUESTN = vec![b' '; MXWDTH as usize];
        let mut SECTN = vec![b' '; WDSIZE as usize];
        let mut PAGMRK = vec![b' '; WDSIZE as usize];
        let mut DOMARK: bool = false;
        let mut PAGESZ: i32 = 0;
        let mut PAGEWD: i32 = 0;
        let mut PAGMLN: i32 = 0;
        let mut QLENTH: i32 = 0;
        let mut WFACTR: i32 = 0;
        let mut FREQ = StackArray::<i32, 5>::new(1..=NSECN);
        let mut NEED = StackArray::<i32, 5>::new(1..=NSECN);
        let mut SIZE = StackArray::<i32, 5>::new(1..=NSECN);
        let mut VISIBL = StackArray::<bool, 5>::new(1..=NSECN);
        let mut KEEPSP = StackArray::<bool, 5>::new(1..=NSECN);
        let mut ROW: i32 = 0;
        let mut PAGENO: i32 = 0;
        let mut BODY: bool = false;
        let mut DOPRMT: bool = false;
        let mut DIDPMT: bool = false;

        PAGESZ = 24;
        PAGEWD = 80;
        {
            use f2rust_std::data::Val;

            let mut clist =
                [Val::I(-1), Val::I(-1), Val::I(-1), Val::I(-1), Val::I(-1)].into_iter();
            FREQ.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0), Val::I(0), Val::I(0), Val::I(0), Val::I(0)].into_iter();
            NEED.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0), Val::I(0), Val::I(0), Val::I(0), Val::I(0)].into_iter();
            SIZE.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        ROW = 0;
        PAGENO = 0;
        BODY = true;
        DOMARK = false;
        DOPRMT = false;
        DIDPMT = false;
        WFACTR = 0;
        fstr::assign(&mut SECTN, b"BODY");
        fstr::assign(&mut RESPNS, b" ");

        Self {
            TITLE,
            HEADER,
            FOOTER,
            MYLINE,
            RESPNS,
            QUESTN,
            SECTN,
            PAGMRK,
            DOMARK,
            PAGESZ,
            PAGEWD,
            PAGMLN,
            QLENTH,
            WFACTR,
            FREQ,
            NEED,
            SIZE,
            VISIBL,
            KEEPSP,
            ROW,
            PAGENO,
            BODY,
            DOPRMT,
            DIDPMT,
        }
    }
}

//$Procedure PAGMAN (Page Manager)
pub fn PAGMAN(WHICH: &[u8], LINE: &[u8], VALUE: i32) {

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //
    // The arrays TITLE, HEADER and FOOTER are used to store the
    // text that will be written to the TITLE, HEADER and FOOTER
    // sections of a page.
    //
    //
    // The variable RESPNS is used to keep track of any response
    // that the user may supply to a prompt that can be triggered
    // at the completion of a page.
    //

    //
    // The variable SECTN contains the name of the section to which
    // lines of text should be sent.
    //
    //
    // The array FREQ is used to store the
    // frequency with which footers, headers and titles should
    // be displayed PAGESZ and PAGEWD give the size of the page
    // in height and width.
    //
    // The array SIZE is  used to maintain the
    // size of the TITLE, HEADER, BODY, and FOOTER sections.
    //
    // The array NEED is used to determine how many lines
    // need to be devoted to the TITLE, HEADER and FOOTER section
    // on a page (the value will be a function of FREQ, the page
    // number and the array KEEPSP)
    //
    // The array KEEPSP is used to store whether or not sections
    // should be kept but presented as white space when the
    // page number and frequency imply that the section should
    // not be printed on a given page.
    //
    // The array INVIS is used to keep track of whether or not
    // a section should be visible on the current page.
    //

    //
    // The variable ROW points to the position of the last
    // row in the body portion of the page where text was last
    // written.  PAGENO is the page number of the page that is
    // currently being filled.
    //
    //
    // The logical BODY is used to indicate whether the section
    // has been set to BODY since the last call to PAGRST to reset
    // the dynamic page attributes.
    //

    //
    // The logical DOPRMT is used to indicate whether or not a prompt
    // should be issued when the production of a page is finished.
    //

    //
    // Loop counter
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //
}

//$Procedure PAGRST (Page Reset)
pub fn PAGRST(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.ROW = 0;
    save.PAGENO = 1;
    save.SIZE[H] = 0;
    save.SIZE[F] = 0;
    save.SIZE[T] = 0;
    save.SIZE[P] = 0;
    save.DOPRMT = false;
    save.DIDPMT = false;
    fstr::assign(&mut save.RESPNS, b" ");
    save.WFACTR = 0;
    save.BODY = false;
}

//$Procedure PAGSFT (Page Soft Reset)
pub fn PAGSFT(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if (save.ROW > 0) {
        fstr::assign(&mut save.MYLINE, b" ");

        while (save.ROW < save.SIZE[B]) {
            NSPWLN(fstr::substr(&save.MYLINE, 1..=save.PAGEWD), ctx)?;
            save.ROW = (save.ROW + 1);
        }

        //
        // The user may want to have the page number appear
        // in the footer.  So we replace the PAGMRK by the
        // number if this is the case.
        //
        for I in 1..=save.NEED[F] {
            if save.VISIBL[F] {
                spicelib::REPMI(
                    &save.FOOTER[I],
                    &save.PAGMRK,
                    save.PAGENO,
                    &mut save.MYLINE,
                    ctx,
                );
                NSPWLN(fstr::substr(&save.MYLINE, 1..=save.PAGEWD), ctx)?;
            } else {
                NSPWLN(fstr::substr(&save.MYLINE, 1..=save.PAGEWD), ctx)?;
            }
        }

        save.PAGENO = (save.PAGENO + 1);
    }

    save.ROW = 0;
    save.SIZE[H] = 0;
    save.SIZE[F] = 0;
    save.SIZE[T] = 0;
    save.SIZE[P] = 0;
    save.DOPRMT = false;
    save.DIDPMT = false;
    fstr::assign(&mut save.RESPNS, b" ");
    save.WFACTR = 0;
    save.BODY = false;

    Ok(())
}

//$Procedure      PAGSET (Page Set attributes )
pub fn PAGSET(WHICH: &[u8], VALUE: i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if fstr::eq(WHICH, b"PAGEHEIGHT") {
        save.PAGESZ = VALUE;
    } else if fstr::eq(WHICH, b"PAGEWIDTH") {
        save.PAGEWD = VALUE;
    } else if fstr::eq(WHICH, b"HEADERFREQUENCY") {
        save.FREQ[H] = VALUE;
    } else if fstr::eq(WHICH, b"TITLEFREQUENCY") {
        save.FREQ[T] = VALUE;
    } else if fstr::eq(WHICH, b"FOOTERFREQUENCY") {
        save.FREQ[F] = VALUE;
    } else if fstr::eq(WHICH, b"SPACETITLE") {
        save.KEEPSP[T] = true;
    } else if fstr::eq(WHICH, b"NOSPACETITLE") {
        save.KEEPSP[T] = false;
    } else if fstr::eq(WHICH, b"SPACEHEADER") {
        save.KEEPSP[H] = true;
    } else if fstr::eq(WHICH, b"NOSPACEHEADER") {
        save.KEEPSP[H] = false;
    } else if fstr::eq(WHICH, b"SPACEFOOTER") {
        save.KEEPSP[F] = true;
    } else if fstr::eq(WHICH, b"NOSPACEFOOTER") {
        save.KEEPSP[F] = false;
    } else if fstr::eq(WHICH, b"NOPAGEMARK") {
        save.DOMARK = false;
    } else if fstr::eq(WHICH, b"DOPAGEMARK") {
        save.DOMARK = true;
    } else if fstr::eq(WHICH, b"PROMPT") {
        save.DOPRMT = true;
        save.WFACTR = 1;
        save.SIZE[P] = 1;
    } else if fstr::eq(WHICH, b"NOPROMPT") {
        save.DOPRMT = false;
        save.DIDPMT = false;
        fstr::assign(&mut save.RESPNS, b" ");
        save.WFACTR = 0;
        save.SIZE[P] = 0;
    }
}

//$Procedure      PAGSMK (Page set page number marker )
pub fn PAGSMK(WHICH: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.PAGMRK, WHICH);
    save.PAGMLN = spicelib::RTRIM(&save.PAGMRK);
    save.DOMARK = true;
}

//$Procedure      PAGSCN (Page Section)
pub fn PAGSCN(WHICH: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(&mut save.SECTN, WHICH);
    save.BODY = fstr::eq(&save.SECTN, b"BODY");
}

//$Procedure PAGPUT (Page put a line of text )
pub fn PAGPUT(LINE: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // We handle the TITLE, HEADER and FOOTER sections first.
    //
    save.DIDPMT = false;

    if !save.BODY {
        if fstr::eq(&save.SECTN, b"TITLE") {
            save.SIZE[T] = intrinsics::MIN0(&[ROOMT, (save.SIZE[T] + 1)]);
            fstr::assign(save.TITLE.get_mut(save.SIZE[T]), LINE);
        } else if fstr::eq(&save.SECTN, b"HEADER") {
            save.SIZE[H] = intrinsics::MIN0(&[ROOMH, (save.SIZE[H] + 1)]);
            fstr::assign(save.HEADER.get_mut(save.SIZE[H]), LINE);
        } else if fstr::eq(&save.SECTN, b"FOOTER") {
            save.SIZE[F] = intrinsics::MIN0(&[ROOMF, (save.SIZE[F] + 1)]);
            fstr::assign(save.FOOTER.get_mut(save.SIZE[F]), LINE);
        } else if fstr::eq(&save.SECTN, b"PROMPT") {
            save.SIZE[P] = 1;
            fstr::assign(&mut save.QUESTN, LINE);
            save.QLENTH = (spicelib::RTRIM(LINE) + 1);
        }

        return Ok(());
    }
    //
    // The only way to get to this point is if we are working on
    // the body section of a page.  If the row number is zero, then
    // we need to see how much room is available on this page for
    // the body.  And, if appropriate output the TITLE and
    // HEADER sections of this page.
    //
    if (save.ROW == 0) {
        //
        // We need to compute how much room is available
        // for the body of this page.
        //
        for I in T..=F {
            //
            // First determine how much room is needed for
            // this section and whether or not it will be
            // visible on this page if we simply fill it with
            // blanks.
            //
            if (save.FREQ[I] < 0) {
                save.NEED[I] = 0;
                save.VISIBL[I] = false;
            } else if (save.PAGENO == 1) {
                save.NEED[I] = save.SIZE[I];
                save.VISIBL[I] = true;
            } else if (save.FREQ[I] == 0) {
                save.NEED[I] = 0;
                save.VISIBL[I] = true;
            } else if (save.FREQ[I] == 1) {
                save.NEED[I] = save.SIZE[I];
                save.VISIBL[I] = true;
            } else if (intrinsics::MOD(save.PAGENO, save.FREQ[I]) == 1) {
                save.NEED[I] = save.SIZE[I];
                save.VISIBL[I] = true;
            } else {
                save.NEED[I] = 0;
                save.VISIBL[I] = true;
            }

            if save.KEEPSP[I] {
                save.NEED[I] = save.SIZE[I];
            }
        }

        save.SIZE[B] = ((((save.PAGESZ - save.NEED[T]) - save.NEED[H]) - save.NEED[F])
            - (save.WFACTR * save.SIZE[P]));
        //
        // We haven't yet written a line in the body of the
        // page, we will write out the title and header sections
        // (provided we are on the right page number)
        //
        // We allow for the possibility that the user might
        // place the page number in the title section.
        //
        fstr::assign(&mut save.MYLINE, b" ");

        for I in 1..=save.NEED[T] {
            if save.VISIBL[T] {
                if save.DOMARK {
                    spicelib::REPMI(
                        &save.TITLE[I],
                        fstr::substr(&save.PAGMRK, 1..=save.PAGMLN),
                        save.PAGENO,
                        &mut save.MYLINE,
                        ctx,
                    );
                    NSPWLN(fstr::substr(&save.MYLINE, 1..=save.PAGEWD), ctx)?;
                } else {
                    NSPWLN(fstr::substr(&save.TITLE[I], 1..=save.PAGEWD), ctx)?;
                }
            } else {
                NSPWLN(fstr::substr(&save.MYLINE, 1..=save.PAGEWD), ctx)?;
            }
        }
        //
        // Next output whatever portion of the header section is
        // appropriate.
        //
        fstr::assign(&mut save.MYLINE, b" ");

        for I in 1..=save.NEED[H] {
            if save.VISIBL[H] {
                NSPWLN(fstr::substr(&save.HEADER[I], 1..=save.PAGEWD), ctx)?;
            } else {
                NSPWLN(fstr::substr(&save.MYLINE, 1..=save.PAGEWD), ctx)?;
            }
        }
    }
    //
    // Write the line and update the number of lines we
    // have written so far.
    //
    save.ROW = (save.ROW + 1);
    fstr::assign(&mut save.MYLINE, LINE);
    NSPWLN(fstr::substr(&save.MYLINE, 1..=save.PAGEWD), ctx)?;

    //
    // If we reached the end of the body section, write out
    // the footer (provided we are on the right page). And
    // update the page number.
    //
    if (save.ROW == save.SIZE[B]) {
        //
        // The user may want to have the page number appear
        // in the footer.  So we replace the PAGMRK by the
        // number if this is the case.
        //
        fstr::assign(&mut save.MYLINE, b" ");

        for I in 1..=save.NEED[F] {
            if save.VISIBL[F] {
                if save.DOMARK {
                    spicelib::REPMI(
                        &save.FOOTER[I],
                        fstr::substr(&save.PAGMRK, 1..=save.PAGMLN),
                        save.PAGENO,
                        &mut save.MYLINE,
                        ctx,
                    );
                    NSPWLN(fstr::substr(&save.MYLINE, 1..=save.PAGEWD), ctx)?;
                } else {
                    NSPWLN(fstr::substr(&save.FOOTER[I], 1..=save.PAGEWD), ctx)?;
                }
            } else {
                NSPWLN(fstr::substr(&save.MYLINE, 1..=save.PAGEWD), ctx)?;
            }
        }
        //
        // Advance the page number and reset the row to zero.
        // (we won't have written anything in the body of the
        // next page until later.)
        //
        save.PAGENO = (save.PAGENO + 1);
        save.ROW = 0;

        if save.DOPRMT {
            spicelib::PROMPT(
                fstr::substr(&save.QUESTN, 1..=save.QLENTH),
                &mut save.RESPNS,
                ctx,
            )?;
            save.DIDPMT = true;
        }
    }

    Ok(())
}

//$Procedure PAGPMT ( Page prompt returned )
pub fn PAGPMT(VALUE: &mut i32, LINE: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if save.DIDPMT {
        *VALUE = 1;
        fstr::assign(LINE, &save.RESPNS);
    } else {
        *VALUE = 0;
        fstr::assign(LINE, b" ");
    }
}
