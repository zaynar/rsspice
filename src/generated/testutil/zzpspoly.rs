//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure ZZPSPOLY ( Plate set, create polygon )
pub fn ZZPSPOLY(
    N: i32,
    VRTCES: &[f64],
    VOUT: &mut [f64],
    POUT: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VRTCES = DummyArray2D::new(VRTCES, 1..=2, 1..);
    let mut VOUT = DummyArrayMut::new(VOUT, LBCELL..);
    let mut POUT = DummyArrayMut::new(POUT, LBCELL..);
    let mut PERP = StackArray::<f64, 2>::new(1..=2);
    let mut CPOUT: i32 = 0;
    let mut CVOUT: i32 = 0;
    let mut J: i32 = 0;
    let mut NP: i32 = 0;
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

    spicelib::CHKIN(b"ZZPSPOLY", ctx)?;

    //
    // Check the space in the output cells.
    //
    NV = (N + 1);
    CVOUT = (3 * NV);

    NP = N;
    CPOUT = (3 * NP);

    if (spicelib::SIZED(VOUT.as_slice(), ctx)? < CVOUT) {
        spicelib::SETMSG(
            b"Output vertex array size is #; required space is # elements.",
            ctx,
        );
        spicelib::ERRINT(b"#", spicelib::SIZED(VOUT.as_slice(), ctx)?, ctx);
        spicelib::ERRINT(b"#", CVOUT, ctx);
        spicelib::SIGERR(b"SPICE(VERTARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZPSPOLY", ctx)?;
        return Ok(());
    }

    if (spicelib::SIZEI(POUT.as_slice(), ctx)? < CPOUT) {
        spicelib::SETMSG(
            b"Output plate array size is #; required space is # elements.",
            ctx,
        );
        spicelib::ERRINT(b"#", spicelib::SIZEI(POUT.as_slice(), ctx)?, ctx);
        spicelib::ERRINT(b"#", CPOUT, ctx);
        spicelib::SIGERR(b"SPICE(PLTARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZPSPOLY", ctx)?;
        return Ok(());
    }

    //
    // Check the order of the input vertices.
    //
    for I in 1..=(N - 1) {
        PERP[1] = -VRTCES[[2, I]];
        PERP[2] = VRTCES[[1, I]];

        if (spicelib::VDOTG(VRTCES.subarray([1, (I + 1)]), PERP.as_slice(), 2) <= 0.0) {
            spicelib::SETMSG(b"Input vertices are not in strictly increasing order: rotation angle from vertex # to vertex # about the +Z axis is not positive.", ctx);
            spicelib::ERRINT(b"#", I, ctx);
            spicelib::ERRINT(b"#", (I + 1), ctx);
            spicelib::SIGERR(b"SPICE(BADVERTEXORDER)", ctx)?;
            spicelib::CHKOUT(b"ZZPSPOLY", ctx)?;
            return Ok(());
        }
    }

    //
    // Create 3-D vertices.
    //
    J = 1;

    for I in 1..=N {
        VOUT[J] = VRTCES[[1, I]];
        VOUT[(J + 1)] = VRTCES[[2, I]];
        VOUT[(J + 2)] = 0.0;
        J = (J + 3);
    }

    //
    // The last vertex is the origin.
    //
    spicelib::CLEARD(3, VOUT.subarray_mut((CVOUT - 2)));

    spicelib::SCARDD(CVOUT, VOUT.as_slice_mut(), ctx)?;

    //
    // Create plates.
    //
    J = 1;

    for I in 1..=(N - 1) {
        POUT[J] = NV;
        POUT[(J + 1)] = I;
        POUT[(J + 2)] = (I + 1);
        J = (J + 3);
    }

    //
    // Fill in the last plate.
    //
    POUT[J] = NV;
    POUT[(J + 1)] = N;
    POUT[(J + 2)] = 1;

    spicelib::SCARDI(CPOUT, POUT.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"ZZPSPOLY", ctx)?;
    Ok(())
}
