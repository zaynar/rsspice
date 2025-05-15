//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure ZZPSBALL ( Plate set, create ball )
pub fn ZZPSBALL(
    R: f64,
    NLON: i32,
    NLAT: i32,
    VOUT: &mut [f64],
    POUT: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut VOUT = DummyArrayMut::new(VOUT, LBCELL..);
    let mut POUT = DummyArrayMut::new(POUT, LBCELL..);
    let mut MAXP: i32 = 0;
    let mut MAXV: i32 = 0;
    let mut NP: i32 = 0;
    let mut NV: i32 = 0;

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZPSBALL", ctx)?;

    MAXV = (spicelib::SIZED(VOUT.as_slice(), ctx)? / 3);
    MAXP = (spicelib::SIZEI(POUT.as_slice(), ctx)? / 3);

    //
    // From the header of ZZELLPLT:
    //
    //
    // MAXV       is the maximum number of vertices to return. MAXV must
    //            be at least
    //
    //               ( NLON * ( NLAT - 1 ) )  +  2
    //
    //            The array VERTS must have size at least 3*MAXV.
    //
    //
    // MAXP       is the maximum number of plates to return. MAXP must
    //            be at least
    //
    //               2 * NLON * ( NLAT - 1 )
    //
    //            The array PLATES must have size at least 3*MAXP.
    //

    support::ZZELLPLT(
        R,
        R,
        R,
        NLON,
        NLAT,
        MAXV,
        MAXP,
        &mut NV,
        VOUT.subarray_mut(1),
        &mut NP,
        POUT.subarray_mut(1),
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"ZZPSBALL", ctx)?;
        return Ok(());
    }

    spicelib::SCARDD((3 * NV), VOUT.as_slice_mut(), ctx)?;
    spicelib::SCARDI((3 * NP), POUT.as_slice_mut(), ctx)?;

    spicelib::CHKOUT(b"ZZPSBALL", ctx)?;
    Ok(())
}
