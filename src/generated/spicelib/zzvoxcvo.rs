//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure  ZZVOXCVO ( Voxel to coarse voxel offset )
pub fn ZZVOXCVO(
    VIXYZ: &[i32],
    NVOX: &[i32],
    CGRSCL: i32,
    CGXYZ: &mut [i32],
    CGOFF: &mut [i32],
    CGOF1D: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VIXYZ = DummyArray::new(VIXYZ, 1..=3);
    let NVOX = DummyArray::new(NVOX, 1..=3);
    let mut CGXYZ = DummyArrayMut::new(CGXYZ, 1..=3);
    let mut CGOFF = DummyArrayMut::new(CGOFF, 1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    for I in 1..=3 {
        if (NVOX[I] < 1) {
            CHKIN(b"ZZVOXCVO", ctx)?;

            SETMSG(
                b"Voxel grid dimensions must be positive but were # # #.",
                ctx,
            );
            ERRINT(b"#", NVOX[1], ctx);
            ERRINT(b"#", NVOX[2], ctx);
            ERRINT(b"#", NVOX[3], ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;

            CHKOUT(b"ZZVOXCVO", ctx)?;
            return Ok(());
        }
    }

    for I in 1..=3 {
        if ((VIXYZ[I] < 1) || (VIXYZ[I] > NVOX[I])) {
            CHKIN(b"ZZVOXCVO", ctx)?;

            SETMSG(b"Voxel grid coordinates must be inside grid having dimensions # x # x # but were # # #.", ctx);
            ERRINT(b"#", NVOX[1], ctx);
            ERRINT(b"#", NVOX[2], ctx);
            ERRINT(b"#", NVOX[3], ctx);
            ERRINT(b"#", VIXYZ[1], ctx);
            ERRINT(b"#", VIXYZ[2], ctx);
            ERRINT(b"#", VIXYZ[3], ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;

            CHKOUT(b"ZZVOXCVO", ctx)?;
            return Ok(());
        }
    }

    if (CGRSCL < 1) {
        CHKIN(b"ZZVOXCVO", ctx)?;

        SETMSG(b"Coarse voxel grid scale must be positive but was #.", ctx);
        ERRINT(b"#", NVOX[1], ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;

        CHKOUT(b"ZZVOXCVO", ctx)?;
        return Ok(());
    }

    for I in 1..=3 {
        //
        // Set the Ith coarse grid coordinate. Recall these coordinates
        // are 1-based.
        //
        CGXYZ[I] = (((VIXYZ[I] - 1) / CGRSCL) + 1);

        //
        // Set the Ith coarse grid coordinate offset. These offsets
        // are 1-based as well.
        //
        CGOFF[I] = (VIXYZ[I] - (CGRSCL * (CGXYZ[I] - 1)));
    }

    //
    // Convert the coarse grid-relative offset to a relative
    // ID. The ID is a one-dimensional offset.
    //
    *CGOF1D = (((((CGOFF[3] - 1) * CGRSCL) * CGRSCL) + ((CGOFF[2] - 1) * CGRSCL)) + CGOFF[1]);

    Ok(())
}
