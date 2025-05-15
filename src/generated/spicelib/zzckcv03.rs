//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure ZZCKCV03 ( Private --- C-kernel segment coverage, type 03 )
pub fn ZZCKCV03(
    HANDLE: i32,
    ARRBEG: i32,
    ARREND: i32,
    SCLKID: i32,
    TOL: f64,
    TIMSYS: &[u8],
    SCHEDL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SCHEDL = DummyArrayMut::new(SCHEDL, LBCELL..);
    let mut BEGIN: f64 = 0.0;
    let mut BUFFER = StackArray::<f64, 2>::new(1..=2);
    let mut ET: f64 = 0.0;
    let mut FINISH: f64 = 0.0;
    let mut START: f64 = 0.0;
    let mut TICK: f64 = 0.0;
    let mut INTAT: i32 = 0;
    let mut INTBEG: i32 = 0;
    let mut INVLS: i32 = 0;
    let mut LSTINT: i32 = 0;
    let mut LSTTIK: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut NREC: i32 = 0;
    let mut RSIZE: i32 = 0;
    let mut TICKAT: i32 = 0;
    let mut NAVSLN: i32 = 0;
    let mut AVSLN: i32 = 0;
    let mut SEGLEN: i32 = 0;
    let mut BAIL: bool = false;
    let mut ISTDB: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZCKCV03", ctx)?;
    }

    //
    // Check tolerance value.
    //
    if (TOL < 0.0) {
        SETMSG(b"Tolerance must be non-negative; actual value was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZCKCV03", ctx)?;
        return Ok(());
    }

    //
    // Set a logical flag indicating whether the time systm is SCLK.
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
            CHKOUT(b"ZZCKCV03", ctx)?;
            return Ok(());
        }
    }

    //
    // Get the number of intervals and pointing instances ( records )
    // in this segment, and from that determine the number of respective
    // directory epochs.
    //
    DAFGDA(HANDLE, (ARREND - 1), ARREND, BUFFER.as_slice_mut(), ctx)?;

    INVLS = intrinsics::IDNINT(BUFFER[1]);
    NREC = intrinsics::IDNINT(BUFFER[2]);
    NDIR = ((NREC - 1) / 100);

    //
    // Determine the size of the pointing packets.  This is dependent
    // on whether angular rate data is present in the segment or not.
    // We can determine this with the following computation:
    //
    // Assume a record size of 4, i.e. no angular rate data.
    //
    NAVSLN = (((((5 * NREC) + NDIR) + INVLS) + ((INVLS - 1) / 100)) + 2);

    //
    // Assume a record size of 7, i.e. angular rate data.
    //
    AVSLN = (((((8 * NREC) + NDIR) + INVLS) + ((INVLS - 1) / 100)) + 2);

    //
    // Compute the actual length of the segment.
    //
    SEGLEN = ((ARREND - ARRBEG) + 1);

    if (SEGLEN == NAVSLN) {
        RSIZE = 4;
    } else if (SEGLEN == AVSLN) {
        RSIZE = 7;
    } else {
        SETMSG(b"The requested segment in file # reports a length of # d.p. numbers, but the metadata in the segment indicates the length must either be # (no angular rate data) or # (angular rate data). Perhaps the segment is not type 3?", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", SEGLEN, ctx);
        ERRINT(b"#", NAVSLN, ctx);
        ERRINT(b"#", AVSLN, ctx);
        SIGERR(b"SPICE(BADCK3SEGMENT)", ctx)?;
        CHKOUT(b"ZZCKCV03", ctx)?;
        return Ok(());
    }

    //
    // Recall that the segment is layed out as:
    //
    //   +------------------------------+
    //   |                              |
    //   |  Pointing                    |
    //   |                              |
    //   +------------------------------+
    //   |                        |
    //   |  SCLK times            |
    //   |                        |
    //   +------------------------+
    //   |                        |
    //   |  SCLK directory        |
    //   |                        |
    //   +------------------------+
    //   |                        |
    //   |  Interval start times  |
    //   |                        |
    //   +------------------------+
    //   |                        |
    //   |  Start times directory |
    //   |                        |
    //   +------------------------+
    //   |                        |
    //   |  Number of intervals   |
    //   |                        |
    //   +------------------------+
    //   |                        |
    //   |  Number of pointing    |
    //   |      instances         |
    //   |                        |
    //   +------------------------+
    //

    TICKAT = (ARRBEG + (RSIZE * NREC));
    LSTTIK = ((TICKAT + NREC) - 1);
    INTBEG = (((ARRBEG + (RSIZE * NREC)) + NREC) + NDIR);
    INTAT = INTBEG;
    LSTINT = ((INTBEG + INVLS) - 1);

    DAFGDA(HANDLE, INTAT, INTAT, std::slice::from_mut(&mut START), ctx)?;
    DAFGDA(HANDLE, TICKAT, TICKAT, std::slice::from_mut(&mut TICK), ctx)?;

    while ((TICK < START) && (TICKAT < LSTTIK)) {
        TICKAT = (TICKAT + 1);
        DAFGDA(HANDLE, TICKAT, TICKAT, std::slice::from_mut(&mut TICK), ctx)?;
    }

    //
    // If we did not find a TICK at least as big as START, we can
    // just return now.
    //
    if (TICK < START) {
        CHKOUT(b"ZZCKCV03", ctx)?;
        return Ok(());
    }

    BAIL = false;

    while (((INTAT <= LSTINT) && (TICKAT <= LSTTIK)) && !BAIL) {
        //
        // At this point, we have an interval that begins at START
        // and ends at FINISH (unless of course we never found a "good"
        // TICK to start with.)
        //
        BEGIN = START;

        //
        // If the start of the interval was the start of the LAST
        // interval available, we can short cut the remainder of the
        // reads.
        //
        if (INTAT == LSTINT) {
            DAFGDA(
                HANDLE,
                LSTTIK,
                LSTTIK,
                std::slice::from_mut(&mut FINISH),
                ctx,
            )?;

            BAIL = true;
        //
        // The routine will return at the end of this loop
        // iteration.  But first, we may have to update BEGIN
        // and FINISH, depending on the values of TOL and TIMSYS,
        // and we have to insert these values into SCHEDL.
        // We'll carry out these tasks at the end of this IF block.
        //
        } else {
            //
            // This is the expected case.  Get the start of the next
            // interval.
            //
            INTAT = (INTAT + 1);
            DAFGDA(HANDLE, INTAT, INTAT, std::slice::from_mut(&mut START), ctx)?;

            //
            // Read forward from the last tick until we reach the
            // START of the next interval or until we run out of TICKS.
            //
            while ((TICK < START) && (TICKAT < LSTTIK)) {
                FINISH = TICK;
                TICKAT = (TICKAT + 1);
                DAFGDA(HANDLE, TICKAT, TICKAT, std::slice::from_mut(&mut TICK), ctx)?;
            }

            //
            // A structurally correct CK-3 segment should never allow the
            // next test to pass, but it's just easier to check than
            // police the writers of C-kernels.  The only way to get into
            // the block below is if TICKAT .EQ. LSTTIK
            //
            if (TICK < START) {
                FINISH = TICK;
                TICKAT = (TICKAT + 1);
            }
        }

        //
        // Adjust the interval using the tolerance.
        //
        if (TOL > 0.0) {
            BEGIN = intrinsics::DMAX1(&[(BEGIN - TOL), 0.0]);
            FINISH = (FINISH + TOL);
        }

        //
        // Convert the time to TDB if necessary.
        //
        if ISTDB {
            SCT2E(SCLKID, BEGIN, &mut ET, ctx)?;
            BEGIN = ET;

            SCT2E(SCLKID, FINISH, &mut ET, ctx)?;
            FINISH = ET;
        }

        //
        // Insert the interval into the window.
        //
        WNINSD(BEGIN, FINISH, SCHEDL.as_slice_mut(), ctx)?;
    }

    CHKOUT(b"ZZCKCV03", ctx)?;
    Ok(())
}
