//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;

//$Procedure ZZCOREPC ( Correct epoch for aberration )
pub fn ZZCOREPC(
    ABCORR: &[u8],
    ET: f64,
    LT: f64,
    ETCORR: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut CORBLK = StackArray::<bool, 6>::new(1..=ABATSZ);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZCOREPC", ctx)?;

    //
    // Parse the aberration correction string.  Obtain a correction
    // attribute block.
    //
    ZZPRSCOR(ABCORR, CORBLK.as_slice_mut(), ctx)?;

    if CORBLK[LTIDX] {
        //
        // Light time corrections are used.  The output epoch
        // must be adjusted according to whether the correction
        // is for received or transmitted radiation.
        //
        if CORBLK[XMTIDX] {
            //
            // This is the transmission case.
            //
            *ETCORR = (ET + LT);
        } else {
            //
            // This is the reception case.
            //
            *ETCORR = (ET - LT);
        }
    } else {
        //
        // Light time corrections are not used.
        //
        *ETCORR = ET;
    }

    CHKOUT(b"ZZCOREPC", ctx)?;
    Ok(())
}
