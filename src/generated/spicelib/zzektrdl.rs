//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const EPARCH: i32 = 1;
const EPNIPT: i32 = 5;
const EPPSZC: i32 = (EPARCH + 1);
const EPBASC: i32 = (EPPSZC + 1);
const EPNPC: i32 = (EPBASC + 1);
const EPNFPC: i32 = (EPNPC + 1);
const EPFPC: i32 = (EPNFPC + 1);
const EPPSZD: i32 = (EPPSZC + EPNIPT);
const EPBASD: i32 = (EPPSZD + 1);
const EPNPD: i32 = (EPBASD + 1);
const EPNFPD: i32 = (EPNPD + 1);
const EPFPD: i32 = (EPNFPD + 1);
const EPPSZI: i32 = (EPPSZD + EPNIPT);
const EPBASI: i32 = (EPPSZI + 1);
const EPNPI: i32 = (EPBASI + 1);
const EPNFPI: i32 = (EPNPI + 1);
const EPFPI: i32 = (EPNFPI + 1);
const EPMDSZ: i32 = (1 + (3 * EPNIPT));
const PGSIZC: i32 = 1024;
const PGSIZD: i32 = 128;
const PGSIZI: i32 = 256;
const PGBASC: i32 = 0;
const PGBASD: i32 = 0;
const PGBASI: i32 = 256;
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
const TERM: i32 = 1;
const LCHECK: i32 = (TERM + 1);
const RCHECK: i32 = (LCHECK + 1);
const BALNCE: i32 = (RCHECK + 1);
const MERG32: i32 = (BALNCE + 1);
const MERG31: i32 = (MERG32 + 1);
const LLCHCK: i32 = (MERG31 + 1);
const RRCHCK: i32 = (LLCHCK + 1);

//$Procedure      ZZEKTRDL ( EK tree, delete value )
pub fn ZZEKTRDL(HANDLE: i32, TREE: i32, KEY: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IDX: i32 = 0;
    let mut LEFT: i32 = 0;
    let mut LEVEL: i32 = 0;
    let mut LKEY: i32 = 0;
    let mut LLKEY: i32 = 0;
    let mut LLSIB: i32 = 0;
    let mut LNKEY: i32 = 0;
    let mut LNODE: i32 = 0;
    let mut LPIDX: i32 = 0;
    let mut LPKEY: i32 = 0;
    let mut LRKEY: i32 = 0;
    let mut LRSIB: i32 = 0;
    let mut LSIB: i32 = 0;
    let mut MNODE: i32 = 0;
    let mut NKEYS: i32 = 0;
    let mut NODE: i32 = 0;
    let mut NOFFST: i32 = 0;
    let mut PARENT: i32 = 0;
    let mut PKEY: i32 = 0;
    let mut POFFST: i32 = 0;
    let mut PTR: i32 = 0;
    let mut RIGHT: i32 = 0;
    let mut RKEY: i32 = 0;
    let mut RLKEY: i32 = 0;
    let mut RLSIB: i32 = 0;
    let mut RNODE: i32 = 0;
    let mut ROOT: i32 = 0;
    let mut RPIDX: i32 = 0;
    let mut RPKEY: i32 = 0;
    let mut RRKEY: i32 = 0;
    let mut RRSIB: i32 = 0;
    let mut RSIB: i32 = 0;
    let mut STATE: i32 = 0;
    let mut TRGKEY: i32 = 0;
    let mut TRUST: i32 = 0;
    let mut UNDRFL: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in for speed.
    //
    // Set the variable ROOT, so we'll have something mnemonic to go
    // by when referring to the root node.
    //
    ROOT = TREE;

    //
    // Work with a local copy of the input key.
    //
    LKEY = KEY;

    //
    // The first step is to delete the key from the tree without
    // balancing.  This step may cause a node to underflow.  We'll
    // handle the underflow later.
    //
    ZZEKTRUD(HANDLE, TREE, LKEY, &mut TRGKEY, &mut UNDRFL, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // If the deletion didn't result in an underflow, we're done.
    //
    if !UNDRFL {
        return Ok(());
    }

    //
    // Handle node underflows, as required.  We describe our approach
    // below.  If any step fails, we try the next step.  We proceed
    // until we succeed in resolving the underflow.
    //
    //    1) If an immediate sibling can contribute a key, balance NODE
    //       with that sibling.
    //
    //    2) If both left and right siblings exist, but neither can
    //       contribute a key, execute a 3-2 merge.
    //
    //    3) If the left sibling has its own left sibling, and if that
    //       second left sibling can contribute a key, rotate a key
    //       from that sibling into NODE's left sibling.  Then execute
    //       (1).
    //
    //    4) If the left sibling has its own left sibling, and if that
    //       second left sibling cannot contribute a key, execute a 3-2
    //       merge using NODE as the rightmost child.
    //
    //    5) Same as (3), except on the right side.
    //
    //    6) Same as (4), except on the right side.
    //
    //    7) Arrival at this step implies that NODE is a child of the
    //       root and has one sibling.  Execute a 3-1 merge.
    //

    STATE = LCHECK;

    while (STATE != TERM) {
        if (STATE == LCHECK) {
            //
            // Look up the node containing the target key TRGKEY.  This
            // is where the underflow occurred; note that this node may
            // be different from the one that contained LKEY.
            //
            ZZEKTRLK(
                HANDLE,
                TREE,
                TRGKEY,
                &mut IDX,
                &mut NODE,
                &mut NOFFST,
                &mut LEVEL,
                &mut PTR,
                ctx,
            )?;

            //
            // Look up the siblings of NODE.  If either sibling exists
            // and has a surplus of keys, we can remove the underflow
            // by balancing.
            //
            ZZEKTRSB(
                HANDLE, TREE, TRGKEY, &mut LSIB, &mut LKEY, &mut RSIB, &mut RKEY, ctx,
            )?;

            if (LSIB > 0) {
                NKEYS = ZZEKTRNK(HANDLE, TREE, LSIB, ctx)?;

                if (NKEYS > MNKEYC) {
                    //
                    // The left sibling can contribute a key.
                    //
                    LNKEY = LKEY;
                    LNODE = LSIB;
                    RNODE = NODE;

                    STATE = BALNCE;
                } else if (RSIB > 0) {
                    //
                    // The left sibling cannot help with balancing, but
                    // the right sibling may be able to.
                    //
                    STATE = RCHECK;
                } else {
                    //
                    // The right sibling does not exist; the only chance
                    // of balancing will come from the left sibling of
                    // LSIB, if such a sibling exists.
                    //
                    STATE = LLCHCK;
                }
            } else {
                //
                // There is no left sibling, so there must be a right
                // sibling.  Examine it.
                //
                STATE = RCHECK;
            }
        } else if (STATE == RCHECK) {
            //
            // See whether there's a node surplus in the right sibling
            // The left sibling has already been checked and found wanting,
            // or wasn't found at all.
            //
            NKEYS = ZZEKTRNK(HANDLE, TREE, RSIB, ctx)?;

            if (NKEYS > MNKEYC) {
                //
                // The right sibling can contribute a key.
                //
                LNKEY = TRGKEY;
                LNODE = NODE;
                RNODE = RSIB;

                STATE = BALNCE;
            } else if (LSIB > 0) {
                //
                // NODE has siblings on both sides, and each one contains
                // the minimum number of keys.  Execute a 3-2 merge.
                //
                LNKEY = LKEY;
                LNODE = LSIB;
                MNODE = NODE;
                RNODE = RSIB;

                STATE = MERG32;
            } else {
                //
                // Look for the right sibling of the right sibling.
                //
                STATE = RRCHCK;
            }
        } else if (STATE == LLCHCK) {
            //
            // See whether the left sibling has its own left sibling.
            //
            ZZEKTRSB(
                HANDLE, TREE, LKEY, &mut LLSIB, &mut LLKEY, &mut LRSIB, &mut LRKEY, ctx,
            )?;

            if (LLSIB > 0) {
                NKEYS = ZZEKTRNK(HANDLE, TREE, LLSIB, ctx)?;

                if (NKEYS > MNKEYC) {
                    //
                    // The left**2 sibling can contribute a key.  Rotate
                    // this key into the left sibling.  We'll need the
                    // parent and index of left parent key of LSIB in order
                    // to do this rotation.
                    //
                    ZZEKTRPI(
                        HANDLE,
                        TREE,
                        LKEY,
                        &mut PARENT,
                        &mut PKEY,
                        &mut POFFST,
                        &mut LPIDX,
                        &mut LPKEY,
                        &mut LLSIB,
                        &mut RPIDX,
                        &mut RPKEY,
                        &mut LRSIB,
                        ctx,
                    )?;

                    ZZEKTRRK(HANDLE, TREE, LLSIB, LSIB, PARENT, LPIDX, 1, ctx)?;

                    //
                    // Now LSIB has a one-key surplus, so we can balance
                    // LSIB and NODE.
                    //
                    LNKEY = LKEY;
                    LNODE = LSIB;
                    RNODE = NODE;

                    STATE = BALNCE;
                } else {
                    //
                    // The left**2 sibling contains the minimum allowed
                    // number of keys.  Execute a 3-2 merge, with NODE
                    // as the right node.
                    //
                    LNKEY = LLKEY;
                    LNODE = LLSIB;
                    MNODE = LSIB;
                    RNODE = NODE;

                    STATE = MERG32;
                }
            } else {
                //
                // LSIB and NODE are the only children of their parent.
                // The parent must be the root.  Also, LSIB and NODE
                // together contain the one less than twice the minimum
                // allowed number of keys.  Execute a 3-1 merge.
                //
                LNODE = LSIB;
                RNODE = NODE;

                STATE = MERG31;
            }
        } else if (STATE == RRCHCK) {
            //
            // See whether the right sibling has its own right sibling.
            //
            ZZEKTRSB(
                HANDLE, TREE, RKEY, &mut RLSIB, &mut RLKEY, &mut RRSIB, &mut RRKEY, ctx,
            )?;

            if (RRSIB > 0) {
                NKEYS = ZZEKTRNK(HANDLE, TREE, RRSIB, ctx)?;

                if (NKEYS > MNKEYC) {
                    //
                    // The right**2 sibling can contribute a key.  Rotate
                    // this key into the right sibling.  We'll need the
                    // parent and index of the right parent key of RSIB in
                    // order to do this rotation.
                    //
                    ZZEKTRPI(
                        HANDLE,
                        TREE,
                        RKEY,
                        &mut PARENT,
                        &mut PKEY,
                        &mut POFFST,
                        &mut LPIDX,
                        &mut LPKEY,
                        &mut RLSIB,
                        &mut RPIDX,
                        &mut RPKEY,
                        &mut RRSIB,
                        ctx,
                    )?;

                    ZZEKTRRK(HANDLE, TREE, RSIB, RRSIB, PARENT, RPIDX, -1, ctx)?;

                    //
                    // Now RSIB has a one-key surplus, so we can balance
                    // RSIB and NODE.
                    //
                    LNKEY = TRGKEY;
                    LNODE = NODE;
                    RNODE = RSIB;

                    STATE = BALNCE;
                } else {
                    //
                    // The right**2 sibling contains the minimum allowed
                    // number of keys.  Execute a 3-2 merge, with NODE
                    // as the left node.
                    //
                    LNKEY = TRGKEY;
                    LNODE = NODE;
                    MNODE = RSIB;
                    RNODE = RRSIB;

                    STATE = MERG32;
                }
            } else {
                //
                // RSIB and NODE are the only children of their parent.
                // The parent must be the root.  Also, RSIB and NODE
                // together contain one less than twice the minimum allowed
                // number of keys.  Execute a 3-1 merge.
                //
                LNODE = NODE;
                RNODE = RSIB;

                STATE = MERG31;
            }
        } else if (STATE == BALNCE) {
            //
            // LNODE has a right sibling, and between the two nodes,
            // there are enough keys to accommodate the underflow.  After
            // balancing these nodes, we're done.
            //
            ZZEKTRPI(
                HANDLE,
                TREE,
                LNKEY,
                &mut PARENT,
                &mut PKEY,
                &mut POFFST,
                &mut LPIDX,
                &mut LPKEY,
                &mut RLSIB,
                &mut RPIDX,
                &mut RPKEY,
                &mut RRSIB,
                ctx,
            )?;

            //
            // The common parent of the nodes is PARENT.  The right parent
            // key of the left node is at location RPIDX.  We're ready to
            // balance the nodes.
            //
            ZZEKTRBN(HANDLE, TREE, LNODE, RNODE, PARENT, RPIDX, ctx)?;

            STATE = TERM;
        } else if (STATE == MERG32) {
            //
            // LNODE, MNODE, and RNODE are siblings, and between the three
            // nodes, there's an underflow of one key.  Merge these three
            // nodes into two.  This merging process removes a key from the
            // parent; the parent may underflow as a result.
            //
            // After executing the 3-2 merge, to ensure that we reference
            // the parent correctly, we'll obtain a fresh key from the
            // parent.
            //
            // To start with, we'll get a trusted key from the
            // leftmost node LNODE.  The first key of LNODE won't be
            // touched by the merge.
            //
            ZZEKTRKI(HANDLE, TREE, LNKEY, 1, &mut TRUST, ctx)?;

            ZZEKTRPI(
                HANDLE,
                TREE,
                LNKEY,
                &mut PARENT,
                &mut PKEY,
                &mut POFFST,
                &mut LPIDX,
                &mut LPKEY,
                &mut RLSIB,
                &mut RPIDX,
                &mut RPKEY,
                &mut RRSIB,
                ctx,
            )?;

            //
            // The right parent key of the left node is the left parent
            // key of the middle node.  The index of this key is required
            // by ZZEKTR32.
            //
            ZZEKTR32(
                HANDLE,
                TREE,
                LNODE,
                MNODE,
                RNODE,
                PARENT,
                RPIDX,
                &mut UNDRFL,
                ctx,
            )?;

            if UNDRFL {
                //
                // We'll need to handle underflow in the parent.
                // The parent should be correctly identified by the
                // parent of TRUST.
                //
                // Note that a 3-2 merge can't create an underflow in
                // the parent if the parent is the root:  the parent
                // contains at least one key after this merge.
                //
                ZZEKTRPI(
                    HANDLE,
                    TREE,
                    TRUST,
                    &mut PARENT,
                    &mut PKEY,
                    &mut POFFST,
                    &mut LPIDX,
                    &mut LPKEY,
                    &mut LEFT,
                    &mut RPIDX,
                    &mut RPKEY,
                    &mut RIGHT,
                    ctx,
                )?;

                TRGKEY = PKEY;
                STATE = LCHECK;
            } else {
                STATE = TERM;
            }
        } else if (STATE == MERG31) {
            //
            // We've got an underflow in the two children of the root.
            // Move all of the keys from these children into the root.
            // The root contains the maximum allowed number of keys
            // after this merge.
            //
            ZZEKTR31(HANDLE, TREE, ctx)?;
            STATE = TERM;
        }
    }

    Ok(())
}
