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

//$Procedure      ZZEKTRBN ( EK tree, balance nodes )
pub fn ZZEKTRBN(
    HANDLE: i32,
    TREE: i32,
    LEFT: i32,
    RIGHT: i32,
    PARENT: i32,
    PKIDX: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut LNKEYS: i32 = 0;
    let mut RNKEYS: i32 = 0;
    let mut ROOT: i32 = 0;
    let mut SCHLEP: i32 = 0;
    let mut SUM: i32 = 0;

    //
    // Non-SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in for speed.
    //
    ROOT = TREE;

    if ((LEFT == ROOT) || (RIGHT == ROOT)) {
        CHKIN(b"ZZEKTRBN", ctx)?;
        SETMSG(b"Input node is root; only children can be balanced.", ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTRBN", ctx)?;
    }

    //
    // Get the key counts for the left and right nodes.
    //
    LNKEYS = ZZEKTRNK(HANDLE, TREE, LEFT, ctx)?;
    RNKEYS = ZZEKTRNK(HANDLE, TREE, RIGHT, ctx)?;

    //
    // Balancing the nodes should give each of them a key count in
    // the range of
    //
    //    MNKEYC : MXKEYC
    //
    // If that's not possible, we have a serious problem.
    //

    SUM = (LNKEYS + RNKEYS);

    if ((SUM > (2 * MXKEYC)) || (SUM < (2 * MNKEYC))) {
        CHKIN(b"ZZEKTRBN", ctx)?;
        SETMSG(b"Node # and right sibling # contain # and # keys respectively; count sum should be in range #:#.", ctx);
        ERRINT(b"#", LEFT, ctx);
        ERRINT(b"#", RIGHT, ctx);
        ERRINT(b"#", LNKEYS, ctx);
        ERRINT(b"#", RNKEYS, ctx);
        ERRINT(b"#", (2 * MNKEYC), ctx);
        ERRINT(b"#", (2 * MXKEYC), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTRBN", ctx)?;
        return Ok(());
    }

    //
    // Now, the actions we take depend on whether we must schlep keys
    // to the right or left.
    //
    if (LNKEYS > RNKEYS) {
        SCHLEP = (LNKEYS - ((SUM + 1) / 2));
    } else if (LNKEYS < RNKEYS) {
        SCHLEP = -(RNKEYS - ((SUM + 1) / 2));
    } else {
        SCHLEP = 0;
    }

    //
    // Rotate the requested number of keys.
    //
    ZZEKTRRK(HANDLE, TREE, LEFT, RIGHT, PARENT, PKIDX, SCHLEP, ctx)?;

    Ok(())
}
