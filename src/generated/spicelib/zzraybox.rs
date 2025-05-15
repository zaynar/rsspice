//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LEFT: i32 = 1;
const MIDDLE: i32 = 2;
const RIGHT: i32 = 3;

//$Procedure ZZRAYBOX ( Ray-box intercept )
pub fn ZZRAYBOX(
    VERTEX: &[f64],
    RAYDIR: &[f64],
    BOXORI: &[f64],
    EXTENT: &[f64],
    XPT: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let BOXORI = DummyArray::new(BOXORI, 1..=3);
    let EXTENT = DummyArray::new(EXTENT, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut LIMIT: f64 = 0.0;
    let mut MAXT: f64 = 0.0;
    let mut NEAR = StackArray::<f64, 3>::new(1..=3);
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut PLNDST = StackArray::<f64, 3>::new(1..=3);
    let mut R: f64 = 0.0;
    let mut SPHXPT = StackArray::<f64, 3>::new(1..=3);
    let mut SPHVTX = StackArray::<f64, 3>::new(1..=3);
    let mut T = StackArray::<f64, 3>::new(1..=3);
    let mut UDIR = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut SECTOR = StackArray::<i32, 3>::new(1..=3);
    let mut MAXIDX: i32 = 0;
    let mut SPHFND: bool = false;

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
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // No intercept has been found yet.
    //
    *FOUND = false;

    //
    // Check for a zero ray direction vector.
    //
    if VZERO(RAYDIR.as_slice()) {
        CHKIN(b"ZZRAYBOX", ctx)?;
        SETMSG(
            b"Input ray direction was the zero vector; this vector must be non-zero.",
            ctx,
        );
        SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        CHKOUT(b"ZZRAYBOX", ctx)?;
        return Ok(());
    }

    VHAT(RAYDIR.as_slice(), UDIR.as_slice_mut());

    //
    // Check the box extents.
    //
    if (intrinsics::DMIN1(&[EXTENT[1], EXTENT[2], EXTENT[3]]) <= 0.0) {
        CHKIN(b"ZZRAYBOX", ctx)?;
        SETMSG(
            b"All box extents should be strictly positive but the extents were #, #, #.",
            ctx,
        );
        ERRDP(b"#", EXTENT[1], ctx);
        ERRDP(b"#", EXTENT[2], ctx);
        ERRDP(b"#", EXTENT[3], ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZRAYBOX", ctx)?;
        return Ok(());
    }

    //
    // Compute the coordinates of the center of the box, and compute
    // the offset of the ray's vertex from the center.
    //
    for I in 1..=3 {
        CENTER[I] = (BOXORI[I] + (EXTENT[I] / 2 as f64));
    }

    VSUB(VERTEX.as_slice(), CENTER.as_slice(), OFFSET.as_slice_mut());

    //
    // If the ray's vertex is inside the box, we consider the
    // vertex to be the intercept.
    //
    if (((f64::abs(OFFSET[1]) <= (EXTENT[1] / 2 as f64))
        && (f64::abs(OFFSET[2]) <= (EXTENT[2] / 2 as f64)))
        && (f64::abs(OFFSET[3]) <= (EXTENT[3] / 2 as f64)))
    {
        VEQU(VERTEX.as_slice(), XPT.as_slice_mut());
        *FOUND = true;

        return Ok(());
    }
    //
    // Compute the intercept of the ray on the surface of a bounding
    // sphere that contains the box. Let R be the radius of this sphere.
    //
    R = ((0.5 * (1.0 + 0.001)) * VNORM(EXTENT.as_slice()));

    if (VNORM(OFFSET.as_slice()) < R) {
        //
        // The vertex is already inside the bounding sphere.
        //
        VEQU(OFFSET.as_slice(), SPHXPT.as_slice_mut());
    } else {
        SURFPT(
            OFFSET.as_slice(),
            UDIR.as_slice(),
            R,
            R,
            R,
            SPHXPT.as_slice_mut(),
            &mut SPHFND,
            ctx,
        )?;

        if !SPHFND {
            //
            // The ray misses the bounding sphere.
            //
            return Ok(());
        }
    }

    //
    // Shift the sphere intercept so as to be relative to the
    // box's origin. From this point on, we'll treat BOXORI
    // as the origin of the reference frame.
    //
    for I in 1..=3 {
        SPHVTX[I] = ((SPHXPT[I] + CENTER[I]) - BOXORI[I]);
    }

    //
    // Classify the position of the vertex relative to the planes
    // bounding the box: each coordinate will be classified as
    // "left," "middle," or "right" depending on whether it is
    // less than the lower bound for that coordinate, between
    // the bounds, or greater than the upper bound.
    //
    for I in 1..=3 {
        if (SPHVTX[I] < 0.0) {
            SECTOR[I] = LEFT;
            NEAR[I] = 0.0;
        } else if (SPHVTX[I] > EXTENT[I]) {
            SECTOR[I] = RIGHT;
            NEAR[I] = EXTENT[I];
        } else {
            SECTOR[I] = MIDDLE;
            NEAR[I] = 0.0;
        }
    }

    //
    // At this point, SPHVTX is a point on the ray that is outside,
    // but close to, the box. SPHVTX is an offset from BOXORI we'll
    // need to add BOXORI to it to obtain the corresponding point in
    // the input reference frame.
    //
    // We'll use SPHVTX as the new ray vertex.
    //
    // Find the distances of the vertex's components from the nearest
    // bounding planes of the box; find the corresponding distances
    // along the ray that would be traveled in order to move each
    // component from the vertex to the nearest bounding plane. Call the
    // latter distance for the Ith coordinate T(I). We're only
    // interested in the vertex components that are "outside" the
    // bounding planes. Mark the values of T(I) for components in the
    // "middle" using the value -1.
    //
    // Find the index of the maximum T value while we're at it. If
    // there's an intercept, it occurs at the point on the ray
    // corresponding to the maximum value of T.
    //
    MAXIDX = 1;
    MAXT = -1.0;

    for I in 1..=3 {
        T[I] = -1.0;

        if (SECTOR[I] != MIDDLE) {
            PLNDST[I] = (NEAR[I] - SPHVTX[I]);

            //
            // Prepare for a "safe" division.
            //
            LIMIT = (((2 as f64) * R) * f64::abs(UDIR[I]));

            if (f64::abs(PLNDST[I]) > LIMIT) {
                //
                // The ray can't get to the nearest bounding plane
                // before exiting the bounding sphere. No intersection
                // is possible.
                //
                return Ok(());
            }
            //
            // The magnitude of the following quotient is bounded by 2R.
            //
            T[I] = (PLNDST[I] / UDIR[I]);

            if (T[I] < 0.0) {
                //
                // This component of the ray is going in the wrong
                // direction. No intersection is possible.
                //
                return Ok(());
            }

            if (T[I] > MAXT) {
                MAXIDX = I;
                MAXT = T[I];
            }
        }
    }

    //
    // We should have a positive value of T for at least one
    // coordinate. However, if we don't, there's no intersection.
    //
    if (MAXT < 0.0) {
        return Ok(());
    }

    //
    // Compute the candidate intercept. Note that we're now working
    // in a frame centered at the box origin.
    //
    VLCOM(
        1.0,
        SPHVTX.as_slice(),
        MAXT,
        UDIR.as_slice(),
        XPT.as_slice_mut(),
    );

    //
    // Decide whether XPT is actually on the surface of the box.
    // Sharpen XPT as part of the process.
    //
    for I in 1..=3 {
        if (I == MAXIDX) {
            //
            // XPT is supposed to lie exactly on the bounding plane
            // orthogonal to the Ith axis and nearest to SPHVTX.
            //
            XPT[I] = NEAR[I];
        } else {
            if (SECTOR[I] == MIDDLE) {
                //
                // The Ith component of the vertex is between the
                // bounding planes for the Ith coordinate. If the
                // Ith component of XPT is outside these bounds,
                // the ray misses the box.
                //
                if ((XPT[I] < 0.0) || (XPT[I] > EXTENT[I])) {
                    return Ok(());
                }
            } else {
                //
                // The Ith component of the vertex SPHVTX is outside of the
                // bounding planes for the Ith coordinate. Since T(MAXIDX)
                // is greater than or equal to T(I), XPT(I) should be on or
                // past the bounding plane closest to SPHVTX(I). Sharpen
                // XPT(I) if necessary. If XPT(I) is beyond the bounding
                // plane farthest from SPHVTX(I), no intersection can
                // exist.
                //
                if (SECTOR[I] == LEFT) {
                    //
                    // Sharpen the Ith component of XPT.
                    //
                    XPT[I] = intrinsics::DMAX1(&[XPT[I], 0.0]);

                    if (XPT[I] > EXTENT[I]) {
                        //
                        // The ray hits the MAXIDX face too far away from
                        // SPHVTX(I). There's no intersection with the box.
                        //
                        return Ok(());
                    }
                } else {
                    //
                    // SECTOR(I) .EQ. RIGHT
                    //
                    // Sharpen the Ith component of XPT.
                    //
                    XPT[I] = intrinsics::DMIN1(&[XPT[I], EXTENT[I]]);

                    if (XPT[I] < 0.0) {
                        //
                        // The ray hits the MAXIDX face too far away from
                        // SPHVTX(I). There's no intersection with the box.
                        //
                        return Ok(());
                    }
                }
                //
                // End of block in which the Ith component of XPT is
                // either sharpened or found to be off the surface of the
                // box. This block deals with the components other than
                // MAXIDX.
                //
            }
            //
            // End of block in which the Ith component of XPT is either
            // sharpened or found to be off the surface of the box. This
            // block deals with all components.
            //
        }
    }
    //
    // End of loop in which XPT is either sharpened or found to be off
    // the surface of the box. Getting here means XPT is valid.
    //
    // Shift XPT to the input reference frame.
    //
    VADD(XPT.as_slice(), BOXORI.as_slice(), VTEMP.as_slice_mut());
    VEQU(VTEMP.as_slice(), XPT.as_slice_mut());

    *FOUND = true;

    Ok(())
}
