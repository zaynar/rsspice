//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZTOGRID ( Model coordinates to voxel grid coordinates )
pub fn ZZTOGRID(
    MODXYZ: &[f64],
    VOXORI: &[f64],
    VOXSIZ: f64,
    GRDXYZ: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let MODXYZ = DummyArray::new(MODXYZ, 1..=3);
    let VOXORI = DummyArray::new(VOXORI, 1..=3);
    let mut GRDXYZ = DummyArrayMut::new(GRDXYZ, 1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    if (VOXSIZ <= 0.0) {
        CHKIN(b"ZZTOGRID", ctx)?;
        SETMSG(b"Voxel size was #; must be positive.", ctx);
        ERRDP(b"#", VOXSIZ, ctx);
        SIGERR(b"SPICE(NONPOSITIVEVALUE)", ctx)?;
        CHKOUT(b"ZZTOGRID", ctx)?;
        return Ok(());
    }

    //
    // Convert model coordinates to voxel grid coordinates
    // via a Galilean transform.
    //
    for I in 1..=3 {
        GRDXYZ[I] = ((MODXYZ[I] - VOXORI[I]) / VOXSIZ);
    }

    Ok(())
}
