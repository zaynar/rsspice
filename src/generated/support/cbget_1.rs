//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCBUF: i32 = 0;

//$Procedure CBGET ( Character buffer, get )
pub fn CBGET_1(
    BEGIN: i32,
    END: i32,
    BUFFER: CharArray,
    STRING: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let BUFFER = DummyCharArray::new(BUFFER, None, LBCBUF..);
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
        spicelib::CHKIN(b"CBGET_1", ctx)?;

        if (((BEGIN < 1) || (END > SIZECB_1(BUFFER.as_arg(), ctx)?)) || (BEGIN > END)) {
            spicelib::SETMSG(b"Tried to access locations #:#.", ctx);
            spicelib::ERRINT(b"#", BEGIN, ctx);
            spicelib::ERRINT(b"#", END, ctx);

            spicelib::SIGERR(b"SPICE(CBNOSUCHSTR)", ctx)?;
            spicelib::CHKOUT(b"CBGET_1", ctx)?;
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
    // Assign one character at a time, changing input lines when
    // necessary. Do not assign any characters beyond the end of
    // the output string.
    //
    NEXT = 1;
    LAST = intrinsics::LEN(STRING);

    for I in BEGIN..=END {
        if (NEXT <= LAST) {
            fstr::assign(
                fstr::substr_mut(STRING, NEXT..=NEXT),
                fstr::substr(BUFFER.get(L), B..=B),
            );
            NEXT = (NEXT + 1);
        }

        if (B < BUFLEN) {
            B = (B + 1);
        } else {
            L = (L + 1);
            B = 1;
        }
    }

    //
    // Pad the output string with blanks, if necessary.
    //
    if (NEXT <= LAST) {
        fstr::assign(fstr::substr_mut(STRING, NEXT..), b" ");
    }

    spicelib::CHKOUT(b"CBGET_1", ctx)?;
    Ok(())
}
