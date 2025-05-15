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
const IXNV: i32 = 1;
const IXNP: i32 = (IXNV + 1);
const IXNVXT: i32 = (IXNP + 1);
const IXVGRX: i32 = (IXNVXT + 1);
const IXCGSC: i32 = (IXVGRX + 3);
const IXVXPS: i32 = (IXCGSC + 1);
const IXVXLS: i32 = (IXVXPS + 1);
const IXVTLS: i32 = (IXVXLS + 1);
const IXPLAT: i32 = (IXVTLS + 1);
const IXDSCR: i32 = 1;
const DSCSZ2: i32 = 24;
const IXVTBD: i32 = (IXDSCR + DSCSZ2);
const IXVXOR: i32 = (IXVTBD + 6);
const IXVXSZ: i32 = (IXVXOR + 3);
const IXVERT: i32 = (IXVXSZ + 1);
const KWNV: i32 = 1;
const KWNP: i32 = (KWNV + 1);
const KWNVXT: i32 = (KWNP + 1);
const KWVGRX: i32 = (KWNVXT + 1);
const KWCGSC: i32 = (KWVGRX + 1);
const KWVXPS: i32 = (KWCGSC + 1);
const KWVXLS: i32 = (KWVXPS + 1);
const KWVTLS: i32 = (KWVXLS + 1);
const KWPLAT: i32 = (KWVTLS + 1);
const KWVXPT: i32 = (KWPLAT + 1);
const KWVXPL: i32 = (KWVXPT + 1);
const KWVTPT: i32 = (KWVXPL + 1);
const KWVTPL: i32 = (KWVTPT + 1);
const KWCGPT: i32 = (KWVTPL + 1);
const KWDSC: i32 = (KWCGPT + 1);
const KWVTBD: i32 = (KWDSC + 1);
const KWVXOR: i32 = (KWVTBD + 1);
const KWVXSZ: i32 = (KWVXOR + 1);
const KWVERT: i32 = (KWVXSZ + 1);
const MAXVRT: i32 = 16000002;
const MAXPLT: i32 = (2 * (MAXVRT - 2));
const MAXNPV: i32 = (((3 * MAXPLT) / 2) + 1);
const MAXVOX: i32 = 100000000;
const MAXCGR: i32 = 100000;
const MAXEDG: i32 = 120;
const SIVGRX: i32 = 1;
const SICGSC: i32 = (SIVGRX + 3);
const SIVXNP: i32 = (SICGSC + 1);
const SIVXNL: i32 = (SIVXNP + 1);
const SIVTNL: i32 = (SIVXNL + 1);
const SICGRD: i32 = (SIVTNL + 1);
const IXIFIX: i32 = (MAXCGR + 7);
const SIVTBD: i32 = 1;
const SIVXOR: i32 = (SIVTBD + 6);
const SIVXSZ: i32 = (SIVXOR + 3);
const IXDFIX: i32 = 10;
const MAXVXP: i32 = (MAXPLT / 2);
const MAXCEL: i32 = 60000000;
const MXNVLS: i32 = (MAXCEL + (MAXVXP / 2));
const SPAISZ: i32 = ((((IXIFIX + MAXVXP) + MXNVLS) + MAXVRT) + MAXNPV);
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
const BUFSIZ: i32 = 1000;

struct SaveVars {
    LIMIT: f64,
    MAXR: f64,
    VOXORI: StackArray<f64, 3>,
    VOXSIZ: f64,
    CGRSCL: i32,
    CORSYS: i32,
    PRVDSC: StackArray<i32, 8>,
    PRVHAN: i32,
    VGREXT: StackArray<i32, 3>,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LIMIT: f64 = 0.0;
        let mut MAXR: f64 = 0.0;
        let mut VOXORI = StackArray::<f64, 3>::new(1..=3);
        let mut VOXSIZ: f64 = 0.0;
        let mut CGRSCL: i32 = 0;
        let mut CORSYS: i32 = 0;
        let mut PRVDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
        let mut PRVHAN: i32 = 0;
        let mut VGREXT = StackArray::<i32, 3>::new(1..=3);
        let mut PASS1: bool = false;

        LIMIT = -1.0;
        PASS1 = true;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), DLADSZ as usize))
                .chain([]);

            PRVDSC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PRVHAN = 0;

        Self {
            LIMIT,
            MAXR,
            VOXORI,
            VOXSIZ,
            CGRSCL,
            CORSYS,
            PRVDSC,
            PRVHAN,
            VGREXT,
            PASS1,
        }
    }
}

//$Procedure ZZPTPL02 ( DSK, map point to plate, type 2 )
pub fn ZZPTPL02(
    HANDLE: i32,
    DLADSC: &[i32],
    DSKDSC: &[f64],
    POINT: &[f64],
    PLID: &mut i32,
    PLATE: &mut [i32],
    VERTS: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DLADSC = DummyArray::new(DLADSC, 1..);
    let DSKDSC = DummyArray::new(DSKDSC, 1..);
    let POINT = DummyArray::new(POINT, 1..=3);
    let mut PLATE = DummyArrayMut::new(PLATE, 1..=3);
    let mut VERTS = DummyArrayMut2D::new(VERTS, 1..=3, 1..=3);
    let mut BOXCTR = StackArray::<f64, 3>::new(1..=3);
    let mut DIST: f64 = 0.0;
    let mut DMIN: f64 = 0.0;
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut PNTOFF = StackArray::<f64, 3>::new(1..=3);
    let mut PTSRFM: f64 = 0.0;
    let mut VRTTMP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XPDFRC: f64 = 0.0;
    let mut XVERTS = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut CGRCOR = StackArray::<i32, 3>::new(1..=3);
    let mut CGREXT = StackArray::<i32, 3>::new(1..=3);
    let mut CGROFF = StackArray::<i32, 3>::new(1..=3);
    let mut CGRPTR: i32 = 0;
    let mut CGRVID: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut N: i32 = 0;
    let mut NPLATE: i32 = 0;
    let mut NREAD: i32 = 0;
    let mut PIDTMP: i32 = 0;
    let mut PLTBUF = ActualArray::<i32>::new(1..=BUFSIZ);
    let mut PLTPTR: i32 = 0;
    let mut PLTTMP = StackArray::<i32, 3>::new(1..=3);
    let mut PTRLOC: i32 = 0;
    let mut PTROFF: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut START: i32 = 0;
    let mut VGRCOR = StackArray::<i32, 3>::new(1..=3);
    let mut VID: i32 = 0;
    let mut INSIDE: bool = false;

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

    CHKIN(b"ZZPTPL02", ctx)?;

    //
    // No plate has been found so far.
    //
    *FOUND = false;

    //
    // Decide whether we're looking at the segment we saw
    // on the previous call.
    //
    if (save.PASS1
        || !DLASSG(
            HANDLE,
            save.PRVHAN,
            DLADSC.as_slice(),
            save.PRVDSC.as_slice(),
        ))
    {
        //
        // We'll need to look up the voxel grid parameters for this
        // segment.
        //
        DSKD02(
            HANDLE,
            DLADSC.as_slice(),
            KWVXOR,
            1,
            3,
            &mut N,
            save.VOXORI.as_slice_mut(),
            ctx,
        )?;
        DSKD02(
            HANDLE,
            DLADSC.as_slice(),
            KWVXSZ,
            1,
            1,
            &mut N,
            std::slice::from_mut(&mut save.VOXSIZ),
            ctx,
        )?;
        DSKI02(
            HANDLE,
            DLADSC.as_slice(),
            KWVGRX,
            1,
            3,
            &mut N,
            save.VGREXT.as_slice_mut(),
            ctx,
        )?;
        DSKI02(
            HANDLE,
            DLADSC.as_slice(),
            KWCGSC,
            1,
            1,
            &mut N,
            std::slice::from_mut(&mut save.CGRSCL),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZPTPL02", ctx)?;
            return Ok(());
        }

        if (save.VOXSIZ == 0.0) {
            SETMSG(b"Voxel edge length is zero; length must be positive.", ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"ZZPTPL02", ctx)?;
            return Ok(());
        }

        if (save.CGRSCL == 0) {
            SETMSG(b"Coarse voxel scale is zero; scale must be positive.", ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"ZZPTPL02", ctx)?;
            return Ok(());
        }

        save.CORSYS = intrinsics::IDNINT(DSKDSC[SYSIDX]);

        ZZSEGBOX(
            DSKDSC.as_slice(),
            BOXCTR.as_slice_mut(),
            &mut save.MAXR,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZPTPL02", ctx)?;
            return Ok(());
        }

        //
        // We successfully obtained the desired segment parameters, so we
        // don't need to execute this code again until the segment
        // changes. Save the current handle and DLA descriptor.
        //
        save.PRVHAN = HANDLE;
        MOVEI(DLADSC.as_slice(), DLADSZ, save.PRVDSC.as_slice_mut());

        save.PASS1 = false;
    }

    //
    // Look up the point-plate membership margin; compute
    // the distance limit. This call must be made on every
    // call to ZZPTPL02.
    //
    DSKGTL(KEYPTM, &mut PTSRFM, ctx)?;

    save.LIMIT = (PTSRFM * save.MAXR);

    //
    // Look up the plate expansion fraction. This call must be made on
    // every call to ZZPTPL02.
    //
    DSKGTL(KEYXFR, &mut XPDFRC, ctx)?;

    //
    // Find out whether the point is within the volume element
    // bounding the segment.
    //
    ZZINVELT(
        POINT.as_slice(),
        save.CORSYS,
        DSKDSC.subarray(PARIDX),
        DSKDSC.subarray(MN1IDX),
        PTSRFM,
        0,
        &mut INSIDE,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZPTPL02", ctx)?;
        return Ok(());
    }

    if !INSIDE {
        //
        // The point is too far from the segment to be considered
        // to lie on a plate in that segment.
        //
        CHKOUT(b"ZZPTPL02", ctx)?;
        return Ok(());
    }

    //
    // Map the point to the coordinates of a voxel containing it. If the
    // point is outside the voxel grid, map the point to the closest
    // voxel.
    //
    VSUB(
        POINT.as_slice(),
        save.VOXORI.as_slice(),
        OFFSET.as_slice_mut(),
    );

    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            J = (((OFFSET[I] / save.VOXSIZ) as i32) + 1);

            VGRCOR[I] = BRCKTI(J, 1, save.VGREXT[I]);

            I += m3__;
        }
    }

    //
    // Compute the coordinates of the coarse voxel containing the fine
    // voxel we just identified. Get the 1-d offset of the fine voxel
    // relative the coarse voxel; this offset gives us the index of the
    // pointer associating the fine voxel with its plate list. The
    // 1-d offset PTROFF is 1-based.
    //
    ZZVOXCVO(
        VGRCOR.as_slice(),
        save.VGREXT.as_slice(),
        save.CGRSCL,
        CGRCOR.as_slice_mut(),
        CGROFF.as_slice_mut(),
        &mut PTROFF,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZPTPL02", ctx)?;
        return Ok(());
    }

    //
    // Fetch the pointer from the coarse voxel to the first element of
    // its fine voxel pointer array.
    //
    // We'll need the 1-D offset of the coarse voxel from the base of
    // the coarse voxel grid.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = 3;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            CGREXT[I] = (save.VGREXT[I] / save.CGRSCL);
            I += m3__;
        }
    }

    CGRVID = ZZVOX2ID(CGRCOR.as_slice(), CGREXT.as_slice());

    DSKI02(
        HANDLE,
        DLADSC.as_slice(),
        KWCGPT,
        CGRVID,
        1,
        &mut N,
        std::slice::from_mut(&mut CGRPTR),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZPTPL02", ctx)?;
        return Ok(());
    }

    if (CGRPTR < 1) {
        //
        // There are no non-empty fine voxels, hence no plates, in the
        // coarse voxel we're looking at.
        //
        CHKOUT(b"ZZPTPL02", ctx)?;
        return Ok(());
    }
    //
    // Look up the pointer to the plate list for this voxel, and if
    // the pointer is non-null, look up the plate count.
    //
    PTRLOC = ((CGRPTR - 1) + PTROFF);

    DSKI02(
        HANDLE,
        DLADSC.as_slice(),
        KWVXPT,
        PTRLOC,
        1,
        &mut N,
        std::slice::from_mut(&mut PLTPTR),
        ctx,
    )?;

    if (FAILED(ctx) || (PLTPTR < 1)) {
        CHKOUT(b"ZZPTPL02", ctx)?;
        return Ok(());
    }

    DSKI02(
        HANDLE,
        DLADSC.as_slice(),
        KWVXPL,
        PLTPTR,
        1,
        &mut N,
        std::slice::from_mut(&mut NPLATE),
        ctx,
    )?;

    if (FAILED(ctx) || (NPLATE < 1)) {
        CHKOUT(b"ZZPTPL02", ctx)?;
        return Ok(());
    }

    //
    // Loop through the plates, keeping track of the minimum plate-point
    // distance.
    //
    DMIN = DPMAX();
    REMAIN = NPLATE;
    NREAD = intrinsics::MIN0(&[REMAIN, BUFSIZ]);
    I = 1;

    while (REMAIN > 0) {
        //
        // Look up the current set of plate IDs.
        //
        DSKI02(
            HANDLE,
            DLADSC.as_slice(),
            KWVXPL,
            (PLTPTR + I),
            NREAD,
            &mut N,
            PLTBUF.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZPTPL02", ctx)?;
            return Ok(());
        }
        //
        // Look up the vertices of each plate in the buffer and find
        // the distance of the point from that plate. Quit if we
        // find a match.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = NREAD;
            let m3__: i32 = 1;
            J = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                PIDTMP = PLTBUF[J];
                START = ((3 * (PIDTMP - 1)) + 1);

                DSKI02(
                    HANDLE,
                    DLADSC.as_slice(),
                    KWPLAT,
                    START,
                    3,
                    &mut N,
                    PLTTMP.as_slice_mut(),
                    ctx,
                )?;

                for K in 1..=3 {
                    VID = PLTTMP[K];
                    START = ((3 * (VID - 1)) + 1);

                    DSKD02(
                        HANDLE,
                        DLADSC.as_slice(),
                        KWVERT,
                        START,
                        3,
                        &mut N,
                        VRTTMP.subarray_mut([1, K]),
                        ctx,
                    )?;
                }

                if FAILED(ctx) {
                    CHKOUT(b"ZZPTPL02", ctx)?;
                    return Ok(());
                }

                //
                // Work with an expanded version of the plate.
                //
                PLTEXP(VRTTMP.as_slice(), XPDFRC, XVERTS.as_slice_mut());

                PLTNRM(
                    XVERTS.subarray([1, 1]),
                    XVERTS.subarray([1, 2]),
                    XVERTS.subarray([1, 3]),
                    NORMAL.as_slice_mut(),
                );

                if FAILED(ctx) {
                    CHKOUT(b"ZZPTPL02", ctx)?;
                    return Ok(());
                }

                VHATIP(NORMAL.as_slice_mut());

                VSUB(
                    POINT.as_slice(),
                    XVERTS.subarray([1, 1]),
                    PNTOFF.as_slice_mut(),
                );

                if (f64::abs(VDOT(PNTOFF.as_slice(), NORMAL.as_slice())) <= save.LIMIT) {
                    //
                    // The input point lies in a narrow region of space
                    // bounded by two planes, both of which are parallel
                    // to the plate. The plate lies between the planes.
                    //
                    // This test does not rule out a comparison between POINT
                    // and a distant plate, if POINT is close to the plane
                    // containing that plate. However, the proportion of such
                    // cases will normally be small.
                    //
                    PLTNP(
                        POINT.as_slice(),
                        XVERTS.subarray([1, 1]),
                        XVERTS.subarray([1, 2]),
                        XVERTS.subarray([1, 3]),
                        PNEAR.as_slice_mut(),
                        &mut DIST,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"ZZPTPL02", ctx)?;
                        return Ok(());
                    }
                } else {
                    DIST = DPMAX();
                }

                if (DIST <= save.LIMIT) {
                    //
                    // We have a reasonable candidate for the closest plate.
                    //
                    *FOUND = true;

                    if (DIST < DMIN) {
                        DMIN = DIST;
                        *PLID = PIDTMP;
                        //
                        // Set the output vertices to the original version.
                        //
                        MOVEI(PLTTMP.as_slice(), 3, PLATE.as_slice_mut());
                        MOVED(VRTTMP.as_slice(), 9, VERTS.as_slice_mut());
                        //
                        // We'll return the above values if we don't find
                        // a better match.
                        //
                    }
                }

                J += m3__;
            }
        }

        //
        // Prepare to read the next set of plate IDs, if any.
        //
        REMAIN = (REMAIN - NREAD);
        I = (I + NREAD);
        NREAD = intrinsics::MIN0(&[REMAIN, BUFSIZ]);
    }

    CHKOUT(b"ZZPTPL02", ctx)?;
    Ok(())
}
