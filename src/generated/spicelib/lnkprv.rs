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

/// LNK, previous node
///
/// Find the node preceding a specified node in a doubly linked list
/// pool.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NODE       I   Number of an allocated node.
///  POOL       I   A doubly linked list pool.
///  LBPOOL     P   Lower bound of pool column indices.
///
///  The function returns the number of the predecessor of the node
///  indicated by NODE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NODE     is the number of an allocated node in POOL.
///
///  POOL     is a doubly linked list pool.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the number of the predecessor of the node
///  indicated by NODE. If NODE is the head node of a list, the
///  function returns the negative of the node number of the tail
///  of the list.
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
///  1)  If NODE is the head node of a list, the function returns the
///      negative of the node number of the tail of the list.
///
///  2)  If NODE is not a valid node number, the error
///      SPICE(INVALIDNODE) is signaled. The value 0 is returned.
///
///  3)  If NODE is not the number of an allocated node, the error
///      SPICE(UNALLOCATEDNODE) is signaled. The value 0 is returned.
/// ```
///
/// # Particulars
///
/// ```text
///  The raison d'etre of this routine is to allow backward traversal
///  of lists in a doubly linked list pool.
///
///  Traversing a list is often performed in cases where the list is
///  used to index elements of a data structure, and the elements
///  indexed by the list must be searched.
///
///  To traverse a list in forward order, use LNKNXT.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Let POOL be doubly linked list pool, and let
///
///        3 <--> 7 <--> 1
///
///      be a list in the pool. The table below shows the effects
///      of function references to LNKPRV, where nodes in this list
///      are used as inputs:
///
///         Function reference               Value Returned
///         ------------------               --------------
///
///         LNKPRV ( 1, POOL )                     7
///         LNKPRV ( 7, POOL )                     3
///         LNKPRV ( 3, POOL )                    -1
///
///
///  2)  Backward traversal of a list: Let POOL be a doubly linked
///      list pool, and let NODE be an allocated node in the pool.
///      To traverse the list containing NODE in backward order
///      and print out the nodes of the list, we can use the
///      following code fragment:
///
///         C
///         C     Find the tail of the list containing NODE.
///         C
///               PREV = LNKTL ( NODE, POOL )
///
///         C
///         C     Traverse the list, printing out node numbers
///         C     as we go.
///         C
///               WRITE (*,*) 'The list, in backward order, is: '
///
///               DO WHILE ( PREV .GT. 0 )
///
///                  WRITE (*,*) PREV
///                  PREV = LNKPRV ( PREV, POOL )
///
///               END DO
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
pub fn lnkprv(ctx: &mut SpiceContext, node: i32, pool: &[[i32; 2]]) -> crate::Result<i32> {
    let ret = LNKPRV(node, pool.as_flattened(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure LNKPRV ( LNK, previous node )
pub fn LNKPRV(NODE: i32, POOL: &[i32], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let POOL = DummyArray2D::new(POOL, 1..=2, LBPOOL..);
    let mut LNKPRV: i32 = 0;

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
    // If the node is out of range, something's very wrong.
    //
    if ((NODE < 1) || (NODE > POOL[[SIZROW, SIZCOL]])) {
        LNKPRV = 0;

        CHKIN(b"LNKPRV", ctx)?;
        SETMSG(b"NODE was #; valid range is 1 to #.", ctx);
        ERRINT(b"#", NODE, ctx);
        ERRINT(b"#", POOL[[SIZROW, SIZCOL]], ctx);
        SIGERR(b"SPICE(INVALIDNODE)", ctx)?;
        CHKOUT(b"LNKPRV", ctx)?;
        return Ok(LNKPRV);

    //
    // We don't do free nodes.
    //
    } else if (POOL[[BCKWRD, NODE]] == FREE) {
        LNKPRV = 0;

        CHKIN(b"LNKPRV", ctx)?;
        SETMSG(
            b"NODE was #; backward pointer = #; forward pointer = #. \"FREE\" is #)",
            ctx,
        );
        ERRINT(b"#", NODE, ctx);
        ERRINT(b"#", POOL[[BCKWRD, NODE]], ctx);
        ERRINT(b"#", POOL[[FORWRD, NODE]], ctx);
        ERRINT(b"#", FREE, ctx);
        SIGERR(b"SPICE(UNALLOCATEDNODE)", ctx)?;
        CHKOUT(b"LNKPRV", ctx)?;
        return Ok(LNKPRV);
    }

    //
    // Just return the backward pointer of NODE.
    //
    LNKPRV = POOL[[BCKWRD, NODE]];

    Ok(LNKPRV)
}
