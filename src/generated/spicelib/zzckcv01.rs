//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const BUFSIZ: i32 = 100;
const QAVSIZ: i32 = 7;
const QSIZ: i32 = 4;

//$Procedure ZZCKCV01 ( Private --- C-kernel segment coverage, type 01 )
pub fn ZZCKCV01(
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
    let mut BUFFER = StackArray::<f64, 100>::new(1..=BUFSIZ);
    let mut ET: f64 = 0.0;
    let mut FINISH: f64 = 0.0;
    let mut N: i32 = 0;
    let mut NREC: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut PSIZ: i32 = 0;
    let mut REMAIN: i32 = 0;
    let mut TBASE: i32 = 0;
    let mut NAVSLN: i32 = 0;
    let mut AVSLN: i32 = 0;
    let mut SEGLEN: i32 = 0;
    let mut ISTDB: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
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
        CHKIN(b"ZZCKCV01", ctx)?;
    }

    //
    // Check tolerance value.
    //
    if (TOL < 0.0) {
        SETMSG(b"Tolerance must be non-negative; actual value was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZCKCV01", ctx)?;
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
            CHKOUT(b"ZZCKCV01", ctx)?;
            return Ok(());
        }
    }

    //
    // The coverage window is the set of singleton intervals consisting
    // of the epochs of the pointing records. We'll need to find the
    // epochs.
    //
    // First, get the number of records in this segment.
    //
    DAFGDA(HANDLE, ARREND, ARREND, BUFFER.as_slice_mut(), ctx)?;

    NREC = (BUFFER[1] as i32);

    //
    // Determine the size of the pointing packets. This is dependent
    // on whether angular rate data is present in the segment or not.
    // We can determine this with the following computation:
    //
    // Assume a record size of 4, i.e. no angular rate data.
    //
    NAVSLN = (((5 * NREC) + ((NREC - 1) / 100)) + 1);

    //
    // Assume a record size of 7, i.e. angular rate data.
    //
    AVSLN = (((8 * NREC) + ((NREC - 1) / 100)) + 1);

    //
    // Compute the actual length of the segment.
    //
    SEGLEN = ((ARREND - ARRBEG) + 1);

    if (SEGLEN == NAVSLN) {
        PSIZ = QSIZ;
    } else if (SEGLEN == AVSLN) {
        PSIZ = QAVSIZ;
    } else {
        SETMSG(b"The requested segment in file # reports a length of # d.p. numbers, but the metadata in the segment indicates the length must either be # (no angular rate data) or # (angular rate data). Perhaps the segment is not type 1?", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", SEGLEN, ctx);
        ERRINT(b"#", NAVSLN, ctx);
        ERRINT(b"#", AVSLN, ctx);
        SIGERR(b"SPICE(BADCK1SEGMENT)", ctx)?;
        CHKOUT(b"ZZCKCV01", ctx)?;
        return Ok(());
    }

    //
    // The epochs start right after the pointing data. Let TBASE be the
    // address preceding the first epoch.
    //
    TBASE = ((ARRBEG + (NREC * PSIZ)) - 1);

    //
    // Grab the epochs. Make a singleton interval out of each one; add
    // the interval to the coverage window.
    //
    // For efficiency, we'll read the epochs into a buffer of length
    // BUFSIZ.
    //
    REMAIN = NREC;
    OFFSET = 0;

    while (REMAIN > 0) {
        //
        // Buffer the next set of epochs.
        //
        N = intrinsics::MIN0(&[BUFSIZ, REMAIN]);

        DAFGDA(
            HANDLE,
            ((TBASE + OFFSET) + 1),
            ((TBASE + OFFSET) + N),
            BUFFER.as_slice_mut(),
            ctx,
        )?;

        //
        // Insert the current batch of N singleton intervals.
        //
        for I in 1..=N {
            BEGIN = BUFFER[I];
            FINISH = BUFFER[I];

            if (TOL > 0.0) {
                //
                // Adjust the interval using the tolerance.
                //
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

            WNINSD(BEGIN, FINISH, SCHEDL.as_slice_mut(), ctx)?;
        }

        OFFSET = (OFFSET + N);
        REMAIN = (REMAIN - N);
    }

    CHKOUT(b"ZZCKCV01", ctx)?;
    Ok(())
}
