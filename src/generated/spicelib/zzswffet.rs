//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXFRM: i32 = 1013;
const MAXBAS: i32 = 15000;
const LBSNGL: i32 = -5;
const TMALGN: &[u8] = b"FRAME_#_ALIGNED_WITH";
const TMCENT: &[u8] = b"FRAME_#_CENTER";
const TMCLAS: &[u8] = b"FRAME_#_CLASS";
const TMCLID: &[u8] = b"FRAME_#_CLASS_ID";
const TMID: &[u8] = b"FRAME_#";
const TMFNAM: &[u8] = b"FRAME_#_NAME";
const TMSTRT: &[u8] = b"FRAME_#_START";
const TMSTOP: &[u8] = b"FRAME_#_STOP";
const FRNMLN: i32 = 32;
const TIMLEN: i32 = 80;
const IXFNAM: i32 = 1;
const IXID: i32 = (IXFNAM + 1);
const IXCENT: i32 = (IXID + 1);
const IXCLAS: i32 = (IXCENT + 1);
const IXCLID: i32 = (IXCLAS + 1);
const IXALGN: i32 = (IXCLID + 1);
const IXSTRT: i32 = (IXALGN + 1);
const IXSTOP: i32 = (IXSTRT + 1);
const KVNMLN: i32 = 32;
const NKV: i32 = 8;

//$Procedure ZZSWFFET ( Private, switch frame kernel pool data fetch )
pub fn ZZSWFFET(
    FRAMID: i32,
    HDFRAM: &mut [i32],
    FRPOOL: &mut [i32],
    FIDLST: &mut [i32],
    BASBEG: &mut [i32],
    FREE: &mut i32,
    BASCNT: &mut [i32],
    USETIM: &mut [bool],
    BINARY: &mut [bool],
    CLSSES: &mut [i32],
    CLSIDS: &mut [i32],
    BASLST: &mut [i32],
    STARTS: &mut [f64],
    STOPS: &mut [f64],
    FRAMAT: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut HDFRAM = DummyArrayMut::new(HDFRAM, 1..);
    let mut FRPOOL = DummyArrayMut::new(FRPOOL, LBSNGL..);
    let mut FIDLST = DummyArrayMut::new(FIDLST, 1..);
    let mut BASBEG = DummyArrayMut::new(BASBEG, 1..);
    let mut BASCNT = DummyArrayMut::new(BASCNT, 1..);
    let mut USETIM = DummyArrayMut::new(USETIM, 1..);
    let mut BINARY = DummyArrayMut::new(BINARY, 1..);
    let mut CLSSES = DummyArrayMut::new(CLSSES, 1..);
    let mut CLSIDS = DummyArrayMut::new(CLSIDS, 1..);
    let mut BASLST = DummyArrayMut::new(BASLST, 1..);
    let mut STARTS = DummyArrayMut::new(STARTS, 1..);
    let mut STOPS = DummyArrayMut::new(STOPS, 1..);
    let mut BASNAM = [b' '; FRNMLN as usize];
    let mut BASTYP = [b' '; 1 as usize];
    let mut FRNAME = [b' '; FRNMLN as usize];
    let mut KVALGN = [b' '; KVNMLN as usize];
    let mut KVCLID = [b' '; KVNMLN as usize];
    let mut KVCENT = [b' '; KVNMLN as usize];
    let mut KVCLAS = [b' '; KVNMLN as usize];
    let mut KVID = [b' '; KVNMLN as usize];
    let mut KVFNAM = [b' '; KVNMLN as usize];
    let mut KVNAMS = ActualCharArray::new(KVNMLN, 1..=NKV);
    let mut KVSTRT = [b' '; KVNMLN as usize];
    let mut KVSTOP = [b' '; KVNMLN as usize];
    let mut STPTYP = [b' '; 1 as usize];
    let mut STRTYP = [b' '; 1 as usize];
    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut CENTER: i32 = 0;
    let mut FID: i32 = 0;
    let mut FRCENT: i32 = 0;
    let mut FRCLAS: i32 = 0;
    let mut FRCLID: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut N: i32 = 0;
    let mut NBASES: i32 = 0;
    let mut NFRAVL: i32 = 0;
    let mut NSTART: i32 = 0;
    let mut NSTOP: i32 = 0;
    let mut ROOM: i32 = 0;
    let mut HAVTIM: bool = false;
    let mut INFFND: bool = false;
    let mut KVFND = StackArray::<bool, 8>::new(1..=NKV);
    let mut NEW: bool = false;
    let mut STRXST: bool = false;
    let mut STPXST: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Kernel variable name templates:
    //

    //
    // Other parameters
    //

    //
    // Indices of items in the arrays of kernel variables:
    //

    //
    // Local variables
    //

    //
    // KVFND(*) indicates whether a specified kernel variable was found.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSWFFET", ctx)?;

    //
    // No result found yet.
    //
    *FRAMAT = 0;

    //
    // Create names of kernel variables, using the input frame ID.
    //
    // Basic frame specification variables, excluding that for the
    // frame ID. We'll need the frame name before we can create the
    // name of the kernel variable for the frame ID.
    //
    REPMI(TMCENT, b"#", FRAMID, &mut KVCENT, ctx);
    REPMI(TMCLAS, b"#", FRAMID, &mut KVCLAS, ctx);
    REPMI(TMCLID, b"#", FRAMID, &mut KVCLID, ctx);
    REPMI(TMFNAM, b"#", FRAMID, &mut KVFNAM, ctx);

    //
    // Switch frame specification variables:
    //
    REPMI(TMALGN, b"#", FRAMID, &mut KVALGN, ctx);
    REPMI(TMSTRT, b"#", FRAMID, &mut KVSTRT, ctx);
    REPMI(TMSTOP, b"#", FRAMID, &mut KVSTOP, ctx);

    //
    // Below, we use logical flags to describe the status
    // of kernel variable availability:
    //
    //     KVFND(*) indicates that a variable exists.
    //
    // Look up the basic variables.
    //
    GIPOOL(
        &KVCLAS,
        1,
        1,
        &mut N,
        std::slice::from_mut(&mut FRCLAS),
        &mut KVFND[IXCLAS],
        ctx,
    )?;
    GIPOOL(
        &KVCLID,
        1,
        1,
        &mut N,
        std::slice::from_mut(&mut FRCLID),
        &mut KVFND[IXCLID],
        ctx,
    )?;
    GCPOOL(
        &KVFNAM,
        1,
        1,
        &mut N,
        CharArrayMut::from_mut(&mut FRNAME),
        &mut KVFND[IXFNAM],
        ctx,
    )?;

    if FAILED(ctx) {
        ZZSWFCLN(
            HDFRAM.as_slice_mut(),
            FRPOOL.as_slice_mut(),
            BASBEG.as_slice_mut(),
            FRAMAT,
            ctx,
        )?;

        CHKOUT(b"ZZSWFFET", ctx)?;
        return Ok(());
    }

    //
    // Fetch the switch frame ID variable.
    //
    if KVFND[IXFNAM] {
        //
        // Use the frame name to fetch the switch frame ID variable.
        //
        REPMC(TMID, b"#", &FRNAME, &mut KVID);

        GIPOOL(
            &KVID,
            1,
            1,
            &mut N,
            std::slice::from_mut(&mut FID),
            &mut KVFND[IXID],
            ctx,
        )?;

        if FAILED(ctx) {
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }

        //
        // The frame ID of the frame specification had better match
        // the input frame ID.
        //
        if (KVFND[IXID] && (FID != FRAMID)) {
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            SETMSG(
                b"Input frame ID was #, but ID in frame specification from kernel pool was #. ",
                ctx,
            );
            ERRINT(b"#", FRAMID, ctx);
            ERRINT(b"#", FID, ctx);
            SIGERR(b"SPICE(BADFRAMESPEC)", ctx)?;
            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }
    //
    // The frame name must always be found. If not, the error
    // will be diagnosed in the block below.
    //
    } else {
        //
        // We couldn't find the frame name. Indicate the frame ID
        // variable [sic] wasn't found, since its found flag won't be set
        // by GIPOOL.
        //
        KVFND[IXID] = false;
    }

    //
    // Look up the central body of the frame. The name of the kernel
    // variable for the body could refer to the frame by name or frame
    // ID; the body itself could be specified by name or body ID.
    //
    if KVFND[IXFNAM] {
        ZZDYNBID(&FRNAME, FRAMID, b"CENTER", &mut FRCENT, ctx)?;

        if FAILED(ctx) {
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }

        KVFND[IXCENT] = true;
    } else {
        KVFND[IXCENT] = false;
    }

    //
    // Look up the type and count of the base frame list. It
    // may have either string or numeric type.
    //
    DTPOOL(&KVALGN, &mut KVFND[IXALGN], &mut NBASES, &mut BASTYP, ctx)?;

    if FAILED(ctx) {
        //
        // This code should be unreachable but is provided for safety.
        //
        ZZSWFCLN(
            HDFRAM.as_slice_mut(),
            FRPOOL.as_slice_mut(),
            BASBEG.as_slice_mut(),
            FRAMAT,
            ctx,
        )?;

        CHKOUT(b"ZZSWFFET", ctx)?;
        return Ok(());
    }

    //
    // Store the kernel variable names in order to prepare for
    // checking availability of required variables.
    //
    fstr::assign(KVNAMS.get_mut(IXID), &KVID);
    fstr::assign(KVNAMS.get_mut(IXFNAM), &KVFNAM);
    fstr::assign(KVNAMS.get_mut(IXCENT), &KVCENT);
    fstr::assign(KVNAMS.get_mut(IXCLAS), &KVCLAS);
    fstr::assign(KVNAMS.get_mut(IXCLID), &KVCLID);
    fstr::assign(KVNAMS.get_mut(IXALGN), &KVALGN);

    //
    // Check for required variables that haven't been supplied.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = IXALGN;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // The first 6 items are needed; start and stop times are needed
            // if at least one is present.
            //
            if !KVFND[I] {
                ZZSWFCLN(
                    HDFRAM.as_slice_mut(),
                    FRPOOL.as_slice_mut(),
                    BASBEG.as_slice_mut(),
                    FRAMAT,
                    ctx,
                )?;

                SETMSG(b"Kernel variable #, needed for specification of switch frame having frame ID #, was not found in the kernel pool. This can occur when a frame kernel providing the required switch frame specification has not been loaded, or if the specification is present but is incorrect.", ctx);
                ERRCH(b"#", &KVNAMS[I], ctx);
                ERRINT(b"#", FRAMID, ctx);
                SIGERR(b"SPICE(MISSINGFRAMEVAR)", ctx)?;
                CHKOUT(b"ZZSWFFET", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Find out whether time bounds are supplied, and if so, which data
    // type is used to represent them.
    //
    fstr::assign(KVNAMS.get_mut(IXSTRT), &KVSTRT);
    fstr::assign(KVNAMS.get_mut(IXSTOP), &KVSTOP);

    DTPOOL(&KVSTRT, &mut KVFND[IXSTRT], &mut NSTART, &mut STRTYP, ctx)?;
    DTPOOL(&KVSTOP, &mut KVFND[IXSTOP], &mut NSTOP, &mut STPTYP, ctx)?;

    if FAILED(ctx) {
        //
        // This code should be unreachable but is provided for safety.
        //
        ZZSWFCLN(
            HDFRAM.as_slice_mut(),
            FRPOOL.as_slice_mut(),
            BASBEG.as_slice_mut(),
            FRAMAT,
            ctx,
        )?;

        CHKOUT(b"ZZSWFFET", ctx)?;
        return Ok(());
    }

    //
    // Availability of starts and stops is optional...however both
    // must be provided if either is, and the counts must match
    // those of the base frames.
    //
    // Note that it's necessary to perform these checks before buffering
    // base frame data, since that process assumes that the number of
    // start and stop times is either zero or matches the number of
    // base frames.
    //
    STRXST = KVFND[IXSTRT];
    STPXST = KVFND[IXSTOP];

    HAVTIM = (STRXST && STPXST);

    if HAVTIM {
        //
        // Make sure that counts of base frames, start times, and
        // stop times are equal.
        //
        if ((NSTART != NSTOP) || (NSTART != NBASES)) {
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            SETMSG(b"Kernel variables for the switch frame having frame ID # have mismatched sizes: number of base frames = #; number of start times = #; number of stop times = #.", ctx);
            ERRINT(b"#", FRAMID, ctx);
            ERRINT(b"#", NBASES, ctx);
            ERRINT(b"#", NSTART, ctx);
            ERRINT(b"#", NSTOP, ctx);
            SIGERR(b"SPICE(COUNTMISMATCH)", ctx)?;
            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }
    } else {
        //
        // Check for inconsistent presence of start and stop times.
        //
        if (STRXST || STPXST) {
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            //
            // Create long message template to be filled in.
            //
            SETMSG(b"Kernel variable #, which specifies base frame applicability # times, was not provided for the switch frame having frame ID #, while the kernel variable # specifying base frame applicability # times was provided. Switch frame applicability start and stop times are optional, but both must be provided if either is.", ctx);

            if STRXST {
                //
                // Stop times are missing.
                //
                ERRCH(b"#", &KVSTOP, ctx);
                ERRCH(b"#", b"stop", ctx);
                ERRINT(b"#", FRAMID, ctx);
                ERRCH(b"#", &KVSTRT, ctx);
                ERRCH(b"#", b"start", ctx);
            } else {
                //
                // Start times are missing.
                //
                ERRCH(b"#", &KVSTRT, ctx);
                ERRCH(b"#", b"start", ctx);
                ERRINT(b"#", FRAMID, ctx);
                ERRCH(b"#", &KVSTOP, ctx);
                ERRCH(b"#", b"stop", ctx);
            }

            SIGERR(b"SPICE(PARTIALFRAMESPEC)", ctx)?;
            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }
    }

    //
    // Compute room for the output data.
    //
    ROOM = ((MAXBAS + 1) - *FREE);

    //
    // Check the available room in the frame ID pool and in the frame
    // base arrays.
    //
    ZZHSIAVL(FRPOOL.as_slice(), &mut NFRAVL);

    if ((NFRAVL == 0) || (ROOM < NBASES)) {
        //
        // There's no room for another frame in the frame pool, or
        // there's no room for the new frame's data in the base
        // frame arrays.
        //
        // If we can make enough room by re-initializing the whole
        // local frame database, do so. If the new frame has too
        // much data to fit in the empty base frame arrays, that's
        // an error.
        //
        if (NBASES <= MAXBAS) {
            //
            // We can fit the frame data in by clearing out the
            // database.
            //
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            //
            // The database is now initialized.
            //
            *FREE = 1;
            ROOM = MAXBAS;
        } else {
            //
            // We can't make enough room.
            //
            // Initialize local data structures, for safety.
            //
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            SETMSG(b"The requested frame # has # associated base frames. The maximum number that can be supported is #.", ctx);
            ERRINT(b"#", FRAMID, ctx);
            ERRINT(b"#", NBASES, ctx);
            ERRINT(b"#", MAXBAS, ctx);
            SIGERR(b"SPICE(TOOMANYBASEFRAMES)", ctx)?;
            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }
    }

    //
    // Add the frame and its data to the local database.
    //
    // Start by adding the frame ID to the frame hash structure.
    // The output argument NEW indicates whether the item is new;
    // we don't need to check this argument. We also don't need to
    // check for failure of ZZHSIADD, since we've already ensured
    // there's room in the hash for a new clock.
    //
    ZZHSIADD(
        HDFRAM.as_slice_mut(),
        FRPOOL.as_slice_mut(),
        FIDLST.as_slice_mut(),
        FRAMID,
        FRAMAT,
        &mut NEW,
        ctx,
    )?;

    //
    // At this point, FRAMAT is set.
    //
    // Store the frame ID, base start index, base count, and time
    // interval availability attributes for this frame.
    //
    FIDLST[*FRAMAT] = FRAMID;
    BASBEG[*FRAMAT] = *FREE;
    BASCNT[*FRAMAT] = NBASES;
    USETIM[*FRAMAT] = HAVTIM;

    //
    // Look up the base frame variables. The base frame list may have
    // either string or numeric type.
    //
    KVFND[IXALGN] = false;

    if fstr::eq(&BASTYP, b"N") {
        //
        // The frames are specified by ID code. We can buffer the frame
        // ID codes immediately.
        //
        GIPOOL(
            &KVALGN,
            1,
            ROOM,
            &mut BASCNT[*FREE],
            BASLST.subarray_mut(*FREE),
            &mut KVFND[IXALGN],
            ctx,
        )?;

        if FAILED(ctx) {
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }
    } else if fstr::eq(&BASTYP, b"C") {
        //
        // The frames are specified by name. We must convert the
        // names to frame ID codes before we can can buffer the IDs.
        //
        KVFND[IXALGN] = false;

        {
            let m1__: i32 = 1;
            let m2__: i32 = NBASES;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                GCPOOL(
                    &KVALGN,
                    I,
                    1,
                    &mut N,
                    CharArrayMut::from_mut(&mut BASNAM),
                    &mut KVFND[IXALGN],
                    ctx,
                )?;

                if FAILED(ctx) {
                    //
                    // This code should be unreachable but is provided for
                    // safety.
                    //
                    ZZSWFCLN(
                        HDFRAM.as_slice_mut(),
                        FRPOOL.as_slice_mut(),
                        BASBEG.as_slice_mut(),
                        FRAMAT,
                        ctx,
                    )?;

                    CHKOUT(b"ZZSWFFET", ctx)?;
                    return Ok(());
                }

                NAMFRM(&BASNAM, &mut BASLST[((*FREE - 1) + I)], ctx)?;

                if (BASLST[((*FREE - 1) + I)] == 0) {
                    ZZSWFCLN(
                        HDFRAM.as_slice_mut(),
                        FRPOOL.as_slice_mut(),
                        BASBEG.as_slice_mut(),
                        FRAMAT,
                        ctx,
                    )?;

                    SETMSG(b"Base frame name # of switch frame # could not be translated to a frame ID code ", ctx);
                    ERRCH(b"#", &BASNAM, ctx);
                    ERRINT(b"#", FRAMID, ctx);
                    SIGERR(b"SPICE(FRAMENAMENOTFOUND)", ctx)?;
                    CHKOUT(b"ZZSWFFET", ctx)?;
                    return Ok(());
                }

                I += m3__;
            }
        }
    } else {
        //
        // Backstop: this code should be unreachable.
        //
        ZZSWFCLN(
            HDFRAM.as_slice_mut(),
            FRPOOL.as_slice_mut(),
            BASBEG.as_slice_mut(),
            FRAMAT,
            ctx,
        )?;

        SETMSG(b"Base frame kernel variable # exists but DTPOOL returned data type # rather than one of the expected values: \'C\' or \'N\'.", ctx);
        ERRCH(b"#", &KVALGN, ctx);
        ERRCH(b"#", &BASTYP, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZSWFFET", ctx)?;
        return Ok(());
    }

    //
    // Get attributes of each base frame.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NBASES;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // Store the frame class and frame class ID for this base frame.
            //
            J = ((*FREE - 1) + I);

            FRINFO(
                BASLST[J],
                &mut CENTER,
                &mut CLSSES[J],
                &mut CLSIDS[J],
                &mut INFFND,
                ctx,
            )?;

            if FAILED(ctx) {
                ZZSWFCLN(
                    HDFRAM.as_slice_mut(),
                    FRPOOL.as_slice_mut(),
                    BASBEG.as_slice_mut(),
                    FRAMAT,
                    ctx,
                )?;

                CHKOUT(b"ZZSWFFET", ctx)?;
                return Ok(());
            }

            if !INFFND {
                ZZSWFCLN(
                    HDFRAM.as_slice_mut(),
                    FRPOOL.as_slice_mut(),
                    BASBEG.as_slice_mut(),
                    FRAMAT,
                    ctx,
                )?;

                SETMSG(
                    b"No specification was found for base frame # of switch frame #.",
                    ctx,
                );
                ERRINT(b"#", BASLST[J], ctx);
                ERRINT(b"#", FRAMID, ctx);
                SIGERR(b"SPICE(FRAMEINFONOTFOUND)", ctx)?;
                CHKOUT(b"ZZSWFFET", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Fetch interval bounds if they're present.
    //
    if USETIM[*FRAMAT] {
        //
        // Fetch start times.
        //
        // Note that DTPOOL sets the type to 'X' if the target
        // kernel variable is not found.
        //
        if fstr::eq(&STRTYP, b"N") {
            //
            // The start times are represented by double precision
            // numbers.
            //
            GDPOOL(
                &KVSTRT,
                1,
                ROOM,
                &mut NSTART,
                STARTS.subarray_mut(*FREE),
                &mut KVFND[IXSTRT],
                ctx,
            )?;
        } else if fstr::eq(&STRTYP, b"C") {
            //
            // The start times are represented by strings.
            //
            // We must convert each string to a numeric value and store
            // the value in the start time buffer.
            //
            KVFND[IXSTRT] = false;

            {
                let m1__: i32 = 1;
                let m2__: i32 = NBASES;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    GCPOOL(
                        &KVSTRT,
                        I,
                        1,
                        &mut N,
                        CharArrayMut::from_mut(&mut TIMSTR),
                        &mut KVFND[IXSTRT],
                        ctx,
                    )?;

                    STR2ET(&TIMSTR, &mut STARTS[((*FREE - 1) + I)], ctx)?;

                    I += m3__;
                }
            }
        } else {
            //
            // Backstop: this code should be unreachable.
            //
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            SETMSG(b"Start time kernel variable # exists but DTPOOL returned data type # rather than one of the expected values: \'C\' or \'N\'.", ctx);
            ERRCH(b"#", &KVSTRT, ctx);
            ERRCH(b"#", &STRTYP, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }

        if FAILED(ctx) {
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }

        //
        // Fetch stop times.
        //
        if fstr::eq(&STPTYP, b"N") {
            //
            // The stop times are represented by double precision numbers.
            //
            GDPOOL(
                &KVSTOP,
                1,
                ROOM,
                &mut NSTOP,
                STOPS.subarray_mut(*FREE),
                &mut KVFND[IXSTOP],
                ctx,
            )?;
        } else if fstr::eq(&STPTYP, b"C") {
            //
            // The stop times are represented by strings.
            //
            // We must convert each string to a numeric value and store
            // the value in the stop time buffer.
            //
            KVFND[IXSTOP] = false;

            {
                let m1__: i32 = 1;
                let m2__: i32 = NSTOP;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    GCPOOL(
                        &KVSTOP,
                        I,
                        1,
                        &mut N,
                        CharArrayMut::from_mut(&mut TIMSTR),
                        &mut KVFND[IXSTOP],
                        ctx,
                    )?;

                    STR2ET(&TIMSTR, &mut STOPS[((*FREE - 1) + I)], ctx)?;

                    I += m3__;
                }
            }
        } else {
            //
            // Backstop: this code should be unreachable.
            //
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            SETMSG(b"Stop time kernel variable # exists but DTPOOL returned data type # rather than one of the expected values: \'C\' or \'N\'.", ctx);
            ERRCH(b"#", &KVSTOP, ctx);
            ERRCH(b"#", &STPTYP, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }

        if FAILED(ctx) {
            ZZSWFCLN(
                HDFRAM.as_slice_mut(),
                FRPOOL.as_slice_mut(),
                BASBEG.as_slice_mut(),
                FRAMAT,
                ctx,
            )?;

            CHKOUT(b"ZZSWFFET", ctx)?;
            return Ok(());
        }

        //
        // Singleton intervals and out-of-order intervals are not
        // allowed.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = NSTART;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                J = ((*FREE - 1) + I);

                if (STARTS[J] >= STOPS[J]) {
                    ZZSWFCLN(
                        HDFRAM.as_slice_mut(),
                        FRPOOL.as_slice_mut(),
                        BASBEG.as_slice_mut(),
                        FRAMAT,
                        ctx,
                    )?;

                    SETMSG(b"Interval time bounds are not strictly increasing at interval index # for switch frame #. Time bounds are #:# TDB (# TDB : # TDB)", ctx);
                    ERRINT(b"#", I, ctx);
                    ERRINT(b"#", FRAMID, ctx);
                    ERRDP(b"#", STARTS[J], ctx);
                    ERRDP(b"#", STOPS[J], ctx);
                    ETCAL(STARTS[J], &mut TIMSTR, ctx);
                    ERRCH(b"#", &TIMSTR, ctx);
                    ETCAL(STOPS[J], &mut TIMSTR, ctx);
                    ERRCH(b"#", &TIMSTR, ctx);
                    SIGERR(b"SPICE(BADTIMEBOUNDS)", ctx)?;
                    CHKOUT(b"ZZSWFFET", ctx)?;
                    return Ok(());
                }

                I += m3__;
            }
        }
    }

    //
    // Determine whether binary search on the time intervals is
    // possible.
    //
    if USETIM[*FRAMAT] {
        I = 2;
        BINARY[*FRAMAT] = true;

        while ((I <= NBASES) && BINARY[*FRAMAT]) {
            //
            // Note that proper ordering of start and stop times for
            // each interval has already been verified.
            //
            J = ((BASBEG[*FRAMAT] - 1) + I);

            if (STOPS[(J - 1)] > STARTS[J]) {
                BINARY[*FRAMAT] = false;
            }

            I = (I + 1);
        }
    } else {
        BINARY[*FRAMAT] = false;
    }

    //
    // We've found the data. The switch frame database has been
    // updated. FRAMAT is already set.
    //
    // Account for the base frame storage used.
    //
    *FREE = (*FREE + NBASES);

    CHKOUT(b"ZZSWFFET", ctx)?;
    Ok(())
}
