//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure ZZPSXLAT ( Plate set, translate )
pub fn ZZPSXLAT(
    V1: &[f64],
    OFFSET: &[f64],
    VOUT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let V1 = DummyArray::new(V1, LBCELL..);
    let OFFSET = DummyArray::new(OFFSET, 1..=3);
    let mut VOUT = DummyArrayMut::new(VOUT, LBCELL..);
    let mut CV1: i32 = 0;
    let mut J: i32 = 0;
    let mut NV: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZPSXLAT", ctx)?;

    //
    // Make sure the cardinality of the input vertex set is a
    // multiple of 3.
    //
    CV1 = spicelib::CARDD(V1.as_slice(), ctx)?;

    NV = (CV1 / 3);

    if ((NV * 3) != CV1) {
        spicelib::SETMSG(
            b"Input vertex set cardinality # is not a multiple of 3.",
            ctx,
        );
        spicelib::ERRINT(b"#", CV1, ctx);
        spicelib::SIGERR(b"SPICE(BADVERTEXARRAY)", ctx)?;
        spicelib::CHKOUT(b"ZZPSXLAT", ctx)?;
        return Ok(());
    }

    //
    // Check room in output vertex array.
    //
    CV1 = spicelib::CARDD(V1.as_slice(), ctx)?;

    if (spicelib::SIZED(VOUT.as_slice(), ctx)? < CV1) {
        spicelib::SETMSG(
            b"Output vertex array size is #; required space is # elements.",
            ctx,
        );
        spicelib::ERRINT(b"#", spicelib::SIZED(VOUT.as_slice(), ctx)?, ctx);
        spicelib::ERRINT(b"#", CV1, ctx);
        spicelib::SIGERR(b"SPICE(VERTARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZPSXLAT", ctx)?;
        return Ok(());
    }

    //
    // Add the offset vector to each vertex of the input plate set and
    // append the result to the output plate set. We've already
    // verified there's enough room in the output set.
    //
    for I in 1..=NV {
        J = (1 + ((I - 1) * 3));

        spicelib::VADD(OFFSET.as_slice(), V1.subarray(J), VOUT.subarray_mut(J));
    }

    spicelib::SCARDD(CV1, VOUT.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"ZZPSXLAT", ctx)?;
    Ok(())
}
