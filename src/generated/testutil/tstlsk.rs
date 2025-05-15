//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const LNSIZE: i32 = 80;
const NLINES: i32 = 29;

//$Procedure      TSTLSK ( Test Leapseconds Kernel )
pub fn TSTLSK(ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FILE = [b' '; LNSIZE as usize];
    let mut TEXT = ActualCharArray::new(LNSIZE, 1..=NLINES);

    BEGDAT(&mut TEXT[1]);
    fstr::assign(TEXT.get_mut(2), b" ");
    fstr::assign(TEXT.get_mut(3), b"DELTET/DELTA_T_A       =   32.184");
    fstr::assign(TEXT.get_mut(4), b"DELTET/K               =    1.657D-3");
    fstr::assign(TEXT.get_mut(5), b"DELTET/EB              =    1.671D-2");
    fstr::assign(
        TEXT.get_mut(6),
        b"DELTET/M               = (  6.239996D0   1.99096871D-7 )",
    );
    fstr::assign(TEXT.get_mut(7), b" ");
    fstr::assign(
        TEXT.get_mut(8),
        b"DELTET/DELTA_AT        = ( 10,   @1972-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(9),
        b"                           11,   @1972-JUL-1",
    );
    fstr::assign(
        TEXT.get_mut(10),
        b"                           12,   @1973-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(11),
        b"                           13,   @1974-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(12),
        b"                           14,   @1975-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(13),
        b"                           15,   @1976-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(14),
        b"                           16,   @1977-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(15),
        b"                           17,   @1978-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(16),
        b"                           18,   @1979-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(17),
        b"                           19,   @1980-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(18),
        b"                           20,   @1981-JUL-1",
    );
    fstr::assign(
        TEXT.get_mut(19),
        b"                           21,   @1982-JUL-1",
    );
    fstr::assign(
        TEXT.get_mut(20),
        b"                           22,   @1983-JUL-1",
    );
    fstr::assign(
        TEXT.get_mut(21),
        b"                           23,   @1985-JUL-1",
    );
    fstr::assign(
        TEXT.get_mut(22),
        b"                           24,   @1988-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(23),
        b"                           25,   @1990-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(24),
        b"                           26,   @1991-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(25),
        b"                           27,   @1992-JUL-1",
    );
    fstr::assign(
        TEXT.get_mut(26),
        b"                           28,   @1993-JUL-1",
    );
    fstr::assign(
        TEXT.get_mut(27),
        b"                           29,   @1994-JUL-1",
    );
    fstr::assign(
        TEXT.get_mut(28),
        b"                           30,   @1996-JAN-1",
    );
    fstr::assign(
        TEXT.get_mut(29),
        b"                           31,   @1997-JUL-1 )",
    );

    support::NEWFIL_1(b"lsk{0-9}{0-9}{0-9}{0-9}.tmp", &mut FILE, ctx)?;
    TSTTXT(&FILE, TEXT.as_arg(), NLINES, true, false, ctx)?;

    Ok(())
}
