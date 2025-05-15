//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXFRM: i32 = 1013;
const MAXBAS: i32 = 15000;
const LBSNGL: i32 = -5;

//$Procedure ZZSWFINI ( Private, switch frame initialization )
pub fn ZZSWFINI(
    HDFRAM: &mut [i32],
    FRPOOL: &mut [i32],
    FIDLST: &mut [i32],
    BASBEG: &mut [i32],
    FREE: &mut i32,
    PRVAT: &mut i32,
    PRVFRM: &mut i32,
    SAMFRM: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut HDFRAM = DummyArrayMut::new(HDFRAM, 1..);
    let mut FRPOOL = DummyArrayMut::new(FRPOOL, LBSNGL..);
    let mut FIDLST = DummyArrayMut::new(FIDLST, 1..);
    let mut BASBEG = DummyArrayMut::new(BASBEG, 1..);

    //
    // SPICELIB functions
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZSWFINI", ctx)?;

    //
    // Initialize the frame hash head node array, hash collision list
    // pool, and parallel frame ID list. Initialize the array of start
    // indices of associated base frame information. Indicate the base
    // frame data buffers are empty by setting FREE to 1.
    //
    ZZHSIINI(MAXFRM, HDFRAM.as_slice_mut(), FRPOOL.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        //
        // This code should be unreachable. It's provided for safety.
        //
        CHKOUT(b"ZZSWFINI", ctx)?;
        return Ok(());
    }

    CLEARI(MAXFRM, FIDLST.as_slice_mut());
    CLEARI(MAXFRM, BASBEG.as_slice_mut());

    *FREE = 1;

    //
    // Initialize all saved frame identity information.
    //
    *PRVFRM = 0;
    *PRVAT = 0;
    *SAMFRM = false;

    CHKOUT(b"ZZSWFINI", ctx)?;
    Ok(())
}
