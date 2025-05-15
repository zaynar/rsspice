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

/// LNK, number of free nodes
///
/// Return the number of free nodes in a doubly linked list pool.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  POOL       I   A doubly linked list pool.
///  LBPOOL     P   Lower bound of pool column indices.
///
///  The function returns the number of free nodes in the pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SIZE     is the number of nodes in the pool.
///
///  POOL     is a doubly linked list pool.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the number of free nodes in the pool.
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
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows the caller to find the number of free nodes
///  available in a doubly linked list pool, without having to make
///  use of knowledge of the internal structure of the pool.
///
///  Routines that allocate nodes can use this routine to determine
///  how many nodes can be allocated safely---an attempt to allocate
///  a node when no free nodes are available causes a SPICELIB error
///  to be signaled.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Let POOL be a doubly linked list pool containing 5 nodes.
///      If POOL contains the list
///
///         4 <--> 5 <--> 1 <--> 2
///
///
///      and the node 3 is unallocated, then the function reference
///
///         NFREE  =  LNKNFN ( POOL )
///
///
///      will assign the value 1 to NFREE.
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
pub fn lnknfn(pool: &[[i32; 2]]) -> i32 {
    let ret = LNKNFN(pool.as_flattened());
    ret
}

//$Procedure LNKNFN ( LNK, number of free nodes )
pub fn LNKNFN(POOL: &[i32]) -> i32 {
    let POOL = DummyArray2D::new(POOL, 1..=2, LBPOOL..);
    let mut LNKNFN: i32 = 0;

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
    // Grab the number of free nodes from the control area.
    //
    LNKNFN = POOL[[NFRROW, NFRCOL]];

    LNKNFN
}
