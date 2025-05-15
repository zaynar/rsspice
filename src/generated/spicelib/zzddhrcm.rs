//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure ZZDDHRCM ( Private --- DDH Request Count )
pub fn ZZDDHRCM(NUT: i32, UTCST: &mut [i32], REQCNT: &mut i32) {
    let mut UTCST = DummyArrayMut::new(UTCST, 1..);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Check to see if REQCNT is INTMAX, otherwise just increment
    // REQCNT.
    //
    if (*REQCNT == INTMAX()) {
        *REQCNT = ((INTMAX() / 2) + 1);

        for I in 1..=NUT {
            UTCST[I] = intrinsics::MAX0(&[1, (UTCST[I] / 2)]);
        }
    } else {
        *REQCNT = (*REQCNT + 1);
    }
}
