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

//$Procedure      ZZEKTR1S ( EK tree, one-shot load )
pub fn ZZEKTR1S(
    HANDLE: i32,
    TREE: i32,
    SIZE: i32,
    VALUES: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let VALUES = DummyArray::new(VALUES, 1..);
    let mut BASE: i32 = 0;
    let mut BASIDX: i32 = 0;
    let mut BIGSIZ: i32 = 0;
    let mut CHILD: i32 = 0;
    let mut D: i32 = 0;
    let mut DIV: i32 = 0;
    let mut KEY: i32 = 0;
    let mut KIDBAS: i32 = 0;
    let mut LEVEL: i32 = 0;
    let mut MAXSIZ: i32 = 0;
    let mut N: i32 = 0;
    let mut NBIG: i32 = 0;
    let mut NEXT: i32 = 0;
    let mut NKEYS: i32 = 0;
    let mut NKIDS: i32 = 0;
    let mut NNODES: i32 = 0;
    let mut NODE: i32 = 0;
    let mut NPRED: i32 = 0;
    let mut NSMALL: i32 = 0;
    let mut PAGE = StackArray::<i32, 256>::new(1..=PGSIZI);
    let mut Q: i32 = 0;
    let mut REQSIZ: i32 = 0;
    let mut S: i32 = 0;
    let mut STLSIZ = StackArray::<i32, 10>::new(1..=TRMXDP);
    let mut STNBIG = StackArray::<i32, 10>::new(1..=TRMXDP);
    let mut STNEXT = StackArray::<i32, 10>::new(1..=TRMXDP);
    let mut STNKEY = StackArray::<i32, 10>::new(1..=TRMXDP);
    let mut STNODE = StackArray::<i32, 10>::new(1..=TRMXDP);
    let mut STNBAS = StackArray::<i32, 10>::new(1..=TRMXDP);
    let mut SUBD: i32 = 0;
    let mut SUBSIZ: i32 = 0;
    let mut TOTNOD: i32 = 0;
    let mut TSIZE: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Non-SPICELIB functions
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

    CHKIN(b"ZZEKTR1S", ctx)?;

    //
    // Make sure the input tree is empty.
    //
    TSIZE = ZZEKTRSZ(HANDLE, TREE, ctx)?;

    if (TSIZE > 0) {
        SETMSG(b"Tree has size #; should be empty.EK = #; TREE = #.", ctx);
        ERRINT(b"#", TSIZE, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", TREE, ctx);
        SIGERR(b"SPICE(NONEMPTYTREE)", ctx)?;
        CHKOUT(b"ZZEKTR1S", ctx)?;
        return Ok(());
    }

    //
    // Compute the tree depth required.  The largest tree of a given
    // depth D contains the root node plus S(D) child nodes, where
    //
    //
    //    S(1)  =  1
    //
    //
    // and if D is at least 2,
    //                                D - 2
    //                                ____
    //                                \              i
    //    S(D)  =  MAX_SIZE      *    /      MAX_SIZE
    //                     Root       ----           Child
    //                                i = 0
    //
    //
    //                                D - 2
    //                                ____
    //                                \            i
    //          =  MXKIDR        *    /      MXKIDC
    //                                ----
    //                                i = 0
    //
    //
    //                                     D-1
    //                               MXKIDC   -  1
    //          =  MXKIDR        *   -------------
    //                                MXKIDC  - 1
    //
    //
    // If all of these nodes are full, the number of keys that
    // can be held in this tree is
    //
    //    MXKEYR  +  S(D) * MXKEYC
    //
    // We want the minimum value of D such that this expression
    // is greater than or equal to SIZE.
    //

    TSIZE = MXKEYR;
    D = 1;
    S = 1;

    while (TSIZE < SIZE) {
        D = (D + 1);

        if (D == 2) {
            S = MXKEYR;
        } else {
            //
            // For computational purposes, the relationship
            //
            //    S(D+1)  =   MXKIDR  +  MXKIDC * S(D)
            //
            // is handy.
            //
            //
            S = (MXKIDR + (MXKIDC * S));
        }

        TSIZE = (MXKEYR + (S * MXKEYC));
    }

    //
    // If the tree must be deeper than we expected, we've a problem.
    //
    if (D > TRMXDP) {
        SETMSG(
            b"Tree has depth #; max supported depth is #.EK = #; TREE = #.",
            ctx,
        );
        ERRINT(b"#", D, ctx);
        ERRINT(b"#", TRMXDP, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", TREE, ctx);
        SIGERR(b"SPICE(COUNTTOOLARGE)", ctx)?;
        CHKOUT(b"ZZEKTR1S", ctx)?;
        return Ok(());
    }

    //
    // The basic error checks are done.  At this point, we can build the
    // tree.
    //
    // The approach is to fill in the tree in a top-down fashion.
    // We decide how big each subtree of the root will be; this
    // information allows us to decide which keys actually belong
    // in the root.  Having filled in the root, we repeat the process
    // for each subtree of the root in left-to-right order.
    //
    // We use a stack to keep track of the ancestors of the
    // node we're currently considering.  The table below shows the
    // items we save on the stack and the stack variables associated
    // with those items:
    //
    //
    //    Item                                 Stack Variable
    //    ----                                 ---------------
    //    Node number                          STNODE
    //
    //    Number of keys in node               STNKEY
    //
    //    Larger subtree size                  STLSIZ
    //
    //    Number of large subtrees             STNBIG
    //
    //    Index of next subtree to visit       STNEXT
    //
    //    Base index of node                   STNBAS
    //
    //
    NODE = TREE;
    SUBSIZ = SIZE;
    NEXT = 1;
    LEVEL = 1;
    BASIDX = 0;

    while (LEVEL > 0) {
        //
        // At this point, LEVEL, NEXT, NODE, SUBSIZ and BASIDX are set.
        //

        if (NEXT == 1) {
            //
            // This node has not been visited yet.  We'll fill in this
            // node before proceeding to fill in its descendants.  The
            // first step is to compute the number and sizes of the
            // subtrees of this node.
            //
            // Decide the large subtree size and the number of subtrees of
            // this node.  The depth SUBD of the subtrees of this node is
            // D - LEVEL.  Each subtree has size bounded by the sizes of
            // the subtree of depth SUBD in which all nodes contain MNKEYC
            // keys and the by the subtree of depth SUBD in which all nodes
            // contain MXKEYC keys.  If this node is not the root and is
            // not a leaf node, the number of subtrees must be between
            // MNKIDC and MXKIDC.
            //
            if (LEVEL == 1) {
                //
                // We're working on the root.  The number of subtrees is
                // anywhere between 0 and MXKIDR, inclusive.  We'll create
                // a tree with the minimum number of subtrees of the root.
                //
                if (D > 1) {
                    //
                    // We'll find the number of subtrees of maximum size
                    // that we would need to hold the non-root keys of the
                    // tree.  We'll then determine the actual required sizes
                    // of these subtrees.
                    //
                    SUBD = (D - 1);

                    NNODES = 0;

                    for I in 1..=SUBD {
                        NNODES = ((MXKIDC * NNODES) + 1);
                    }

                    MAXSIZ = (NNODES * MXKEYC);

                    //
                    // If we had NKIDS subtrees of size MAXSIZ, NKIDS
                    // would be the smallest integer such that
                    //
                    //    ( NKIDS - 1 )  +  NKIDS * MAXSIZ  >  SUBSIZ
                    //                                      -
                    //
                    // or equivalently,
                    //
                    //     NKIDS * ( MAXSIZ + 1 )  >  SUBSIZ + 1
                    //                             -
                    //
                    // We'll compute this value of NKIDS.
                    //
                    //
                    Q = (SUBSIZ + 1);
                    DIV = (MAXSIZ + 1);
                    NKIDS = (((Q + DIV) - 1) / DIV);

                    //
                    // The minimum number of keys we must store in child
                    // nodes is the number of keys in the tree, minus those
                    // that can be accommodated in the root:
                    //
                    N = (SUBSIZ - (NKIDS - 1));

                    //
                    // Now we can figure out how large the subtrees would
                    // have to be in order to hold N keys, if all subtrees
                    // had the same size.
                    //
                    BIGSIZ = (((N + NKIDS) - 1) / NKIDS);

                    //
                    // We may have more capacity than we need if all subtrees
                    // have size BIGSIZ.  So, we'll allow some subtrees to
                    // have size BIGSIZ-1.  Not all subtrees can have the
                    // smaller size (otherwise BIGSIZ would have been
                    // smaller).  The first NBIG subtrees will have the
                    // larger size.
                    //
                    NSMALL = ((NKIDS * BIGSIZ) - N);
                    NBIG = (NKIDS - NSMALL);

                    NKEYS = (NKIDS - 1);
                } else {
                    //
                    // All keys are in the root.
                    //
                    NKEYS = SIZE;
                    NKIDS = 0;
                }

                //
                // Read in the root page.
                //
                ZZEKPGRI(HANDLE, TREE, PAGE.as_slice_mut(), ctx)?;

                //
                // We have enough information to fill in the root node.
                // We'll allocate nodes for the immediate children.
                // There is one key `between' each child pointer.
                //
                for I in 1..=NKEYS {
                    //
                    // The Ith key may be found by considering the number
                    // of keys in the subtree between the Ith key and its
                    // predecessor in the root.
                    //
                    if (I == 1) {
                        NPRED = 0;
                    } else {
                        NPRED = PAGE[((TRKEYR + I) - 1)];
                    }

                    if (D > 1) {
                        //
                        // The tree contains subtrees.
                        //
                        if (I <= NBIG) {
                            KEY = ((NPRED + BIGSIZ) + 1);
                        } else {
                            KEY = (NPRED + BIGSIZ);
                        }
                    } else {
                        KEY = I;
                    }

                    PAGE[(TRKEYR + I)] = KEY;
                    PAGE[(TRDATR + I)] = VALUES[KEY];
                }

                TOTNOD = 1;

                for I in 1..=NKIDS {
                    //
                    // Allocate a node for the Ith child.  Store pointers
                    // to these nodes.
                    //
                    ZZEKPGAL(HANDLE, INT, &mut CHILD, &mut BASE, ctx)?;
                    PAGE[(TRKIDR + I)] = CHILD;
                    TOTNOD = (TOTNOD + 1);
                }

                //
                // Fill in the root's metadata.  There is one item that
                // we'll have to fill in when we're done:  the number of
                // nodes in the tree.  We know the rest of the information
                // now.
                //
                PAGE[TRNKEY] = SIZE;
                PAGE[TRDPTH] = D;
                PAGE[TRKEYR] = NKEYS;
                PAGE[TRNNOD] = 0;

                //
                // Write out the root.
                //
                ZZEKPGWI(HANDLE, TREE, PAGE.as_slice(), ctx)?;
            } else if (LEVEL < D) {
                //
                // The current node is a non-leaf child node.
                //
                CLEARI(PGSIZI, PAGE.as_slice_mut());
                //
                // The tree headed by this node has depth D-LEVEL+1 and
                // must hold SUBSIZ keys.  We must figure out the size
                // and number of subtrees of the current node.  Unlike in
                // the case of the root, we must have between MNKIDC
                // and MXKIDC subtrees of this node.  We start out by
                // computing the required subtree size if there were
                // exactly MNKIDC subtrees.  In this case, the total
                // number of keys in the subtrees would be
                //
                //    SUBSIZ  -  MNKEYC
                //
                //
                N = (SUBSIZ - MNKEYC);
                REQSIZ = (((N + MNKEYC) - 1) / MNKEYC);

                //
                // Compute the maximum allowable number of keys in
                // a subtree.
                //
                SUBD = (D - LEVEL);

                NNODES = 0;

                for I in 1..=SUBD {
                    NNODES = ((MXKIDC * NNODES) + 1);
                }

                MAXSIZ = (NNODES * MXKEYC);

                //
                // If the number REQSIZ we came up with is a valid size,
                // we'll be able to get the correct number of children
                // by using subtrees of size REQSIZ and REQSIZ-1.  Note
                // that it's impossible for REQSIZ to be too small,
                // since the smallest possible number of subtrees is
                // MNKIDC.
                //
                if (REQSIZ <= MAXSIZ) {
                    //
                    // Decide how many large and small subtrees we need.
                    //
                    NKIDS = MNKIDC;
                    BIGSIZ = REQSIZ;
                    NSMALL = ((BIGSIZ * NKIDS) - N);
                    NBIG = (NKIDS - NSMALL);
                } else {
                    //
                    //
                    // See how many subtrees of size MAXSIZ it would take
                    // to hold the requisite number of keys.  We know the
                    // number is more than MNKIDC.  If we have NKIDS
                    // subtrees of size MAXSIZ, the total number of
                    // keys in the subtree headed by NODE is
                    //
                    //   ( NKIDS - 1 )  +  ( NKIDS * MAXSIZ )
                    //
                    // or
                    //
                    //     NKIDS * ( MAXSIZ + 1 )   -   1
                    //
                    // We must find the smallest value of NKIDS such
                    // that the above quantity is greater than or equal
                    // to SUBSIZ.
                    //
                    Q = (SUBSIZ + 1);
                    DIV = (MAXSIZ + 1);
                    NKIDS = (((Q + DIV) - 1) / DIV);

                    //
                    // We know that NKIDS subtrees of size MAXSIZ, plus
                    // NKIDS-1 keys in NODE, can hold at least SUBSIZ
                    // keys.  We now want to find the smallest subtree
                    // size such that NKIDS subtrees of that size,
                    // together with the NKIDS-1 keys in NODE, contain
                    // at least SUBSIZ keys.  The size we seek will
                    // become BIGSIZ, the larger of the two subtree
                    // sizes we'll use.  So BIGSIZ is the smallest
                    // integer such that
                    //
                    //    ( NKIDS - 1 ) + ( NKIDS * BIGSIZ )  >  SUBSIZ
                    //                                        -
                    //
                    // or equivalently
                    //
                    //    BIGSIZ * NKIDS  >  SUBSIZ - NKIDS + 1
                    //                    -
                    //
                    Q = ((SUBSIZ - NKIDS) + 1);
                    DIV = NKIDS;
                    BIGSIZ = (((Q + DIV) - 1) / DIV);

                    NSMALL = ((BIGSIZ * NKIDS) - Q);
                    NBIG = (NKIDS - NSMALL);
                }

                //
                // Fill in the keys for the current node.
                //
                NKEYS = (NKIDS - 1);

                for I in 1..=NKEYS {
                    //
                    // The Ith key may be found by considering the number
                    // of keys in the subtree between the Ith key and its
                    // predecessor in the current node.
                    //
                    if (I == 1) {
                        NPRED = BASIDX;
                    } else {
                        NPRED = (BASIDX + PAGE[((TRKEYC + I) - 1)]);
                    }

                    if (I <= NBIG) {
                        KEY = ((NPRED + BIGSIZ) + 1);
                    } else {
                        KEY = (NPRED + BIGSIZ);
                    }

                    PAGE[(TRKEYC + I)] = (KEY - BASIDX);
                    PAGE[(TRDATC + I)] = VALUES[KEY];
                }

                for I in 1..=NKIDS {
                    //
                    // Allocate a node for the Ith child.  Store pointers
                    // to these nodes.
                    //
                    ZZEKPGAL(HANDLE, INT, &mut CHILD, &mut BASE, ctx)?;

                    PAGE[(TRKIDC + I)] = CHILD;
                    TOTNOD = (TOTNOD + 1);
                }

                //
                // We can now fill in the metadata for the current node.
                //
                PAGE[TRNKC] = NKEYS;

                ZZEKPGWI(HANDLE, NODE, PAGE.as_slice(), ctx)?;
            }

            //
            // Unless the current node is a leaf node, prepare to visit
            // the first child of the current node.
            //
            if (LEVEL < D) {
                //
                // Push our current state.
                //
                STNODE[LEVEL] = NODE;
                STNKEY[LEVEL] = NKEYS;
                STLSIZ[LEVEL] = BIGSIZ;
                STNBIG[LEVEL] = NBIG;
                STNEXT[LEVEL] = 2;
                STNBAS[LEVEL] = BASIDX;

                //
                // NEXT is already set to 1.  BASIDX is set, since the
                // base index of the first child is that of the parent.
                //
                if (LEVEL == 1) {
                    KIDBAS = TRKIDR;
                } else {
                    KIDBAS = TRKIDC;
                }

                LEVEL = (LEVEL + 1);
                NODE = PAGE[(KIDBAS + 1)];
                SUBSIZ = BIGSIZ;
            } else if (LEVEL > 1) {
                //
                // The current node is a child leaf node.  There are no
                // calculations to do; we simply assign keys and pointers,
                // write out metadata, and pop our state.
                //
                NKEYS = SUBSIZ;

                for I in 1..=NKEYS {
                    KEY = (BASIDX + I);
                    PAGE[(TRKEYC + I)] = I;
                    PAGE[(TRDATC + I)] = VALUES[KEY];
                }

                //
                // We can now fill in the metadata for the current node.
                //
                PAGE[TRNKC] = NKEYS;

                ZZEKPGWI(HANDLE, NODE, PAGE.as_slice(), ctx)?;

                //
                // A leaf node is a subtree unto itself, and we're
                // done with this subtree.  Pop our state.
                //
                LEVEL = (LEVEL - 1);

                if (LEVEL >= 1) {
                    NODE = STNODE[LEVEL];
                    NKEYS = STNKEY[LEVEL];
                    BIGSIZ = STLSIZ[LEVEL];
                    NBIG = STNBIG[LEVEL];
                    NEXT = STNEXT[LEVEL];
                    BASIDX = STNBAS[LEVEL];
                    NKIDS = (NKEYS + 1);

                    //
                    // Read in the current node.
                    //
                    ZZEKPGRI(HANDLE, NODE, PAGE.as_slice_mut(), ctx)?;
                }
            } else {
                //
                // The only node is the root.  Pop out.
                //
                LEVEL = 0;
            }
        //
        // We've decided which node to go to next at this point.
        // At this point, LEVEL, NEXT, NODE, SUBSIZ and BASIDX are set.
        //
        } else {
            //
            // The current node has been visited already.  Visit the
            // next child, if there is one.
            //
            if (NEXT <= NKIDS) {
                //
                // Prepare to visit the next child of the current node.
                //
                STNEXT[LEVEL] = (NEXT + 1);

                if (LEVEL == 1) {
                    KIDBAS = TRKIDR;
                } else {
                    KIDBAS = TRKIDC;
                }

                NODE = PAGE[(KIDBAS + NEXT)];

                if (NEXT <= NBIG) {
                    SUBSIZ = STLSIZ[LEVEL];
                } else {
                    SUBSIZ = (STLSIZ[LEVEL] - 1);
                }

                if (NEXT <= (NBIG + 1)) {
                    BASIDX = ((STNBAS[LEVEL] + ((NEXT - 1) * STLSIZ[LEVEL])) + (NEXT - 1));
                } else {
                    BASIDX = (((STNBAS[LEVEL] + (NBIG * STLSIZ[LEVEL]))
                        + (((NEXT - NBIG) - 1) * (STLSIZ[LEVEL] - 1)))
                        + (NEXT - 1));
                }

                LEVEL = (LEVEL + 1);
                NEXT = 1;

            //
            // LEVEL, NEXT, NODE, SUBSIZ, and BASIDX are set.
            //
            } else {
                //
                // We're done with the current subtree.  Pop the stack.
                //
                LEVEL = (LEVEL - 1);

                if (LEVEL >= 1) {
                    NODE = STNODE[LEVEL];
                    NKEYS = STNKEY[LEVEL];
                    BIGSIZ = STLSIZ[LEVEL];
                    NBIG = STNBIG[LEVEL];
                    NEXT = STNEXT[LEVEL];
                    BASIDX = STNBAS[LEVEL];
                    NKIDS = (NKEYS + 1);
                    //
                    // Read in the current node.
                    //
                    ZZEKPGRI(HANDLE, NODE, PAGE.as_slice_mut(), ctx)?;
                }
            }
        }

        //
        // On this pass through the loop, we either---
        //
        //    - Visited a node for the first time and filled in the
        //      node.
        //
        //    - Advanced to a new node that has not yet been visited.
        //
        //    - Exited from a completed subtree.
        //
        // Each of these actions can be performed a finite number of
        // times.  Therefore, we made progress toward loop termination.
        //
    }

    //
    // The last chore is setting the total number of nodes in the root.
    //
    BASE = ZZEKTRBS(TREE, ctx)?;

    DASUDI(HANDLE, (BASE + TRNNOD), (BASE + TRNNOD), &[TOTNOD], ctx)?;

    CHKOUT(b"ZZEKTR1S", ctx)?;
    Ok(())
}
