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

//$Procedure ZZDDHRMU ( Private --- DDH Remove Unit )
pub fn ZZDDHRMU(
    UINDEX: i32,
    NFT: i32,
    UTCST: &mut [i32],
    UTHAN: &mut [i32],
    UTLCK: &mut [bool],
    UTLUN: &mut [i32],
    NUT: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut UTCST = DummyArrayMut::new(UTCST, 1..);
    let mut UTHAN = DummyArrayMut::new(UTHAN, 1..);
    let mut UTLCK = DummyArrayMut::new(UTLCK, 1..);
    let mut UTLUN = DummyArrayMut::new(UTLUN, 1..);

    //
    // Local Variables
    //

    //
    // First check to see if NUT is 0.  If so, just return, as there
    // are no rows to remove.
    //
    if (*NUT == 0) {
        return Ok(());
    }

    //
    // Check to see if we found the UINDEX in the unit table.
    // If not, use discovery check-in, signal an error and return.
    //
    if ((UINDEX > *NUT) || (UINDEX < 1)) {
        CHKIN(b"ZZDDHRMU", ctx)?;
        SETMSG(b"Attempt to remove row # from the unit table failed because valid row indices range from 1 to NUT.", ctx);
        ERRINT(b"#", UINDEX, ctx);
        ERRINT(b"#", *NUT, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZDDHRMU", ctx)?;
        return Ok(());
    }

    //
    // We have found the row we need to remove from the table.
    // Check to see whether we are to remove this row or simply
    // mark it as zero cost and reserve the unit.  We know this
    // is the case when NFT is greater than or equal to NUT.
    //
    if (NFT >= *NUT) {
        //
        // Zero the cost, clear the handle, and unlock the unit.
        //
        UTCST[UINDEX] = 0;
        UTHAN[UINDEX] = 0;
        UTLCK[UINDEX] = false;

        //
        // Reserve the unit for the handle manager's usage and
        // return.
        //
        RESLUN(UTLUN[UINDEX], ctx);
        return Ok(());
    }

    //
    // If we reach here, then we have to remove the row from the
    // unit table and compress.
    //
    for I in (UINDEX + 1)..=*NUT {
        UTCST[(I - 1)] = UTCST[I];
        UTHAN[(I - 1)] = UTHAN[I];
        UTLCK[(I - 1)] = UTLCK[I];
        UTLUN[(I - 1)] = UTLUN[I];
    }

    //
    // Decrement NUT.
    //
    *NUT = (*NUT - 1);

    Ok(())
}
