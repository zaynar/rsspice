//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const STARS: &[u8] = b"*****";
const LINLEN: i32 = 80;

//
//$ Procedure     GETOPT_2 ( Get option string from a specified list )
//
pub fn GETOPT_2(
    TITLE: &[u8],
    TINDNT: i32,
    NOPT: i32,
    OPTNAM: CharArray,
    OPTTXT: CharArray,
    OINDNT: i32,
    OPTION: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let OPTNAM = DummyCharArray::new(OPTNAM, None, 1..);
    let OPTTXT = DummyCharArray::new(OPTTXT, None, 1..);
    let mut LINE = [b' '; LINLEN as usize];
    let mut MSG = [b' '; LINLEN as usize];
    let mut MYOPT = [b' '; LINLEN as usize];
    let mut SPACE = [b' '; LINLEN as usize];
    let mut ITASK: i32 = 0;
    let mut NAMLEN: i32 = 0;
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
        spicelib::CHKIN(b"GETOPT_2", ctx)?;
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
        spicelib::CHKOUT(b"GETOPT_2", ctx)?;
        return Ok(());
    }
    //
    // Do until we get an option
    //

    NAMLEN = spicelib::NBWID(OPTNAM.as_arg(), NOPT);

    DONE = false;
    fstr::assign(&mut SPACE, b" ");
    while !DONE {
        //
        // Display the menu title if it is non blank
        //
        if fstr::ne(TITLE, b" ") {
            if (TINDNT > 0) {
                fstr::assign(
                    &mut LINE,
                    &fstr::concat(fstr::substr(&SPACE, 1..=TINDNT), TITLE),
                );
            } else {
                fstr::assign(&mut LINE, TITLE);
            }

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
                writer.write_str(&LINE)?;
                writer.finish()?;
            }
        }
        fstr::assign(&mut LINE, b" ");
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
            writer.start()?;
            writer.write_str(fstr::substr(&LINE, 1..=1))?;
            writer.finish()?;
        }

        {
            let m1__: i32 = 1;
            let m2__: i32 = NOPT;
            let m3__: i32 = 1;
            ITASK = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if fstr::ne(OPTNAM.get(ITASK), b" ") {
                    fstr::assign(
                        &mut MYOPT,
                        &fstr::concat(
                            &fstr::concat(
                                &fstr::concat(b"( ", fstr::substr(OPTNAM.get(ITASK), 1..=NAMLEN)),
                                b" ) ",
                            ),
                            OPTTXT.get(ITASK),
                        ),
                    );
                } else {
                    fstr::assign(
                        &mut MYOPT,
                        &fstr::concat(fstr::substr(&SPACE, 1..=(NAMLEN + 5)), OPTTXT.get(ITASK)),
                    );
                }

                if (OINDNT > 0) {
                    fstr::assign(
                        &mut LINE,
                        &fstr::concat(fstr::substr(&SPACE, 1..=OINDNT), &MYOPT),
                    );
                } else {
                    fstr::assign(&mut LINE, &MYOPT);
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
        spicelib::PROMPT(
            &fstr::concat(fstr::substr(&SPACE, 1..=OINDNT), b"Option: "),
            &mut LINE,
            ctx,
        )?;

        if fstr::ne(&LINE, b" ") {
            spicelib::LJUST(&LINE.clone(), &mut LINE);
            spicelib::UCASE(&LINE.clone(), &mut LINE, ctx);

            ITASK = spicelib::ISRCHC(&LINE, NOPT, OPTNAM.as_arg());

            if (ITASK == 0) {
                fstr::assign(
                    &mut MSG,
                    &fstr::concat(
                        &fstr::concat(STARS, b" \'#\' was not a valid option."),
                        b" Please try again.",
                    ),
                );
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
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
                    writer.start()?;
                    writer.write_str(STARS)?;
                    writer.finish()?;
                }
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
                    writer.start()?;
                    writer.write_str(fstr::substr(&MSG, 1..=spicelib::RTRIM(&MSG)))?;
                    writer.finish()?;
                }
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::ListDirectedWriter::new(ctx.default_write_unit()?, None)?;
                    writer.start()?;
                    writer.write_str(STARS)?;
                    writer.finish()?;
                }
            } else {
                *OPTION = ITASK;
                DONE = true;
            }
        }
    }

    spicelib::CHKOUT(b"GETOPT_2", ctx)?;
    Ok(())
}
