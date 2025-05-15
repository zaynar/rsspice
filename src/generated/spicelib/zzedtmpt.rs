//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ATOL: f64 = 0.00000000000001;
const CNVLIM: f64 = 0.000000000000001;
const MAXITR: i32 = 20;

//$Procedure ZZEDTMPT ( Ellipsoid terminator point in half-plane )
pub fn ZZEDTMPT(
    UMBRAL: bool,
    A: f64,
    B: f64,
    C: f64,
    R: f64,
    AXIS: &[f64],
    PLNVEC: &[f64],
    POINT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let AXIS = DummyArray::new(AXIS, 1..=3);
    let PLNVEC = DummyArray::new(PLNVEC, 1..=3);
    let mut POINT = DummyArrayMut::new(POINT, 1..=3);
    let mut ANGERR: f64 = 0.0;
    let mut ANGLE: f64 = 0.0;
    let mut CONST: f64 = 0.0;
    let mut D: f64 = 0.0;
    let mut H: f64 = 0.0;
    let mut HPLNML = StackArray::<f64, 3>::new(1..=3);
    let mut MAXR: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PROJ = StackArray::<f64, 3>::new(1..=3);
    let mut S: f64 = 0.0;
    let mut SGNNML = StackArray::<f64, 3>::new(1..=3);
    let mut SRCPNT = StackArray::<f64, 3>::new(1..=3);
    let mut TA: f64 = 0.0;
    let mut TARGPT = StackArray::<f64, 3>::new(1..=3);
    let mut TAXIS = StackArray::<f64, 3>::new(1..=3);
    let mut TB: f64 = 0.0;
    let mut TC: f64 = 0.0;
    let mut THETA: f64 = 0.0;
    let mut TMPVEC = StackArray::<f64, 3>::new(1..=3);
    let mut TPLNVC = StackArray::<f64, 3>::new(1..=3);
    let mut TRANS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut UTAXIS = StackArray::<f64, 3>::new(1..=3);
    let mut XA: f64 = 0.0;
    let mut XB: f64 = 0.0;
    let mut XC: f64 = 0.0;
    let mut NITR: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Tolerance used for arcsine arguments:
    //

    //
    // Angular error used to determine convergence:
    //

    //
    // Maximum number of iterations allowed for root finding:
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZEDTMPT", ctx)?;

    //
    // Check A, B, C, and R.
    //
    if (((A <= 0.0) || (B <= 0.0)) || (C <= 0.0)) {
        SETMSG(
            b"Target radii must be strictly positive but were #, #, #.",
            ctx,
        );
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", C, ctx);
        SIGERR(b"SPICE(INVALIDAXISLENGTH)", ctx)?;
        CHKOUT(b"ZZEDTMPT", ctx)?;
        return Ok(());
    }

    if (R <= 0.0) {
        SETMSG(b"Source radius must be strictly positive but was #.", ctx);
        ERRDP(b"#", R, ctx);
        SIGERR(b"SPICE(INVALIDRADIUS)", ctx)?;
        CHKOUT(b"ZZEDTMPT", ctx)?;
        return Ok(());
    }

    //
    // Check AXIS and PLNVEC.
    //
    if VZERO(AXIS.as_slice()) {
        SETMSG(b"AXIS must be a non-zero vector but is in fact zero.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZEDTMPT", ctx)?;
        return Ok(());
    }

    if ((R + intrinsics::DMAX1(&[A, B, C])) >= VNORM(AXIS.as_slice())) {
        SETMSG(b"Centers of source and target are too close together; distance is #. Radius of source is #; semi-axis lengths are #, #, #.", ctx);
        ERRDP(b"#", VNORM(AXIS.as_slice()), ctx);
        ERRDP(b"#", R, ctx);
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", C, ctx);
        SIGERR(b"SPICE(OBJECTSTOOCLOSE)", ctx)?;
        CHKOUT(b"ZZEDTMPT", ctx)?;
        return Ok(());
    }

    if VZERO(PLNVEC.as_slice()) {
        SETMSG(
            b"PLNVEC must be a non-zero vector but is in fact zero.",
            ctx,
        );
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZEDTMPT", ctx)?;
        return Ok(());
    }

    //
    // Transform the source, target, axis, and plane vector
    // so that the target becomes a unit sphere.
    //
    CLEARD(9, TRANS.as_slice_mut());

    TA = (1.0 / A);
    TB = (1.0 / B);
    TC = (1.0 / C);

    XA = (TA * R);
    XB = (TB * R);
    XC = (TC * R);

    TRANS[[1, 1]] = TA;
    TRANS[[2, 2]] = TB;
    TRANS[[3, 3]] = TC;

    //
    // TNEGAX is the negative of the transformed axis.
    // UTAXIS is the unit vector in the direction of TNEGAX.
    //
    MXV(TRANS.as_slice(), PLNVEC.as_slice(), TPLNVC.as_slice_mut());
    MXV(TRANS.as_slice(), AXIS.as_slice(), TAXIS.as_slice_mut());
    VHAT(TAXIS.as_slice(), UTAXIS.as_slice_mut());

    //
    // Let HPLNML be a normal vector to the plane containing
    // the transformed axis and plane vectors.
    //
    VCRSS(TPLNVC.as_slice(), TAXIS.as_slice(), HPLNML.as_slice_mut());

    if VZERO(HPLNML.as_slice()) {
        SETMSG(
            b"Plane reference vector and axis are linearly dependent.",
            ctx,
        );
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"ZZEDTMPT", ctx)?;
        return Ok(());
    }

    //
    // Let MAXR be an outer bounding radius for the transformed
    // source sphere.
    //
    MAXR = intrinsics::DMAX1(&[XA, XB, XC]);

    D = VNORM(TAXIS.as_slice());

    if UMBRAL {
        //
        // Find the angle between the negative axis and a ray tangent to
        // both the transformed target and the outer bounding sphere of
        // the transformed source. Here a tangent point on the
        // transformed target is the vertex, and the tangent ray is
        // confined to the half-plane normal to HPLNML, containing
        // TPLNVC, and bounded by the line containing TNEGAX. The
        // tangent ray does not cross the line containing TNEGAX.
        //
        ANGLE = DASINE(((MAXR - 1.0) / D), ATOL, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZEDTMPT", ctx)?;
            return Ok(());
        }

        //
        // Create the tangent point on the transformed target.
        //
        THETA = -(HALFPI(ctx) + ANGLE);

        VROTV(
            UTAXIS.as_slice(),
            HPLNML.as_slice(),
            THETA,
            TARGPT.as_slice_mut(),
        );

        //
        // S is the sign applied to pi/2 - ANGLE.
        //
        S = 1.0;
    } else {
        //
        // This is the penumbral case. The tangent ray crosses
        // the line containing TNEGAX.
        //
        //
        // The tangent line always slopes downward (toward AXIS)
        // toward the light source.
        //
        ANGLE = DASINE(((MAXR + 1.0) / D), ATOL, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZEDTMPT", ctx)?;
            return Ok(());
        }

        //
        // Create the tangent point on the transformed target.
        //
        THETA = (ANGLE - HALFPI(ctx));

        VROTV(
            UTAXIS.as_slice(),
            HPLNML.as_slice(),
            THETA,
            TARGPT.as_slice_mut(),
        );

        S = -1.0;
    }

    //
    // The tangent point is also a normal direction for the plane
    // tangent to both objects. Get the corresponding unit normal and
    // the plane constant.
    //
    VHAT(TARGPT.as_slice(), NORMAL.as_slice_mut());

    CONST = VDOT(NORMAL.as_slice(), TARGPT.as_slice());
    //
    // Find the height of the plane relative to the transformed source.
    // We'll find the unique point on the transformed source where the
    // outward normal is parallel to NORMAL and find the height of this
    // point relative to the plane.
    //
    // Let SGNNML be the "signed" normal which is parallel to NORMAL
    // in the umbral case and anti-parallel otherwise.
    //
    VSCL(S, NORMAL.as_slice(), SGNNML.as_slice_mut());

    EDNMPT(XA, XB, XC, SGNNML.as_slice(), SRCPNT.as_slice_mut(), ctx)?;

    //
    // Express the source point as an offset from the transformed
    // target center.
    //
    VADD(SRCPNT.as_slice(), TAXIS.as_slice(), TMPVEC.as_slice_mut());
    VEQU(TMPVEC.as_slice(), SRCPNT.as_slice_mut());

    //
    // H is the height of the surface point on the source, relative
    // to the plane tangent to the target at TARGPT. ANGERR is the
    // corresponding angular error estimate: an estimate of the
    // amount by which TARGPT needs to be rotated in the positive
    // sense about HPLNML to make the plane contain SRCPNT.
    //
    H = (VDOT(SRCPNT.as_slice(), NORMAL.as_slice()) - CONST);

    ANGERR = TOUCHD(-(H / D));

    NITR = 0;

    //
    // The loop terminates when the angular error magnitude
    // stops decreasing. If the iteration count exceeds the
    // limit, an error will be signaled.
    //
    while ((f64::abs(ANGERR) > CNVLIM) && (NITR <= MAXITR)) {
        //
        // Rotate the target point about HPLNML in the positive sense
        // by the angular error. This should make the tangent plane
        // closer to the source point.
        //
        VROTV(
            TARGPT.as_slice(),
            HPLNML.as_slice(),
            ANGERR,
            TMPVEC.as_slice_mut(),
        );
        VEQU(TMPVEC.as_slice(), TARGPT.as_slice_mut());
        VHAT(TARGPT.as_slice(), NORMAL.as_slice_mut());

        //
        // Re-compute the normal and constant of the tangent plane.
        //
        CONST = VDOT(NORMAL.as_slice(), TARGPT.as_slice());

        VSCL(S, NORMAL.as_slice(), SGNNML.as_slice_mut());

        //
        // Find the near point on the source to the tangent plane.
        //
        EDNMPT(XA, XB, XC, SGNNML.as_slice(), SRCPNT.as_slice_mut(), ctx)?;

        VADD(SRCPNT.as_slice(), TAXIS.as_slice(), TMPVEC.as_slice_mut());
        VEQU(TMPVEC.as_slice(), SRCPNT.as_slice_mut());

        //
        // Re-compute the height error and angular error.
        //
        H = (VDOT(SRCPNT.as_slice(), NORMAL.as_slice()) - CONST);

        VPERP(SRCPNT.as_slice(), HPLNML.as_slice(), PROJ.as_slice_mut());

        D = VDIST(PROJ.as_slice(), TARGPT.as_slice());
        ANGERR = TOUCHD(-(H / D));

        NITR = (NITR + 1);

        if (NITR > MAXITR) {
            SETMSG(
                b"Tangent finding loop failed to converge. Iteration count = #.",
                ctx,
            );
            ERRINT(b"#", NITR, ctx);
            SIGERR(b"SPICE(NOCONVERGENCE)", ctx)?;
            CHKOUT(b"ZZEDTMPT", ctx)?;
            return Ok(());
        }
    }

    //
    // Apply the inverse distortion transformation to TARGPT in order to
    // obtain the tangent point on the original, ellipsoidal target.
    //
    POINT[1] = (A * TARGPT[1]);
    POINT[2] = (B * TARGPT[2]);
    POINT[3] = (C * TARGPT[3]);

    CHKOUT(b"ZZEDTMPT", ctx)?;
    Ok(())
}
