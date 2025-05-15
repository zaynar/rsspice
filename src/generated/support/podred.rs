//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      PODRED ( Pod, remove elements, double precision )
pub fn PODRED(N: i32, LOC: i32, POD: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut POD = DummyArrayMut::new(POD, LBCELL..);
    let mut END: i32 = 0;
    let mut NUMBER: i32 = 0;
    let mut OFFSET: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"PODRED", ctx)?;
    }

    //
    // Three things can go `wrong':
    //
    //    1) No items to remove.
    //
    //    2) Too many items to remove.
    //
    //    3) No place to remove them from.
    //
    PODOND(POD.as_slice(), &mut OFFSET, &mut NUMBER, ctx)?;

    if (N < 1) {
        spicelib::CHKOUT(b"PODRED", ctx)?;
        return Ok(());
    } else if (((LOC + N) - 1) > NUMBER) {
        spicelib::SETMSG(b"LOC = #; N = #; there are only # elements.", ctx);
        spicelib::ERRINT(b"#", LOC, ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::ERRINT(b"#", NUMBER, ctx);
        spicelib::SIGERR(b"SPICE(NOTENOUGHPEAS)", ctx)?;
        spicelib::CHKOUT(b"PODRED", ctx)?;
        return Ok(());
    } else if ((LOC < 1) || (LOC > NUMBER)) {
        spicelib::SETMSG(b"Location (#) must be in the range [1,#].", ctx);
        spicelib::ERRINT(b"#", LOC, ctx);
        spicelib::ERRINT(b"#", NUMBER, ctx);
        spicelib::SIGERR(b"SPICE(BADPODLOCATION)", ctx)?;
        spicelib::CHKOUT(b"PODRED", ctx)?;
        return Ok(());
    }

    //
    // No problem. This is just like $Examples, above.
    //
    END = (OFFSET + NUMBER);

    spicelib::REMLAD(N, (OFFSET + LOC), POD.subarray_mut(1), &mut END, ctx)?;
    spicelib::SCARDD(END, POD.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"PODRED", ctx)?;
    Ok(())
}
