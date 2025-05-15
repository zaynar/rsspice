//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 320;
const STYSIZ: i32 = 120;

//$Procedure      CHCKSI ( Check Scalar Integer )
//
pub fn CHCKSI(
    NAME: &[u8],
    VAL: i32,
    COMP: &[u8],
    EXP: i32,
    TOL: i32,
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
    fstr::assign(&mut MESSGE, b"This is a placeholder message that should have never been displayed. It indicates a CHCKSI bug.");

    FAIL = false;

    if fstr::eq(COMP, b"=") {
        FAIL = (VAL != EXP);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not the value expected. The value was: # the expected value was #. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMI(&MESSGE.clone(), b"#", VAL, &mut MESSGE, ctx);
            spicelib::REPMI(&MESSGE.clone(), b"#", EXP, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"<") {
        FAIL = (VAL >= EXP);

        if FAIL {
            fstr::assign(
                &mut MESSGE,
                b"The value of # was not less than # as was expected.  The value was #. ",
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMI(&MESSGE.clone(), b"#", EXP, &mut MESSGE, ctx);
            spicelib::REPMI(&MESSGE.clone(), b"#", VAL, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b">") {
        FAIL = (VAL <= EXP);

        if FAIL {
            fstr::assign(
                &mut MESSGE,
                b"The value of # was not greater than # as was expected.  The value was #. ",
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMI(&MESSGE.clone(), b"#", EXP, &mut MESSGE, ctx);
            spicelib::REPMI(&MESSGE.clone(), b"#", VAL, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b">=") {
        FAIL = (VAL < EXP);

        if FAIL {
            fstr::assign(
                &mut MESSGE,
                b"The value of # was not greater than # as was expected.  The value was #. ",
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMI(&MESSGE.clone(), b"#", EXP, &mut MESSGE, ctx);
            spicelib::REPMI(&MESSGE.clone(), b"#", VAL, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"<=") {
        FAIL = (VAL > EXP);

        if FAIL {
            fstr::assign(
                &mut MESSGE,
                b"The value of # was not less than # as was expected.  The value was #. ",
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMI(&MESSGE.clone(), b"#", EXP, &mut MESSGE, ctx);
            spicelib::REPMI(&MESSGE.clone(), b"#", VAL, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"!=") {
        FAIL = (VAL == EXP);

        if FAIL {
            fstr::assign(
                &mut MESSGE,
                b"The value of # should have been different from #. It wasn\'t. ",
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMI(&MESSGE.clone(), b"#", EXP, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"~") {
        FAIL = (TOL < i32::abs((VAL - EXP)));

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not within # of #.  The value was # . The difference between the actual and expected value was #. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMI(&MESSGE.clone(), b"#", TOL, &mut MESSGE, ctx);
            spicelib::REPMI(&MESSGE.clone(), b"#", EXP, &mut MESSGE, ctx);
            spicelib::REPMI(&MESSGE.clone(), b"#", VAL, &mut MESSGE, ctx);
            spicelib::REPMI(&MESSGE.clone(), b"#", (VAL - EXP), &mut MESSGE, ctx);
        }
    } else {
        FAIL = true;

        fstr::assign(&mut MESSGE, b"The comparison # is not recognized. ");
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(COMP, 1..=spicelib::RTRIM(COMP)),
            &mut MESSGE,
        );
    }

    if !FAIL {
        if VERBOS(ctx) {
            fstr::assign(
                &mut MESSGE,
                b"The comparison \'# # #\' was satisfied. The value of # was #. ",
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(COMP, 1..=spicelib::RTRIM(COMP)),
                &mut MESSGE,
            );
            spicelib::REPMI(&MESSGE.clone(), b"#", EXP, &mut MESSGE, ctx);
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMI(&MESSGE.clone(), b"#", VAL, &mut MESSGE, ctx);

            TSTLOG(b" ", FAIL, ctx)?;
            TSTLOG(&MESSGE, FAIL, ctx)?;
        }
    } else {
        TSTLOG(b" ", FAIL, ctx)?;
        TSTLOG(&MESSGE, FAIL, ctx)?;
    }

    TSTLGS(&GOOD, &BAD, ctx);
    *OK = !FAIL;

    Ok(())
}
