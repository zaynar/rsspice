//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
const UBEL: i32 = 9;
const UBPL: i32 = 4;
const MAXFOV: i32 = 10000;

struct SaveVars {
    ORIGIN: StackArray<f64, 3>,
    WORK: ActualArray2D<f64>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut WORK = ActualArray2D::<f64>::new(1..=3, 1..=MAXFOV);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            ORIGIN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ORIGIN, WORK }
    }
}

//$Procedure      ZZELVUPY ( Is ellipse in polygonal field of view? )
pub fn ZZELVUPY(
    ELLIPS: &[f64],
    VERTEX: &[f64],
    AXIS: &[f64],
    N: i32,
    BOUNDS: &[f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let ELLIPS = DummyArray::new(ELLIPS, 1..=UBEL);
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let AXIS = DummyArray::new(AXIS, 1..=3);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=3, 1..=N);
    let mut ERRMSG = [b' '; LMSGLN as usize];
    let mut A: f64 = 0.0;
    let mut APEX = StackArray::<f64, 3>::new(1..=3);
    let mut ASEP: f64 = 0.0;
    let mut B: f64 = 0.0;
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut CONSEP: f64 = 0.0;
    let mut CP = StackArray::<f64, 3>::new(1..=3);
    let mut CTRVEC = StackArray::<f64, 3>::new(1..=3);
    let mut D: f64 = 0.0;
    let mut EASIZE: f64 = 0.0;
    let mut EBSCTR = StackArray::<f64, 3>::new(1..=3);
    let mut EDGE1 = StackArray::<f64, 3>::new(1..=3);
    let mut EDGE2 = StackArray::<f64, 3>::new(1..=3);
    let mut ELLSCL = StackArray::<f64, 9>::new(1..=UBEL);
    let mut EPLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut FOVPLN = StackArray::<f64, 4>::new(1..=UBPL);
    let mut GV1 = StackArray::<f64, 3>::new(1..=3);
    let mut GV2 = StackArray::<f64, 3>::new(1..=3);
    let mut HAFEDG: f64 = 0.0;
    let mut HAFSEC: f64 = 0.0;
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut PASIZE: f64 = 0.0;
    let mut PLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut SCALE: f64 = 0.0;
    let mut SMAJOR = StackArray::<f64, 3>::new(1..=3);
    let mut SMINOR = StackArray::<f64, 3>::new(1..=3);
    let mut VBSCTR = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut VXPT1 = StackArray::<f64, 3>::new(1..=3);
    let mut VXPT2 = StackArray::<f64, 3>::new(1..=3);
    let mut X: f64 = 0.0;
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut XPT1 = StackArray::<f64, 3>::new(1..=3);
    let mut XPT2 = StackArray::<f64, 3>::new(1..=3);
    let mut Y: f64 = 0.0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut NXPTS: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
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

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZELVUPY", ctx)?;

    //
    // We start out by checking the inputs.
    //
    // The next step will be to look for an intersection of the ellipse
    // and pyramid.  There are three intersection cases:
    //
    //    1) The ellipse is completely contained in the pyramid.
    //
    //    2) The ellipse "contains" the field of view in the sense
    //       that the intersection of the pyramid and the plane of the
    //       ellipse is contained in the region bounded by the ellipse.
    //
    //    3) One or more sides of the pyramid intersect the ellipse.
    //
    // There is also a non-intersection case:  this is when cones
    // bounding the ellipse and pyramid and having their apexes in
    // common with that of the pyramid intersect only in that common
    // apex.  Before test (1), we perform this non-intersection test,
    // since it can be done quickly.
    //
    // No intersection has been found so far.  Set the default value
    // of the FOUND flag here so it won't have to be set in every error
    // checking block below.
    //
    *FOUND = false;

    //
    // Validate the ellipse.  First find the center and the semi-axes
    // of the ellipse.
    //
    EL2CGV(
        ELLIPS.as_slice(),
        CENTER.as_slice_mut(),
        GV1.as_slice_mut(),
        GV2.as_slice_mut(),
    );
    SAELGV(
        GV1.as_slice(),
        GV2.as_slice(),
        SMAJOR.as_slice_mut(),
        SMINOR.as_slice_mut(),
        ctx,
    )?;

    //
    // Check the semi-axis lengths.
    //
    // If the semi-major axis is the zero vector, we'd expect
    // the semi-minor axis to be the zero vector as well.  But
    // round-off error could conceivably violate this assumption.
    //
    if (VZERO(SMAJOR.as_slice()) || VZERO(SMINOR.as_slice())) {
        SETMSG(b"Input ellipse has semi-major axis length # and semi-minor axis length #.  Both vectors are required to be non-zero.", ctx);
        ERRDP(b"#", VNORM(SMAJOR.as_slice()), ctx);
        ERRDP(b"#", VNORM(SMINOR.as_slice()), ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZELVUPY", ctx)?;
        return Ok(());
    }

    //
    // Scale the vectors defining the ellipse and the vertex of the
    // pyramid so that the largest of these vectors has unit length.
    //
    SCALE = (1.0
        / intrinsics::DMAX1(&[
            VNORM(CENTER.as_slice()),
            VNORM(SMAJOR.as_slice()),
            VNORM(VERTEX.as_slice()),
        ]));

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            CENTER[I] = (SCALE * CENTER[I]);
            SMAJOR[I] = (SCALE * SMAJOR[I]);
            SMINOR[I] = (SCALE * SMINOR[I]);
            APEX[I] = (SCALE * VERTEX[I]);

            I += m3__;
        }
    }

    //
    // Create a scaled ellipse.  We'll perform the FOV side-ellipse
    // intersection computations using this ellipse.
    //
    CGV2EL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        ELLSCL.as_slice_mut(),
        ctx,
    )?;

    //
    // After scaling, make sure the semi-axes have sufficient length to
    // prevent numerical problems.  Let A and B be the scaled semi-axis
    // lengths of the ellipse.
    //
    A = VNORM(SMAJOR.as_slice());
    B = VNORM(SMINOR.as_slice());

    if (B == 0.0) {
        SETMSG(b"Scaled ellipse\'s semi-minor axis length = 0.", ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"ZZELVUPY", ctx)?;
        return Ok(());
    }

    //
    // Validate the input pyramid.
    //
    // The axis must not be the zero vector.
    //
    if VZERO(AXIS.as_slice()) {
        SETMSG(b"The pyramid\'s axis the zero vector.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZELVUPY", ctx)?;
        return Ok(());
    }

    //
    // There must be at least three boundary vectors.
    //
    if (N < 3) {
        SETMSG(
            b"The number of boundary vectors was #; this number must be at least 3.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZELVUPY", ctx)?;
        return Ok(());
    }

    //
    // There must be no more than MAXFOV boundary vectors.
    //
    if (N > MAXFOV) {
        SETMSG(
            b"The number of boundary vectors was #; this number must not exceed #.",
            ctx,
        );
        ERRINT(b"#", N, ctx);
        ERRINT(b"#", MAXFOV, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZELVUPY", ctx)?;
        return Ok(());
    }

    //
    // We must initialize certain variables before continuing with
    // the checks.
    //
    // Let CTRVEC be the vector from the apex to the center of the
    // ellipse.  This vector will be used in several places later;
    // it's convenient to compute it here.
    //
    VSUB(CENTER.as_slice(), APEX.as_slice(), CTRVEC.as_slice_mut());

    //
    // Compute PASIZE:  an upper bound on the angular radius of a
    // circular cone whose axis is the input central axis.  While
    // we're at it, check the angular separation of the boundary
    // vectors from the central axis and from each other.
    //
    PASIZE = 0.0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Each boundary vector must have angular separation from the
            // axis of less than pi/2 radians.  Keep track of the maximum
            // angular separation PASIZE as we go.  We'll use this variable
            // later in a non-intersection test.
            //
            ASEP = VSEP(AXIS.as_slice(), BOUNDS.subarray([1, I]), ctx);

            if (ASEP >= (PI(ctx) / 2 as f64)) {
                SETMSG(b"The angular separation of boundary vector # from the axis is #. This number must less than pi/2.", ctx);
                ERRINT(b"#", I, ctx);
                ERRDP(b"#", ASEP, ctx);
                SIGERR(b"SPICE(INVALIDFOV)", ctx)?;
                CHKOUT(b"ZZELVUPY", ctx)?;
                return Ok(());
            }

            PASIZE = intrinsics::DMAX1(&[PASIZE, ASEP]);

            //
            // Each boundary vector must have non-zero angular separation
            // from its neighbors.
            //
            if (I < N) {
                J = (I + 1);
            } else {
                J = 1;
            }

            UCRSS(
                BOUNDS.subarray([1, I]),
                BOUNDS.subarray([1, J]),
                CP.as_slice_mut(),
            );

            if VZERO(CP.as_slice()) {
                //
                // The cross product may be zero because one of the
                // boundary vectors is zero.  Check this first.
                //
                if (VZERO(BOUNDS.subarray([1, J])) || VZERO(BOUNDS.subarray([1, I]))) {
                    fstr::assign(&mut ERRMSG, b"The # boundary vector is the zero vector.");

                    if VZERO(BOUNDS.subarray([1, I])) {
                        J = I;
                    }

                    REPMOT(&ERRMSG.clone(), b"#", J, b"L", &mut ERRMSG, ctx)?;
                    SETMSG(&ERRMSG, ctx);
                    SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
                } else {
                    SETMSG(b"The angular separation of boundary vector # from vector # is 0.This number must be positive.", ctx);
                    ERRINT(b"#", I, ctx);
                    ERRINT(b"#", J, ctx);
                    SIGERR(b"SPICE(INVALIDFOV)", ctx)?;
                }

                CHKOUT(b"ZZELVUPY", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // That's it for the error checks.  We'll now answer the question
    // this routine is meant to answer:  does the ellipse or the region
    // it bounds intersect the pyramid?
    //
    // We'll start out with a simple check to rule out intersection
    // when the ellipse and pyramid are contained in disjoint right
    // circular cones with a common apex.
    //
    // Find the angular radius (that is, one-half of the angular extent)
    // of a bounding cone of the ellipse as seen from the apex.  The
    // cone circumscribes a sphere of radius A centered at the ellipse's
    // center, where A is the length of the semi-major axis.  Note that
    // the cone does not in general circumscribe the ellipse itself.
    //
    // The test can be performed only if the apex of the FOV is outside
    // of the sphere of radius A centered at the ellipse center.
    //
    D = VDIST(CENTER.as_slice(), APEX.as_slice());

    if (A < D) {
        EASIZE = f64::asin((A / D));

        //
        // The variable PASIZE already contains the angular radius of a
        // bounding cone of the pyramid as seen from the pyramid's apex.
        // The angular radius is the maximum of the angular separations
        // of each pyramid edge from the pyramid's axis. Check whether
        // the bounding cones of ellipse and pyramid are disjoint. Recall
        // CTRVEC is the vector from the apex to the center of the
        // ellipse.  If the angular separation of CTRVEC and AXIS exceeds
        // the sum of the angular radii of the ellipse's and pyramid's
        // bounding cones, there can be no intersection.
        //
        CONSEP = (VSEP(CTRVEC.as_slice(), AXIS.as_slice(), ctx) - (EASIZE + PASIZE));

        if (CONSEP > 0.0) {
            CHKOUT(b"ZZELVUPY", ctx)?;
            return Ok(());
        }
    }

    //
    // At this point, we have to take a more detailed look at the
    // possible intersection of ellipse and pyramid.  First check
    // whether the center of the ellipse is contained in the pyramid.
    // If the ellipse is completely contained in the pyramid, this
    // check will yield a positive result.
    //
    // The center of the ellipse is inside the pyramid if a plane
    // containing this point and normal to the axis vector
    // chops the pyramid in a polygon that has non-zero winding
    // number about the center.
    //
    // The center of the ellipse must lie in the correct half-space
    // for this test to be applicable.
    //
    if (VDOT(AXIS.as_slice(), CTRVEC.as_slice()) > 0.0) {
        //
        // Construct the plane and find the polygon.
        //
        NVP2PL(
            AXIS.as_slice(),
            CTRVEC.as_slice(),
            FOVPLN.as_slice_mut(),
            ctx,
        )?;

        //
        // Create the planar FOV boundary using the intersections
        // of the FOV boundary vectors with FOVPLN.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = N;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                INRYPL(
                    save.ORIGIN.as_slice(),
                    BOUNDS.subarray([1, I]),
                    FOVPLN.as_slice(),
                    &mut NXPTS,
                    save.WORK.subarray_mut([1, I]),
                    ctx,
                )?;

                //
                // We expect to have a single point of intersection for each
                // boundary vector.
                //
                if (NXPTS != 1) {
                    SETMSG(
                        b"NXPTS = # for boundary vector #/FOV plane intersection.",
                        ctx,
                    );
                    ERRINT(b"#", NXPTS, ctx);
                    ERRINT(b"#", I, ctx);
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(b"ZZELVUPY", ctx)?;
                    return Ok(());
                }

                I += m3__;
            }
        }

        //
        // Now WORK contains the polygon representing the intersection of
        // the pyramid with the plane FOVPLN. If the winding number of
        // the polygon about the ellipse center is non-zero, we conclude
        // the center is in the pyramid.
        //
        if (ZZWIND(
            FOVPLN.as_slice(),
            N,
            save.WORK.as_slice(),
            CTRVEC.as_slice(),
            ctx,
        )? != 0)
        {
            //
            // The center of the ellipse is inside the pyramid.  We're
            // done.
            //
            *FOUND = true;

            CHKOUT(b"ZZELVUPY", ctx)?;
            return Ok(());
        }
    }

    //
    // Check whether the ray defined by APEX and the first boundary
    // vector of the pyramid (the "boundary ray") intersects the plane
    // region bounded by the ellipse.  If the intersection of the
    // pyramid and the plane of the ellipse is completely contained in
    // the region bounded by the ellipse, this check will yield a
    // positive result.
    //
    // First find the intersection of the boundary ray and the plane
    // containing the ellipse; represent this plane using the SPICELIB
    // plane EPLANE.

    // We don't check FAILED() here because the spanning vectors
    // are orthogonal, and because PSV2PL (via a call to UCRSS)
    // does scaling to prevent underflow.
    //
    PSV2PL(
        CENTER.as_slice(),
        SMAJOR.as_slice(),
        SMINOR.as_slice(),
        EPLANE.as_slice_mut(),
        ctx,
    )?;

    INRYPL(
        APEX.as_slice(),
        BOUNDS.subarray([1, 1]),
        EPLANE.as_slice(),
        &mut NXPTS,
        XPT.as_slice_mut(),
        ctx,
    )?;

    //
    // The routine INRYPL can return the NXPTS values 1, 0, or INF---a
    // code indicating an infinite number of intersection points of ray
    // and plane.  If the value is 1, the boundary ray may intersect
    // the region bounded by the ellipse.
    //
    if (NXPTS == 1) {
        //
        // The boundary ray intersects the plane of the ellipse in a
        // single point. Decide whether this point is inside the ellipse.
        // To test for containment, find the "coordinates" of the
        // center-to-point vector relative to the two-dimensional basis
        // formed by the semi-axes of the ellipse.  Call this
        // center-to-point vector OFFSET.  Recall A and B are the
        // semi-axis lengths of the ellipse. Let X and Y be the
        // coordinates of OFFSET in the two-dimensional reference frame
        // whose basis consists of normalized versions of SMAJOR and
        // SMINOR.
        //
        // Note that we could have the special case in which the vertex
        // of the pyramid lies in the plane of the ellipse, in which case
        // the FOV "sees" the ellipse edge-on.  However, since NXPTS is
        // not INF, the boundary vector does not lie in the plane of the
        // ellipse.  So in this special case, APEX would be in the region
        // bounded by the ellipse.
        //
        VSUB(XPT.as_slice(), CENTER.as_slice(), OFFSET.as_slice_mut());

        X = (VDOT(OFFSET.as_slice(), SMAJOR.as_slice()) / A);
        Y = (VDOT(OFFSET.as_slice(), SMINOR.as_slice()) / B);

        if ((f64::powf((X / A), 2.0) + f64::powf((Y / B), 2.0)) <= 1.0) {
            //
            // The boundary-vector-plane intercept lies in the
            // topologically closed region bounded by the ellipse.
            //
            *FOUND = true;

            CHKOUT(b"ZZELVUPY", ctx)?;
            return Ok(());
        }
    }

    //
    // Check whether one of the pyramid's sides intersects the ellipse.
    // For each side, we first test whether the plane containing that
    // side intersects the ellipse.  If it does, the intersection is
    // a (possibly degenerate) line segment with endpoints on the
    // ellipse.  The triangle (or segment) defined by the pyramid's
    // apex and this segment (point) is then checked for intersection
    // with the currently considered side of the pyramid.
    //
    I = 1;

    while ((I <= N) && !*FOUND) {
        //
        // Create a SPICELIB plane containing the Ith side of the
        // pyramid.
        //
        if (I < N) {
            J = (I + 1);
        } else {
            J = 1;
        }

        //
        // Although PSV2PL can signal an error if the spanning
        // vectors are linearly dependent, it won't do so here
        // because we've already ensured the cross product of
        // these vectors is non-zero.
        //
        PSV2PL(
            APEX.as_slice(),
            BOUNDS.subarray([1, I]),
            BOUNDS.subarray([1, J]),
            PLANE.as_slice_mut(),
            ctx,
        )?;
        //
        // Find the intersection of the plane and the ellipse.
        //
        INELPL(
            ELLSCL.as_slice(),
            PLANE.as_slice(),
            &mut NXPTS,
            XPT1.as_slice_mut(),
            XPT2.as_slice_mut(),
            ctx,
        )?;

        //
        // If the ellipse-plane intersection is non-empty, test it to see
        // whether it has non-empty intersection with the current side of
        // the pyramid.
        //
        if (NXPTS > 0) {
            //
            // Let EDGE1 and EDGE2 be the unit length boundary vectors
            // forming the edges of the currently considered side of the
            // pyramid.
            //
            VHAT(BOUNDS.subarray([1, I]), EDGE1.as_slice_mut());
            VHAT(BOUNDS.subarray([1, J]), EDGE2.as_slice_mut());

            //
            // Let EBSCTR ("pyramid edge bisector") be a bisector of the
            // sector bounded by EDGE1 and EDGE2.
            //
            VLCOM(
                0.5,
                EDGE1.as_slice(),
                0.5,
                EDGE2.as_slice(),
                EBSCTR.as_slice_mut(),
            );

            //
            // Let HAFEDG be half of the angular measure of this sector.
            //
            HAFEDG = (VSEP(EDGE1.as_slice(), EDGE2.as_slice(), ctx) / 2.0);

            //
            // Let VXPT1 and VXPT2 be the unit vectors pointing from the
            // pyramid's apex to the points of intersection of the ellipse
            // and the plane containing the currently considered side of
            // the pyramid.
            //
            VSUB(XPT1.as_slice(), APEX.as_slice(), VTEMP.as_slice_mut());
            VHAT(VTEMP.as_slice(), VXPT1.as_slice_mut());

            VSUB(XPT2.as_slice(), APEX.as_slice(), VTEMP.as_slice_mut());
            VHAT(VTEMP.as_slice(), VXPT2.as_slice_mut());

            //
            // At this point we'll introduce a bit of terminology. We're
            // going to work with plane regions defined by pairs of
            // vectors with a common endpoint.  We'll abuse standard
            // terminology a bit and call the region bounded by such a
            // vector pair a "sector."  Strictly speaking, sectors refer
            // only to subsets of a disc.
            //
            // When it's convenient, we'll also identify "sectors" with
            // regions of the unit circle.  This will make it possible
            // to talk about intersections of sectors in terms of
            // intersections of the associated arcs on the unit circle.
            // By the "endpoints" of a sector we mean the endpoints
            // of the arc associated with the sector on the unit circle.
            //
            // Let VBSCTR ("VXPT bisector") be a bisector of the sector
            // bounded by VXPT1 and VXPT2.
            //
            VLCOM(
                0.5,
                VXPT1.as_slice(),
                0.5,
                VXPT2.as_slice(),
                VBSCTR.as_slice_mut(),
            );

            //
            // Let HAFSEC be half of the angular measure of the sector
            // bounded by VXPT1 and VXPT2.
            //
            HAFSEC = (VSEP(VXPT1.as_slice(), VXPT2.as_slice(), ctx) / 2.0);

            //
            // EDGE1, EDGE2, VXPT1, and VXPT2 are four co-planar vectors
            // emanating from APEX.  We want to find out whether the
            // sector bounded by EDGE1 and EDGE2 intersects the sector
            // bounded by VXPT1 and VXPT2.  If there's an intersection, at
            // least one endpoint of one sector is contained in the other
            // sector.
            //
            // Because of potential round-off problems when the sectors
            // are nearly coincident, we perform the precautionary check
            // (case 3) on the angle bisector of the sector defined by
            // VXPT1 and VXPT2.
            //
            // If the sector defined by VXPT1 and VXPT2 has no endpoint
            // contained in the other sector, it's possible that the
            // former sector contains the latter.  In that case the
            // angular bisector of the latter sector is contained in the
            // former (case 4).
            //
            // We test a vector's containment in a sector by comparing the
            // vector's angular separation from the sector's angle
            // bisector to one-half of the angular measure of the sector.
            //
            //    Case 1:  VXPT1 lies between EDGE1 and EDGE2.
            //    Case 2:  VXPT2 lies between EDGE1 and EDGE2.
            //    Case 3:  VBSCTR lies between EDGE1 and EDGE2.
            //    Case 4:  EBSCTR lies between VXPT1 and VXPT2.
            //
            if (VSEP(VXPT1.as_slice(), EBSCTR.as_slice(), ctx) <= HAFEDG) {
                *FOUND = true;
            } else if (VSEP(VXPT2.as_slice(), EBSCTR.as_slice(), ctx) <= HAFEDG) {
                *FOUND = true;
            } else if (VSEP(VBSCTR.as_slice(), EBSCTR.as_slice(), ctx) <= HAFEDG) {
                *FOUND = true;
            } else if (VSEP(EBSCTR.as_slice(), VBSCTR.as_slice(), ctx) <= HAFSEC) {
                *FOUND = true;
            }

            if *FOUND {
                //
                // We've found an intersection.  We're done.
                //
                CHKOUT(b"ZZELVUPY", ctx)?;
                return Ok(());
            }
        }

        //
        // If no intersection was found, look at the next side of the
        // pyramid.
        //
        I = (I + 1);
    }

    //
    // If we got this far, the ellipse is not in view.  FOUND has
    // already been set to .FALSE.
    //

    CHKOUT(b"ZZELVUPY", ctx)?;
    Ok(())
}
