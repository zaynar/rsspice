//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    BLANK: Vec<u8>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BLANK = vec![b' '; 132 as usize];

        fstr::assign(&mut BLANK, b" ");

        Self { BLANK }
    }
}

//$Procedure RDSTMT ( Read a statement entered on one or more lines)
pub fn RDSTMT(
    PRMPT: &[u8],
    DELIM: &[u8],
    STMT: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DELIM = &DELIM[..1 as usize];
    let mut SPACE = [b' '; 1 as usize];
    let mut TAB = [b' '; 1 as usize];
    let mut PRLEN: i32 = 0;
    let mut LINE = [b' '; 132 as usize];
    let mut MYPRM = [b' '; 132 as usize];
    let mut END: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Read the first statement. Use the prompt. Return immediately
    // if a blank line or an error is encountered.
    //
    PRLEN = (spicelib::RTRIM(PRMPT) + 1);
    fstr::assign(&mut MYPRM, PRMPT);
    fstr::assign(&mut LINE, b" ");
    fstr::assign(&mut SPACE, b" ");
    fstr::assign(&mut TAB, &intrinsics::CHAR(9));
    //
    // Set up the white-space/line-break accountant.
    //

    spicelib::PROMPT(fstr::substr(&MYPRM, 1..=PRLEN), &mut LINE, ctx)?;

    if fstr::eq(&LINE, b" ") {
        fstr::assign(STMT, b" ");
        return Ok(());
    } else {
        fstr::assign(STMT, &LINE);
    }

    //
    // Record the size of the white-space and line-break fields.
    //
    RSTBUF(ctx);
    spicelib::REPLCH(&LINE.clone(), &TAB, &SPACE, &mut LINE);
    PUTBUF(&LINE, ctx);

    //
    // Read succeeding lines. Indent to the length of the original
    // prompt. Add the input line to the current statement. Quit when:
    //
    //        - A delimiter is encountered. (Return the statement
    //          up to the delimiter.)
    //
    //        - A blank line or an error is encountered. (Return
    //          a blank statement.)
    //
    while (intrinsics::INDEX(STMT, DELIM) == 0) {
        spicelib::PROMPT(fstr::substr(&save.BLANK, 1..=PRLEN), &mut LINE, ctx)?;
        spicelib::REPLCH(&LINE.clone(), &TAB, &SPACE, &mut LINE);
        PUTBUF(&LINE, ctx);

        if fstr::eq(&LINE, b" ") {
            DMPBUF(ctx);
            fstr::assign(STMT, b" ");
            return Ok(());
        } else {
            spicelib::SUFFIX(&LINE, 1, STMT);
        }
    }

    //
    // If we made it to here, we encountered a delimiter. Take the
    // entire statement up to the character before the delimiter.
    //
    END = intrinsics::INDEX(STMT, DELIM);
    fstr::assign(fstr::substr_mut(STMT, END..), b" ");

    Ok(())
}
