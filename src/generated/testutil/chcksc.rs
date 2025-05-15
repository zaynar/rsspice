//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 320;
const MAXLEN: i32 = 4000;
const STYSIZ: i32 = 120;

//$Procedure      CHCKSC ( Check scalar character string )
//
pub fn CHCKSC(
    NAME: &[u8],
    VAL: &[u8],
    COMP: &[u8],
    EXP: &[u8],
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut MESSGE = [b' '; LNSIZE as usize];
    let mut FAIL: bool = false;
    let mut LVAL: i32 = 0;
    let mut LEXP: i32 = 0;
    let mut MYVAL = vec![b' '; MAXLEN as usize];
    let mut MYEXP = vec![b' '; MAXLEN as usize];
    let mut MYCOMP = [b' '; 4 as usize];
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
    fstr::assign(&mut MESSGE, b"This is a placeholder message that should have never been displayed. It indicates a CHCKSC bug.");

    LVAL = intrinsics::LEN(VAL);
    LEXP = intrinsics::LEN(EXP);
    fstr::assign(&mut MYVAL, VAL);
    fstr::assign(&mut MYEXP, EXP);

    if fstr::eq(fstr::substr(COMP, 1..=1), b"~") {
        fstr::assign(&mut MYCOMP, fstr::substr(COMP, 2..));
        spicelib::UCASE(&MYVAL.clone(), &mut MYVAL, ctx);
        spicelib::UCASE(&MYEXP.clone(), &mut MYEXP, ctx);
    } else {
        fstr::assign(&mut MYCOMP, COMP);
    }

    FAIL = false;

    if fstr::eq(&MYCOMP, b"=") {
        FAIL = fstr::ne(
            fstr::substr(&MYVAL, 1..=LVAL),
            fstr::substr(&MYEXP, 1..=LEXP),
        );

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not the value expected./crThe value was:/cr(3:) \'#\' /cr(-3:) the expected value was:/cr(3:) \'#\'. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYVAL, 1..=LVAL),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYEXP, 1..=LEXP),
                &mut MESSGE,
            );
        }
    } else if fstr::eq(&MYCOMP, b"<") {
        FAIL = fstr::ge(
            fstr::substr(&MYVAL, 1..=LVAL),
            fstr::substr(&MYEXP, 1..=LEXP),
        );

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not less than \'#\' as was expected. /cr The value was: \'#\'. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYEXP, 1..=LEXP),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYVAL, 1..=LVAL),
                &mut MESSGE,
            );
        }
    } else if fstr::eq(&MYCOMP, b">") {
        FAIL = fstr::le(
            fstr::substr(&MYVAL, 1..=LVAL),
            fstr::substr(&MYEXP, 1..=LEXP),
        );

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not greater than \'#\' as was expected. /cr The value was: \'#\'. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYEXP, 1..=LEXP),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYVAL, 1..=LVAL),
                &mut MESSGE,
            );
        }
    } else if fstr::eq(&MYCOMP, b">=") {
        FAIL = fstr::lt(
            fstr::substr(&MYVAL, 1..=LVAL),
            fstr::substr(&MYEXP, 1..=LEXP),
        );

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not greater than \'#\' as was expected. /cr The value was: \'#\'. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYEXP, 1..=LEXP),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYVAL, 1..=LVAL),
                &mut MESSGE,
            );
        }
    } else if fstr::eq(&MYCOMP, b"<=") {
        FAIL = fstr::gt(
            fstr::substr(&MYVAL, 1..=LVAL),
            fstr::substr(&MYEXP, 1..=LEXP),
        );

        if FAIL {
            fstr::assign(&mut MESSGE, b"The value of # was not less than \'#\' as was expected. /cr The value was: \'#\'. ");
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYEXP, 1..=LEXP),
                &mut MESSGE,
            );
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYVAL, 1..=LVAL),
                &mut MESSGE,
            );
        }
    } else if fstr::eq(&MYCOMP, b"!=") {
        FAIL = fstr::eq(
            fstr::substr(&MYVAL, 1..=LVAL),
            fstr::substr(&MYEXP, 1..=LEXP),
        );

        if FAIL {
            fstr::assign(
                &mut MESSGE,
                b"The value of # should have been different from \'#\'./cr It wasn\'t. ",
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
                fstr::substr(&MYEXP, 1..=LEXP),
                &mut MESSGE,
            );
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
            fstr::assign(&mut MESSGE, b"The comparison/cr/cr(3:) # # \'#\' /cr/cr(-3:)was satisfied./crThe value of # was \'#\'. ");
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
            spicelib::REPMC(
                &MESSGE.clone(),
                b"#",
                fstr::substr(&MYEXP, 1..=LEXP),
                &mut MESSGE,
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
                fstr::substr(&MYVAL, 1..=LVAL),
                &mut MESSGE,
            );

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
