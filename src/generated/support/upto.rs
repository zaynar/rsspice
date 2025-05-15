//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure UPTO ( Up to the next index of a substring )
pub fn UPTO(STRING: &[u8], SUBSTR: &[u8], START: i32) -> i32 {
    let mut UPTO: i32 = 0;
    let mut STRLEN: i32 = 0;
    let mut I: i32 = 0;
    let mut B: i32 = 0;

    //
    // Local variables
    //

    //
    // Just like it says in the header.
    //
    STRLEN = intrinsics::LEN(STRING);
    B = intrinsics::MAX0(&[1, START]);

    if (B > STRLEN) {
        UPTO = 0;
    } else {
        I = intrinsics::INDEX(fstr::substr(STRING, B..), SUBSTR);

        if (I > 0) {
            UPTO = ((B + I) - 2);
        } else {
            UPTO = STRLEN;
        }
    }

    UPTO
}
