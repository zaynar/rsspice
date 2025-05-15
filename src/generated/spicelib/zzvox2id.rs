//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZVOX2ID ( Return voxel index from coords )
pub fn ZZVOX2ID(VIXYZ: &[i32], NVOX: &[i32]) -> i32 {
    let VIXYZ = DummyArray::new(VIXYZ, 1..=3);
    let NVOX = DummyArray::new(NVOX, 1..=3);
    let mut ZZVOX2ID: i32 = 0;

    //
    // Convert from voxel coordinates to voxel index. A more
    // readable form for the following function:
    //
    //    NX   = NVOX(1)
    //    NY   = NVOX(2)
    //    NXNY = NX * NY
    //
    //    ZZVOX2ID = VIXYZ(1) + (VIXYZ(2)-1)*NX + (VIXYZ(3)-1)*NXNY
    //
    // Expressing the function in this format improves runtime
    // performance as per Horner's Rule.
    //
    ZZVOX2ID = (VIXYZ[1] + (NVOX[1] * ((VIXYZ[2] - 1) + ((VIXYZ[3] - 1) * NVOX[2]))));

    ZZVOX2ID
}
