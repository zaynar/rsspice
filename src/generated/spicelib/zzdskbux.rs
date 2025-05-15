//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;
const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
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
const CTRSIZ: i32 = 2;
const RADSCL: f64 = 1.01;
const MAXHIT: i32 = 1000;

//$Procedure ZZDSKBUX ( DSK, buffered unprioritized ray intercept )
pub fn ZZDSKBUX(
    BODYID: i32,
    NSURF: i32,
    SRFLST: &[i32],
    ET: f64,
    FIXFID: i32,
    NSEG: i32,
    HANBUF: &[i32],
    DLABUF: &[i32],
    DSKBUF: &[f64],
    OFFBUF: &[f64],
    CTRBUF: &[f64],
    RADBUF: &[f64],
    VERTEX: &[f64],
    RAYDIR: &[f64],
    XPT: &mut [f64],
    SEGIDX: &mut i32,
    DC: &mut [f64],
    IC: &mut [i32],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SRFLST = DummyArray::new(SRFLST, 1..);
    let HANBUF = DummyArray::new(HANBUF, 1..);
    let DLABUF = DummyArray2D::new(DLABUF, 1..=DLADSZ, 1..);
    let DSKBUF = DummyArray2D::new(DSKBUF, 1..=DSKDSZ, 1..);
    let OFFBUF = DummyArray2D::new(OFFBUF, 1..=3, 1..);
    let CTRBUF = DummyArray2D::new(CTRBUF, 1..=3, 1..);
    let RADBUF = DummyArray::new(RADBUF, 1..);
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut DC = DummyArrayMut::new(DC, 1..);
    let mut IC = DummyArrayMut::new(IC, 1..);
    let mut DIST: f64 = 0.0;
    let mut DMIN: f64 = 0.0;
    let mut MAXRAD: f64 = 0.0;
    let mut MINRAD: f64 = 0.0;
    let mut SEGDIR = StackArray::<f64, 3>::new(1..=3);
    let mut SEGVTX = StackArray::<f64, 3>::new(1..=3);
    let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut SGDIST = ActualArray::<f64>::new(1..=MAXHIT);
    let mut SGMARG: f64 = 0.0;
    let mut SGXBUF = ActualArray3D::<f64>::new(1..=3, 1..=3, 1..=MAXHIT);
    let mut SPHVTX = StackArray::<f64, 3>::new(1..=3);
    let mut SRFX = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DTYPE: i32 = 0;
    let mut I: i32 = 0;
    let mut IORDER = ActualArray::<i32>::new(1..=MAXHIT);
    let mut J: i32 = 0;
    let mut K: i32 = 0;
    let mut NHIT: i32 = 0;
    let mut NXPTS: i32 = 0;
    let mut PRVFRM: i32 = 0;
    let mut SEGFID: i32 = 0;
    let mut SGHIT = ActualArray::<i32>::new(1..=MAXHIT);
    let mut SURFCE: i32 = 0;
    let mut WINNER: i32 = 0;
    let mut BODYOK: bool = false;
    let mut DONE: bool = false;
    let mut MULTFR: bool = false;
    let mut SURFOK: bool = false;
    let mut TIMEOK: bool = false;
    let mut XFND: bool = false;

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

    CHKIN(b"ZZDSKBUX", ctx)?;

    //
    // Check the incoming segment count.
    //
    if (NSEG <= 0) {
        SETMSG(b"Input segment list was empty. This may be due to no DSKs containing data for body # having been loaded.", ctx);
        ERRINT(b"#", BODYID, ctx);
        SIGERR(b"SPICE(NODSKSEGMENTS)", ctx)?;
        CHKOUT(b"ZZDSKBUX", ctx)?;
        return Ok(());
    }

    //
    // No intercept has been found.
    //
    *SEGIDX = 0;
    *FOUND = false;

    //
    // Indicate we haven't yet seen a segment frame different
    // from the one designated by FIXFID.
    //
    MULTFR = false;

    //
    // Obtain the "greedy" segment margin.
    //
    DSKGTL(KEYSGR, &mut SGMARG, ctx)?;

    //
    // Make a local copy of the ray. We'll update this copy
    // later if need be.
    //
    VEQU(VERTEX.as_slice(), SEGVTX.as_slice_mut());
    VEQU(RAYDIR.as_slice(), SEGDIR.as_slice_mut());

    //
    // Obtain the radius of an outer bounding sphere for the given body
    // and surface list.
    //
    ZZDSKSPH(
        BODYID,
        NSURF,
        SRFLST.as_slice(),
        &mut MINRAD,
        &mut MAXRAD,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZDSKBUX", ctx)?;
        return Ok(());
    }
    //
    // Scale up the bounding sphere to avoid round-off difficulties.
    // We'll use this value in the loop below.
    //
    MAXRAD = (MAXRAD * RADSCL);

    //
    // If the ray's vertex is distant from the target, use a vertex on
    // the surface of the outer bounding sphere of the target.
    //
    // Note that distant vertices can give rise to large transverse
    // displacements of the ray's intercepts on the segments'
    // boundaries. In cases where the ray intercept is very close to the
    // segment's spatial coverage boundaries, this can cause the ray to
    // miss all plates in type 2 segments, unless a large plate
    // expansion factor is used. Using an intercept on the outer
    // bounding sphere greatly ameliorates this problem.
    //
    if (VNORM(SEGVTX.as_slice()) > MAXRAD) {
        //
        // Find the intercept of the ray with the outer bounding
        // sphere. We'll use this intercept as the vertex for
        // later computations.
        //
        SURFPT(
            SEGVTX.as_slice(),
            SEGDIR.as_slice(),
            MAXRAD,
            MAXRAD,
            MAXRAD,
            SPHVTX.as_slice_mut(),
            &mut XFND,
            ctx,
        )?;

        if (FAILED(ctx) || !XFND) {
            //
            // It would be highly unusual for the SURFPT call to
            // fail to produce an intercept. Check anyway.
            //
            CHKOUT(b"ZZDSKBUX", ctx)?;
            return Ok(());
        }

        VEQU(SPHVTX.as_slice(), SEGVTX.as_slice_mut());
    }

    //
    // By default, each segment in the input list must be checked
    // for intersection.
    //
    // We start out by trying to eliminate segments from consideration
    // by comparing the ray's distance from their centers to the radii
    // of their bounding spheres. Only those segments whose bounding
    // spheres are hit are examined further.
    //
    NHIT = 0;
    PRVFRM = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = NSEG;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // BODYOK indicates whether the input body ID matches that
            // of the current segment.
            //
            SURFOK = false;
            TIMEOK = false;

            BODYOK = (BODYID == intrinsics::IDNINT(DSKBUF[[CTRIDX, I]]));

            if BODYOK {
                //
                // See whether the current segment contains a surface we're
                // supposed to consider. If the surface list is empty, we
                // consider all surfaces. Otherwise, the surface of the
                // segment must be on the surface list in order to qualify.
                //
                J = 0;

                if (NSURF > 0) {
                    SURFCE = intrinsics::IDNINT(DSKBUF[[SRFIDX, I]]);
                    J = ISRCHI(SURFCE, NSURF, SRFLST.as_slice());
                }

                SURFOK = ((NSURF == 0) || (J > 0));

                //
                // See whether the segment covers the input epoch.
                //
                TIMEOK = ((ET >= DSKBUF[[BTMIDX, I]]) && (ET <= DSKBUF[[ETMIDX, I]]));
            }

            if ((BODYOK && SURFOK) && TIMEOK) {
                //
                // This segment is to be considered.
                //
                // In order to do any geometric comparison, the ray must be in
                // the same frame as the segment we're checking. Transform the
                // input vertex and ray if need be.
                //
                // Get the segment's frame ID. Get the transformation from the
                // input frame to the output frame if needed.

                SEGFID = intrinsics::IDNINT(DSKBUF[[FRMIDX, I]]);

                if (SEGFID != FIXFID) {
                    //
                    // We have a segment that uses a different frame
                    // from that specified by FIXFID.
                    //
                    MULTFR = true;

                    if (SEGFID != PRVFRM) {
                        //
                        // The frame of the current segment doesn't match
                        // that of the previous segment, so we'll need
                        // to look up the transformation from the input
                        // frame to the segment frame.
                        //
                        // Otherwise, XFORM already contains the correct
                        // transformation.
                        //
                        REFCHG(FIXFID, SEGFID, ET, XFORM.as_slice_mut(), ctx)?;

                        if FAILED(ctx) {
                            CHKOUT(b"ZZDSKBUX", ctx)?;
                            return Ok(());
                        }

                        //
                        // Transform the local copy of the ray to the segment's
                        // frame, and shift the local copy of the ray's vertex
                        // so that it represents an offset relative to center of
                        // the segment's frame.
                        //
                        MXV(XFORM.as_slice(), RAYDIR.as_slice(), SEGDIR.as_slice_mut());
                        MXV(XFORM.as_slice(), VERTEX.as_slice(), SEGVTX.as_slice_mut());

                        //
                        // The direction of the buffered offset is from the body
                        // to the segment frame's center. The offset is
                        // expressed in the segment's frame.
                        //
                        VSUB(
                            SEGVTX.as_slice(),
                            OFFBUF.subarray([1, I]),
                            VTEMP.as_slice_mut(),
                        );
                        VEQU(VTEMP.as_slice(), SEGVTX.as_slice_mut());
                    }
                } else if MULTFR {
                    //
                    // The input and segment frames are the same for this
                    // segment, but the current values of SEGVTX and SEGDIR
                    // need to be reset.
                    //
                    VEQU(VERTEX.as_slice(), SEGVTX.as_slice_mut());
                    VEQU(RAYDIR.as_slice(), SEGDIR.as_slice_mut());
                }

                //
                // Find the distance of the ray from the "center" of the
                // segment's coverage volume.
                //
                NPLNPT(
                    SEGVTX.as_slice(),
                    SEGDIR.as_slice(),
                    CTRBUF.subarray([1, I]),
                    PNEAR.as_slice_mut(),
                    &mut DIST,
                    ctx,
                )?;

                if (DIST <= RADBUF[I]) {
                    //
                    // The line containing the ray intersects the bounding
                    // surface. We'll check the boundary of the segment for an
                    // intersection.
                    //
                    ZZRYTELT(
                        SEGVTX.as_slice(),
                        SEGDIR.as_slice(),
                        DSKBUF.subarray([1, I]),
                        SGMARG,
                        &mut NXPTS,
                        XPT.as_slice_mut(),
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"ZZDSKBUX", ctx)?;
                        return Ok(());
                    }

                    if (NXPTS > 0) {
                        //
                        // The ray hits the boundary of this segment. Save the
                        // index of the segment in the "hit list." Record the
                        // distance from the ray's vertex to the intercept in
                        // the parallel array SGDIST.

                        if (NHIT == MAXHIT) {
                            SETMSG(
                                b"Too many segments were hit by the input ray. Buffer size is #.",
                                ctx,
                            );
                            ERRINT(b"#", MAXHIT, ctx);
                            SIGERR(b"SPICE(BUFFERTOOSMALL)", ctx)?;
                            CHKOUT(b"ZZDSKBUX", ctx)?;
                            return Ok(());
                        }

                        NHIT = (NHIT + 1);
                        SGHIT[NHIT] = I;
                        SGDIST[NHIT] = VDIST(SEGVTX.as_slice(), XPT.as_slice());
                        //
                        // Save the frame transformation for this segment.
                        //
                        MOVED(XFORM.as_slice(), 9, SGXBUF.subarray_mut([1, 1, NHIT]));
                    }
                }
                //
                // The current segment matched the input criteria.
                //
                // Update the saved segment frame ID to that of the segment
                // we just examined.
                //
                PRVFRM = SEGFID;
            }

            I += m3__;
        }
    }

    //
    // Leave now if no segments were hit.
    //
    if (NHIT == 0) {
        CHKOUT(b"ZZDSKBUX", ctx)?;
        return Ok(());
    }

    //
    // Find the order of the segments on the hit list, where
    // the metric is the distance of the ray intercepts from
    // the ray's vertex.
    //
    ORDERD(SGDIST.as_slice(), NHIT, IORDER.as_slice_mut());

    //
    // Now process the segments on the hit list in order. If we find a
    // surface intercept (that is, a ray intercept with the surface
    // represented by the segment's data, as opposed to the segment's
    // boundary), compare its distance from the ray's vertex to the
    // vertex-boundary distance of the next segment. If the
    // vertex-surface distance is smaller, we terminate the search,
    // since no other segment can contribute a closer intercept.
    //
    I = 1;
    DONE = false;
    WINNER = 0;
    PRVFRM = 0;

    while !DONE {
        //
        // J is the index in the hit list of the segment
        // we're considering. K is the index of that segment
        // in the parallel input arrays.
        //
        J = IORDER[I];
        K = SGHIT[J];

        SEGFID = intrinsics::IDNINT(DSKBUF[[FRMIDX, K]]);

        if (SEGFID != FIXFID) {
            if (SEGFID != PRVFRM) {
                //
                // Transform and shift the input ray.
                //
                // Here J is an index in the hit list and K is
                // an index in the input arrays.
                //
                MOVED(SGXBUF.subarray([1, 1, J]), 9, XFORM.as_slice_mut());

                MXV(XFORM.as_slice(), VERTEX.as_slice(), SEGVTX.as_slice_mut());
                MXV(XFORM.as_slice(), RAYDIR.as_slice(), SEGDIR.as_slice_mut());

                VSUB(
                    SEGVTX.as_slice(),
                    OFFBUF.subarray([1, K]),
                    VTEMP.as_slice_mut(),
                );
                VEQU(VTEMP.as_slice(), SEGVTX.as_slice_mut());
            }
        } else if MULTFR {
            VEQU(VERTEX.as_slice(), SEGVTX.as_slice_mut());
            VEQU(RAYDIR.as_slice(), SEGDIR.as_slice_mut());
        }

        //
        // Find the surface intercept using the segment topography
        // data.
        //
        DTYPE = intrinsics::IDNINT(DSKBUF[[TYPIDX, K]]);

        ZZDSKSGX(
            HANBUF[K],
            DLABUF.subarray([1, K]),
            DTYPE,
            ET,
            SEGVTX.as_slice(),
            SEGDIR.as_slice(),
            SRFX.as_slice_mut(),
            DC.as_slice_mut(),
            IC.as_slice_mut(),
            &mut XFND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDSKBUX", ctx)?;
            return Ok(());
        }

        if XFND {
            //
            // At least one surface intercept exists.
            //
            if !*FOUND {
                //
                // The intercept we just found is the first one, and
                // at least for now, it is the winner.
                //
                // Save the intercept and vertex-intercept distance
                // for this segment.
                //
                *FOUND = true;
                DMIN = VDIST(SRFX.as_slice(), SEGVTX.as_slice());

                VEQU(SRFX.as_slice(), XPT.as_slice_mut());

                WINNER = J;
            } else {
                //
                // At least one surface intercept was found already.
                //
                // Compare the vertex-intercept distance for this segment
                // to the best found so far.
                //
                DIST = VDIST(SRFX.as_slice(), SEGVTX.as_slice());

                if (DIST < DMIN) {
                    //
                    // This intercept is closer to the ray's vertex than
                    // any we've seen yet. We have a new winner.
                    //
                    DMIN = DIST;

                    VEQU(SRFX.as_slice(), XPT.as_slice_mut());

                    WINNER = J;
                }
            }
        }

        //
        // If there's at least one solution in hand, see whether
        // we can stop looking for better solutions.
        //
        if *FOUND {
            if (I < NHIT) {
                //
                // There are more segments in the hit list. Compare the
                // minimum vertex-intercept distance of the segments we've
                // checked to the vertex-boundary distance of the next
                // segment.
                //
                J = IORDER[(I + 1)];

                if (DMIN <= SGDIST[J]) {
                    //
                    // The best intercept we've found is closer to the
                    // vertex than any intercept that may exist in the
                    // current segment, or any of the remaining segments in
                    // the hit list.
                    //
                    DONE = true;
                }
            }
        }

        if !DONE {
            if (I == NHIT) {
                //
                // We've looked at all of the segments.
                //
                DONE = true;
            } else {
                //
                // Consider the next segment.
                //
                I = (I + 1);
            }
        }
    }

    //
    // If we have an intercept, it may be represented in a frame
    // other than the input ray frame. Transform it back to the
    // input frame, and shift it as well if the segment frame and
    // input frame have different centers.
    //
    if *FOUND {
        //
        // K is the index in the input arrays of the "winning" segment.
        // We'll return this index as SEGIDX.
        //
        K = SGHIT[WINNER];
        *SEGIDX = K;

        SEGFID = intrinsics::IDNINT(DSKBUF[[FRMIDX, K]]);

        if (SEGFID != FIXFID) {
            //
            // The segment frame and input frame differ. The intercept
            // must be converted back to the input frame. It also may
            // need to be shifted so that it is expressed as an offset
            // from the target body. If the shift is done, it must be
            // done before the frame transformation, since the offset
            // is expressed relative to the segment's frame.
            //
            if !VZERO(OFFBUF.subarray([1, K])) {
                //
                // OFFBUF(*,K) contains the offset of the segment frame
                // center from the segment's central body.
                //
                VADD(
                    XPT.as_slice(),
                    OFFBUF.subarray([1, K]),
                    VTEMP.as_slice_mut(),
                );
                VEQU(VTEMP.as_slice(), XPT.as_slice_mut());
            }

            MOVED(SGXBUF.subarray([1, 1, WINNER]), 9, XFORM.as_slice_mut());

            MTXV(XFORM.as_slice(), XPT.as_slice(), VTEMP.as_slice_mut());
            VEQU(VTEMP.as_slice(), XPT.as_slice_mut());
        }
    }
    //
    // FOUND and XPT are set.
    //
    CHKOUT(b"ZZDSKBUX", ctx)?;
    Ok(())
}
