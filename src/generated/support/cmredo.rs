//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NONE: i32 = 0;
const COMBUF: i32 = (NONE + 1);
const KEYBRD: i32 = (COMBUF + 1);
const INPFIL: i32 = (KEYBRD + 1);
const WDSIZE: i32 = 32;
const LNSIZE: i32 = 300;

struct SaveVars {
    START: Vec<u8>,
    EXIT: Vec<u8>,
    STOP: Vec<u8>,
    FRSTWD: Vec<u8>,
    SCNDWD: Vec<u8>,
    REST: Vec<u8>,
    B1: i32,
    B2: i32,
    E1: i32,
    E2: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut START = vec![b' '; WDSIZE as usize];
        let mut EXIT = vec![b' '; WDSIZE as usize];
        let mut STOP = vec![b' '; WDSIZE as usize];
        let mut FRSTWD = vec![b' '; WDSIZE as usize];
        let mut SCNDWD = vec![b' '; WDSIZE as usize];
        let mut REST = vec![b' '; LNSIZE as usize];
        let mut B1: i32 = 0;
        let mut B2: i32 = 0;
        let mut E1: i32 = 0;
        let mut E2: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            START,
            EXIT,
            STOP,
            FRSTWD,
            SCNDWD,
            REST,
            B1,
            B2,
            E1,
            E2,
            FIRST,
        }
    }
}

//$Procedure      CMREDO ( COMMND loop trap )
pub fn CMREDO(
    COMMND: &[u8],
    FROM: i32,
    TRAP: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    if save.FIRST {
        TRNLAT(b"STOP", &mut save.STOP, ctx);
        TRNLAT(b"EXIT", &mut save.EXIT, ctx);
        TRNLAT(b"START", &mut save.START, ctx);

        save.FIRST = false;
    }

    spicelib::NEXTWD(COMMND, &mut save.FRSTWD, &mut save.REST);
    spicelib::NEXTWD(&save.REST.to_vec(), &mut save.SCNDWD, &mut save.REST);

    spicelib::UCASE(&save.FRSTWD.to_vec(), &mut save.FRSTWD, ctx);
    spicelib::UCASE(&save.SCNDWD.to_vec(), &mut save.SCNDWD, ctx);

    save.B1 = 1;
    save.B2 = 1;
    save.E1 = spicelib::RTRIM(&save.FRSTWD);
    save.E2 = spicelib::RTRIM(&save.SCNDWD);

    if fstr::ne(&save.REST, b" ") {
        *TRAP = false;
        return Ok(());
    }

    if fstr::eq(&save.FRSTWD, b" ") {
        *TRAP = false;
        return Ok(());
    }

    if fstr::eq(&save.FRSTWD, &save.START) {
        *TRAP = true;
        PUTCOM(COMMND, FROM, ctx)?;
        return Ok(());
    } else if ((fstr::eq(&save.FRSTWD, &save.EXIT) && fstr::eq(&save.SCNDWD, b" "))
        && (FROM != KEYBRD))
    {
        *TRAP = true;
        PUTCOM(COMMND, FROM, ctx)?;
        return Ok(());
    } else if ((fstr::eq(&save.FRSTWD, &save.STOP) && fstr::eq(&save.SCNDWD, b" "))
        && (FROM != KEYBRD))
    {
        *TRAP = true;
        PUTCOM(COMMND, FROM, ctx)?;
        return Ok(());
    } else if (FROM != KEYBRD) {
        *TRAP = false;
        return Ok(());
    } else if (fstr::eq(&save.SCNDWD, b" ")
        && !M2WMCH(&save.FRSTWD, save.B1, save.E1, b"RECALL", ctx)?)
    {
        *TRAP = false;
        return Ok(());
    } else if (M2WMCH(&save.FRSTWD, save.B1, save.E1, b"RECALL", ctx)?
        && M2WMCH(&save.SCNDWD, save.B2, save.E2, b"@int(1:20)", ctx)?)
    {
        *TRAP = true;
        PUTCOM(COMMND, FROM, ctx)?;
    } else if (M2WMCH(&save.FRSTWD, save.B1, save.E1, b"RECALL", ctx)?
        && M2WMCH(&save.SCNDWD, save.B2, save.E2, b"ALL", ctx)?)
    {
        *TRAP = true;
        PUTCOM(COMMND, FROM, ctx)?;
    } else if (M2WMCH(&save.FRSTWD, save.B1, save.E1, b"EDIT", ctx)?
        && M2WMCH(&save.SCNDWD, save.B2, save.E2, b"@int(1:20)", ctx)?)
    {
        *TRAP = true;
        PUTCOM(COMMND, FROM, ctx)?;
    } else if (M2WMCH(&save.FRSTWD, save.B1, save.E1, b"DO", ctx)?
        && M2WMCH(&save.SCNDWD, save.B2, save.E2, b"@int(1:20)", ctx)?)
    {
        *TRAP = true;
        PUTCOM(COMMND, FROM, ctx)?;
    } else if ((M2WMCH(&save.FRSTWD, save.B1, save.E1, b"RECALL", ctx)?
        && fstr::ne(&save.SCNDWD, b" "))
        && fstr::eq(&save.REST, b" "))
    {
        *TRAP = true;
        PUTCOM(COMMND, FROM, ctx)?;
    } else if ((M2WMCH(&save.FRSTWD, save.B1, save.E1, b"EDIT", ctx)?
        && fstr::ne(&save.SCNDWD, b" "))
        && fstr::eq(&save.REST, b" "))
    {
        *TRAP = true;
        PUTCOM(COMMND, FROM, ctx)?;
    } else if ((M2WMCH(&save.FRSTWD, save.B1, save.E1, b"DO", ctx)?
        && fstr::ne(&save.SCNDWD, b" "))
        && fstr::eq(&save.REST, b" "))
    {
        *TRAP = true;
        PUTCOM(COMMND, FROM, ctx)?;
    } else {
        *TRAP = false;
    }

    Ok(())
}
