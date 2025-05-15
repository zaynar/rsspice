//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure      PODREI ( Pod, remove elements, integer )
pub fn PODREI(N: i32, LOC: i32, POD: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
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
        spicelib::CHKIN(b"PODREI", ctx)?;
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
    PODONI(POD.as_slice(), &mut OFFSET, &mut NUMBER, ctx)?;

    if (N < 1) {
        spicelib::CHKOUT(b"PODREI", ctx)?;
        return Ok(());
    } else if (((LOC + N) - 1) > NUMBER) {
        spicelib::SETMSG(b"LOC = #; N = #; there are only # elements.", ctx);
        spicelib::ERRINT(b"#", LOC, ctx);
        spicelib::ERRINT(b"#", N, ctx);
        spicelib::ERRINT(b"#", NUMBER, ctx);
        spicelib::SIGERR(b"SPICE(NOTENOUGHPEAS)", ctx)?;
        spicelib::CHKOUT(b"PODREI", ctx)?;
        return Ok(());
    } else if ((LOC < 1) || (LOC > NUMBER)) {
        spicelib::SETMSG(b"Location (#) must be in the range [1,#].", ctx);
        spicelib::ERRINT(b"#", LOC, ctx);
        spicelib::ERRINT(b"#", NUMBER, ctx);
        spicelib::SIGERR(b"SPICE(BADPODLOCATION)", ctx)?;
        spicelib::CHKOUT(b"PODREI", ctx)?;
        return Ok(());
    }

    //
    // No problem. This is just like $Examples, above.
    //
    END = (OFFSET + NUMBER);

    spicelib::REMLAI(N, (OFFSET + LOC), POD.subarray_mut(1), &mut END, ctx)?;
    spicelib::SCARDI(END, POD.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"PODREI", ctx)?;
    Ok(())
}
