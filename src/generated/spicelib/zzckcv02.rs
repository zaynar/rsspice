//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const PSIZE: i32 = 8;
const BSIZE: i32 = 100;

//$Procedure ZZCKCV02 ( Private --- C-kernel segment coverage, type 02 )
pub fn ZZCKCV02(
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
    let mut ET: f64 = 0.0;
    let mut FINISH: f64 = 0.0;
    let mut FIRST = StackArray::<f64, 100>::new(1..=BSIZE);
    let mut LAST = StackArray::<f64, 100>::new(1..=BSIZE);
    let mut ARRSIZ: i32 = 0;
    let mut BEGAT: i32 = 0;
    let mut ENDAT: i32 = 0;
    let mut GET: i32 = 0;
    let mut GOT: i32 = 0;
    let mut NREC: i32 = 0;
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
        CHKIN(b"ZZCKCV02", ctx)?;
    }

    //
    // Check tolerance value.
    //
    if (TOL < 0.0) {
        SETMSG(b"Tolerance must be non-negative; actual value was #.", ctx);
        ERRDP(b"#", TOL, ctx);
        SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZCKCV02", ctx)?;
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
            CHKOUT(b"ZZCKCV02", ctx)?;
            return Ok(());
        }
    }

    //
    // Determine the size of the array and the number of records
    // in it.
    //
    ARRSIZ = ((ARREND - ARRBEG) + 1);
    NREC = intrinsics::IDNINT((((100.0 * (ARRSIZ as f64)) + 1.0) / 1001.0));

    //
    // The variable GOT tells us how many time endpoints we've
    // gotten so far.
    //
    GOT = 0;

    while (GOT < NREC) {
        GET = intrinsics::MIN0(&[BSIZE, (NREC - GOT)]);

        BEGAT = ((ARRBEG + (NREC * PSIZE)) + GOT);
        ENDAT = (((ARRBEG + (NREC * PSIZE)) + NREC) + GOT);

        //
        // Retrieve the list next list of windows.
        //
        DAFGDA(
            HANDLE,
            BEGAT,
            ((BEGAT + GET) - 1),
            FIRST.as_slice_mut(),
            ctx,
        )?;
        DAFGDA(HANDLE, ENDAT, ((ENDAT + GET) - 1), LAST.as_slice_mut(), ctx)?;

        //
        // Insert the coverage intervals into the schedule.
        //
        for I in 1..=GET {
            //
            // Adjust the interval using the tolerance.
            //
            BEGIN = FIRST[I];
            FINISH = LAST[I];

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

            WNINSD(BEGIN, FINISH, SCHEDL.as_slice_mut(), ctx)?;
        }

        GOT = (GOT + GET);
    }

    CHKOUT(b"ZZCKCV02", ctx)?;
    Ok(())
}
