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

/// LNK, insert list after node
///
/// Insert the list containing a specified node into a another list,
/// following a specified node.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  PREV       I   Node after which a new list is to be inserted.
///  LIST       I   Node in the list to be inserted.
///  POOL      I-O  A doubly linked list pool.
///  LBPOOL     P   Lower bound of pool column indices.
/// ```
///
/// # Detailed Input
///
/// ```text
///  PREV     is a node in a list. PREV is permitted to be
///           nil, in which case POOL is not modified.
///
///  LIST     is a node in the list to be inserted. The entire
///           list containing the node LIST is to be inserted
///           into the list containing PREV. The inserted list
///           will be located between PREV and its successor,
///           if any.
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
///              Let HEAD and TAIL be the head and tail nodes of
///              the list containing LIST. Then on output
///
///                 -- The successor of PREV is HEAD.
///                 -- The predecessor of HEAD is PREV.
///
///
///              Let NEXT be the node that on input was the
///              successor of PREV; if NEXT exists, then on
///              output
///
///                 -- The successor of TAIL is NEXT.
///                 -- The predecessor of NEXT is TAIL.
///
///              If NEXT is nil, the forward pointer of the
///              inserted sublist is set to the negative of
///              the head of the list containing PREV.
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
///  1)  If LIST is not a valid node number, the error
///      SPICE(INVALIDNODE) is signaled. POOL will not be
///      modified.
///
///  2)  If PREV is positive but is not a valid node number, the error
///      SPICE(INVALIDNODE) is signaled. POOL will not be
///      modified.
///
///  3)  It is not an error for PREV to be non-positive; if it is,
///      the call to this routine does not affect the pool.
///
///  4)  If either of PREV or LIST are valid node numbers but are not
///      allocated, the error SPICE(UNALLOCATEDNODE) is signaled. POOL
///      will not be modified.
///
///  5)  If LIST belongs to the same list as does PREV, this routine
///      may fail in mysterious ways. For efficiency, this error
///      condition is not checked.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is used for augmenting lists by inserting other
///  lists into them. The case of insertion of a single allocated
///  node is not special: this is insertion of a singleton list.
///
///  To insert a list into a list BEFORE a specified element, use the
///  routine LNKILB.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Let POOL be a doubly linked list pool that contains the lists
///
///          3 <--> 7 <--> 1    and    500 <--> 2 <--> 80
///
///      To insert the second list into the first after node 7, use the
///      call
///
///          CALL LNKILA ( 7, 500, POOL )
///
///      The resulting list will be:
///
///          3 <--> 7 <--> 500 <--> 2 <--> 80 <--> 1
///
///
///  2)  Let POOL be a doubly linked list pool that contains 5 nodes.
///      The sequence of calls
///
///         TAIL = 0
///
///         DO I = 1, 5
///            CALL LNKAN  ( POOL, NODE       )
///            CALL LNKILA ( TAIL, NODE, POOL )
///            TAIL = NODE
///         END DO
///
///      builds the list
///
///          1 <--> 2 <--> 3 <--> 4 <--> 5
///
///      Note that the first call to LNKILA does not cause an error
///      to be signaled, even though TAIL is 0 at that point.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Linked list pools must be initialized via the routine
///      LNKINI. Failure to initialize a linked list pool
///      will almost certainly lead to confusing results.
///
///  2)  For efficiency, discovery check-in is used in this routine.
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
///         Added note about efficiency in $Restrictions section.
///
/// -    SPICELIB Version 1.0.0, 19-DEC-1995 (NJB) (WLT)
/// ```
pub fn lnkila(
    ctx: &mut SpiceContext,
    prev: i32,
    list: i32,
    pool: &mut [[i32; 2]],
) -> crate::Result<()> {
    LNKILA(prev, list, pool.as_flattened_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LNKILA ( LNK, insert list after node )
pub fn LNKILA(PREV: i32, LIST: i32, POOL: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut POOL = DummyArrayMut2D::new(POOL, 1..=2, LBPOOL..);
    let mut NEXT: i32 = 0;
    let mut HEAD: i32 = 0;
    let mut TAIL: i32 = 0;

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
    // Use discovery check-in.
    //

    //
    // If PREV is non-positive, return now.
    //
    if (PREV <= 0) {
        return Ok(());
    }

    //
    // At this point, PREV and LIST must be a valid node numbers, and
    // both PREV and LIST must be allocated as well.
    //
    if (((PREV > POOL[[SIZROW, SIZCOL]]) || (LIST < 1)) || (LIST > POOL[[SIZROW, SIZCOL]])) {
        CHKIN(b"LNKILA", ctx)?;
        SETMSG(b"PREV was #.  LIST was #. Valid range is 1 to #.", ctx);
        ERRINT(b"#", PREV, ctx);
        ERRINT(b"#", LIST, ctx);
        ERRINT(b"#", POOL[[SIZROW, SIZCOL]], ctx);
        SIGERR(b"SPICE(INVALIDNODE)", ctx)?;
        CHKOUT(b"LNKILA", ctx)?;
        return Ok(());
    } else if ((POOL[[BCKWRD, PREV]] == FREE) || (POOL[[BCKWRD, LIST]] == FREE)) {
        CHKIN(b"LNKILA", ctx)?;
        SETMSG(b"Node PREV: node number = #; backward pointer = #;  forward pointer = #. Node LIST: node number = #; backward pointer = #;  forward pointer = #. (\"FREE\" is #)", ctx);
        ERRINT(b"#", PREV, ctx);
        ERRINT(b"#", POOL[[BCKWRD, PREV]], ctx);
        ERRINT(b"#", POOL[[FORWRD, PREV]], ctx);
        ERRINT(b"#", LIST, ctx);
        ERRINT(b"#", POOL[[BCKWRD, LIST]], ctx);
        ERRINT(b"#", POOL[[FORWRD, LIST]], ctx);
        ERRINT(b"#", FREE, ctx);
        SIGERR(b"SPICE(UNALLOCATEDNODE)", ctx)?;
        CHKOUT(b"LNKILA", ctx)?;
        return Ok(());
    }

    //
    // Find the head and tail of the list containing LIST.
    //
    HEAD = LIST;

    while (POOL[[BCKWRD, HEAD]] > 0) {
        HEAD = POOL[[BCKWRD, HEAD]];
    }

    TAIL = -POOL[[BCKWRD, HEAD]];

    //
    // Let NEXT be the forward pointer of PREV.
    //
    // Insert HEAD after PREV.
    //
    // If PREV has a successor, TAIL precedes it.
    //
    // If PREV has no successor, TAIL is the new tail of the list.
    // The backward pointer of the head of the merged list should
    // be set to -TAIL.
    //
    // In either case, the forward pointer of TAIL should be set
    // to the forward pointer of PREV.
    //
    NEXT = POOL[[FORWRD, PREV]];
    POOL[[FORWRD, PREV]] = HEAD;
    POOL[[BCKWRD, HEAD]] = PREV;

    if (NEXT > 0) {
        POOL[[BCKWRD, NEXT]] = TAIL;
    } else {
        POOL[[BCKWRD, -NEXT]] = -TAIL;
    }

    POOL[[FORWRD, TAIL]] = NEXT;

    Ok(())
}
