//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TB4POS: i32 = 4;
const TTLPOS: i32 = 10;
const LINLEN: i32 = 80;
const STDOUT: i32 = 6;

//$Procedure GETOPT ( Get an option from a menu )
pub fn GETOPT(
    TITLE: &[u8],
    NOPT: i32,
    OPTNAM: CharArray,
    OPTTXT: CharArray,
    OPTION: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let OPTNAM = DummyCharArray::new(OPTNAM, None, 1..);
    let OPTTXT = DummyCharArray::new(OPTTXT, None, 1..);
    let mut LINE = [b' '; LINLEN as usize];
    let mut PRMPT = [b' '; LINLEN as usize];
    let mut MSG = [b' '; LINLEN as usize];
    let mut I: i32 = 0;
    let mut IOPT: i32 = 0;
    let mut DONE: bool = false;
    let mut OK: bool = false;
    let mut OKALPH: bool = false;
    let mut OKDIGI: bool = false;
    let mut OKEQU: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //

    //
    // Mnemonic for the standard output.
    //
    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"GETOPT", ctx)?;
    }
    //
    // Check to make sure that the number of menu options is positive.
    // if it is not, then signal an error with an appropriate error
    // message.
    //
    if (NOPT < 1) {
        spicelib::SETMSG(b"The number of options was not positive: #.", ctx);
        spicelib::ERRINT(b"#", NOPT, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        spicelib::CHKOUT(b"GETOPT", ctx)?;
        return Ok(());
    }
    //
    // Initialize the option prompt.
    //
    fstr::assign(&mut PRMPT, b" ");
    fstr::assign(fstr::substr_mut(&mut PRMPT, TB4POS..), b"Option: ");
    //
    // Check to make sure that all of the option names are alphanumeric
    // and uppercase. The only exception is the period, which signals a
    // blank line.
    //
    OK = true;
    {
        let m1__: i32 = 1;
        let m2__: i32 = NOPT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            OKDIGI = ((intrinsics::ICHAR(&OPTNAM[I]) >= intrinsics::ICHAR(b"0"))
                && (intrinsics::ICHAR(&OPTNAM[I]) <= intrinsics::ICHAR(b"9")));

            OKALPH = ((intrinsics::ICHAR(&OPTNAM[I]) >= intrinsics::ICHAR(b"A"))
                && (intrinsics::ICHAR(&OPTNAM[I]) <= intrinsics::ICHAR(b"Z")));

            OKEQU = (intrinsics::ICHAR(&OPTNAM[I]) == intrinsics::ICHAR(b"."));

            OK = (OK && ((OKDIGI || OKALPH) || OKEQU));

            if !OK {
                spicelib::SETMSG(
                    b"An illegal option name was found: option #, name \'#\'. ",
                    ctx,
                );
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(ILLEGALOPTIONNAME)", ctx)?;
                spicelib::CHKOUT(b"GETOPT", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }
    //
    // Do until we get a valid option.
    //
    DONE = false;
    while !DONE {
        //
        // Display the menu title if it is non blank
        //
        if fstr::ne(TITLE, b" ") {
            fstr::assign(&mut LINE, b" ");
            fstr::assign(fstr::substr_mut(&mut LINE, TTLPOS..), b"#");
            spicelib::REPMC(&LINE.clone(), b"#", TITLE, &mut LINE);
            spicelib::WRITLN(&LINE, STDOUT, ctx)?;
        }
        //
        // Display the menu and read in an option.
        //
        spicelib::WRITLN(b" ", STDOUT, ctx)?;

        {
            let m1__: i32 = 1;
            let m2__: i32 = NOPT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(&mut LINE, b" ");
                if fstr::ne(OPTNAM.get(I), b".") {
                    fstr::assign(fstr::substr_mut(&mut LINE, TB4POS..), b"( # ) #");
                    spicelib::REPMC(&LINE.clone(), b"#", &OPTNAM[I], &mut LINE);
                    spicelib::REPMC(&LINE.clone(), b"#", &OPTTXT[I], &mut LINE);
                }

                spicelib::WRITLN(&LINE, STDOUT, ctx)?;

                I += m3__;
            }
        }

        spicelib::WRITLN(b" ", STDOUT, ctx)?;

        I = (spicelib::RTRIM(&PRMPT) + 1);
        spicelib::PROMPT(fstr::substr(&PRMPT, 1..=I), &mut LINE, ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"GETOPT", ctx)?;
            return Ok(());
        }
        //
        // Initialize the option value to zero, invalid option.
        //
        IOPT = 0;
        if fstr::eq(&LINE, b" ") {
            spicelib::WRITLN(b" ", STDOUT, ctx)?;
        } else {
            spicelib::LJUST(&LINE.clone(), &mut LINE);
            spicelib::UCASE(&LINE.clone(), &mut LINE, ctx);
            //
            // Check to make sure that the option we got is a valid
            // candidate: It must be alpha numeric.
            //
            OKDIGI = ((intrinsics::ICHAR(fstr::substr(&LINE, 1..=1)) >= intrinsics::ICHAR(b"0"))
                && (intrinsics::ICHAR(fstr::substr(&LINE, 1..=1)) <= intrinsics::ICHAR(b"9")));

            OKALPH = ((intrinsics::ICHAR(fstr::substr(&LINE, 1..=1)) >= intrinsics::ICHAR(b"A"))
                && (intrinsics::ICHAR(fstr::substr(&LINE, 1..=1)) <= intrinsics::ICHAR(b"Z")));

            OK = (OKDIGI || OKALPH);
            //
            // If we got a valid candidate for an option, see if it is one
            // of the options that we are supplying.
            //
            if OK {
                IOPT = spicelib::ISRCHC(fstr::substr(&LINE, 1..=1), NOPT, OPTNAM.as_arg());
                OK = (IOPT != 0);
            }

            if !OK {
                fstr::assign(&mut MSG, b"\'#\' was not a valid option. Please try again.");
                spicelib::REPMC(&MSG.clone(), b"#", fstr::substr(&LINE, 1..=1), &mut MSG);
                spicelib::WRITLN(b" ", STDOUT, ctx)?;
                fstr::assign(&mut LINE, b" ");
                fstr::assign(fstr::substr_mut(&mut LINE, TB4POS..), b"***");
                spicelib::WRITLN(&LINE, STDOUT, ctx)?;
                fstr::assign(fstr::substr_mut(&mut LINE, TB4POS..), b"*** #");
                spicelib::REPMC(&LINE.clone(), b"#", &MSG, &mut LINE);
                spicelib::WRITLN(&LINE, STDOUT, ctx)?;
                fstr::assign(fstr::substr_mut(&mut LINE, TB4POS..), b"***");
                spicelib::WRITLN(&LINE, STDOUT, ctx)?;
                spicelib::WRITLN(b" ", STDOUT, ctx)?;
            } else {
                *OPTION = IOPT;
                DONE = true;
            }
        }
    }

    spicelib::CHKOUT(b"GETOPT", ctx)?;
    Ok(())
}
