//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXFRM: i32 = 1013;
const MAXBAS: i32 = 15000;
const LBSNGL: i32 = -5;

//$Procedure ZZSWFCLN ( Private, switch frame clean up )
pub fn ZZSWFCLN(
    HDFRAM: &mut [i32],
    FRPOOL: &mut [i32],
    BASBEG: &mut [i32],
    FRAMAT: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut HDFRAM = DummyArrayMut::new(HDFRAM, 1..);
    let mut FRPOOL = DummyArrayMut::new(FRPOOL, LBSNGL..);
    let mut BASBEG = DummyArrayMut::new(BASBEG, 1..);

    //
    // This routine must perform its function after an error has been
    // signaled, so it does not return upon entry after a SPICE error
    // occurs.
    //
    CHKIN(b"ZZSWFCLN", ctx)?;

    *FRAMAT = 0;

    //
    // Both of the following routines will execute even when a SPICE
    // error condition exists.
    //
    CLEARI(MAXFRM, BASBEG.as_slice_mut());
    ZZHSIINI(MAXFRM, HDFRAM.as_slice_mut(), FRPOOL.as_slice_mut(), ctx)?;

    CHKOUT(b"ZZSWFCLN", ctx)?;
    Ok(())
}
