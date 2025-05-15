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

//$Procedure      ZZEKTR31 ( EK tree, 3-1 merge )
pub fn ZZEKTR31(HANDLE: i32, TREE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut C1PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut C2PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut CHILD = StackArray::<i32, 2>::new(1..=2);
    let mut DELTA: i32 = 0;
    let mut MIDDLE: i32 = 0;
    let mut NLKEYS: i32 = 0;
    let mut NRKEYS: i32 = 0;
    let mut ROOT: i32 = 0;
    let mut RPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut SUM: i32 = 0;

    //
    // Local variables
    //

    //
    // Use discovery check-in for speed.
    //
    ROOT = TREE;

    ZZEKPGRI(HANDLE, ROOT, RPAGE.as_slice_mut(), ctx)?;

    NRKEYS = RPAGE[TRNKR];

    //
    // There must be exactly 1 key in the root.
    //
    if (NRKEYS != 1) {
        CHKIN(b"ZZEKTR31", ctx)?;
        SETMSG(b"Number of keys in root = #; should be 1.", ctx);
        ERRINT(b"#", NRKEYS, ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTR31", ctx)?;
        return Ok(());
    }

    //
    // Read in the child pages.  Get the key counts for these pages.
    //
    CHILD[1] = RPAGE[(TRKIDR + 1)];
    CHILD[2] = RPAGE[(TRKIDR + 2)];

    ZZEKPGRI(HANDLE, CHILD[1], C1PAGE.as_slice_mut(), ctx)?;
    ZZEKPGRI(HANDLE, CHILD[2], C2PAGE.as_slice_mut(), ctx)?;

    NLKEYS = C1PAGE[TRNKC];
    NRKEYS = C2PAGE[TRNKC];
    SUM = (NLKEYS + NRKEYS);

    //
    // The sum of the number of keys in the two input nodes must
    // sum exactly to value representing an underflow level of 1 key.
    //
    if (SUM != ((2 * MNKEYC) - 1)) {
        CHKIN(b"ZZEKTR31", ctx)?;
        SETMSG(
            b"Number of keys in nodes LEFT = #; in RIGHT = #; counts summing to # were expected.",
            ctx,
        );
        ERRINT(b"#", NLKEYS, ctx);
        ERRINT(b"#", NRKEYS, ctx);
        ERRINT(b"#", ((2 * MNKEYC) - 1), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTR31", ctx)?;
        return Ok(());
    }

    //
    // Shift the key and data pointer in the root to right to allow
    // insertion of NLKEYS new entries on the left.  The child pointers
    // need not be shifted; they'll be overwritten later.
    //
    RPAGE[((TRKEYR + NLKEYS) + 1)] = RPAGE[(TRKEYR + 1)];
    RPAGE[((TRDATR + NLKEYS) + 1)] = RPAGE[(TRDATR + 1)];

    //
    // Copy in the keys, data pointers, and child pointers from the
    // left child into the root.  The number of predecessors of the
    // new keys is unchanged by this operation.
    //
    MOVEI(
        C1PAGE.subarray((TRKEYC + 1)),
        NLKEYS,
        RPAGE.subarray_mut((TRKEYR + 1)),
    );
    MOVEI(
        C1PAGE.subarray((TRDATC + 1)),
        NLKEYS,
        RPAGE.subarray_mut((TRDATR + 1)),
    );
    MOVEI(
        C1PAGE.subarray((TRKIDC + 1)),
        (NLKEYS + 1),
        RPAGE.subarray_mut((TRKIDR + 1)),
    );

    //
    // Copy in the keys, data pointers, and child pointers from the
    // right child into the root.  The number of predecessors of the
    // new keys is increased by the value of the last key already
    // present.
    //
    MIDDLE = (NLKEYS + 1);
    DELTA = RPAGE[(TRKEYR + MIDDLE)];

    for I in 1..=NRKEYS {
        RPAGE[((TRKEYR + MIDDLE) + I)] = (C2PAGE[(TRKEYC + I)] + DELTA);
    }

    MOVEI(
        C2PAGE.subarray((TRDATC + 1)),
        NRKEYS,
        RPAGE.subarray_mut(((TRDATR + MIDDLE) + 1)),
    );
    MOVEI(
        C2PAGE.subarray((TRKIDC + 1)),
        (NRKEYS + 1),
        RPAGE.subarray_mut(((TRKIDR + MIDDLE) + 1)),
    );

    //
    // Now the root must be updated.  The root now contains
    // the maximum allowed number of keys.  The depth of the tree
    // has decreased, as well as the number of nodes in the tree.
    //
    RPAGE[TRNKR] = MXKEYR;
    RPAGE[TRDPTH] = (RPAGE[TRDPTH] - 1);
    RPAGE[TRNNOD] = (RPAGE[TRNNOD] - 2);

    //
    // Write out the updated root.
    //
    ZZEKPGWI(HANDLE, ROOT, RPAGE.as_slice(), ctx)?;

    //
    // Free the pages occupied by the deleted children.
    //
    for I in 1..=2 {
        ZZEKPGFR(HANDLE, INT, CHILD[I], ctx)?;
    }

    Ok(())
}
