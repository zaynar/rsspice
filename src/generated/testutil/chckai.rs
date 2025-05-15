//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 800;
const STYSIZ: i32 = 120;

//$Procedure      CHCKAI ( Check an array of integers )
pub fn CHCKAI(
    NAME: &[u8],
    ARRAY: &[i32],
    COMP: &[u8],
    EXP: &[i32],
    SIZE: i32,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let EXP = DummyArray::new(EXP, 1..);
    let mut MESSGE = [b' '; LNSIZE as usize];
    let mut FAIL: bool = false;
    let mut GOOD = [b' '; STYSIZ as usize];
    let mut BAD = [b' '; STYSIZ as usize];

    //
    // SPICELIB functions
    //
    //
    // Testing Utility Functions
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
    fstr::assign(&mut MESSGE, b"This is a placeholder message that should have never been displayed. It indicates a CHCKAI bug.");

    FAIL = false;

    if fstr::eq(COMP, b"=") {
        for I in 1..=SIZE {
            FAIL = (ARRAY[I] != EXP[I]);

            if FAIL {
                fstr::assign(&mut MESSGE, b"Value # of array # was not the value expected. /cr(3:)/cr The value was:         # /crthe expected value was #./crThe difference between these is: # . ");
                spicelib::REPMI(&MESSGE.clone(), b"#", I, &mut MESSGE, ctx);
                spicelib::REPMC(
                    &MESSGE.clone(),
                    b"#",
                    fstr::substr(NAME, 1..=spicelib::RTRIM(NAME)),
                    &mut MESSGE,
                );
                spicelib::REPMI(&MESSGE.clone(), b"#", ARRAY[I], &mut MESSGE, ctx);
                spicelib::REPMI(&MESSGE.clone(), b"#", EXP[I], &mut MESSGE, ctx);
                spicelib::REPMI(&MESSGE.clone(), b"#", (ARRAY[I] - EXP[I]), &mut MESSGE, ctx);
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
        if VERBOS(ctx) {
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
