//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBPOOL: i32 = -5;
const NEXT: i32 = 1;
const PREV: i32 = 2;

//$Procedure      ZZCLN ( Private --- clean up )
pub fn ZZCLN(
    LOOKAT: i32,
    NAMEAT: i32,
    NAMLST: &mut [i32],
    DATLST: &mut [i32],
    NMPOOL: &mut [i32],
    CHPOOL: &mut [i32],
    DPPOOL: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NAMLST = DummyArrayMut::new(NAMLST, 1..);
    let mut DATLST = DummyArrayMut::new(DATLST, 1..);
    let mut NMPOOL = DummyArrayMut2D::new(NMPOOL, 1..=2, LBPOOL..);
    let mut CHPOOL = DummyArrayMut2D::new(CHPOOL, 1..=2, LBPOOL..);
    let mut DPPOOL = DummyArrayMut2D::new(DPPOOL, 1..=2, LBPOOL..);
    let mut HEAD: i32 = 0;
    let mut TAIL: i32 = 0;

    //
    // Local Parameters and Variables
    //

    //
    // First perform the clean up function. This variable
    // has been corrupted so there's no point in hanging
    // on to it.
    //
    // First remove the data...
    //
    CHKIN(b"ZZCLN", ctx)?;

    HEAD = DATLST[NAMEAT];

    if (HEAD < 0) {
        HEAD = -HEAD;
        TAIL = -CHPOOL[[PREV, HEAD]];
        LNKFSL(HEAD, TAIL, CHPOOL.as_slice_mut(), ctx)?;
    } else if (HEAD > 0) {
        TAIL = -DPPOOL[[PREV, HEAD]];
        LNKFSL(HEAD, TAIL, DPPOOL.as_slice_mut(), ctx)?;
    }
    //
    // Remove the sub-list head from the data list.
    //
    DATLST[NAMEAT] = 0;
    //
    // If this was a singleton list remove the pointer to
    // the head of the list.
    //
    HEAD = NAMLST[LOOKAT];
    TAIL = -NMPOOL[[PREV, HEAD]];

    if (HEAD == TAIL) {
        NAMLST[LOOKAT] = 0;
    } else if (NAMLST[LOOKAT] == NAMEAT) {
        NAMLST[LOOKAT] = NMPOOL[[NEXT, NAMEAT]];
    }
    //
    // Finally free up this node in the NMPOOL.
    //
    HEAD = NAMEAT;
    TAIL = NAMEAT;

    LNKFSL(HEAD, TAIL, NMPOOL.as_slice_mut(), ctx)?;

    CHKOUT(b"ZZCLN", ctx)?;
    Ok(())
}
