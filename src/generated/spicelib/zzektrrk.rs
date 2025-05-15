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

//$Procedure      ZZEKTRRK ( EK tree, rotate keys )
pub fn ZZEKTRRK(
    HANDLE: i32,
    TREE: i32,
    LEFT: i32,
    RIGHT: i32,
    PARENT: i32,
    PKIDX: i32,
    NROT: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DATBAS: i32 = 0;
    let mut DPAR: i32 = 0;
    let mut DROTAT: i32 = 0;
    let mut DSHIFT: i32 = 0;
    let mut FUTRPK: i32 = 0;
    let mut KEYBAS: i32 = 0;
    let mut KIDBAS: i32 = 0;
    let mut LNKEYS: i32 = 0;
    let mut LNSIZE: i32 = 0;
    let mut LPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut LSIB: i32 = 0;
    let mut NVOPAR: i32 = 0;
    let mut PPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut REMAIN: i32 = 0;
    let mut RNKEYS: i32 = 0;
    let mut ROOT: i32 = 0;
    let mut RPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut RSIB: i32 = 0;
    let mut SCHLEP: i32 = 0;
    let mut SUBSIZ: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in for speed.
    //
    if (NROT == 0) {
        return Ok(());
    }

    ROOT = TREE;

    if ((LEFT == ROOT) || (RIGHT == ROOT)) {
        CHKIN(b"ZZEKTRRK", ctx)?;
        SETMSG(
            b"Input node is root; only children are eligible for key rotation.",
            ctx,
        );
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTRRK", ctx)?;
    }

    //
    // Read in the input nodes.
    //
    ZZEKPGRI(HANDLE, LEFT, LPAGE.as_slice_mut(), ctx)?;
    ZZEKPGRI(HANDLE, RIGHT, RPAGE.as_slice_mut(), ctx)?;
    ZZEKPGRI(HANDLE, PARENT, PPAGE.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        return Ok(());
    }

    //
    // Set the base index of the parent keys.  This value depends on
    // whether the parent is the root.  Do the same for the pointer
    // bases.
    //
    if (PARENT == TREE) {
        KEYBAS = TRKEYR;
        DATBAS = TRDATR;
        KIDBAS = TRKIDR;
    } else {
        KEYBAS = TRKEYC;
        DATBAS = TRDATC;
        KIDBAS = TRKIDC;
    }

    //
    // Verify that LEFT and RIGHT are siblings, and that PARENT is
    // their common parent.
    //
    LSIB = PPAGE[(KIDBAS + PKIDX)];
    RSIB = PPAGE[((KIDBAS + PKIDX) + 1)];

    if ((LSIB != LEFT) || (RSIB != RIGHT)) {
        CHKIN(b"ZZEKTRRK", ctx)?;
        SETMSG(b"LEFT, RIGHT, PARENT, and PKIDX are inconsistent. LEFT = #; RIGHT = #; PARENT = #; PKIDX = #; LSIB derived from PARENT = #; RSIB = #.", ctx);
        ERRINT(b"#", LEFT, ctx);
        ERRINT(b"#", RIGHT, ctx);
        ERRINT(b"#", PARENT, ctx);
        ERRINT(b"#", PKIDX, ctx);
        ERRINT(b"#", LSIB, ctx);
        ERRINT(b"#", RSIB, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTRRK", ctx)?;
        return Ok(());
    }

    //
    // Get the key counts for the left and right nodes.
    //
    LNKEYS = LPAGE[TRNKC];
    RNKEYS = RPAGE[TRNKC];

    //
    // The requested rotation will not be permitted to cause an
    // underflow of more than one key in the source node, nor an
    // overflow of more than one key in the destination node.
    //
    if (NROT > 0) {
        if (((LNKEYS - NROT) < (MNKEYC - 1)) || ((RNKEYS + NROT) > (MXKEYC + 1))) {
            CHKIN(b"ZZEKTRRK", ctx)?;
            SETMSG(b"Node # and right sibling # contain # and # keys respectively; rotation of # keys to the right will violate the key count bounds of #:#.", ctx);
            ERRINT(b"#", LEFT, ctx);
            ERRINT(b"#", RIGHT, ctx);
            ERRINT(b"#", LNKEYS, ctx);
            ERRINT(b"#", RNKEYS, ctx);
            ERRINT(b"#", NROT, ctx);
            ERRINT(b"#", (MNKEYC - 1), ctx);
            ERRINT(b"#", (MXKEYC + 1), ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKTRRK", ctx)?;
            return Ok(());
        }
    } else if (NROT < 0) {
        if (((LNKEYS - NROT) > (MXKEYC + 1)) || ((RNKEYS + NROT) < (MNKEYC - 1))) {
            CHKIN(b"ZZEKTRRK", ctx)?;
            SETMSG(b"Node # and right sibling # contain # and # keys respectively; rotation of # keys to the left will violate the key count bounds of #:#.", ctx);
            ERRINT(b"#", LEFT, ctx);
            ERRINT(b"#", RIGHT, ctx);
            ERRINT(b"#", LNKEYS, ctx);
            ERRINT(b"#", RNKEYS, ctx);
            ERRINT(b"#", -NROT, ctx);
            ERRINT(b"#", (MNKEYC - 1), ctx);
            ERRINT(b"#", (MXKEYC + 1), ctx);
            SIGERR(b"SPICE(BUG)", ctx)?;
            CHKOUT(b"ZZEKTRRK", ctx)?;
            return Ok(());
        }
    }

    //
    // Compute the size of the tree headed by the left subnode.  We'll
    // need this later.  The size of this tree is one less than the
    // difference of the parent key and its predecessor, if any.
    //
    if (PKIDX == 1) {
        LNSIZE = (PPAGE[(KEYBAS + 1)] - 1);
    } else {
        LNSIZE = ((PPAGE[(KEYBAS + PKIDX)] - PPAGE[((KEYBAS + PKIDX) - 1)]) - 1);
    }

    //
    // Now, the actions we take depend on whether we must schlep keys
    // to the right or left.
    //
    if (NROT > 0) {
        //
        // We'll rotate keys to the right.  There are a bunch of numbers
        // to compute first:
        //
        //    -- The number of keys remaining in the input node:  REMAIN
        //
        //    -- The size of the subtree headed by the
        //       rotated keys:  SUBSIZ
        //
        //    -- The offset delta to be applied to the rotated
        //       keys:  DROTAT
        //
        //    -- The offset delta to be applied to the keys shifted
        //       right in the sibling:  DSHIFT
        //
        //    -- The new value of the old right parent key,
        //       which gets rotated into the sibling:  NVOPAR
        //
        //    -- The offset delta to apply to the new right parent key,
        //       DPAR.  Note that the successors of this key in the
        //       parent node remain unchanged.
        //
        //
        SCHLEP = NROT;
        REMAIN = (LNKEYS - SCHLEP);

        //
        // The size of the rotated subtree is the original size of the
        // subtree headed by LEFT, minus the value of the key preceding
        // the rotated subtree.  That key, which resides at location
        // REMAIN + 1, is the future right parent key; this key is also
        // the successor of the subtree left behind.
        //
        FUTRPK = LPAGE[((TRKEYC + REMAIN) + 1)];
        SUBSIZ = (LNSIZE - FUTRPK);

        //
        // The rotated set of keys will no longer be preceded by the
        // set of keys of size NEWRPK that they originally followed.
        //
        DROTAT = -FUTRPK;

        //
        // The shifted keys in the right sibling get SUBSIZ + 1 new
        // predecessors.
        //
        DSHIFT = (SUBSIZ + 1);

        //
        // The old right parent key will become the successor of the
        // shifted subtree.  Its value is just one greater than the
        // size of this subtree.
        //
        NVOPAR = DSHIFT;

        //
        // The new parent key has DSHIFT fewer predecessors after
        // the rotation.
        //
        DPAR = -DSHIFT;

        //
        // It's time for some action.  First of all, shift the keys
        // in the sibling to the right.  Their data pointers and child
        // pointers move along with them.  Update all the keys by
        // applying the shift delta to them.
        //
        // Move the rightmost elements of each data component first.
        // Adjust the keys at the same time.  Note that the regions
        // allocated to keys, data pointers, and child pointers occupy
        // non-overlapping addresses, so the order in which we shift
        // these data sets is not important.  Within each data set, we
        // must be careful not to trash occupied addresses.
        //
        for I in intrinsics::range(RNKEYS, 1, -1) {
            RPAGE[((TRKEYC + I) + SCHLEP)] = (RPAGE[(TRKEYC + I)] + DSHIFT);
        }

        for I in intrinsics::range(RNKEYS, 1, -1) {
            RPAGE[((TRDATC + I) + SCHLEP)] = RPAGE[(TRDATC + I)];
        }

        for I in intrinsics::range((RNKEYS + 1), 1, -1) {
            RPAGE[((TRKIDC + I) + SCHLEP)] = RPAGE[(TRKIDC + I)];
        }

        //
        // `Move' the old parent key to its target destination in the
        // sibling.  Actually, only the data pointer is copied; the key
        // is simply set to its new value.
        //
        RPAGE[(TRKEYC + SCHLEP)] = NVOPAR;
        RPAGE[(TRDATC + SCHLEP)] = PPAGE[(DATBAS + PKIDX)];

        //
        // `Move' the future parent key to its target destination in the
        // parent.  The data pointer is copied; the key is adjusted by
        // the offset delta we've computed.
        //
        PPAGE[(DATBAS + PKIDX)] = LPAGE[((TRDATC + REMAIN) + 1)];
        PPAGE[(KEYBAS + PKIDX)] = (PPAGE[(KEYBAS + PKIDX)] + DPAR);

        //
        // Rotate the subtree following the future parent key to its
        // destination in the sibling.  Update the keys to account for
        // their new offset.
        //
        for I in 1..=(SCHLEP - 1) {
            RPAGE[(TRKEYC + I)] = (LPAGE[(((TRKEYC + REMAIN) + 1) + I)] + DROTAT);
        }

        MOVEI(
            LPAGE.subarray(((TRDATC + REMAIN) + 2)),
            (SCHLEP - 1),
            RPAGE.subarray_mut((TRDATC + 1)),
        );
        MOVEI(
            LPAGE.subarray(((TRKIDC + REMAIN) + 2)),
            SCHLEP,
            RPAGE.subarray_mut((TRKIDC + 1)),
        );

        //
        // Update the key counts in both the input node and sibling.
        //
        LPAGE[TRNKC] = (LPAGE[TRNKC] - SCHLEP);
        RPAGE[TRNKC] = (RPAGE[TRNKC] + SCHLEP);

        //
        // Update the pages in the kernel.
        //
        ZZEKPGWI(HANDLE, PARENT, PPAGE.as_slice(), ctx)?;
        ZZEKPGWI(HANDLE, LEFT, LPAGE.as_slice(), ctx)?;
        ZZEKPGWI(HANDLE, RIGHT, RPAGE.as_slice(), ctx)?;
    } else {
        //
        // Rotation to the left is almost, but not quite, a mirror image
        // of rotation to the right.
        //
        SCHLEP = -NROT;
        REMAIN = (RNKEYS - SCHLEP);

        //
        // The size of the rotated subtree is one less than the value of
        // the future parent key.  This key resides at location
        // SCHLEP and is also the predecessor of the subtree
        // left behind.
        //
        FUTRPK = RPAGE[(TRKEYC + SCHLEP)];
        SUBSIZ = (FUTRPK - 1);

        //
        // The rotated set of keys will be preceded by the keys already
        // present in LEFT, as well as the key moved in from the parent
        // node.
        //
        DROTAT = (LNSIZE + 1);

        //
        // The shifted keys in the right sibling lose SUBSIZ + 1
        // predecessors.
        //
        DSHIFT = -(SUBSIZ + 1);

        //
        // The old parent key will become the successor of the
        // keys already in LEFT; it will be the predecessor of the
        // rotated subtree.
        //
        NVOPAR = DROTAT;

        //
        // The new parent key has (-DSHIFT) more predecessors after
        // the rotation.
        //
        DPAR = -DSHIFT;

        //
        // It's time for some action.
        //
        // `Move' the old parent key to its target destination in the
        // input node.  Actually, only the data pointer is copied; the key
        // is simply set to its new value.
        //
        LPAGE[((TRKEYC + LNKEYS) + 1)] = NVOPAR;
        LPAGE[((TRDATC + LNKEYS) + 1)] = PPAGE[(DATBAS + PKIDX)];

        //
        // `Move' the future parent key to its target destination in the
        // parent.  The data pointer is copied; the key is adjusted by
        // the offset delta we've computed.
        //
        PPAGE[(DATBAS + PKIDX)] = RPAGE[(TRDATC + SCHLEP)];
        PPAGE[(KEYBAS + PKIDX)] = (PPAGE[(KEYBAS + PKIDX)] + DPAR);

        //
        // Rotate the subtree following the future parent key to its
        // destination in the sibling.  Update the keys to account for
        // their new offset.
        //
        MOVEI(
            RPAGE.subarray((TRKEYC + 1)),
            (SCHLEP - 1),
            LPAGE.subarray_mut(((TRKEYC + LNKEYS) + 2)),
        );
        MOVEI(
            RPAGE.subarray((TRDATC + 1)),
            (SCHLEP - 1),
            LPAGE.subarray_mut(((TRDATC + LNKEYS) + 2)),
        );
        MOVEI(
            RPAGE.subarray((TRKIDC + 1)),
            SCHLEP,
            LPAGE.subarray_mut(((TRKIDC + LNKEYS) + 2)),
        );

        for I in 1..=(SCHLEP - 1) {
            LPAGE[(((TRKEYC + LNKEYS) + 1) + I)] = (LPAGE[(((TRKEYC + LNKEYS) + 1) + I)] + DROTAT);
        }

        //
        // Shift the remaining elements of the sibling to the left.
        // Their data pointers and child pointers move along with them.
        // Update all the keys by applying the shift delta to them.
        //
        // Move the leftmost elements of each data component first.
        // Adjust the keys at the same time.
        //
        for I in 1..=REMAIN {
            RPAGE[(TRKEYC + I)] = (RPAGE[((TRKEYC + I) + SCHLEP)] + DSHIFT);
        }

        for I in 1..=REMAIN {
            RPAGE[(TRDATC + I)] = RPAGE[((TRDATC + I) + SCHLEP)];
        }

        for I in 1..=(REMAIN + 1) {
            RPAGE[(TRKIDC + I)] = RPAGE[((TRKIDC + I) + SCHLEP)];
        }

        //
        // Update the key counts in both the input node and sibling.
        //
        LPAGE[TRNKC] = (LPAGE[TRNKC] + SCHLEP);
        RPAGE[TRNKC] = (RPAGE[TRNKC] - SCHLEP);

        //
        // Update the pages in the kernel.
        //
        ZZEKPGWI(HANDLE, PARENT, PPAGE.as_slice(), ctx)?;
        ZZEKPGWI(HANDLE, LEFT, LPAGE.as_slice(), ctx)?;
        ZZEKPGWI(HANDLE, RIGHT, RPAGE.as_slice(), ctx)?;
    }

    Ok(())
}
