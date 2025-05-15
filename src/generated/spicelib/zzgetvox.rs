//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZGETVOX ( Coordinates of voxel containing a point )
pub fn ZZGETVOX(
    VOXSIZ: f64,
    VOXORI: &[f64],
    NVOX: &[i32],
    XYZ: &[f64],
    INBOX: &mut bool,
    VOXCOR: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VOXORI = DummyArray::new(VOXORI, 1..=3);
    let NVOX = DummyArray::new(NVOX, 1..=3);
    let XYZ = DummyArray::new(XYZ, 1..=3);
    let mut VOXCOR = DummyArrayMut::new(VOXCOR, 1..=3);
    let mut TERM: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables.
    //

    //
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    if (VOXSIZ <= 0.0) {
        CHKIN(b"ZZGETVOX", ctx)?;
        SETMSG(b"Voxel size was #; must be positive.", ctx);
        ERRDP(b"#", VOXSIZ, ctx);
        SIGERR(b"SPICE(NONPOSITIVEVALUE)", ctx)?;
        CHKOUT(b"ZZGETVOX", ctx)?;
        return Ok(());
    }

    //
    // Initialize 'point in box' flag and voxel coordinates. The
    // coordinates are assigned out-of-range values.
    //
    *INBOX = false;

    VOXCOR[1] = 0;
    VOXCOR[2] = 0;
    VOXCOR[3] = 0;

    //
    // Scale the point's coordinates to voxel grid space
    // and determine the indices of the voxel that contains it.
    //
    for I in 1..=3 {
        //
        // A Galilean transform. Calculate the voxel coordinate
        // corresponding to the body centered coordinate. This
        // operation performs the same task as TOGRID, but
        // including the operation here improves ZZGETVOX's
        // runtime performance.
        //
        TERM = ((XYZ[I] - VOXORI[I]) / VOXSIZ);

        //
        // Calculate the voxel index for each degree of freedom
        // corresponding to the voxel coordinate.
        //
        // If the point is outside of the grid, return now.
        //
        if ((TERM < 0.0) || (TERM > NVOX[I] as f64)) {
            return Ok(());
        }

        //
        // Assign a 1-based value to the Ith component of the voxel's
        // coordinates. The outer surface of the grid is considered part
        // of the grid.
        //
        // Note that TERM is non-negative at this point.
        //
        if ((TERM as i32) < NVOX[I]) {
            VOXCOR[I] = (1 + (TERM as i32));
        } else {
            //
            // TERM is NVOX(I), since the cases
            //
            //    TERM > NVOX(I)
            //    TERM < NVOX(I)
            //
            // have been ruled out.
            //
            VOXCOR[I] = NVOX[I];
        }
    }

    *INBOX = true;

    Ok(())
}
