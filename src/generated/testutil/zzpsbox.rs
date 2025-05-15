//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure ZZPSBOX ( Plate set, create box )
pub fn ZZPSBOX(
    A: f64,
    B: f64,
    C: f64,
    VOUT: &mut [f64],
    POUT: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut VOUT = DummyArrayMut::new(VOUT, LBCELL..);
    let mut POUT = DummyArrayMut::new(POUT, LBCELL..);
    let mut CPOUT: i32 = 0;
    let mut CVOUT: i32 = 0;
    let mut J: i32 = 0;
    let mut NP: i32 = 0;
    let mut NV: i32 = 0;
    let mut PLATE = StackArray2D::<i32, 36>::new(1..=3, 1..=12);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZPSBOX", ctx)?;

    //
    // Check the space in the output cells.
    //
    NV = 8;
    CVOUT = (3 * NV);

    NP = 12;
    CPOUT = (3 * NP);

    if (spicelib::SIZED(VOUT.as_slice(), ctx)? < CVOUT) {
        spicelib::SETMSG(b"Output vertex array is too small to contain box vertex array. Output vertex array size is #; required space is # elements.", ctx);
        spicelib::ERRINT(b"#", spicelib::SIZED(VOUT.as_slice(), ctx)?, ctx);
        spicelib::ERRINT(b"#", CVOUT, ctx);
        spicelib::SIGERR(b"SPICE(VERTARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZPSBOX", ctx)?;
        return Ok(());
    }

    if (spicelib::SIZEI(POUT.as_slice(), ctx)? < CPOUT) {
        spicelib::SETMSG(b"Output plate array is too small to contain box plate array. Output plate array size is #; required space is # elements.", ctx);
        spicelib::ERRINT(b"#", spicelib::SIZEI(POUT.as_slice(), ctx)?, ctx);
        spicelib::ERRINT(b"#", CPOUT, ctx);
        spicelib::SIGERR(b"SPICE(PLTARRAYTOOSMALL)", ctx)?;
        spicelib::CHKOUT(b"ZZPSBOX", ctx)?;
        return Ok(());
    }

    //
    // Check the box dimensions.
    //
    if (((A <= 0.0) || (B <= 0.0)) || (C <= 0.0)) {
        spicelib::SETMSG(
            b"Box dimensions must be positive. Actual dimensions were A = #; B = #; C = #.",
            ctx,
        );
        spicelib::ERRDP(b"#", A, ctx);
        spicelib::ERRDP(b"#", B, ctx);
        spicelib::ERRDP(b"#", C, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDDGELENGTH)", ctx)?;
        spicelib::CHKOUT(b"ZZPSBOX", ctx)?;
        return Ok(());
    }

    //
    // Create the vertices.
    //
    J = 1;

    for IZ in 0..=1 {
        for IY in 0..=1 {
            for IX in 0..=1 {
                VOUT[(J + 2)] = (((IZ as f64) - 0.5) * C);
                VOUT[(J + 1)] = (((IY as f64) - 0.5) * B);
                VOUT[J] = (((IX as f64) - 0.5) * A);
                J = (J + 3);
            }
        }
    }

    spicelib::SCARDD(CVOUT, VOUT.as_slice_mut(), ctx)?;

    //
    // Create the plates.
    //

    //
    // +Z face:
    //
    PLATE[[1, 1]] = 5;
    PLATE[[2, 1]] = 6;
    PLATE[[3, 1]] = 7;

    PLATE[[1, 2]] = 6;
    PLATE[[2, 2]] = 8;
    PLATE[[3, 2]] = 7;

    //
    // -Z face:
    //
    PLATE[[1, 3]] = 1;
    PLATE[[2, 3]] = 3;
    PLATE[[3, 3]] = 2;

    PLATE[[1, 4]] = 2;
    PLATE[[2, 4]] = 3;
    PLATE[[3, 4]] = 4;

    //
    // +Y face:
    //
    PLATE[[1, 5]] = 4;
    PLATE[[2, 5]] = 3;
    PLATE[[3, 5]] = 8;

    PLATE[[1, 6]] = 3;
    PLATE[[2, 6]] = 7;
    PLATE[[3, 6]] = 8;

    //
    // -Y face:
    //
    PLATE[[1, 7]] = 1;
    PLATE[[2, 7]] = 2;
    PLATE[[3, 7]] = 5;

    PLATE[[1, 8]] = 2;
    PLATE[[2, 8]] = 6;
    PLATE[[3, 8]] = 5;

    //
    // +X face:
    //
    PLATE[[1, 9]] = 2;
    PLATE[[2, 9]] = 4;
    PLATE[[3, 9]] = 6;

    PLATE[[1, 10]] = 4;
    PLATE[[2, 10]] = 8;
    PLATE[[3, 10]] = 6;

    //
    // -X face:
    //
    PLATE[[1, 11]] = 3;
    PLATE[[2, 11]] = 1;
    PLATE[[3, 11]] = 7;

    PLATE[[1, 12]] = 1;
    PLATE[[2, 12]] = 5;
    PLATE[[3, 12]] = 7;

    spicelib::MOVEI(PLATE.as_slice(), CPOUT, POUT.subarray_mut(1));

    spicelib::SCARDI(CPOUT, POUT.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"ZZPSBOX", ctx)?;
    Ok(())
}
