//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure LBUPD ( Line buffer, update )
pub fn LBUPD_1(
    NLINE: i32,
    NCOM: i32,
    PTRS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PTRS = DummyArrayMut::new(PTRS, LBCELL..);

    //
    // SPICELIB functions
    //

    //
    // Standard error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"LBUPD_1", ctx)?;

        if (((NLINE < 0) || (NCOM < 1))
            || (((NLINE + NCOM) * 2) > spicelib::SIZEI(PTRS.as_slice(), ctx)?))
        {
            spicelib::SETMSG(b"Tried to store # lines, # holes.", ctx);
            spicelib::ERRINT(b"#", NLINE, ctx);
            spicelib::ERRINT(b"#", NCOM, ctx);
            spicelib::SIGERR(b"SPICE(LBCORRUPTED)", ctx)?;
            spicelib::CHKOUT(b"LBUPD_1", ctx)?;
            return Ok(());
        }
    }

    //
    // Save the current number of lines in element -2. We can infer the
    // cardinality of the cell from the total number of intervals.
    //
    PTRS[-2] = NLINE;

    spicelib::SCARDI((2 * (NLINE + NCOM)), PTRS.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"LBUPD_1", ctx)?;
    Ok(())
}
