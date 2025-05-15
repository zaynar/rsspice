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
const PARIDX: i32 = 1;
const NKDIDX: i32 = 2;
const BEGIDX: i32 = 3;

//$Procedure      ZZEKTRFR ( EK tree, free )
pub fn ZZEKTRFR(HANDLE: i32, TREE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DEPTH: i32 = 0;
    let mut FIRST: i32 = 0;
    let mut KIDBAS: i32 = 0;
    let mut LEVEL: i32 = 0;
    let mut NKEYS: i32 = 0;
    let mut NKIDS: i32 = 0;
    let mut NODE: i32 = 0;
    let mut PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut REMAIN: i32 = 0;
    let mut STACK = StackArray2D::<i32, 30>::new(1..=3, 1..=TRMXDP);

    //
    // SPICELIB functions
    //

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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZEKTRFR", ctx)?;

    //
    // Read in the root node.
    //
    ZZEKPGRI(HANDLE, TREE, PAGE.as_slice_mut(), ctx)?;

    //
    // Check the depth of the tree.  If the tree is deeper than
    // we expected, we've a problem.
    //
    DEPTH = PAGE[TRDPTH];

    if (DEPTH > TRMXDP) {
        SETMSG(
            b"Tree has depth #; max supported depth is #.EK = #; TREE = #.",
            ctx,
        );
        ERRINT(b"#", DEPTH, ctx);
        ERRINT(b"#", TRMXDP, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", TREE, ctx);
        SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        CHKOUT(b"ZZEKTRFR", ctx)?;
        return Ok(());
    }

    //
    // We traverse the tree in post-order fashion:  at each node,
    // we first delete all of the node's children in left-to-right
    // order, then we delete the node itself.  We use a stack to
    // keep track of the ancestors of the node we're currently
    // considering.
    //
    LEVEL = 1;
    REMAIN = PAGE[TRNNOD];
    NODE = TREE;

    //
    // Initialize the child count and the location of the first
    // child in the current node.  The child count of the root is
    // one more than the number of keys in the root if the root has
    // children; otherwise, the child count is zero.
    //
    NKEYS = PAGE[TRNKR];

    if (DEPTH == 1) {
        NKIDS = 0;
    } else {
        NKIDS = (NKEYS + 1);
    }

    FIRST = 1;

    while (REMAIN > 0) {
        //
        // At this point,
        //
        //    NODE is the current node to consider.
        //    NKIDS is the number of children of NODE.
        //    FIRST is the index of the first child in NODE.
        //
        if (NKIDS > 0) {
            //
            // This node has children, so push the current node, the
            // number of children, and the location of the first child on
            // the stack.  Before incrementing the stack level, determine
            // the base address of the child pointers.
            //
            if (LEVEL == 1) {
                KIDBAS = TRKIDR;
            } else {
                KIDBAS = TRKIDC;
            }

            STACK[[PARIDX, LEVEL]] = NODE;
            STACK[[NKDIDX, LEVEL]] = NKIDS;
            STACK[[BEGIDX, LEVEL]] = FIRST;
            LEVEL = (LEVEL + 1);

            //
            // Read in the first child node.
            //
            NODE = PAGE[(KIDBAS + FIRST)];

            ZZEKPGRI(HANDLE, NODE, PAGE.as_slice_mut(), ctx)?;

            //
            // We've never visited this node before, so the node's
            // metadata is valid, and the first child pointer, if any,
            // is at location 1.
            //
            NKEYS = PAGE[TRNKC];

            if (LEVEL < DEPTH) {
                NKIDS = (NKEYS + 1);
            } else {
                NKIDS = 0;
            }

            FIRST = 1;
        } else {
            //
            // This node has no children.  We can free this page.
            //
            ZZEKPGFR(HANDLE, INT, NODE, ctx)?;

            REMAIN = (REMAIN - 1);

            //
            // Obtain the parent node by popping the stack.
            //
            LEVEL = (LEVEL - 1);

            if (LEVEL > 0) {
                NODE = STACK[[PARIDX, LEVEL]];
                FIRST = STACK[[BEGIDX, LEVEL]];
                NKIDS = STACK[[NKDIDX, LEVEL]];

                //
                // The parent has one less child, and the location of the
                // first child is the successor of the stored location.
                //
                NKIDS = (NKIDS - 1);
                FIRST = (FIRST + 1);

                //
                // The parent page has been overwritten; read it back in.
                //
                ZZEKPGRI(HANDLE, NODE, PAGE.as_slice_mut(), ctx)?;
            }
        }
        //
        // On this pass through the loop, we either visited a node
        // for the first time, or we deleted a node.  Therefore, we
        // made progress toward loop termination.
        //
    }

    CHKOUT(b"ZZEKTRFR", ctx)?;
    Ok(())
}
