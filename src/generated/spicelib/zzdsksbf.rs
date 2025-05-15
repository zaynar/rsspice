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
const CTRSIZ: i32 = 2;
const MAXDC: i32 = 1;
const MAXIC: i32 = 1;
const STSIZE: i32 = 10000;
const BTSIZE: i32 = 10;

struct SaveVars {
    BTNBOD: i32,
    BTBODY: StackArray<i32, 10>,
    BTSEGP: StackArray<i32, 10>,
    BTSTSZ: StackArray<i32, 10>,
    STHAN: ActualArray<i32>,
    STDSCR: ActualArray2D<f64>,
    STDLAD: ActualArray2D<i32>,
    STFREE: i32,
    STOFF: ActualArray2D<f64>,
    STCTR: ActualArray2D<f64>,
    STRAD: ActualArray<f64>,
    BSRCTR: StackArray<i32, 2>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BTNBOD: i32 = 0;
        let mut BTBODY = StackArray::<i32, 10>::new(1..=BTSIZE);
        let mut BTSEGP = StackArray::<i32, 10>::new(1..=BTSIZE);
        let mut BTSTSZ = StackArray::<i32, 10>::new(1..=BTSIZE);
        let mut STHAN = ActualArray::<i32>::new(1..=STSIZE);
        let mut STDSCR = ActualArray2D::<f64>::new(1..=DSKDSZ, 1..=STSIZE);
        let mut STDLAD = ActualArray2D::<i32>::new(1..=DLADSZ, 1..=STSIZE);
        let mut STFREE: i32 = 0;
        let mut STOFF = ActualArray2D::<f64>::new(1..=3, 1..=STSIZE);
        let mut STCTR = ActualArray2D::<f64>::new(1..=3, 1..=STSIZE);
        let mut STRAD = ActualArray::<f64>::new(1..=STSIZE);
        let mut BSRCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut FIRST: bool = false;

        BTNBOD = 0;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), BTSIZE as usize))
                .chain([]);

            BTBODY
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), CTRSIZ as usize))
                .chain([]);

            BSRCTR
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;
        STFREE = 1;

        Self {
            BTNBOD,
            BTBODY,
            BTSEGP,
            BTSTSZ,
            STHAN,
            STDSCR,
            STDLAD,
            STFREE,
            STOFF,
            STCTR,
            STRAD,
            BSRCTR,
            FIRST,
        }
    }
}

//$Procedure ZZDSKSBF ( DSK, manage the API segment buffer )
pub fn ZZDSKSBF(
    BODYID: i32,
    NSURF: i32,
    SRFLST: &[i32],
    ET: f64,
    FIXFID: i32,
    VERTEX: &[f64],
    RAYDIR: &[f64],
    POINT: &[f64],
    XPT: &[f64],
    HANDLE: i32,
    DLADSC: &[i32],
    DSKDSC: &[f64],
    DC: &[f64],
    IC: &[i32],
    FOUND: bool,
    NORMAL: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Sizes of source info arrays returned by
    // ZZDSKBUX:
    //

    //
    // Body table variables
    // --------------------
    //
    //    BTNBOD  is the number of bodies in the body table.
    //
    //    BTBODY  is an array of body ID codes.
    //
    //    BTSEGP  is an array of pointers (start indices) to entries in
    //            the segment table. The Ith pointer indicates the start
    //            index for entries for the Ith body.
    //
    //    BTSTSZ  is an array of segment table entry counts. The Ith
    //            element of BTSTSZ is the number of entries in the
    //            segment table for the Ith body.
    //
    //

    //
    // Segment table variables
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

    CHKIN(b"ZZDSKSBF", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZDSKSBF", ctx)?;
    Ok(())
}

//$Procedure ZZSBFXR ( DSK, prepare and perform unprioritized intercept )
pub fn ZZSBFXR(
    BODYID: i32,
    NSURF: i32,
    SRFLST: &[i32],
    ET: f64,
    FIXFID: i32,
    VERTEX: &[f64],
    RAYDIR: &[f64],
    XPT: &mut [f64],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SRFLST = DummyArray::new(SRFLST, 1..);
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut LOCDC = StackArray::<f64, 1>::new(1..=MAXDC);
    let mut BIX: i32 = 0;
    let mut J: i32 = 0;
    let mut LOCIC = StackArray::<i32, 1>::new(1..=MAXIC);
    let mut NSEG: i32 = 0;
    let mut SEGIDX: i32 = 0;
    let mut UPDATE: bool = false;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSBFXR", ctx)?;

    if save.FIRST {
        //
        // Initialize BSR counter.
        //
        ZZCTRUIN(save.BSRCTR.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // See whether the state of the loaded DSK set has changed
    // since the last call.
    //
    ZZDSKCHK(save.BSRCTR.as_slice_mut(), &mut UPDATE, ctx)?;

    if UPDATE {
        //
        // Make sure the ZZDSKBSR subsystem has completed the segment
        // list for the input body since the last time the BSR loaded
        // kernel state changed.
        //
        ZZDSKBBL(BODYID, ctx)?;

        //
        // Initialize the local buffers. We restart from scratch
        // each time the BSR loaded kernel state changes.
        //
        ZZDSKSBI(
            BTSIZE,
            STSIZE,
            save.BTBODY.as_slice_mut(),
            &mut save.BTNBOD,
            save.BTSEGP.as_slice_mut(),
            save.BTSTSZ.as_slice_mut(),
            save.STHAN.as_slice_mut(),
            save.STDSCR.as_slice_mut(),
            save.STDLAD.as_slice_mut(),
            &mut save.STFREE,
            save.STOFF.as_slice_mut(),
            save.STCTR.as_slice_mut(),
            save.STRAD.as_slice_mut(),
            ctx,
        )?;
    }

    if FAILED(ctx) {
        CHKOUT(b"ZZSBFXR", ctx)?;
        return Ok(());
    }

    //
    // Find the index of the input body ID in the body table. If
    // we re-initialized the tables, the index will be zero.
    //
    BIX = ISRCHI(BODYID, save.BTNBOD, save.BTBODY.as_slice());

    if (BIX == 0) {
        //
        // We don't have buffered information for this body. Update
        // the body and segment tables to store data for it.
        //
        ZZDSKSBA(
            BODYID,
            BTSIZE,
            STSIZE,
            save.BTBODY.as_slice_mut(),
            &mut save.BTNBOD,
            save.BTSEGP.as_slice_mut(),
            save.BTSTSZ.as_slice_mut(),
            save.STHAN.as_slice_mut(),
            save.STDSCR.as_slice_mut(),
            save.STDLAD.as_slice_mut(),
            &mut save.STFREE,
            save.STOFF.as_slice_mut(),
            save.STCTR.as_slice_mut(),
            save.STRAD.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSBFXR", ctx)?;
            return Ok(());
        }
        //
        // The new body's position in the body table is at the end.
        //
        BIX = save.BTNBOD;
    }

    //
    // Find the ray-surface intercept, using the buffered segment
    // data.
    //
    J = save.BTSEGP[BIX];
    NSEG = save.BTSTSZ[BIX];

    ZZDSKBUX(
        BODYID,
        NSURF,
        SRFLST.as_slice(),
        ET,
        FIXFID,
        NSEG,
        save.STHAN.subarray(J),
        save.STDLAD.subarray([1, J]),
        save.STDSCR.subarray([1, J]),
        save.STOFF.subarray([1, J]),
        save.STCTR.subarray([1, J]),
        save.STRAD.subarray(J),
        VERTEX.as_slice(),
        RAYDIR.as_slice(),
        XPT.as_slice_mut(),
        &mut SEGIDX,
        LOCDC.as_slice_mut(),
        LOCIC.as_slice_mut(),
        FOUND,
        ctx,
    )?;

    CHKOUT(b"ZZSBFXR", ctx)?;
    Ok(())
}

//$Procedure ZZSBFXRI ( DSK, unprioritized intercept with info )
pub fn ZZSBFXRI(
    BODYID: i32,
    NSURF: i32,
    SRFLST: &[i32],
    ET: f64,
    FIXFID: i32,
    VERTEX: &[f64],
    RAYDIR: &[f64],
    XPT: &mut [f64],
    HANDLE: &mut i32,
    DLADSC: &mut [i32],
    DSKDSC: &mut [f64],
    DC: &mut [f64],
    IC: &mut [i32],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SRFLST = DummyArray::new(SRFLST, 1..);
    let VERTEX = DummyArray::new(VERTEX, 1..=3);
    let RAYDIR = DummyArray::new(RAYDIR, 1..=3);
    let mut XPT = DummyArrayMut::new(XPT, 1..=3);
    let mut DLADSC = DummyArrayMut::new(DLADSC, 1..);
    let mut DSKDSC = DummyArrayMut::new(DSKDSC, 1..);
    let mut DC = DummyArrayMut::new(DC, 1..);
    let mut IC = DummyArrayMut::new(IC, 1..);
    let mut BIX: i32 = 0;
    let mut J: i32 = 0;
    let mut NSEG: i32 = 0;
    let mut SEGIDX: i32 = 0;
    let mut UPDATE: bool = false;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSBFXRI", ctx)?;

    if save.FIRST {
        //
        // Initialize BSR counter.
        //
        ZZCTRUIN(save.BSRCTR.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // See whether the state of the loaded DSK set has changed
    // since the last call.
    //
    ZZDSKCHK(save.BSRCTR.as_slice_mut(), &mut UPDATE, ctx)?;

    if UPDATE {
        //
        // Make sure the ZZDSKBSR subsystem has completed the segment
        // list for the input body since the last time the BSR loaded
        // kernel state changed.
        //
        ZZDSKBBL(BODYID, ctx)?;

        //
        // Initialize the local buffers. We restart from scratch
        // each time the BSR loaded kernel state changes.
        //
        ZZDSKSBI(
            BTSIZE,
            STSIZE,
            save.BTBODY.as_slice_mut(),
            &mut save.BTNBOD,
            save.BTSEGP.as_slice_mut(),
            save.BTSTSZ.as_slice_mut(),
            save.STHAN.as_slice_mut(),
            save.STDSCR.as_slice_mut(),
            save.STDLAD.as_slice_mut(),
            &mut save.STFREE,
            save.STOFF.as_slice_mut(),
            save.STCTR.as_slice_mut(),
            save.STRAD.as_slice_mut(),
            ctx,
        )?;
    }

    if FAILED(ctx) {
        CHKOUT(b"ZZSBFXRI", ctx)?;
        return Ok(());
    }

    //
    // Find the index of the input body ID in the body table. If
    // we re-initialized the tables, the index will be zero.
    //
    BIX = ISRCHI(BODYID, save.BTNBOD, save.BTBODY.as_slice());

    if (BIX == 0) {
        //
        // We don't have buffered information for this body. Update
        // the body and segment tables to store data for it.
        //
        ZZDSKSBA(
            BODYID,
            BTSIZE,
            STSIZE,
            save.BTBODY.as_slice_mut(),
            &mut save.BTNBOD,
            save.BTSEGP.as_slice_mut(),
            save.BTSTSZ.as_slice_mut(),
            save.STHAN.as_slice_mut(),
            save.STDSCR.as_slice_mut(),
            save.STDLAD.as_slice_mut(),
            &mut save.STFREE,
            save.STOFF.as_slice_mut(),
            save.STCTR.as_slice_mut(),
            save.STRAD.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSBFXRI", ctx)?;
            return Ok(());
        }
        //
        // The new body's position in the body table is at the end.
        //
        BIX = save.BTNBOD;
    }

    //
    // Find the ray-surface intercept, using the buffered segment
    // data.
    //
    J = save.BTSEGP[BIX];
    NSEG = save.BTSTSZ[BIX];

    ZZDSKBUX(
        BODYID,
        NSURF,
        SRFLST.as_slice(),
        ET,
        FIXFID,
        NSEG,
        save.STHAN.subarray(J),
        save.STDLAD.subarray([1, J]),
        save.STDSCR.subarray([1, J]),
        save.STOFF.subarray([1, J]),
        save.STCTR.subarray([1, J]),
        save.STRAD.subarray(J),
        VERTEX.as_slice(),
        RAYDIR.as_slice(),
        XPT.as_slice_mut(),
        &mut SEGIDX,
        DC.as_slice_mut(),
        IC.as_slice_mut(),
        FOUND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZSBFXRI", ctx)?;
        return Ok(());
    }

    if *FOUND {
        //
        // Adjust the segment index to make it relative to the base value
        // 1, instead of the current base J.
        //
        SEGIDX = ((SEGIDX + J) - 1);

        *HANDLE = save.STHAN[SEGIDX];

        MOVEI(
            save.STDLAD.subarray([1, SEGIDX]),
            DLADSZ,
            DLADSC.as_slice_mut(),
        );
        MOVED(
            save.STDSCR.subarray([1, SEGIDX]),
            DSKDSZ,
            DSKDSC.as_slice_mut(),
        );
    }

    CHKOUT(b"ZZSBFXRI", ctx)?;
    Ok(())
}

//$Procedure ZZSBFNRM ( DSK, prepare and compute unprioritized normal )
pub fn ZZSBFNRM(
    BODYID: i32,
    NSURF: i32,
    SRFLST: &[i32],
    ET: f64,
    FIXFID: i32,
    POINT: &[f64],
    NORMAL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SRFLST = DummyArray::new(SRFLST, 1..);
    let POINT = DummyArray::new(POINT, 1..=3);
    let mut NORMAL = DummyArrayMut::new(NORMAL, 1..=3);
    let mut BIX: i32 = 0;
    let mut J: i32 = 0;
    let mut NSEG: i32 = 0;
    let mut UPDATE: bool = false;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSBFNRM", ctx)?;

    //
    // The following block of code is straight cut-and-paste from
    // ZZSBFXR. (We need to consider packaging this code in a
    // utility routine.)
    //
    if save.FIRST {
        //
        // Initialize BSR counter.
        //
        ZZCTRUIN(save.BSRCTR.as_slice_mut(), ctx);

        save.FIRST = false;
    }

    //
    // See whether the state of the loaded DSK set has changed
    // since the last call.
    //
    ZZDSKCHK(save.BSRCTR.as_slice_mut(), &mut UPDATE, ctx)?;

    if UPDATE {
        //
        // Make sure the ZZDSKBSR subsystem has completed the segment
        // list for the input body since the last time the BSR loaded
        // kernel state changed.
        //
        ZZDSKBBL(BODYID, ctx)?;

        //
        // Initialize the local buffers. We restart from scratch
        // each time the BSR loaded kernel state changes.
        //
        ZZDSKSBI(
            BTSIZE,
            STSIZE,
            save.BTBODY.as_slice_mut(),
            &mut save.BTNBOD,
            save.BTSEGP.as_slice_mut(),
            save.BTSTSZ.as_slice_mut(),
            save.STHAN.as_slice_mut(),
            save.STDSCR.as_slice_mut(),
            save.STDLAD.as_slice_mut(),
            &mut save.STFREE,
            save.STOFF.as_slice_mut(),
            save.STCTR.as_slice_mut(),
            save.STRAD.as_slice_mut(),
            ctx,
        )?;
    }

    if FAILED(ctx) {
        CHKOUT(b"ZZSBFNRM", ctx)?;
        return Ok(());
    }

    //
    // Find the index of the input body ID in the body table. If
    // we re-initialized the tables, the index will be zero.
    //
    BIX = ISRCHI(BODYID, save.BTNBOD, save.BTBODY.as_slice());

    if (BIX == 0) {
        //
        // We don't have buffered information for this body. Update
        // the body and segment tables to store data for it.
        //
        ZZDSKSBA(
            BODYID,
            BTSIZE,
            STSIZE,
            save.BTBODY.as_slice_mut(),
            &mut save.BTNBOD,
            save.BTSEGP.as_slice_mut(),
            save.BTSTSZ.as_slice_mut(),
            save.STHAN.as_slice_mut(),
            save.STDSCR.as_slice_mut(),
            save.STDLAD.as_slice_mut(),
            &mut save.STFREE,
            save.STOFF.as_slice_mut(),
            save.STCTR.as_slice_mut(),
            save.STRAD.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZSBFNRM", ctx)?;
            return Ok(());
        }
        //
        // The new body's position in the body table is at the end.
        //
        BIX = save.BTNBOD;
    }

    //
    // Find the outward unit normal vector, using the buffered segment
    // data.
    //
    J = save.BTSEGP[BIX];
    NSEG = save.BTSTSZ[BIX];

    ZZDSKBUN(
        BODYID,
        NSURF,
        SRFLST.as_slice(),
        ET,
        FIXFID,
        NSEG,
        save.STHAN.subarray(J),
        save.STDLAD.subarray([1, J]),
        save.STDSCR.subarray([1, J]),
        save.STOFF.subarray([1, J]),
        save.STCTR.subarray([1, J]),
        save.STRAD.subarray(J),
        POINT.as_slice(),
        NORMAL.as_slice_mut(),
        ctx,
    )?;

    CHKOUT(b"ZZSBFNRM", ctx)?;
    Ok(())
}
