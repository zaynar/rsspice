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

//$Procedure      ZZEKTRSB ( EK tree, identify siblings )
pub fn ZZEKTRSB(
    HANDLE: i32,
    TREE: i32,
    KEY: i32,
    LSIB: &mut i32,
    LKEY: &mut i32,
    RSIB: &mut i32,
    RKEY: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ADDRSS: i32 = 0;
    let mut BASE: i32 = 0;
    let mut KEYBAS: i32 = 0;
    let mut KIDBAS: i32 = 0;
    let mut LOFFST: i32 = 0;
    let mut LLPIDX: i32 = 0;
    let mut LPIDX: i32 = 0;
    let mut LPKEY: i32 = 0;
    let mut NKBAS: i32 = 0;
    let mut PARENT: i32 = 0;
    let mut PKEY: i32 = 0;
    let mut POFFST: i32 = 0;
    let mut ROFFST: i32 = 0;
    let mut RPIDX: i32 = 0;
    let mut RPKEY: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local variables
    //

    //
    // Start out by looking up the parent node.  We get LSIB
    // and RSIB for free.
    //
    ZZEKTRPI(
        HANDLE,
        TREE,
        KEY,
        &mut PARENT,
        &mut PKEY,
        &mut POFFST,
        &mut LPIDX,
        &mut LPKEY,
        LSIB,
        &mut RPIDX,
        &mut RPKEY,
        RSIB,
        ctx,
    )?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Set the base addresses for the child pointers and keys,
    // based on whether the parent is the root.
    //
    if (PARENT == TREE) {
        KEYBAS = TRKEYR;
        KIDBAS = TRKIDR;
        NKBAS = TRNKR;
    } else {
        KEYBAS = TRKEYC;
        KIDBAS = TRKIDC;
        NKBAS = TRNKC;
    }

    //
    // We need to find absolute keys in each sibling that exists.
    // To do this, we need the node offset of each sibling node.
    // That offset is the value of the parent key preceding each node,
    // plus the parent's offset.
    //
    if (LPIDX > 1) {
        //
        // The left parent key has a predecessor.  This predecessor is
        // the immediate predecessor of the left sibling node.
        //
        LLPIDX = (LPIDX - 1);
        BASE = ZZEKTRBS(PARENT, ctx)?;
        ADDRSS = ((BASE + KEYBAS) + LLPIDX);

        DASRDI(
            HANDLE,
            ADDRSS,
            ADDRSS,
            std::slice::from_mut(&mut LOFFST),
            ctx,
        )?;

        LOFFST = (LOFFST + POFFST);

        //
        // Get the first key from the left sibling.  Convert the key
        // to an absolute key.
        //
        BASE = ZZEKTRBS(*LSIB, ctx)?;
        ADDRSS = ((BASE + TRKEYC) + 1);

        DASRDI(HANDLE, ADDRSS, ADDRSS, std::slice::from_mut(LKEY), ctx)?;

        *LKEY = (*LKEY + LOFFST);
    } else if (LPIDX == 1) {
        //
        // The left parent key is the first key.  The left sibling has
        // no predecessor.
        //
        // Get the first key from the left sibling.  Convert the key
        // to an absolute key.
        //
        BASE = ZZEKTRBS(*LSIB, ctx)?;
        ADDRSS = ((BASE + TRKEYC) + 1);

        DASRDI(HANDLE, ADDRSS, ADDRSS, std::slice::from_mut(LKEY), ctx)?;

        *LKEY = (*LKEY + POFFST);
    } else {
        //
        // There's no left sibling.  Set the left sibling's key to a
        // value that won't be mistaken for a valid one.
        //
        *LKEY = 0;
    }

    //
    // LKEY is set.  It's time to produce an absolute key for the
    // right sibling.
    //
    if (RPIDX > 0) {
        //
        // The right parent key exists.  This key is the
        // immediate predecessor of the right sibling node.
        //
        ROFFST = (RPKEY + POFFST);

        //
        // Get the first key from the right sibling.  Convert the key
        // to an absolute key.
        //
        BASE = ZZEKTRBS(*RSIB, ctx)?;
        ADDRSS = ((BASE + TRKEYC) + 1);

        DASRDI(HANDLE, ADDRSS, ADDRSS, std::slice::from_mut(RKEY), ctx)?;

        *RKEY = (*RKEY + ROFFST);
    } else {
        //
        // There's no right sibling.  Set the right sibling's key to a
        // value that won't be mistaken for a valid one.
        //
        *RKEY = 0;
    }
    //
    // All outputs are set.
    //

    Ok(())
}
