//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBPOOL: i32 = -5;
const SIZROW: i32 = 1;
const SIZCOL: i32 = 0;
const NFRROW: i32 = 2;
const NFRCOL: i32 = 0;
const FREROW: i32 = 1;
const FRECOL: i32 = -1;
const FORWRD: i32 = 1;
const BCKWRD: i32 = 2;
const FREE: i32 = 0;

/// LNK, extract sublist from list
///
/// Extract a specified sublist from a list.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HEAD,
///  TAIL       I   Head and tail nodes of a sublist to be extracted.
///  POOL      I-O  A doubly linked list pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HEAD,
///  TAIL     are, respectively, the head and tail nodes of a
///           sublist to be extracted.
///
///  POOL     is a doubly linked list pool.
/// ```
///
/// # Detailed Output
///
/// ```text
///  POOL     is the input pool, with the following
///           modifications:
///
///              -- The sublist bounded by HEAD and
///                 by TAIL is now a separate list from
///                 the list that originally contained it.
///
///              If on input, HEAD was preceded by the node
///              PREV, and tail was followed by the node
///              NEXT, then on output
///
///              -- The successor of PREV is NEXT.
///              -- The predecessor of NEXT is PREV.
/// ```
///
/// # Parameters
///
/// ```text
///  LBPOOL   is the lower bound of the column indices of the POOL
///           array. The columns indexed LBPOOL to 0 are reserved
///           as a control area for the pool.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If either of HEAD or TAIL are not valid node numbers, the
///      error SPICE(INVALIDNODE) is signaled. POOL will not be
///      modified.
///
///  2)  If either of HEAD or TAIL are valid node numbers but are not
///      allocated, the error SPICE(UNALLOCATEDNODE) is signaled. POOL
///      will not be modified.
///
///  3)  If TAIL cannot be reached by forward traversal of the list
///      containing HEAD, the error SPICE(INVALIDSUBLIST) is signaled.
///      POOL will not be modified.
/// ```
///
/// # Particulars
///
/// ```text
///  Extracting a sublist from a list is necessary when a list is
///  to be re-arranged in some way. For example, to move a node
///  in a list to the head of the list, the node (which is a
///  singleton sublist) is first extracted from the list containing
///  it, then inserted before the head of the list.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Let POOL be a doubly linked list pool, and let
///
///         9 <--> 8 <--> 4 <--> 2000 <--> 1
///
///      be a list in POOL. To extract the sublist
///
///         4 <--> 2000
///
///      the call
///
///         CALL LNKXSL ( 4, 2000, POOL )
///
///      can be used. After the call is made, POOL will contain the
///      separate lists
///
///         9 <--> 8 <--> 1
///
///      and
///
///         4 <--> 2000
///
///
///  2)  Let POOL be a doubly linked list pool, and let
///
///         9 <--> 8 <--> 4 <--> 2000 <--> 1
///
///      be a list in POOL. To move the node 2000 to the
///      head of the list, the code fragment
///
///         CALL LNKXSL ( 2000, 2000, POOL )
///         CALL LNKILB ( 2000, 9,    POOL )
///
///      can be used. The resulting list will be
///
///         2000 <--> 9 <--> 8 <--> 4 <--> 1
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Linked list pools must be initialized via the routine
///      LNKINI. Failure to initialize a linked list pool
///      will almost certainly lead to confusing results.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 24-NOV-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 19-DEC-1995 (NJB) (WLT)
/// ```
pub fn lnkxsl(
    ctx: &mut SpiceContext,
    head: i32,
    tail: i32,
    pool: &mut [[i32; 2]],
) -> crate::Result<()> {
    LNKXSL(head, tail, pool.as_flattened_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LNKXSL ( LNK, extract sublist from list  )
pub fn LNKXSL(HEAD: i32, TAIL: i32, POOL: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut POOL = DummyArrayMut2D::new(POOL, 1..=2, LBPOOL..);
    let mut NEXT: i32 = 0;
    let mut NODE: i32 = 0;
    let mut PREV: i32 = 0;

    //
    // Local parameters
    //

    //
    // The control area contains 3 elements.  They are:
    //
    //    The "size" of the pool, that is, the number
    //    of nodes in the pool.
    //
    //    The number of free nodes in the pool.
    //
    //    The "free pointer," which is the column index of the first free
    //    node.
    //
    // Parameters defining the row and column indices of these control
    // elements are given below.
    //

    //
    // Each assigned node consists of a backward pointer and a forward
    // pointer.
    //
    //    +-------------+       +-------------+       +-------------+
    //    |  forward--> |       |  forward--> |       |  forward--> |
    //    +-------------+  ...  +-------------+  ...  +-------------+
    //    | <--backward |       | <--backward |       | <--backward |
    //    +-------------+       +-------------+       +-------------+
    //
    //        node 1                 node I              node SIZE
    //
    //
    //

    //
    // Free nodes say that that's what they are.  The way they say it
    // is by containing the value FREE in their backward pointers.
    // Needless to say, FREE is a value that cannot be a valid pointer.
    //

    //
    // Local variables
    //

    //
    // HEAD and TAIL must be valid node numbers.  These nodes
    // must be allocated as well.
    //
    if ((((HEAD < 1) || (HEAD > POOL[[SIZROW, SIZCOL]])) || (TAIL < 1))
        || (TAIL > POOL[[SIZROW, SIZCOL]]))
    {
        CHKIN(b"LNKXSL", ctx)?;
        SETMSG(b"HEAD was #.  TAIL was #. Valid range is 1 to #.", ctx);
        ERRINT(b"#", HEAD, ctx);
        ERRINT(b"#", TAIL, ctx);
        ERRINT(b"#", POOL[[SIZROW, SIZCOL]], ctx);
        SIGERR(b"SPICE(INVALIDNODE)", ctx)?;
        CHKOUT(b"LNKXSL", ctx)?;
        return Ok(());
    } else if ((POOL[[BCKWRD, HEAD]] == FREE) || (POOL[[BCKWRD, TAIL]] == FREE)) {
        CHKIN(b"LNKXSL", ctx)?;
        SETMSG(b"Node HEAD: node number = #; backward pointer = #;  forward pointer = #. Node TAIL: node number = #; backward pointer = #;  forward pointer = #. (\"FREE\" is #)", ctx);
        ERRINT(b"#", HEAD, ctx);
        ERRINT(b"#", POOL[[BCKWRD, HEAD]], ctx);
        ERRINT(b"#", POOL[[FORWRD, HEAD]], ctx);
        ERRINT(b"#", TAIL, ctx);
        ERRINT(b"#", POOL[[BCKWRD, TAIL]], ctx);
        ERRINT(b"#", POOL[[FORWRD, TAIL]], ctx);
        ERRINT(b"#", FREE, ctx);
        SIGERR(b"SPICE(UNALLOCATEDNODE)", ctx)?;
        CHKOUT(b"LNKXSL", ctx)?;
        return Ok(());
    }

    //
    // Starting at HEAD, search forward, looking for TAIL (apologies to
    // ZZ Top).
    //
    NODE = HEAD;

    while ((NODE != TAIL) && (NODE > 0)) {
        NODE = POOL[[FORWRD, NODE]];
    }

    //
    // If we didn't find TAIL, that's an error.
    //
    if (NODE != TAIL) {
        CHKIN(b"LNKXSL", ctx)?;
        SETMSG(
            b"Node # cannot be found by forward traversal, starting at node #.",
            ctx,
        );
        ERRINT(b"#", TAIL, ctx);
        ERRINT(b"#", HEAD, ctx);
        SIGERR(b"SPICE(INVALIDSUBLIST)", ctx)?;
        CHKOUT(b"LNKXSL", ctx)?;
        return Ok(());
    }

    //
    // We reached TAIL.  Extract the sublist between HEAD and TAIL
    // inclusive.
    //
    // Find the predecessor of HEAD and the successor of TAIL.
    //
    PREV = POOL[[BCKWRD, HEAD]];
    NEXT = POOL[[FORWRD, TAIL]];

    //
    // If the input list did not start with HEAD, then we must update
    // the forward pointer of the tail node, as well as the backward
    // pointer of the head node, of the sublist that preceded HEAD.
    //
    if (PREV > 0) {
        //
        // Update the forward pointer of PREV with the forward pointer of
        // TAIL.
        //
        // If TAIL had a successor, the predecessor of HEAD will now
        // point forward to it.  If TAIL was the tail of the input list,
        // the forward pointer of TAIL was the negative of the head of
        // the input list---this is the correct forward pointer for the
        // predecessor of HEAD in this case, since the predecessor of
        // HEAD will become the tail of the main list after the sublist
        // ranging from HEAD to TAIL is removed.
        //
        POOL[[FORWRD, PREV]] = NEXT;

        //
        // If TAIL is the tail of the input list, we must update the
        // backward pointer of the head of the input list to point to
        // the negative of the new tail of the list, which now is PREV.
        //
        if (NEXT <= 0) {
            //
            // In this case, we can read off the number of the head
            // node from NEXT:  it is just -NEXT.
            //
            POOL[[BCKWRD, -NEXT]] = -PREV;
        }
    }

    //
    // The portion of the input list that preceded HEAD (if such
    // portion existed) has now been taken care of.
    //
    // We now must perform the analogous updates to the portion of
    // the input list that followed TAIL.
    //
    // If the input list did not end with TAIL, then we must update
    // the backward pointer of the head node, as well as the forward
    // pointer of the tail node, of the sublist that followed TAIL.
    //
    if (NEXT > 0) {
        //
        // Update the backward pointer of NEXT with the backward pointer
        // of HEAD.
        //
        // If HEAD had a predecessor, the successor of TAIL will now
        // point backward to it.  If HEAD was the head of the input list,
        // the backward pointer of HEAD was the negative of the tail of
        // the input list---this is the correct backward pointer for the
        // successor of TAIL in this case, since the successor of TAIL
        // will become the head of the main list after the sublist
        // ranging from HEAD to TAIL is removed.
        //
        POOL[[BCKWRD, NEXT]] = PREV;

        //
        // If HEAD is the head of the input list, we must update the
        // forward pointer of the tail of the input list to point to
        // the negative of the new head of the list, which now is NEXT.
        //
        if (PREV <= 0) {
            //
            // In this case, we can read off the number of the tail
            // node from PREV:  it is just -PREV.
            //
            POOL[[FORWRD, -PREV]] = -NEXT;
        }
    }
    //
    // The portion of the input list that followed TAIL (if such
    // portion existed) has now been taken care of.
    //

    //
    // Cauterize the sublist.
    //
    POOL[[BCKWRD, HEAD]] = -TAIL;
    POOL[[FORWRD, TAIL]] = -HEAD;

    Ok(())
}
