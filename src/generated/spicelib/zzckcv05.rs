//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const C05TP0: i32 = 0;
const C05TP1: i32 = (C05TP0 + 1);
const C05TP2: i32 = (C05TP1 + 1);
const C05TP3: i32 = (C05TP2 + 1);
const C05PS0: i32 = 8;
const C05PS1: i32 = 4;
const C05PS2: i32 = 14;
const C05PS3: i32 = 7;
const LBCELL: i32 = -5;

//$Procedure ZZCKCV05 ( Private --- C-kernel segment coverage, type 05 )
pub fn ZZCKCV05(
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
    let mut BUFFER = StackArray::<f64, 4>::new(1..=4);
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
    let mut SUBTYP: i32 = 0;
    let mut TICKAT: i32 = 0;
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
        CHKIN(b"ZZCKCV05", ctx)?;
    }

    //
    // Check tolerance value.
    //
    if (TOL < 0.0) {
        SETMSG(b"Tolerance must be non-negative; actual value was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZCKCV05", ctx)?;
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
            CHKOUT(b"ZZCKCV05", ctx)?;
            return Ok(());
        }
    }

    //
    // Get the meta-data associated with this segment that we
    // require to produce the schedule.
    //
    // BUFFER(1) = Subtype Code
    // BUFFER(2) = Window Size
    // BUFFER(3) = Number of Interpolation Intervals
    // BUFFER(4) = Number of Packets
    //
    DAFGDA(HANDLE, (ARREND - 3), ARREND, BUFFER.as_slice_mut(), ctx)?;

    SUBTYP = intrinsics::IDNINT(BUFFER[1]);
    INVLS = intrinsics::IDNINT(BUFFER[3]);
    NREC = intrinsics::IDNINT(BUFFER[4]);
    NDIR = ((NREC - 1) / 100);

    //
    // Compute the packet size.  This requires parameters listed
    // in the include file 'ck05.inc' and is based on the subtype.
    //
    if (SUBTYP == C05TP0) {
        RSIZE = C05PS0;
    } else if (SUBTYP == C05TP1) {
        RSIZE = C05PS1;
    } else if (SUBTYP == C05TP2) {
        RSIZE = C05PS2;
    } else if (SUBTYP == C05TP3) {
        RSIZE = C05PS3;
    } else {
        SETMSG(b"CK type 5 subtype <#> is not supported.", ctx);
        ERRINT(b"#", SUBTYP, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZCKCV05", ctx)?;
        return Ok(());
    }

    //
    // Recall that the segment is layed out as:
    //
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
    //   |    Seconds per tick    |
    //   +------------------------+
    //   |      Subtype code      |
    //   +------------------------+
    //   |      Window size       |
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
        CHKOUT(b"ZZCKCV05", ctx)?;
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
            // A structurally correct CK-5 segment should never allow the
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
        // Truncate the interval using the segment bounds.
        //
        BEGIN = intrinsics::DMAX1(&[BEGIN, DC[1]]);
        FINISH = intrinsics::DMIN1(&[FINISH, DC[2]]);

        //
        // Adjust the interval using the tolerance.  Empty
        // intervals *do not get expanded*; this choice is
        // consistent with the type 5 reading algorithm.
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
        }

        //
        // Insert the interval into the window.
        //
        if (BEGIN <= FINISH) {
            WNINSD(BEGIN, FINISH, SCHEDL.as_slice_mut(), ctx)?;
        }
    }

    CHKOUT(b"ZZCKCV05", ctx)?;
    Ok(())
}
