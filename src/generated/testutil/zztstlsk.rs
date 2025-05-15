//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LNSIZE: i32 = 80;
const NLINES: i32 = 29;

//$Procedure TSTLSK (Create a test LSK)
pub fn ZZTSTLSK(NAMELS: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LSK = ActualCharArray::new(LNSIZE, 1..=NLINES);
    let mut LSKLN = [b' '; LNSIZE as usize];
    let mut R: i32 = 0;
    let mut UNIT: i32 = 0;

    //
    // Spicelib Functions
    //

    //
    // Test Utility Functions
    //

    //
    // Local Variables.
    //

    KILFIL(NAMELS, ctx)?;

    BEGDAT(&mut LSK[1]);
    fstr::assign(LSK.get_mut(2), b" ");
    fstr::assign(LSK.get_mut(3), b"DELTET/DELTA_T_A       =   32.184");
    fstr::assign(LSK.get_mut(4), b"DELTET/K               =    1.657D-3");
    fstr::assign(LSK.get_mut(5), b"DELTET/EB              =    1.671D-2");
    fstr::assign(
        LSK.get_mut(6),
        b"DELTET/M               = (  6.239996D0   1.99096871D-7 )",
    );
    fstr::assign(LSK.get_mut(7), b" ");
    fstr::assign(
        LSK.get_mut(8),
        b"DELTET/DELTA_AT        = ( 10,   @1972-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(9),
        b"                           11,   @1972-JUL-1",
    );
    fstr::assign(
        LSK.get_mut(10),
        b"                           12,   @1973-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(11),
        b"                           13,   @1974-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(12),
        b"                           14,   @1975-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(13),
        b"                           15,   @1976-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(14),
        b"                           16,   @1977-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(15),
        b"                           17,   @1978-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(16),
        b"                           18,   @1979-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(17),
        b"                           19,   @1980-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(18),
        b"                           20,   @1981-JUL-1",
    );
    fstr::assign(
        LSK.get_mut(19),
        b"                           21,   @1982-JUL-1",
    );
    fstr::assign(
        LSK.get_mut(20),
        b"                           22,   @1983-JUL-1",
    );
    fstr::assign(
        LSK.get_mut(21),
        b"                           23,   @1985-JUL-1",
    );
    fstr::assign(
        LSK.get_mut(22),
        b"                           24,   @1988-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(23),
        b"                           25,   @1990-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(24),
        b"                           26,   @1991-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(25),
        b"                           27,   @1992-JUL-1",
    );
    fstr::assign(
        LSK.get_mut(26),
        b"                           28,   @1993-JUL-1",
    );
    fstr::assign(
        LSK.get_mut(27),
        b"                           29,   @1994-JUL-1",
    );
    fstr::assign(
        LSK.get_mut(28),
        b"                           30,   @1996-JAN-1",
    );
    fstr::assign(
        LSK.get_mut(29),
        b"                           31,   @1997-JUL-1 )",
    );

    //
    // Create the LSK kernel.
    //
    spicelib::TXTOPN(NAMELS, &mut UNIT, ctx)?;

    for I in 1..=NLINES {
        fstr::assign(&mut LSKLN, LSK.get(I));
        R = spicelib::RTRIM(&LSKLN);
        spicelib::WRITLN(fstr::substr(&LSKLN, 1..=R), UNIT, ctx)?;
    }

    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UNIT),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    Ok(())
}
