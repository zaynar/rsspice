//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

// See GrammarPatcher::patch_swap
pub fn SWAPC_ARRAY(A: i32, B: i32, ARRAY: CharArrayMut) {
    let mut ARRAY = DummyCharArrayMut::new(ARRAY, None, 1..);
    let mut TEMP = [b' '; 1];
    let mut M: i32 = 0;

    M = intrinsics::MIN0(&[intrinsics::LEN(&ARRAY[A]), intrinsics::LEN(&ARRAY[B])]);

    for I in 1..=M {
        fstr::assign(&mut TEMP, fstr::substr(ARRAY.get(A), I..=I));
        let val = fstr::substr(ARRAY.get(B), I..=I).to_vec();
        fstr::assign(fstr::substr_mut(ARRAY.get_mut(A), I..=I), &val);
        fstr::assign(fstr::substr_mut(ARRAY.get_mut(B), I..=I), &TEMP);
    }

    if (intrinsics::LEN(&ARRAY[A]) > M) {
        fstr::assign(fstr::substr_mut(ARRAY.get_mut(A), (M + 1)..), b" ");
    }

    if (intrinsics::LEN(&ARRAY[B]) > M) {
        fstr::assign(fstr::substr_mut(ARRAY.get_mut(B), (M + 1)..), b" ");
    }
}
