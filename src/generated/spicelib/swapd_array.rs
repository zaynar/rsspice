//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

// See GrammarPatcher::patch_swap
pub fn SWAPD_ARRAY(A: i32, B: i32, ARRAY: &mut [f64]) {
    let mut ARRAY = DummyArrayMut::new(ARRAY, 1..);
    let mut TEMP: f64 = 0.0;

    TEMP = ARRAY[A];
    ARRAY[A] = ARRAY[B];
    ARRAY[B] = TEMP;
}
