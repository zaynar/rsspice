//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NONE: i32 = 0;
const LOWER: i32 = 1;
const UPPER: i32 = 2;

//$Procedure ZZRYTREC ( DSK, ray touches rectangular element )
pub fn ZZRYTREC(
    VERTEX: &[f64],
    RAYDIR: &[f64],
    BOUNDS: &[f64],
    MARGIN: f64,
    NXPTS: &mut i32,
    XPT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut BOXORI = StackArray::<f64, 3>::new(1..=3);
    let mut EXTENT = StackArray::<f64, 3>::new(1..=3);
    let mut DELTA = StackArray::<f64, 3>::new(1..=3);
    let mut L = StackArray::<f64, 3>::new(1..=3);
    let mut FOUND: bool = false;
    let mut INSIDE: bool = false;

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

    if RETURN(ctx) {
        return Ok(());
    }

    //
    // Compute the original volume edge lengths from the coordinate
    // bounds.
    //
    for I in 1..=3 {
        L[I] = (BOUNDS[[UPPER, I]] - BOUNDS[[LOWER, I]]);

        if (L[I] <= 0.0) {
            CHKIN(b"ZZRYTREC", ctx)?;
            SETMSG(
                b"Coordinate # bounds were #:#; bounds must be strictly increasing.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            ERRDP(b"#", BOUNDS[[LOWER, I]], ctx);
            ERRDP(b"#", BOUNDS[[UPPER, I]], ctx);
            SIGERR(b"SPICE(BADCOORDBOUNDS)", ctx)?;
            CHKOUT(b"ZZRYTREC", ctx)?;
            return Ok(());
        }
    }

    //
    // Determine whether the vertex is inside the element.
    // Use double the margin for this test, since we don't
    // want to have false negative tests for rays having
    // vertices lying on the expanded element boundary.
    //
    *NXPTS = 0;

    ZZINREC(
        VERTEX.as_slice(),
        BOUNDS.as_slice(),
        ((2 as f64) * MARGIN),
        NONE,
        &mut INSIDE,
        ctx,
    )?;

    if INSIDE {
        //
        // We know the answer.
        //
        *NXPTS = 1;

        VEQU(VERTEX.as_slice(), XPT.as_slice_mut());

        return Ok(());
    }

    //
    // Expand the box using the specified margin.
    //
    for I in 1..=3 {
        DELTA[I] = (MARGIN * f64::abs(L[I]));

        BOXORI[I] = (BOUNDS[[LOWER, I]] - DELTA[I]);

        EXTENT[I] = (L[I] + ((2 as f64) * DELTA[I]));
    }

    //
    // Find the ray-surface intercept on the expanded element,
    // if the intercept exists.
    //
    ZZRAYBOX(
        VERTEX.as_slice(),
        RAYDIR.as_slice(),
        BOXORI.as_slice(),
        EXTENT.as_slice(),
        XPT.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    if FOUND {
        *NXPTS = 1;
    }

    Ok(())
}
