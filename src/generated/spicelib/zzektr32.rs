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
const LSIZE: i32 = ((3 * MNKEYC) / 2);
const RSIZE: i32 = ((3 * MNKEYC) - LSIZE);

//$Procedure      ZZEKTR32 ( EK tree, 3-2 merge )
pub fn ZZEKTR32(
    HANDLE: i32,
    TREE: i32,
    LEFT: i32,
    MIDDLE: i32,
    RIGHT: i32,
    PARENT: i32,
    LPKIDX: i32,
    UNDRFL: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut C1PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut C2PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut C3PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut DATBAS: i32 = 0;
    let mut KEYBAS: i32 = 0;
    let mut KIDBAS: i32 = 0;
    let mut LEFTSZ: i32 = 0;
    let mut LMIDSZ: i32 = 0;
    let mut LPKEY: i32 = 0;
    let mut LSIB: i32 = 0;
    let mut MIDSIZ: i32 = 0;
    let mut MSIB: i32 = 0;
    let mut N: i32 = 0;
    let mut NLKEYS: i32 = 0;
    let mut NMKEYS: i32 = 0;
    let mut NNODE: i32 = 0;
    let mut NPKEYS: i32 = 0;
    let mut NRKEYS: i32 = 0;
    let mut PPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut PSIZE: i32 = 0;
    let mut RBASE: i32 = 0;
    let mut RMIDSZ: i32 = 0;
    let mut ROOT: i32 = 0;
    let mut RPKEY: i32 = 0;
    let mut RSHIFT: i32 = 0;
    let mut RSIB: i32 = 0;
    let mut SIZBAS: i32 = 0;
    let mut SUM: i32 = 0;

    //
    // Non-SPICELIB functions
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
    // The plan is to take three sibling nodes, two of which contain
    // the minimum number of keys and one of which is underflowing by one
    // key, and to merge these nodes into two nodes.  This process
    // reduces the number of nodes in the parent by one and may cause the
    // parent to underflow.
    //
    // After the merge, the sum of the numbers of keys in the two
    // children will be exactly (3*MNKEYC).  The numbers of keys in the
    // left and right nodes will be, respectively:
    //
    //
    //    LSIZE  =   INT (  (3*MNKEYC)/2  )
    //    RSIZE  =   (3*MNKEYC)  - LSIZE
    //
    // We need to be sure that LSIZE and RSIZE are in the range
    //
    //    MNKEYC : MXKEYC
    //
    //
    // The definition of LSIZE implies that
    //
    //    LSIZE  =  MNKEYC + INT ( MNKEYC/2 )
    //
    //
    // so
    //
    //    MNKEYC + INT ( MNKEYC/2 )  <  LSIZE <  (3/2)*MNKEYC
    //                               -        -
    //
    // and since
    //
    //    MNKEYC  =  MNKIDC - 1
    //            =  INT ( ( 2*MXKIDC + 1 ) / 3 ) - 1
    //            =  INT ( ( 2*MXKEYC + 3 ) / 3 ) - 1
    //            =  INT ( ( 2*MXKEYC     ) / 3 )
    //
    // we have
    //
    //    (3/2) * MNKEYC  =  (3/2) * INT ( (2*MXKEYC) / 3 )  <  MXKEYC
    //                                                       -
    //
    // Thus  LSIZE is guaranteed to be in range.
    //
    // When MNKEYC is even, RSIZE is equal to LSIZE and thus is
    // within bounds.  When MNKEYC is odd, RSIZE exceeds LSIZE by 1, so
    //
    //    MNKEYC  <  RSIZE
    //
    //
    // It remains to be shown that
    //
    //    RSIZE   <  MXKEYC
    //            -
    //
    // when MNKEYC is odd.  When this is the case, the quantity
    //
    //    (3/2) * MNKEYC
    //
    // is not an integer and therefore is strictly less than MXKEYC.
    // This quantity is also greater than LSIZE, so we conclude that
    //
    //    LSIZE  <  MXKEYC - 1
    //           -
    //
    // Since RSIZE exceeds LSIZE by 1, we have
    //
    //    RSIZE  <  MXKEYC
    //           -
    //
    // as we claimed.
    //
    //
    // All right, read in the child and parent pages.
    //
    ZZEKPGRI(HANDLE, LEFT, C1PAGE.as_slice_mut(), ctx)?;
    ZZEKPGRI(HANDLE, MIDDLE, C2PAGE.as_slice_mut(), ctx)?;
    ZZEKPGRI(HANDLE, RIGHT, C3PAGE.as_slice_mut(), ctx)?;
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
        SIZBAS = TRNKR;
    } else {
        KEYBAS = TRKEYC;
        DATBAS = TRDATC;
        KIDBAS = TRKIDC;
        SIZBAS = TRNKC;
    }

    //
    // Check the left parent key of the middle child.
    //
    PSIZE = PPAGE[SIZBAS];

    if ((LPKIDX < 1) || (LPKIDX > (PSIZE - 1))) {
        CHKIN(b"ZZEKTR32", ctx)?;
        SETMSG(
            b"Left parent key of MIDDLE is out of range.  Value is #; valid range is 1:#",
            ctx,
        );
        ERRINT(b"#", LPKIDX, ctx);
        ERRINT(b"#", (PSIZE - 1), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTR32", ctx)?;
        return Ok(());
    }

    //
    // Retain the left and right parent key values of the middle child.
    //
    LPKEY = PPAGE[(KEYBAS + LPKIDX)];
    RPKEY = PPAGE[((KEYBAS + LPKIDX) + 1)];

    //
    // Verify that LEFT, MIDDLE, and RIGHT are siblings, and that PARENT
    // is their common parent.
    //
    LSIB = PPAGE[(KIDBAS + LPKIDX)];
    MSIB = PPAGE[((KIDBAS + LPKIDX) + 1)];
    RSIB = PPAGE[((KIDBAS + LPKIDX) + 2)];

    if (((LSIB != LEFT) || (MSIB != MIDDLE)) || (RSIB != RIGHT)) {
        CHKIN(b"ZZEKTR32", ctx)?;
        SETMSG(b"LEFT, RIGHT, MIDDLE, PARENT, and PKIDX are inconsistent. LEFT = #; MIDDLE = #; RIGHT = #; PARENT = #; LPKIDX = #; LSIB derived from PARENT = #; MSIB = #; RSIB = #.", ctx);
        ERRINT(b"#", LEFT, ctx);
        ERRINT(b"#", MIDDLE, ctx);
        ERRINT(b"#", RIGHT, ctx);
        ERRINT(b"#", PARENT, ctx);
        ERRINT(b"#", LPKIDX, ctx);
        ERRINT(b"#", LSIB, ctx);
        ERRINT(b"#", MSIB, ctx);
        ERRINT(b"#", RSIB, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTR32", ctx)?;
        return Ok(());
    }

    //
    // Get the number of keys in the parent.
    //
    if (PARENT == ROOT) {
        NPKEYS = PPAGE[TRNKR];
    } else {
        NPKEYS = PPAGE[TRNKC];
    }

    //
    // Get the number of keys in each child.
    //
    NLKEYS = C1PAGE[TRNKC];
    NMKEYS = C2PAGE[TRNKC];
    NRKEYS = C3PAGE[TRNKC];

    SUM = ((NLKEYS + NMKEYS) + NRKEYS);

    //
    // The sum of the number of keys in the three input nodes must
    // sum exactly to value representing an underflow level of 1 key.
    //
    if (SUM != ((3 * MNKEYC) - 1)) {
        CHKIN(b"ZZEKTR32", ctx)?;
        SETMSG(b"Number of keys in nodes LEFT = #; in MIDDLE = #; in RIGHT = #; counts summing to # were expected.", ctx);
        ERRINT(b"#", NLKEYS, ctx);
        ERRINT(b"#", NMKEYS, ctx);
        ERRINT(b"#", NRKEYS, ctx);
        ERRINT(b"#", ((3 * MNKEYC) - 1), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTR32", ctx)?;
        return Ok(());
    }

    //
    // We're set to carry out the merge.  Here's an overview of what
    // gets moved where.
    //
    //    The left parent key of the middle node moves into the left
    //    node, at the end of the node.
    //
    //    The first N-1 keys and N child pointers of the middle node get
    //    moved into the left node, where
    //
    //       N  =  LSIZE - ( 1 + NLKEYS ) + 1
    //
    //    The Nth key of the middle node moves into the parent,
    //    replacing the left parent key of the middle node.
    //
    //    The right parent key of the middle node moves into the right
    //    node, at the beginning of the node.
    //
    //    The keys from position N+1 onward in the middle node, as
    //    well as all of the remaining child pointers, move into the
    //    right node, at the beginning.
    //
    //    The right parent key's location is filled in by shifting
    //    the keys, data pointers, and child pointers in the parent
    //    to the left by one position.  The child pointer removed by this
    //    operation is the pointer to the middle child.
    //
    //    The middle child node disappears.
    //
    // Before re-arranging things, we'll need to have on hand the key
    // counts for various sets of keys.  We'll use the variable LEFTSZ
    // for the number of keys in the subtree headed by LEFT.  We'll
    // use the variable LMIDSZ to refer to the `subtree' headed by
    // the set of keys in the middle node that will be shifted into
    // the left child.  The variable RMSIZE will represent the size of
    // the key set moved from the middle child into the right child.
    // MIDSIZ will be the key count for the subtree headed by the middle
    // child.
    //
    // Consistent with usage above, the variable N will represent
    // the index of the key in the middle node that will rapturously
    // ascend into the parent.
    //

    if (LPKIDX == 1) {
        LEFTSZ = (LPKEY - 1);
    } else {
        LEFTSZ = ((LPKEY - PPAGE[((KEYBAS + LPKIDX) - 1)]) - 1);
    }

    N = ((LSIZE - (1 + NLKEYS)) + 1);

    LMIDSZ = (C2PAGE[(TRKEYC + N)] - 1);
    MIDSIZ = ((RPKEY - LPKEY) - 1);
    RMIDSZ = ((MIDSIZ - LMIDSZ) - 1);

    //
    // Move the left parent key into the left child.  The key itself
    // doesn't really move; its value is simply re-assigned.  The
    // data pointer is copied, however.  The child pointer at location
    // LSIZE+1 is unaffected by this move.
    //
    C1PAGE[((TRKEYC + NLKEYS) + 1)] = (LEFTSZ + 1);
    C1PAGE[((TRDATC + NLKEYS) + 1)] = PPAGE[(DATBAS + LPKIDX)];

    //
    // Move the first N-1 keys and data pointers, and the first N
    // child pointers, from the middle child into the left
    // child.  The moved keys will gain LEFTSZ + 1 predecessors.
    //
    for I in 1..=(N - 1) {
        C1PAGE[(((TRKEYC + NLKEYS) + 1) + I)] = ((C2PAGE[(TRKEYC + I)] + LEFTSZ) + 1);
    }

    MOVEI(
        C2PAGE.subarray((TRDATC + 1)),
        (N - 1),
        C1PAGE.subarray_mut(((TRDATC + NLKEYS) + 2)),
    );
    MOVEI(
        C2PAGE.subarray((TRKIDC + 1)),
        N,
        C1PAGE.subarray_mut(((TRKIDC + NLKEYS) + 2)),
    );

    //
    // Set the key count in the left child.
    //
    C1PAGE[TRNKC] = LSIZE;

    //
    // The left child is complete.  Now it's time to set up the right
    // child.  First off, we'll shift the node's contents to the right
    // by the number of new keys we're going to insert.  Shift the
    // rightmost elements first.  The shifted keys will gain RMIDSZ+1
    // predecessors, so  we adjust the keys as we shift them.
    //
    RSHIFT = ((NMKEYS - N) + 1);

    for I in intrinsics::range(NRKEYS, 1, -1) {
        C3PAGE[((TRKEYC + RSHIFT) + I)] = ((C3PAGE[(TRKEYC + I)] + RMIDSZ) + 1);
    }

    for I in intrinsics::range(NRKEYS, 1, -1) {
        C3PAGE[((TRDATC + RSHIFT) + I)] = C3PAGE[(TRDATC + I)];
    }

    for I in intrinsics::range((NRKEYS + 1), 1, -1) {
        C3PAGE[((TRKIDC + RSHIFT) + I)] = C3PAGE[(TRKIDC + I)];
    }

    //
    // The key at location RSHIFT receives the former right parent key
    // of the middle child.   The key value is simply assigned; the
    // data pointer is copied.  The child pointer at location RSHIFT
    // will be set later.
    //
    C3PAGE[(TRKEYC + RSHIFT)] = (RMIDSZ + 1);
    C3PAGE[(TRDATC + RSHIFT)] = PPAGE[((DATBAS + LPKIDX) + 1)];

    //
    // The first RSHIFT-1 locations in the right child are filled in
    // with data from the middle child.  The moved keys lose LMIDSZ+1
    // predecessors.
    //
    for I in 1..=(RSHIFT - 1) {
        C3PAGE[(TRKEYC + I)] = ((C2PAGE[((TRKEYC + N) + I)] - LMIDSZ) - 1);
    }

    MOVEI(
        C2PAGE.subarray(((TRDATC + N) + 1)),
        (RSHIFT - 1),
        C3PAGE.subarray_mut((TRDATC + 1)),
    );
    MOVEI(
        C2PAGE.subarray(((TRKIDC + N) + 1)),
        RSHIFT,
        C3PAGE.subarray_mut((TRKIDC + 1)),
    );

    //
    // Update the key count in the right child.
    //
    C3PAGE[TRNKC] = RSIZE;

    //
    // The right child is complete.  It's time to update the parent.
    //
    // The key at location N in the middle child replaces the left parent
    // key.  The key value is actually re-assigned; the data pointer does
    // move.  The left parent key increases by the number of keys moved
    // into the subtree headed by the left child.
    //
    PPAGE[(KEYBAS + LPKIDX)] = ((LPKEY + LMIDSZ) + 1);
    PPAGE[(DATBAS + LPKIDX)] = C2PAGE[(TRDATC + N)];

    //
    // The parent keys, data pointers, and child pointers at locations
    // LPKIDX+2 onward get shifted left by one position.  The keys lose
    // no  predecessors as the result of these re-arrangements.
    //
    for I in (LPKIDX + 1)..=(NPKEYS - 1) {
        PPAGE[(KEYBAS + I)] = PPAGE[((KEYBAS + I) + 1)];
    }

    for I in (LPKIDX + 1)..=(NPKEYS - 1) {
        PPAGE[(DATBAS + I)] = PPAGE[((DATBAS + I) + 1)];
    }

    for I in (LPKIDX + 1)..=NPKEYS {
        PPAGE[(KIDBAS + I)] = PPAGE[((KIDBAS + I) + 1)];
    }

    //
    // Zero out the freed locations.
    //
    PPAGE[(KEYBAS + NPKEYS)] = 0;
    PPAGE[(DATBAS + NPKEYS)] = 0;
    PPAGE[((KIDBAS + NPKEYS) + 1)] = 0;

    //
    // The only required change to the parent's metadata is
    // updating the key count.  At this point, we can set the
    // underflow flag, depending on the status of the parent.
    //
    if (PARENT == ROOT) {
        PPAGE[TRNKR] = (PPAGE[TRNKR] - 1);
        *UNDRFL = (PPAGE[TRNKR] == 0);
    } else {
        PPAGE[TRNKC] = (PPAGE[TRNKC] - 1);
        *UNDRFL = (PPAGE[TRNKC] == (MNKEYC - 1));
    }

    //
    // The last change we must make is to update the node count in
    // the root.
    //
    if (PARENT == ROOT) {
        PPAGE[TRNNOD] = (PPAGE[TRNNOD] - 1);
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
            &[(NNODE - 1)],
            ctx,
        )?;
    }

    //
    // Write out our updates.
    //
    ZZEKPGWI(HANDLE, PARENT, PPAGE.as_slice(), ctx)?;
    ZZEKPGWI(HANDLE, LEFT, C1PAGE.as_slice(), ctx)?;
    ZZEKPGWI(HANDLE, RIGHT, C3PAGE.as_slice(), ctx)?;

    //
    // Free the page used by the middle child.
    //
    ZZEKPGFR(HANDLE, INT, MIDDLE, ctx)?;

    Ok(())
}
