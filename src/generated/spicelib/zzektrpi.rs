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

//$Procedure      ZZEKTRPI ( EK tree, parent information )
pub fn ZZEKTRPI(
    HANDLE: i32,
    TREE: i32,
    KEY: i32,
    PARENT: &mut i32,
    PKEY: &mut i32,
    POFFST: &mut i32,
    LPIDX: &mut i32,
    LPKEY: &mut i32,
    LSIB: &mut i32,
    RPIDX: &mut i32,
    RPKEY: &mut i32,
    RSIB: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut CHILD: i32 = 0;
    let mut LKEY: i32 = 0;
    let mut MAXKEY: i32 = 0;
    let mut NEWKEY: i32 = 0;
    let mut OFFSET: i32 = 0;
    let mut PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut PREV: i32 = 0;
    let mut PRVKEY: i32 = 0;
    let mut TOTKEY: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in in this puppy.
    //
    // Nothing found to begin with.
    //
    FOUND = false;

    //
    // Get a local copy of the input key.  We may overwrite the input
    // key when we set PKEY.
    //
    LKEY = KEY;

    //
    // Start out by reading in the root page.  The node level starts
    // out at 1.
    //
    ZZEKPGRI(HANDLE, TREE, PAGE.as_slice_mut(), ctx)?;

    *PARENT = 0;
    *PKEY = 0;
    *POFFST = 0;
    *LPIDX = 0;
    *LPKEY = 0;
    *LSIB = 0;
    *RPIDX = 0;
    *RPKEY = 0;
    *RSIB = 0;

    //
    // Find out how many keys are in the tree.  If LKEY is outside
    // this range, we won't find it.
    //
    TOTKEY = PAGE[TRNKEY];

    if ((LKEY < 1) || (LKEY > TOTKEY)) {
        CHKIN(b"ZZEKTRPI", ctx)?;
        SETMSG(b"Key = #; valid range = 1:#. Tree = #, file = #", ctx);
        ERRINT(b"#", LKEY, ctx);
        ERRINT(b"#", TOTKEY, ctx);
        ERRINT(b"#", TREE, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"ZZEKTRPI", ctx)?;
        return Ok(());
    }

    //
    // Find the last key at this level that is less than or equal to
    // the requested key.
    //
    PREV = LSTLEI(LKEY, PAGE[TRNKR], PAGE.subarray((TRKEYR + 1)));

    if (PREV > 0) {
        PRVKEY = PAGE[(TRKEYR + PREV)];
    } else {
        PRVKEY = 0;
    }

    //
    // If we were lucky enough to get an exact match, we can quit now.
    // The root has no parent so the output values remain set to zero.
    //
    if (PRVKEY == LKEY) {
        return Ok(());
    }

    //
    // Still here?  Traverse the pointer path until we find the key
    // or run out of progeny.
    //
    OFFSET = PRVKEY;
    *PARENT = TREE;
    *PKEY = PAGE[(TRKEYR + 1)];
    MAXKEY = PAGE[TRNKR];

    if (PREV > 0) {
        *LPIDX = PREV;
        *LPKEY = PAGE[(TRKEYR + *LPIDX)];
        *LSIB = PAGE[(TRKIDR + *LPIDX)];
    } else {
        *LPIDX = 0;
        *LPKEY = 0;
        *LSIB = 0;
    }

    if (PREV < MAXKEY) {
        *RPIDX = (PREV + 1);
        *RPKEY = PAGE[(TRKEYR + *RPIDX)];
        *RSIB = PAGE[((TRKIDR + *RPIDX) + 1)];
    } else {
        *RPIDX = 0;
        *RPKEY = 0;
        *RSIB = 0;
    }

    CHILD = PAGE[((TRKIDR + PREV) + 1)];
    FOUND = false;

    while ((CHILD > 0) && !FOUND) {
        //
        // Read in the child page.
        //
        ZZEKPGRI(HANDLE, CHILD, PAGE.as_slice_mut(), ctx)?;

        //
        // Find the last key at this level that is less than or equal to
        // the requested key.  Since the keys we're looking at now are
        // ordinal positions relative to the subtree whose root is the
        // current node, we must subtract from LKEY the position of the
        // node preceding the first key of this subtree.
        //
        NEWKEY = (LKEY - OFFSET);
        PREV = LSTLEI(NEWKEY, PAGE[TRNKC], PAGE.subarray((TRKEYC + 1)));

        if (PREV > 0) {
            PRVKEY = PAGE[(TRKEYC + PREV)];
        } else {
            PRVKEY = 0;
        }

        //
        // If we were lucky enough to get an exact match, we can quit.
        // The outputs are set.
        //
        if (PRVKEY == NEWKEY) {
            FOUND = true;
        } else {
            //
            // Record information from the current node before we read the
            // next child page.
            //
            *PARENT = CHILD;
            *POFFST = OFFSET;
            *PKEY = (PAGE[(TRKEYC + 1)] + OFFSET);
            MAXKEY = PAGE[TRNKC];

            if (PREV > 0) {
                *LPIDX = PREV;
                *LPKEY = PAGE[(TRKEYC + *LPIDX)];
                *LSIB = PAGE[(TRKIDC + *LPIDX)];
            } else {
                *LPIDX = 0;
                *LPKEY = 0;
                *LSIB = 0;
            }

            if (PREV < MAXKEY) {
                *RPIDX = (PREV + 1);
                *RPKEY = PAGE[(TRKEYC + *RPIDX)];
                *RSIB = PAGE[((TRKIDC + *RPIDX) + 1)];
            } else {
                *RPIDX = 0;
                *RPKEY = 0;
                *RSIB = 0;
            }

            //
            // Update the offset of the tree headed by CHILD, and set
            // the new child node.
            //
            OFFSET = (PRVKEY + OFFSET);
            CHILD = PAGE[((TRKIDC + PREV) + 1)];
        }
    }

    //
    // If we found the key, our outputs are already set.  If not, we've
    // got trouble.
    //
    if !FOUND {
        CHKIN(b"ZZEKTRPI", ctx)?;
        SETMSG(b"Key #; valid range = 1:#. Tree = #, file = #.  Key was not found.  This probably indicates a corrupted file or a bug in the EK code.", ctx);
        ERRINT(b"#", LKEY, ctx);
        ERRINT(b"#", TOTKEY, ctx);
        ERRINT(b"#", TREE, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(ITEMNOTFOUND)", ctx)?;
        CHKOUT(b"ZZEKTRPI", ctx)?;
        return Ok(());
    }

    Ok(())
}
