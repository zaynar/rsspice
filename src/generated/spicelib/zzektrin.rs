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
const SPLT23: i32 = (BALNCE + 1);
const SPLT13: i32 = (SPLT23 + 1);

//$Procedure      ZZEKTRIN ( EK tree, insert value )
pub fn ZZEKTRIN(
    HANDLE: i32,
    TREE: i32,
    KEY: i32,
    VALUE: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut IDX: i32 = 0;
    let mut LEFT: i32 = 0;
    let mut LEVEL: i32 = 0;
    let mut LKEY: i32 = 0;
    let mut LNODE: i32 = 0;
    let mut LPIDX: i32 = 0;
    let mut LPKEY: i32 = 0;
    let mut LVAL: i32 = 0;
    let mut NKEYS: i32 = 0;
    let mut NODE: i32 = 0;
    let mut NOFFST: i32 = 0;
    let mut NSIZE: i32 = 0;
    let mut PARENT: i32 = 0;
    let mut PKEY: i32 = 0;
    let mut PKIDX: i32 = 0;
    let mut POFFST: i32 = 0;
    let mut RIGHT: i32 = 0;
    let mut RNODE: i32 = 0;
    let mut ROOT: i32 = 0;
    let mut RPIDX: i32 = 0;
    let mut RPKEY: i32 = 0;
    let mut STATE: i32 = 0;
    let mut TRUST: i32 = 0;
    let mut OVERFL: bool = false;

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
    // Work with local copies of the input key and value.
    //
    LKEY = KEY;
    LVAL = VALUE;

    //
    // The first step is to insert the key into the tree without
    // balancing.  This step may cause a node to overflow.  We'll
    // handle the overflow later.  In general, the probability of
    // overflow is low:  each overflow creates at least one new node,
    // but the ratio of nodes to keys is very small.
    //
    ZZEKTRUI(HANDLE, TREE, LKEY, LVAL, &mut OVERFL, ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // If the insertion didn't result in an overflow, we're done.
    //
    if !OVERFL {
        return Ok(());
    }

    //
    // Handle node overflows, as required.
    //
    STATE = LCHECK;

    while (STATE != TERM) {
        if (STATE == LCHECK) {
            //
            // Look up the node containing LKEY.
            //
            ZZEKTRLK(
                HANDLE,
                TREE,
                LKEY,
                &mut IDX,
                &mut NODE,
                &mut NOFFST,
                &mut LEVEL,
                &mut LVAL,
                ctx,
            )?;

            if (NODE == ROOT) {
                STATE = SPLT13;
            } else {
                //
                // See if there's room in the left sibling.  Of course,
                // there must be a left sibling in order for there to be
                // room.
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
                    &mut LEFT,
                    &mut RPIDX,
                    &mut RPKEY,
                    &mut RIGHT,
                    ctx,
                )?;

                if (LEFT > 0) {
                    NKEYS = ZZEKTRNK(HANDLE, TREE, LEFT, ctx)?;

                    if (NKEYS < MXKEYC) {
                        LNODE = LEFT;
                        RNODE = NODE;
                        PKIDX = LPIDX;
                        STATE = BALNCE;
                    } else {
                        STATE = RCHECK;
                    }
                } else {
                    STATE = RCHECK;
                }
            }
        } else if (STATE == RCHECK) {
            //
            // See whether there's room in the right sibling, if there
            // is a right sibling.  The left sibling has already been
            // checked and found wanting.
            //
            if (RIGHT > 0) {
                NKEYS = ZZEKTRNK(HANDLE, TREE, RIGHT, ctx)?;

                if (NKEYS < MXKEYC) {
                    LNODE = NODE;
                    RNODE = RIGHT;
                    PKIDX = RPIDX;

                    STATE = BALNCE;
                } else {
                    LNODE = NODE;
                    RNODE = RIGHT;
                    PKIDX = RPIDX;
                    STATE = SPLT23;
                }
            } else {
                //
                // The left sibling is full, but at least it's there.
                //
                LNODE = LEFT;
                RNODE = NODE;
                PKIDX = LPIDX;
                STATE = SPLT23;
            }
        } else if (STATE == BALNCE) {
            //
            // LNODE has a right sibling, and between the two nodes,
            // there's enough room to accommodate the overflow.  After
            // balancing these nodes, we're done.
            //
            ZZEKTRBN(HANDLE, TREE, LNODE, RNODE, PARENT, PKIDX, ctx)?;

            STATE = TERM;
        } else if (STATE == SPLT23) {
            //
            // LNODE has a right sibling, and between the two nodes,
            // there's an overflow of one key.  Split these two nodes
            // into three.  This splitting process adds a key to the
            // parent; the parent may overflow as a result.
            //
            // After executing the 2-3 split, to ensure that we reference
            // the parent correctly, we'll obtain a fresh key from the
            // parent.  The old key PKEY may not be in the parent any more;
            // this key may have been rotated into the middle node created
            // by the 2-3 split.
            //
            // To start with, we'll get a trusted key from the
            // original node NODE.  If NODE got mapped to LNODE,
            // then the first key in NODE will be unchanged by
            // the 2-3 split.  If NODE got mapped to RNODE, then
            // the last key in NODE will be unchanged.
            //
            if (NODE == LNODE) {
                //
                // Save the first key from NODE.
                //
                ZZEKTRKI(HANDLE, TREE, LKEY, 1, &mut TRUST, ctx)?;
            } else {
                //
                // Save the last key from NODE.
                //
                NSIZE = ZZEKTRNK(HANDLE, TREE, NODE, ctx)?;

                ZZEKTRKI(HANDLE, TREE, LKEY, NSIZE, &mut TRUST, ctx)?;
            }

            ZZEKTR23(HANDLE, TREE, LNODE, RNODE, PARENT, PKIDX, &mut OVERFL, ctx)?;

            if OVERFL {
                if (PARENT == ROOT) {
                    STATE = SPLT13;
                } else {
                    //
                    // We'll need to handle overflow in the parent.
                    // The parent should be correctly identified by the
                    // parent of TRUST.
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

                    LKEY = PKEY;
                    STATE = LCHECK;
                }
            } else {
                STATE = TERM;
            }
        } else if (STATE == SPLT13) {
            //
            // We've got an overflow in the root.  Split the root,
            // creating two new children.  The root contains a single
            // key after this split.
            //
            ZZEKTR13(HANDLE, TREE, ctx)?;
            STATE = TERM;
        }
    }

    Ok(())
}
