//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const LMSGLN: i32 = (23 * 80);
const SMSGLN: i32 = 25;
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
const MAXSRF: i32 = 100;
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
const CTRSIZ: i32 = 2;
const FRNMLN: i32 = 32;

struct SaveVars {
    SVMAXR: f64,
    SVMINR: f64,
    CTR: StackArray<i32, 2>,
    PRVBOD: i32,
    PRVFID: i32,
    PRVLST: StackArray<i32, 100>,
    PRVNLS: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SVMAXR: f64 = 0.0;
        let mut SVMINR: f64 = 0.0;
        let mut CTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut PRVBOD: i32 = 0;
        let mut PRVFID: i32 = 0;
        let mut PRVLST = StackArray::<i32, 100>::new(1..=MAXSRF);
        let mut PRVNLS: i32 = 0;
        let mut FIRST: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(-1), Val::I(-1)].into_iter();
            CTR.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;
        PRVFID = 0;
        PRVBOD = 0;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), MAXSRF as usize))
                .chain([]);

            PRVLST
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PRVNLS = -1;
        SVMAXR = -1.0;
        SVMINR = -1.0;

        Self {
            SVMAXR,
            SVMINR,
            CTR,
            PRVBOD,
            PRVFID,
            PRVLST,
            PRVNLS,
            FIRST,
        }
    }
}

//$Procedure ZZDSKSPH ( DSK, bounding spheres for target body )
pub fn ZZDSKSPH(
    BODYID: i32,
    NSURF: i32,
    SRFLST: &[i32],
    MINRAD: &mut f64,
    MAXRAD: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SRFLST = DummyArray::new(SRFLST, 1..);
    let mut ERRMSG = [b' '; LMSGLN as usize];
    let mut FRNAME = [b' '; FRNMLN as usize];
    let mut BOXCTR = StackArray::<f64, 3>::new(1..=3);
    let mut BOXRAD: f64 = 0.0;
    let mut CTRMNR: f64 = 0.0;
    let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
    let mut F: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut LX: f64 = 0.0;
    let mut LY: f64 = 0.0;
    let mut LZ: f64 = 0.0;
    let mut MAXR: f64 = 0.0;
    let mut MIDTIM: f64 = 0.0;
    let mut MINR: f64 = 0.0;
    let mut OFFMAG: f64 = 0.0;
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut RE: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut SGMAXR: f64 = 0.0;
    let mut SGMINR: f64 = 0.0;
    let mut CORSYS: i32 = 0;
    let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
    let mut FRAMID: i32 = 0;
    let mut FRCENT: i32 = 0;
    let mut FRCLID: i32 = 0;
    let mut FRCLAS: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut I: i32 = 0;
    let mut SURFID: i32 = 0;
    let mut FOUND: bool = false;
    let mut NEWLST: bool = false;
    let mut SAME: bool = false;
    let mut SEGFND: bool = false;
    let mut UPDATE: bool = false;

    //
    // SPICELIB functions
    //

    //
    // EXTERNAL routines
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

    CHKIN(b"ZZDSKSPH", ctx)?;

    if save.FIRST {
        ZZCTRUIN(save.CTR.as_slice_mut(), ctx);
    }

    //
    // Check NSURF.
    //
    if (NSURF < 0) {
        SETMSG(b"NSURF must be non-negative but was #.", ctx);
        ERRINT(b"#", NSURF, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZDSKSPH", ctx)?;
        return Ok(());
    }

    //
    // Determine whether the input body surface list matches
    // the previous values. The following code applies whether
    // or not the surface list is non-empty.
    //
    NEWLST = true;

    if !save.FIRST {
        if (BODYID == save.PRVBOD) {
            if (NSURF == save.PRVNLS) {
                SAME = true;
                I = 1;

                while ((I <= NSURF) && SAME) {
                    SAME = (SRFLST[I] == save.PRVLST[I]);
                    I = (I + 1);
                }
                //
                // If SAME is true here, the body and surface list are the
                // same as on the previous call.
                //
                NEWLST = !SAME;
            }
        }
    }

    //
    // Set PRVNLS to a value that can't match a valid value, so
    // the surface list won't match after an error occurs. We'll
    // reset PRVNLS prior to exit if all goes well.
    //
    save.PRVNLS = -1;

    //
    // Check for DSK update in ZZDSKBSR.
    //
    ZZDSKCHK(save.CTR.as_slice_mut(), &mut UPDATE, ctx)?;

    //
    // Initialize the temporary variables MINR, MAXR.
    //
    MINR = save.SVMINR;
    MAXR = save.SVMAXR;

    if ((save.FIRST || UPDATE) || NEWLST) {
        //
        // Initialize the saved radius data.
        //
        save.SVMAXR = -1.0;
        save.SVMINR = DPMAX();

        //
        // Prepare to fetch segment data. Initialize the ZZDSKBSR
        // segment list for the body of interest.
        //
        ZZDSKBBL(BODYID, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDSKSPH", ctx)?;
            return Ok(());
        }

        //
        // Fetch segment DSK descriptors for the indicated body and
        // surface list.
        //
        save.PRVFID = 0;
        CLEARD(3, OFFSET.as_slice_mut());

        //
        // Re-initialize MINR and MAXR.
        //
        MAXR = -1.0;
        MINR = DPMAX();

        //
        // Examine all segments for BODYID.
        //
        ZZDSKBSS(BODYID, ctx)?;

        ZZDSKSBD(BODYID, ctx);
        ZZDSKSNS(
            ZZDSKBDC,
            &mut HANDLE,
            DLADSC.as_slice_mut(),
            DSKDSC.as_slice_mut(),
            &mut SEGFND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDSKSPH", ctx)?;
            return Ok(());
        }

        while SEGFND {
            if (NSURF > 0) {
                SURFID = intrinsics::IDNINT(DSKDSC[SRFIDX]);

                I = ISRCHI(SURFID, NSURF, SRFLST.as_slice());
            } else {
                I = 1;
            }

            if (I > 0) {
                //
                // If we're checking surface IDs, this segment qualifies.
                // Otherwise, we're not checking surface IDs, so the segment
                // qualifies by default.
                //
                // Get the frame ID of this segment, and look up the frame's
                // center.
                //
                FRAMID = intrinsics::IDNINT(DSKDSC[FRMIDX]);

                if (FRAMID != save.PRVFID) {
                    //
                    // Get the frame center for the current segment.
                    //
                    FRINFO(
                        FRAMID,
                        &mut FRCENT,
                        &mut FRCLAS,
                        &mut FRCLID,
                        &mut FOUND,
                        ctx,
                    )?;

                    if !FOUND {
                        SETMSG(b"No frame specification was found for frame ID #.", ctx);
                        ERRINT(b"#", FRAMID, ctx);
                        SIGERR(b"SPICE(NOFRAMEDATA)", ctx)?;
                        CHKOUT(b"ZZDSKSPH", ctx)?;
                        return Ok(());
                    }

                    if (FRCENT == BODYID) {
                        //
                        // The frame is centered at the target, so
                        // the frame center offset magnitude is zero.
                        //
                        OFFMAG = 0.0;
                    } else {
                        FRMNAM(FRAMID, &mut FRNAME, ctx)?;

                        if FAILED(ctx) {
                            CHKOUT(b"ZZDSKSPH", ctx)?;
                            return Ok(());
                        }

                        if fstr::eq(&FRNAME, b" ") {
                            SETMSG(b"No frame name was found for frame ID #.", ctx);
                            ERRINT(b"#", FRAMID, ctx);
                            SIGERR(b"SPICE(FRAMENAMENOTFOUND)", ctx)?;
                            CHKOUT(b"ZZDSKSPH", ctx)?;
                            return Ok(());
                        }

                        MIDTIM = ((DSKDSC[BTMIDX] + DSKDSC[ETMIDX]) / 2 as f64);

                        SPKGPS(
                            FRCENT,
                            MIDTIM,
                            &FRNAME,
                            BODYID,
                            OFFSET.as_slice_mut(),
                            &mut LT,
                            ctx,
                        )?;

                        if FAILED(ctx) {
                            CHKOUT(b"ZZDSKSPH", ctx)?;
                            return Ok(());
                        }

                        OFFMAG = VNORM(OFFSET.as_slice());
                    }
                }

                //
                // Get the segment coordinate system and derive the maximum
                // radius of the segment.
                //
                CORSYS = intrinsics::IDNINT(DSKDSC[SYSIDX]);

                //
                // Get bounding radii for the segment relative to the
                // origin of the segment's coordinate system. We'll account
                // for the offset of the origin from the segment's central
                // body as a subsequent step.
                //
                if (CORSYS == LATSYS) {
                    SGMINR = DSKDSC[MN3IDX];
                    SGMAXR = DSKDSC[MX3IDX];
                } else if (CORSYS == PDTSYS) {
                    //
                    // Use the reference spheroid and altitude bounds to
                    // generate initial bounding radii.
                    //
                    RE = DSKDSC[PARIDX];
                    F = DSKDSC[(PARIDX + 1)];
                    RP = (RE * (1.0 - F));

                    if (F >= 0.0) {
                        //
                        // The spheroid is oblate. The maximum altitude over
                        // the equator is an upper bound for the distance of
                        // any surface point from the origin. The minimum
                        // altitude over either pole is a lower bound for
                        // the distance of any surface point from the origin.
                        //
                        // The DSK descriptor gives us the altitude bounds.
                        //
                        SGMAXR = (RE + DSKDSC[MX3IDX]);
                        SGMINR = (RP + DSKDSC[MN3IDX]);
                    } else {
                        // The spheroid is prolate. The maximum altitude over
                        // either pole is an upper bound for the distance of
                        // any surface point from the origin.
                        //
                        SGMAXR = (RP + DSKDSC[MX3IDX]);
                        SGMINR = (RE + DSKDSC[MN3IDX]);
                    }
                } else if (CORSYS == RECSYS) {
                    ZZRECBOX(
                        DSKDSC.subarray(MN1IDX),
                        BOXCTR.as_slice_mut(),
                        &mut LX,
                        &mut LY,
                        &mut LZ,
                        &mut BOXRAD,
                        ctx,
                    )?;
                    //
                    // SGMINR is a lower bound on the distance of the
                    // segment from the origin of the coordinate system.
                    //
                    SGMINR = intrinsics::DMAX1(&[(VNORM(BOXCTR.as_slice()) - BOXRAD), 0.0]);

                    SGMAXR = (VNORM(BOXCTR.as_slice()) + BOXRAD);
                } else {
                    SETMSG(b"Coordinate system # is not currently supported.", ctx);
                    ERRINT(b"#", CORSYS, ctx);
                    SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                    CHKOUT(b"ZZDSKSPH", ctx)?;
                    return Ok(());
                }

                //
                // Apply the triangle inequality to derive minimum and
                // maximum values of the distance of the surface from the
                // body center, given the offset between the frame center
                // and the body center, and given bounds on the distance of
                // the surface from the frame's center.
                //
                if (OFFMAG <= SGMINR) {
                    //
                    // The segment's central body is inside the inner
                    // bounding sphere of the segment.
                    //
                    CTRMNR = (SGMINR - OFFMAG);
                } else if (OFFMAG >= SGMAXR) {
                    //
                    // The segment's central body is outside the outer
                    // bounding sphere of the segment.
                    //
                    CTRMNR = (OFFMAG - SGMAXR);
                } else {
                    //
                    // The segment's central body is between the bounding
                    // spheres. No positive lower radius bound exists.
                    //
                    CTRMNR = 0.0;
                }

                //
                // Update the segment's outer bounding radius to
                // account for the frame center offset (which may
                // be zero).
                //
                SGMAXR = (SGMAXR + OFFMAG);

                //
                // Update the global minimum and maximum radii.
                //
                MINR = intrinsics::DMIN1(&[MINR, CTRMNR]);
                MAXR = intrinsics::DMAX1(&[MAXR, SGMAXR]);
            }
            //
            // Look at the next segment.
            //
            ZZDSKSBD(BODYID, ctx);
            ZZDSKSNS(
                ZZDSKBDC,
                &mut HANDLE,
                DLADSC.as_slice_mut(),
                DSKDSC.as_slice_mut(),
                &mut SEGFND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZDSKSPH", ctx)?;
                return Ok(());
            }
        }

        if ((MAXR > 0.0) && !FAILED(ctx)) {
            //
            // Update the saved bounds.
            //
            save.SVMINR = MINR;
            save.SVMAXR = MAXR;
        }
    }

    if (MAXR < 0.0) {
        //
        // We tried to update the radius bounds but didn't find any
        // segments for the specified body.
        //
        // We have no radius data for the specified surface list.
        //
        if (NSURF == 0) {
            fstr::assign(
                &mut ERRMSG,
                b"No segments were found matching the body ID #.",
            );
        } else {
            fstr::assign(
                &mut ERRMSG,
                b"No segments were found matching the body ID # and the surface list <@>.",
            );

            {
                let m1__: i32 = 1;
                let m2__: i32 = (NSURF - 1);
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    REPMC(&ERRMSG.clone(), b"@", b"*, @", &mut ERRMSG);
                    REPMI(&ERRMSG.clone(), b"*", SRFLST[I], &mut ERRMSG, ctx);

                    I += m3__;
                }
            }

            REPMI(&ERRMSG.clone(), b"@", SRFLST[NSURF], &mut ERRMSG, ctx);
        }

        SETMSG(&ERRMSG, ctx);
        ERRINT(b"#", BODYID, ctx);
        SIGERR(b"SPICE(DSKDATANOTFOUND)", ctx)?;
        CHKOUT(b"ZZDSKSPH", ctx)?;
        return Ok(());
    }

    if !FAILED(ctx) {
        save.FIRST = false;

        save.PRVBOD = BODYID;
        save.PRVNLS = NSURF;

        if NEWLST {
            MOVEI(SRFLST.as_slice(), NSURF, save.PRVLST.as_slice_mut());
        }

        *MAXRAD = save.SVMAXR;
        *MINRAD = save.SVMINR;
    }

    CHKOUT(b"ZZDSKSPH", ctx)?;
    Ok(())
}
