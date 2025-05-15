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

/// LNK, initialize
///
/// Initialize a doubly linked list pool.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SIZE       I   Number of nodes in the pool.
///  POOL      I-O  An array that is a linked pool on output.
///  LBPOOL     P   Lower bound of pool column indices.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SIZE     is the number of nodes in the pool.
///
///  POOL     is an integer array that will contain the linked
///           pool on output.
/// ```
///
/// # Detailed Output
///
/// ```text
///  POOL     is an initialized doubly linked list pool.
///           The status of the pool is as follows:
///
///             --  All nodes in the pool are on the free list.
///
///             --  The free pointer indicates the first node.
///
///             --  The total node count is set to the input
///                 value, SIZE.
///
///             --  The free node count is the input value, SIZE.
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
///  1)  If the requested number of nodes is nonpositive, the error
///      SPICE(INVALIDCOUNT) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  LNKINI must be called once to initialize a doubly linked list
///  pool before the pool is used. LNKINI can be called at any time
///  to re-initialize a doubly linked list pool.
///
///  The functions
///
///     LNKNFN ( LNK, number of free nodes ) and
///     LNKSIZ ( LNK, size of pool )
///
///  will both return the value PLSIZE if called immediately after a
///  call to LNKINI.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Let POOL be a doubly linked list pool with a maximum of
///      100 nodes. POOL should be declared as follows:
///
///         INTEGER               LBPOOL
///         PARAMETER           ( LBPOOL = -5 )
///
///         INTEGER               PLSIZE
///         PARAMETER           ( PLSIZE = 100 )
///
///         INTEGER               POOL ( 2, LBPOOL : PLSIZE )
///
///
///      To initialize POOL, use the call
///
///         CALL LNKINI ( PLSIZE, POOL )
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
pub fn lnkini(ctx: &mut SpiceContext, size: i32, pool: &mut [[i32; 2]]) -> crate::Result<()> {
    LNKINI(size, pool.as_flattened_mut(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure LNKINI ( LNK, initialize )
pub fn LNKINI(SIZE: i32, POOL: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut POOL = DummyArrayMut2D::new(POOL, 1..=2, LBPOOL..);

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
    // The requested number of nodes must be valid.
    //
    if (SIZE < 1) {
        CHKIN(b"LNKINI", ctx)?;
        SETMSG(b"A linked list cannot have # nodes.", ctx);
        ERRINT(b"#", SIZE, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"LNKINI", ctx)?;
        return Ok(());
    }

    //
    // Initialize the pool.  The free list occupies the whole pool at
    // this point.
    //

    //
    // POOL( SIZROW, SIZCOL ) is the pool size.
    //
    POOL[[SIZROW, SIZCOL]] = SIZE;

    //
    // POOL( NFRROW, NFRCOL ) is the number of free nodes.
    //
    POOL[[NFRROW, NFRCOL]] = SIZE;

    //
    // POOL( FREROW, FRECOL) is the "free" pointer.  It points to the
    // first free node, which is node 1.
    //
    POOL[[FREROW, FRECOL]] = 1;

    //
    // Initialize the backward and forward pointers.  The last forward
    // pointer is zero.  All of the backward pointers contain the value
    // FREE.
    //
    for I in 1..=(SIZE - 1) {
        POOL[[FORWRD, I]] = (I + 1);
        POOL[[BCKWRD, I]] = FREE;
    }

    POOL[[FORWRD, SIZE]] = 0;
    POOL[[BCKWRD, SIZE]] = FREE;

    Ok(())
}
