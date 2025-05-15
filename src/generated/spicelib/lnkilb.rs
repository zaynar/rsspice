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

/// LNK, insert list before node
///
/// Insert the list containing a specified node into a another list,
/// preceding a specified node.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  LIST       I   Node in the list to be inserted.
///  NEXT       I   Node before which a new list is to be inserted.
///  POOL      I-O  A doubly linked list pool.
///  LBPOOL     P   Lower bound of pool column indices.
/// ```
///
/// # Detailed Input
///
/// ```text
///  LIST     is a node in the list to be inserted. The entire
///           list containing LIST is to be inserted into the
///           list containing NEXT. The inserted list will be
///           located between NEXT and its predecessor, if any.
///
///  NEXT     is a node in a list. NEXT is permitted to be
///           nil, in which case POOL is not modified.
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
///                 -- The successor of TAIL is NEXT.
///
///                 -- The predecessor of NEXT is TAIL.
///
///
///              Let PREV be the node that on input was the
///              predecessor of NEXT; if PREV exists, then on
///              output
///
///                 -- The successor of PREV is HEAD.
///
///                 -- The predecessor of HEAD is PREV.
///
///              If PREV is nil, the backward pointer of the
///              inserted sublist is set to the negative of
///              the tail of the list containing NEXT.
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
///  For efficiency, discovery check-in is used in this routine.
///
///  1)  If LIST is not a valid node number, the error
///      SPICE(INVALIDNODE) is signaled. POOL will not be
///      modified.
///
///  2)  If NEXT is positive but is not a valid node number, the error
///      SPICE(INVALIDNODE) is signaled. POOL will not be
///      modified.
///
///  3)  It is not an error for NEXT to be non-positive; if it is,
///      the call to this routine does not affect the pool.
///
///  4)  If either of LIST or NEXT are valid node numbers but are not
///      allocated, the error SPICE(UNALLOCATEDNODE) is signaled. POOL
///      will not be modified.
///
///  5)  If LIST belongs to the same list as does NEXT, this routine
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
///  To insert a list into a list AFTER a specified element, use the
///  routine LNKILA.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Let POOL be a doubly linked list pool that contains the lists
///
///          3 <--> 7 <--> 1    and    500 <--> 2 <--> 80
///
///      To insert the second list into the first before node 7, use
///      the call
///
///          CALL LNKILB ( 500, 7, POOL )
///
///      The resulting list will be:
///
///          3 <--> 500 <--> 2 <--> 80 <--> 7 <--> 1
///
///
///  2)  Let POOL be a doubly linked list pool that contains 5 nodes.
///      The sequence of calls
///
///         HEAD = 0
///
///         DO I = 1, 5
///            CALL LNKAN  ( POOL, NODE       )
///            CALL LNKILB ( NODE, HEAD, POOL )
///            HEAD = NODE
///         END DO
///
///      builds the list
///
///          5 <--> 4 <--> 3 <--> 2 <--> 1
///
///      Note that the first call to LNKILB does not cause an error
///      to be signaled, even though HEAD is 0 at that point.
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
pub fn lnkilb(
    ctx: &mut SpiceContext,
    list: i32,
    next: i32,
    pool: &mut [[i32; 2]],
) -> crate::Result<()> {
    LNKILB(list, next, pool.as_flattened_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LNKILB ( LNK, insert list before node )
pub fn LNKILB(LIST: i32, NEXT: i32, POOL: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut POOL = DummyArrayMut2D::new(POOL, 1..=2, LBPOOL..);
    let mut HEAD: i32 = 0;
    let mut PREV: i32 = 0;
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
    // If NEXT is non-positive, return now.
    //
    if (NEXT <= 0) {
        return Ok(());
    }

    //
    // If we arrived here, NEXT and LIST must be valid node numbers.
    // These nodes must be allocated as well.
    //
    if (((NEXT > POOL[[SIZROW, SIZCOL]]) || (LIST < 1)) || (LIST > POOL[[SIZROW, SIZCOL]])) {
        CHKIN(b"LNKILB", ctx)?;
        SETMSG(b"NEXT was #.  LIST was #. Valid range is 1 to #.", ctx);
        ERRINT(b"#", NEXT, ctx);
        ERRINT(b"#", LIST, ctx);
        ERRINT(b"#", POOL[[SIZROW, SIZCOL]], ctx);
        SIGERR(b"SPICE(INVALIDNODE)", ctx)?;
        CHKOUT(b"LNKILB", ctx)?;
        return Ok(());
    } else if ((POOL[[BCKWRD, NEXT]] == FREE) || (POOL[[BCKWRD, LIST]] == FREE)) {
        CHKIN(b"LNKILB", ctx)?;
        SETMSG(b"Node NEXT: node number = #; backward pointer = #;  forward pointer = #. Node LIST: node number = #; backward pointer = #;  forward pointer = #. (\"FREE\" is #)", ctx);
        ERRINT(b"#", NEXT, ctx);
        ERRINT(b"#", POOL[[BCKWRD, NEXT]], ctx);
        ERRINT(b"#", POOL[[FORWRD, NEXT]], ctx);
        ERRINT(b"#", LIST, ctx);
        ERRINT(b"#", POOL[[BCKWRD, LIST]], ctx);
        ERRINT(b"#", POOL[[FORWRD, LIST]], ctx);
        ERRINT(b"#", FREE, ctx);
        SIGERR(b"SPICE(UNALLOCATEDNODE)", ctx)?;
        CHKOUT(b"LNKILB", ctx)?;
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
    // Let PREV be the backward pointer of NEXT.
    //
    // Insert TAIL before NEXT.
    //
    // If NEXT has a predecessor, HEAD follows it.
    //
    // If NEXT has no predecessor, HEAD is the new head of the list.
    // The forward pointer of the tail of the merged list should
    // be set to -HEAD.
    //
    // In either case, the backward pointer of HEAD should be set
    // to the backward pointer of NEXT.
    //
    //
    PREV = POOL[[BCKWRD, NEXT]];
    POOL[[FORWRD, TAIL]] = NEXT;
    POOL[[BCKWRD, NEXT]] = TAIL;

    if (PREV > 0) {
        POOL[[FORWRD, PREV]] = HEAD;
    } else {
        POOL[[FORWRD, -PREV]] = -HEAD;
    }

    POOL[[BCKWRD, HEAD]] = PREV;

    Ok(())
}
