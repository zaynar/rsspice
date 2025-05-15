//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const RNAME: &[u8] = b"T_TSTBOX";

//
// Utility routine T_TSTBOX: create latitude bounding
// box for testing. The bounds of segment may span the
// X-Y plane.
//
pub fn T_TSTBOX(
    BOUNDS: &[f64],
    CENTER: &mut [f64],
    LR: &mut f64,
    LT: &mut f64,
    LZ: &mut f64,
    RADIUS: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let mut CENTER = DummyArrayMut::new(CENTER, 1..=3);
    let mut CTR = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut DIAG = StackArray::<f64, 3>::new(1..=3);
    let mut EXTENT = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut H: f64 = 0.0;
    let mut LOCBDS = StackArray2D::<f64, 6>::new(1..=2, 1..=3);
    let mut LON: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXRC: f64 = 0.0;
    let mut MAXZ: f64 = 0.0;
    let mut MINLAT: f64 = 0.0;
    let mut MINRC: f64 = 0.0;
    let mut MINZ: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut R1: f64 = 0.0;
    let mut R2: f64 = 0.0;

    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(RNAME, ctx)?;

    MINLAT = BOUNDS[[1, 2]];
    MAXLAT = BOUNDS[[2, 2]];

    if ((MINLAT >= 0.0) || (MAXLAT < 0.0)) {
        //
        // The volume element doesn't cross the X-Y plane.
        // We can delegate the job.
        //
        T_MKBOX(
            BOUNDS.as_slice(),
            CENTER.as_slice_mut(),
            LR,
            LT,
            LZ,
            RADIUS,
            ctx,
        )?;

        spicelib::CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // The volume element crosses the X-Y plane. Divide the
    // element into upper and lower parts; find bounding boxes
    // for each.
    //
    for I in 1..=2 {
        spicelib::MOVED(BOUNDS.as_slice(), 6, LOCBDS.as_slice_mut());

        LOCBDS[[I, 2]] = 0.0;

        let [arg2, arg3, arg4] = EXTENT
            .get_disjoint_mut([[1, I], [2, I], [3, I]])
            .expect("mutable array elements passed to function must have disjoint indexes");
        T_MKBOX(
            LOCBDS.as_slice(),
            CTR.subarray_mut([1, I]),
            arg2,
            arg3,
            arg4,
            &mut R,
            ctx,
        )?;
    }

    //
    // At this point the longitude of the center and the tangential
    // extent of the box are known. We'll need to recompute the
    // radius and Z component of the center and the radial and Z
    // extents of the box.
    //
    *LT = EXTENT[[2, 1]];

    //
    // Start with the Z values. Recall that the two boxes
    // are bounded by the X-Y plane, which simplifies our
    // calculations. The box at index 1 is the upper box.
    //
    MAXZ = EXTENT[[3, 1]];
    MINZ = -EXTENT[[3, 2]];
    *LZ = intrinsics::DMAX1(&[0.0, (MAXZ - MINZ)]);
    CENTER[3] = (MAXZ - (*LZ / 2 as f64));

    //
    // Both boxes have the maximum radial component, so we can
    // compute this component using the first box alone. The
    // minimum radial component must be derived using both boxes.
    //
    // Note the radial component of the box center is always
    // positive, and the longitude of the box center is always
    // the same as that of the midpoint of the centers of any
    // arc of constant latitude on the outer bounding sphere.
    //
    spicelib::RECCYL(CTR.subarray([1, 1]), &mut R1, &mut LON, &mut H, ctx);

    MAXRC = (R1 + (EXTENT[[1, 1]] / 2 as f64));

    spicelib::RECCYL(CTR.subarray([1, 2]), &mut R2, &mut LON, &mut H, ctx);

    MINRC = intrinsics::DMIN1(&[
        (R1 - (EXTENT[[1, 1]] / 2 as f64)),
        (R2 - (EXTENT[[1, 2]] / 2 as f64)),
    ]);

    *LR = intrinsics::DMAX1(&[0.0, (MAXRC - MINRC)]);

    H = CENTER[3];

    spicelib::CYLREC((MAXRC - (*LR / 2 as f64)), LON, H, CENTER.as_slice_mut());

    spicelib::VPACK(
        (*LT / 2 as f64),
        (*LR / 2 as f64),
        (*LZ / 2 as f64),
        DIAG.as_slice_mut(),
    );

    *RADIUS = spicelib::VNORM(DIAG.as_slice());

    spicelib::CHKOUT(RNAME, ctx)?;
    Ok(())
}
