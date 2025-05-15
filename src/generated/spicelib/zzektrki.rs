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

//$Procedure      ZZEKTRKI ( EK tree, look up key by index )
pub fn ZZEKTRKI(
    HANDLE: i32,
    TREE: i32,
    NODKEY: i32,
    N: i32,
    KEY: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ADDRSS: i32 = 0;
    let mut BASE: i32 = 0;
    let mut IDX: i32 = 0;
    let mut LEVEL: i32 = 0;
    let mut NODE: i32 = 0;
    let mut NOFFST: i32 = 0;
    let mut PTR: i32 = 0;
    let mut SIZE: i32 = 0;

    //
    // Functions
    //

    //
    // Local variables
    //

    ZZEKTRLK(
        HANDLE,
        TREE,
        NODKEY,
        &mut IDX,
        &mut NODE,
        &mut NOFFST,
        &mut LEVEL,
        &mut PTR,
        ctx,
    )?;

    SIZE = ZZEKTRNK(HANDLE, TREE, NODE, ctx)?;

    //
    // Reject bad indices.
    //
    if ((N < 0) || (N > SIZE)) {
        CHKIN(b"ZZEKTRKI", ctx)?;
        SETMSG(b"Key index = #; valid range in node # is 1:#", ctx);
        ERRINT(b"#", N, ctx);
        ERRINT(b"#", NODE, ctx);
        ERRINT(b"#", SIZE, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKTRKI", ctx)?;
        return Ok(());
    }

    BASE = ZZEKTRBS(NODE, ctx)?;

    if (LEVEL == 1) {
        ADDRSS = ((BASE + TRKEYR) + N);
    } else {
        ADDRSS = ((BASE + TRKEYC) + N);
    }

    DASRDI(HANDLE, ADDRSS, ADDRSS, std::slice::from_mut(KEY), ctx)?;

    //
    // Map the key from relative to absolute.
    //
    *KEY = (*KEY + NOFFST);

    Ok(())
}
