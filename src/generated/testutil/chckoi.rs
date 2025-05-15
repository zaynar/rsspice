//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LNSIZE: i32 = 240;
const STYSIZ: i32 = 120;

//$Procedure      CHCKOI ( Check order of an d.p. array )
pub fn CHCKOI(
    NAME: &[u8],
    ARRAY: &[i32],
    ORDER: &[u8],
    SIZE: i32,
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let ARRAY = DummyArray::new(ARRAY, 1..);
    let mut MESSGE = [b' '; LNSIZE as usize];
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut R1: i32 = 0;
    let mut R2: i32 = 0;
    let mut FAIL: bool = false;
    let mut GOOD = [b' '; STYSIZ as usize];
    let mut BAD = [b' '; STYSIZ as usize];

    //
    //
    // SPICELIB Functions
    //
    //
    // Test Utility Functions
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
    // So far the test case has not failed.
    //
    FAIL = false;
    J = (SIZE + 1);

    if fstr::eq(ORDER, b"=") {
        {
            let m1__: i32 = 1;
            let m2__: i32 = (SIZE - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                FAIL = (FAIL || !(ARRAY[I] == ARRAY[(I + 1)]));
                I += m3__;
            }
        }

        if FAIL {
            J = intrinsics::MIN0(&[J, I]);
        }
    } else if fstr::eq(ORDER, b"<") {
        {
            let m1__: i32 = 1;
            let m2__: i32 = (SIZE - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                FAIL = (FAIL || !(ARRAY[I] < ARRAY[(I + 1)]));
                I += m3__;
            }
        }

        if FAIL {
            J = intrinsics::MIN0(&[J, I]);
        }
    } else if fstr::eq(ORDER, b"<=") {
        {
            let m1__: i32 = 1;
            let m2__: i32 = (SIZE - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                FAIL = (FAIL || !(ARRAY[I] <= ARRAY[(I + 1)]));
                I += m3__;
            }
        }

        if FAIL {
            J = intrinsics::MIN0(&[J, I]);
        }
    } else if fstr::eq(ORDER, b">") {
        {
            let m1__: i32 = 1;
            let m2__: i32 = (SIZE - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                FAIL = (FAIL || !(ARRAY[I] > ARRAY[(I + 1)]));
                I += m3__;
            }
        }

        if FAIL {
            J = intrinsics::MIN0(&[J, I]);
        }
    } else if fstr::eq(ORDER, b"=>") {
        {
            let m1__: i32 = 1;
            let m2__: i32 = (SIZE - 1);
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                FAIL = (FAIL || !(ARRAY[I] >= ARRAY[(I + 1)]));
                I += m3__;
            }
        }

        if FAIL {
            J = intrinsics::MIN0(&[J, I]);
        }
    }

    R1 = spicelib::RTRIM(NAME);
    R2 = spicelib::RTRIM(ORDER);

    if FAIL {
        fstr::assign(&mut MESSGE, b"The input array # is does not satisfy the condition \"#(@) ? #($)\".  The two offending values are:/cr/cr(3:) #(@) = % /cr #($) = &. ");

        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(NAME, 1..=R1),
            &mut MESSGE,
        );
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(NAME, 1..=R1),
            &mut MESSGE,
        );
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(NAME, 1..=R1),
            &mut MESSGE,
        );
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(NAME, 1..=R1),
            &mut MESSGE,
        );
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(NAME, 1..=R1),
            &mut MESSGE,
        );
        spicelib::REPMC(
            &MESSGE.clone(),
            b"?",
            fstr::substr(ORDER, 1..=R2),
            &mut MESSGE,
        );
        spicelib::REPMI(&MESSGE.clone(), b"@", J, &mut MESSGE, ctx);
        spicelib::REPMI(&MESSGE.clone(), b"@", J, &mut MESSGE, ctx);
        spicelib::REPMI(&MESSGE.clone(), b"$", (J + 1), &mut MESSGE, ctx);
        spicelib::REPMI(&MESSGE.clone(), b"$", (J + 1), &mut MESSGE, ctx);
        spicelib::REPMI(&MESSGE.clone(), b"%", ARRAY[J], &mut MESSGE, ctx);
        spicelib::REPMI(&MESSGE.clone(), b"&", ARRAY[(J + 1)], &mut MESSGE, ctx);
        TSTLOG(b" ", FAIL, ctx)?;
        TSTLOG(&MESSGE, FAIL, ctx)?;
    } else if VERBOS(ctx) {
        fstr::assign(
            &mut MESSGE,
            b"The relationship #(I) ? #(I+1) is true for all elements of the input array # ",
        );

        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(NAME, 1..=R1),
            &mut MESSGE,
        );
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(NAME, 1..=R1),
            &mut MESSGE,
        );
        spicelib::REPMC(
            &MESSGE.clone(),
            b"#",
            fstr::substr(NAME, 1..=R1),
            &mut MESSGE,
        );
        spicelib::REPMC(
            &MESSGE.clone(),
            b"?",
            fstr::substr(ORDER, 1..=R2),
            &mut MESSGE,
        );
        TSTLOG(b" ", FAIL, ctx)?;
        TSTLOG(&MESSGE, FAIL, ctx)?;
    }

    TSTLGS(&GOOD, &BAD, ctx);
    *OK = !FAIL;

    Ok(())
}
