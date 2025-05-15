//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const RNAME: &[u8] = b"T_MKPBOX";
const UBPL: i32 = 4;

struct SaveVars {
    Z: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut Z = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { Z }
    }
}

//
// Utility routine T_MKPBOX: create box using coordinates
// of the corners of a volume element. This technique
// is valid only for "small" elements, for which the
// bounding box is not tangent at interior points of
// the element's surface. Note that this routine cannot
// be used for elements that cross the X-Y plane.
//
pub fn T_MKPBOX(
    BOUNDS: &[f64],
    CORPAR: &[f64],
    CENTER: &mut [f64],
    LR: &mut f64,
    LT: &mut f64,
    LZ: &mut f64,
    RADIUS: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let CORPAR = DummyArray::new(CORPAR, 1..);
    let mut CENTER = DummyArrayMut::new(CENTER, 1..=3);
    let mut ALT: f64 = 0.0;
    let mut CORNER = StackArray4D::<f64, 24>::new(1..=3, 1..=2, 1..=2, 1..=2);
    let mut CPLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut DIAG = StackArray::<f64, 3>::new(1..=3);
    let mut DLON: f64 = 0.0;
    let mut F: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MAXRC: f64 = 0.0;
    let mut MAXZ: f64 = 0.0;
    let mut MIDARC = StackArray::<f64, 3>::new(1..=3);
    let mut MIDLON: f64 = 0.0;
    let mut MIDPT = StackArray::<f64, 3>::new(1..=3);
    let mut MINLAT: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut MINRC: f64 = 0.0;
    let mut MINZ: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut P = StackArray::<f64, 3>::new(1..=3);
    let mut R: f64 = 0.0;
    let mut RADVEC = StackArray::<f64, 3>::new(1..=3);
    let mut RE: f64 = 0.0;
    let mut I: i32 = 0;

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

    RE = CORPAR[1];
    F = CORPAR[2];

    MINLAT = BOUNDS[[1, 2]];

    MINLON = BOUNDS[[1, 1]];
    MAXLON = BOUNDS[[2, 1]];

    if (MAXLON <= MINLON) {
        MAXLON = (MAXLON + spicelib::TWOPI(ctx));
    }

    DLON = (MAXLON - MINLON);
    MIDLON = (MINLON + (DLON / 2 as f64));

    //
    // Compute Cartesian coordinates of the box corners.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 2;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            LON = BOUNDS[[I, 1]];

            for J in 1..=2 {
                LAT = BOUNDS[[J, 2]];

                for K in 1..=2 {
                    ALT = BOUNDS[[K, 3]];

                    spicelib::GEOREC(LON, LAT, ALT, RE, F, CORNER.subarray_mut([1, I, J, K]), ctx)?;
                }
            }

            I += m3__;
        }
    }

    //
    // Pick the latitude of the segment so that its length
    // is the transverse length of the box.
    //
    if (MINLAT >= 0.0) {
        //
        // Use the lower pair of corners on the surface of maximum
        // radius.
        //
        I = 1;
    } else {
        I = 2;
    }

    //
    // Find the midpoint of the latitude boundary of maximum
    // altitude that is closest to the X-Y plane. This is the
    // point with the greatest component in the radial direction.
    //
    spicelib::GEOREC(
        MIDLON,
        BOUNDS[[I, 2]],
        BOUNDS[[2, 3]],
        RE,
        F,
        MIDARC.as_slice_mut(),
        ctx,
    )?;

    //
    // Create a central plane for the box: this plane contains
    // the midpoint of each line segment connecting a pair of
    // corners at the same latitude and altitude, and is normal
    // to each such line segment.

    spicelib::VSUB(
        CORNER.subarray([1, 2, I, 2]),
        CORNER.subarray([1, 1, I, 2]),
        NORMAL.as_slice_mut(),
    );

    spicelib::VLCOM(
        0.5,
        CORNER.subarray([1, 2, 1, 2]),
        0.5,
        CORNER.subarray([1, 1, 1, 2]),
        MIDPT.as_slice_mut(),
    );

    spicelib::NVP2PL(
        NORMAL.as_slice(),
        MIDPT.as_slice(),
        CPLANE.as_slice_mut(),
        ctx,
    )?;

    //
    // Compute the radius of one of the corners at latitude index I.
    // We'll use this later.
    //
    spicelib::RECLAT(CORNER.subarray([1, 2, I, 2]), &mut R, &mut LON, &mut LAT);

    if (DLON <= spicelib::PI(ctx)) {
        //
        // The segment connecting the two corners selected above
        // has length equal to the tangential extent of the element.
        //
        *LT = spicelib::VNORM(NORMAL.as_slice());
    } else {
        //
        // The widest part of the element is at the longitudes 90 degrees
        // away from MIDLON. The width is just the width of the outer
        // circle formed by projecting the element orthogonally onto the
        // X-Y plane.
        //
        spicelib::VEQU(CORNER.subarray([1, 2, I, 2]), P.as_slice_mut());
        P[3] = 0.0;

        *LT = (2.0 * spicelib::VNORM(P.as_slice()));
    }

    //
    // For each corner on the volume face of minimum longitude,
    // compute the minimum and maximum Z values of the corners.
    //
    MINZ = spicelib::DPMAX();
    MAXZ = spicelib::DPMIN();

    for J in 1..=2 {
        for K in 1..=2 {
            MINZ = intrinsics::DMIN1(&[MINZ, CORNER[[3, 1, J, K]]]);
            MAXZ = intrinsics::DMAX1(&[MAXZ, CORNER[[3, 1, J, K]]]);
        }
    }

    //
    // Compute the Z extent of the element.
    //
    *LZ = intrinsics::DMAX1(&[(MAXZ - MINZ), 0.0]);

    //
    // Let RADVEC be a unit vector in the radial direction.
    //
    spicelib::UCRSS(NORMAL.as_slice(), save.Z.as_slice(), RADVEC.as_slice_mut());

    //
    // Find the component of MIDARC in the RADVEC (radial) direction.
    //
    MAXRC = spicelib::VDOT(MIDARC.as_slice(), RADVEC.as_slice());

    if (DLON < spicelib::PI(ctx)) {
        //
        // For the corners on the intersection of the volume face of
        // minimum longitude and surface of minimum altitude, compute the
        // corners' components in the radial direction. Select the
        // minimum component.
        //
        MINRC = intrinsics::DMIN1(&[
            spicelib::VDOT(CORNER.subarray([1, 1, 1, 1]), RADVEC.as_slice()),
            spicelib::VDOT(CORNER.subarray([1, 1, 2, 1]), RADVEC.as_slice()),
        ]);
    } else {
        //
        // The element "wraps" around the Z axis; the points having
        // minimum tangential components lie on the surface of
        // maximum altitude.
        //
        MINRC = intrinsics::DMIN1(&[
            spicelib::VDOT(CORNER.subarray([1, 1, 1, 2]), RADVEC.as_slice()),
            spicelib::VDOT(CORNER.subarray([1, 1, 2, 2]), RADVEC.as_slice()),
        ]);
    }

    //
    // We now have the extent of the element in the radial direction.
    //
    *LR = intrinsics::DMAX1(&[(MAXRC - MINRC), 0.0]);

    //
    // MIDARC lies on the central, vertical plane of the box, at the end
    // of maximum tangential component. We can derive the coordinates of
    // the center of the box from MIDARC, MINZ, and the box extents.
    //
    spicelib::CYLREC(
        (MAXRC - (*LR / 2 as f64)),
        MIDLON,
        (MINZ + (*LZ / 2 as f64)),
        CENTER.as_slice_mut(),
    );

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
