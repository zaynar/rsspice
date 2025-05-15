//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;

//$Procedure ZZDDHGTU ( Private --- DDH Get Unit )
pub fn ZZDDHGTU(
    UTCST: &mut [i32],
    UTHAN: &mut [i32],
    UTLCK: &mut [bool],
    UTLUN: &mut [i32],
    NUT: &mut i32,
    UINDEX: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut UTCST = DummyArrayMut::new(UTCST, 1..);
    let mut UTHAN = DummyArrayMut::new(UTHAN, 1..);
    let mut UTLCK = DummyArrayMut::new(UTLCK, 1..);
    let mut UTLUN = DummyArrayMut::new(UTLUN, 1..);
    let mut I: i32 = 0;
    let mut ORDERV = StackArray::<i32, 23>::new(1..=UTSIZE);
    let mut DONE: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE discovery error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // First check the case when the unit table is completely empty.
    //
    if (*NUT == 0) {
        *NUT = 1;
        *UINDEX = 1;

        UTCST[*UINDEX] = 0;
        UTHAN[*UINDEX] = 0;
        UTLCK[*UINDEX] = false;

        GETLUN(&mut UTLUN[*UINDEX], ctx)?;

        //
        // Check FAILED to see if GETLUN signaled an error.  If so, then
        // return an invalid unit to the caller.
        //
        if FAILED(ctx) {
            UTLUN[*UINDEX] = -1;
            return Ok(());
        }

        //
        // If we end up here, then GETLUN succeeded and we have the new
        // unit.  Now return.
        //
        return Ok(());
    }

    //
    // If we reach here, then the table contains at least one entry.
    // Order the table rows by cost.
    //
    ORDERI(UTCST.as_slice(), *NUT, ORDERV.as_slice_mut());

    //
    // Now check to for '0' cost rows as this indicates rows whose
    // logical units are reserved for this suite of routines usage,
    // but are not currently assigned a file.
    //
    if (UTCST[ORDERV[1]] <= 0) {
        *UINDEX = ORDERV[1];

        //
        // '0' cost rows end up in the unit table as the result of a
        // row deletion, occurring when excess files are present.
        // When this process occurs, the logical unit listed in this
        // row is reserved for this module's usage only with RESLUN.
        // Free it, since we're about to reassign it.
        //
        FRELUN(UTLUN[*UINDEX], ctx);

        return Ok(());
    }

    //
    // Now if no '0' cost rows exist, check to see if we can
    // expand the table.
    //
    if (*NUT < UTSIZE) {
        //
        // Now increment NUT and set UINDEX.
        //
        *NUT = (*NUT + 1);
        *UINDEX = *NUT;

        //
        // Prepare the default values for the new row.
        //
        UTCST[*UINDEX] = 0;
        UTHAN[*UINDEX] = 0;
        UTLCK[*UINDEX] = false;

        GETLUN(&mut UTLUN[*UINDEX], ctx)?;

        //
        // Check FAILED to see if GETLUN signaled an error.  If so, then
        // return an invalid unit to the caller.
        //
        if FAILED(ctx) {
            UTLUN[*UINDEX] = -1;
            return Ok(());
        }

        //
        // If we end up here, then GETLUN worked properly.  Now return.
        //
        return Ok(());
    }

    //
    // If we reach here, then we have no zero-cost rows and a full unit
    // table.  Now it's time to determine which entry in the table to
    // bump.  We do this by stepping through the order vector until
    // we find the first 'non-locked' row.
    //
    I = 0;
    DONE = false;

    while (!DONE && (I != *NUT)) {
        I = (I + 1);
        DONE = !UTLCK[ORDERV[I]];
    }

    //
    // Before going any further, signal an error if we discover
    // we have not found a row.
    //
    if !DONE {
        *UINDEX = 0;
        CHKIN(b"ZZDDHGTU", ctx)?;
        SETMSG(b"The unit table is full and all entries are locked.  This should never happen. Contact NAIF.", ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZDDHGTU", ctx)?;
        return Ok(());
    }

    //
    // Clear UTCST and UTHAN since we intend to disconnect
    // the unit upon return.
    //
    UTCST[ORDERV[I]] = 0;
    UTHAN[ORDERV[I]] = 0;

    //
    // Set UINDEX and CLSLUN, then return.
    //
    *UINDEX = ORDERV[I];

    //
    // At this point we need to close the unit from the row of interest.
    //
    {
        use f2rust_std::io;

        let specs = io::CloseSpecs {
            unit: Some(UTLUN[*UINDEX]),
            ..Default::default()
        };
        ctx.close(specs)?;
    }

    Ok(())
}
