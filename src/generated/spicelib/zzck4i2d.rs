//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZCK4I2D ( Pack set of integers into a single DP )
pub fn ZZCK4I2D(I: &[i32], NSETS: i32, PARCOD: f64, DPCOEF: &mut f64) {
    let I = DummyArray::new(I, 1..);
    let mut X: f64 = 0.0;

    //
    // Local variables
    //

    //
    // Let's pack it!
    //
    *DPCOEF = 0.0;

    X = 1 as f64;

    for K in intrinsics::range(1, NSETS, 1) {
        *DPCOEF = (*DPCOEF + ((I[K] as f64) * X));
        X = (X * PARCOD);
    }

    //
    // All done.
    //
}
