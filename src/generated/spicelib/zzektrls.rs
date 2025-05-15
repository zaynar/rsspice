//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MXKIDC: i32 = 63;
const MXKEYC: i32 = (MXKIDC - 1);
const MNKIDC: i32 = (((2 * MXKIDC) + 1) / 3);
const MNKEYC: i32 = (MNKIDC - 1);
const MXKIDR: i32 = ((2 * (((2 * MXKIDC) - 2) / 3)) + 1);
const MXKEYR: i32 = (MXKIDR - 1);
const MNKIDR: i32 = 2;
const TRTYPE: i32 = 1;
const TRVERS: i32 = 1;
const TRNNOD: i32 = (TRTYPE + 1);
const TRNKEY: i32 = (TRNNOD + 1);
const TRDPTH: i32 = (TRNKEY + 1);
const TRNKR: i32 = (TRDPTH + 1);
const TRKEYR: i32 = TRNKR;
const TRKIDR: i32 = ((TRKEYR + MXKEYR) + 1);
const TRDATR: i32 = ((TRKIDR + MXKIDR) + 1);
const TRSIZR: i32 = ((TRDATR + MXKEYR) + 1);
const TRNKC: i32 = 1;
const TRKEYC: i32 = TRNKC;
const TRKIDC: i32 = ((TRKEYC + MXKEYC) + 1);
const TRDATC: i32 = ((TRKIDC + MXKIDC) + 1);
const TRSIZC: i32 = ((TRDATC + MXKEYC) + 1);
const TRMXDP: i32 = 10;

//$Procedure      ZZEKTRLS ( EK tree, linear search )
pub fn ZZEKTRLS(HANDLE: i32, TREE: i32, IVAL: i32, ctx: &mut Context) -> f2rust_std::Result<i32> {
    let mut ZZEKTRLS: i32 = 0;
    let mut N: i32 = 0;
    let mut VALUE: i32 = 0;

    //
    // Non-SPICELIB functions
    //
    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    ZZEKTRLS = 0;
    N = ZZEKTRSZ(HANDLE, TREE, ctx)?;

    for I in 1..=N {
        ZZEKTRDP(HANDLE, TREE, I, &mut VALUE, ctx)?;

        if (IVAL == VALUE) {
            ZZEKTRLS = I;
            return Ok(ZZEKTRLS);
        }
    }

    Ok(ZZEKTRLS)
}
