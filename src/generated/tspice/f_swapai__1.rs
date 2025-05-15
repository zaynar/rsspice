//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//
// T_SWAPAI is a utility routine used for testing SWAPAI. T_SWAPAI
// uses an alternate implementation of the swapping algorithm that
// uses work space to build the output array:  the swapping
// algorithm is quite simple when the constraint of in-place
// operation is removed.
//
// This routine does no error checking.
//
pub fn T_SWAPAI(
    SIZE: i32,
    N: i32,
    LOCN: i32,
    M: i32,
    LOCM: i32,
    ARRAY: &mut [i32],
    WORK: &mut [i32],
) {
    let mut ARRAY = DummyArrayMut::new(ARRAY, 1..);
    let mut WORK = DummyArrayMut::new(WORK, 1..);
    let mut LOC: i32 = 0;
    let mut LOWER: i32 = 0;
    let mut NLOW: i32 = 0;
    let mut NMOVE: i32 = 0;
    let mut NUP: i32 = 0;
    let mut TO: i32 = 0;
    let mut UPPER: i32 = 0;

    //
    // Local variables
    //

    //
    // We'll build the output array in WORK; then we'll copy the
    // result back to ARRAY.
    //
    // Identify the start indices of the "top" and "bottom" array slices
    // to be swapped.  We consider the lower addresses to be at the
    // "top" of the array.
    //
    UPPER = intrinsics::MIN0(&[LOCN, LOCM]);
    LOWER = intrinsics::MAX0(&[LOCN, LOCM]);

    if (UPPER == LOCN) {
        NUP = N;
        NLOW = M;
    } else {
        NUP = M;
        NLOW = N;
    }

    //
    // Move the elements preceding UPPER into WORK.
    //
    TO = 1;
    NMOVE = (UPPER - 1);
    spicelib::MOVEI(ARRAY.subarray(1), NMOVE, WORK.subarray_mut(TO));

    //
    // Move the elements in the lower slice into WORK.
    //
    TO = (TO + NMOVE);
    NMOVE = NLOW;
    spicelib::MOVEI(ARRAY.subarray(LOWER), NMOVE, WORK.subarray_mut(TO));

    //
    // Move the elements between the slices into WORK.
    //
    TO = (TO + NMOVE);
    NMOVE = ((LOWER - 1) - ((UPPER + NUP) - 1));

    spicelib::MOVEI(ARRAY.subarray((UPPER + NUP)), NMOVE, WORK.subarray_mut(TO));

    //
    // Move the elements in the upper slice into WORK.
    //
    TO = (TO + NMOVE);
    NMOVE = NUP;
    spicelib::MOVEI(ARRAY.subarray(UPPER), NMOVE, WORK.subarray_mut(TO));

    //
    // Move the elements below the lower slice into WORK.
    //
    TO = (TO + NMOVE);
    LOC = (LOWER + NLOW);
    NMOVE = (SIZE - (LOC - 1));

    spicelib::MOVEI(ARRAY.subarray(LOC), NMOVE, WORK.subarray_mut(TO));

    //
    // Copy WORK into ARRAY.
    //
    spicelib::MOVEI(WORK.as_slice(), SIZE, ARRAY.as_slice_mut());
}
