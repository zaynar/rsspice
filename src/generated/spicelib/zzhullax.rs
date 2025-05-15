//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MARGIN: f64 = 0.000000000001;

//$Procedure   ZZHULLAX ( Pyramidal FOV convex hull to FOV axis )
pub fn ZZHULLAX(
    INST: &[u8],
    N: i32,
    BOUNDS: &[f64],
    AXIS: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=3, 1..=N);
    let mut AXIS = DummyArrayMut::new(AXIS, 1..=3);
    let mut CP = StackArray::<f64, 3>::new(1..=3);
    let mut DELTA: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut R: f64 = 0.0;
    let mut RAY1 = StackArray::<f64, 3>::new(1..=3);
    let mut RAY2 = StackArray::<f64, 3>::new(1..=3);
    let mut SEP: f64 = 0.0;
    let mut TRANS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut XVEC = StackArray::<f64, 3>::new(1..=3);
    let mut YVEC = StackArray::<f64, 3>::new(1..=3);
    let mut ZVEC = StackArray::<f64, 3>::new(1..=3);
    let mut I: i32 = 0;
    let mut M: i32 = 0;
    let mut MAXIX: i32 = 0;
    let mut MINIX: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut XIDX: i32 = 0;
    let mut FOUND: bool = false;
    let mut OK: bool = false;
    let mut PASS1: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZHULLAX", ctx)?;

    //
    // Nothing found yet.
    //
    FOUND = false;
    XIDX = 0;

    //
    // We must have at least 3 boundary vectors.
    //
    if (N < 3) {
        SETMSG(
            b"Polygonal FOV requires at least 3 boundary vectors but number supplied for # was #.",
            ctx,
        );
        ERRCH(b"#", INST, ctx);
        ERRINT(b"#", N, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZHULLAX", ctx)?;
        return Ok(());
    }

    //
    // Find an exterior face of the pyramid defined by the
    // input boundary vectors. Since most polygonal FOVs will have
    // an exterior face bounded by two consecutive rays, we'll
    // try pairs of consecutive rays first. If this fails, we'll
    // try each pair of rays.
    //
    I = 1;

    while ((I <= N) && !FOUND) {
        //
        // Set the index of the next ray. When we get to the
        // last boundary vector, the next ray is the first.
        //
        if (I == N) {
            NEXT = 1;
        } else {
            NEXT = (I + 1);
        }

        //
        // Find the cross product of the first ray with the
        // second. Depending on the ordering of the boundary
        // vectors, this could be an inward or outward normal,
        // in the case the current face is exterior.
        //
        VCRSS(
            BOUNDS.subarray([1, I]),
            BOUNDS.subarray([1, NEXT]),
            CP.as_slice_mut(),
        );

        //
        // We insist on consecutive boundary vectors being
        // linearly independent.
        //
        if VZERO(CP.as_slice()) {
            SETMSG(b"Polygonal FOV must have linearly independent consecutive boundary but vectors at indices # and # have cross product equal to the zero vector. Instrument is #.", ctx);
            ERRINT(b"#", I, ctx);
            ERRINT(b"#", NEXT, ctx);
            ERRCH(b"#", INST, ctx);
            SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
            CHKOUT(b"ZZHULLAX", ctx)?;
            return Ok(());
        }

        //
        // See whether the other boundary vectors have angular
        // separation of at least MARGIN from the plane containing
        // the current face.
        //
        PASS1 = true;
        OK = true;
        M = 1;

        while ((M <= N) && OK) {
            //
            // Find the angular separation of CP and the Mth vector if the
            // latter is not an edge of the current face.
            //
            if ((M != I) && (M != NEXT)) {
                SEP = VSEP(CP.as_slice(), BOUNDS.subarray([1, M]), ctx);

                if PASS1 {
                    //
                    // Adjust CP if necessary so that it points
                    // toward the interior of the pyramid.
                    //
                    if (SEP > HALFPI(ctx)) {
                        //
                        // Invert the cross product vector and adjust SEP
                        // accordingly. Within this "M" loop, all other
                        // angular separations will be computed using the new
                        // value of CP.
                        //
                        VSCLIP(-1.0, CP.as_slice_mut());

                        SEP = (PI(ctx) - SEP);
                    }

                    PASS1 = false;
                }

                OK = (SEP < (HALFPI(ctx) - MARGIN));
            }

            if OK {
                //
                // Consider the next boundary vector.
                //
                M = (M + 1);
            }
        }

        //
        // We've tested each boundary vector against the current face, or
        // else the loop terminated early because a vector with
        // insufficient angular separation from the plane containing the
        // face was found.
        //
        if OK {
            //
            // The current face is exterior. It's bounded by rays I and
            // NEXT.
            //
            XIDX = I;
            FOUND = true;
        } else {
            //
            // Look at the next face of the pyramid.
            //
            I = (I + 1);
        }
    }

    //
    // If we didn't find an exterior face, we'll have to look at each
    // face bounded by a pair of rays, even if those rays are not
    // adjacent. (This can be a very slow process is N is large.)
    //
    if !FOUND {
        I = 1;

        while ((I <= N) && !FOUND) {
            //
            // Consider all ray pairs (I,NEXT) where NEXT > I.
            //
            NEXT = (I + 1);

            while ((NEXT <= N) && !FOUND) {
                //
                // Find the cross product of the first ray with the second.
                // If the current face is exterior, CP could be an inward
                // or outward normal, depending on the ordering of the
                // boundary vectors.
                //
                VCRSS(
                    BOUNDS.subarray([1, I]),
                    BOUNDS.subarray([1, NEXT]),
                    CP.as_slice_mut(),
                );

                //
                // It's allowable for non-consecutive boundary vectors to
                // be linearly dependent, but if we have such a pair,
                // it doesn't define an exterior face.
                //
                if !VZERO(CP.as_slice()) {
                    //
                    // The rays having direction vectors indexed I and NEXT
                    // define a semi-infinite sector of a plane that might
                    // be of interest.
                    //
                    // Check whether all of the boundary vectors that are
                    // not edges of the current face have angular separation
                    // of at least MARGIN from the plane containing the
                    // current face.
                    //
                    PASS1 = true;
                    OK = true;
                    M = 1;

                    while ((M <= N) && OK) {
                        //
                        // Find the angular separation of CP and the Mth
                        // vector if the latter is not an edge of the current
                        // face.
                        //
                        if ((M != I) && (M != NEXT)) {
                            SEP = VSEP(CP.as_slice(), BOUNDS.subarray([1, M]), ctx);

                            if PASS1 {
                                //
                                // Adjust CP if necessary so that it points
                                // toward the interior of the pyramid.
                                //
                                if (SEP > HALFPI(ctx)) {
                                    //
                                    // Invert the cross product vector and
                                    // adjust SEP accordingly. Within this "M"
                                    // loop, all other angular separations will
                                    // be computed using the new value of CP.
                                    //

                                    VSCLIP(-1.0, CP.as_slice_mut());

                                    SEP = (PI(ctx) - SEP);
                                }

                                PASS1 = false;
                            }

                            OK = (SEP < (HALFPI(ctx) - MARGIN));
                        }

                        if OK {
                            //
                            // Consider the next boundary vector.
                            //
                            M = (M + 1);
                        }
                    }

                    //
                    // We've tested each boundary vector against the current
                    // face, or else the loop terminated early because a
                    // vector with insufficient angular separation from the
                    // plane containing the face was found.
                    //
                    if OK {
                        //
                        // The current face is exterior. It's bounded by rays
                        // I and NEXT.

                        XIDX = I;
                        FOUND = true;
                    }
                    //
                    // End of angular separation test block.
                    //
                }
                //
                // End of non-zero cross product block.
                //

                if !FOUND {
                    //
                    // Look at the face bounded by the rays
                    // at indices I and NEXT+1.
                    //
                    NEXT = (NEXT + 1);
                }
            }
            //
            // End of NEXT loop.
            //
            if !FOUND {
                //
                // Look at the face bounded by the pairs of rays
                // including the ray at index I+1.
                //
                I = (I + 1);
            }
        }
        //
        // End of I loop.
        //
    }
    //
    // End of search for exterior face using each pair of rays.
    //
    // If we still haven't found an exterior face, we can't continue.
    //
    if !FOUND {
        SETMSG(
            b"Unable to find face of convex hull of FOV of instrument #.",
            ctx,
        );
        ERRCH(b"#", INST, ctx);
        SIGERR(b"SPICE(FACENOTFOUND)", ctx)?;
        CHKOUT(b"ZZHULLAX", ctx)?;
        return Ok(());
    }

    //
    // Arrival at this point means that the rays at indices
    // XIDX and NEXT define a plane such that all boundary
    // vectors lie in a half-space bounded by that plane.
    //
    // We're now going to define a set of orthonormal basis vectors:
    //
    //    +X  points along the angle bisector of the bounding vectors
    //        of the exterior face.
    //
    //    +Y  points along CP.
    //
    //    +Z  is the cross product of +X and +Y.
    //
    // We'll call the reference frame having these basis vectors
    // the "face frame."
    //
    //
    VHAT(BOUNDS.subarray([1, I]), RAY1.as_slice_mut());
    VHAT(BOUNDS.subarray([1, NEXT]), RAY2.as_slice_mut());

    VLCOM(
        0.5,
        RAY1.as_slice(),
        0.5,
        RAY2.as_slice(),
        XVEC.as_slice_mut(),
    );

    VHATIP(XVEC.as_slice_mut());
    VHAT(CP.as_slice(), YVEC.as_slice_mut());
    UCRSS(XVEC.as_slice(), YVEC.as_slice(), ZVEC.as_slice_mut());

    //
    // Create a transformation matrix to map the input boundary
    // vectors into the face frame.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            TRANS[[1, I]] = XVEC[I];
            TRANS[[2, I]] = YVEC[I];
            TRANS[[3, I]] = ZVEC[I];

            I += m3__;
        }
    }
    //
    // Now we're going to compute the longitude of each boundary in the
    // face frame. The vectors with indices XIDX and NEXT are excluded.
    // We expect all longitudes to be between MARGIN and pi - MARGIN.
    //
    MINLON = PI(ctx);
    MAXLON = 0.0;
    MINIX = 1;
    MAXIX = 1;

    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if ((I != XIDX) && (I != NEXT)) {
                //
                // The current vector is not a boundary of our edge,
                // so find its longitude.
                //
                MXV(TRANS.as_slice(), BOUNDS.subarray([1, I]), V.as_slice_mut());

                RECLAT(V.as_slice(), &mut R, &mut LON, &mut LAT);
                //
                // Update the longitude bounds.
                //
                if (LON < MINLON) {
                    MINIX = I;
                    MINLON = LON;
                }

                if (LON > MAXLON) {
                    MAXIX = I;
                    MAXLON = LON;
                }
            }

            I += m3__;
        }
    }

    //
    // If the longitude bounds are not as expected, don't try
    // to continue.
    //
    if (MINLON < ((2 as f64) * MARGIN)) {
        SETMSG(b"Minimum boundary vector longitude in exterior face frame is # radians. Minimum occurs at index #. This FOV does not conform to the requirements of this routine. Instrument is #.", ctx);
        ERRDP(b"#", MINLON, ctx);
        ERRINT(b"#", MINIX, ctx);
        ERRCH(b"#", INST, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZHULLAX", ctx)?;
        return Ok(());
    } else if (MAXLON > (PI(ctx) - ((2 as f64) * MARGIN))) {
        SETMSG(b"Maximum boundary vector longitude in exterior face frame is # radians. Maximum occurs at index #. This FOV does not conform to the requirements of this routine. Instrument is #.", ctx);
        ERRDP(b"#", MAXLON, ctx);
        ERRINT(b"#", MAXIX, ctx);
        ERRCH(b"#", INST, ctx);
        SIGERR(b"SPICE(FOVTOOWIDE)", ctx)?;
        CHKOUT(b"ZZHULLAX", ctx)?;
        return Ok(());
    }

    //
    // Let delta represent the amount we can rotate the exterior
    // face clockwise about +Z without contacting another boundary
    // vector.
    //
    DELTA = (PI(ctx) - MAXLON);

    //
    // Rotate +Y by -DELTA/2 about +Z. The result is our candidate
    // FOV axis. Make the axis vector unit length.
    //
    VROTV(
        YVEC.as_slice(),
        ZVEC.as_slice(),
        -(DELTA / 2 as f64),
        AXIS.as_slice_mut(),
    );
    VHATIP(AXIS.as_slice_mut());

    //
    // If we have a viable result, ALL boundary vectors have
    // angular separation less than HALFPI-MARGIN from AXIS.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            SEP = VSEP(BOUNDS.subarray([1, I]), AXIS.as_slice(), ctx);

            if (SEP > (HALFPI(ctx) - MARGIN)) {
                SETMSG(b"Boundary vector at index # has angular separation of # radians from candidate FOV axis. This FOV does not conform to the requirements of this routine. Instrument is #.", ctx);
                ERRINT(b"#", I, ctx);
                ERRDP(b"#", SEP, ctx);
                ERRCH(b"#", INST, ctx);
                SIGERR(b"SPICE(FOVTOOWIDE)", ctx)?;
                CHKOUT(b"ZZHULLAX", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    CHKOUT(b"ZZHULLAX", ctx)?;
    Ok(())
}
