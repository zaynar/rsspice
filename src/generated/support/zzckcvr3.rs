//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;

//$Procedure ZZCKCVR3 ( Private --- C-kernel segment coverage, type 03 )
pub fn ZZCKCVR3(
    HANDLE: i32,
    ARRBEG: i32,
    ARREND: i32,
    SCHEDL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SCHEDL = DummyArrayMut::new(SCHEDL, LBCELL..);
    let mut BEGIN: f64 = 0.0;
    let mut BUFFER = StackArray::<f64, 2>::new(1..=2);
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

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"ZZCKCVR3", ctx)?;
    }

    //
    // Get the number of intervals and pointing instances ( records )
    // in this segment, and from that determine the number of respective
    // directory epochs.
    //
    spicelib::DAFGDA(HANDLE, (ARREND - 1), ARREND, BUFFER.as_slice_mut(), ctx)?;

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
        spicelib::SETMSG(b"The requested segment in file # reports a length of # d.p. numbers, but the metadata in the segment indicates the length must either be # (no angular rate data) or # (angular rate data). Perhaps the segment is not type 3?", ctx);
        spicelib::ERRHAN(b"#", HANDLE, ctx)?;
        spicelib::ERRINT(b"#", SEGLEN, ctx);
        spicelib::ERRINT(b"#", NAVSLN, ctx);
        spicelib::ERRINT(b"#", AVSLN, ctx);
        spicelib::SIGERR(b"SPICE(BADCK3SEGMENT)", ctx)?;
        spicelib::CHKOUT(b"ZZCKCVR3", ctx)?;
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

    spicelib::DAFGDA(HANDLE, INTAT, INTAT, std::slice::from_mut(&mut START), ctx)?;
    spicelib::DAFGDA(HANDLE, TICKAT, TICKAT, std::slice::from_mut(&mut TICK), ctx)?;

    while ((TICK < START) && (TICKAT < LSTTIK)) {
        TICKAT = (TICKAT + 1);
        spicelib::DAFGDA(HANDLE, TICKAT, TICKAT, std::slice::from_mut(&mut TICK), ctx)?;
    }

    //
    // If we did not find a TICK at least as big as START, we can
    // just return now.
    //
    if (TICK < START) {
        spicelib::CHKOUT(b"ZZCKCVR3", ctx)?;
        return Ok(());
    }

    while ((INTAT <= LSTINT) && (TICKAT <= LSTTIK)) {
        //
        // At this point, we have an interval that begins at START
        // and ends at FINISH (unless of course we never found a "good"
        // TICK to start with.)
        //
        BEGIN = START;

        //
        // If the the start of the interval was the start of the LAST
        // interval available, we can short cut the remainder of the
        // reads.
        //
        if (INTAT == LSTINT) {
            spicelib::DAFGDA(
                HANDLE,
                LSTTIK,
                LSTTIK,
                std::slice::from_mut(&mut FINISH),
                ctx,
            )?;
            spicelib::WNINSD(START, FINISH, SCHEDL.as_slice_mut(), ctx)?;
            spicelib::CHKOUT(b"ZZCKCVR3", ctx)?;
            return Ok(());
        }

        //
        // This is the expected case.  Get the start of the next
        // interval.
        //
        INTAT = (INTAT + 1);
        spicelib::DAFGDA(HANDLE, INTAT, INTAT, std::slice::from_mut(&mut START), ctx)?;

        //
        // Read forward from the last tick until we reach the
        // START of the next interval or until we run out of TICKS.
        //
        while ((TICK < START) && (TICKAT < LSTTIK)) {
            FINISH = TICK;
            TICKAT = (TICKAT + 1);
            spicelib::DAFGDA(HANDLE, TICKAT, TICKAT, std::slice::from_mut(&mut TICK), ctx)?;
        }

        //
        // A structurally correct CK-3 segment should never allow
        // the next test to pass, but it's just easier to check than
        // police the writers of C-kernels.  The only way to get into
        // the block below is if TICKAT .EQ. LSTTIK
        //
        if (TICK < START) {
            FINISH = TICK;
            TICKAT = (TICKAT + 1);
        }

        //
        // Insert the interval into the window.
        //
        spicelib::WNINSD(BEGIN, FINISH, SCHEDL.as_slice_mut(), ctx)?;
    }

    spicelib::CHKOUT(b"ZZCKCVR3", ctx)?;
    Ok(())
}
