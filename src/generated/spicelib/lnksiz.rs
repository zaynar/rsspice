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

/// LNK, size
///
/// Return the size of a doubly linked list pool.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  POOL       I   A doubly linked list pool.
///  LBPOOL     P   Lower bound of pool column indices.
///
///  The function returns the size of the pool.
/// ```
///
/// # Detailed Input
///
/// ```text
///  POOL     is a doubly linked list pool.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the size (total number of nodes) of the pool.
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
///  This routine allows an application program to determine the size
///  of a doubly linked list pool at run-time, without having to rely
///  on knowledge of the internals of the doubly linked list pool
///  structure.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Let POOL be a doubly linked list pool. The total number of
///      nodes in a doubly linked list pool is set when the pool is
///      initialized by LNKINI. This number is returned by LNKSIZ:
///
///         C
///         C     This sequence of calls will assign the value 100
///         C     to the variable SIZE:
///         C
///               CALL LNKINI ( 100, POOL )
///               SIZE = LNKSIZ ( POOL )
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
pub fn lnksiz(pool: &[[i32; 2]]) -> i32 {
    let ret = LNKSIZ(pool.as_flattened());
    ret
}

//$Procedure LNKSIZ ( LNK, size )
pub fn LNKSIZ(POOL: &[i32]) -> i32 {
    let POOL = DummyArray2D::new(POOL, 1..=2, LBPOOL..);
    let mut LNKSIZ: i32 = 0;

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
    // Grab the pool size from the control area.
    //
    LNKSIZ = POOL[[SIZROW, SIZCOL]];

    LNKSIZ
}
