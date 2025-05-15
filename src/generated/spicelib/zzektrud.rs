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

//$Procedure      ZZEKTRUD ( EK tree, unbalanced deletion )
pub fn ZZEKTRUD(
    HANDLE: i32,
    TREE: i32,
    KEY: i32,
    TRGKEY: &mut i32,
    UNDRFL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATPTR: i32 = 0;
    let mut DEPTH: i32 = 0;
    let mut KEYIDX: i32 = 0;
    let mut LEAF: i32 = 0;
    let mut LEVEL: i32 = 0;
    let mut LOFFST: i32 = 0;
    let mut LPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut LPIDX: i32 = 0;
    let mut LPIDX2: i32 = 0;
    let mut LPKEY: i32 = 0;
    let mut LPKEY2: i32 = 0;
    let mut LSIB: i32 = 0;
    let mut LSIB2: i32 = 0;
    let mut NKEYS: i32 = 0;
    let mut NLKEYS: i32 = 0;
    let mut NNODE: i32 = 0;
    let mut PARENT: i32 = 0;
    let mut PAREN2: i32 = 0;
    let mut PKEY: i32 = 0;
    let mut PKEY2: i32 = 0;
    let mut POFFS2: i32 = 0;
    let mut POFFST: i32 = 0;
    let mut PREV: i32 = 0;
    let mut ROOT: i32 = 0;
    let mut RPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut RPIDX: i32 = 0;
    let mut RPIDX2: i32 = 0;
    let mut RPKEY: i32 = 0;
    let mut RPKEY2: i32 = 0;
    let mut RSIB: i32 = 0;
    let mut RSIB2: i32 = 0;
    let mut TARGET: i32 = 0;
    let mut TNKEYS: i32 = 0;
    let mut TOFFST: i32 = 0;
    let mut TOTKEY: i32 = 0;
    let mut TPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    //
    // Set the variable ROOT, so we'll have something mnemonic to go
    // by when referring to the root node.
    //
    ROOT = TREE;

    //
    // We always need to update the root page, so read it now.
    //
    ZZEKPGRI(HANDLE, ROOT, RPAGE.as_slice_mut(), ctx)?;

    //
    // The allowed range of keys is 1 to TOTKEY, where TOTKEY is the
    // total number of keys already present.
    //
    TOTKEY = RPAGE[TRNKEY];

    if ((KEY < 1) || (KEY > TOTKEY)) {
        CHKIN(b"ZZEKTRUD", ctx)?;
        SETMSG(b"Key = #. Valid range is 1:#.  File = #.", ctx);
        ERRINT(b"#", KEY, ctx);
        ERRINT(b"#", TOTKEY, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        CHKOUT(b"ZZEKTRUD", ctx)?;
        return Ok(());
    }

    //
    // Get the number of nodes in the tree.  Also save the tree's depth.
    //
    NNODE = RPAGE[TRNNOD];
    DEPTH = RPAGE[TRDPTH];

    //
    // Find the point at which the deletion is to occur.  When the
    // tree contains only one node, no search is necessary.
    //
    if (NNODE == 1) {
        //
        // This is the simplest case; all we need do is delete the
        // key from the root node.
        //
        // Set:
        //
        //    - The number of keys in the tree
        //    - The number of keys in the root
        //    - The last key
        //    - The data pointer for the last key
        //    - The child pointer following the last key
        //
        // In the root node, relative keys coincide with absolute keys,
        // so the key value need not be adjusted.
        //
        NKEYS = TOTKEY;
        RPAGE[TRNKEY] = (NKEYS - 1);
        RPAGE[TRNKR] = (NKEYS - 1);

        //
        // Shift the keys, data pointer, and child pointers to the left
        // of the deleted key.  Update the shifted keys.
        //
        for I in KEY..=(NKEYS - 1) {
            RPAGE[(TRKEYR + I)] = (RPAGE[((TRKEYR + I) + 1)] - 1);
            RPAGE[(TRDATR + I)] = RPAGE[((TRDATR + I) + 1)];
        }

        for I in KEY..=NKEYS {
            RPAGE[(TRKIDR + I)] = RPAGE[((TRKIDR + I) + 1)];
        }

        //
        // Zero out the freed entries.
        //
        RPAGE[(TRKEYR + NKEYS)] = 0;
        RPAGE[(TRDATR + NKEYS)] = 0;
        RPAGE[((TRKIDR + NKEYS) + 1)] = 0;

        //
        // Update the key count.
        //
        NKEYS = (NKEYS - 1);

        //
        // Underflow never occurs in the root; the tree simply becomes
        // empty if no keys are left.
        //
        *UNDRFL = false;

        //
        // The first key in the root will serve as the target key,
        // as long as the root isn't empty.
        //
        if (NKEYS > 0) {
            *TRGKEY = RPAGE[(TRKEYR + 1)];
        } else {
            *TRGKEY = 0;
        }

        //
        // Write the page back out, and we're all set.
        //
        ZZEKPGWI(HANDLE, ROOT, RPAGE.as_slice(), ctx)?;
    } else if (KEY == TOTKEY) {
        //
        // The deleted key is the last key in the tree.  This case
        // is simple, because no remaining keys change as a result of
        // this deletion.
        //
        ZZEKTRLK(
            HANDLE,
            TREE,
            KEY,
            &mut KEYIDX,
            &mut TARGET,
            &mut TOFFST,
            &mut LEVEL,
            &mut DATPTR,
            ctx,
        )?;

        if FAILED(ctx) {
            return Ok(());
        }

        ZZEKPGRI(HANDLE, TARGET, TPAGE.as_slice_mut(), ctx)?;

        NKEYS = TPAGE[TRNKC];

        //
        // Zero out the freed entries.
        //
        TPAGE[(TRKEYC + NKEYS)] = 0;
        TPAGE[(TRDATC + NKEYS)] = 0;
        TPAGE[((TRKIDC + NKEYS) + 1)] = 0;

        //
        // Update the key count for this node:
        //
        TPAGE[TRNKC] = (TPAGE[TRNKC] - 1);

        //
        // Since the key we deleted has no successors, there's no need
        // to adjust any other keys.  We must decrement the total
        // node count in the root, however.
        //
        RPAGE[TRNKEY] = (TOTKEY - 1);

        //
        // Underflow occurs when the node started out at the minimum
        // key count.
        //
        *UNDRFL = (NKEYS == MNKEYC);

        //
        // The first key in the target page is the target key.  Return
        // an absolute key.
        //
        *TRGKEY = (TPAGE[(TRKEYC + 1)] + TOFFST);

        //
        // Write the affected pages back out.
        //
        ZZEKPGWI(HANDLE, ROOT, RPAGE.as_slice(), ctx)?;
        ZZEKPGWI(HANDLE, TARGET, TPAGE.as_slice(), ctx)?;
    } else {
        //
        // Locate the item we wish to delete.
        //
        ZZEKTRLK(
            HANDLE,
            TREE,
            KEY,
            &mut KEYIDX,
            &mut TARGET,
            &mut TOFFST,
            &mut LEVEL,
            &mut DATPTR,
            ctx,
        )?;

        if (LEVEL == DEPTH) {
            //
            // The node containing KEY is a leaf node, which is what we
            // want.  Deletions always take place at leaf nodes.
            //
            // Since we'll have to update the ancestors of TARGET,
            // look up a key in the parent node now.  The order of
            // operations here is delicate; since the deletion
            // we're going to do will temporarily screw up our
            // addressing method, we want to do this look-up while
            // we're sure it will work.
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
                &mut LSIB,
                &mut RPIDX,
                &mut RPKEY,
                &mut RSIB,
                ctx,
            )?;

            if FAILED(ctx) {
                return Ok(());
            }

            //
            // Read the target page.  Get the key count for this node.
            //
            ZZEKPGRI(HANDLE, TARGET, TPAGE.as_slice_mut(), ctx)?;

            TNKEYS = TPAGE[TRNKC];

            //
            // Each node is allowed to underflow by 1 element.  If there
            // is already a deficit, OK, that's it.
            //
            if (TNKEYS < MNKEYC) {
                CHKIN(b"ZZEKTRUD", ctx)?;
                SETMSG(b"Node = #. Tree = #. File = #. Key count = #; max allowed, including overflow, is #.", ctx);
                ERRINT(b"#", TARGET, ctx);
                ERRINT(b"#", TREE, ctx);
                ERRHAN(b"#", HANDLE, ctx)?;
                ERRINT(b"#", TNKEYS, ctx);
                ERRINT(b"#", (MXKEYC + 1), ctx);
                SIGERR(b"SPICE(BUG)", ctx)?;
                CHKOUT(b"ZZEKTRUD", ctx)?;
                return Ok(());
            }

            //
            // Shift the keys, data pointers, and child pointers starting
            // at KEY to the left by 1 position.  Careful, move the
            // leftmost elements first.  Update the shifted key values
            // while we're at it.
            //
            for I in KEYIDX..=(TNKEYS - 1) {
                TPAGE[(TRKEYC + I)] = (TPAGE[((TRKEYC + I) + 1)] - 1);
            }

            for I in KEYIDX..=(TNKEYS - 1) {
                TPAGE[(TRDATC + I)] = TPAGE[((TRDATC + I) + 1)];
            }

            for I in KEYIDX..=TNKEYS {
                TPAGE[(TRKIDC + I)] = TPAGE[((TRKIDC + I) + 1)];
            }

            //
            // Update the key count for the target node.
            //
            TPAGE[TRNKC] = (TNKEYS - 1);

            //
            // Underflow occurs when the node started out at the minimum
            // count.
            //
            *UNDRFL = (TNKEYS == MNKEYC);

            //
            // The first key in the target page is the target key.
            //
            *TRGKEY = (TPAGE[(TRKEYC + 1)] + TOFFST);

            //
            // Write the target page back out.
            //
            ZZEKPGWI(HANDLE, TARGET, TPAGE.as_slice(), ctx)?;
        } else {
            //
            // The node containing KEY is not a leaf node.  Therefore,
            // KEY > 1 and KEY has a predecessor.  This predecessor
            // is guaranteed to reside in a leaf node.  This is simply
            // a property of B*-trees, of which EK trees are a subclass.
            // Find this predecessor.
            //
            ZZEKTRLK(
                HANDLE,
                TREE,
                (KEY - 1),
                &mut PREV,
                &mut LEAF,
                &mut LOFFST,
                &mut LEVEL,
                &mut DATPTR,
                ctx,
            )?;

            if FAILED(ctx) {
                return Ok(());
            }

            //
            // Since we'll have to update the ancestors of LEAF,
            // look up a key in the parent node now.  The order of
            // operations here is delicate; since the deletion
            // we're going to do will temporarily screw up our
            // addressing method, we want to do this look-up while
            // we're sure it will work.
            //
            ZZEKTRPI(
                HANDLE,
                TREE,
                (KEY - 1),
                &mut PARENT,
                &mut PKEY,
                &mut POFFST,
                &mut LPIDX,
                &mut LPKEY,
                &mut LSIB,
                &mut RPIDX,
                &mut RPKEY,
                &mut RSIB,
                ctx,
            )?;

            if FAILED(ctx) {
                return Ok(());
            }

            //
            // Since deletions are allowed only in leaf nodes, we'll
            // perform a little sleight-of-code:  We'll move the key's
            // predecessor into the key's location, then remove the
            // predecessor from its leaf node.  The order of the keys
            // is not disturbed by this re-arrangement.
            //
            // Moving the key's predecessor into the key's location is
            // accomplished simply by transferring the data pointer.
            //
            ZZEKPGRI(HANDLE, LEAF, LPAGE.as_slice_mut(), ctx)?;

            if (TARGET == ROOT) {
                //
                // The root page has already been read into RPAGE.
                //
                RPAGE[(TRDATR + KEYIDX)] = LPAGE[(TRDATC + PREV)];
            } else {
                ZZEKPGRI(HANDLE, TARGET, TPAGE.as_slice_mut(), ctx)?;

                TPAGE[(TRDATC + KEYIDX)] = LPAGE[(TRDATC + PREV)];
            }

            //
            // The keys and data pointers in the leaf must be shifted
            // left to account for the deletion.  We'll zero out the
            // freed elements.  All child pointers are NIL and hence need
            // not be shifted.
            //
            NLKEYS = LPAGE[TRNKC];

            for I in PREV..=(NLKEYS - 1) {
                LPAGE[(TRKEYC + I)] = (LPAGE[((TRKEYC + I) + 1)] - 1);
                LPAGE[(TRDATC + I)] = LPAGE[((TRDATC + I) + 1)];
            }

            //
            // Update the key count for the leaf node.
            //
            LPAGE[TRNKC] = (NLKEYS - 1);

            //
            // Underflow occurs when the leaf node started out at the
            // minimum count.
            //
            *UNDRFL = (NLKEYS == MNKEYC);

            //
            // The first key in the leaf page is the target key.
            //
            *TRGKEY = (LPAGE[(TRKEYC + 1)] + LOFFST);

            //
            // Write the leaf, and if necessary, the target page back out.
            //
            ZZEKPGWI(HANDLE, LEAF, LPAGE.as_slice(), ctx)?;

            if (TARGET != ROOT) {
                ZZEKPGWI(HANDLE, TARGET, TPAGE.as_slice(), ctx)?;
            }

            //
            // The next step will be to update the ancestors of LEAF.
            // For the purposes of this operation, LEAF is the target
            // node.
            //
            TARGET = LEAF;
        }

        //
        // We must update the affected keys in every ancestor of TARGET.
        // We've already looked up information for the parent of
        // TARGET.  See the note at the prior call to ZZEKTRPI.
        //
        while (PARENT != ROOT) {
            //
            // Before going to work on the parent, get *its* parent's info.
            // This is the last chance to do so.
            //
            ZZEKTRPI(
                HANDLE,
                TREE,
                PKEY,
                &mut PAREN2,
                &mut PKEY2,
                &mut POFFS2,
                &mut LPIDX2,
                &mut LPKEY2,
                &mut LSIB2,
                &mut RPIDX2,
                &mut RPKEY2,
                &mut RSIB2,
                ctx,
            )?;

            //
            // Read the parent node.  All keys from the right parent key
            // onward get decremented.  Remember that there may be no
            // right parent key.
            //
            ZZEKPGRI(HANDLE, PARENT, TPAGE.as_slice_mut(), ctx)?;

            TNKEYS = TPAGE[TRNKC];

            if (RPIDX > 0) {
                for I in RPIDX..=TNKEYS {
                    TPAGE[(TRKEYC + I)] = (TPAGE[(TRKEYC + I)] - 1);
                }

                //
                // Write the updated page back out.
                //
                ZZEKPGWI(HANDLE, PARENT, TPAGE.as_slice(), ctx)?;
            }

            PARENT = PAREN2;
            PKEY = PKEY2;
            RPIDX = RPIDX2;
        }

        //
        // Update the keys in the root.  Recall that the root page has
        // already been read into RPAGE.
        //
        TNKEYS = RPAGE[TRNKR];

        if (RPIDX > 0) {
            for I in RPIDX..=TNKEYS {
                RPAGE[(TRKEYR + I)] = (RPAGE[(TRKEYR + I)] - 1);
            }
        }

        //
        // Update the total key count for the tree.
        //
        RPAGE[TRNKEY] = (TOTKEY - 1);

        //
        // Write the updated root page back out.
        //
        ZZEKPGWI(HANDLE, ROOT, RPAGE.as_slice(), ctx)?;
    }

    Ok(())
}
