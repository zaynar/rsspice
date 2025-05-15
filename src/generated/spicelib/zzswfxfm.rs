//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const MAXFRM: i32 = 1013;
const MAXBAS: i32 = 15000;
const LBSNGL: i32 = -5;
const CTRSIZ: i32 = 2;
const J2CODE: i32 = 1;
const FPLSIZ: i32 = ((MAXFRM - LBSNGL) + 1);
const D3: i32 = 3;
const D6: i32 = 6;
const SIZE33: i32 = 9;
const SIZE66: i32 = 36;

struct SaveVars {
    STARTS: ActualArray<f64>,
    STOPS: ActualArray<f64>,
    BASBEG: ActualArray<i32>,
    BASCNT: ActualArray<i32>,
    BASLST: ActualArray<i32>,
    CLSIDS: ActualArray<i32>,
    CLSSES: ActualArray<i32>,
    FIDLST: ActualArray<i32>,
    FRPOOL: ActualArray<i32>,
    HDFRAM: ActualArray<i32>,
    BINARY: ActualArray<bool>,
    USETIM: ActualArray<bool>,
    XIDENT: StackArray2D<f64, 36>,
    FREE: i32,
    POLCTR: StackArray<i32, 2>,
    PRVAT: i32,
    PRVFRM: i32,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STARTS = ActualArray::<f64>::new(1..=MAXBAS);
        let mut STOPS = ActualArray::<f64>::new(1..=MAXBAS);
        let mut BASBEG = ActualArray::<i32>::new(1..=MAXFRM);
        let mut BASCNT = ActualArray::<i32>::new(1..=MAXFRM);
        let mut BASLST = ActualArray::<i32>::new(1..=MAXBAS);
        let mut CLSIDS = ActualArray::<i32>::new(1..=MAXBAS);
        let mut CLSSES = ActualArray::<i32>::new(1..=MAXBAS);
        let mut FIDLST = ActualArray::<i32>::new(1..=MAXFRM);
        let mut FRPOOL = ActualArray::<i32>::new(LBSNGL..=MAXFRM);
        let mut HDFRAM = ActualArray::<i32>::new(1..=MAXFRM);
        let mut BINARY = ActualArray::<bool>::new(1..=MAXFRM);
        let mut USETIM = ActualArray::<bool>::new(1..=MAXFRM);
        let mut XIDENT = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
        let mut FREE: i32 = 0;
        let mut POLCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut PRVAT: i32 = 0;
        let mut PRVFRM: i32 = 0;
        let mut PASS1: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), MAXFRM as usize))
                .chain([]);

            BASBEG
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), MAXFRM as usize))
                .chain([]);

            BASCNT
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), MAXBAS as usize))
                .chain([]);

            BASLST
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), MAXBAS as usize))
                .chain([]);

            CLSIDS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), MAXBAS as usize))
                .chain([]);

            CLSSES
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::L(false), MAXFRM as usize))
                .chain([]);

            BINARY
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), MAXFRM as usize))
                .chain([]);

            FIDLST
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FREE = 1;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), FPLSIZ as usize))
                .chain([]);

            FRPOOL
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), MAXFRM as usize))
                .chain([]);

            HDFRAM
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PASS1 = true;
        PRVAT = 0;
        PRVFRM = 0;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), MAXBAS as usize))
                .chain([]);

            STARTS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), MAXBAS as usize))
                .chain([]);

            STOPS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::L(false), MAXFRM as usize))
                .chain([]);

            USETIM
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            STARTS,
            STOPS,
            BASBEG,
            BASCNT,
            BASLST,
            CLSIDS,
            CLSSES,
            FIDLST,
            FRPOOL,
            HDFRAM,
            BINARY,
            USETIM,
            XIDENT,
            FREE,
            POLCTR,
            PRVAT,
            PRVFRM,
            PASS1,
        }
    }
}

//$Procedure ZZSWFXFM ( Private, get switch frame transformation )
pub fn ZZSWFXFM(
    INFRM: i32,
    ET: f64,
    NDIM: i32,
    XFORM: &mut [f64],
    OUTFRM: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut XFORM = DummyArrayMut2D::new(XFORM, 1..=NDIM, 1..=NDIM);
    let mut RMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TSIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut BASIDX: i32 = 0;
    let mut FRAMAT: i32 = 0;
    let mut HEAD: i32 = 0;
    let mut LB: i32 = 0;
    let mut LSTBEG: i32 = 0;
    let mut SAMFRM: bool = false;
    let mut TIMEOK: bool = false;
    let mut UPDATE: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // ID code of the J2000 frame:
    //

    //
    // Size of linked list pool:
    //
    // The add-only hash pool for frame IDs is singly linked.
    //

    //
    // Dimensions
    //

    //
    // Matrix sizes.
    //

    //
    // Local variables
    //

    //
    // Overview of frame storage data structures
    // =========================================
    //
    // This routine uses an add-only hash to index switch frames
    // known to this routine. The hash maps input frame ID codes to
    // head nodes of collision lists stored in the array HDFRAM.
    // The collision lists are stored in the singly linked list pool
    // FRPOOL.
    //
    // Each node of a collision list is the index in the array FIDLST of
    // a switch frame ID. The array FIDLST has the following associated,
    // parallel arrays:
    //
    //    BASBEG       Start indices in the base frame attribute arrays
    //                 of values associated with specific switch frames.
    //                 For example, BASLST( BASBEG(FRAMAT) ) is the
    //                 index of the first base frame associated with
    //                 the switch frame having ID FIDLST(FRAMAT).
    //
    //    BASCNT       Base count for each switch frame
    //
    //    USETIM       Indicates whether time intervals are associated
    //                 with that frame's base frames
    //
    //    BINARY       Indicates whether time intervals are eligible for
    //                 binary search. To be eligible, the intervals
    //                 must have start times in ascending order and
    //                 must be disjoint or have singleton overlap.
    //
    // Base frame attributes are stored in a set of parallel arrays.
    // Attributes for bases of a given switch frame are listed in
    // contiguous elements. The arrays and their contents are:
    //
    //    BASLST       Base frame ID codes. The base frames of a given
    //                 switch frame are listed in ascending priority
    //                 order in consecutive elements of BASLST.
    //
    //    STARTS       Base frame interval start times, expressed as
    //                 seconds past J2000 TDB. Valid for all bases of
    //                 a given switch frame, or unused for that frame.
    //
    //    STOPS        Base frame interval start times, expressed as
    //                 seconds past J2000 TDB. Valid for all bases of
    //                 a given switch frame, or unused for that frame.
    //
    //    CLSSES       Base frame classes.
    //
    //    CLSIDS       Base frame class IDs.
    //
    // Each base frame list for a given switch frame is ordered
    // according to ascending priority.
    //

    //
    // Other declarations
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

    CHKIN(b"ZZSWFXFM", ctx)?;

    //
    // No frame found yet.
    //
    *FOUND = false;

    if save.PASS1 {
        //
        // Initialize pool data structures.
        //
        ZZSWFINI(
            save.HDFRAM.as_slice_mut(),
            save.FRPOOL.as_slice_mut(),
            save.FIDLST.as_slice_mut(),
            save.BASBEG.as_slice_mut(),
            &mut save.FREE,
            &mut save.PRVAT,
            &mut save.PRVFRM,
            &mut SAMFRM,
            ctx,
        )?;
        //
        // Initialize the local pool counter.
        //
        ZZCTRUIN(save.POLCTR.as_slice_mut(), ctx);
        //
        // Note: no FAILED() check is needed after the two calls above.
        //
        // Create a 6x6 identity matrix.
        //
        FILLD(0.0, SIZE66, save.XIDENT.as_slice_mut());

        for I in 1..=6 {
            save.XIDENT[[I, I]] = 1.0;
        }

        save.PASS1 = false;
    }

    //
    // Check the transformation dimension.
    //
    if ((NDIM != D3) && (NDIM != D6)) {
        SETMSG(b"Transformation dimension must be 3 or 6 but was #.", ctx);
        ERRINT(b"#", NDIM, ctx);
        SIGERR(b"SPICE(BADDIMENSION)", ctx)?;
        CHKOUT(b"ZZSWFXFM", ctx)?;
        return Ok(());
    }

    //
    // Reinitialize local database if the kernel pool has been
    // updated. The call below syncs POLCTR.
    //
    ZZPCTRCK(save.POLCTR.as_slice_mut(), &mut UPDATE, ctx);

    if UPDATE {
        //
        // Initialize local data structures.
        //
        ZZSWFINI(
            save.HDFRAM.as_slice_mut(),
            save.FRPOOL.as_slice_mut(),
            save.FIDLST.as_slice_mut(),
            save.BASBEG.as_slice_mut(),
            &mut save.FREE,
            &mut save.PRVAT,
            &mut save.PRVFRM,
            &mut SAMFRM,
            ctx,
        )?;

    //
    // SAMFRM is set to .FALSE. and PRVFRM is set to 0 at this point.
    //
    } else {
        //
        // SAMFRM indicates that INFRM is the same switch frame we saw on
        // the previous call, no kernel pool update has occurred since
        // that call, and that the call was successful. We don't save the
        // previous switch frame information for unsuccessful calls: for
        // those, we set PRVFRM to zero so that it won't match any valid
        // frame ID.
        //
        SAMFRM = ((INFRM != 0) && (INFRM == save.PRVFRM));
    }

    if SAMFRM {
        //
        // We can use information saved from the previous call. FRAMAT is
        // the index of the switch frame ID in the switch frame ID array.
        //
        FRAMAT = save.PRVAT;
    } else {
        //
        // Look up the input frame.
        //
        ZZHSICHK(
            save.HDFRAM.as_slice(),
            save.FRPOOL.as_slice(),
            save.FIDLST.as_slice(),
            INFRM,
            &mut FRAMAT,
            ctx,
        )?;
    }

    if (FRAMAT == 0) {
        //
        // This frame is unknown to the switch frame subsystem. Try to
        // look up the frame's specification from the kernel pool.
        // ZZSWFFET checks and returns the frame specification.
        //
        ZZSWFFET(
            INFRM,
            save.HDFRAM.as_slice_mut(),
            save.FRPOOL.as_slice_mut(),
            save.FIDLST.as_slice_mut(),
            save.BASBEG.as_slice_mut(),
            &mut save.FREE,
            save.BASCNT.as_slice_mut(),
            save.USETIM.as_slice_mut(),
            save.BINARY.as_slice_mut(),
            save.CLSSES.as_slice_mut(),
            save.CLSIDS.as_slice_mut(),
            save.BASLST.as_slice_mut(),
            save.STARTS.as_slice_mut(),
            save.STOPS.as_slice_mut(),
            &mut FRAMAT,
            ctx,
        )?;

        if (FAILED(ctx) || (FRAMAT == 0)) {
            //
            // We were unable to obtain information for the requested
            // frame. Initialize all information about the previous frame.
            // For safety, initialize the local database.
            //
            ZZSWFINI(
                save.HDFRAM.as_slice_mut(),
                save.FRPOOL.as_slice_mut(),
                save.FIDLST.as_slice_mut(),
                save.BASBEG.as_slice_mut(),
                &mut save.FREE,
                &mut save.PRVAT,
                &mut save.PRVFRM,
                &mut SAMFRM,
                ctx,
            )?;

            CHKOUT(b"ZZSWFXFM", ctx)?;
            return Ok(());
        }
    }

    //
    // At this point, FRAMAT is set.
    //
    // Below, HEAD is the common index in each parallel array of base
    // frame data of the first item for a given switch frame. BASIDX is
    // the current index. The index obtained by adding a switch frame's
    // base frame count to HEAD is the successor of the last index of
    // base frame data for that switch frame.
    //
    HEAD = save.BASBEG[FRAMAT];

    //
    // At this point, the frame's information is present in the
    // local database. FRAMAT is the index of the frame in the hash
    // data structure. HEAD is the start index of the base frame data
    // for the input switch frame.
    //
    // Search for the highest-priority frame that provides data
    // for the request time.
    //
    // For safety, reset FOUND.
    //
    *FOUND = false;

    if save.BINARY[FRAMAT] {
        //
        // We'll do a binary search. Suppress follow-on linear searching
        // and frame data lookup by default: when BASIDX is zero, the
        // linear search and data fetch loop won't execute. We'll update
        // BASIDX if an interval is found.
        //
        BASIDX = 0;

        //
        // Find the last interval, if any, that contains ET. Start by
        // finding the last start time that's less than or equal to ET.
        //
        // Here we rely on the assumption that start and stop times for
        // each frame are stored in increasing time order.
        //
        LSTBEG = LSTLED(ET, save.BASCNT[FRAMAT], save.STARTS.subarray(HEAD));

        //
        // If LSTBEG is zero, then ET is strictly less than all of the
        // start times, in which case there's no match.
        //
        if (LSTBEG > 0) {
            //
            // ET could belong to the interval having start time stored at
            // relative index LSTBEG, which is equivalent to the base
            // frame array index HEAD+LSTBEG-1.
            //
            if (ET <= save.STOPS[((HEAD + LSTBEG) - 1)]) {
                //
                // This interval contains ET. Prepare to fetch data for
                // the base frame corresponding to this interval.
                //
                BASIDX = ((HEAD + LSTBEG) - 1);
            }
        }
    } else {
        //
        // Prepare for a linear search and fetching base frame data.
        //
        BASIDX = ((HEAD + save.BASCNT[FRAMAT]) - 1);
    }

    //
    // Search the base list in order of decreasing priority. The search
    // will terminate once we've examined the base frame at index LB,
    // if not before.
    //
    LB = HEAD;

    while ((BASIDX >= LB) && !*FOUND) {
        if save.USETIM[FRAMAT] {
            TIMEOK = ((ET >= save.STARTS[BASIDX]) && (ET <= save.STOPS[BASIDX]));
        } else {
            TIMEOK = false;
        }

        if (TIMEOK || !save.USETIM[FRAMAT]) {
            //
            // This base frame is applicable.
            //
            if (save.CLSSES[BASIDX] == CK) {
                //
                // We can't know whether there are CK data for the request
                // time without looking up the data, so that's what we do.
                // If we find data, we return the transformation to the CK
                // frame's base frame and the frame ID of the CK frame's
                // base frame.
                //
                if (NDIM == D6) {
                    CKFXFM(
                        save.CLSIDS[BASIDX],
                        ET,
                        XFORM.as_slice_mut(),
                        OUTFRM,
                        FOUND,
                        ctx,
                    )?;
                } else {
                    CKFROT(
                        save.CLSIDS[BASIDX],
                        ET,
                        XFORM.as_slice_mut(),
                        OUTFRM,
                        FOUND,
                        ctx,
                    )?;
                }

                if FAILED(ctx) {
                    ZZSWFINI(
                        save.HDFRAM.as_slice_mut(),
                        save.FRPOOL.as_slice_mut(),
                        save.FIDLST.as_slice_mut(),
                        save.BASBEG.as_slice_mut(),
                        &mut save.FREE,
                        &mut save.PRVAT,
                        &mut save.PRVFRM,
                        &mut SAMFRM,
                        ctx,
                    )?;

                    CHKOUT(b"ZZSWFXFM", ctx)?;
                    return Ok(());
                }

                //
                // If we got this far and we're in binary search mode, we
                // won't find a result, except possibly when ET is equal to
                // the interval start time.
                //
                if (save.BINARY[FRAMAT] && !*FOUND) {
                    if (ET > save.STARTS[BASIDX]) {
                        //
                        // The next lowest-indexed interval doesn't cover ET,
                        // so no data can be found to satisfy this request.
                        //
                        save.PRVFRM = 0;
                        save.PRVAT = 0;

                        CHKOUT(b"ZZSWFXFM", ctx)?;
                        return Ok(());
                    }
                }
            //
            // FOUND is set. If we did find data, XFORM and OUTFRM
            // are set. If FOUND is .FALSE., we'll look at the
            // remaining lower-priority bases, if any, if we're not in
            // binary search mode. If we're in binary search mode,
            // we'll look at the next lower-priority base only if ET is
            // equal to the interval start time.
            //
            } else if (save.CLSSES[BASIDX] == TK) {
                TKFRAM(save.CLSIDS[BASIDX], RMAT.as_slice_mut(), OUTFRM, FOUND, ctx)?;

                if FAILED(ctx) {
                    *FOUND = false;

                    ZZSWFINI(
                        save.HDFRAM.as_slice_mut(),
                        save.FRPOOL.as_slice_mut(),
                        save.FIDLST.as_slice_mut(),
                        save.BASBEG.as_slice_mut(),
                        &mut save.FREE,
                        &mut save.PRVAT,
                        &mut save.PRVFRM,
                        &mut SAMFRM,
                        ctx,
                    )?;

                    CHKOUT(b"ZZSWFXFM", ctx)?;
                    return Ok(());
                }

                if (NDIM == D6) {
                    for I in 1..=3 {
                        for J in 1..=3 {
                            XFORM[[I, J]] = RMAT[[I, J]];
                            XFORM[[(I + 3), (J + 3)]] = RMAT[[I, J]];
                            XFORM[[(I + 3), J]] = 0.0;
                            XFORM[[I, (J + 3)]] = 0.0;
                        }
                    }
                } else {
                    MOVED(RMAT.as_slice(), SIZE33, XFORM.as_slice_mut());
                }
            //
            // OUTFRM and FOUND have been set by TKFRAM.
            //
            } else if (save.CLSSES[BASIDX] == PCK) {
                if (NDIM == D6) {
                    TISBOD(b"J2000", save.CLSIDS[BASIDX], ET, TSIPM.as_slice_mut(), ctx)?;
                    INVSTM(TSIPM.as_slice(), XFORM.as_slice_mut(), ctx)?;
                } else {
                    TIPBOD(b"J2000", save.CLSIDS[BASIDX], ET, RMAT.as_slice_mut(), ctx)?;
                    XPOSE(RMAT.as_slice(), XFORM.as_slice_mut());
                }

                if FAILED(ctx) {
                    *FOUND = false;

                    ZZSWFINI(
                        save.HDFRAM.as_slice_mut(),
                        save.FRPOOL.as_slice_mut(),
                        save.FIDLST.as_slice_mut(),
                        save.BASBEG.as_slice_mut(),
                        &mut save.FREE,
                        &mut save.PRVAT,
                        &mut save.PRVFRM,
                        &mut SAMFRM,
                        ctx,
                    )?;

                    CHKOUT(b"ZZSWFXFM", ctx)?;
                    return Ok(());
                }

                *OUTFRM = J2CODE;
                *FOUND = true;
            } else if (save.CLSSES[BASIDX] == INERTL) {
                IRFROT(save.BASLST[BASIDX], J2CODE, RMAT.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    *FOUND = false;

                    ZZSWFINI(
                        save.HDFRAM.as_slice_mut(),
                        save.FRPOOL.as_slice_mut(),
                        save.FIDLST.as_slice_mut(),
                        save.BASBEG.as_slice_mut(),
                        &mut save.FREE,
                        &mut save.PRVAT,
                        &mut save.PRVFRM,
                        &mut SAMFRM,
                        ctx,
                    )?;

                    CHKOUT(b"ZZSWFXFM", ctx)?;
                    return Ok(());
                }

                if (NDIM == D6) {
                    for I in 1..=3 {
                        for J in 1..=3 {
                            XFORM[[I, J]] = RMAT[[I, J]];
                            XFORM[[(I + 3), (J + 3)]] = RMAT[[I, J]];
                            XFORM[[(I + 3), J]] = 0.0;
                            XFORM[[I, (J + 3)]] = 0.0;
                        }
                    }
                } else {
                    MOVED(RMAT.as_slice(), SIZE33, XFORM.as_slice_mut());
                }

                *OUTFRM = J2CODE;
                *FOUND = true;
            } else {
                //
                // For other non-CK frames---switch and dynamic---we just
                // return the base frame and the identity matrix.
                //
                if (NDIM == D6) {
                    MOVED(save.XIDENT.as_slice(), SIZE66, XFORM.as_slice_mut());
                } else {
                    IDENT(XFORM.as_slice_mut());
                }

                *OUTFRM = save.BASLST[BASIDX];
                *FOUND = true;
            }
        }

        //
        // If we got this far and we're in binary search mode,
        // we've already found a result. We'll exit the loop at
        // the next loop condition test.
        //
        // Check the next lower priority base frame and interval.
        //
        BASIDX = (BASIDX - 1);
    }

    //
    // FOUND is set.
    //
    // We either found data, or we ran out of base frames to try.
    // If we found data, the outputs XFORM and OUTFRM are set.
    //
    if *FOUND {
        //
        // Save values for re-use.
        //
        save.PRVFRM = INFRM;
        save.PRVAT = FRAMAT;
    } else {
        //
        // Ensure the saved values won't be re-used.
        //
        save.PRVFRM = 0;
    }

    CHKOUT(b"ZZSWFXFM", ctx)?;
    Ok(())
}
