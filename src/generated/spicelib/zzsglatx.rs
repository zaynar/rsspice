//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const UBPL: i32 = 4;

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

//$Procedure ZZSGLATX ( Line segment latitude extent )
pub fn ZZSGLATX(
    P1: &[f64],
    P2: &[f64],
    MINLAT: &mut f64,
    MINP: &mut [f64],
    MAXLAT: &mut f64,
    MAXP: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let P1 = DummyArray::new(P1, 1..=3);
    let P2 = DummyArray::new(P2, 1..=3);
    let mut MINP = DummyArrayMut::new(MINP, 1..=3);
    let mut MAXP = DummyArrayMut::new(MAXP, 1..=3);
    let mut CREASE = StackArray::<f64, 3>::new(1..=3);
    let mut DIR = StackArray::<f64, 3>::new(1..=3);
    let mut DP1: f64 = 0.0;
    let mut DP2: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LAT1: f64 = 0.0;
    let mut LAT2: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PLANE2 = StackArray::<f64, 4>::new(1..=UBPL);
    let mut R: f64 = 0.0;
    let mut T = StackArray::<f64, 3>::new(1..=3);
    let mut NXPTS: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSGLATX", ctx)?;

    //
    // Start by computing latitude at the segment's endpoints.
    //
    RECLAT(P1.as_slice(), &mut R, &mut LON, &mut LAT1);
    RECLAT(P2.as_slice(), &mut R, &mut LON, &mut LAT2);

    //
    // Initialize the outputs using latitudes of the endpoints.
    // If there are interior extrema, we'll update these outputs
    // as needed.
    //
    if (LAT1 <= LAT2) {
        *MINLAT = LAT1;
        *MAXLAT = LAT2;

        VEQU(P1.as_slice(), MINP.as_slice_mut());
        VEQU(P2.as_slice(), MAXP.as_slice_mut());
    } else {
        *MINLAT = LAT2;
        *MAXLAT = LAT1;

        VEQU(P2.as_slice(), MINP.as_slice_mut());
        VEQU(P1.as_slice(), MAXP.as_slice_mut());
    }

    //
    // We want to work with the plane containing the origin, P1, and P2.
    // We'll call this plane PLANE1. First see whether P1 and P2 are
    // linearly independent.
    //
    VCRSS(P1.as_slice(), P2.as_slice(), NORMAL.as_slice_mut());

    if VZERO(NORMAL.as_slice()) {
        //
        // We have a special case: P1 and P2 define a line passing
        // through the origin. The latitude extrema lie on the
        // segment endpoints, and possibly at every point on the
        // segment. We've already computed the outputs.
        //
        CHKOUT(b"ZZSGLATX", ctx)?;
        return Ok(());
    }

    //
    // At this point we know that NORMAL is non-zero. Convert it
    // to a unit vector.
    //
    VHATIP(NORMAL.as_slice_mut());

    //
    // Let ALPHA be the non-negative angle between PLANE1 and the X-Y
    // plane. Then ALPHA and -ALPHA are, respectively, the maximum and
    // minimum possible latitudes attained on the input segment.
    // However, these values are not necessarily attained on the
    // segment; we'll need to perform further analysis to find out. We
    // don't need to compute ALPHA, but we'll refer to it in the
    // discussion below.
    //
    // The next step is to find the normal vector to the plane defined
    // by Z and NORMAL. We'll call this plane PLANE2. This plane might
    // not exist if NORMAL and Z are linearly dependent. If PLANE2
    // does exist, the X-Y plane and PLANE1 intersect in a "crease"
    // that is normal to PLANE2.
    //
    VCRSS(save.Z.as_slice(), NORMAL.as_slice(), CREASE.as_slice_mut());

    if VZERO(CREASE.as_slice()) {
        //
        // Z and NORMAL are linearly dependent; PLANE1 coincides (up to
        // round-off error) with the X-Y plane. We've already computed
        // the outputs.
        //
        CHKOUT(b"ZZSGLATX", ctx)?;
        return Ok(());
    }

    //
    // At this point we know CREASE is non-zero. Convert
    // it to a unit vector.
    //
    VHATIP(CREASE.as_slice_mut());

    //
    // By construction, CREASE is orthogonal to NORMAL. PLANE2
    // cuts PLANE1 in a line L passing through the origin. If
    // the line segment has an interior latitude extremum,
    // the point T where that extremum is attained lies on L.
    // The segment is tangent at T to a nappe of a cone, centered on
    // the Z-axis and having its apex at the origin, for which
    // the half-angle is (pi/2)-ALPHA. The point T lies in PLANE2
    // since L is contained in PLANE2.
    //
    // If a single tangent point T exists in the interior of the
    // segment, then the endpoints must be on opposite sides of PLANE2.
    // See whether this is the case.
    //
    DP1 = VDOT(P1.as_slice(), CREASE.as_slice());
    DP2 = VDOT(P2.as_slice(), CREASE.as_slice());

    if OPSGND(DP1, DP2) {
        //
        // The segment crosses PLANE2 at an interior point; this
        // point is where the extremum occurs. Solve for the
        // intersection.
        //
        // CREASE is guaranteed to be a unit vector. A zero input
        // vector is the only cause for which NVC2PL will signal
        // an error. Therefore we don't check FAILED after the
        // following call.
        //
        NVC2PL(CREASE.as_slice(), 0.0, PLANE2.as_slice_mut(), ctx)?;

        VSUB(P2.as_slice(), P1.as_slice(), DIR.as_slice_mut());

        INRYPL(
            P1.as_slice(),
            DIR.as_slice(),
            PLANE2.as_slice(),
            &mut NXPTS,
            T.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSGLATX", ctx)?;
            return Ok(());
        }

        if (NXPTS == 1) {
            //
            // This is the normal case: we have one intersection of the
            // segment with PLANE2. Update whichever of the extrema is
            // superseded by the interior value.
            //
            // Note that this case can occur when NORMAL is orthogonal to
            // Z, making ALPHA equal to pi/2. The nappes are degenerate in
            // this case, consisting of the positive and negative Z-axes.
            // This degenerate case occurs when the segment intersects the
            // Z-axis in a point other than the origin, and the endpoints
            // are linearly independent.
            //
            // This is not a special case computationally.
            //
            RECLAT(T.as_slice(), &mut R, &mut LON, &mut LAT);

            if (LAT > *MAXLAT) {
                *MAXLAT = LAT;
                VEQU(T.as_slice(), MAXP.as_slice_mut());
            } else if (LAT < *MINLAT) {
                *MINLAT = LAT;
                VEQU(T.as_slice(), MINP.as_slice_mut());
            }
            //
            // There can be only one local extremum, so we're done.
            //
        }
        //
        // If NXPTS is not 1, then even though the endpoints are on
        // opposite sides of PLANE2, either the segment was found to lie
        // in PLANE2 or no intersection was found. This situation must be
        // due to finite precision arithmetic error. We'll make do with
        // the extrema already found.
    }

    //
    // We reach this point if we found a local extremum or if any of the
    // following are true:
    //
    //    1)  The segment misses PLANE2 altogether, in which case
    //        there's no tangency point.
    //
    //    2)  One endpoint lies on PLANE2 and one endpoint does not.
    //
    //    3)  Both endpoints lie in PLANE2. Then both endpoints lie
    //        in L, so we should have found them to be linearly
    //        dependent. This situation must be due to finite precision
    //        arithmetic error.
    //
    // In all of the numbered cases the extrema occur at the endpoints.
    // and have been found already. In all cases, the outputs are set.
    //
    CHKOUT(b"ZZSGLATX", ctx)?;
    Ok(())
}
