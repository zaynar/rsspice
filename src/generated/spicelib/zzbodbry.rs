//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      ZZBODBRY ( Return barycenter code for a body )
pub fn ZZBODBRY(BODY: i32) -> i32 {
    let mut ZZBODBRY: i32 = 0;

    if ((BODY >= 100) && (BODY <= 999)) {
        //
        // BODY is a "traditional" NAIF planet or natural satellite
        // ID code.
        //
        ZZBODBRY = (BODY / 100);
    } else if ((BODY >= 10000) && (BODY <= 99999)) {
        //
        // BODY is an "extended" NAIF natural satellite ID code.
        //
        ZZBODBRY = (BODY / 10000);
    } else {
        //
        // BODY is a barycenter code or is not associated with a
        // planetary system.  In either case, we simply return
        // the input value BODY.
        //
        ZZBODBRY = BODY;
    }

    ZZBODBRY
}
