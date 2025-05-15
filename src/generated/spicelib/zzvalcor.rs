//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;

//$Procedure ZZVALCOR ( Validate aberration correction )
pub fn ZZVALCOR(ABCORR: &[u8], ATTBLK: &mut [bool], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ATTBLK = DummyArrayMut::new(ATTBLK, 1..);

    //
    // SPICELIB functions
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZVALCOR", ctx)?;

    //
    // Parse the aberration correction string and obtain
    // an attribute block.
    //
    ZZPRSCOR(ABCORR, ATTBLK.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZVALCOR", ctx)?;
        return Ok(());
    }

    //
    // Check the attribute block. We don't allow relativistic
    // corrections.
    //
    if ATTBLK[RELIDX] {
        SETMSG(b"Aberration correction specification # calls for relativistic corrections, which are not supported.", ctx);
        ERRCH(b"#", ABCORR, ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(b"ZZVALCOR", ctx)?;
        return Ok(());
    }

    //
    // Stellar aberration corrections are allowed only if light
    // time corrections are specified as well.
    //
    if (ATTBLK[STLIDX] && !ATTBLK[LTIDX]) {
        SETMSG(b"Aberration correction specification # calls for stellar aberration correction without light time correction; this combination is not supported.", ctx);
        ERRCH(b"#", ABCORR, ctx);
        SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        CHKOUT(b"ZZVALCOR", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZVALCOR", ctx)?;
    Ok(())
}
