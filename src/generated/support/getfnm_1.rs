//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const PRMLEN: i32 = 80;
const DPRMPT: &[u8] = b"Filename? ";
const OSTAT: &[u8] = b"OLD";
const NSTAT: &[u8] = b"NEW";
const FNMLEN: i32 = 1000;
const ACTLEN: i32 = 10;

struct SaveVars {
    BADCHR: Vec<u8>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BADCHR = vec![b' '; 162 as usize];
        let mut FIRST: bool = false;

        FIRST = true;

        Self { BADCHR, FIRST }
    }
}

//$Procedure GETFNM_1 ( Get a filename from standard input )
pub fn GETFNM_1(
    PRMPT: &[u8],
    FSTAT: &[u8],
    FNAME: &mut [u8],
    VALID: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut MYFNAM = [b' '; FNMLEN as usize];
    let mut MYPRMT = [b' '; PRMLEN as usize];
    let mut NAMBUF = [b' '; FNMLEN as usize];
    let mut OLDACT = [b' '; ACTLEN as usize];
    let mut STATUS = [b' '; 3 as usize];
    let mut I: i32 = 0;
    let mut LENGTH: i32 = 0;
    let mut TRYAGN: bool = false;
    let mut MYVLID: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Maximum length of a filename.
    //
    //
    // Length of an error action
    //
    //
    // Local Variables
    //

    //
    // Saved Variables
    //
    //
    // Initial Values
    //
    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"GETFNM_1", ctx)?;
    }
    //
    // We are going to be signalling errors and resetting the error
    // handling, so we need to be in RETURN mode. First we get the
    // current mode and save it, then we set the mode to return. Upon
    // leaving the subroutine, we will restore the error handling mode
    // that was in effect when we entered.
    //
    spicelib::ERRACT(b"GET", &mut OLDACT, ctx)?;
    spicelib::ERRACT(b"SET", &mut b"RETURN".clone(), ctx)?;
    //
    // If this is the first time this routine has been called,
    // initialize the ``bad character'' string.
    //
    if save.FIRST {
        save.FIRST = false;

        {
            let m1__: i32 = 0;
            let m2__: i32 = 32;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(
                    fstr::substr_mut(&mut save.BADCHR, (I + 1)..=(I + 1)),
                    &intrinsics::CHAR(I),
                );
                I += m3__;
            }
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = 129;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(
                    fstr::substr_mut(&mut save.BADCHR, (33 + I)..=(33 + I)),
                    &intrinsics::CHAR((126 + I)),
                );
                I += m3__;
            }
        }
    }
    //
    // Left justify and convert the file status to upper case for
    // comparisons.
    //
    spicelib::LJUST(FSTAT, &mut STATUS);
    spicelib::UCASE(&STATUS.clone(), &mut STATUS, ctx);
    //
    // Check to see if we have a valid status for the filename.
    //
    if (fstr::ne(&STATUS, OSTAT) && fstr::ne(&STATUS, NSTAT)) {
        spicelib::SETMSG(b"The file status \'#\' was not valid. The file status must have a value of \'NEW\' or \'OLD\'.", ctx);
        spicelib::ERRCH(b"#", &STATUS, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        spicelib::CHKOUT(b"GETFNM_1", ctx)?;
        return Ok(());
    }
    //
    // Store the input value for the prompt into our local value. We do
    // this for pedantic Fortran compilers that issue warnings for
    // CHARACTER*(*) variables used with concatenation.
    //
    fstr::assign(&mut MYPRMT, PRMPT);
    //
    // Read in a potential filename, and test it for validity.
    //
    TRYAGN = true;

    while TRYAGN {
        //
        // Set the value of the valid flag to .TRUE.. We assume that the
        // name entered will be a valid one.
        //
        MYVLID = true;
        //
        // Get the filename.
        //
        if fstr::eq(&MYPRMT, b" ") {
            spicelib::PROMPT(DPRMPT, &mut MYFNAM, ctx)?;
        } else {
            spicelib::PROMPT(
                &fstr::concat(fstr::substr(&MYPRMT, 1..=spicelib::LASTNB(&MYPRMT)), b" "),
                &mut MYFNAM,
                ctx,
            )?;
        }

        if spicelib::FAILED(ctx) {
            MYVLID = false;
        }

        //
        // Translate the name if it's an environment variable.
        //
        EXPFNM_2(&MYFNAM, &mut NAMBUF, ctx)?;

        fstr::assign(&mut MYFNAM, &NAMBUF);

        if spicelib::FAILED(ctx) {
            MYVLID = false;
        }

        if MYVLID {
            if fstr::eq(&MYFNAM, b" ") {
                MYVLID = false;

                spicelib::SETMSG(b"The filename entered was blank.", ctx);
                spicelib::SIGERR(b"SPICE(BLANKFILENAME)", ctx)?;
            }
        }

        if MYVLID {
            //
            // Left justify the filename.
            //
            spicelib::LJUST(&MYFNAM.clone(), &mut MYFNAM);
            //
            // Check for bad characters in the filename.
            //
            LENGTH = spicelib::LASTNB(&MYFNAM);
            I = spicelib::CPOS(fstr::substr(&MYFNAM, 1..=LENGTH), &save.BADCHR, 1);

            if (I > 0) {
                MYVLID = false;

                spicelib::SETMSG(
                    b"The filename entered contains non printing characters or embedded blanks.",
                    ctx,
                );
                spicelib::SIGERR(b"SPICE(ILLEGALCHARACTER)", ctx)?;
            }
        }

        if MYVLID {
            //
            // We know that the filename that was entered was nonblank and
            // had no bad characters. So, now we take care of the status
            // question.
            //
            if fstr::eq(&STATUS, OSTAT) {
                if !spicelib::EXISTS(fstr::substr(&MYFNAM, 1..=spicelib::RTRIM(&MYFNAM)), ctx)? {
                    MYVLID = false;

                    spicelib::SETMSG(b"A file with the name \'#\' does not exist.", ctx);
                    spicelib::ERRCH(b"#", &MYFNAM, ctx);
                    spicelib::SIGERR(b"SPICE(FILEDOESNOTEXIST)", ctx)?;
                }
            } else if fstr::eq(&STATUS, NSTAT) {
                if spicelib::EXISTS(fstr::substr(&MYFNAM, 1..=spicelib::RTRIM(&MYFNAM)), ctx)? {
                    MYVLID = false;

                    spicelib::SETMSG(b"A file with the name \'#\' already exists.", ctx);
                    spicelib::ERRCH(b"#", &MYFNAM, ctx);
                    spicelib::SIGERR(b"SPICE(FILEALREADYEXISTS)", ctx)?;
                }
            }
        }

        if MYVLID {
            TRYAGN = false;
        } else {
            spicelib::WRITLN(b" ", 6, ctx)?;
            CNFIRM(b"Try again? (Yes/No) ", &mut TRYAGN, ctx)?;
            spicelib::WRITLN(b" ", 6, ctx)?;

            if TRYAGN {
                spicelib::RESET(ctx);
            }
        }
    }
    //
    // At this point, we have done the best we can. If the status
    // was new, we might still have an invalid filename, but the
    // exact reasons for its invalidity are system dependent, and
    // therefore hard to test.
    //

    *VALID = MYVLID;

    if *VALID {
        fstr::assign(FNAME, fstr::substr(&MYFNAM, 1..=spicelib::RTRIM(&MYFNAM)));
    }
    //
    // Restore the error action.
    //
    spicelib::ERRACT(b"SET", &mut OLDACT, ctx)?;

    spicelib::CHKOUT(b"GETFNM_1", ctx)?;
    Ok(())
}
