//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const STARS: &[u8] = b"*****";
const TOVR: &[u8] = b"    ";
const TTOVR: &[u8] = b"        ";
const LINLEN: i32 = 80;

//$ Procedure      GETOPT_1 ( Get option string from a specified list )
//
pub fn GETOPT_1(
    TITLE: &[u8],
    NOPT: i32,
    OPTNAM: CharArray,
    NAMLEN: i32,
    OPTTXT: CharArray,
    TXTLEN: i32,
    OPTVAL: CharArray,
    OPTION: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let OPTNAM = DummyCharArray::new(OPTNAM, None, 1..);
    let OPTTXT = DummyCharArray::new(OPTTXT, None, 1..);
    let OPTVAL = DummyCharArray::new(OPTVAL, None, 1..);
    let mut LINE = [b' '; LINLEN as usize];
    let mut MSG = [b' '; LINLEN as usize];
    let mut ITASK: i32 = 0;
    let mut DONE: bool = false;

    //

    //
    // SPICELIB functions
    //
    //
    // Local variables
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //
    // None.
    //
    //
    // Initial values
    //
    // None.
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"GETOPT_1", ctx)?;
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
        spicelib::CHKOUT(b"GETOPT_1", ctx)?;
        return Ok(());
    }
    //
    // Check to make sure that the length of the option names is at
    // least 1. If not, then signal an error with an appropriate error
    // message.
    //
    if (NAMLEN < 1) {
        spicelib::SETMSG(b"The length of the option names was not positive: #.", ctx);
        spicelib::ERRINT(b"#", NAMLEN, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        spicelib::CHKOUT(b"GETOPT_1", ctx)?;
        return Ok(());
    }
    //
    // Check to make sure that the length of the descriptive text for
    // each option is at least 1. If not, then signal an error with an
    // appropriate error message.
    //
    if (TXTLEN < 1) {
        spicelib::SETMSG(
            b"The length of the option descriptions was not positive: #.",
            ctx,
        );
        spicelib::ERRINT(b"#", TXTLEN, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDARGUMENT)", ctx)?;
        spicelib::CHKOUT(b"GETOPT_1", ctx)?;
        return Ok(());
    }
    //
    // Do until we get an option
    //
    DONE = false;
    while !DONE {
        //
        // Display the menu title if it is non blank
        //
        if fstr::ne(TITLE, b" ") {
            fstr::assign(&mut LINE, &fstr::concat(&fstr::concat(TTOVR, TTOVR), TITLE));
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
                writer.start()?;
                writer.finish()?;
            }
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
                writer.start()?;
                writer.write_str(fstr::substr(&LINE, 1..=spicelib::RTRIM(&LINE)))?;
                writer.finish()?;
            }
        }
        //
        // Display the menu and read in an option.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
            writer.start()?;
            writer.finish()?;
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = NOPT;
            let m3__: i32 = 1;
            ITASK = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                fstr::assign(
                    &mut LINE,
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(
                                &fstr::concat(TTOVR, b"( "),
                                fstr::substr(OPTNAM.get(ITASK), 1..=NAMLEN),
                            ),
                            b" ) ",
                        ),
                        fstr::substr(OPTTXT.get(ITASK), 1..=TXTLEN),
                    ),
                );
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
                    writer.start()?;
                    writer.write_str(fstr::substr(&LINE, 1..=spicelib::RTRIM(&LINE)))?;
                    writer.finish()?;
                }

                ITASK += m3__;
            }
        }

        //
        // Initialize the task indicator to zero, invalid task.
        //
        ITASK = 0;

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
            writer.start()?;
            writer.finish()?;
        }
        spicelib::PROMPT(&fstr::concat(TOVR, b"Option: "), &mut LINE, ctx)?;

        if fstr::ne(&LINE, b" ") {
            spicelib::LJUST(&LINE.clone(), &mut LINE);
            spicelib::UCASE(&LINE.clone(), &mut LINE, ctx);

            ITASK = spicelib::ISRCHC(fstr::substr(&LINE, 1..=NAMLEN), NOPT, OPTNAM.as_arg());

            if (ITASK == 0) {
                fstr::assign(&mut MSG, b"\'#\' was not a valid option. Please try again.");
                spicelib::REPMC(&MSG.clone(), b"#", &LINE, &mut MSG);
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
                    writer.start()?;
                    writer.finish()?;
                }
                fstr::assign(&mut LINE, &fstr::concat(TOVR, STARS));
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
                    writer.start()?;
                    writer.write_str(fstr::substr(&LINE, 1..=spicelib::RTRIM(&LINE)))?;
                    writer.finish()?;
                }
                fstr::assign(
                    &mut LINE,
                    &fstr::concat(
                        &fstr::concat(&fstr::concat(TOVR, STARS), b" "),
                        fstr::substr(&MSG, 1..=spicelib::RTRIM(&MSG)),
                    ),
                );
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
                    writer.start()?;
                    writer.write_str(fstr::substr(&LINE, 1..=spicelib::RTRIM(&LINE)))?;
                    writer.finish()?;
                }
                fstr::assign(&mut LINE, &fstr::concat(TOVR, STARS));
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
                    writer.start()?;
                    writer.write_str(fstr::substr(&LINE, 1..=spicelib::RTRIM(&LINE)))?;
                    writer.finish()?;
                }
            } else {
                fstr::assign(OPTION, OPTVAL.get(ITASK));
                DONE = true;
            }
        }
    }

    spicelib::CHKOUT(b"GETOPT_1", ctx)?;
    Ok(())
}
