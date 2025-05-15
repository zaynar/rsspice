//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const XFRACT: f64 = 0.0000000001;
const KEYXFR: i32 = 1;
const SGREED: f64 = 0.00000001;
const KEYSGR: i32 = (KEYXFR + 1);
const SGPADM: f64 = 0.0000000001;
const KEYSPM: i32 = (KEYSGR + 1);
const PTMEMM: f64 = 0.0000001;
const KEYPTM: i32 = (KEYSPM + 1);
const ANGMRG: f64 = 0.000000000001;
const KEYAMG: i32 = (KEYPTM + 1);
const LONALI: f64 = 0.000000000001;
const KEYLAL: i32 = (KEYAMG + 1);
pub const LONIDX: i32 = 1;
pub const LATIDX: i32 = 2;
pub const RADIDX: i32 = 3;
const RADFAC: f64 = 1.1;
const WEST: i32 = 1;
const EAST: i32 = 2;
const SOUTH: i32 = 1;
const NORTH: i32 = 2;
const INNER: i32 = 1;
const OUTER: i32 = 2;

struct SaveVars {
    ORIGIN: StackArray<f64, 3>,
    Z: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut Z = StackArray::<f64, 3>::new(1..=3);

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
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ORIGIN, Z }
    }
}

//$Procedure ZZRYTLAT ( DSK, ray touches latitudinal element )
pub fn ZZRYTLAT(
    VERTEX: &[f64],
    RAYDIR: &[f64],
    BOUNDS: &[f64],
    MARGIN: f64,
    NXPTS: &mut i32,
    XPT: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let BOUNDS = DummyArray2D::new(BOUNDS, 1..=2, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut ANGLE: f64 = 0.0;
    let mut DIST: f64 = 0.0;
    let mut EASTB = StackArray::<f64, 3>::new(1..=3);
    let mut EBACK = StackArray::<f64, 3>::new(1..=3);
    let mut ENDPT2 = StackArray::<f64, 3>::new(1..=3);
    let mut LONCOV: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MAXR: f64 = 0.0;
    let mut MINLAT: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut MINR: f64 = 0.0;
    let mut MNDIST: f64 = 0.0;
    let mut NEGDIR = StackArray::<f64, 3>::new(1..=3);
    let mut S: f64 = 0.0;
    let mut SRFX = StackArray::<f64, 3>::new(1..=3);
    let mut UDIR = StackArray::<f64, 3>::new(1..=3);
    let mut VLAT: f64 = 0.0;
    let mut VLON: f64 = 0.0;
    let mut VR: f64 = 0.0;
    let mut WBACK = StackArray::<f64, 3>::new(1..=3);
    let mut WESTB = StackArray::<f64, 3>::new(1..=3);
    let mut XPT2 = StackArray::<f64, 3>::new(1..=3);
    let mut NX: i32 = 0;
    let mut FOUND: bool = false;
    let mut INSIDE: bool = false;
    let mut XIN: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Radius expansion factor:
    //

    //
    // Element boundary indices:
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
    // CAUTION: this routine doesn't check in, so it won't
    // appear in the traceback if a lower-level routine
    // signals an error.
    //

    //
    // Determine whether the vertex is inside the element.
    // Use double the margin for this test, since we don't
    // want to have false negative tests for rays having
    // vertices lying on the expanded element boundary.
    //
    ZZINLAT(
        VERTEX.as_slice(),
        BOUNDS.as_slice(),
        ((2 as f64) * MARGIN),
        0,
        &mut INSIDE,
        ctx,
    )?;

    if FAILED(ctx) {
        return Ok(());
    }

    if INSIDE {
        //
        // We know the answer.
        //
        *NXPTS = 1;

        VEQU(VERTEX.as_slice(), XPT.as_slice_mut());

        return Ok(());
    }

    //
    // Extract the segment's coordinate bounds into easily
    // readable variables.
    //
    MINR = BOUNDS[[INNER, RADIDX]];
    MAXR = BOUNDS[[OUTER, RADIDX]];
    //
    // Normalize the longitude bounds. After this step, the bounds will
    // be in order and differ by no more than 2*pi.
    //
    ZZNRMLON(
        BOUNDS[[WEST, LONIDX]],
        BOUNDS[[EAST, LONIDX]],
        ANGMRG,
        &mut MINLON,
        &mut MAXLON,
        ctx,
    )?;

    if FAILED(ctx) {
        return Ok(());
    }

    MINLAT = BOUNDS[[SOUTH, LATIDX]];
    MAXLAT = BOUNDS[[NORTH, LATIDX]];

    //
    // The vertex is outside the element.
    //
    // Indicate no intersection to start.
    //
    *NXPTS = 0;
    //
    // We'll use a unit length copy of the ray's direction vector.
    //
    VHAT(RAYDIR.as_slice(), UDIR.as_slice_mut());
    //
    // Initialize the distance to the closest solution. We'll keep track
    // of this quantity in order to compare competing solutions.
    //
    MNDIST = DPMAX();
    //
    // Find the intersection of the ray and outer bounding sphere, if
    // possible. Often this intersection is the closest to the vertex.
    // If the intersection exists and is on the boundary of the element,
    // it's a winner.
    //
    ZZRYXSPH(
        VERTEX.as_slice(),
        UDIR.as_slice(),
        MAXR,
        SRFX.as_slice_mut(),
        &mut FOUND,
    );

    if !FOUND {
        //
        // There are no intersections. The ray cannot hit the
        // volume element.
        //
        return Ok(());
    }
    //
    // Get the latitudinal coordinates of the ray's vertex.
    //
    RECLAT(VERTEX.as_slice(), &mut VR, &mut VLON, &mut VLAT);

    //
    // The ray hits the outer bounding sphere. See whether
    // the longitude and latitude are within bounds, taking
    // the margin into account. Exclude the radius coordinate
    // from testing.
    //
    ZZINLAT(
        SRFX.as_slice(),
        BOUNDS.as_slice(),
        MARGIN,
        RADIDX,
        &mut XIN,
        ctx,
    )?;

    if FAILED(ctx) {
        return Ok(());
    }

    if XIN {
        //
        // This solution is a candidate.
        //
        VEQU(SRFX.as_slice(), XPT.as_slice_mut());
        *NXPTS = 1;

        if (VR > MAXR) {
            //
            // The vertex is outside this sphere, and the segment
            // lies within the sphere.
            //
            // No other intersection can be closer to the vertex;
            // we don't need to check the other surfaces.
            //
            return Ok(());
        } else {
            //
            // We have a possible solution.
            //
            MNDIST = VDIST(VERTEX.as_slice(), XPT.as_slice());
        }
    }

    //
    // So far there may be a candidate solution. We'll try the latitude
    // boundaries next.
    //
    // For testing intersections with the latitude boundaries, we'll
    // need a far endpoint for the line segment on which to perform the
    // test.
    //
    S = (VNORM(VERTEX.as_slice()) + (RADFAC * MAXR));

    VLCOM(
        1.0,
        VERTEX.as_slice(),
        S,
        UDIR.as_slice(),
        ENDPT2.as_slice_mut(),
    );

    //
    // Now try the upper latitude bound. We can skip this test
    // if the upper bound is pi/2 radians.
    //
    if (MAXLAT < HALFPI(ctx)) {
        //
        // Let ANGLE be the angular separation of the surface of latitude
        // MAXLAT and the +Z axis. Note that the surface might be the
        // lower nappe of the cone.
        //
        ANGLE = intrinsics::DMAX1(&[0.0, (HALFPI(ctx) - MAXLAT)]);

        INCNSG(
            save.ORIGIN.as_slice(),
            save.Z.as_slice(),
            ANGLE,
            VERTEX.as_slice(),
            ENDPT2.as_slice(),
            &mut NX,
            SRFX.as_slice_mut(),
            XPT2.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            return Ok(());
        }

        if (NX >= 1) {
            //
            // See whether SRFX is in the element.
            //
            ZZINLAT(
                SRFX.as_slice(),
                BOUNDS.as_slice(),
                MARGIN,
                LATIDX,
                &mut XIN,
                ctx,
            )?;

            if FAILED(ctx) {
                return Ok(());
            }

            if XIN {
                //
                // SRFX is a candidate solution.
                //
                DIST = VDIST(VERTEX.as_slice(), SRFX.as_slice());

                if (DIST < MNDIST) {
                    VEQU(SRFX.as_slice(), XPT.as_slice_mut());

                    *NXPTS = 1;

                    if (VLAT > MAXLAT) {
                        //
                        // Since the latitude of the vertex is greater than
                        // MAXLAT, this is the best solution, since the
                        // volume element is on the other side of the maximum
                        // latitude boundary.
                        //
                        return Ok(());
                    }
                    //
                    // This is the best solution seen so far.
                    //
                    MNDIST = DIST;
                }
            }

            if (NX == 2) {
                //
                // Check the second solution as well.
                //
                ZZINLAT(
                    XPT2.as_slice(),
                    BOUNDS.as_slice(),
                    MARGIN,
                    LATIDX,
                    &mut XIN,
                    ctx,
                )?;

                if FAILED(ctx) {
                    return Ok(());
                }

                if XIN {
                    //
                    // XPT2 is a candidate solution.
                    //
                    DIST = VDIST(VERTEX.as_slice(), XPT2.as_slice());

                    if (DIST < MNDIST) {
                        VEQU(XPT2.as_slice(), XPT.as_slice_mut());

                        *NXPTS = 1;
                        MNDIST = DIST;
                        //
                        // This is the best solution seen so far.
                        // However, it's not necessarily the best
                        // solution. So we continue.
                        //
                    }
                }
            }
            //
            // We've handled the second root, if any.
            //
        }
        //
        // We're done with the upper latitude boundary.
        //
    }

    //
    // Try the lower latitude bound. We can skip this test if the lower
    // bound is -pi/2 radians.
    //
    if (MINLAT > -HALFPI(ctx)) {
        //
        // Let ANGLE be the angular separation of the surface
        // of latitude MINLAT and the +Z axis. Note that the
        // surface might be the lower nappe of the cone.
        //
        ANGLE = (HALFPI(ctx) - MINLAT);

        INCNSG(
            save.ORIGIN.as_slice(),
            save.Z.as_slice(),
            ANGLE,
            VERTEX.as_slice(),
            ENDPT2.as_slice(),
            &mut NX,
            SRFX.as_slice_mut(),
            XPT2.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            return Ok(());
        }

        if (NX >= 1) {
            //
            // See whether SRFX is in the element.
            //
            ZZINLAT(
                SRFX.as_slice(),
                BOUNDS.as_slice(),
                MARGIN,
                LATIDX,
                &mut XIN,
                ctx,
            )?;

            if FAILED(ctx) {
                return Ok(());
            }

            if XIN {
                //
                // SRFX is a candidate solution.
                //
                DIST = VDIST(VERTEX.as_slice(), SRFX.as_slice());

                if (DIST < MNDIST) {
                    VEQU(SRFX.as_slice(), XPT.as_slice_mut());

                    *NXPTS = 1;

                    if (VLAT < MINLAT) {
                        //
                        // Since the latitude of the vertex is less than
                        // MINLAT, this is the best solution, since the
                        // volume element is on the other side of the minimum
                        // latitude boundary.
                        //
                        return Ok(());
                    }
                    //
                    // This is the best solution seen so far.
                    //
                    MNDIST = DIST;
                }
            }

            if (NX == 2) {
                //
                // Check the second solution as well.
                //
                ZZINLAT(
                    XPT2.as_slice(),
                    BOUNDS.as_slice(),
                    MARGIN,
                    LATIDX,
                    &mut XIN,
                    ctx,
                )?;

                if FAILED(ctx) {
                    return Ok(());
                }

                if XIN {
                    //
                    // XPT2 is a candidate solution.
                    //
                    DIST = VDIST(VERTEX.as_slice(), XPT2.as_slice());

                    if (DIST < MNDIST) {
                        VEQU(XPT2.as_slice(), XPT.as_slice_mut());

                        *NXPTS = 1;
                        MNDIST = DIST;
                        //
                        // This is the best solution seen so far.
                        // However, it's not necessarily the best
                        // solution. So we continue.
                        //
                        return Ok(());
                    }
                }
            }
        }
        //
        // We're done with the lower latitude boundary.
        //
    }

    //
    // Perform longitude boundary checks if the coverage is not
    // 2*pi radians. Note that MAXLON > MINLON at this point.
    //
    LONCOV = (MAXLON - MINLON);

    if (f64::cos(LONCOV) < 1.0) {
        //
        // We have distinct longitude boundaries. Go to work.
        //
        //
        // Check the longitude boundaries. Try the plane of western
        // longitude first.
        //
        // The vector WESTB is an outward-facing vector normal to
        // the west boundary.
        //
        VPACK(
            f64::sin(MINLON),
            -f64::cos(MINLON),
            0.0,
            WESTB.as_slice_mut(),
        );

        S = (RADFAC * (VNORM(VERTEX.as_slice()) + MAXR));

        ZZINRYPL(
            VERTEX.as_slice(),
            UDIR.as_slice(),
            WESTB.as_slice(),
            0.0,
            S,
            &mut NX,
            SRFX.as_slice_mut(),
        );

        if (NX == 1) {
            //
            // We have one point of intersection. Determine whether it's a
            // candidate solution.  Don't use longitude in the following
            // inclusion test. Note that we'll perform a separate check
            // later in place of the longitude check.
            //
            ZZINLAT(
                SRFX.as_slice(),
                BOUNDS.as_slice(),
                MARGIN,
                LONIDX,
                &mut XIN,
                ctx,
            )?;

            if FAILED(ctx) {
                return Ok(());
            }

            if XIN {
                //
                // Make sure the intercept is not too far on the
                // "wrong" side of the Z axis.
                //
                UCRSS(WESTB.as_slice(), save.Z.as_slice(), WBACK.as_slice_mut());

                if (VDOT(SRFX.as_slice(), WBACK.as_slice()) < (MARGIN * MAXR)) {
                    //
                    // The intercept is either on the same side of the Z
                    // axis as the west face of the segment, or is very
                    // close to the Z axis.
                    //
                    DIST = VDIST(VERTEX.as_slice(), SRFX.as_slice());

                    if (DIST < MNDIST) {
                        //
                        // Record the intercept, distance, and surface index.
                        //
                        VEQU(SRFX.as_slice(), XPT.as_slice_mut());

                        *NXPTS = 1;
                        MNDIST = DIST;
                    }
                }
            }
        }
        //
        // We're done with the western boundary.
        //
        //
        // Try the plane of eastern longitude next.
        //
        // The vector EASTB is an outward-facing vector normal to
        // the east boundary.

        VPACK(
            -f64::sin(MAXLON),
            f64::cos(MAXLON),
            0.0,
            EASTB.as_slice_mut(),
        );

        ZZINRYPL(
            VERTEX.as_slice(),
            UDIR.as_slice(),
            EASTB.as_slice(),
            0.0,
            S,
            &mut NX,
            SRFX.as_slice_mut(),
        );

        if (NX == 1) {
            //
            // We have one point of intersection. Determine whether it's a
            // candidate solution.
            //
            ZZINLAT(
                SRFX.as_slice(),
                BOUNDS.as_slice(),
                MARGIN,
                LONIDX,
                &mut XIN,
                ctx,
            )?;

            if FAILED(ctx) {
                return Ok(());
            }

            if XIN {
                //
                // Make sure the intercept is not too far on the "wrong"
                // side of the Z axis.
                //
                UCRSS(save.Z.as_slice(), EASTB.as_slice(), EBACK.as_slice_mut());

                if (VDOT(SRFX.as_slice(), EBACK.as_slice()) < (MARGIN * MAXR)) {
                    //
                    // The intercept is either on the same side of the Z
                    // axis as the east face of the segment, or is very
                    // close to the Z axis.
                    //
                    DIST = VDIST(VERTEX.as_slice(), SRFX.as_slice());

                    if (DIST < MNDIST) {
                        //
                        // Record the intercept, distance, and surface index.
                        //
                        VEQU(SRFX.as_slice(), XPT.as_slice_mut());

                        *NXPTS = 1;
                        MNDIST = DIST;
                    }
                }
            }
        }
    }
    //
    // End of longitude boundary checks.
    //

    //
    // Find the intersection of the ray and inner bounding
    // sphere, if possible.
    //
    if (MINR > 0.0) {
        ZZRYXSPH(
            VERTEX.as_slice(),
            UDIR.as_slice(),
            MINR,
            SRFX.as_slice_mut(),
            &mut FOUND,
        );

        if FOUND {
            //
            // See whether this solution is in the element.
            //
            ZZINLAT(
                SRFX.as_slice(),
                BOUNDS.as_slice(),
                MARGIN,
                RADIDX,
                &mut XIN,
                ctx,
            )?;

            if FAILED(ctx) {
                return Ok(());
            }

            if XIN {
                DIST = VDIST(VERTEX.as_slice(), SRFX.as_slice());

                if (DIST < MNDIST) {
                    //
                    // Record the intercept, distance, and surface index.
                    //
                    VEQU(SRFX.as_slice(), XPT.as_slice_mut());

                    *NXPTS = 1;
                    MNDIST = DIST;
                }
            }
        }

        //
        // Unlike the outer sphere, either intersection of the ray with
        // the inner sphere can be an optimal solution. We'll test for
        // the case where the intercept further from the ray's vertex is
        // the correct solution.
        //
        VMINUS(UDIR.as_slice(), NEGDIR.as_slice_mut());

        ZZRYXSPH(
            ENDPT2.as_slice(),
            NEGDIR.as_slice(),
            MINR,
            SRFX.as_slice_mut(),
            &mut FOUND,
        );

        if FOUND {
            ZZINLAT(
                SRFX.as_slice(),
                BOUNDS.as_slice(),
                MARGIN,
                RADIDX,
                &mut XIN,
                ctx,
            )?;

            if FAILED(ctx) {
                return Ok(());
            }

            if XIN {
                DIST = VDIST(VERTEX.as_slice(), SRFX.as_slice());

                if (DIST < MNDIST) {
                    //
                    // Record the intercept, distance, and surface index.
                    //
                    VEQU(SRFX.as_slice(), XPT.as_slice_mut());

                    *NXPTS = 1;
                    MNDIST = DIST;
                }
            }
        }
    }

    //
    // NXPTS and XPT are set.
    //
    Ok(())
}
