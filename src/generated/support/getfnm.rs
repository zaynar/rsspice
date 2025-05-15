//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXNAM: i32 = 1000;
const DPRMPT: &[u8] = b"Filename? ";
const OSTAT: &[u8] = b"OLD";
const NSTAT: &[u8] = b"NEW";

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

//$Procedure GETFNM ( Get a filename from standard input )
pub fn GETFNM(
    PRMPT: &[u8],
    FSTAT: &[u8],
    FNAME: &mut [u8],
    VALID: &mut bool,
    MESSG: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut NAMBUF = [b' '; MAXNAM as usize];
    let mut STATUS = [b' '; 3 as usize];
    let mut I: i32 = 0;
    let mut LENGTH: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
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
        spicelib::CHKIN(b"GETFNM", ctx)?;
    }
    //
    // If this is the first time this routine has been called, initialize
    // the ``bad character'' string.
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
    // Set the value of the valid flag to .TRUE.. We might as well assume
    // that the name entered will be a valid one.
    //
    *VALID = true;
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
        *VALID = false;
        fstr::assign(
            MESSG,
            &fstr::concat(
                &fstr::concat(b"The status \'", &STATUS),
                b"\' was not recognized.",
            ),
        );
        spicelib::CHKOUT(b"GETFNM", ctx)?;
        return Ok(());
    }
    //
    // Read in a potential filename, and test it for validity.
    //
    if fstr::eq(PRMPT, b" ") {
        spicelib::PROMPT(DPRMPT, FNAME, ctx)?;
    } else {
        spicelib::PROMPT(PRMPT, FNAME, ctx)?;
    }

    //
    // The string we just obtained could be an environment variable.
    // If so, expand it.
    //
    EXPFNM_2(FNAME, &mut NAMBUF, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"GETFNM", ctx)?;
        return Ok(());
    }

    fstr::assign(FNAME, &NAMBUF);

    if fstr::eq(FNAME, b" ") {
        *VALID = false;
        fstr::assign(MESSG, b"A blank filename is not valid.");
        spicelib::CHKOUT(b"GETFNM", ctx)?;
        return Ok(());
    }

    //
    // Left justify the filename.
    //
    spicelib::LJUST(&FNAME.to_vec(), FNAME);

    //
    // Check for bad characters in the filename.
    //
    LENGTH = spicelib::LASTNB(FNAME);
    I = spicelib::CPOS(fstr::substr(FNAME, 1..=LENGTH), &save.BADCHR, 1);

    if (I > 0) {
        *VALID = false;
        fstr::assign(
            MESSG,
            b"Invalid filename. Illegal character encountered: decimal value: #",
        );
        spicelib::REPMI(
            &MESSG.to_vec(),
            b"#",
            intrinsics::ICHAR(fstr::substr(FNAME, I..=I)),
            MESSG,
            ctx,
        );
        spicelib::CHKOUT(b"GETFNM", ctx)?;
        return Ok(());
    }

    //
    // We know that the filename that was entered was nonblank and had
    // no bad characters. So, now we take care of the status question.
    //
    if fstr::eq(&STATUS, OSTAT) {
        if !spicelib::EXISTS(fstr::substr(FNAME, 1..=spicelib::RTRIM(FNAME)), ctx)? {
            *VALID = false;
            fstr::assign(MESSG, b"The file does not exist.");
            spicelib::CHKOUT(b"GETFNM", ctx)?;
            return Ok(());
        }
    } else if fstr::eq(&STATUS, NSTAT) {
        if spicelib::EXISTS(fstr::substr(FNAME, 1..=spicelib::RTRIM(FNAME)), ctx)? {
            *VALID = false;
            fstr::assign(MESSG, b"The file already exists.");
            spicelib::CHKOUT(b"GETFNM", ctx)?;
            return Ok(());
        }
    }

    //
    // At this point, we have done the best we can. If the status
    // was new, we might still have an invalid filename, but the
    // exact reasons for its invalidity are system dependent.
    //
    spicelib::CHKOUT(b"GETFNM", ctx)?;

    Ok(())
}
