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
pub const ALTIDX: i32 = 3;
const RADFAC: f64 = 1.1;
const WEST: i32 = 1;
const EAST: i32 = 2;
const SOUTH: i32 = 1;
const NORTH: i32 = 2;
const LOWER: i32 = 1;
const UPPER: i32 = 2;
const NONE: i32 = 0;

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

//$Procedure ZZRYTPDT ( DSK, ray touches planetodetic element )
pub fn ZZRYTPDT(
    VERTEX: &[f64],
    RAYDIR: &[f64],
    BOUNDS: &[f64],
    CORPAR: &[f64],
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
    let CORPAR = DummyArray::new(CORPAR, 1..);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut AMNALT: f64 = 0.0;
    let mut AMXALT: f64 = 0.0;
    let mut ANGLE: f64 = 0.0;
    let mut APEX = StackArray::<f64, 3>::new(1..=3);
    let mut DIST: f64 = 0.0;
    let mut EASTB = StackArray::<f64, 3>::new(1..=3);
    let mut EBACK = StackArray::<f64, 3>::new(1..=3);
    let mut EMAX: f64 = 0.0;
    let mut EMIN: f64 = 0.0;
    let mut ENDPT2 = StackArray::<f64, 3>::new(1..=3);
    let mut F: f64 = 0.0;
    let mut LONCOV: f64 = 0.0;
    let mut MAXALT: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXLON: f64 = 0.0;
    let mut MAXR: f64 = 0.0;
    let mut MINALT: f64 = 0.0;
    let mut MINLAT: f64 = 0.0;
    let mut MINLON: f64 = 0.0;
    let mut MNDIST: f64 = 0.0;
    let mut NEGDIR = StackArray::<f64, 3>::new(1..=3);
    let mut PMAX: f64 = 0.0;
    let mut PMIN: f64 = 0.0;
    let mut RE: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut S: f64 = 0.0;
    let mut SRFX = StackArray::<f64, 3>::new(1..=3);
    let mut UDIR = StackArray::<f64, 3>::new(1..=3);
    let mut VTXANG: f64 = 0.0;
    let mut VTXLVL: f64 = 0.0;
    let mut VTXOFF = StackArray::<f64, 3>::new(1..=3);
    let mut WBACK = StackArray::<f64, 3>::new(1..=3);
    let mut WESTB = StackArray::<f64, 3>::new(1..=3);
    let mut XPT2 = StackArray::<f64, 3>::new(1..=3);
    let mut XINCPT: f64 = 0.0;
    let mut YINCPT: f64 = 0.0;
    let mut NX: i32 = 0;
    let mut FOUND: bool = false;
    let mut INSIDE: bool = false;
    let mut XIN: bool = false;
    let mut XVAL1: bool = false;
    let mut XVAL2: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Altitude expansion factor:
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

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZRYTPDT", ctx)?;

    if (MARGIN < 0.0) {
        SETMSG(b"Margin must be non-negative but was #.", ctx);
        ERRDP(b"#", MARGIN, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

    if VZERO(RAYDIR.as_slice()) {
        SETMSG(b"The ray\'s direction was the zero vector.", ctx);
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

    //
    // Determine whether the vertex is inside the element.
    //
    ZZINPDT(
        VERTEX.as_slice(),
        BOUNDS.as_slice(),
        CORPAR.as_slice(),
        MARGIN,
        NONE,
        &mut INSIDE,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

    if INSIDE {
        //
        // We know the answer.
        //
        *NXPTS = 1;

        VEQU(VERTEX.as_slice(), XPT.as_slice_mut());

        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

    //
    // Get semi-axis lengths of the reference spheroid.
    //
    RE = CORPAR[1];
    F = CORPAR[2];
    RP = (RE * (1.0 - F));

    //
    // Extract the segment's coordinate bounds into easily
    // readable variables.
    //
    MINALT = BOUNDS[[LOWER, ALTIDX]];
    MAXALT = BOUNDS[[UPPER, ALTIDX]];
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
        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

    MINLAT = BOUNDS[[SOUTH, LATIDX]];
    MAXLAT = BOUNDS[[NORTH, LATIDX]];

    //
    // Compute adjusted altitude bounds, taking margin into
    // account.
    //
    AMNALT = (MINALT - (MARGIN * f64::abs(MINALT)));
    AMXALT = (MAXALT + (MARGIN * f64::abs(MAXALT)));

    //
    // Generate semi-axis lengths of inner and outer bounding
    // ellipsoids.
    //
    if (RE >= RP) {
        //
        // The reference spheroid is oblate.
        //
        ZZELLBDS(
            RE, RP, AMXALT, AMNALT, &mut EMAX, &mut PMAX, &mut EMIN, &mut PMIN, ctx,
        )?;
    } else {
        //
        // The reference spheroid is prolate.
        //
        ZZELLBDS(
            RP, RE, AMXALT, AMNALT, &mut PMAX, &mut EMAX, &mut PMIN, &mut EMIN, ctx,
        )?;
    }

    if FAILED(ctx) {
        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

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
    // Find the intersection of the ray and outer bounding ellipsoid, if
    // possible. Often this intersection is the closest to the vertex.
    // If the intersection exists and is on the boundary of the element,
    // it's a winner.
    //
    SURFPT(
        VERTEX.as_slice(),
        UDIR.as_slice(),
        EMAX,
        EMAX,
        PMAX,
        SRFX.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

    if !FOUND {
        //
        // There are no intersections. The ray cannot hit the volume
        // element.
        //
        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

    //
    // The ray hits the outer bounding ellipsoid. See whether
    // the longitude and latitude are within bounds, taking
    // the margin into account. Exclude the altitude coordinate
    // from testing.
    //
    ZZINPDT(
        SRFX.as_slice(),
        BOUNDS.as_slice(),
        CORPAR.as_slice(),
        MARGIN,
        ALTIDX,
        &mut XIN,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

    if XIN {
        //
        // This solution is a candidate.
        //
        VEQU(SRFX.as_slice(), XPT.as_slice_mut());
        *NXPTS = 1;

        //
        // Find the level surface parameter of the vertex relative
        // to the adjusted outer bounding ellipsoid.
        //
        VTXLVL = ((f64::powi((VERTEX[1] / EMAX), 2) + f64::powi((VERTEX[2] / EMAX), 2))
            + f64::powi((VERTEX[3] / PMAX), 2));

        if (VTXLVL > 1.0) {
            //
            // The vertex is outside this ellipsoid, and the DSK segment
            // lies within the ellipsoid.
            //
            // No other intersection can be closer to the vertex;
            // we don't need to check the other surfaces.
            //
            CHKOUT(b"ZZRYTPDT", ctx)?;
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
    MAXR = intrinsics::DMAX1(&[EMAX, PMAX]);

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

        //
        // Compute the Z coordinate of the apex of the latitude cone.
        //
        ZZELNAXX(RE, RP, MAXLAT, &mut XINCPT, &mut YINCPT, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZRYTPDT", ctx)?;
            return Ok(());
        }

        APEX[1] = 0.0;
        APEX[2] = 0.0;
        APEX[3] = YINCPT;

        //
        // Find the offset of the ray's vertex from the cone's apex,
        // and find the angular separation of the offset from the +Z
        // axis. This separation enables us to compare the latitude of
        // the vertex to the latitude boundary without making a RECGEO
        // call to compute the planetodetic coordinates of the vertex.
        //
        // (The comparison will be done later.)
        //
        VSUB(VERTEX.as_slice(), APEX.as_slice(), VTXOFF.as_slice_mut());

        VTXANG = VSEP(VTXOFF.as_slice(), save.Z.as_slice(), ctx);

        //
        // Check for intersection of the ray with the latitude cone.
        //
        INCNSG(
            APEX.as_slice(),
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
            CHKOUT(b"ZZRYTPDT", ctx)?;
            return Ok(());
        }

        //
        // Unlike the case of latitudinal coordinates, for planetodetic
        // coordinates, the surface of the latitude cone does not
        // coincide with the set of points having that latitude (which is
        // equal to pi/2 - the cone's angular separation from the +Z
        // axis). The subset of the cone having the specified latitude is
        // truncated by the X-Y plane. If we ignore round-off errors, we
        // can assert that the Z-coordinate of a point having the given
        // planetodetic latitude must match the direction of the nappe of
        // the cone: positive if ANGLE < pi/2, negative if ANGLE > pi/2,
        // and 0 if ANGLE = pi/2.
        //
        // However, we cannot ignore round-off errors. For a cone having
        // angle from its central axis of nearly pi/2, it's possible for
        // a valid ray-cone intercept to be on the "wrong" side of the
        // X-Y plane due to round-off errors. So we use a more robust
        // check to determine whether an intercept should be considered
        // to have the same latitude as the cone.
        //
        // Check all intercepts.
        //
        if (NX > 0) {
            //
            // Check the first intercept.
            //
            XVAL1 = ZZPDPLTC(RE, F, SRFX.as_slice(), MAXLAT, ctx)?;
            XVAL2 = false;

            if (NX == 2) {
                //
                // Check the second intercept.
                //
                XVAL2 = ZZPDPLTC(RE, F, XPT2.as_slice(), MAXLAT, ctx)?;
            }

            if (XVAL1 && !XVAL2) {
                NX = 1;
            } else if (XVAL2 && !XVAL1) {
                //
                // Only the second solution is valid. Overwrite
                // the first.
                //
                NX = 1;

                VEQU(XPT2.as_slice(), SRFX.as_slice_mut());
            } else if (!XVAL1 && !XVAL2) {
                //
                // Neither solution is valid.
                //
                NX = 0;
            }
        }

        if (NX >= 1) {
            //
            // The ray intercept SRFX lies on the upper latitude boundary.
            //
            // See whether SRFX meets the longitude and proxy altitude
            // constraints.
            //
            ZZINPDT(
                SRFX.as_slice(),
                BOUNDS.as_slice(),
                CORPAR.as_slice(),
                MARGIN,
                LATIDX,
                &mut XIN,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZRYTPDT", ctx)?;
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

                    if (VTXANG < ANGLE) {
                        if ((MAXLAT < 0.0) || (VERTEX[3] > 0.0)) {
                            //
                            // If MAXLAT is negative, the vertex offset
                            // being outside the cone is enough to
                            // guarantee the planetodetic latitude of the
                            // vertex is greater than that of the cone.
                            //
                            // If MAXLAT is non-negative, the angle of the
                            // vertex offset relative to the +Z axis is not
                            // enough; we need the vertex to lie above the
                            // X-Y plane as well.
                            //
                            // Getting here means one of these conditions
                            // was met.
                            //
                            // Since the latitude of the vertex is greater
                            // than MAXLAT, this is the best solution, since
                            // the volume element is on the other side of the
                            // maximum latitude boundary.
                            //
                            CHKOUT(b"ZZRYTPDT", ctx)?;
                            return Ok(());
                        }
                    }
                    //
                    // This is the best solution seen so far, but we
                    // need to check the remaining boundaries.
                    //
                    MNDIST = DIST;
                }
            }

            if (NX == 2) {
                //
                // Check the second solution as well.
                //
                ZZINPDT(
                    XPT2.as_slice(),
                    BOUNDS.as_slice(),
                    CORPAR.as_slice(),
                    MARGIN,
                    LATIDX,
                    &mut XIN,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZRYTPDT", ctx)?;
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

        // Compute the Z coordinate of the apex of the latitude cone.
        //
        ZZELNAXX(RE, RP, MINLAT, &mut XINCPT, &mut YINCPT, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZRYTPDT", ctx)?;
            return Ok(());
        }

        APEX[1] = 0.0;
        APEX[2] = 0.0;
        APEX[3] = YINCPT;

        INCNSG(
            APEX.as_slice(),
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
            CHKOUT(b"ZZRYTPDT", ctx)?;
            return Ok(());
        }

        //
        // Find the offset of the ray's vertex from the cone's apex,
        // and find the angular separation of the offset from the +Z
        // axis. This separation enables us to compare the latitude of
        // the vertex to the latitude boundary without making a RECGEO
        // call to compute the planetodetic coordinates of the vertex.
        //
        // (The comparison will be done later.)
        //
        VSUB(VERTEX.as_slice(), APEX.as_slice(), VTXOFF.as_slice_mut());

        VTXANG = VSEP(VTXOFF.as_slice(), save.Z.as_slice(), ctx);

        //
        // Check whether the latitude of the intercept can be
        // considered to match that of the cone.
        //
        if (NX > 0) {
            //
            // Check the first intercept.
            //
            XVAL1 = ZZPDPLTC(RE, F, SRFX.as_slice(), MINLAT, ctx)?;
            XVAL2 = false;

            if (NX == 2) {
                //
                // Check the second intercept.
                //
                XVAL2 = ZZPDPLTC(RE, F, XPT2.as_slice(), MINLAT, ctx)?;
            }

            if (XVAL1 && !XVAL2) {
                NX = 1;
            } else if (XVAL2 && !XVAL1) {
                //
                // Only the second solution is valid. Overwrite
                // the first.
                //
                NX = 1;

                VEQU(XPT2.as_slice(), SRFX.as_slice_mut());
            } else if (!XVAL1 && !XVAL2) {
                //
                // Neither solution is valid.
                //
                NX = 0;
            }
        }

        if (NX >= 1) {
            //
            // The ray intercept SRFX lies on the lower latitude boundary.
            //
            // See whether SRFX meets the longitude and proxy altitude
            // constraints.
            //
            ZZINPDT(
                SRFX.as_slice(),
                BOUNDS.as_slice(),
                CORPAR.as_slice(),
                MARGIN,
                LATIDX,
                &mut XIN,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZRYTPDT", ctx)?;
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

                    if (VTXANG > ANGLE) {
                        if ((MINLAT > 0.0) || (VERTEX[3] < 0.0)) {
                            //
                            // If MINLAT is positive, the vertex offset
                            // being outside the cone is enough to
                            // guarantee the planetodetic latitude of the
                            // vertex is less than that of the cone.
                            //
                            // If MINLAT is non-positive, the angle of the
                            // vertex offset relative to the +Z axis is not
                            // enough; we need the vertex to lie below the
                            // X-Y plane as well.
                            //
                            // Getting here means one of these conditions
                            // was met.
                            //
                            // Since the latitude of the vertex is less than
                            // than MINLAT, this is the best solution, since
                            // the volume element is on the other side of the
                            // minimum latitude boundary.
                            //
                            CHKOUT(b"ZZRYTPDT", ctx)?;
                            return Ok(());
                        }
                    }
                    //
                    // This is the best solution seen so far, but we
                    // need to check the remaining boundaries.

                    MNDIST = DIST;
                }
            }

            if (NX == 2) {
                //
                // Check the second solution as well.
                //
                ZZINPDT(
                    XPT2.as_slice(),
                    BOUNDS.as_slice(),
                    CORPAR.as_slice(),
                    MARGIN,
                    LATIDX,
                    &mut XIN,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZRYTPDT", ctx)?;
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
                        CHKOUT(b"ZZRYTPDT", ctx)?;
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
            ZZINPDT(
                SRFX.as_slice(),
                BOUNDS.as_slice(),
                CORPAR.as_slice(),
                MARGIN,
                LONIDX,
                &mut XIN,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZRYTPDT", ctx)?;
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
            ZZINPDT(
                SRFX.as_slice(),
                BOUNDS.as_slice(),
                CORPAR.as_slice(),
                MARGIN,
                LONIDX,
                &mut XIN,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZRYTPDT", ctx)?;
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
    // Find the intersection of the ray and lower bounding
    // ellipsoid, if possible.
    //
    SURFPT(
        VERTEX.as_slice(),
        UDIR.as_slice(),
        EMIN,
        EMIN,
        PMIN,
        SRFX.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

    if FOUND {
        //
        // See whether this solution is in the element.
        //
        ZZINPDT(
            SRFX.as_slice(),
            BOUNDS.as_slice(),
            CORPAR.as_slice(),
            MARGIN,
            ALTIDX,
            &mut XIN,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZRYTPDT", ctx)?;
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
    // Unlike the outer ellipsoid, either intersection of the ray with
    // the inner ellipsoid might be a valid solution. We'll test for the
    // case where the intersection farther from the ray's vertex is the
    // correct one.
    //
    VMINUS(UDIR.as_slice(), NEGDIR.as_slice_mut());

    SURFPT(
        ENDPT2.as_slice(),
        NEGDIR.as_slice(),
        EMIN,
        EMIN,
        PMIN,
        SRFX.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZRYTPDT", ctx)?;
        return Ok(());
    }

    if FOUND {
        ZZINPDT(
            SRFX.as_slice(),
            BOUNDS.as_slice(),
            CORPAR.as_slice(),
            MARGIN,
            ALTIDX,
            &mut XIN,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZRYTPDT", ctx)?;
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
                //
                // There's no need to update MNDIST at this point.
                //
            }
        }
    }

    //
    // NXPTS and XPT are set.
    //
    CHKOUT(b"ZZRYTPDT", ctx)?;
    Ok(())
}
