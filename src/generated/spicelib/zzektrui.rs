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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

//$Procedure      ZZEKTRUI ( EK tree, unbalanced insertion )
pub fn ZZEKTRUI(
    HANDLE: i32,
    TREE: i32,
    KEY: i32,
    VALUE: i32,
    OVERFL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATLOC: i32 = 0;
    let mut DATPTR: i32 = 0;
    let mut DEPTH: i32 = 0;
    let mut IDX: i32 = 0;
    let mut KEYLOC: i32 = 0;
    let mut KIDLOC: i32 = 0;
    let mut LEVEL: i32 = 0;
    let mut LPIDX: i32 = 0;
    let mut LPIDX2: i32 = 0;
    let mut LPKEY: i32 = 0;
    let mut LPKEY2: i32 = 0;
    let mut LSIB: i32 = 0;
    let mut LSIB2: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut NKEYS: i32 = 0;
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
    // The allowed range of keys is 1 to (TOTKEY+1), where TOTKEY is the
    // total number of keys already present.
    //
    TOTKEY = RPAGE[TRNKEY];

    if ((KEY < 1) || (KEY > (TOTKEY + 1))) {
        CHKIN(b"ZZEKTRUI", ctx)?;
        SETMSG(b"Key = #. Valid range is 1:#.  File = #.", ctx);
        ERRINT(b"#", KEY, ctx);
        ERRINT(b"#", (TOTKEY + 1), ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"ZZEKTRUI", ctx)?;
        return Ok(());
    }

    //
    // Get the number of nodes in the tree.  Also save the tree's depth.
    //
    NNODE = RPAGE[TRNNOD];
    DEPTH = RPAGE[TRDPTH];

    //
    // Find the point at which the insertion is to occur.  When the
    // tree contains only one node, no search is necessary.
    //
    if (NNODE == 1) {
        //
        // This is the simplest case; all we need do is set up the
        // key in the root node.
        //
        // Set:
        //
        //    - The number of keys in the tree
        //    - The number of keys in the root
        //    - The last key
        //    - The data value for the last key
        //    - The child pointer following the last key
        //
        // In the root node, relative keys coincide with absolute keys,
        // so the key value need not be adjusted.
        //
        NKEYS = TOTKEY;
        RPAGE[TRNKEY] = (NKEYS + 1);
        RPAGE[TRNKR] = (NKEYS + 1);

        //
        // Shift the keys, data value, and child pointers to the right
        // of the new key.  Update the shifted keys.
        //
        for I in intrinsics::range(NKEYS, KEY, -1) {
            RPAGE[((TRKEYR + I) + 1)] = (RPAGE[(TRKEYR + I)] + 1);
            RPAGE[((TRDATR + I) + 1)] = RPAGE[(TRDATR + I)];
        }

        for I in intrinsics::range((NKEYS + 1), KEY, -1) {
            RPAGE[((TRKIDR + I) + 1)] = RPAGE[(TRKIDR + I)];
        }

        RPAGE[(TRKEYR + KEY)] = KEY;
        RPAGE[(TRDATR + KEY)] = VALUE;
        RPAGE[(TRKIDR + KEY)] = 0;

        //
        // Update the key count.
        //
        NKEYS = (NKEYS + 1);

        //
        // The node into which the key was inserted was the root.
        //
        TARGET = ROOT;

        //
        // Overflow occurs when the root started out full.
        //
        *OVERFL = (NKEYS == (MXKEYR + 1));

        //
        // Write the page back out, and we're all set.
        //
        ZZEKPGWI(HANDLE, ROOT, RPAGE.as_slice(), ctx)?;
    } else if (KEY == (TOTKEY + 1)) {
        //
        // The new key will be the last key in the tree.  This case
        // is simple:  the key goes in the last node of the tree.
        // Since every child node contains more than one key, we can
        // find the node by looking up the last key already present.
        //
        ZZEKTRLK(
            HANDLE,
            TREE,
            (KEY - 1),
            &mut IDX,
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

        KEYLOC = ((TRKEYC + NKEYS) + 1);
        DATLOC = ((TRDATC + NKEYS) + 1);
        KIDLOC = ((TRKIDC + NKEYS) + 1);

        //
        // The last node in the tree is always at the lowest level,
        // so the relative value of the new key can be computed from
        // that of its predecessor.
        //
        TPAGE[KEYLOC] = (TPAGE[(KEYLOC - 1)] + 1);
        TPAGE[DATLOC] = VALUE;
        TPAGE[KIDLOC] = 0;

        //
        // Update the key count for this node:
        //
        TPAGE[TRNKC] = (TPAGE[TRNKC] + 1);

        //
        // Since the key we inserted has no successors, there's no need
        // to adjust any other keys.  We must increment the total
        // node count in the root, however.
        //
        RPAGE[TRNKEY] = (TOTKEY + 1);

        //
        // Overflow occurs when the node started out full.
        //
        *OVERFL = (NKEYS == MXKEYC);

        //
        // Write the affected pages back out.
        //
        ZZEKPGWI(HANDLE, ROOT, RPAGE.as_slice(), ctx)?;
        ZZEKPGWI(HANDLE, TARGET, TPAGE.as_slice(), ctx)?;
    } else {
        //
        // The item we wish to insert will displace the item whose
        // ordinal position is KEY.  Locate this target item.
        //
        ZZEKTRLK(
            HANDLE,
            TREE,
            KEY,
            &mut NEXT,
            &mut TARGET,
            &mut TOFFST,
            &mut LEVEL,
            &mut DATPTR,
            ctx,
        )?;

        if (LEVEL == DEPTH) {
            //
            // The node containing KEY is a leaf node, which is what we
            // want.  Insertions always take place at leaf nodes.
            //
            // Since we'll have to update the ancestors of TARGET,
            // look up a key in the parent node now.  The order of
            // operations here is delicate; since the insertion
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
            // Each node is allowed to overflow by 1 element.  If there's
            // no more room, OK, that's it.
            //
            if (TNKEYS > MXKEYC) {
                CHKIN(b"ZZEKTRUI", ctx)?;
                SETMSG(b"Node = #. Tree = #. File = #. Key count = #; max allowed, including overflow, is #.", ctx);
                ERRINT(b"#", TARGET, ctx);
                ERRINT(b"#", TREE, ctx);
                ERRHAN(b"#", HANDLE, ctx)?;
                ERRINT(b"#", TNKEYS, ctx);
                ERRINT(b"#", (MXKEYC + 1), ctx);
                SIGERR(b"SPICE(NODETOOFULL)", ctx)?;
                CHKOUT(b"ZZEKTRUI", ctx)?;
                return Ok(());
            }

            //
            // Shift the keys, data values, and child pointers starting
            // at NEXT over to the right by 1 position.  Careful, move the
            // rightmost elements first.  Update the shifted key values
            // while we're at it.
            //
            for I in intrinsics::range(TNKEYS, NEXT, -1) {
                TPAGE[((TRKEYC + I) + 1)] = (TPAGE[(TRKEYC + I)] + 1);
            }

            for I in intrinsics::range(TNKEYS, NEXT, -1) {
                TPAGE[((TRDATC + I) + 1)] = TPAGE[(TRDATC + I)];
            }

            for I in intrinsics::range((TNKEYS + 1), NEXT, -1) {
                TPAGE[((TRKIDC + I) + 1)] = TPAGE[(TRKIDC + I)];
            }

            //
            // The new key simply takes the value of the old one.  The
            // corresponding data value must be set, however.
            //
            TPAGE[(TRDATC + NEXT)] = VALUE;
        } else {
            //
            // The node containing KEY is not a leaf node.  Therefore,
            // KEY > 1 and KEY has a predecessor.  This predecessor
            // is guaranteed to reside in a leaf node.  This is simply
            // a property of B*-trees, of which EK trees are a subclass.
            //
            ZZEKTRLK(
                HANDLE,
                TREE,
                (KEY - 1),
                &mut PREV,
                &mut TARGET,
                &mut TOFFST,
                &mut LEVEL,
                &mut DATPTR,
                ctx,
            )?;

            if FAILED(ctx) {
                return Ok(());
            }

            //
            // Since we'll have to update the ancestors of TARGET,
            // look up a key in the parent node now.  The order of
            // operations here is delicate; since the insertion
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
            // The predecessor of KEY will be the last key present in the
            // node TARGET.  Make sure there's room in the node.
            //
            ZZEKPGRI(HANDLE, TARGET, TPAGE.as_slice_mut(), ctx)?;

            TNKEYS = TPAGE[TRNKC];

            if (TNKEYS > (MXKEYC + 1)) {
                CHKIN(b"ZZEKTRUI", ctx)?;
                SETMSG(b"Node = #. Tree = #. File = #. Key count = #; max allowed, including overflow, is #.", ctx);
                ERRINT(b"#", TARGET, ctx);
                ERRINT(b"#", TREE, ctx);
                ERRHAN(b"#", HANDLE, ctx)?;
                ERRINT(b"#", TNKEYS, ctx);
                ERRINT(b"#", (MXKEYC + 1), ctx);
                SIGERR(b"SPICE(NODETOOFULL)", ctx)?;
                CHKOUT(b"ZZEKTRUI", ctx)?;
                return Ok(());
            }

            //
            // Set the new key and the corresponding data and child
            // pointers.
            //
            TPAGE[((TRKEYC + PREV) + 1)] = (PREV + 1);
            TPAGE[((TRDATC + PREV) + 1)] = VALUE;
            TPAGE[((TRKIDC + PREV) + 2)] = 0;
        }

        //
        // Update the key count for the target node.
        //
        TPAGE[TRNKC] = (TNKEYS + 1);

        //
        // Overflow occurs when the node started out full.
        //
        *OVERFL = (TNKEYS == MXKEYC);

        //
        // Write the target page back out.
        //
        ZZEKPGWI(HANDLE, TARGET, TPAGE.as_slice(), ctx)?;

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
            // onward get incremented.  Remember that there may be no
            // right parent key.
            //
            ZZEKPGRI(HANDLE, PARENT, TPAGE.as_slice_mut(), ctx)?;

            TNKEYS = TPAGE[TRNKC];

            if (RPIDX > 0) {
                for I in RPIDX..=TNKEYS {
                    TPAGE[(TRKEYC + I)] = (TPAGE[(TRKEYC + I)] + 1);
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
                RPAGE[(TRKEYR + I)] = (RPAGE[(TRKEYR + I)] + 1);
            }
        }

        //
        // Update the total key count for the tree.
        //
        RPAGE[TRNKEY] = (TOTKEY + 1);

        //
        // Write the updated root page back out.
        //
        ZZEKPGWI(HANDLE, ROOT, RPAGE.as_slice(), ctx)?;
    }

    Ok(())
}
