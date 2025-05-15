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
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;

//$Procedure      ZZEKTR23 ( EK tree, 2-3 split )
pub fn ZZEKTR23(
    HANDLE: i32,
    TREE: i32,
    LEFT: i32,
    RIGHT: i32,
    PARENT: i32,
    PKIDX: i32,
    OVERFL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BASE: i32 = 0;
    let mut C1PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut C2PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut C3PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut DATBAS: i32 = 0;
    let mut KEYBAS: i32 = 0;
    let mut KIDBAS: i32 = 0;
    let mut LDELTA: i32 = 0;
    let mut LNMOVE: i32 = 0;
    let mut LSHIFT: i32 = 0;
    let mut LSIB: i32 = 0;
    let mut LSIZE: i32 = 0;
    let mut LTRSIZ: i32 = 0;
    let mut MSIZE: i32 = 0;
    let mut NEW: i32 = 0;
    let mut NLKEYS: i32 = 0;
    let mut NNODE: i32 = 0;
    let mut NPKEYS: i32 = 0;
    let mut NRKEYS: i32 = 0;
    let mut PPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut RBASE: i32 = 0;
    let mut RDELTA: i32 = 0;
    let mut RNMOVE: i32 = 0;
    let mut ROOT: i32 = 0;
    let mut RSHIFT: i32 = 0;
    let mut RSIB: i32 = 0;
    let mut RSIZE: i32 = 0;
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
    // The plan is to take two sibling nodes, one of which is full and
    // one of which is overflowing by 1 key, and to split off about
    // one third of the keys from each one into a new node.  The new
    // node will be a child of the common parent of the input nodes and
    // will be inserted between them.
    //
    // After the split, the sum of the numbers of keys in the three
    // children will be exactly 2*MXKEYC.  The numbers of keys in the
    // left, middle, and right nodes will be, respectively:
    //
    LSIZE = ((2 * MXKEYC) / 3);
    MSIZE = (((2 * MXKEYC) + 1) / 3);
    RSIZE = (((2 * MXKEYC) + 2) / 3);

    //
    // Note that exactly one of the numerators above is a multiple of 3,
    // so the sum of the above numbers is 1 less than if real division
    // were performed.  Therefore, the sum of the numbers of keys in the
    // child nodes is 2*MXKEYC.  The parent will contain one more node
    // than it did before the split:  the key originally between LEFT and
    // RIGHT will be moved down into the middle child, and the
    // smallest key moved from LEFT and the largest key moved from RIGHT
    // will go into PARENT.
    //
    ZZEKPGRI(HANDLE, LEFT, C1PAGE.as_slice_mut(), ctx)?;
    ZZEKPGRI(HANDLE, RIGHT, C2PAGE.as_slice_mut(), ctx)?;
    ZZEKPGRI(HANDLE, PARENT, PPAGE.as_slice_mut(), ctx)?;

    //
    // The actual addresses in the parent node depend on whether the
    // parent is the root.  Compute the necessary bases to avoid a lot
    // of cases.
    //
    ROOT = TREE;

    if (PARENT == ROOT) {
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
        CHKIN(b"ZZEKTR23", ctx)?;
        SETMSG(b"LEFT, RIGHT, PARENT, and PKIDX are inconsistent. LEFT = #; RIGHT = #; PARENT = #; PKIDX = #; LSIB derived from PARENT = #; RSIB = #.", ctx);
        ERRINT(b"#", LEFT, ctx);
        ERRINT(b"#", RIGHT, ctx);
        ERRINT(b"#", PARENT, ctx);
        ERRINT(b"#", PKIDX, ctx);
        ERRINT(b"#", LSIB, ctx);
        ERRINT(b"#", RSIB, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTR23", ctx)?;
        return Ok(());
    }

    NLKEYS = C1PAGE[TRNKC];
    NRKEYS = C2PAGE[TRNKC];
    SUM = (NLKEYS + NRKEYS);

    //
    // The sum of the number of keys in the two input nodes must
    // sum exactly to the value representing an overflow level of 1 key.
    //
    if (SUM != ((2 * MXKEYC) + 1)) {
        CHKIN(b"ZZEKTR23", ctx)?;
        SETMSG(
            b"Number of keys in LEFT = #; number of keys in right = #; but sum should be #.",
            ctx,
        );
        ERRINT(b"#", LEFT, ctx);
        ERRINT(b"#", RIGHT, ctx);
        ERRINT(b"#", ((2 * MXKEYC) + 1), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTR23", ctx)?;
        return Ok(());
    }

    //
    // Allocate a new page.  This page will become the right sibling
    // of LEFT and the left sibling of RIGHT.
    //
    ZZEKPGAL(HANDLE, INT, &mut NEW, &mut BASE, ctx)?;
    CLEARI(PGSIZI, C3PAGE.as_slice_mut());

    //
    // It's time to set up the keys in the middle child.  First, we'll
    // take the last LSHIFT keys from the left node, where
    //
    LSHIFT = (NLKEYS - (LSIZE + 1));

    //
    // When these keys are moved, they lose LDELTA predecessors, where
    // LDELTA is the size of the key set preceding and including the key
    // at location LSIZE + 1.  The size of this subtree is just the
    // key value at location LSIZE+1.
    //
    LDELTA = C1PAGE[((TRKEYC + LSIZE) + 1)];

    for I in 1..=LSHIFT {
        C3PAGE[(TRKEYC + I)] = (C1PAGE[(((TRKEYC + LSIZE) + 1) + I)] - LDELTA);
    }

    MOVEI(
        C1PAGE.subarray(((TRDATC + LSIZE) + 2)),
        LSHIFT,
        C3PAGE.subarray_mut((TRDATC + 1)),
    );
    MOVEI(
        C1PAGE.subarray(((TRKIDC + LSIZE) + 2)),
        (LSHIFT + 1),
        C3PAGE.subarray_mut((TRKIDC + 1)),
    );

    //
    // Compute the size of the tree headed by the left subnode.  We'll
    // need this shortly.  The size of this tree is one less than the
    // difference of the parent key and its predecessor, if any.
    //
    if (PKIDX == 1) {
        LTRSIZ = (PPAGE[(KEYBAS + 1)] - 1);
    } else {
        LTRSIZ = ((PPAGE[(KEYBAS + PKIDX)] - PPAGE[((KEYBAS + PKIDX) - 1)]) - 1);
    }

    //
    // The next item to add to the middle child is the middle key
    // from the parent.  The data pointer is copied; the key value is
    // simply set.  The value of the key is one more than the size of
    // the entire key set (including descendants) we moved into the
    // middle from the left.  LNMOVE is the size of this key set.
    //
    // No child pointer is copied.
    //
    LNMOVE = (LTRSIZ - LDELTA);
    C3PAGE[((TRKEYC + LSHIFT) + 1)] = (LNMOVE + 1);

    C3PAGE[((TRDATC + LSHIFT) + 1)] = PPAGE[(DATBAS + PKIDX)];

    //
    // Now we copy keys from the right child into the middle.  We'll
    // take the first RSHIFT keys from the right node, where
    //
    RSHIFT = (NRKEYS - (RSIZE + 1));

    //
    // When these keys are moved, they gain RDELTA predecessors, where
    // RDELTA is the size of the key set already in the middle node.
    //
    RDELTA = (LNMOVE + 1);

    for I in 1..=RSHIFT {
        C3PAGE[(((TRKEYC + LSHIFT) + 1) + I)] = (C2PAGE[(TRKEYC + I)] + RDELTA);
    }

    MOVEI(
        C2PAGE.subarray((TRDATC + 1)),
        RSHIFT,
        C3PAGE.subarray_mut(((TRDATC + LSHIFT) + 2)),
    );
    MOVEI(
        C2PAGE.subarray((TRKIDC + 1)),
        (RSHIFT + 1),
        C3PAGE.subarray_mut(((TRKIDC + LSHIFT) + 2)),
    );

    //
    // Save the size of the entire key set moved into the middle from
    // the right.
    //
    RNMOVE = (C2PAGE[((TRKEYC + RSHIFT) + 1)] - 1);

    //
    // Set the key count in the new child.
    //
    C3PAGE[TRNKC] = MSIZE;

    //
    // The middle child is complete.
    //
    // The next step is to set up the parent node.  The original parent
    // key at index PKIDX is replaced by the key from the left child
    // at location LSIZE + 1.  The following parent keys are shifted
    // right by one location, making room for a second key following
    // the one at PKIDX.  This newly freed slot is filled in with the
    // key at location RSHIFT+1 in the right child.
    //
    // The keys in the parent to the right of position PKIDX+1 gain no
    // predecessors as the result of these re-arrangements.
    //
    // Get the number of keys in the parent.
    //
    if (PARENT == ROOT) {
        NPKEYS = PPAGE[TRNKR];
    } else {
        NPKEYS = PPAGE[TRNKC];
    }

    //
    // Make room for the new key.  Shift elements starting from the
    // right.
    //
    for I in intrinsics::range(NPKEYS, (PKIDX + 1), -1) {
        PPAGE[((KEYBAS + I) + 1)] = PPAGE[(KEYBAS + I)];
    }

    for I in intrinsics::range(NPKEYS, (PKIDX + 1), -1) {
        PPAGE[((DATBAS + I) + 1)] = PPAGE[(DATBAS + I)];
    }

    for I in intrinsics::range((NPKEYS + 1), (PKIDX + 1), -1) {
        PPAGE[((KIDBAS + I) + 1)] = PPAGE[(KIDBAS + I)];
    }

    //
    // Copy in the data pointer from the left child.  Note that
    // no child pointer comes along.
    //
    PPAGE[(DATBAS + PKIDX)] = C1PAGE[((TRDATC + LSIZE) + 1)];

    //
    // Set the key value at PKIDX.  The value exceeds that of the
    // preceding key, if any, by one more than the size of the subtree
    // headed by the left child.  That size is one less than
    // LDELTA, since LDELTA includes the key at location LSIZE+1.
    //
    if (PKIDX == 1) {
        PPAGE[(KEYBAS + PKIDX)] = LDELTA;
    } else {
        PPAGE[(KEYBAS + PKIDX)] = (PPAGE[((KEYBAS + PKIDX) - 1)] + LDELTA);
    }

    //
    // Copy in the data pointer from the right child.  Again, note that
    // no child pointer comes along.
    //
    PPAGE[((DATBAS + PKIDX) + 1)] = C2PAGE[((TRDATC + RSHIFT) + 1)];

    //
    // Set the key value at PKIDX+1.  The value exceeds that of the
    // preceding key by one more than the size of the subtree headed by
    // the middle child.
    //
    PPAGE[((KEYBAS + PKIDX) + 1)] = (((PPAGE[(KEYBAS + PKIDX)] + LNMOVE) + RNMOVE) + 2);

    //
    // The child pointer at PKIDX+1 does get set:  it points to the
    // middle child.
    //
    PPAGE[((KIDBAS + PKIDX) + 1)] = NEW;

    //
    // Remarkably, the only required change to the parent's metadata is
    // updating the key count.  At this point, we can set the overflow
    // flag, depending on the status of the parent.
    //
    if (PARENT == ROOT) {
        PPAGE[TRNKR] = (PPAGE[TRNKR] + 1);
        *OVERFL = (PPAGE[TRNKR] == (MXKEYR + 1));
    } else {
        PPAGE[TRNKC] = (PPAGE[TRNKC] + 1);
        *OVERFL = (PPAGE[TRNKC] == (MXKEYC + 1));
    }

    //
    // Update the metadata in the first child.  This node has lost
    // just enough keys to give it size LSIZE.
    //
    C1PAGE[TRNKC] = LSIZE;

    //
    // For safety, clean out the vacated key and pointer locations.
    // Clear the overflow addresses as well.
    //
    CLEARI(
        ((MXKEYC + 1) - LSIZE),
        C1PAGE.subarray_mut(((TRKEYC + LSIZE) + 1)),
    );
    CLEARI(
        ((MXKEYC + 1) - LSIZE),
        C1PAGE.subarray_mut(((TRDATC + LSIZE) + 1)),
    );
    CLEARI(
        ((MXKIDC + 1) - (LSIZE + 1)),
        C1PAGE.subarray_mut(((TRKIDC + LSIZE) + 2)),
    );

    //
    // The first child is set.
    //
    // To adjust the second child, we must shift the keys and pointers
    // left to fill in the vacated space.  The keys in this second child
    // must be adjusted to account for the loss of the predecessors
    // moved to the middle child and the parent.
    //
    // Shift elements starting from the left.
    //
    for I in 1..=RSIZE {
        C2PAGE[(TRKEYC + I)] = (C2PAGE[(((TRKEYC + RSHIFT) + 1) + I)] - (RNMOVE + 1));
    }

    for I in 1..=RSIZE {
        C2PAGE[(TRDATC + I)] = C2PAGE[(((TRDATC + RSHIFT) + 1) + I)];
    }

    for I in 1..=(RSIZE + 1) {
        C2PAGE[(TRKIDC + I)] = C2PAGE[(((TRKIDC + RSHIFT) + 1) + I)];
    }

    //
    // Update the key count in the second child.  This node has lost
    // just enough keys to give it size RSIZE.
    //
    C2PAGE[TRNKC] = RSIZE;

    //
    // For safety, clean out the vacated key and pointer locations.
    // Clear the overflow addresses as well.
    //
    CLEARI(
        ((MXKEYC + 1) - RSIZE),
        C2PAGE.subarray_mut(((TRKEYC + RSIZE) + 1)),
    );
    CLEARI(
        ((MXKEYC + 1) - RSIZE),
        C2PAGE.subarray_mut(((TRDATC + RSIZE) + 1)),
    );
    CLEARI(
        ((MXKIDC + 1) - (RSIZE + 1)),
        C2PAGE.subarray_mut(((TRKIDC + RSIZE) + 2)),
    );

    //
    // The second child is set.
    //
    // The last change we must make is to update the node count in
    // the root.
    //
    if (PARENT == ROOT) {
        PPAGE[TRNNOD] = (PPAGE[TRNNOD] + 1);
    } else {
        //
        // We won't read in the whole root page; we'll just get the
        // base address of the root and update the affected location.
        //
        RBASE = ZZEKTRBS(ROOT, ctx)?;

        DASRDI(
            HANDLE,
            (RBASE + TRNNOD),
            (RBASE + TRNNOD),
            std::slice::from_mut(&mut NNODE),
            ctx,
        )?;
        DASUDI(
            HANDLE,
            (RBASE + TRNNOD),
            (RBASE + TRNNOD),
            &[(NNODE + 1)],
            ctx,
        )?;
    }

    //
    // Write out our updates.
    //
    ZZEKPGWI(HANDLE, PARENT, PPAGE.as_slice(), ctx)?;
    ZZEKPGWI(HANDLE, LEFT, C1PAGE.as_slice(), ctx)?;
    ZZEKPGWI(HANDLE, RIGHT, C2PAGE.as_slice(), ctx)?;
    ZZEKPGWI(HANDLE, NEW, C3PAGE.as_slice(), ctx)?;

    Ok(())
}
