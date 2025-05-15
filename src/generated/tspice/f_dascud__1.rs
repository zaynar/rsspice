//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//
// PACKFS is a utility subroutine which packs DAS file summary
// information into an integer array.
//
pub fn PACKFS(
    NRESVR: i32,
    NRESVC: i32,
    NCOMR: i32,
    NCOMC: i32,
    FREE: i32,
    LASTLA: &[i32],
    LASTRC: &[i32],
    LASTWD: &[i32],
    FSUM: &mut [i32],
) {
    let LASTLA = DummyArray::new(LASTLA, 1..);
    let LASTRC = DummyArray::new(LASTRC, 1..);
    let LASTWD = DummyArray::new(LASTWD, 1..);
    let mut FSUM = DummyArrayMut::new(FSUM, 1..);

    FSUM[1] = NRESVR;
    FSUM[2] = NRESVC;
    FSUM[3] = NCOMR;
    FSUM[4] = NCOMC;
    FSUM[5] = FREE;

    spicelib::MOVEI(LASTLA.as_slice(), 3, FSUM.subarray_mut(6));
    spicelib::MOVEI(LASTRC.as_slice(), 3, FSUM.subarray_mut(9));
    spicelib::MOVEI(LASTWD.as_slice(), 3, FSUM.subarray_mut(12));
}
