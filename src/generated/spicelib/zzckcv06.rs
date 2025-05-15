//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 23;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const C06TP0: i32 = 0;
const C06TP1: i32 = (C06TP0 + 1);
const C06TP2: i32 = (C06TP1 + 1);
const C06TP3: i32 = (C06TP2 + 1);
const C06NST: i32 = 4;
const C06PS0: i32 = 8;
const C06PS1: i32 = 4;
const C06PS2: i32 = 14;
const C06PS3: i32 = 7;
const C06MXZ: i32 = C06PS2;
const C06MNZ: i32 = C06PS1;
const MAXRSZ: i32 = (4 + ((MAXDEG + 1) * (C06PS3 + 1)));
const LBCELL: i32 = -5;
const NVDSIZ: i32 = 100;
const DIRSIZ: i32 = 100;
const NSGPAR: i32 = 2;
const MCTLSZ: i32 = 4;

//$Procedure ZZCKCV06 ( Private --- C-kernel segment coverage, type 06 )
pub fn ZZCKCV06(
    HANDLE: i32,
    ARRBEG: i32,
    ARREND: i32,
    SCLKID: i32,
    DC: &[f64],
    TOL: f64,
    TIMSYS: &[u8],
    SCHEDL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DC = DummyArray::new(DC, 1..=2);
    let mut SCHEDL = DummyArrayMut::new(SCHEDL, LBCELL..);
    let mut BEGIN: f64 = 0.0;
    let mut BUFFER = StackArray::<f64, 4>::new(1..=MCTLSZ);
    let mut ET: f64 = 0.0;
    let mut FINISH: f64 = 0.0;
    let mut IVLBDS = StackArray::<f64, 2>::new(1..=2);
    let mut LSTEPC: f64 = 0.0;
    let mut EPADDR: i32 = 0;
    let mut IVLBAS: i32 = 0;
    let mut MINIE: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut NINTVL: i32 = 0;
    let mut NIVDIR: i32 = 0;
    let mut NREC: i32 = 0;
    let mut PTRBAS: i32 = 0;
    let mut ISTDB: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Number of elements in a type 6 mini-segment
    // interval directory:
    //

    //
    // Mini-segment epoch directory size:
    //

    //
    // Type 6 control area size:
    //

    //
    // Type 6 mini-segment control area size:
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZCKCV06", ctx)?;

    //
    // Check tolerance value.
    //
    if (TOL < 0.0) {
        SETMSG(b"Tolerance must be non-negative; actual value was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZCKCV06", ctx)?;
        return Ok(());
    }

    //
    // Set a logical flag indicating whether the time system is SCLK.
    //
    ISTDB = EQSTR(TIMSYS, b"TDB");

    //
    // Check time system.
    //
    if !ISTDB {
        if !EQSTR(TIMSYS, b"SCLK") {
            SETMSG(
                b"Time system spec TIMSYS was #; allowed values are SCLK and TDB.",
                ctx,
            );
            ERRCH(b"#", TIMSYS, ctx);
            SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
            CHKOUT(b"ZZCKCV06", ctx)?;
            return Ok(());
        }
    }

    //
    // Fetch the mini-segment count from the segment.
    //
    DAFGDA(HANDLE, ARREND, ARREND, BUFFER.as_slice_mut(), ctx)?;

    NINTVL = intrinsics::IDNINT(BUFFER[1]);

    //
    // Each mini-segment contributes a coverage interval to the
    // total coverage of the segment. Since mini-segments can
    // contain gaps, we need to examine not only the mini-segment
    // interval bounds but the final epochs of the mini-segments.
    //
    // Let IVLBAS be the base address of the mini-segment interval
    // bounds. Let PTRBAS be the base address of the mini-segment
    // pointers.
    //
    // First compute PTRBAS. There are NINTVL+1 pointers.
    //
    PTRBAS = ((ARREND - NSGPAR) - (NINTVL + 1));

    //
    // Compute the number of mini-segment interval directories.
    // There are NINTVL + 1 interval boundaries, so the directory
    // count is
    //
    //    (  ( NINTVL + 1 ) - 1  )  /  NVDSIZ
    //
    //
    NIVDIR = (NINTVL / NVDSIZ);

    //
    // The interval bounds and their directories precede the
    // mini-segment pointers.
    //
    IVLBAS = ((PTRBAS - NIVDIR) - (NINTVL + 1));

    //
    // Now loop over the mini-segments and find the contribution
    // from each one.
    //
    for I in 1..=NINTVL {
        //
        // Find the interval bounds for this mini-segment.
        //
        DAFGDA(
            HANDLE,
            (IVLBAS + I),
            ((IVLBAS + I) + 1),
            IVLBDS.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZCKCV06", ctx)?;
            return Ok(());
        }

        //
        // Now find the last epoch of this mini-segment, since
        // there could be a gap at the end.
        //
        // Find the begin and end pointers for the current
        // mini-segment. Convert these from relative to
        // absolute DAF addresses.
        //
        DAFGDA(
            HANDLE,
            (PTRBAS + I),
            ((PTRBAS + I) + 1),
            BUFFER.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZCKCV06", ctx)?;
            return Ok(());
        }

        MINIE = (((ARRBEG - 1) + intrinsics::IDNINT(BUFFER[2])) - 1);

        //
        // Fetch the mini-segment's record count NREC.
        //
        DAFGDA(HANDLE, MINIE, MINIE, BUFFER.as_slice_mut(), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZCKCV06", ctx)?;
            return Ok(());
        }

        NREC = intrinsics::IDNINT(BUFFER[1]);

        //
        // Compute the number of epoch directories for this
        // mini-segment.
        //
        NDIR = ((NREC - 1) / DIRSIZ);

        //
        // The last epoch precedes the mini-segment control
        // area and the epoch directories.
        //
        EPADDR = ((MINIE - MCTLSZ) - NDIR);

        DAFGDA(
            HANDLE,
            EPADDR,
            EPADDR,
            std::slice::from_mut(&mut LSTEPC),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"ZZCKCV06", ctx)?;
            return Ok(());
        }

        BEGIN = IVLBDS[1];
        //
        // The smaller of LSTEPC and IVLBDS(2) is the
        // end of the mini-segment's coverage.
        //
        FINISH = intrinsics::DMIN1(&[LSTEPC, IVLBDS[2]]);
        //
        // Truncate the interval using the segment bounds.
        //
        BEGIN = intrinsics::DMAX1(&[BEGIN, DC[1]]);
        FINISH = intrinsics::DMIN1(&[FINISH, DC[2]]);

        //
        // Adjust the interval using the tolerance. Empty
        // intervals *do not get expanded*; this choice is
        // consistent with the type 6 reading algorithm.
        //
        if (BEGIN <= FINISH) {
            if (TOL > 0.0) {
                BEGIN = intrinsics::DMAX1(&[(BEGIN - TOL), 0.0]);
                FINISH = (FINISH + TOL);
            }
        }

        //
        // Convert the time to TDB if necessary.
        //
        if ISTDB {
            SCT2E(SCLKID, BEGIN, &mut ET, ctx)?;
            BEGIN = ET;

            SCT2E(SCLKID, FINISH, &mut ET, ctx)?;
            FINISH = ET;

            if FAILED(ctx) {
                CHKOUT(b"ZZCKCV06", ctx)?;
                return Ok(());
            }
        }

        //
        // Insert the interval into the window.
        //
        if (BEGIN <= FINISH) {
            WNINSD(BEGIN, FINISH, SCHEDL.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"ZZCKCV06", ctx)?;
                return Ok(());
            }
        }
    }

    CHKOUT(b"ZZCKCV06", ctx)?;
    Ok(())
}
