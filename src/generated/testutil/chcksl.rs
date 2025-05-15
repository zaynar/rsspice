//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 320;
const STYSIZ: i32 = 120;

//$Procedure      CHCKSL ( Check Scalar logical )
//
pub fn CHCKSL(
    NAME: &[u8],
    VAL: bool,
    EXP: bool,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut MESSGE = [b' '; LNSIZE as usize];
    let mut FAIL: bool = false;
    let mut GOOD = [b' '; STYSIZ as usize];
    let mut BAD = [b' '; STYSIZ as usize];

    //

    //
    // Test Utility functions
    //

    //
    // SPICELIB functions
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

    //
    // Set placeholder message.
    //
    fstr::assign(&mut MESSGE, b"This is a placeholder message that should have never been displayed. It indicates a CHCKSL bug.");

    //
    // Just make sure that the values of VAL and EXP are the same.
    //
    if EXP {
        *OK = VAL;

        if *OK {
            if VERBOS(ctx) {
                fstr::assign(&mut MESSGE, b"The value of # was TRUE as expected. ");
            }
        } else {
            fstr::assign(
                &mut MESSGE,
                b"The expected value of # was TRUE.  The actual value was FALSE",
            );
        }
    } else {
        *OK = !VAL;

        if *OK {
            if VERBOS(ctx) {
                fstr::assign(&mut MESSGE, b"The value of # was FALSE as expected. ");
            }
        } else {
            fstr::assign(
                &mut MESSGE,
                b"The expected value of # was FALSE.  The actual value was TRUE",
            );
        }
    }

    FAIL = !*OK;

    if (FAIL || VERBOS(ctx)) {
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
            &mut MESSGE,
        );
    }

    if *OK {
        if VERBOS(ctx) {
            TSTLOG(b" ", FAIL, ctx)?;
            TSTLOG(&MESSGE, FAIL, ctx)?;
        }
    } else {
        TSTLOG(b" ", FAIL, ctx)?;
        TSTLOG(&MESSGE, FAIL, ctx)?;
    }

    TSTLGS(&GOOD, &BAD, ctx);

    Ok(())
}
