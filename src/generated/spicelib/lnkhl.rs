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

/// LNK, head of list
///
/// Return the head node of the list containing a specified node.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NODE       I   Number of a node.
///  POOL       I   A doubly linked list pool.
///  LBPOOL     P   Lower bound of pool column indices.
///
///  The function returns the number of the head node of the list
///  containing NODE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NODE     is the number of a node in POOL. Normally,
///           NODE will designate an allocated node, but NODE
///           is permitted to be less than or equal to zero.
///
///  POOL     is a doubly linked list pool.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the number of the head node of the list
///  containing NODE. If NODE is non-positive, the function returns
///  zero.
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
///  1)  If the NODE is less than or equal to zero, NODE is not
///      considered to be erroneous. The value 0 is returned.
///
///  2)  If NODE is greater than the size of the pool, the error
///      SPICE(INVALIDNODE) is signaled. The value 0 is returned.
///
///  3)  If NODE is not the number of an allocated node, the error
///      SPICE(UNALLOCATEDNODE) is signaled. The value 0 is returned.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides a convenient way to find the head of a list
///  in a doubly linked list pool. The need to find the head of a
///  list arises in applications such as buffer management. For
///  example, in a system using a "least recently used" buffer
///  replacement policy, the head of a list may point to the most
///  recently accessed buffer element.
/// ```
///
/// # Examples
///
/// ```text
///  1)  If POOL is a doubly linked list pool that contains the list
///
///         3 <--> 7 <--> 1 <--> 44
///
///      any of function references
///
///         HEAD = LNKHL ( 3,  POOL )
///         HEAD = LNKHL ( 7,  POOL )
///         HEAD = LNKHL ( 44, POOL )
///
///      will assign the value 3 to HEAD.
///
///
///  2)  If POOL is a doubly linked list pool that contains the
///      singleton list consisting of the allocated node
///
///         44
///
///      the function reference
///
///         HEAD = LNKHL ( 44, POOL )
///
///      will assign the value 44 to HEAD.
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
pub fn lnkhl(ctx: &mut SpiceContext, node: i32, pool: &[[i32; 2]]) -> crate::Result<i32> {
    let ret = LNKHL(node, pool.as_flattened(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(ret)
}

//$Procedure LNKHL ( LNK, head of list )
pub fn LNKHL(NODE: i32, POOL: &[i32], ctx: &mut Context) -> f2rust_std::Result<i32> {
    let POOL = DummyArray2D::new(POOL, 1..=2, LBPOOL..);
    let mut LNKHL: i32 = 0;
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
    // If the node is non-positive, we regard it as the nil node.
    //
    if (NODE < 1) {
        LNKHL = 0;
        return Ok(LNKHL);

    //
    // If the node is out of range, something's very wrong.
    //
    } else if (NODE > POOL[[SIZROW, SIZCOL]]) {
        LNKHL = 0;

        CHKIN(b"LNKHL", ctx)?;
        SETMSG(b"NODE was #; valid range is 1 to #.", ctx);
        ERRINT(b"#", NODE, ctx);
        ERRINT(b"#", POOL[[SIZROW, SIZCOL]], ctx);
        SIGERR(b"SPICE(INVALIDNODE)", ctx)?;
        CHKOUT(b"LNKHL", ctx)?;
        return Ok(LNKHL);

    //
    // We don't do free nodes.
    //
    } else if (POOL[[BCKWRD, NODE]] == FREE) {
        LNKHL = 0;

        CHKIN(b"LNKHL", ctx)?;
        SETMSG(
            b"NODE was #; backward pointer = #; forward pointer = #. \"FREE\" is #)",
            ctx,
        );
        ERRINT(b"#", NODE, ctx);
        ERRINT(b"#", POOL[[BCKWRD, NODE]], ctx);
        ERRINT(b"#", POOL[[FORWRD, NODE]], ctx);
        ERRINT(b"#", FREE, ctx);
        SIGERR(b"SPICE(UNALLOCATEDNODE)", ctx)?;
        CHKOUT(b"LNKHL", ctx)?;
        return Ok(LNKHL);
    }

    //
    // Find the head of the list.
    //
    LNKHL = NODE;
    PREV = POOL[[BCKWRD, NODE]];

    while (PREV > 0) {
        LNKHL = PREV;
        PREV = POOL[[BCKWRD, LNKHL]];
    }

    //
    // LNKHL is now the head of the list.
    //
    Ok(LNKHL)
}
