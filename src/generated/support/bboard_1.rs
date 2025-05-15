//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MAXNL: i32 = 32;
pub const MAXCL: i32 = 255;
pub const MAXI: i32 = 100;
pub const MAXD: i32 = 100;
pub const MAXC: i32 = 100;
pub const MAXS: i32 = 100;
pub const MAXIV: i32 = 5000;
pub const MAXDV: i32 = 5000;
pub const MAXCV: i32 = 300;
pub const MAXCHR: i32 = 5000;
const LBCELL: i32 = -5;
const LBCBUF: i32 = 0;

struct SaveVars {
    INTAB: ActualCharArray,
    IPTAB: StackArray<i32, 106>,
    IVTAB: ActualArray<i32>,
    DNTAB: ActualCharArray,
    DPTAB: StackArray<i32, 106>,
    DVTAB: ActualArray<f64>,
    CNTAB: ActualCharArray,
    CPTAB: StackArray<i32, 106>,
    CVTAB: ActualCharArray,
    NBUF: ActualCharArray,
    PBUF: ActualArray<i32>,
    VBUF: ActualCharArray,
    WHAT: Vec<u8>,
    WHICH: Vec<u8>,
    POS: i32,
    FND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut INTAB = ActualCharArray::new(MAXNL, LBCELL..=MAXI);
        let mut IPTAB = StackArray::<i32, 106>::new(LBCELL..=MAXI);
        let mut IVTAB = ActualArray::<i32>::new(LBCELL..=MAXIV);
        let mut DNTAB = ActualCharArray::new(MAXNL, LBCELL..=MAXD);
        let mut DPTAB = StackArray::<i32, 106>::new(LBCELL..=MAXD);
        let mut DVTAB = ActualArray::<f64>::new(LBCELL..=MAXDV);
        let mut CNTAB = ActualCharArray::new(MAXNL, LBCELL..=MAXC);
        let mut CPTAB = StackArray::<i32, 106>::new(LBCELL..=MAXC);
        let mut CVTAB = ActualCharArray::new(MAXCL, LBCELL..=MAXCV);
        let mut NBUF = ActualCharArray::new(MAXNL, LBCELL..=MAXS);
        let mut PBUF = ActualArray::<i32>::new(LBCELL..=((MAXS * 4) + 4));
        let mut VBUF = ActualCharArray::new(100, LBCBUF..=(MAXCHR / 100));
        let mut WHAT = vec![b' '; MAXNL as usize];
        let mut WHICH = vec![b' '; MAXNL as usize];
        let mut POS: i32 = 0;
        let mut FND: bool = false;

        Self {
            INTAB,
            IPTAB,
            IVTAB,
            DNTAB,
            DPTAB,
            DVTAB,
            CNTAB,
            CPTAB,
            CVTAB,
            NBUF,
            PBUF,
            VBUF,
            WHAT,
            WHICH,
            POS,
            FND,
        }
    }
}

//$Procedure BBOARD ( Bulletin board )
pub fn BBOARD(
    ACTION: &[u8],
    ITEM: &[u8],
    N: i32,
    IVALS: &[i32],
    DVALS: &[f64],
    CVALS: CharArray,
    SVAL: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local variables
    //
    // Integer, DP, and character items are stored in symbol tables.
    // Later, they should be stored in card catalogs, when the necessary
    // routines have been completed.
    //
    // Strings are stored in a string buffer.
    //
    // Actions, where input, are compressed and converted to uppercase
    // (WHAT). Item names are compressed (WHICH).
    //

    //
    // Save everything between calls.
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBOARD", ctx)?;
    }

    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    spicelib::CHKOUT(b"BBOARD", ctx)?;
    Ok(())
}

//$Procedure BBPUTI ( Bulletin board, put, integer )
pub fn BBPUTI_1(
    ACTION: &[u8],
    ITEM: &[u8],
    N: i32,
    IVALS: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let IVALS = DummyArray::new(IVALS, 1..);

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBPUTI_1", ctx)?;
    }

    //
    // Compress spaces, change cases, as needed.
    //
    spicelib::CMPRSS(b" ", 0, ACTION, &mut save.WHAT);
    spicelib::UCASE(&save.WHAT.to_vec(), &mut save.WHAT, ctx);

    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by the symbol table routines. (Later,
    // it will be done by the card catalog routines.) Note that
    // items must be pushed and appended one at a time.
    //
    if fstr::eq(&save.WHAT, b"POST") {
        spicelib::SYPUTI(
            &save.WHICH,
            IVALS.as_slice(),
            N,
            save.INTAB.as_arg_mut(),
            save.IPTAB.as_slice_mut(),
            save.IVTAB.as_slice_mut(),
            ctx,
        )?;
    } else if fstr::eq(&save.WHAT, b"PUSH") {
        for I in intrinsics::range(N, 1, -1) {
            spicelib::SYPSHI(
                &save.WHICH,
                IVALS[I],
                save.INTAB.as_arg_mut(),
                save.IPTAB.as_slice_mut(),
                save.IVTAB.as_slice_mut(),
                ctx,
            )?;
        }
    } else if fstr::eq(&save.WHAT, b"APPEND") {
        for I in 1..=N {
            spicelib::SYENQI(
                &save.WHICH,
                IVALS[I],
                save.INTAB.as_arg_mut(),
                save.IPTAB.as_slice_mut(),
                save.IVTAB.as_slice_mut(),
                ctx,
            )?;
        }
    } else {
        spicelib::SETMSG(b"Sorry, # is not a legal action.", ctx);
        spicelib::ERRCH(b"#", &save.WHAT, ctx);
        spicelib::SIGERR(b"SPICE(UNNATURALACT)", ctx)?;
    }

    spicelib::CHKOUT(b"BBPUTI_1", ctx)?;
    Ok(())
}

//$Procedure BBPUTD ( Bulletin board, put, DP )
pub fn BBPUTD_1(
    ACTION: &[u8],
    ITEM: &[u8],
    N: i32,
    DVALS: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DVALS = DummyArray::new(DVALS, 1..);

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBPUTD_1", ctx)?;
    }

    //
    // Compress spaces, change cases, as needed.
    //
    spicelib::CMPRSS(b" ", 0, ACTION, &mut save.WHAT);
    spicelib::UCASE(&save.WHAT.to_vec(), &mut save.WHAT, ctx);

    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by the symbol table routines. (Later,
    // it will be done by the card catalog routines.) Note that
    // items must be pushed and appended one at a time.
    //
    if fstr::eq(&save.WHAT, b"POST") {
        spicelib::SYPUTD(
            &save.WHICH,
            DVALS.as_slice(),
            N,
            save.DNTAB.as_arg_mut(),
            save.DPTAB.as_slice_mut(),
            save.DVTAB.as_slice_mut(),
            ctx,
        )?;
    } else if fstr::eq(&save.WHAT, b"PUSH") {
        for I in intrinsics::range(N, 1, -1) {
            spicelib::SYPSHD(
                &save.WHICH,
                DVALS[I],
                save.DNTAB.as_arg_mut(),
                save.DPTAB.as_slice_mut(),
                save.DVTAB.as_slice_mut(),
                ctx,
            )?;
        }
    } else if fstr::eq(&save.WHAT, b"APPEND") {
        for I in 1..=N {
            spicelib::SYENQD(
                &save.WHICH,
                DVALS[I],
                save.DNTAB.as_arg_mut(),
                save.DPTAB.as_slice_mut(),
                save.DVTAB.as_slice_mut(),
                ctx,
            )?;
        }
    } else {
        spicelib::SETMSG(b"Sorry, # is not a legal action.", ctx);
        spicelib::ERRCH(b"#", &save.WHAT, ctx);
        spicelib::SIGERR(b"SPICE(UNNATURALACT)", ctx)?;
    }

    spicelib::CHKOUT(b"BBPUTD_1", ctx)?;
    Ok(())
}

//$Procedure BBPUTC ( Bulletin board, put, character )
pub fn BBPUTC_1(
    ACTION: &[u8],
    ITEM: &[u8],
    N: i32,
    CVALS: CharArray,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let CVALS = DummyCharArray::new(CVALS, None, 1..);

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBPUTC_1", ctx)?;
    }

    //
    // Compress spaces, change cases, as needed.
    //
    spicelib::CMPRSS(b" ", 0, ACTION, &mut save.WHAT);
    spicelib::UCASE(&save.WHAT.to_vec(), &mut save.WHAT, ctx);

    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by the symbol table routines. (Later,
    // it will be done by the card catalog routines.) Note that
    // items must be pushed and appended one at a time.
    //
    if fstr::eq(&save.WHAT, b"POST") {
        spicelib::SYPUTC(
            &save.WHICH,
            CVALS.as_arg(),
            N,
            save.CNTAB.as_arg_mut(),
            save.CPTAB.as_slice_mut(),
            save.CVTAB.as_arg_mut(),
            ctx,
        )?;
    } else if fstr::eq(&save.WHAT, b"PUSH") {
        for I in intrinsics::range(N, 1, -1) {
            spicelib::SYPSHC(
                &save.WHICH,
                &CVALS[I],
                save.CNTAB.as_arg_mut(),
                save.CPTAB.as_slice_mut(),
                save.CVTAB.as_arg_mut(),
                ctx,
            )?;
        }
    } else if fstr::eq(&save.WHAT, b"APPEND") {
        for I in 1..=N {
            spicelib::SYENQC(
                &save.WHICH,
                &CVALS[I],
                save.CNTAB.as_arg_mut(),
                save.CPTAB.as_slice_mut(),
                save.CVTAB.as_arg_mut(),
                ctx,
            )?;
        }
    } else {
        spicelib::SETMSG(b"Sorry, # is not a legal action.", ctx);
        spicelib::ERRCH(b"#", &save.WHAT, ctx);
        spicelib::SIGERR(b"SPICE(UNNATURALACT)", ctx)?;
    }

    spicelib::CHKOUT(b"BBPUTC_1", ctx)?;
    Ok(())
}

//$Procedure BBPUTS ( Bulletin board, put, string )
pub fn BBPUTS_1(
    ACTION: &[u8],
    ITEM: &[u8],
    SVAL: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBPUTS_1", ctx)?;
    }

    spicelib::CMPRSS(b" ", 0, ACTION, &mut save.WHAT);
    spicelib::UCASE(&save.WHAT.to_vec(), &mut save.WHAT, ctx);

    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    if fstr::eq(&save.WHAT, b"POST") {
        SBSET_1(
            &save.WHICH,
            SVAL,
            save.NBUF.as_arg_mut(),
            save.PBUF.as_slice_mut(),
            save.VBUF.as_arg_mut(),
            ctx,
        )?;
    } else {
        spicelib::SETMSG(b"Sorry, # is not a legal action.", ctx);
        spicelib::ERRCH(b"#", &save.WHAT, ctx);
        spicelib::SIGERR(b"SPICE(UNNATURALACT)", ctx)?;
    }

    spicelib::CHKOUT(b"BBPUTS_1", ctx)?;
    Ok(())
}

pub fn BBGETI_1(
    ACTION: &[u8],
    ITEM: &[u8],
    N: &mut i32,
    IVALS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut IVALS = DummyArrayMut::new(IVALS, 1..);

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBGETI_1", ctx)?;
    }

    //
    // Compress spaces, change cases, as needed.
    //
    spicelib::CMPRSS(b" ", 0, ACTION, &mut save.WHAT);
    spicelib::UCASE(&save.WHAT.to_vec(), &mut save.WHAT, ctx);

    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by the symbol table routines. (Later,
    // it will be done by the card catalog routines.) Note that
    // items must be popped one at a time.
    //
    if (fstr::eq(&save.WHAT, b"COPY") || fstr::eq(&save.WHAT, b"TAKE")) {
        spicelib::SYGETI(
            &save.WHICH,
            save.INTAB.as_arg(),
            save.IPTAB.as_slice(),
            save.IVTAB.as_slice(),
            N,
            IVALS.as_slice_mut(),
            &mut save.FND,
            ctx,
        )?;

        if !save.FND {
            spicelib::SETMSG(b"Could not find item #.", ctx);
            spicelib::ERRCH(b"#", &save.WHICH, ctx);
            spicelib::SIGERR(b"SPICE(ALLGONE)", ctx)?;
        } else if fstr::eq(&save.WHAT, b"TAKE") {
            spicelib::SYDELI(
                &save.WHICH,
                save.INTAB.as_arg_mut(),
                save.IPTAB.as_slice_mut(),
                save.IVTAB.as_slice_mut(),
                ctx,
            )?;
        }
    } else if fstr::eq(&save.WHAT, b"POP") {
        for I in 1..=*N {
            spicelib::SYPOPI(
                &save.WHICH,
                save.INTAB.as_arg_mut(),
                save.IPTAB.as_slice_mut(),
                save.IVTAB.as_slice_mut(),
                &mut IVALS[I],
                &mut save.FND,
                ctx,
            )?;
        }

        if !save.FND {
            spicelib::SETMSG(b"Could not find item #.", ctx);
            spicelib::ERRCH(b"#", &save.WHICH, ctx);
            spicelib::SIGERR(b"SPICE(ALLGONE)", ctx)?;
        }
    } else {
        spicelib::SETMSG(b"Sorry, # is not a legal action.", ctx);
        spicelib::ERRCH(b"#", &save.WHAT, ctx);
        spicelib::SIGERR(b"SPICE(UNNATURALACT)", ctx)?;
    }

    spicelib::CHKOUT(b"BBGETI_1", ctx)?;
    Ok(())
}

//$Procedure BBGETD ( Bulletin board, get, DP )
pub fn BBGETD_1(
    ACTION: &[u8],
    ITEM: &[u8],
    N: &mut i32,
    DVALS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DVALS = DummyArrayMut::new(DVALS, 1..);

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBGETD_1", ctx)?;
    }

    //
    // Compress spaces, change cases, as needed.
    //
    spicelib::CMPRSS(b" ", 0, ACTION, &mut save.WHAT);
    spicelib::UCASE(&save.WHAT.to_vec(), &mut save.WHAT, ctx);

    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by the symbol table routines. (Later,
    // it will be done by the card catalog routines.) Note that
    // items must be popped one at a time.
    //
    if (fstr::eq(&save.WHAT, b"COPY") || fstr::eq(&save.WHAT, b"TAKE")) {
        spicelib::SYGETD(
            &save.WHICH,
            save.DNTAB.as_arg(),
            save.DPTAB.as_slice(),
            save.DVTAB.as_slice(),
            N,
            DVALS.as_slice_mut(),
            &mut save.FND,
            ctx,
        )?;

        if !save.FND {
            spicelib::SETMSG(b"Could not find item #.", ctx);
            spicelib::ERRCH(b"#", &save.WHICH, ctx);
            spicelib::SIGERR(b"SPICE(ALLGONE)", ctx)?;
        } else if fstr::eq(&save.WHAT, b"TAKE") {
            spicelib::SYDELD(
                &save.WHICH,
                save.DNTAB.as_arg_mut(),
                save.DPTAB.as_slice_mut(),
                save.DVTAB.as_slice_mut(),
                ctx,
            )?;
        }
    } else if fstr::eq(&save.WHAT, b"POP") {
        for I in 1..=*N {
            spicelib::SYPOPD(
                &save.WHICH,
                save.DNTAB.as_arg_mut(),
                save.DPTAB.as_slice_mut(),
                save.DVTAB.as_slice_mut(),
                &mut DVALS[I],
                &mut save.FND,
                ctx,
            )?;
        }

        if !save.FND {
            spicelib::SETMSG(b"Could not find item #.", ctx);
            spicelib::ERRCH(b"#", &save.WHICH, ctx);
            spicelib::SIGERR(b"SPICE(ALLGONE)", ctx)?;
        }
    } else {
        spicelib::SETMSG(b"Sorry, # is not a legal action.", ctx);
        spicelib::ERRCH(b"#", &save.WHAT, ctx);
        spicelib::SIGERR(b"SPICE(UNNATURALACT)", ctx)?;
    }

    spicelib::CHKOUT(b"BBGETD_1", ctx)?;
    Ok(())
}

//$Procedure BBGETC ( Bulletin board, get, character )
pub fn BBGETC_1(
    ACTION: &[u8],
    ITEM: &[u8],
    N: &mut i32,
    CVALS: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut CVALS = DummyCharArrayMut::new(CVALS, None, 1..);

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBGETC_1", ctx)?;
    }

    //
    // Compress spaces, change cases, as needed.
    //
    spicelib::CMPRSS(b" ", 0, ACTION, &mut save.WHAT);
    spicelib::UCASE(&save.WHAT.to_vec(), &mut save.WHAT, ctx);

    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by the symbol table routines. (Later,
    // it will be done by the card catalog routines.) Note that
    // items must be popped one at a time.
    //
    if (fstr::eq(&save.WHAT, b"COPY") || fstr::eq(&save.WHAT, b"TAKE")) {
        spicelib::SYGETC(
            &save.WHICH,
            save.CNTAB.as_arg(),
            save.CPTAB.as_slice(),
            save.CVTAB.as_arg(),
            N,
            CVALS.as_arg_mut(),
            &mut save.FND,
            ctx,
        )?;

        if !save.FND {
            spicelib::SETMSG(b"Could not find item #.", ctx);
            spicelib::ERRCH(b"#", &save.WHICH, ctx);
            spicelib::SIGERR(b"SPICE(ALLGONE)", ctx)?;
        } else if fstr::eq(&save.WHAT, b"TAKE") {
            spicelib::SYDELC(
                &save.WHICH,
                save.CNTAB.as_arg_mut(),
                save.CPTAB.as_slice_mut(),
                save.CVTAB.as_arg_mut(),
                ctx,
            )?;
        }
    } else if fstr::eq(&save.WHAT, b"POP") {
        for I in 1..=*N {
            spicelib::SYPOPC(
                &save.WHICH,
                save.CNTAB.as_arg_mut(),
                save.CPTAB.as_slice_mut(),
                save.CVTAB.as_arg_mut(),
                &mut CVALS[I],
                &mut save.FND,
                ctx,
            )?;
        }

        if !save.FND {
            spicelib::SETMSG(b"Could not find item #.", ctx);
            spicelib::ERRCH(b"#", &save.WHICH, ctx);
            spicelib::SIGERR(b"SPICE(ALLGONE)", ctx)?;
        }
    } else {
        spicelib::SETMSG(b"Sorry, # is not a legal action.", ctx);
        spicelib::ERRCH(b"#", &save.WHAT, ctx);
        spicelib::SIGERR(b"SPICE(UNNATURALACT)", ctx)?;
    }

    spicelib::CHKOUT(b"BBGETC_1", ctx)?;
    Ok(())
}

//$Procedure BBGETS ( Bulletin board, get, string )
pub fn BBGETS_1(
    ACTION: &[u8],
    ITEM: &[u8],
    SVAL: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBGETS_1", ctx)?;
    }

    spicelib::CMPRSS(b" ", 0, ACTION, &mut save.WHAT);
    spicelib::UCASE(&save.WHAT.to_vec(), &mut save.WHAT, ctx);

    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    if (fstr::eq(&save.WHAT, b"COPY") || fstr::eq(&save.WHAT, b"TAKE")) {
        SBGET_1(
            &save.WHICH,
            save.NBUF.as_arg(),
            save.PBUF.as_slice(),
            save.VBUF.as_arg(),
            SVAL,
            &mut save.POS,
            ctx,
        )?;

        if (save.POS == 0) {
            spicelib::SETMSG(b"Could not find item #.", ctx);
            spicelib::ERRCH(b"#", &save.WHICH, ctx);
            spicelib::SIGERR(b"SPICE(ALLGONE)", ctx)?;
        } else if fstr::eq(&save.WHAT, b"TAKE") {
            SBREM_1(
                &save.WHICH,
                save.NBUF.as_arg_mut(),
                save.PBUF.as_slice_mut(),
                save.VBUF.as_arg_mut(),
                ctx,
            )?;
        }
    } else {
        spicelib::SETMSG(b"Sorry, # is not a legal action.", ctx);
        spicelib::ERRCH(b"#", &save.WHAT, ctx);
        spicelib::SIGERR(b"SPICE(UNNATURALACT)", ctx)?;
    }

    spicelib::CHKOUT(b"BBGETS_1", ctx)?;
    Ok(())
}

//$Procedure BBREMI ( Bulletin board, remove, integer )
pub fn BBREMI_1(ITEM: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBREMI_1", ctx)?;
    }

    //
    // Compress spaces as needed.
    //
    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by a symbol table routine. (Later,
    // it will be done by a card catalog routine.)
    //
    spicelib::SYDELI(
        &save.WHICH,
        save.INTAB.as_arg_mut(),
        save.IPTAB.as_slice_mut(),
        save.IVTAB.as_slice_mut(),
        ctx,
    )?;

    spicelib::CHKOUT(b"BBREMI_1", ctx)?;
    Ok(())
}

//$Procedure BBREMD ( Bulletin board, remove, DP )
pub fn BBREMD_1(ITEM: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBREMD_1", ctx)?;
    }

    //
    // Compress spaces as needed.
    //
    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by a symbol table routine. (Later,
    // it will be done by a card catalog routine.)
    //
    spicelib::SYDELD(
        &save.WHICH,
        save.DNTAB.as_arg_mut(),
        save.DPTAB.as_slice_mut(),
        save.DVTAB.as_slice_mut(),
        ctx,
    )?;

    spicelib::CHKOUT(b"BBREMD_1", ctx)?;
    Ok(())
}

//$Procedure BBREMC ( Bulletin board, remove, character )
pub fn BBREMC_1(ITEM: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBREMC_1", ctx)?;
    }

    //
    // Compress spaces as needed.
    //
    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by a symbol table routine. (Later,
    // it will be done by a card catalog routine.)
    //
    spicelib::SYDELC(
        &save.WHICH,
        save.CNTAB.as_arg_mut(),
        save.CPTAB.as_slice_mut(),
        save.CVTAB.as_arg_mut(),
        ctx,
    )?;

    spicelib::CHKOUT(b"BBREMC_1", ctx)?;
    Ok(())
}

//$Procedure BBREMS ( Bulletin board, remove, string )
pub fn BBREMS_1(ITEM: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBREMS_1", ctx)?;
    }

    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);
    SBREM_1(
        &save.WHICH,
        save.NBUF.as_arg_mut(),
        save.PBUF.as_slice_mut(),
        save.VBUF.as_arg_mut(),
        ctx,
    )?;

    spicelib::CHKOUT(b"BBREMS_1", ctx)?;
    Ok(())
}

//$Procedure BBFNDI ( Bulletin board, find, integer )
pub fn BBFNDI_1(ITEM: &[u8], N: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBFNDI_1", ctx)?;
    }

    //
    // Compress spaces as needed.
    //
    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by a symbol table routine. (Later,
    // it will be done by a card catalog routine.)
    //
    *N = spicelib::SYDIMI(
        &save.WHICH,
        save.INTAB.as_arg(),
        save.IPTAB.as_slice(),
        save.IVTAB.as_slice(),
        ctx,
    )?;

    spicelib::CHKOUT(b"BBFNDI_1", ctx)?;
    Ok(())
}

//$Procedure BBFNDD ( Bulletin board, find, DP )
pub fn BBFNDD_1(ITEM: &[u8], N: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBFNDD_1", ctx)?;
    }

    //
    // Compress spaces as needed.
    //
    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by a symbol table routine. (Later,
    // it will be done by a card catalog routine.)
    //
    *N = spicelib::SYDIMD(
        &save.WHICH,
        save.DNTAB.as_arg(),
        save.DPTAB.as_slice(),
        save.DVTAB.as_slice(),
        ctx,
    )?;

    spicelib::CHKOUT(b"BBFNDD_1", ctx)?;
    Ok(())
}

//$Procedure BBFNDC ( Bulletin board, find, character )
pub fn BBFNDC_1(ITEM: &[u8], N: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBFNDC_1", ctx)?;
    }

    //
    // Compress spaces as needed.
    //
    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);

    //
    // The real work is done by a symbol table routine. (Later,
    // it will be done by a card catalog routine.)
    //
    *N = spicelib::SYDIMC(
        &save.WHICH,
        save.CNTAB.as_arg(),
        save.CPTAB.as_slice(),
        save.CVTAB.as_arg(),
        ctx,
    )?;

    spicelib::CHKOUT(b"BBFNDC_1", ctx)?;
    Ok(())
}

//$Procedure BBFNDS ( Bulletin board, find, string )
pub fn BBFNDS_1(ITEM: &[u8], N: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBFNDS_1", ctx)?;
    }

    spicelib::CMPRSS(b" ", 0, ITEM, &mut save.WHICH);
    SBGET_1(
        &save.WHICH,
        save.NBUF.as_arg(),
        save.PBUF.as_slice(),
        save.VBUF.as_arg(),
        &mut save.WHAT,
        &mut save.POS,
        ctx,
    )?;

    if (save.POS > 0) {
        *N = 1;
    } else {
        *N = 0;
    }

    spicelib::CHKOUT(b"BBFNDS_1", ctx)?;
    Ok(())
}

//$Procedure BBCLR ( Bulletin board, clear )
pub fn BBCLR_1(ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"BBCLR_1", ctx)?;
    }

    //
    // Wipe out all three symbol tables.
    //
    spicelib::SSIZEC(MAXI, save.INTAB.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(MAXI, save.IPTAB.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXIV, save.IVTAB.as_slice_mut(), ctx)?;

    spicelib::SSIZEC(MAXD, save.DNTAB.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(MAXD, save.DPTAB.as_slice_mut(), ctx)?;
    spicelib::SSIZED(MAXDV, save.DVTAB.as_slice_mut(), ctx)?;

    spicelib::SSIZEC(MAXC, save.CNTAB.as_arg_mut(), ctx)?;
    spicelib::SSIZEI(MAXC, save.CPTAB.as_slice_mut(), ctx)?;
    spicelib::SSIZEC(MAXCV, save.CVTAB.as_arg_mut(), ctx)?;

    //
    // Re-initialize the string buffer.
    //
    SBINIT_1(
        MAXS,
        ((MAXS * 4) + 4),
        (MAXCHR / 100),
        save.NBUF.as_arg_mut(),
        save.PBUF.as_slice_mut(),
        save.VBUF.as_arg_mut(),
        ctx,
    )?;

    spicelib::CHKOUT(b"BBCLR_1", ctx)?;
    Ok(())
}
