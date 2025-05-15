//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LOWER: i32 = 1;
const UPPER: i32 = 2;

//$Procedure ZZRECBOX (Bounding box for rectangular volume element)
pub fn ZZRECBOX(
    BOUNDS: &[f64],
    CENTER: &mut [f64],
    LX: &mut f64,
    LY: &mut f64,
    LZ: &mut f64,
    RADIUS: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let mut CENTER = DummyArrayMut::new(CENTER, 1..=3);
    let mut DIAG = StackArray::<f64, 3>::new(1..=3);
    let mut L = StackArray::<f64, 3>::new(1..=3);
    let mut MAXCOR = StackArray::<f64, 3>::new(1..=3);
    let mut MINCOR = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Element boundary indices:
    //

    //
    // Local variables
    //

    //
    // This routine uses discovery check-in. We check RETURN in order to
    // avoid performing math operations using invalid operands.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Get local copies of the bounds of the volume element.
    //
    for I in 1..=3 {
        MINCOR[I] = BOUNDS[[LOWER, I]];
        MAXCOR[I] = BOUNDS[[UPPER, I]];
        L[I] = (MAXCOR[I] - MINCOR[I]);

        if (L[I] <= 0.0) {
            CHKIN(b"ZZRECBOX", ctx)?;
            SETMSG(
                b"Coordinate # bounds were #:#; bounds must be strictly increasing.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", MINCOR[I], ctx);
            ERRDP(b"#", MAXCOR[I], ctx);
            SIGERR(b"SPICE(BOUNDSOUTOFORDER)", ctx)?;
            CHKOUT(b"ZZRECBOX", ctx)?;
            return Ok(());
        }
    }

    //
    // Set output box dimensions.
    //
    *LX = L[1];
    *LY = L[2];
    *LZ = L[3];

    //
    // Compute the coordinates of the center of the box.
    //
    for I in 1..=3 {
        CENTER[I] = (MINCOR[I] + (L[I] / 2 as f64));
    }

    //
    // The radius is the distance from the center of the box
    // to any corner.
    //
    VPACK(
        (*LX / 2 as f64),
        (*LY / 2 as f64),
        (*LZ / 2 as f64),
        DIAG.as_slice_mut(),
    );

    *RADIUS = VNORM(DIAG.as_slice());

    Ok(())
}
