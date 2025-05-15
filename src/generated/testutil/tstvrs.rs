//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      TSTVRS ( Test Utility version strings )
pub fn TSTVRS(VERSTR: &mut [u8]) {
    //
    // At the current time only the TOOLKIT version number is specified.
    //
    fstr::assign(VERSTR, b"TST001");
}
