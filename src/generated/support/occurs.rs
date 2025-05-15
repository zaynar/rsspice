//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure OCCURS ( Count occurrences of a substring in a string )
pub fn OCCURS(STR: &[u8], SUB: &[u8]) -> i32 {
    let mut OCCURS: i32 = 0;
    let mut LSTR: i32 = 0;
    let mut LSUB: i32 = 0;
    let mut I: i32 = 0;

    //
    // Local variables
    //

    LSTR = intrinsics::LEN(STR);
    LSUB = intrinsics::LEN(SUB);

    I = 0;
    OCCURS = 0;

    while (I <= (LSTR - LSUB)) {
        if fstr::eq(fstr::substr(STR, (I + 1)..=(I + LSUB)), SUB) {
            OCCURS = (OCCURS + 1);
            I = (I + LSUB);
        } else {
            I = (I + 1);
        }
    }

    OCCURS
}
