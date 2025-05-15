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
const MAXHIT: i32 = 1000;

//$Procedure ZZDSKBUN ( DSK, buffered unprioritized normal vector )
pub fn ZZDSKBUN(
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
    POINT: &[f64],
    NORMAL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SRFLST = DummyArray::new(SRFLST, 1..);
    let HANBUF = DummyArray::new(HANBUF, 1..);
    let DLABUF = DummyArray2D::new(DLABUF, 1..=DLADSZ, 1..);
    let DSKBUF = DummyArray2D::new(DSKBUF, 1..=DSKDSZ, 1..);
    let OFFBUF = DummyArray2D::new(OFFBUF, 1..=3, 1..);
    let CTRBUF = DummyArray2D::new(CTRBUF, 1..=3, 1..);
    let RADBUF = DummyArray::new(RADBUF, 1..);
    let POINT = DummyArray::new(POINT, 1..=3);
    let mut NORMAL = DummyArrayMut::new(NORMAL, 1..=3);
    let mut DIST: f64 = 0.0;
    let mut SEGPT = StackArray::<f64, 3>::new(1..=3);
    let mut SGMARG: f64 = 0.0;
    let mut SGXBUF = ActualArray3D::<f64>::new(1..=3, 1..=3, 1..=MAXHIT);
    let mut VERTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut CORSYS: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut NHIT: i32 = 0;
    let mut PLATE = StackArray::<i32, 3>::new(1..=3);
    let mut PLID: i32 = 0;
    let mut PRVFRM: i32 = 0;
    let mut SEGFID: i32 = 0;
    let mut SGHIT = ActualArray::<i32>::new(1..=MAXHIT);
    let mut SURFCE: i32 = 0;
    let mut WINNER: i32 = 0;
    let mut BODYOK: bool = false;
    let mut DONE: bool = false;
    let mut INSIDE: bool = false;
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

    CHKIN(b"ZZDSKBUN", ctx)?;

    //
    // Check the incoming segment count.
    //
    if (NSEG <= 0) {
        SETMSG(b"Input segment list was empty. This may be due to no DSKs containing data for body # having been loaded.", ctx);
        ERRINT(b"#", BODYID, ctx);
        SIGERR(b"SPICE(NODSKSEGMENTS)", ctx)?;
        CHKOUT(b"ZZDSKBUN", ctx)?;
        return Ok(());
    }

    //
    // Get the segment margin from the tolerance database.
    //
    DSKGTL(KEYSGR, &mut SGMARG, ctx)?;

    //
    // Indicate we haven't yet seen a segment frame different
    // from the one designated by FIXFID.
    //
    MULTFR = false;

    //
    // Make a local copy of the point. We'll update this copy
    // later if need be.
    //
    VEQU(POINT.as_slice(), SEGPT.as_slice_mut());

    //
    // By default, each segment in the input list must be checked
    // for intersection.
    //
    // We start out by trying to eliminate segments from consideration
    // by comparing the point's distance from their centers to the radii
    // of their bounding spheres. Only those segments whose bounding
    // spheres contain the point are examined further.
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
                // In order to do any geometric comparison, the point must be
                // in the same frame as the segment we're checking. Transform
                // the point if need be.
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
                            CHKOUT(b"ZZDSKBUN", ctx)?;
                            return Ok(());
                        }

                        //
                        // Transform the local copy of the point to the segment's
                        // frame, and shift the local copy of the point so
                        // that it represents an offset relative to center of
                        // the segment's frame.
                        //
                        MXV(XFORM.as_slice(), POINT.as_slice(), SEGPT.as_slice_mut());

                        //
                        // The direction of the buffered offset is from the body
                        // to the segment frame's center. The offset is
                        // expressed in the segment's frame.
                        //
                        VSUB(
                            SEGPT.as_slice(),
                            OFFBUF.subarray([1, I]),
                            VTEMP.as_slice_mut(),
                        );
                        VEQU(VTEMP.as_slice(), SEGPT.as_slice_mut());
                    }
                } else if MULTFR {
                    //
                    // The input and segment frames are the same for this
                    // segment, but the current value of SEGPT needs to be
                    // reset.
                    //
                    VEQU(POINT.as_slice(), SEGPT.as_slice_mut());
                }

                //
                // Find the distance of the point from the "center" of the
                // segment's coverage volume.
                //
                DIST = VDIST(CTRBUF.subarray([1, I]), SEGPT.as_slice());

                if (DIST <= RADBUF[I]) {
                    //
                    // The point is inside or on the bounding surface. We'll
                    // check the boundary of the segment for an intersection.
                    //
                    CORSYS = intrinsics::IDNINT(DSKBUF[[SYSIDX, I]]);

                    if (CORSYS == LATSYS) {
                        ZZINLAT(
                            SEGPT.as_slice(),
                            DSKBUF.subarray([MN1IDX, I]),
                            SGMARG,
                            0,
                            &mut INSIDE,
                            ctx,
                        )?;
                    } else if (CORSYS == RECSYS) {
                        ZZINREC(
                            SEGPT.as_slice(),
                            DSKBUF.subarray([MN1IDX, I]),
                            SGMARG,
                            0,
                            &mut INSIDE,
                            ctx,
                        )?;
                    } else if (CORSYS == PDTSYS) {
                        ZZINPDT(
                            SEGPT.as_slice(),
                            DSKBUF.subarray([MN1IDX, I]),
                            DSKBUF.subarray([PARIDX, I]),
                            SGMARG,
                            0,
                            &mut INSIDE,
                            ctx,
                        )?;
                    } else {
                        SETMSG(b"Coordinate system # is not supported.", ctx);
                        ERRINT(b"#", CORSYS, ctx);
                        SIGERR(b"SPICE(BADCOORDSYS)", ctx)?;
                        CHKOUT(b"ZZDSKBUN", ctx)?;
                        return Ok(());
                    }

                    if FAILED(ctx) {
                        CHKOUT(b"ZZDSKBUN", ctx)?;
                        return Ok(());
                    }

                    if INSIDE {
                        //
                        // The point in inside the region enclosed by the
                        // boundary of this segment. Save the index of the
                        // segment in the "hit list."
                        //
                        if (NHIT == MAXHIT) {
                            SETMSG(
                                b"Too many segments contain the input point. Buffer size is #.",
                                ctx,
                            );
                            ERRINT(b"#", MAXHIT, ctx);
                            SIGERR(b"SPICE(TOOMANYHITS)", ctx)?;
                            CHKOUT(b"ZZDSKBUN", ctx)?;
                            return Ok(());
                        }

                        NHIT = (NHIT + 1);
                        SGHIT[NHIT] = I;
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
    // We have an error if no segments were hit. None of that
    // "not found" nonsense here.
    //
    if (NHIT == 0) {
        SETMSG(b"Input point (# # #) in frame # does not lie inside any segment for the specified body (#) and surfaces.", ctx);
        ERRDP(b"#", POINT[1], ctx);
        ERRDP(b"#", POINT[2], ctx);
        ERRDP(b"#", POINT[3], ctx);
        ERRINT(b"#", FIXFID, ctx);
        ERRINT(b"#", BODYID, ctx);
        SIGERR(b"SPICE(POINTNOTINSEGMENT)", ctx)?;
        CHKOUT(b"ZZDSKBUN", ctx)?;
        return Ok(());
    }

    //
    // Now process the segments on the hit list. If we find a segment
    // surface on which the input point is located, we compute the
    // normal vector and terminate the search.
    //
    I = 1;
    DONE = false;
    WINNER = 0;
    PRVFRM = 0;

    while !DONE {
        //
        // I is the index in the hit list of the segment
        // we're considering. J is the index of that segment
        // in the parallel input arrays.
        //
        J = SGHIT[I];

        SEGFID = intrinsics::IDNINT(DSKBUF[[FRMIDX, J]]);

        if (SEGFID != FIXFID) {
            if (SEGFID != PRVFRM) {
                //
                // Transform and shift the input point.
                //
                // Here I is an index in the hit list and J is
                // an index in the input arrays.
                //
                MOVED(SGXBUF.subarray([1, 1, I]), 9, XFORM.as_slice_mut());

                MXV(XFORM.as_slice(), POINT.as_slice(), SEGPT.as_slice_mut());

                VSUB(
                    SEGPT.as_slice(),
                    OFFBUF.subarray([1, J]),
                    VTEMP.as_slice_mut(),
                );
                VEQU(VTEMP.as_slice(), SEGPT.as_slice_mut());
            }
        } else if MULTFR {
            VEQU(POINT.as_slice(), SEGPT.as_slice_mut());
        }

        //
        // If the point lies on the surface described by the
        // current segment, find the outward unit normal
        // vector at the point.
        //
        DTYPE = intrinsics::IDNINT(DSKBUF[[TYPIDX, J]]);
        XFND = false;

        if (DTYPE == 2) {
            //
            // Find the plate on which the point lies, if any.
            //
            ZZPTPL02(
                HANBUF[J],
                DLABUF.subarray([1, J]),
                DSKBUF.subarray([1, J]),
                SEGPT.as_slice(),
                &mut PLID,
                PLATE.as_slice_mut(),
                VERTS.as_slice_mut(),
                &mut XFND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZDSKBUN", ctx)?;
                return Ok(());
            }

            if XFND {
                //
                // Find the unit outward normal at SEGPT. We must
                // convert the output of PLTNRM to unit length.
                //
                PLTNRM(
                    VERTS.subarray([1, 1]),
                    VERTS.subarray([1, 2]),
                    VERTS.subarray([1, 3]),
                    NORMAL.as_slice_mut(),
                );
                if FAILED(ctx) {
                    CHKOUT(b"ZZDSKBUN", ctx)?;
                    return Ok(());
                }

                VHATIP(NORMAL.as_slice_mut());
                //
                // WINNER is the index in the hit list of the current
                // segment.
                //
                WINNER = I;
                DONE = true;
            }
        } else {
            SETMSG(
                b"Segment type is #; this type is not currently supported.",
                ctx,
            );
            ERRINT(b"#", DTYPE, ctx);
            SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            CHKOUT(b"ZZDSKBUN", ctx)?;
            return Ok(());
        }

        if FAILED(ctx) {
            CHKOUT(b"ZZDSKBUN", ctx)?;
            return Ok(());
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
    // If we have an result, it may be represented in a frame
    // other than the input point frame. Transform it back to the
    // input frame, if need be.
    //
    if XFND {
        //
        // J is the index in the input arrays of the "winning" segment.
        //
        J = SGHIT[WINNER];

        SEGFID = intrinsics::IDNINT(DSKBUF[[FRMIDX, J]]);

        if (SEGFID != FIXFID) {
            //
            // The segment frame and input frame differ. The normal
            // vector must be converted back to the input frame.
            //
            MOVED(SGXBUF.subarray([1, 1, WINNER]), 9, XFORM.as_slice_mut());

            MTXV(XFORM.as_slice(), NORMAL.as_slice(), VTEMP.as_slice_mut());
            VEQU(VTEMP.as_slice(), NORMAL.as_slice_mut());
        }
    } else {
        //
        // The input point was not legitimate; otherwise we
        // would have found a solution.
        //
        SETMSG(b"Input point (# # #) in frame # does not lie on the surface contained in any segment for the specified body (#) and surfaces.", ctx);
        ERRDP(b"#", POINT[1], ctx);
        ERRDP(b"#", POINT[2], ctx);
        ERRDP(b"#", POINT[3], ctx);
        ERRINT(b"#", FIXFID, ctx);
        ERRINT(b"#", BODYID, ctx);
        SIGERR(b"SPICE(POINTOFFSURFACE)", ctx)?;
        CHKOUT(b"ZZDSKBUN", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZDSKBUN", ctx)?;
    Ok(())
}
