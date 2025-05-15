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

//$Procedure ZZDSKSBR ( DSK, remove entries from API segment buffer )
pub fn ZZDSKSBR(
    NEEDED: i32,
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
    let mut AVAIL: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut NBTDEL: i32 = 0;
    let mut NSTDEL: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZDSKSBR", ctx)?;

    if (NEEDED > STSIZE) {
        //
        // There's not enough room in the entire segment table.
        //
        SETMSG(b"Size of segment table is #; number of entries requested is #. The segment table is supposed to be declared with sufficient size to accommodate all loaded DSK segments.", ctx);
        ERRINT(b"#", STSIZE, ctx);
        ERRINT(b"#", NEEDED, ctx);
        SIGERR(b"SPICE(SEGTABLETOOSMALL)", ctx)?;
        CHKOUT(b"ZZDSKSBR", ctx)?;
        return Ok(());
    }

    //
    // We can't make room in a body table of zero size.
    //
    if (MAXBOD < 1) {
        SETMSG(b"Body table size must be at least 1 but is #.", ctx);
        ERRINT(b"#", MAXBOD, ctx);
        SIGERR(b"SPICE(INVALIDTABLESIZE)", ctx)?;
        CHKOUT(b"ZZDSKSBR", ctx)?;
        return Ok(());
    }

    //
    // AVAIL is the number of entries currently available.
    //
    AVAIL = ((STSIZE - *STFREE) + 1);

    if (AVAIL < NEEDED) {
        //
        // We need to make room in the segment table.
        //
        // The entries at the end of the body table have the highest
        // priority. We scan forward through this table, summing the
        // entry counts for each body, until we have enough entries. Let
        // NE represent the number of available entries. NE is initially
        // the number of unused entries.
        //
        NSTDEL = 0;
        I = 1;

        while ((I <= *BTNBOD) && (AVAIL < NEEDED)) {
            //
            // Add the segment count for the Ith body to the total.
            //
            NSTDEL = (NSTDEL + BTSTSZ[I]);
            AVAIL = (AVAIL + NSTDEL);
            I = (I + 1);
        }

        //
        // Backstop: we should always have enough room in the segment
        // table at this point.
        //
        if (AVAIL < NEEDED) {
            SETMSG(b"The requested number of segment entries is #; the size STSIZE of the input segment  table is #. This error should have been trapped before this point.", ctx);
            ERRINT(b"#", NEEDED, ctx);
            ERRINT(b"#", STSIZE, ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZDSKSBR", ctx)?;
            return Ok(());
        }

        //
        // At this point, I is the index of the first retained body,
        // unless all were deleted, in which case I is BTNBOD+1. We need
        // to delete the segment table entries of the bodies indexed from
        // 1 to I-1.
        //
        NBTDEL = (I - 1);

        if (NSTDEL > 0) {
            //
            // Adjust the tables to be consistent with the deletions.
            //
            // Shift the body table and update the body table pointers.
            //
            {
                let m1__: i32 = (NBTDEL + 1);
                let m2__: i32 = *BTNBOD;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    J = (I - NBTDEL);
                    BTBODY[J] = BTBODY[I];
                    BTSTSZ[J] = BTSTSZ[I];
                    BTSEGP[J] = (BTSEGP[I] - NSTDEL);

                    I += m3__;
                }
            }

            //
            // Update the body table count.
            //
            *BTNBOD = (*BTNBOD - NBTDEL);

            //
            // Shift the segment table entries forward by NSTDEL to make
            // room at the rear of the table.
            //
            {
                let m1__: i32 = (NSTDEL + 1);
                let m2__: i32 = (*STFREE - 1);
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    J = (I - NSTDEL);
                    STHAN[J] = STHAN[I];

                    MOVED(
                        &STDSCR.subarray([1, I]).to_vec(),
                        DSKDSZ,
                        STDSCR.subarray_mut([1, J]),
                    );
                    MOVEI(
                        &STDLAD.subarray([1, I]).to_vec(),
                        DLADSZ,
                        STDLAD.subarray_mut([1, J]),
                    );
                    MOVED(
                        &STOFF.subarray([1, I]).to_vec(),
                        3,
                        STOFF.subarray_mut([1, J]),
                    );
                    MOVED(
                        &STCTR.subarray([1, I]).to_vec(),
                        3,
                        STCTR.subarray_mut([1, J]),
                    );

                    STRAD[J] = STRAD[I];

                    I += m3__;
                }
            }

            *STFREE = (*STFREE - NSTDEL);
        }
    }

    CHKOUT(b"ZZDSKSBR", ctx)?;
    Ok(())
}
