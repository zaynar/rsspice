//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCBUF: i32 = 0;

//$Procedure CBPUT ( Character buffer, put )
pub fn CBPUT_1(
    BEGIN: i32,
    END: i32,
    STRING: &[u8],
    BUFFER: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BUFFER = DummyCharArrayMut::new(BUFFER, None, LBCBUF..);
    let mut BUFLEN: i32 = 0;
    let mut L: i32 = 0;
    let mut B: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut LAST: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"CBPUT_1", ctx)?;

        if (((BEGIN < 1) || (END > SIZECB_1(BUFFER.as_arg(), ctx)?)) || (BEGIN > END)) {
            spicelib::SETMSG(b"Tried to access locations #:#.", ctx);
            spicelib::ERRINT(b"#", BEGIN, ctx);
            spicelib::ERRINT(b"#", END, ctx);

            spicelib::SIGERR(b"SPICE(CBNOSUCHSTR)", ctx)?;
            spicelib::CHKOUT(b"CBPUT_1", ctx)?;
            return Ok(());
        }
    }

    //
    // Storage begins at location B in line L.
    //
    BUFLEN = intrinsics::LEN(&BUFFER[1]);

    L = (((BEGIN - 1) / BUFLEN) + 1);
    B = (intrinsics::MOD((BEGIN - 1), BUFLEN) + 1);

    //
    // Assign one character at a time, changing output lines when
    // necessary, and assigning blanks if the input string should
    // come to an early end.
    //
    NEXT = 1;
    LAST = intrinsics::LEN(STRING);

    for I in BEGIN..=END {
        if (NEXT <= LAST) {
            fstr::assign(
                fstr::substr_mut(BUFFER.get_mut(L), B..=B),
                fstr::substr(STRING, NEXT..=NEXT),
            );
            NEXT = (NEXT + 1);
        } else {
            fstr::assign(fstr::substr_mut(BUFFER.get_mut(L), B..=B), b" ");
        }

        if (B < BUFLEN) {
            B = (B + 1);
        } else {
            L = (L + 1);
            B = 1;
        }
    }

    spicelib::CHKOUT(b"CBPUT_1", ctx)?;
    Ok(())
}
