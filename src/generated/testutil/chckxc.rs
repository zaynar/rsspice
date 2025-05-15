//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 640;
const STYSIZ: i32 = 120;
const WDSIZE: i32 = 32;

//$Procedure      CHCKXC ( Check exceptions )
pub fn CHCKXC(
    EXCEPT: bool,
    SHORT: &[u8],
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut LMSG = [b' '; LNSIZE as usize];
    let mut MESSGE = [b' '; LNSIZE as usize];
    let mut CTRACE = [b' '; LNSIZE as usize];
    let mut GOOD = [b' '; STYSIZ as usize];
    let mut BAD = [b' '; STYSIZ as usize];
    let mut SMSG = [b' '; WDSIZE as usize];
    let mut FAIL: bool = false;
    let mut DUMMY: bool = false;

    //
    // Test Utility functions
    //

    //
    // Spicelib functions
    //

    //
    // Local Variables
    //

    TSTSTY(&mut GOOD, &mut BAD, ctx);
    TSTLGS(
        b"LEFT 3 RIGHT 75 NEWLINE /cr ",
        b"LEFT 3 RIGHT 75 NEWLINE /cr FLAG --- LEADER ---",
        ctx,
    );

    DUMMY = SETON(b"CHCKXC", ctx);

    if !EXCEPT {
        FAIL = spicelib::FAILED(ctx);

        if FAIL {
            spicelib::GETMSG(b"SHORT", &mut SMSG, ctx)?;
            spicelib::GETMSG(b"LONG", &mut LMSG, ctx)?;
            spicelib::QCKTRC(&mut CTRACE, ctx);

            spicelib::RESET(ctx);

            fstr::assign(&mut MESSGE, b"No exception was expected however one exists.  The short error message was: \'#\'. ");

            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&SMSG, 1..=spicelib::RTRIM(&SMSG)),
                &mut MESSGE,
            );
            TSTLOG(&MESSGE, FAIL, ctx)?;

            fstr::assign(&mut MESSGE, b"The long error message follows:");
            TSTLOG(b" ", FAIL, ctx)?;
            TSTLOG(&MESSGE, FAIL, ctx)?;
            TSTLOG(b" ", FAIL, ctx)?;
            TSTLOG(&LMSG, FAIL, ctx)?;

            fstr::assign(&mut MESSGE, b"The current trace is: ");
            spicelib::SUFFIX(&CTRACE, 1, &mut MESSGE);
            TSTLOG(b" ", FAIL, ctx)?;
            TSTLOG(&MESSGE, FAIL, ctx)?;
        } else if VERBOS(ctx) {
            fstr::assign(
                &mut MESSGE,
                b"No exception was detected.  This is the expected behaviour. ",
            );

            TSTLOG(b" ", FAIL, ctx)?;
            TSTLOG(&MESSGE, FAIL, ctx)?;
        }
    } else {
        if spicelib::FAILED(ctx) {
            spicelib::GETMSG(b"SHORT", &mut SMSG, ctx)?;
            spicelib::GETMSG(b"LONG", &mut LMSG, ctx)?;
            spicelib::QCKTRC(&mut CTRACE, ctx);
            spicelib::RESET(ctx);

            FAIL = fstr::ne(&SMSG, SHORT);

            if FAIL {
                fstr::assign(&mut MESSGE, b"The expected short error message, \'#\' was not found. Instead the short error message was \'#\'. ");

                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(SHORT, 1..=spicelib::RTRIM(SHORT)),
                    &mut MESSGE,
                );
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(&SMSG, 1..=spicelib::RTRIM(&SMSG)),
                    &mut MESSGE,
                );
                TSTLOG(&MESSGE, FAIL, ctx)?;

                fstr::assign(&mut MESSGE, b"The long error message follows:");
                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&LMSG, FAIL, ctx)?;

                fstr::assign(&mut MESSGE, b"The current trace is: ");
                spicelib::SUFFIX(&CTRACE, 1, &mut MESSGE);
                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
            } else if VERBOS(ctx) {
                fstr::assign(
                    &mut MESSGE,
                    b"The short error message was \'#\' as expected. ",
                );

                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(&SMSG, 1..=spicelib::RTRIM(&SMSG)),
                    &mut MESSGE,
                );
                TSTLOG(&MESSGE, FAIL, ctx)?;

                fstr::assign(&mut MESSGE, b"The long error message follows:");
                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&LMSG, FAIL, ctx)?;
            }
        } else {
            FAIL = true;

            fstr::assign(
                &mut MESSGE,
                b"No exception was detected.  The expected short error message was: \'#\'. ",
            );

            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(SHORT, 1..=spicelib::RTRIM(SHORT)),
                &mut MESSGE,
            );
            TSTLOG(&MESSGE, FAIL, ctx)?;
        }
    }

    TSTLGS(&GOOD, &BAD, ctx);

    *OK = !FAIL;

    Ok(())
}
