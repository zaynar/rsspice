//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZCK4D2I ( Unpack a set of integers from DP number )
pub fn ZZCK4D2I(DPCOEF: &mut f64, NSETS: i32, PARCOD: f64, I: &mut [i32]) {
    let mut I = DummyArrayMut::new(I, 1..);
    let mut X: f64 = 0.0;

    //
    // Local variables.
    //

    //
    // Let's unpack it!
    //
    X = f64::powi(PARCOD, (NSETS - 1));

    for K in 0..=(NSETS - 1) {
        I[(NSETS - K)] = (*DPCOEF / X) as i32;
        *DPCOEF = (*DPCOEF - ((I[(NSETS - K)] as f64) * X));
        X = (X / PARCOD);
    }

    //
    // All done.
    //
}
