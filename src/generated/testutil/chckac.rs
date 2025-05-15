//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 320;
const MAXLEN: i32 = 4000;
const STYSIZ: i32 = 120;

//$Procedure CHCKAC ( Check an array of characters )
pub fn CHCKAC(
    NAME: &[u8],
    ARRAY: CharArray,
    COMP: &[u8],
    EXP: CharArray,
    SIZE: i32,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ARRAY = DummyCharArray::new(ARRAY, None, 1..);
    let EXP = DummyCharArray::new(EXP, None, 1..);
    let mut MESSGE = [b' '; LNSIZE as usize];
    let mut GOOD = [b' '; STYSIZ as usize];
    let mut BAD = [b' '; STYSIZ as usize];
    let mut MYCMP = [b' '; 2 as usize];
    let mut MYVAL = vec![b' '; MAXLEN as usize];
    let mut MYEXP = vec![b' '; MAXLEN as usize];
    let mut FAIL: bool = false;
    let mut SN2CAS: bool = false;
    let mut LVAL: i32 = 0;
    let mut LEXP: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Testing Utility Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Setup style strings?
    //
    TSTSTY(&mut GOOD, &mut BAD, ctx);
    TSTLGS(
        b"LEFT 3 RIGHT 75 NEWLINE /cr ",
        b"LEFT 3 RIGHT 75 NEWLINE /cr FLAG --- LEADER ---",
        ctx,
    );

    //
    // Set FAIL to .FALSE.
    //
    FAIL = false;

    //
    // Are we doing case-insensitive checks?
    //
    if fstr::eq(fstr::substr(COMP, 1..=1), b"~") {
        SN2CAS = false;
        fstr::assign(&mut MYCMP, fstr::substr(COMP, 2..));
    } else {
        SN2CAS = true;
        fstr::assign(&mut MYCMP, COMP);
    }

    //
    // Set the length parameters.
    //
    LVAL = intrinsics::LEN(&ARRAY[1]);
    LEXP = intrinsics::LEN(&EXP[1]);

    //
    // Do the comparisons.
    //
    if fstr::eq(&MYCMP, b"=") {
        for I in 1..=SIZE {
            if !SN2CAS {
                spicelib::UCASE(&ARRAY[I], &mut MYVAL, ctx);
                spicelib::UCASE(&EXP[I], &mut MYEXP, ctx);
            } else {
                fstr::assign(&mut MYVAL, ARRAY.get(I));
                fstr::assign(&mut MYEXP, EXP.get(I));
            }

            FAIL = fstr::ne(
                fstr::substr(&MYVAL, 1..=LVAL),
                fstr::substr(&MYEXP, 1..=LEXP),
            );

            if FAIL {
                fstr::assign(&mut MESSGE, b"Value # of array # was not the value expected. /cr(3:)/cr The value was:         # /crthe expected value was #./cr");

                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMC(&MESSGE.clone(), b"#", &ARRAY[I], &mut MESSGE);
                spicelib::REPMC(&MESSGE.clone(), b"#", &EXP[I], &mut MESSGE);

                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLGS(&GOOD, &BAD, ctx);

                *OK = !FAIL;

                return Ok(());
            }
        }
    } else if fstr::eq(&MYCMP, b">") {
        for I in 1..=SIZE {
            if !SN2CAS {
                spicelib::UCASE(&ARRAY[I], &mut MYVAL, ctx);
                spicelib::UCASE(&EXP[I], &mut MYEXP, ctx);
            } else {
                fstr::assign(&mut MYVAL, ARRAY.get(I));
                fstr::assign(&mut MYEXP, EXP.get(I));
            }

            FAIL = fstr::le(
                fstr::substr(&MYVAL, 1..=LVAL),
                fstr::substr(&MYEXP, 1..=LEXP),
            );

            if FAIL {
                fstr::assign(&mut MESSGE, b"Value # of array # was not greater than the value expected. /cr(3:)/cr The value was:         # /crthe expected value was #./cr");

                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMC(&MESSGE.clone(), b"#", &ARRAY[I], &mut MESSGE);
                spicelib::REPMC(&MESSGE.clone(), b"#", &EXP[I], &mut MESSGE);

                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLGS(&GOOD, &BAD, ctx);

                *OK = !FAIL;

                return Ok(());
            }
        }
    } else if fstr::eq(&MYCMP, b"<") {
        for I in 1..=SIZE {
            if !SN2CAS {
                spicelib::UCASE(&ARRAY[I], &mut MYVAL, ctx);
                spicelib::UCASE(&EXP[I], &mut MYEXP, ctx);
            } else {
                fstr::assign(&mut MYVAL, ARRAY.get(I));
                fstr::assign(&mut MYEXP, EXP.get(I));
            }

            FAIL = fstr::ge(
                fstr::substr(&MYVAL, 1..=LVAL),
                fstr::substr(&MYEXP, 1..=LEXP),
            );

            if FAIL {
                fstr::assign(&mut MESSGE, b"Value # of array # was not less than the value expected. /cr(3:)/cr The value was:         # /crthe expected value was #./cr");

                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMC(&MESSGE.clone(), b"#", &ARRAY[I], &mut MESSGE);
                spicelib::REPMC(&MESSGE.clone(), b"#", &EXP[I], &mut MESSGE);

                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLGS(&GOOD, &BAD, ctx);

                *OK = !FAIL;

                return Ok(());
            }
        }
    } else if fstr::eq(&MYCMP, b">=") {
        for I in 1..=SIZE {
            if !SN2CAS {
                spicelib::UCASE(&ARRAY[I], &mut MYVAL, ctx);
                spicelib::UCASE(&EXP[I], &mut MYEXP, ctx);
            } else {
                fstr::assign(&mut MYVAL, ARRAY.get(I));
                fstr::assign(&mut MYEXP, EXP.get(I));
            }

            FAIL = fstr::lt(
                fstr::substr(&MYVAL, 1..=LVAL),
                fstr::substr(&MYEXP, 1..=LEXP),
            );

            if FAIL {
                fstr::assign(&mut MESSGE, b"Value # of array # was not greater than or equal to the value expected. /cr(3:)/cr The value was:         # /crthe expected value was #./cr");

                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMC(&MESSGE.clone(), b"#", &ARRAY[I], &mut MESSGE);
                spicelib::REPMC(&MESSGE.clone(), b"#", &EXP[I], &mut MESSGE);

                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLGS(&GOOD, &BAD, ctx);

                *OK = !FAIL;

                return Ok(());
            }
        }
    } else if fstr::eq(&MYCMP, b"<=") {
        for I in 1..=SIZE {
            if !SN2CAS {
                spicelib::UCASE(&ARRAY[I], &mut MYVAL, ctx);
                spicelib::UCASE(&EXP[I], &mut MYEXP, ctx);
            } else {
                fstr::assign(&mut MYVAL, ARRAY.get(I));
                fstr::assign(&mut MYEXP, EXP.get(I));
            }

            FAIL = fstr::gt(
                fstr::substr(&MYVAL, 1..=LVAL),
                fstr::substr(&MYEXP, 1..=LEXP),
            );

            if FAIL {
                fstr::assign(&mut MESSGE, b"Value # of array # was not less than or equal to the value expected. /cr(3:)/cr The value was:         # /crthe expected value was #./cr");

                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMC(&MESSGE.clone(), b"#", &ARRAY[I], &mut MESSGE);
                spicelib::REPMC(&MESSGE.clone(), b"#", &EXP[I], &mut MESSGE);

                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLGS(&GOOD, &BAD, ctx);

                *OK = !FAIL;

                return Ok(());
            }
        }
    } else if fstr::eq(&MYCMP, b"!=") {
        for I in 1..=SIZE {
            if !SN2CAS {
                spicelib::UCASE(&ARRAY[I], &mut MYVAL, ctx);
                spicelib::UCASE(&EXP[I], &mut MYEXP, ctx);
            } else {
                fstr::assign(&mut MYVAL, ARRAY.get(I));
                fstr::assign(&mut MYEXP, EXP.get(I));
            }

            FAIL = fstr::eq(
                fstr::substr(&MYVAL, 1..=LVAL),
                fstr::substr(&MYEXP, 1..=LEXP),
            );

            if FAIL {
                fstr::assign(&mut MESSGE, b"Value # of array # should have been different from the expected value. /cr(3:)/cr The value was:         # /crthe expected value was #./cr");

                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMC(&MESSGE.clone(), b"#", &ARRAY[I], &mut MESSGE);
                spicelib::REPMC(&MESSGE.clone(), b"#", &EXP[I], &mut MESSGE);

                TSTLOG(b" ", FAIL, ctx)?;
                TSTLOG(&MESSGE, FAIL, ctx)?;
                TSTLGS(&GOOD, &BAD, ctx);

                *OK = !FAIL;

                return Ok(());
            }
        }
    } else {
        FAIL = true;

        fstr::assign(&mut MESSGE, b"The comparison \"#\" is not recognized. ");
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(COMP, 1..=spicelib::RTRIM(COMP)),
            &mut MESSGE,
        );
    }

    if !FAIL {
        fstr::assign(
            &mut MESSGE,
            b"The comparison /cr/cr(3:)\'# # EXPECTED\' /cr/cr(-3:) was satisfied.",
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

        if VERBOS(ctx) {
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
