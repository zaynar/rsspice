//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const PSIZE: i32 = 8;
const BSIZE: i32 = 100;

//$Procedure ZZCKCVR2 ( Private --- C-kernel segment coverage, type 02 )
pub fn ZZCKCVR2(
    HANDLE: i32,
    ARRBEG: i32,
    ARREND: i32,
    SCHEDL: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut SCHEDL = DummyArrayMut::new(SCHEDL, LBCELL..);
    let mut FIRST = StackArray::<f64, 100>::new(1..=BSIZE);
    let mut LAST = StackArray::<f64, 100>::new(1..=BSIZE);
    let mut ARRSIZ: i32 = 0;
    let mut BEGAT: i32 = 0;
    let mut ENDAT: i32 = 0;
    let mut GET: i32 = 0;
    let mut GOT: i32 = 0;
    let mut NREC: i32 = 0;

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
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"ZZCKCVR2", ctx)?;
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
        spicelib::DAFGDA(
            HANDLE,
            BEGAT,
            ((BEGAT + GET) - 1),
            FIRST.as_slice_mut(),
            ctx,
        )?;
        spicelib::DAFGDA(HANDLE, ENDAT, ((ENDAT + GET) - 1), LAST.as_slice_mut(), ctx)?;

        //
        // Insert the coverage intervals into the schedule.
        //
        for I in 1..=GET {
            spicelib::WNINSD(FIRST[I], LAST[I], SCHEDL.as_slice_mut(), ctx)?;
        }

        GOT = (GOT + GET);
    }

    spicelib::CHKOUT(b"ZZCKCVR2", ctx)?;
    Ok(())
}
