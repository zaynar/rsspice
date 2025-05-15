//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure ZZPSUN ( Plate set union )
pub fn ZZPSUN(
    V1: &[f64],
    P1: &[i32],
    V2: &[f64],
    P2: &[i32],
    VOUT: &mut [f64],
    POUT: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let V1 = DummyArray::new(V1, LBCELL..);
    let P1 = DummyArray::new(P1, LBCELL..);
    let V2 = DummyArray::new(V2, LBCELL..);
    let P2 = DummyArray::new(P2, LBCELL..);
    let mut VOUT = DummyArrayMut::new(VOUT, LBCELL..);
    let mut POUT = DummyArrayMut::new(POUT, LBCELL..);
    let mut CP1: i32 = 0;
    let mut CP2: i32 = 0;
    let mut CPOUT: i32 = 0;
    let mut CV1: i32 = 0;
    let mut CV2: i32 = 0;
    let mut CVOUT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZPSUN", ctx)?;

    //
    // Check room in output vertex array.
    //
    CV1 = spicelib::CARDD(V1.as_slice(), ctx)?;
    CV2 = spicelib::CARDD(V2.as_slice(), ctx)?;
    CVOUT = (CV1 + CV2);

    if (spicelib::SIZED(VOUT.as_slice(), ctx)? < CVOUT) {
        spicelib::SETMSG(b"Output vertex array is too small to contain union of input vertex arrays. Output vertex array size is #; required space is # elements.", ctx);
        spicelib::ERRINT(b"#", spicelib::SIZED(VOUT.as_slice(), ctx)?, ctx);
        spicelib::ERRINT(b"#", CVOUT, ctx);
        spicelib::SIGERR(b"SPICE(VERTARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZPSUN", ctx)?;
        return Ok(());
    }

    //
    // Check room in output plate array.
    //
    CP1 = spicelib::CARDI(P1.as_slice(), ctx)?;
    CP2 = spicelib::CARDI(P2.as_slice(), ctx)?;
    CPOUT = (CP1 + CP2);

    if (spicelib::SIZEI(POUT.as_slice(), ctx)? < CPOUT) {
        spicelib::SETMSG(b"Output plate array is too small to contain union of input plate arrays. Output plate array size is #; required space is # elements.", ctx);
        spicelib::ERRINT(b"#", spicelib::SIZEI(POUT.as_slice(), ctx)?, ctx);
        spicelib::ERRINT(b"#", CPOUT, ctx);
        spicelib::SIGERR(b"SPICE(PLTARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZPSUN", ctx)?;
        return Ok(());
    }

    //
    // Copy the vertices and plates of the first input plate set
    // to the output plate set.
    //
    spicelib::COPYD(V1.as_slice(), VOUT.as_slice_mut(), ctx)?;
    spicelib::COPYI(P1.as_slice(), POUT.as_slice_mut(), ctx)?;

    //
    // Append the vertices of the second input plate set to the
    // output plate set. We've already verified there's enough
    // room in the output set.
    //
    spicelib::MOVED(V2.subarray(1), CV2, VOUT.subarray_mut((CV1 + 1)));

    spicelib::SCARDD((CV1 + CV2), VOUT.as_slice_mut(), ctx)?;

    //
    // In the output plate set, the vertex indices of the
    // second input plate set must be shifted by the number
    // of vertices in the first input vertex set.
    //
    // Shift and append the plates of the second input plate set to the
    // output plate set. We've already verified there's enough room in
    // the output set.
    //
    for I in 1..=CP2 {
        POUT[(CP1 + I)] = (P2[I] + (CV1 / 3));
    }

    spicelib::SCARDI((CP1 + CP2), POUT.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"ZZPSUN", ctx)?;
    Ok(())
}
