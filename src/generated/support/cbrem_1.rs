//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCBUF: i32 = 0;

//$Procedure CBREM ( Character buffer, remove )
pub fn CBREM_1(
    BEGIN: i32,
    END: i32,
    BUFFER: CharArrayMut,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BUFFER = DummyCharArrayMut::new(BUFFER, None, LBCBUF..);
    let mut BUFLEN: i32 = 0;
    let mut ENDBUF: i32 = 0;
    let mut L: i32 = 0;
    let mut B: i32 = 0;
    let mut NL: i32 = 0;
    let mut NB: i32 = 0;

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
        spicelib::CHKIN(b"CBREM_1", ctx)?;

        if (((BEGIN < 1) || (END > SIZECB_1(BUFFER.as_arg(), ctx)?)) || (BEGIN > END)) {
            spicelib::SETMSG(b"Tried to access locations #:#.", ctx);
            spicelib::ERRINT(b"#", BEGIN, ctx);
            spicelib::ERRINT(b"#", END, ctx);

            spicelib::SIGERR(b"SPICE(CBNOSUCHSTR)", ctx)?;
            spicelib::CHKOUT(b"CBREM_1", ctx)?;
            return Ok(());
        }
    }

    //
    // Essential limits.
    //
    BUFLEN = intrinsics::LEN(&BUFFER[1]);
    ENDBUF = SIZECB_1(BUFFER.as_arg(), ctx)?;

    //
    // Each guy gets moved from location B in line L to location NB
    // in line NL. (N stands for New.)
    //
    L = ((END / BUFLEN) + 1);
    B = (intrinsics::MOD(END, BUFLEN) + 1);

    NL = (((BEGIN - 1) / BUFLEN) + 1);
    NB = (intrinsics::MOD((BEGIN - 1), BUFLEN) + 1);

    for I in (END + 1)..=ENDBUF {
        let val = fstr::substr(BUFFER.get(L), B..=B).to_vec();
        fstr::assign(fstr::substr_mut(BUFFER.get_mut(NL), NB..=NB), &val);

        if (B < BUFLEN) {
            B = (B + 1);
        } else {
            L = (L + 1);
            B = 1;
        }

        if (NB < BUFLEN) {
            NB = (NB + 1);
        } else {
            NL = (NL + 1);
            NB = 1;
        }
    }

    //
    // Now we can just overwrite the vacated space at the end.
    //
    CBPUT_1(
        (ENDBUF - (END - BEGIN)),
        ENDBUF,
        b" ",
        BUFFER.as_arg_mut(),
        ctx,
    )?;

    spicelib::CHKOUT(b"CBREM_1", ctx)?;
    Ok(())
}
