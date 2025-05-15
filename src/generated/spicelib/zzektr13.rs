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

//$Procedure      ZZEKTR13 ( EK tree, 1-3 split )
pub fn ZZEKTR13(HANDLE: i32, TREE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BASE: i32 = 0;
    let mut C1PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut C2PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut CHILD = StackArray::<i32, 2>::new(1..=2);
    let mut DELTA: i32 = 0;
    let mut MIDDLE: i32 = 0;
    let mut NRKEYS: i32 = 0;
    let mut ROOT: i32 = 0;
    let mut RPAGE = StackArray::<i32, 256>::new(1..=PGSIZI);

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
    // The number of keys in the root must correspond exactly to an
    // overflow level of 1 key.
    //
    if (NRKEYS != (MXKEYR + 1)) {
        CHKIN(b"ZZEKTR13", ctx)?;
        SETMSG(b"Number of keys in root = #; should be #.", ctx);
        ERRINT(b"#", NRKEYS, ctx);
        ERRINT(b"#", (MXKEYR + 1), ctx);
        SIGERR(b"SPICE(BUG)", ctx)?;
        CHKOUT(b"ZZEKTR13", ctx)?;
        return Ok(());
    }

    //
    // Allocate two new pages; these will become children of the root.
    // Each one will be assigned MNKEYC keys.
    //
    for I in 1..=2 {
        ZZEKPGAL(HANDLE, INT, &mut CHILD[I], &mut BASE, ctx)?;
    }

    //
    // Set the key count in the first child.
    //
    CLEARI(PGSIZI, C1PAGE.as_slice_mut());

    C1PAGE[TRNKC] = MNKEYC;

    //
    // Copy in the keys, data pointers, and child pointers from the
    // first MNKEYC locations in the root.  Also take the left child
    // pointer of the middle key.
    //
    MOVEI(
        RPAGE.subarray((TRKEYR + 1)),
        MNKEYC,
        C1PAGE.subarray_mut((TRKEYC + 1)),
    );
    MOVEI(
        RPAGE.subarray((TRDATR + 1)),
        MNKEYC,
        C1PAGE.subarray_mut((TRDATC + 1)),
    );
    MOVEI(
        RPAGE.subarray((TRKIDR + 1)),
        (MNKEYC + 1),
        C1PAGE.subarray_mut((TRKIDC + 1)),
    );

    //
    // Set up the key count in the second child.
    //
    CLEARI(PGSIZI, C2PAGE.as_slice_mut());

    C2PAGE[TRNKC] = MNKEYC;

    //
    // Copy in the keys, data pointers, and child pointers from the
    // last MNKEYC locations in the root.  Also take the last right
    // child pointer.
    //
    MIDDLE = (MNKEYC + 1);

    MOVEI(
        RPAGE.subarray(((TRKEYR + MIDDLE) + 1)),
        MNKEYC,
        C2PAGE.subarray_mut((TRKEYC + 1)),
    );
    MOVEI(
        RPAGE.subarray(((TRDATR + MIDDLE) + 1)),
        MNKEYC,
        C2PAGE.subarray_mut((TRDATC + 1)),
    );
    MOVEI(
        RPAGE.subarray(((TRKIDR + MIDDLE) + 1)),
        (MNKEYC + 1),
        C2PAGE.subarray_mut((TRKIDC + 1)),
    );

    //
    // The keys in this second node must be adjusted to account for the
    // loss of the predecessors assigned to the subtree headed by the
    // left child, as well as of the middle key.
    //
    DELTA = RPAGE[(TRKEYR + MIDDLE)];

    for I in 1..=MNKEYC {
        C2PAGE[(TRKEYC + I)] = (C2PAGE[(TRKEYC + I)] - DELTA);
    }

    //
    // Now the root must be updated.  The root now contains just 1
    // key; that key should be shifted left to the first key location.
    // There are two child pointers; these point to the children just
    // created.  The depth of the tree has increased, as well as the
    // number of nodes in the tree.
    //
    RPAGE[(TRKEYR + 1)] = RPAGE[(TRKEYR + MIDDLE)];
    RPAGE[(TRDATR + 1)] = RPAGE[(TRDATR + MIDDLE)];
    RPAGE[(TRKIDR + 1)] = CHILD[1];
    RPAGE[(TRKIDR + 2)] = CHILD[2];
    RPAGE[TRNKR] = 1;
    RPAGE[TRDPTH] = (RPAGE[TRDPTH] + 1);
    RPAGE[TRNNOD] = (RPAGE[TRNNOD] + 2);

    CLEARI(MXKEYR, RPAGE.subarray_mut((TRKEYR + 2)));
    CLEARI(MXKEYR, RPAGE.subarray_mut((TRDATR + 2)));
    CLEARI(MXKEYR, RPAGE.subarray_mut((TRKIDR + 3)));

    //
    // Write out our updates.
    //
    ZZEKPGWI(HANDLE, ROOT, RPAGE.as_slice(), ctx)?;
    ZZEKPGWI(HANDLE, CHILD[1], C1PAGE.as_slice(), ctx)?;
    ZZEKPGWI(HANDLE, CHILD[2], C2PAGE.as_slice(), ctx)?;

    Ok(())
}
