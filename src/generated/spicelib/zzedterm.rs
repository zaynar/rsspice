//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const UBEL: i32 = 9;
const UBPL: i32 = 4;
const MAXITR: i32 = 10;
const TYPLEN: i32 = 50;

//$Procedure ZZEDTERM ( Ellipsoid terminator )
pub fn ZZEDTERM(
    TYPE: &[u8],
    A: f64,
    B: f64,
    C: f64,
    SRCRAD: f64,
    SRCPOS: &[f64],
    NPTS: i32,
    TRMPTS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SRCPOS = DummyArray::new(SRCPOS, 1..=3);
    let mut TRMPTS = DummyArrayMut2D::new(TRMPTS, 1..=3, 1..=NPTS);
    let mut LOCTYP = [b' '; TYPLEN as usize];
    let mut ANGERR: f64 = 0.0;
    let mut ANGLE: f64 = 0.0;
    let mut D: f64 = 0.0;
    let mut DELTA: f64 = 0.0;
    let mut DIR = StackArray::<f64, 3>::new(1..=3);
    let mut E = StackArray::<f64, 3>::new(1..=3);
    let mut INANG: f64 = 0.0;
    let mut LAMBDA: f64 = 0.0;
    let mut MAXRAD: f64 = 0.0;
    let mut MINRAD: f64 = 0.0;
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut OUTANG: f64 = 0.0;
    let mut PLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut PLCONS: f64 = 0.0;
    let mut PRVANG: f64 = 0.0;
    let mut PRVDIF: f64 = 0.0;
    let mut RMAX: f64 = 0.0;
    let mut RMIN: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut SRCPT = StackArray::<f64, 3>::new(1..=3);
    let mut THETA: f64 = 0.0;
    let mut TRANS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut VTX = StackArray::<f64, 3>::new(1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut Y = StackArray::<f64, 3>::new(1..=3);
    let mut Z = StackArray::<f64, 3>::new(1..=3);
    let mut NITR: i32 = 0;
    let mut UMBRAL: bool = false;

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
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZEDTERM", ctx)?;

    //
    // Check the terminator type.
    //
    LJUST(TYPE, &mut LOCTYP);
    UCASE(&LOCTYP.clone(), &mut LOCTYP, ctx);

    if fstr::eq(&LOCTYP, b"UMBRAL") {
        UMBRAL = true;
    } else if fstr::eq(&LOCTYP, b"PENUMBRAL") {
        UMBRAL = false;
    } else {
        SETMSG(
            b"Terminator type must be UMBRAL or PENUMBRAL but was actually #.",
            ctx,
        );
        ERRCH(b"#", TYPE, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZEDTERM", ctx)?;
        return Ok(());
    }

    //
    // Check the terminator set dimension.
    //
    if (NPTS < 1) {
        SETMSG(b"Set must contain at least one point; NPTS  = #.", ctx);
        ERRINT(b"#", NPTS, ctx);
        SIGERR(b"SPICE(INVALIDSIZE)", ctx)?;
        CHKOUT(b"ZZEDTERM", ctx)?;
        return Ok(());
    }

    //
    // The ellipsoid semi-axes must have positive length.
    //
    if (((A <= 0.0) || (B <= 0.0)) || (C <= 0.0)) {
        SETMSG(b"Semi-axis lengths:  A = #, B = #, C = #. ", ctx);
        ERRDP(b"#", A, ctx);
        ERRDP(b"#", B, ctx);
        ERRDP(b"#", C, ctx);
        SIGERR(b"SPICE(INVALIDAXISLENGTH)", ctx)?;
        CHKOUT(b"ZZEDTERM", ctx)?;
        return Ok(());
    }

    //
    // Check the input light source radius.
    //
    if (SRCRAD <= 0.0) {
        SETMSG(
            b"Light source must have positive radius; actual radius was #.",
            ctx,
        );
        ERRDP(b"#", SRCRAD, ctx);
        SIGERR(b"SPICE(INVALIDRADIUS)", ctx)?;
        CHKOUT(b"ZZEDTERM", ctx)?;
        return Ok(());
    }

    //
    // The light source must not intersect the outer bounding
    // sphere of the ellipsoid.
    //
    D = VNORM(SRCPOS.as_slice());

    RMAX = intrinsics::DMAX1(&[A, B, C]);
    RMIN = intrinsics::DMIN1(&[A, B, C]);

    if ((SRCRAD + RMAX) >= D) {
        //
        // The light source is too close.
        //
        SETMSG(b"Light source intersects outer bounding sphere of the ellipsoid.  Light source radius = #; ellipsoid\'s longest axis = #; sum = #; distance between centers = #.", ctx);
        ERRDP(b"#", SRCRAD, ctx);
        ERRDP(b"#", RMAX, ctx);
        ERRDP(b"#", (SRCRAD + RMAX), ctx);
        ERRDP(b"#", D, ctx);
        SIGERR(b"SPICE(OBJECTSTOOCLOSE)", ctx)?;
        CHKOUT(b"ZZEDTERM", ctx)?;
        return Ok(());
    }

    //
    // Let the negative of the ellipsoid-light source vector be the
    // Z-axis of a frame we'll use to generate the terminator set.
    //
    VMINUS(SRCPOS.as_slice(), Z.as_slice_mut());

    FRAME(Z.as_slice_mut(), X.as_slice_mut(), Y.as_slice_mut());

    //
    // Create the rotation matrix required to convert vectors
    // from the source-centered frame back to the target body-fixed
    // frame.
    //
    VEQU(X.as_slice(), TRANS.subarray_mut([1, 1]));
    VEQU(Y.as_slice(), TRANS.subarray_mut([1, 2]));
    VEQU(Z.as_slice(), TRANS.subarray_mut([1, 3]));

    //
    // Find the maximum and minimum target radii.
    //
    MAXRAD = intrinsics::DMAX1(&[A, B, C]);
    MINRAD = intrinsics::DMIN1(&[A, B, C]);

    if UMBRAL {
        //
        // Compute the angular offsets from the axis of rays tangent to
        // both the source and the bounding spheres of the target, where
        // the tangency points lie in a half-plane bounded by the line
        // containing the origin and SRCPOS.  (We'll call this line
        // the "axis.")
        //
        // OUTANG corresponds to the target's outer bounding sphere;
        // INANG to the inner bounding sphere.
        //
        OUTANG = f64::asin(((SRCRAD - MAXRAD) / D));
        INANG = f64::asin(((SRCRAD - MINRAD) / D));
    } else {
        //
        // Compute the angular offsets from the axis of rays tangent to
        // both the source and the bounding spheres of the target, where
        // the tangency points lie in opposite half-planes bounded by the
        // axis (compare the case above).
        //
        // OUTANG corresponds to the target's outer bounding sphere;
        // INANG to the inner bounding sphere.
        //
        OUTANG = f64::asin(((SRCRAD + MAXRAD) / D));
        INANG = f64::asin(((SRCRAD + MINRAD) / D));
    }

    //
    // Compute the angular delta we'll use for generating
    // terminator points.
    //
    DELTA = (TWOPI(ctx) / NPTS as f64);

    //
    // Generate the terminator points.
    //
    for I in 1..=NPTS {
        THETA = (((I - 1) as f64) * DELTA);

        //
        // Let SRCPT be the surface point on the source lying in
        // the X-Y plane of the frame produced by FRAME
        // and corresponding to the angle THETA.
        //
        LATREC(SRCRAD, THETA, 0.0, SRCPT.as_slice_mut());

        //
        // Now solve for the angle by which SRCPT must be rotated (toward
        // +Z in the umbral case, away from +Z in the penumbral case)
        // so that a plane tangent to the source at SRCPT is also tangent
        // to the target. The rotation is bracketed by OUTANG on the low
        // side and INANG on the high side in the umbral case; the
        // bracketing values are reversed in the penumbral case.
        //
        if UMBRAL {
            ANGLE = OUTANG;
        } else {
            ANGLE = INANG;
        }

        PRVDIF = TWOPI(ctx);
        PRVANG = (ANGLE + HALFPI(ctx));
        NITR = 0;

        while ((NITR <= MAXITR) && (TOUCHD(f64::abs((ANGLE - PRVANG))) < PRVDIF)) {
            NITR = (NITR + 1);
            PRVDIF = TOUCHD(f64::abs((ANGLE - PRVANG)));
            PRVANG = ANGLE;
            //
            // Find the closest point on the ellipsoid to the plane
            // corresponding to "ANGLE".
            //
            // The tangent point on the source is obtained by rotating
            // SRCPT by ANGLE towards +Z.  The plane's normal vector is
            // parallel to VTX in the source-centered frame.
            //
            LATREC(SRCRAD, THETA, ANGLE, VTX.as_slice_mut());

            VEQU(VTX.as_slice(), DIR.as_slice_mut());

            //
            // VTX and DIR are expressed in the source-centered frame.  We
            // must translate VTX to the target frame and rotate both
            // vectors into that frame.
            //
            MXV(TRANS.as_slice(), VTX.as_slice(), VTEMP.as_slice_mut());
            VADD(SRCPOS.as_slice(), VTEMP.as_slice(), VTX.as_slice_mut());

            MXV(TRANS.as_slice(), DIR.as_slice(), VTEMP.as_slice_mut());
            VEQU(VTEMP.as_slice(), DIR.as_slice_mut());

            //
            // Create the plane defined by VTX and DIR.
            //
            NVP2PL(DIR.as_slice(), VTX.as_slice(), PLANE.as_slice_mut(), ctx)?;
            //
            // Find the closest point on the ellipsoid to the plane. At
            // the point we seek, the outward normal on the ellipsoid is
            // parallel to the choice of plane normal that points away
            // from the origin.  We can always obtain this choice from
            // PL2NVC.
            //
            PL2NVC(PLANE.as_slice(), DIR.as_slice_mut(), &mut PLCONS);

            //
            // At the point
            //
            //     E = (x, y, z)
            //
            // on the ellipsoid's surface, an outward normal
            // is
            //
            //     N = ( x/A**2, y/B**2, z/C**2 )
            //
            // which is also
            //
            //     lambda * ( DIR(1), DIR(2), DIR(3) )
            //
            // Equating components in the normal vectors yields
            //
            //     E = lambda * ( DIR(1)*A**2, DIR(2)*B**2, DIR(3)*C**2 )
            //
            // Taking the inner product with the point E itself and
            // applying the ellipsoid equation, we find
            //
            //     lambda * <DIR, E>  =  < N, E >  =  1
            //
            // The first term above is
            //
            //     lambda**2 * || ( A*DIR(1), B*DIR(2), C*DIR(3) ) ||**2
            //
            // So the positive root lambda is
            //
            //     1 / || ( A*DIR(1), B*DIR(2), C*DIR(3) ) ||
            //
            // Having lambda we can compute E.
            //

            VPACK((A * DIR[1]), (B * DIR[2]), (C * DIR[3]), V.as_slice_mut());

            LAMBDA = (1.0 / VNORM(V.as_slice()));

            VPACK((A * V[1]), (B * V[2]), (C * V[3]), E.as_slice_mut());

            VSCL(LAMBDA, E.as_slice(), TRMPTS.subarray_mut([1, I]));

            //
            // Make a new estimate of the plane rotation required to touch
            // the target.
            //
            VSUB(
                TRMPTS.subarray([1, I]),
                VTX.as_slice(),
                OFFSET.as_slice_mut(),
            );

            //
            // Let ANGERR be an estimate of the magnitude of angular error
            // between the plane and the terminator.
            //
            ANGERR = (VSEP(DIR.as_slice(), OFFSET.as_slice(), ctx) - HALFPI(ctx));

            //
            // Let S indicate the sign of the altitude error:  where
            // S is positive, the plane is above E.
            //
            S = f64::copysign(1.0, VDOT(E.as_slice(), DIR.as_slice()));

            if UMBRAL {
                //
                // If the plane is above the target, increase the
                // rotation angle; otherwise decrease the angle.
                //
                ANGLE = (ANGLE + (S * ANGERR));
            } else {
                //
                // This is the penumbral case; decreasing the angle
                // "lowers" the plane toward the target.
                //
                ANGLE = (ANGLE - (S * ANGERR));
            }
        }
    }

    CHKOUT(b"ZZEDTERM", ctx)?;
    Ok(())
}
