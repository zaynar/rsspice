//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZDDHCLU ( Private --- DDH Count Locks )
pub fn ZZDDHCLU(UTLCK: &[bool], NUT: i32) -> i32 {
    let UTLCK = DummyArray::new(UTLCK, 1..);
    let mut ZZDDHCLU: i32 = 0;

    //
    // Local Variables
    //

    //
    // Loop through UTLCK counting the number of TRUE values.
    //
    ZZDDHCLU = 0;

    for I in 1..=NUT {
        if UTLCK[I] {
            ZZDDHCLU = (ZZDDHCLU + 1);
        }
    }

    ZZDDHCLU
}
