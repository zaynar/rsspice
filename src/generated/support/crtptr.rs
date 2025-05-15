//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const SYMLEN: i32 = 1024;

//$Procedure                  CRTPTR (Create pointer)
pub fn CRTPTR(
    BASE: &[u8],
    INDEX: i32,
    PNTER: &[u8],
    CRTPTR: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut CNUM = [b' '; 10 as usize];
    let mut SYM = [b' '; SYMLEN as usize];
    let mut BLEN: i32 = 0;
    let mut CLEN: i32 = 0;
    let mut PLEN: i32 = 0;
    let mut TOTAL: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // This routine will use discovery check-in.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    //
    // Compute the lengths of the strings involved.
    //
    spicelib::INTSTR(INDEX, &mut CNUM, ctx);

    CLEN = spicelib::RTRIM(&CNUM);
    BLEN = spicelib::RTRIM(BASE);
    PLEN = spicelib::RTRIM(PNTER);
    TOTAL = (((CLEN + BLEN) + PLEN) + 2);

    //
    // TOTAL must be SYMLEN characters, or fewer.
    //
    if (TOTAL > SYMLEN) {
        spicelib::CHKIN(b"CRTPTR", ctx)?;
        spicelib::SETMSG(
            b"Symbol exceeds # characters. Increase the value of SYMLEN.",
            ctx,
        );
        spicelib::ERRINT(b"#", SYMLEN, ctx);
        spicelib::SIGERR(b"SPICE(BUFFERTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"CRTPTR", ctx)?;
        return Ok(());
    }

    //
    // And TOTAL must be LEN(CRTPTR) characters, or fewer.
    //
    if (TOTAL > intrinsics::LEN(CRTPTR)) {
        spicelib::CHKIN(b"CRTPTR", ctx)?;
        spicelib::SETMSG(b"Symbol exceeds the dimension of CRTPTR.", ctx);
        spicelib::SIGERR(b"SPICE(DIMENSIONTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"CRTPTR", ctx)?;
        return Ok(());
    }

    //
    // Form the symbol 'BASE~INDEX~PNTER'.
    //
    fstr::assign(&mut SYM, b" ");
    fstr::assign(
        &mut SYM,
        &fstr::concat(
            &fstr::concat(
                &fstr::concat(
                    &fstr::concat(fstr::substr(BASE, 1..=BLEN), b"~"),
                    fstr::substr(&CNUM, 1..=CLEN),
                ),
                b"~",
            ),
            fstr::substr(PNTER, 1..=PLEN),
        ),
    );

    fstr::assign(CRTPTR, &SYM);

    Ok(())
}
