//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

// See GrammarPatcher::patch_swap
pub fn SWAPI_ARRAY(A: i32, B: i32, ARRAY: &mut [i32]) {
    let mut ARRAY = DummyArrayMut::new(ARRAY, 1..);
    let mut TEMP: i32 = 0;

    TEMP = ARRAY[A];
    ARRAY[A] = ARRAY[B];
    ARRAY[B] = TEMP;
}
