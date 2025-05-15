//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 320;
const STYSIZ: i32 = 120;

//$Procedure      CHCKSD ( Check Scalar d.p. )
//
pub fn CHCKSD(
    NAME: &[u8],
    VAL: f64,
    COMP: &[u8],
    EXP: f64,
    TOL: f64,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut MESSGE = [b' '; LNSIZE as usize];
    let mut REL: f64 = 0.0;
    let mut FAIL: bool = false;
    let mut GOOD = [b' '; STYSIZ as usize];
    let mut BAD = [b' '; STYSIZ as usize];
    let mut VALNAN: bool = false;
    let mut EXPNAN: bool = false;

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
    fstr::assign(&mut MESSGE, b"This is a placeholder message that should have never been displayed. It indicates a CHCKSD bug.");

    FAIL = false;

    //
    // Test both actual and expected values to see whether they are NaN.
    //
    VALNAN = ((!(VAL > 1.0) && !(VAL < 1.0)) && !(VAL == 1.0));

    EXPNAN = ((!(EXP > 1.0) && !(EXP < 1.0)) && !(EXP == 1.0));

    if (VALNAN || EXPNAN) {
        //
        // At least one of the input values is NaN; the normal
        // comparisons won't tell us anything. We need to
        // construct a custom error message.
        //
        FAIL = true;

        if !VALNAN {
            //
            // Only EXP is NaN.
            //
            fstr::assign(
                &mut MESSGE,
                b"The value of # was #. The expected value was NaN.",
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", VAL, 14, &mut MESSGE, ctx);
        } else if !EXPNAN {
            //
            // Only VAL is NaN.
            //
            fstr::assign(
                &mut MESSGE,
                b"The value of # was NaN. The expected value was #.",
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
        } else {
            fstr::assign(
                &mut MESSGE,
                b"The value of # and the expected value were NaN.",
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
        }
    } else if fstr::eq(COMP, b"=") {
        FAIL = (VAL != EXP);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not the value expected./cr The value was:         # /cr the expected value was #. /crThe difference between these is: # .");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", VAL, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", (VAL - EXP), 14, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"<") {
        FAIL = (VAL >= EXP);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not less than # as was expected. /cr The value was #. /cr This is greater than # by #. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", VAL, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", (VAL - EXP), 14, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b">") {
        FAIL = (VAL <= EXP);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not greater than # as was expected. /cr The value was #. /crThis is less than # by #. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", VAL, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", (EXP - VAL), 14, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b">=") {
        FAIL = (VAL < EXP);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not greater than # as was expected. /cr The value was #./cr This is less than # by #. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", VAL, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", (EXP - VAL), 14, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"<=") {
        FAIL = (VAL > EXP);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not less than # as was expected. /cr The value was #./cr This is greater than # by #. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", VAL, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", (VAL - EXP), 14, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"!=") {
        FAIL = (VAL == EXP);

        if FAIL {
            fstr::assign(
                &mut MESSGE,
                b"The value of # should have been different from #./cr It wasn\'t. ",
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"~") {
        FAIL = (TOL < f64::abs((VAL - EXP)));

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not within # of #. /cr The value was #./cr The difference between the actual and expected value was #. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", TOL, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", VAL, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", (VAL - EXP), 14, &mut MESSGE, ctx);
        }
    } else if fstr::eq(COMP, b"~/") {
        if (VAL == EXP) {
            REL = 0.0;
        } else {
            REL = (f64::abs((VAL - EXP)) / intrinsics::DMAX1(&[f64::abs(VAL), f64::abs(EXP)]));
        }

        FAIL = (TOL < REL);

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was #.  The expected value was #. /cr The relative difference between # andits expected value was #./cr The maximum relative difference allowed for a successful test is #. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", VAL, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", REL, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", TOL, 14, &mut MESSGE, ctx);
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
            fstr::assign(&mut MESSGE, b"The comparison/cr/cr(3:) \'# # #\' /cr/cr(-3:) was satisfied to the specified tolerance: #./crThe value of # was #. ");

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
            spicelib::REPMD(&MESSGE.clone(), b"#", EXP, 14, &mut MESSGE, ctx);
            spicelib::REPMD(&MESSGE.clone(), b"#", TOL, 14, &mut MESSGE, ctx);
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMD(&MESSGE.clone(), b"#", VAL, 14, &mut MESSGE, ctx);

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
