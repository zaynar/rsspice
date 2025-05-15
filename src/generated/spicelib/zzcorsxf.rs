//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZCORSXF ( Correct state transformation matrix )
pub fn ZZCORSXF(XMIT: bool, DLT: f64, XFORM: &[f64], CORXFM: &mut [f64]) {
    let XFORM = DummyArray2D::new(XFORM, 1..=6, 1..=6);
    let mut CORXFM = DummyArrayMut2D::new(CORXFM, 1..=6, 1..=6);
    let mut LTSIGN: f64 = 0.0;
    let mut SCALE: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Determine the sign of the light time correction.
    //
    if XMIT {
        LTSIGN = 1.0;
    } else {
        LTSIGN = -1.0;
    }

    //
    // Since the only block we're changing is
    // the lower left, first copy the input matrix
    // to the output matrix.
    //
    MOVED(XFORM.as_slice(), 36, CORXFM.as_slice_mut());

    //
    // Adjust the rotation derivative block for
    // the rate of change of light time. All
    // that's required is to scale the block by
    //
    //    1 + LTSIGN*DLT
    //
    //
    SCALE = (1.0 + (LTSIGN * DLT));

    for COL in 1..=3 {
        //
        // Scale the vector starting at index
        // (4,COL) in place.
        //
        VSCLIP(SCALE, CORXFM.subarray_mut([4, COL]));
    }
}
