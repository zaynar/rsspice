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
const FRNMLN: i32 = 32;

//$Procedure ZZDSKSBA ( DSK, add entry to API segment buffer )
pub fn ZZDSKSBA(
    BODYID: i32,
    MAXBOD: i32,
    STSIZE: i32,
    BTBODY: &mut [i32],
    BTNBOD: &mut i32,
    BTSEGP: &mut [i32],
    BTSTSZ: &mut [i32],
    STHAN: &mut [i32],
    STDSCR: &mut [f64],
    STDLAD: &mut [i32],
    STFREE: &mut i32,
    STOFF: &mut [f64],
    STCTR: &mut [f64],
    STRAD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BTBODY = DummyArrayMut::new(BTBODY, 1..);
    let mut BTSEGP = DummyArrayMut::new(BTSEGP, 1..);
    let mut BTSTSZ = DummyArrayMut::new(BTSTSZ, 1..);
    let mut STHAN = DummyArrayMut::new(STHAN, 1..);
    let mut STDSCR = DummyArrayMut2D::new(STDSCR, 1..=DSKDSZ, 1..);
    let mut STDLAD = DummyArrayMut2D::new(STDLAD, 1..=DLADSZ, 1..);
    let mut STOFF = DummyArrayMut2D::new(STOFF, 1..=3, 1..);
    let mut STCTR = DummyArrayMut2D::new(STCTR, 1..=3, 1..);
    let mut STRAD = DummyArrayMut::new(STRAD, 1..);
    let mut FRNAME = [b' '; FRNMLN as usize];
    let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
    let mut ET: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut AVAIL: i32 = 0;
    let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
    let mut FRMCTR: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut NSEG: i32 = 0;
    let mut SEGCLD: i32 = 0;
    let mut SEGCLS: i32 = 0;
    let mut SEGCTR: i32 = 0;
    let mut SEGFID: i32 = 0;
    let mut FRMFND: bool = false;
    let mut SEGFND: bool = false;
    let mut STATUS: bool = false;

    //
    // SPICELIB functions
    //

    //
    // External routines
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSKSBA", ctx)?;

    //
    // Check the body table for presence of the new body. It's
    // an error to call this routine for a body that's already
    // present. (Such a call likely indicates the tables were
    // not re-initialized after a BSR state change.)
    //
    I = ISRCHI(BODYID, *BTNBOD, BTBODY.as_slice());

    if (I > 0) {
        SETMSG(b"Body # is already present in the DSK segment buffer body table. The table must be re-initialized before this body can be added.", ctx);
        ERRINT(b"#", BODYID, ctx);
        SIGERR(b"SPICE(INVALIDADD)", ctx)?;
        CHKOUT(b"ZZDSKSBA", ctx)?;
        return Ok(());
    }

    //
    // Make sure the BSR segment list for the body is up to
    // date.
    //
    ZZDSKBBL(BODYID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZDSKSBA", ctx)?;
        return Ok(());
    }

    //
    // Count the segments in the BSR system for the body.
    //
    NSEG = 0;
    STATUS = ZZDSKSBD(BODYID, ctx);

    ZZDSKBSS(BODYID, ctx)?;
    ZZDSKSNS(
        ZZDSKBDC,
        &mut HANDLE,
        DLADSC.as_slice_mut(),
        DSKDSC.as_slice_mut(),
        &mut SEGFND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZDSKSBA", ctx)?;
        return Ok(());
    }

    while SEGFND {
        NSEG = (NSEG + 1);

        ZZDSKSNS(
            ZZDSKBDC,
            &mut HANDLE,
            DLADSC.as_slice_mut(),
            DSKDSC.as_slice_mut(),
            &mut SEGFND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDSKSBA", ctx)?;
            return Ok(());
        }
    }

    //
    // Check the number of segments for BODY against the size of the
    // segment table. If the table isn't big enough, we can't make
    // room by deleting existing entries. This is a backstop check;
    // this situation should not occur if STSIZE is consistent with
    // the value in ZZDSKBSR.
    //
    if (NSEG > STSIZE) {
        SETMSG(b"The number of segments for body # is #; the size STSIZE of the input segment table is #.", ctx);
        ERRINT(b"#", BODYID, ctx);
        ERRINT(b"#", NSEG, ctx);
        ERRINT(b"#", STSIZE, ctx);
        SIGERR(b"SPICE(SEGMENTTABLEFULL)", ctx)?;
        CHKOUT(b"ZZDSKSBA", ctx)?;
        return Ok(());
    }

    //
    // If we don't have enough room to store new entries in the body
    // table or in the segment table, make room.
    //
    AVAIL = ((STSIZE - *STFREE) + 1);

    if ((*BTNBOD == MAXBOD) || (AVAIL < NSEG)) {
        ZZDSKSBR(
            NSEG,
            MAXBOD,
            STSIZE,
            BTBODY.as_slice_mut(),
            BTNBOD,
            BTSEGP.as_slice_mut(),
            BTSTSZ.as_slice_mut(),
            STHAN.as_slice_mut(),
            STDSCR.as_slice_mut(),
            STDLAD.as_slice_mut(),
            STFREE,
            STOFF.as_slice_mut(),
            STCTR.as_slice_mut(),
            STRAD.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDSKSBA", ctx)?;
            return Ok(());
        }
    }

    //
    // Append the new body ID to the body table. We've ensured there's
    // room in the table.
    //
    *BTNBOD = (*BTNBOD + 1);
    BTBODY[*BTNBOD] = BODYID;
    BTSEGP[*BTNBOD] = *STFREE;
    BTSTSZ[*BTNBOD] = NSEG;

    //
    // Make a second pass through the BSR segment list, this time
    // accumulating segments in the input segment table as we go.
    //
    STATUS = ZZDSKSBD(BODYID, ctx);

    ZZDSKBSS(BODYID, ctx)?;
    ZZDSKSNS(
        ZZDSKBDC,
        &mut HANDLE,
        DLADSC.as_slice_mut(),
        DSKDSC.as_slice_mut(),
        &mut SEGFND,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"ZZDSKSBA", ctx)?;
        return Ok(());
    }

    while SEGFND {
        //
        // Insert handle and descriptor data for the current segment at
        // index STFREE in the segment table.
        //
        STHAN[*STFREE] = HANDLE;

        MOVEI(DLADSC.as_slice(), DLADSZ, STDLAD.subarray_mut([1, *STFREE]));
        MOVED(DSKDSC.as_slice(), DSKDSZ, STDSCR.subarray_mut([1, *STFREE]));

        *STFREE = (*STFREE + 1);

        ZZDSKSNS(
            ZZDSKBDC,
            &mut HANDLE,
            DLADSC.as_slice_mut(),
            DSKDSC.as_slice_mut(),
            &mut SEGFND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZDSKSBA", ctx)?;
            return Ok(());
        }
    }

    //
    // Compute bounding spheres and frame center offsets for each
    // segment.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NSEG;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // J is the index in the segment table of the Ith segment
            // for BODYID.
            //
            J = ((BTSEGP[*BTNBOD] + I) - 1);

            ZZSEGBOX(
                STDSCR.subarray([1, J]),
                STCTR.subarray_mut([1, J]),
                &mut STRAD[J],
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"ZZDSKSBA", ctx)?;
                return Ok(());
            }

            //
            // Obtain the center of the frame for the Ith segment.
            //
            SEGFID = intrinsics::IDNINT(STDSCR[[FRMIDX, J]]);

            FRINFO(
                SEGFID,
                &mut FRMCTR,
                &mut SEGCLS,
                &mut SEGCLD,
                &mut FRMFND,
                ctx,
            )?;

            if !FRMFND {
                SETMSG(
                    b"Could not look up frame info for segment frame having ID #.",
                    ctx,
                );
                ERRINT(b"#", SEGFID, ctx);
                SIGERR(b"SPICE(NOFRAMEINFO)", ctx)?;
                CHKOUT(b"ZZDSKSBA", ctx)?;
                return Ok(());
            }
            //
            // If the frame center is not the same as the central
            // body, compute the offset between the two. Otherwise
            // set the offset to zero.
            //
            SEGCTR = intrinsics::IDNINT(STDSCR[[CTRIDX, J]]);

            if (SEGCTR == FRMCTR) {
                CLEARD(3, STOFF.subarray_mut([1, J]));
            } else {
                FRMNAM(SEGFID, &mut FRNAME, ctx)?;

                if fstr::eq(&FRNAME, b" ") {
                    SETMSG(
                        b"Could not look up frame info for segment frame having ID #.",
                        ctx,
                    );
                    ERRINT(b"#", SEGFID, ctx);
                    SIGERR(b"SPICE(NOFRAMENAME)", ctx)?;
                    CHKOUT(b"ZZDSKSBA", ctx)?;
                    return Ok(());
                }

                //
                // Note that SPK data must be available at the midpoint of the
                // DSK coverage epoch in order for the following call to work.
                //
                ET = ((STDSCR[[BTMIDX, J]] + STDSCR[[ETMIDX, J]]) / 2 as f64);

                SPKGPS(
                    FRMCTR,
                    ET,
                    &FRNAME,
                    SEGCTR,
                    STOFF.subarray_mut([1, J]),
                    &mut LT,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"ZZDSKSBA", ctx)?;
                    return Ok(());
                }
            }

            I += m3__;
        }
    }

    CHKOUT(b"ZZDSKSBA", ctx)?;
    Ok(())
}
