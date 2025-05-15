//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure  ZZINGRD  ( is a voxel inside the grid? )
pub fn ZZINGRD(NVOX: &[i32], VOXEL: &[i32]) -> bool {
    let NVOX = DummyArray::new(NVOX, 1..=3);
    let VOXEL = DummyArray::new(VOXEL, 1..=3);
    let mut ZZINGRD: bool = false;

    ZZINGRD = false;

    //
    // Determine if voxel is outside the voxel grid
    // in any direction.
    //
    for I in 1..=3 {
        if ((VOXEL[I] < 1) || (VOXEL[I] > NVOX[I])) {
            return ZZINGRD;
        }
    }

    ZZINGRD = true;

    ZZINGRD
}
